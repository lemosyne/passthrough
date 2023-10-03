/*
  FUSE: Filesystem in Userspace
  Copyright (C) 2001-2007  Miklos Szeredi <miklos@szeredi.hu>
  Copyright (C) 2011       Sebastian Pipping <sebastian@pipping.org>

  This program can be distributed under the terms of the GNU GPLv2.
  See the file COPYING.
*/

/** @file
 *
 * This file system mirrors the existing file system hierarchy of the
 * system, starting at the root file system. This is implemented by
 * just "passing through" all requests to the corresponding user-space
 * libc functions. Its performance is terrible.
 *
 * Compile with
 *
 *     gcc -Wall passthrough.c `pkg-config fuse3 --cflags --libs` -o passthrough
 *
 * ## Source code ##
 * \include passthrough.c
 */

#define FUSE_USE_VERSION 31

#define _GNU_SOURCE

#include <fuse.h>

#ifdef HAVE_LIBULOCKMGR
#include <ulockmgr.h>
#endif

#include <assert.h>
#include <dirent.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <unistd.h>
#ifdef HAVE_SETXATTR
#include <sys/xattr.h>
#endif
#include <sys/file.h> /* flock(2) */

void *xmp_init(struct fuse_conn_info *conn, struct fuse_config *cfg) {
  (void)conn;
  cfg->use_ino = 1;

  /* Pick up changes from lower filesystem right away. This is
     also necessary for better hardlink support. When the kernel
     calls the unlink() handler, it does not know the inode of
     the to-be-removed entry and can therefore not invalidate
     the cache of the associated inode - resulting in an
     incorrect st_nlink value being reported for any remaining
     hardlinks to this inode. */
  cfg->entry_timeout = 0;
  cfg->attr_timeout = 0;
  cfg->negative_timeout = 0;

  return NULL;
}

int xmp_getattr(const char *path, struct stat *stbuf,
                struct fuse_file_info *fi) {
  int res;

  if (fi)
    res = fstat(fi->fh, stbuf);
  else {
    res = lstat(path, stbuf);
  }

  if (res == -1)
    return -errno;

  return 0;
}

int xmp_access(const char *path, int mask) {
  int res;

  res = access(path, mask);

  if (res == -1)
    return -errno;

  return 0;
}

int xmp_readlink(const char *path, char *buf, size_t size) {
  int res;

  res = readlink(path, buf, size - 1);

  if (res == -1)
    return -errno;

  buf[res] = '\0';
  return 0;
}

struct xmp_dirp {
  DIR *dp;
  struct dirent *entry;
  off_t offset;
};

int xmp_opendir(const char *path, struct fuse_file_info *fi) {
  int res;
  struct xmp_dirp *d = malloc(sizeof(struct xmp_dirp));
  if (d == NULL)
    return -ENOMEM;

  d->dp = opendir(path);

  if (d->dp == NULL) {
    res = -errno;
    free(d);
    return res;
  }
  d->offset = 0;
  d->entry = NULL;

  fi->fh = (unsigned long)d;
  return 0;
}

static struct xmp_dirp *get_dirp(struct fuse_file_info *fi) {
  return (struct xmp_dirp *)(uintptr_t)fi->fh;
}

int xmp_readdir(const char *path, void *buf, fuse_fill_dir_t filler,
                off_t offset, struct fuse_file_info *fi,
                enum fuse_readdir_flags flags) {
  struct xmp_dirp *d = get_dirp(fi);

  (void)path;
  (void)flags;

  if (offset != d->offset) {
#ifndef __FreeBSD__
    seekdir(d->dp, offset);
#else
    /* Subtract the one that we add when calling
       telldir() below */
    seekdir(d->dp, offset - 1);
#endif
    d->entry = NULL;
    d->offset = offset;
  }
  while (1) {
    struct stat st;
    off_t nextoff;
    enum fuse_fill_dir_flags fill_flags = 0;

    if (!d->entry) {
      d->entry = readdir(d->dp);
      if (!d->entry)
        break;
    }
#ifdef HAVE_FSTATAT
    if (flags & FUSE_READDIR_PLUS) {
      int res;

      res = fstatat(dirfd(d->dp), d->entry->d_name, &st, AT_SYMLINK_NOFOLLOW);
      if (res != -1)
        fill_flags |= FUSE_FILL_DIR_PLUS;
    }
#endif
    if (!(fill_flags & FUSE_FILL_DIR_PLUS)) {
      memset(&st, 0, sizeof(st));
      st.st_ino = d->entry->d_ino;
      st.st_mode = d->entry->d_type << 12;
    }
    nextoff = telldir(d->dp);
#ifdef __FreeBSD__
    /* Under FreeBSD, telldir() may return 0 the first time
       it is called. But for libfuse, an offset of zero
       means that offsets are not supported, so we shift
       everything by one. */
    nextoff++;
#endif
    if (filler(buf, d->entry->d_name, &st, nextoff, fill_flags))
      break;

    d->entry = NULL;
    d->offset = nextoff;
  }

  return 0;
}

int xmp_releasedir(const char *path, struct fuse_file_info *fi) {
  struct xmp_dirp *d = get_dirp(fi);
  (void)path;
  closedir(d->dp);
  free(d);
  return 0;
}

int xmp_mknod(const char *path, mode_t mode, dev_t rdev) {
  int res;

  if (S_ISFIFO(mode)) {
    res = mkfifo(path, mode);
  } else {
    res = mknod(path, mode, rdev);
  }
  if (res == -1)
    return -errno;

  return 0;
}

int xmp_mkdir(const char *path, mode_t mode) {
  int res;

  res = mkdir(path, mode);
  if (res == -1)
    return -errno;

  return 0;
}

int xmp_unlink(const char *path) {
  int res;

  res = unlink(path);

  if (res == -1)
    return -errno;

  return 0;
}

int xmp_rmdir(const char *path) {
  int res;

  res = rmdir(path);

  if (res == -1)
    return -errno;

  return 0;
}

int xmp_symlink(const char *from, const char *to) {
  int res;

  res = symlink(from, to);

  if (res == -1)
    return -errno;

  return 0;
}

int xmp_rename(const char *from, const char *to, unsigned int flags) {
  int res;

  /* When we have renameat2() in libc, then we can implement flags */
  if (flags)
    return -EINVAL;

  res = rename(from, to);

  if (res == -1)
    return -errno;

  return 0;
}

int xmp_link(const char *from, const char *to) {
  int res;

  res = link(from, to);

  if (res == -1)
    return -errno;

  return 0;
}

int xmp_chmod(const char *path, mode_t mode, struct fuse_file_info *fi) {
  int res;

  if (fi)
    res = fchmod(fi->fh, mode);
  else {
    res = chmod(path, mode);
  }

  if (res == -1)
    return -errno;

  return 0;
}

int xmp_chown(const char *path, uid_t uid, gid_t gid,
              struct fuse_file_info *fi) {
  int res;

  if (fi)
    res = fchown(fi->fh, uid, gid);
  else {
    res = lchown(path, uid, gid);
  }

  if (res == -1)
    return -errno;

  return 0;
}

int xmp_truncate(const char *path, off_t size, struct fuse_file_info *fi) {
  int res;

  if (fi)
    res = ftruncate(fi->fh, size);
  else {
    res = truncate(path, size);
  }

  if (res == -1)
    return -errno;

  return 0;
}

#ifdef HAVE_UTIMENSAT
int xmp_utimens(const char *path, const struct timespec ts[2],
                struct fuse_file_info *fi) {
  int res;

  /* don't use utime/utimes since they follow symlinks */
  if (fi)
    res = futimens(fi->fh, ts);
  else
    res = utimensat(0, path, ts, AT_SYMLINK_NOFOLLOW);
  if (res == -1)
    return -errno;

  return 0;
}
#endif

int xmp_create(const char *path, mode_t mode, struct fuse_file_info *fi) {
  int fd;

  fd = open(path, fi->flags, mode);

  if (fd == -1)
    return -errno;

  fi->fh = fd;
  return 0;
}

int xmp_open(const char *path, struct fuse_file_info *fi) {
  int fd;

  fd = open(path, fi->flags);

  if (fd == -1)
    return -errno;

  fi->fh = fd;
  return 0;
}

int xmp_read(const char *path, char *buf, size_t size, off_t offset,
             struct fuse_file_info *fi) {
  int res;

  (void)path;
  res = pread(fi->fh, buf, size, offset);
  if (res == -1)
    res = -errno;

  return res;
}

int xmp_read_buf(const char *path, struct fuse_bufvec **bufp, size_t size,
                 off_t offset, struct fuse_file_info *fi) {
  struct fuse_bufvec *src;

  (void)path;

  src = malloc(sizeof(struct fuse_bufvec));
  if (src == NULL)
    return -ENOMEM;

  *src = FUSE_BUFVEC_INIT(size);

  src->buf[0].flags = FUSE_BUF_IS_FD | FUSE_BUF_FD_SEEK;
  src->buf[0].fd = fi->fh;
  src->buf[0].pos = offset;

  *bufp = src;

  return 0;
}

int xmp_write(const char *path, const char *buf, size_t size, off_t offset,
              struct fuse_file_info *fi) {
  int res;

  (void)path;
  res = pwrite(fi->fh, buf, size, offset);
  if (res == -1)
    res = -errno;

  return res;
}

int xmp_write_buf(const char *path, struct fuse_bufvec *buf, off_t offset,
                  struct fuse_file_info *fi) {
  struct fuse_bufvec dst = FUSE_BUFVEC_INIT(fuse_buf_size(buf));

  (void)path;

  dst.buf[0].flags = FUSE_BUF_IS_FD | FUSE_BUF_FD_SEEK;
  dst.buf[0].fd = fi->fh;
  dst.buf[0].pos = offset;

  return fuse_buf_copy(&dst, buf, FUSE_BUF_SPLICE_NONBLOCK);
}

int xmp_statfs(const char *path, struct statvfs *stbuf) {
  int res;

  res = statvfs(path, stbuf);

  if (res == -1)
    return -errno;

  return 0;
}

int xmp_flush(const char *path, struct fuse_file_info *fi) {
  int res;

  (void)path;
  /* This is called from every close on an open file, so call the
     close on the underlying filesystem.	But since flush may be
     called multiple times for an open file, this must not really
     close the file.  This is important if used on a network
     filesystem like NFS which flush the data/metadata on close() */
  res = close(dup(fi->fh));
  if (res == -1)
    return -errno;

  return 0;
}

int xmp_release(const char *path, struct fuse_file_info *fi) {
  (void)path;
  close(fi->fh);

  return 0;
}

int xmp_fsync(const char *path, int isdatasync, struct fuse_file_info *fi) {
  int res;
  (void)path;

#ifndef HAVE_FDATASYNC
  (void)isdatasync;
#else
  if (isdatasync)
    res = fdatasync(fi->fh);
  else
#endif
  res = fsync(fi->fh);
  if (res == -1)
    return -errno;

  return 0;
}

#ifdef HAVE_POSIX_FALLOCATE
int xmp_fallocate(const char *path, int mode, off_t offset, off_t length,
                  struct fuse_file_info *fi) {
  (void)path;

  if (mode)
    return -EOPNOTSUPP;

  return -posix_fallocate(fi->fh, offset, length);
}
#endif

#ifdef HAVE_SETXATTR
/* xattr operations are optional and can safely be left unimplemented */
int xmp_setxattr(const char *path, const char *name, const char *value,
                 size_t size, int flags) {
  int res = lsetxattr(path, name, value, size, flags);
  if (res == -1)
    return -errno;
  return 0;
}

int xmp_getxattr(const char *path, const char *name, char *value, size_t size) {
  int res = lgetxattr(path, name, value, size);
  if (res == -1)
    return -errno;
  return res;
}

int xmp_listxattr(const char *path, char *list, size_t size) {
  int res = llistxattr(path, list, size);
  if (res == -1)
    return -errno;
  return res;
}

int xmp_removexattr(const char *path, const char *name) {
  int res = lremovexattr(path, name);
  if (res == -1)
    return -errno;
  return 0;
}
#endif /* HAVE_SETXATTR */

#ifdef HAVE_LIBULOCKMGR
int xmp_lock(const char *path, struct fuse_file_info *fi, int cmd,
             struct flock *lock) {
  (void)path;

  return ulockmgr_op(fi->fh, cmd, lock, &fi->lock_owner,
                     sizeof(fi->lock_owner));
}
#endif

int xmp_flock(const char *path, struct fuse_file_info *fi, int op) {
  int res;
  (void)path;

  res = flock(fi->fh, op);
  if (res == -1)
    return -errno;

  return 0;
}

ssize_t xmp_copy_file_range(const char *path_in, struct fuse_file_info *fi_in,
                            off_t off_in, const char *path_out,
                            struct fuse_file_info *fi_out, off_t off_out,
                            size_t len, int flags) {
  ssize_t res;
  (void)path_in;
  (void)path_out;

  res = copy_file_range(fi_in->fh, &off_in, fi_out->fh, &off_out, len, flags);
  if (res == -1)
    return -errno;

  return res;
}

off_t xmp_lseek(const char *path, off_t off, int whence,
                struct fuse_file_info *fi) {
  off_t res;
  (void)path;

  res = lseek(fi->fh, off, whence);
  if (res == -1)
    return -errno;

  return res;
}
