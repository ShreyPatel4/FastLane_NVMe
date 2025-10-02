#include "rdma_client.h"

int rdma_client_initialize(void) {
    return 0; // Stub success for now
}

int rdma_client_post_write(const char *queue, const void *buffer, unsigned long length) {
    (void)queue;
    (void)buffer;
    (void)length;
    return 0;
}
