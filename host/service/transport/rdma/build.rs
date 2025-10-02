fn main() {
    cc::Build::new()
        .file("src/ffi/rdma_client.c")
        .compile("rdma_client");
}
