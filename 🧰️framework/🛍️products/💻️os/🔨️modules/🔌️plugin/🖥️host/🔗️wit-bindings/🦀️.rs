//! 🔗️ WIT bindings of the actor world and the kernel to WIT conversions every plugin-host layer shares: the crate's one `bindgen!` invocation, its aliases, the cold pair, UI patch and retained turn wire modules.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;

pub mod actor_bindings {
    // 🐛️ `additional_derives` carries NEITHER `Debug` NOR `Clone`, each for its own reason.
    // `Debug`: wasmtime-wit-bindgen's `type_record`/`print_rust_enum` always emit a hand-written
    // `impl core::fmt::Debug` for every WIT `record`/`variant`/`enum` regardless of
    // `additional_derives`, so requesting it produces `E0119: conflicting implementations` for
    // every generated type — it is already available on all of them.
    // `Clone`: MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse) — `world actor` now
    // imports `host-async`, which carries `stream<u8>` (`blob-read`, `http-fetch`'s
    // `http-response.body`). That lowers to `StreamReader<u8>`, a one-shot resource handle that is
    // deliberately not `Clone`, and `additional_derives` applies blanket to every generated type
    // rather than only the plain data records. Nothing in this crate cloned a generated `wit_*`
    // value (verified by grep before removing the derive — every `.clone()` near a `wit_*`
    // constructor is on the KERNEL-side `String`/`Vec<u8>` being moved INTO it).
    //
    // 🧬️ This is the crate's ONE `bindgen!` invocation for `semio:framework` — `📥️imports/🦀️.rs`'s 24
    // `host-async` implementations re-export this module rather than generating a second, nominally
    // distinct copy of every type (which is what the two-world split forced before the collapse).
    wasmtime::component::bindgen!({
        world: "actor",
        path: "../../../../🧬️schema",
    });
}

pub use actor_bindings::semio::framework::{capabilities as wit_capabilities, effects as wit_effects, events as wit_events, host_async as wit_host_async, instance_lifetime as wit_lifetime, types as wit_types, ui as wit_ui};
pub use actor_bindings::exports::semio::framework::{jobs as wit_jobs, reactor as wit_reactor};

#[path = "🔀️conversion/🦀️.rs"]
pub mod conversion;
#[path = "../📥️cold-pair/🦀️.rs"]
pub mod cold_pair;
#[path = "../📥️ui-patch/🦀️.rs"]
pub mod ui_patch;
#[path = "../🧬️component-codec/🎟️retained-turn/🦀️.rs"]
pub mod retained_turn_wire;

#[cfg(test)]
#[path = "../🧪️tests/💡️service-operation/🦀️.rs"]
mod service_operation_tests;

pub use retained_turn_wire::jobs::{ack_from_wit, budget_to_wit, cancel_from_wit, output_from_wit, source_to_wit, start_from_wit, step_from_wit};
