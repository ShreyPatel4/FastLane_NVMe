#ifndef RDMA_CLIENT_H
#define RDMA_CLIENT_H

int rdma_client_initialize(void);
int rdma_client_post_write(const char *queue, const void *buffer, unsigned long length);

#endif // RDMA_CLIENT_H
