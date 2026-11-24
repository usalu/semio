// 🐛️ `additional_derives` intentionally omitted — see the identical note in
// `🔌️plugin/🖥️host/🦀️.rs`'s own `mod actor_bindings`: wasmtime-wit-bindgen 22.0.1
// hand-writes `Debug` for every WIT record/variant/enum regardless, so requesting it again here
// would conflict.
wasmtime::component::bindgen!({
    world: "actor",
    path: "../../../🧬️schema",
    exports: {
        "semio:framework/reactor.stage-actor-context-budget": async | store,
        "semio:framework/reactor.stage-cold-pair-page": async | store,
        "semio:framework/jobs.stage-job-budget": async | store,
    },
});
