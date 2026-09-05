#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <stdlib.h>
#include <sys/socket.h>
#include <unistd.h>

static void record_attempt(void) {
  const char *path = getenv("SSPF_NETWORK_PROBE");
  if (path == NULL) return;
  int fd = open(path, O_WRONLY | O_CREAT | O_APPEND, 0600);
  if (fd >= 0) {
    const char message[] = "network call blocked\n";
    write(fd, message, sizeof(message) - 1);
    close(fd);
  }
}

int socket(int domain, int type, int protocol) {
  (void)domain;
  (void)type;
  (void)protocol;
  record_attempt();
  errno = EACCES;
  return -1;
}

int connect(int socket_fd, const struct sockaddr *address, socklen_t address_length) {
  (void)socket_fd;
  (void)address;
  (void)address_length;
  record_attempt();
  errno = EACCES;
  return -1;
}
