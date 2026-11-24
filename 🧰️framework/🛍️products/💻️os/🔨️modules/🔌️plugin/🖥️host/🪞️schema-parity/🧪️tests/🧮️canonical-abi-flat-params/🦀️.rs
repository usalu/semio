//! 🧮️ Canonical-ABI flat-parameter law of the plugin WIT schemas: an async-lifted export whose parameters flatten to more than 16 core values is called through an indirect parameter area that `wit-bindgen` never frees (bytecodealliance/wit-bindgen#1757), so every such call leaked guest linear memory. The law reads the schema, never a language binding, and validates its own flattening against `wit-parser`, an independent canonical-ABI implementation.
mod tests {
    use semio_framework_test_fixtures::fixture_path;
    use std::collections::BTreeMap;
    use wit_parser::abi::AbiVariant;
    use wit_parser::{Function, Resolve, Type, TypeDefKind, WorldItem};

    /// 🔢️ The canonical ABI's `MAX_FLAT_PARAMS`, which also bounds the parameters of an async-lifted export (`MAX_FLAT_ASYNC_PARAMS` is 4 and bounds only async-lowered imports, whose parameter area their guest caller owns and frees).
    const MAX_FLAT_ASYNC_EXPORT_PARAMS: usize = 16;

    /// 📥️ Parses every schema file of the plugin WIT directory with `wit-parser` and returns the exported functions of `world actor` keyed `interface.function`.
    fn exported_functions() -> (Resolve, BTreeMap<String, Function>) {
        let mut resolve = Resolve::default();
        let (package, _) = resolve.push_dir(fixture_path!("../../../🧬️schema")).expect("plugin WIT schemas must parse");
        let world = resolve.select_world(&[package], Some("actor")).expect("world actor must exist");
        let mut functions = BTreeMap::new();
        for (key, item) in &resolve.worlds[world].exports {
            let WorldItem::Interface { id, .. } = item else { panic!("world actor exports only interfaces, found `{}`", resolve.name_world_key(key)) };
            let interface = &resolve.interfaces[*id];
            let name = interface.name.as_deref().expect("exported interfaces are named");
            functions.extend(interface.functions.iter().map(|(function, value)| (format!("{name}.{function}"), value.clone())));
        }
        assert!(!functions.is_empty(), "world actor must export functions");
        (resolve, functions)
    }

    /// 🧮️ Number of core values `ty` flattens to under the canonical ABI, written from the specification rather than from `wit-parser`: scalars and handles are one value, strings and lists two, records and tuples the sum of their fields, and a variant-shaped type one discriminant plus the widest case.
    fn flat_count(resolve: &Resolve, ty: &Type) -> usize {
        let Type::Id(id) = ty else { return if matches!(ty, Type::String) { 2 } else { 1 } };
        match &resolve.types[*id].kind {
            TypeDefKind::Type(inner) => flat_count(resolve, inner),
            TypeDefKind::Record(record) => record.fields.iter().map(|field| flat_count(resolve, &field.ty)).sum(),
            TypeDefKind::Tuple(tuple) => tuple.types.iter().map(|inner| flat_count(resolve, inner)).sum(),
            TypeDefKind::Variant(variant) => 1 + variant.cases.iter().map(|case| case.ty.as_ref().map_or(0, |inner| flat_count(resolve, inner))).max().unwrap_or(0),
            TypeDefKind::Option(inner) => 1 + flat_count(resolve, inner),
            TypeDefKind::Result(result) => 1 + [result.ok.as_ref(), result.err.as_ref()].into_iter().flatten().map(|inner| flat_count(resolve, inner)).max().unwrap_or(0),
            TypeDefKind::List(_) | TypeDefKind::Map(..) => 2,
            TypeDefKind::FixedLengthList(inner, length) => flat_count(resolve, inner) * *length as usize,
            TypeDefKind::Flags(flags) => flags.flags.len().div_ceil(32),
            TypeDefKind::Enum(_) | TypeDefKind::Handle(_) | TypeDefKind::Future(_) | TypeDefKind::Stream(_) => 1,
            TypeDefKind::Resource | TypeDefKind::Unknown => panic!("`{}` is not a parameter type", resolve.types[*id].kind.as_str()),
        }
    }

    fn parameter_count(resolve: &Resolve, function: &Function) -> usize {
        function.params.iter().map(|parameter| flat_count(resolve, &parameter.ty)).sum()
    }

    #[test]
    fn every_async_export_flattens_to_at_most_sixteen_core_params_and_is_called_directly() {
        let (resolve, functions) = exported_functions();
        let mut checked = 0usize;
        for (name, function) in functions.iter().filter(|(_, function)| function.kind.is_async()) {
            let count = parameter_count(&resolve, function);
            assert!(count <= MAX_FLAT_ASYNC_EXPORT_PARAMS, "async export `{name}` flattens to {count} core params, above {MAX_FLAT_ASYNC_EXPORT_PARAMS}: its indirect parameter area leaks guest memory on every call, stage the large record through a sync export instead");
            let signature = resolve.wasm_signature(AbiVariant::GuestExportAsync, function);
            assert!(!signature.indirect_params, "wit-parser passes async export `{name}` through an indirect parameter area");
            assert_eq!(signature.params.len(), count, "wit-parser and the specification count different core params for `{name}`");
            checked += 1;
        }
        assert!(checked >= 30, "the law must cover the whole async export surface, found {checked}");
    }

    #[test]
    fn the_own_flattening_agrees_with_wit_parser_on_every_exported_function() {
        let (resolve, functions) = exported_functions();
        for (name, function) in &functions {
            let variant = if function.kind.is_async() { AbiVariant::GuestExportAsync } else { AbiVariant::GuestExport };
            let signature = resolve.wasm_signature(variant, function);
            let count = parameter_count(&resolve, function);
            assert_eq!(count > MAX_FLAT_ASYNC_EXPORT_PARAMS, signature.indirect_params, "`{name}` flattens to {count} core params but wit-parser reports indirect={}", signature.indirect_params);
            if !signature.indirect_params {
                assert_eq!(signature.params.len(), count, "`{name}` core param count differs from wit-parser");
            }
        }
    }

    #[test]
    fn only_the_sync_stages_take_an_indirect_parameter_area() {
        let (resolve, functions) = exported_functions();
        let indirect = functions.iter().filter(|(_, function)| parameter_count(&resolve, function) > MAX_FLAT_ASYNC_EXPORT_PARAMS).map(|(name, function)| (name.as_str(), function.kind.is_async())).collect::<Vec<_>>();
        assert_eq!(indirect, vec![("jobs.stage-job-budget", false), ("reactor.stage-actor-context-budget", false), ("reactor.stage-cold-pair-page", false)]);
    }
}
