//! ♻️ Plugin store lifecycle: store disposers, bounded document store initialization, snapshot close leases and original store genesis.
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

#[path = "../🦀️.rs"]
pub mod store_lifecycle;
#[path = "../../🌱️initialization/🌱️genesis/🦀️.rs"]
pub mod original_store_genesis;
