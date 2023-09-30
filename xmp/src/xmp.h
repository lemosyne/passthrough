#define FUSE_USE_VERSION 31

#include <fuse.h>

void *xmp_init(struct fuse_conn_info *conn, struct fuse_config *cfg);

int xmp_getattr(const char *path, struct stat *stbuf,
                struct fuse_file_info *fi);

int xmp_access(const char *path, int mask);

int xmp_readlink(const char *path, char *buf, size_t size);

int xmp_opendir(const char *path, struct fuse_file_info *fi);

int xmp_readdir(const char *path, void *buf, fuse_fill_dir_t filler,
                off_t offset, struct fuse_file_info *fi,
                enum fuse_readdir_flags flags);

int xmp_releasedir(const char *path, struct fuse_file_info *fi);

int xmp_mknod(const char *path, mode_t mode, dev_t rdev);

int xmp_mkdir(const char *path, mode_t mode);

int xmp_unlink(const char *path);

int xmp_rmdir(const char *path);

int xmp_symlink(const char *from, const char *to);

int xmp_rename(const char *from, const char *to, unsigned int flags);

int xmp_link(const char *from, const char *to);

int xmp_chmod(const char *path, mode_t mode, struct fuse_file_info *fi);

int xmp_chown(const char *path, uid_t uid, gid_t gid,
              struct fuse_file_info *fi);

int xmp_truncate(const char *path, off_t size, struct fuse_file_info *fi);

int xmp_create(const char *path, mode_t mode, struct fuse_file_info *fi);

int xmp_open(const char *path, struct fuse_file_info *fi);

int xmp_read(const char *path, char *buf, size_t size, off_t offset,
             struct fuse_file_info *fi);

int xmp_read_buf(const char *path, struct fuse_bufvec **bufp, size_t size,
                 off_t offset, struct fuse_file_info *fi);

int xmp_write(const char *path, const char *buf, size_t size, off_t offset,
              struct fuse_file_info *fi);

int xmp_write_buf(const char *path, struct fuse_bufvec *buf, off_t offset,
                  struct fuse_file_info *fi);

int xmp_statfs(const char *path, struct statvfs *stbuf);

int xmp_flush(const char *path, struct fuse_file_info *fi);

int xmp_release(const char *path, struct fuse_file_info *fi);

int xmp_fsync(const char *path, int isdatasync, struct fuse_file_info *fi);

#ifdef HAVE_POSIX_FALLOCATE
int xmp_fallocate(const char *path, int mode, off_t offset, off_t length,
                  struct fuse_file_info *fi);
#endif

#ifdef HAVE_SETXATTR
/* xattr operations are optional and can safely be left unimplemented */
int xmp_setxattr(const char *path, const char *name, const char *value,
                 size_t size, int flags);

int xmp_getxattr(const char *path, const char *name, char *value, size_t size);

int xmp_listxattr(const char *path, char *list, size_t size);

int xmp_removexattr(const char *path, const char *name);
#endif /* HAVE_SETXATTR */

#ifdef HAVE_LIBULOCKMGR
int xmp_lock(const char *path, struct fuse_file_info *fi, int cmd,
             struct flock *lock);
#endif

int xmp_flock(const char *path, struct fuse_file_info *fi, int op);

ssize_t xmp_copy_file_range(const char *path_in, struct fuse_file_info *fi_in,
                            off_t off_in, const char *path_out,
                            struct fuse_file_info *fi_out, off_t off_out,
                            size_t len, int flags);

off_t xmp_lseek(const char *path, off_t off, int whence,
                struct fuse_file_info *fi);
