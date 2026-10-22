//! 🔌️ The wasmtime port of the plugin host: the only module of the codebase that names the `wasmtime` and `wasmtime-wasi` crates. The other host layers reach the embedding API through the explicit re-exports below, so the engine version and its feature pin live in this crate's manifest alone. See [`super`].

pub use wasmtime::{Config, Engine, Error, InstanceAllocationStrategy, PoolingAllocationConfig, ResourceLimiter, Result, Store, StoreContextMut, UpdateDeadline};

/// 🧩️ The component-model embedding API of the host layers: components, linkers, resource tables, async accessors and streams, and the WIT binding generator.
pub mod component {
    pub use wasmtime::component::{Accessor, Component, Destination, HasSelf, Linker, Resource, ResourceTable, StreamProducer, StreamReader, StreamResult, VecBuffer, bindgen};
}

/// 🪟️ The WASI preview 2 embedding API of the host layers: guest contexts, their views and the linker registration.
pub mod wasi {
    pub use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

    /// 🚰️ The preview 2 half: linker registration and the in-memory output pipe.
    pub mod p2 {
        pub use wasmtime_wasi::p2::add_to_linker_async;

        /// 📤️ In-memory guest output.
        pub mod pipe {
            pub use wasmtime_wasi::p2::pipe::MemoryOutputPipe;
        }
    }
}

/// 🧬️ The crate path generated code resolves through: pass it as `wasmtime_crate` to `bindgen!`, whose expansion and derives reach hidden wasmtime internals that no explicit list can enumerate. Generated code only; hand-written code uses the re-exports above.
#[doc(hidden)]
pub use wasmtime as generated_code_support;
