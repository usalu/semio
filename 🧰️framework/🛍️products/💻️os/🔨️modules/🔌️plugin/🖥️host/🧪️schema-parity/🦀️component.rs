//! 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): contract-parity test over the
//! ONE world `🧬️schema/📜️component.wit` now declares. Two invariants, both mechanical:
//!
//! 1. **Effect ↔ import parity** — every completable `effects.effect` case (the poll round trip)
//!    has a matching awaitable `host-async` import taking the SAME `*-params` record, so the two
//!    calling conventions inside one world can never drift apart.
//! 2. **The collapsed shape itself** — exactly one world (`actor`), importing exactly `pure` +
//!    `host-async`, with all seven of its exports declared `async func` and the deliberate sync
//!    survivors (`pure`'s three, `host-async`'s `emit`/`emit-patch`) still sync.
//!
//! Parses the WIT with `wit-parser` directly — the same crate BOTH generators
//! (`wasmtime::component::bindgen!`, `wit_bindgen::generate!`) resolve the whole package through —
//! rather than compiling a component, so a drift fails a native `cargo test` immediately instead of
//! surfacing later as a runtime ABI mismatch nobody notices until a real host/guest pairing traps.
//!
//! Mounted from `📦️glue.rs` behind `#[cfg(test)]` — never compiled into the shipped crate, and
//! deliberately NOT added to `🦀️component.rs`, which other packets in this ticket are live in.

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use wit_parser::{Field, Function, FunctionKind, InterfaceId, PackageId, Resolve, Type, TypeDefKind, World, WorldId, WorldKey};

    /// 📂️ `🧬️schema/`, resolved relative to THIS crate's manifest dir (`📦️packages/🦀️rust`) — the
    /// exact same relative path `actor_bindings`'s `wasmtime::component::bindgen!` call in
    /// `🦀️component.rs` already uses, so a directory move breaks both at once rather than silently
    /// diverging.
    async fn schema_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../🧬️schema")
    }

    /// 🌊️ `http-request` is the one effect case whose async import is deliberately renamed —
    /// `http-fetch` reads better for a call that returns a response, and the packet brief
    /// introduces the pair exactly this way. Every other completable effect keeps its case name
    /// verbatim as its `host-async` function name.
    async fn async_fn_name_for(effect_case: &str) -> &str {
        match effect_case {
            "http-request" => "http-fetch",
            other => other,
        }
    }

    /// 🚪️ `respond` is the one `req`-bearing effect that deliberately does NOT get a `host-async`
    /// import: it answers a HOST-issued `request` event, it does not await a HOST-issued completion
    /// of its own, so it stays reachable only through `emit` (`effects.wit`'s `respond-effect` doc
    /// comment explains the direction). Any OTHER `req`-bearing case failing this test's generic
    /// rule is a real drift, not a documented exception.
    async fn is_documented_emit_only_exception(effect_case: &str) -> bool {
        effect_case == "respond"
    }

    async fn record_fields<'a>(resolve: &'a Resolve, ty: &Type) -> &'a [Field] {
        let Type::Id(id) = ty else { panic!("expected a named record type, found {ty:?}") };
        match &resolve.types[*id].kind {
            TypeDefKind::Record(record) => &record.fields,
            other => panic!("expected a record, found {other:?}"),
        }
    }

    /// 🧷️ Resolves a [`Type`] through the alias chain that `use` creates, down to the `TypeDef` that
    /// actually defines it. `use effects.{blob-load-params}` does NOT reuse the original `TypeId` —
    /// `wit-parser` materialises a fresh `TypeDef` whose kind is `TypeDefKind::Type(original)`. So two
    /// genuinely-shared types compare UNEQUAL by raw id, and only their roots may be compared. Without
    /// this, these tests reported drift for a schema that is correct: `host-async` really does
    /// `use effects.{…-params, effect}`, which is exactly the sharing the `*-params` refactor exists
    /// to guarantee.
    async fn canonical_type(resolve: &Resolve, ty: Type) -> Type {
        let mut current = ty;
        loop {
            let Type::Id(id) = current else { return current };
            match &resolve.types[id].kind {
                TypeDefKind::Type(inner) => current = *inner,
                _ => return Type::Id(id),
            }
        }
    }

    /// 🧬️ One parse of `🧬️schema/📜️component.wit` per test, kept cheap and side-effect-free — this
    /// never touches the filesystem outside `🧬️schema/`, never writes, and never depends on a prior
    /// `cargo build` having run.
    struct SchemaFixture {
        resolve: Resolve,
        package: PackageId,
    }

    impl SchemaFixture {
        async fn load() -> Self {
            let mut resolve = Resolve::new();
            let (package, _sources) = resolve.push_path(schema_dir().await).expect(
                "🧬️schema/📜️component.wit must parse under wit-parser 0.252.0 — the exact version \
                 wasmtime 47.0.3 (this crate's own `wasmtime` dependency) resolves this package with",
            );
            Self { resolve, package }
        }

        async fn interface(&self, name: &str) -> InterfaceId {
            *self.resolve.packages[self.package].interfaces.get(name).unwrap_or_else(|| panic!("interface `{name}` must exist in 🧬️schema/📜️component.wit"))
        }

        async fn world(&self, name: &str) -> WorldId {
            *self.resolve.packages[self.package].worlds.get(name).unwrap_or_else(|| panic!("world `{name}` must exist in 🧬️schema/📜️component.wit"))
        }

        async fn effect_variant_cases(&self) -> &[wit_parser::Case] {
            let effects = &self.resolve.interfaces[self.interface("effects").await];
            let effect_type_id = *effects.types.get("effect").expect("variant `effect` must exist in `effects`");
            match &self.resolve.types[effect_type_id].kind {
                TypeDefKind::Variant(variant) => &variant.cases,
                other => panic!("`effects.effect` must be a variant, found {other:?}"),
            }
        }
    }

    /// 🎯️ THE test: every `effects.effect` case whose payload record carries `req: request-id` has
    /// a same-named (modulo the one documented rename) `async func` in `host-async` whose params
    /// are EXACTLY that record's own fields minus `req` — proving the async import reuses the
    /// `*-params` payload rather than inventing a parallel shape. Every `req`-bearing case without a
    /// `host-async` counterpart must be the one documented `respond` exception. This also
    /// automatically flags any FUTURE completable effect added to `effects.effect` that forgets its
    /// `host-async` counterpart.
    #[semio_framework_async_macros::async_test]
    async fn every_req_bearing_effect_has_a_matching_host_async_import() {
        let fixture = SchemaFixture::load().await;
        let host_async = &fixture.resolve.interfaces[fixture.interface("host-async").await];

        let mut checked = BTreeSet::new();
        for case in fixture.effect_variant_cases().await {
            let Some(payload_ty) = &case.ty else { continue };
            let fields = record_fields(&fixture.resolve, payload_ty).await;
            let has_req = fields.iter().any(|field| field.name == "req");
            if !has_req {
                continue;
            }

            if is_documented_emit_only_exception(&case.name).await {
                assert!(
                    !host_async.functions.contains_key(case.name.as_str()),
                    "`{}` is documented as emit-only (answers a host-issued request, never awaits a \
                     host-issued completion of its own) but a `host-async` function of the same name \
                     now exists — either the exception is stale or this is a real drift",
                    case.name
                );
                continue;
            }

            let async_name = async_fn_name_for(&case.name).await;
            let function: &Function = host_async.functions.get(async_name).unwrap_or_else(|| {
                panic!(
                    "effect case `{}` carries `req: request-id` but `host-async` has no `{}` async \
                     import — every completable effect needs a matching host-async counterpart",
                    case.name, async_name
                )
            });
            assert_eq!(function.kind, FunctionKind::AsyncFreestanding, "`host-async.{async_name}` must be declared `async func`");

            let expected_params: Vec<&Field> = fields.iter().filter(|field| field.name != "req").collect();
            assert_eq!(function.params.len(), expected_params.len(), "`host-async.{async_name}` param count must match `{}`'s fields minus `req`", case.name);
            for (param, field) in function.params.iter().zip(expected_params.iter()) {
                assert_eq!(&param.name, &field.name, "`host-async.{async_name}` param name must match `{}.{}`'s field name", case.name, field.name);
                assert_eq!(
                    canonical_type(&fixture.resolve, param.ty).await,
                    canonical_type(&fixture.resolve, field.ty).await,
                    "`host-async.{async_name}` param `{}` must reuse the SAME type as `{}.{}` — a new \
                     type here would be exactly the drift the `*-params` refactor exists to prevent",
                    param.name,
                    case.name,
                    field.name
                );
            }

            checked.insert(case.name.clone());
        }

        // 🧬️ Positive sanity: the ~21 canonical `*-params`-wrapped effects really were exercised
        // above, not silently skipped by an empty `effect` variant or a payload-resolution bug.
        for expected in [
            "storage-read",
            "storage-write",
            "storage-delete",
            "blob-load",
            "blob-write",
            "http-request",
            "document-read",
            "document-write",
            "link-resolve",
            "registry-query",
            "io-compose",
            "io-run",
            "cache-derive",
            "cache-read",
            "invoke-extension",
            "open-window",
            "open-dialog",
            "dispatch-action",
            "spawn-plugin-instance",
            "request-file-open",
            "request-media-frames",
            "request-capability",
        ] {
            assert!(checked.contains(expected), "expected effect case `{expected}` to have been checked");
        }
    }

    /// 🧬️ `spawn-job` is covered separately: `spawn-job-effect` was never wrapped in a `*-params`
    /// record (it correlates by `job: u64`, not `req: request-id`), so it falls outside the generic
    /// req-driven rule above. The packet brief still specifies an async import for it, taking its
    /// exact existing fields.
    #[semio_framework_async_macros::async_test]
    async fn spawn_job_has_a_matching_host_async_import_despite_carrying_no_req() {
        let fixture = SchemaFixture::load().await;
        let host_async = &fixture.resolve.interfaces[fixture.interface("host-async").await];

        let case = fixture.effect_variant_cases().await.iter().find(|case| case.name == "spawn-job").expect("`spawn-job` case must exist in `effects.effect`");
        let fields = record_fields(&fixture.resolve, case.ty.as_ref().expect("spawn-job carries a payload")).await;
        assert!(
            !fields.iter().any(|field| field.name == "req"),
            "this test's premise (`spawn-job-effect` carries no `req`) is stale — merge it into the \
             generic req-driven test above instead"
        );

        let function = host_async.functions.get("spawn-job").expect("`host-async.spawn-job` must exist");
        assert_eq!(function.kind, FunctionKind::AsyncFreestanding);
        assert_eq!(function.params.len(), fields.len());
        for (param, field) in function.params.iter().zip(fields.iter()) {
            assert_eq!(&param.name, &field.name);
            assert_eq!(canonical_type(&fixture.resolve, param.ty).await, canonical_type(&fixture.resolve, field.ty).await);
        }
    }

    /// 🚪️ Every `effect` case — regardless of whether it carries `req` — is reachable through
    /// `host-async.emit`, which takes the whole `effect` variant as its one argument rather than a
    /// hand-written signature per case. This is the "no `req` ⇒ reachable through `emit`" half of
    /// the contract, and it also covers `respond`.
    #[semio_framework_async_macros::async_test]
    async fn emit_carries_the_whole_effect_variant() {
        let fixture = SchemaFixture::load().await;
        let effects = &fixture.resolve.interfaces[fixture.interface("effects").await];
        let host_async = &fixture.resolve.interfaces[fixture.interface("host-async").await];
        let effect_type_id = *effects.types.get("effect").expect("variant `effect` must exist");

        let emit: &Function = host_async.functions.get("emit").expect("`host-async.emit` must exist");
        assert_eq!(emit.kind, FunctionKind::Freestanding, "`emit` must be fire-and-forget, not `async func`");
        assert_eq!(emit.params.len(), 1, "`emit` must take exactly the `effect` variant");
        assert_eq!(canonical_type(&fixture.resolve, emit.params[0].ty).await, canonical_type(&fixture.resolve, Type::Id(effect_type_id)).await, "`emit`'s parameter must be THE SAME `effect` type `effects.effect` defines, not a copy");

        assert!(host_async.functions.contains_key("emit-patch"), "`host-async.emit-patch` must exist");
    }

    /// 🌍️ B1 world-collapse: `actor` is not merely still present, it is the ONLY world. `world
    /// actor-async` and `interface runner` were deleted, and a future packet that resurrects a
    /// second world — or leaves a stray probe/spike world behind in the live schema — fails here
    /// immediately rather than at a bindgen call site nobody re-reads.
    #[semio_framework_async_macros::async_test]
    async fn exactly_one_world_exists() {
        let fixture = SchemaFixture::load().await;
        let worlds: BTreeSet<String> = fixture.resolve.packages[fixture.package].worlds.keys().cloned().collect();
        assert_eq!(worlds, BTreeSet::from(["actor".to_string()]), "`semio:framework` must declare exactly one world, `actor` — B1 world-collapse deleted `actor-async`");
        assert!(
            !fixture.resolve.packages[fixture.package].interfaces.contains_key("runner"),
            "`interface runner` was deleted with `world actor-async` — the collapsed world's turn-loop entry point is `reactor::poll`, not a long-lived `stream<event>`"
        );
    }

    /// 🌍️ The one world's export surface is unchanged by the collapse; its FUNCTIONAL import
    /// surface is the actual change — `{pure}` became `{pure, host-async}`.
    ///
    /// 🧬️ The import check is deliberately NOT "the import set equals `{pure, host-async}`" —
    /// `wit-parser` also surfaces every interface an EXPORTED interface's function signatures merely
    /// reference types from (`reactor` → `use types/effects/events/ui`, `events` → `use
    /// capabilities` too) as a `WorldItem::Interface` in `world.imports`, even though none of those
    /// interfaces declare a single `func` for a host to implement. That is `wit-parser` doing
    /// exactly what it must to resolve the exported functions' argument/return types — it is not
    /// this world quietly growing a new host-callable import. The invariant the docs mean is
    /// function-level: no interface OTHER than `pure`/`host-async` contributes a callable
    /// `func`/`async func`.
    #[semio_framework_async_macros::async_test]
    async fn world_actor_exports_and_imports_exactly() {
        let fixture = SchemaFixture::load().await;
        let actor = &fixture.resolve.worlds[fixture.world("actor").await];

        // 🚫️async: E1 pure accessor, consumed only from sync closures (`export_names`/
        // `functional_import_names`/`type_only_import_names` below build `BTreeSet::collect()` over
        // a plain iterator) — see R9.
        fn world_key_name(resolve: &Resolve, key: &WorldKey) -> String {
            match key {
                WorldKey::Name(name) => name.clone(),
                WorldKey::Interface(id) => resolve.interfaces[*id].name.clone().unwrap_or_default(),
            }
        }

        let export_names = |world: &World| -> BTreeSet<String> { world.exports.keys().map(|key| world_key_name(&fixture.resolve, key)).collect() };

        // 🚪️ Every import whose interface declares at least one function — the ONLY imports a
        // generated `Host` trait actually needs an `impl` for.
        let functional_import_names = |world: &World| -> BTreeSet<String> {
            world
                .imports
                .iter()
                .filter_map(|(key, item)| {
                    let wit_parser::WorldItem::Interface { id, .. } = item else { return None };
                    let has_functions = !fixture.resolve.interfaces[*id].functions.is_empty();
                    has_functions.then(|| world_key_name(&fixture.resolve, key))
                })
                .collect()
        };
        // 🧬️ The complementary set: imports present ONLY because an export's function signature
        // references one of their types, never because a host must implement a function.
        let type_only_import_names = |world: &World| -> BTreeSet<String> {
            world
                .imports
                .iter()
                .filter_map(|(key, item)| {
                    let wit_parser::WorldItem::Interface { id, .. } = item else { return None };
                    let has_functions = !fixture.resolve.interfaces[*id].functions.is_empty();
                    (!has_functions).then(|| world_key_name(&fixture.resolve, key))
                })
                .collect()
        };

        let expected_exports: BTreeSet<String> = ["reactor", "jobs", "checkpoint", "describe"].into_iter().map(String::from).collect();
        assert_eq!(export_names(actor), expected_exports, "the collapse changed `world actor`'s IMPORTS, never its exports");
        assert_eq!(functional_import_names(actor), BTreeSet::from(["pure".to_string(), "host-async".to_string()]), "`world actor` must import exactly `pure` and `host-async` for anything callable — this is the collapse's actual functional change");
        // 🧬️ Positive sanity: this world DOES pull in type-only interfaces transitively (proving
        // the distinction above is real, not vacuously true because nothing showed up here).
        assert!(!type_only_import_names(actor).is_empty(), "expected `world actor` to have at least one type-only implicit import");
    }

    /// ⚡️ S7, reproduced on this ticket: a plain sync `func` EXPORT is uncallable on any Store
    /// configured with `wasm_component_model_async(true)`, and `world actor`'s `host-async` import
    /// REQUIRES such a Store — so all seven exports had to become `async func` together. Scoped
    /// deliberately to exports-of-`world actor`, NOT "every func in the package": `pure`'s three
    /// funcs and `host-async`'s `emit`/`emit-patch` stay plain `func` by design (no suspension
    /// point / deliberate one-way doors), so a package-wide assertion would be actively wrong.
    #[semio_framework_async_macros::async_test]
    async fn every_export_of_world_actor_is_async_func() {
        let fixture = SchemaFixture::load().await;
        let actor = &fixture.resolve.worlds[fixture.world("actor").await];

        let mut seen: BTreeSet<String> = BTreeSet::new();
        for item in actor.exports.values() {
            let wit_parser::WorldItem::Interface { id, .. } = item else { panic!("`world actor` must export interfaces only, never a bare func or type") };
            let interface = &fixture.resolve.interfaces[*id];
            let interface_name = interface.name.clone().unwrap_or_default();
            for function in interface.functions.values() {
                assert_eq!(function.kind, FunctionKind::AsyncFreestanding, "`{interface_name}.{}` must be declared `async func` — a sync export is uncallable on the async-configured Store `host-async` requires (S7)", function.name);
                seen.insert(format!("{interface_name}.{}", function.name));
            }
        }

        // 🧬️ Positive sanity: all seven exports really were walked, not silently skipped by an
        // empty export map or a `WorldItem` shape this loop does not recognise.
        let expected: BTreeSet<String> = ["reactor.poll", "jobs.start-job", "jobs.step-job", "jobs.cancel-job", "checkpoint.checkpoint", "checkpoint.restore", "describe.describe"].into_iter().map(String::from).collect();
        assert_eq!(seen, expected, "`world actor`'s export surface is exactly these seven functions");

        // 🚫️ The deliberate sync survivors, asserted explicitly so a future blanket "make every
        // func async" sweep cannot quietly take them along.
        let pure = &fixture.resolve.interfaces[fixture.interface("pure").await];
        for name in ["log", "now-ms", "trace-span"] {
            let function = pure.functions.get(name).unwrap_or_else(|| panic!("`pure.{name}` must exist"));
            assert_eq!(function.kind, FunctionKind::Freestanding, "`pure.{name}` stays sync — no I/O, no suspension point");
        }
    }

    /// 🛡️ Every `host-async` import except the two documented fire-and-forget doors returns a
    /// `result<_, _>`. A future import declared with a bare return type would let a real host-side
    /// fault decode as that type's Rust default (an empty `Vec<u8>` instead of a propagated error)
    /// rather than failing loud — a silent data-correctness bug this test turns into a test-time
    /// failure. `emit`/`emit-patch` are the exceptions and are asserted to return NOTHING.
    #[semio_framework_async_macros::async_test]
    async fn every_fallible_host_async_import_returns_a_result() {
        let fixture = SchemaFixture::load().await;
        let host_async = &fixture.resolve.interfaces[fixture.interface("host-async").await];

        let mut checked = 0usize;
        for function in host_async.functions.values() {
            if matches!(function.name.as_str(), "emit" | "emit-patch") {
                assert!(function.result.is_none(), "`host-async.{}` is a deliberate one-way door — it must return nothing", function.name);
                assert_eq!(function.kind, FunctionKind::Freestanding, "`host-async.{}` is fire-and-forget, not `async func`", function.name);
                continue;
            }
            assert_eq!(function.kind, FunctionKind::AsyncFreestanding, "`host-async.{}` must be declared `async func`", function.name);
            let result = function.result.unwrap_or_else(|| panic!("`host-async.{}` must return a `result<_, _>`, not nothing", function.name));
            let Type::Id(id) = canonical_type(&fixture.resolve, result).await else { panic!("`host-async.{}` must return a `result<_, _>`, found a primitive", function.name) };
            assert!(matches!(fixture.resolve.types[id].kind, TypeDefKind::Result(_)), "`host-async.{}` must return a `result<_, _>` so a host fault propagates instead of decoding as a default value", function.name);
            checked += 1;
        }
        assert_eq!(checked, 24, "expected all 24 fallible `host-async` imports to have been checked");
    }
}
