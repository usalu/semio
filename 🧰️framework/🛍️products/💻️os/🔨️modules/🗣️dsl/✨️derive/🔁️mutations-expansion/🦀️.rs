//! 🔁️ Mutations expansion of the dsl derive expansion: the mutations attribute parser, the derive mutations expansion, the mutations expansion with its payload law test and the relative path helper. See [`super`].

use quote::quote;
use super::{Data, DeriveInput, Fields, MutationAggregateSourceAuthority, Path, TokenStream, Type, mutation_aggregate_source_authority, mutation_authority_canonical, mutation_authority_relative, mutation_authority_workspace_token, mutation_leaf_include_path, parse_macro_input, to_kebab};

//#region 🔖️Mutations
/// 🗣️ `#[mutations(snapshot = ..., diff = ..., schema = "..." [, retire_cold = path])]` container
/// attrs for `#[derive(Mutations)]` — see that macro's doc. `retire_cold` names a `fn(Self)` that
/// disposes an operation owning fail-closed roots; the generated `Mutation::retire_cold` calls it.
#[derive(Default)]
struct MutationsAttrs {
    pub(super) snapshot: Option<Type>,
    pub(super) diff: Option<Type>,
    pub(super) schema: Option<String>,
    retire_cold: Option<syn::Path>,
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
#[doc(hidden)] pub fn parse_mutations_attrs(input: &DeriveInput) -> syn::Result<MutationsAttrs> {
    let mut out = MutationsAttrs::default();
    for attr in &input.attrs {
        if !attr.path().is_ident("mutations") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("snapshot") {
                if out.snapshot.is_some() { return Err(meta.error("duplicate mutations snapshot")); }
                out.snapshot = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("diff") {
                if out.diff.is_some() { return Err(meta.error("duplicate mutations diff")); }
                out.diff = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("schema") {
                if out.schema.is_some() { return Err(meta.error("duplicate mutations schema")); }
                let value: syn::LitStr = meta.value()?.parse()?;
                out.schema = Some(value.value());
            } else if meta.path.is_ident("retire_cold") {
                if out.retire_cold.is_some() { return Err(meta.error("duplicate mutations retire_cold")); }
                out.retire_cold = Some(meta.value()?.parse()?);
            } else { return Err(meta.error("unsupported mutations attribute")); }
            Ok(())
        })?;
    }
    Ok(out)
}

pub fn expand_derive_mutations(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let source = match input.ident.span().unwrap().local_file() {
        Some(source) => source,
        None => return syn::Error::new_spanned(&input, "Mutations requires a local source file").to_compile_error().into(),
    };
    let compiler_cwd = match std::env::current_dir() {
        Ok(path) => path,
        Err(error) => return syn::Error::new_spanned(&input, error.to_string()).to_compile_error().into(),
    };
    let authority = match mutation_aggregate_source_authority(&source, &compiler_cwd) {
        Ok(authority) => authority,
        Err(error) => return syn::Error::new_spanned(&input, format!("Mutations source authority failed: {error}")).to_compile_error().into(),
    };
    match expand_mutations(&input, &authority) {
        Ok(expanded) => expanded.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[doc(hidden)] pub fn expand_mutations(input: &DeriveInput, authority: &MutationAggregateSourceAuthority) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(input, "#[derive(Mutations)] only supports enums"));
    };
    if data.variants.is_empty() {
        return Err(syn::Error::new_spanned(input, "Mutations requires at least one concrete leaf"));
    }
    if input.attrs.iter().any(|attr| attr.path().is_ident("cfg") || attr.path().is_ident("cfg_attr")) {
        return Err(syn::Error::new_spanned(input, "Mutations does not permit conditional aggregate metadata"));
    }
    let attrs = parse_mutations_attrs(input)?;
    let retire_cold = attrs.retire_cold.map(|retire| quote! {
        fn retire_cold(self) {
            #retire(self)
        }
    });
    let (Some(snapshot_ty), Some(diff_ty), Some(schema)) = (attrs.snapshot, attrs.diff, attrs.schema) else {
        return Err(syn::Error::new_spanned(input, "#[derive(Mutations)] requires #[mutations(snapshot = YourSnapshot, diff = YourDiff, schema = \"your.doc.schema\")]"));
    };
    let map_error = |error: String| syn::Error::new_spanned(input, error);
    let workspace_token = mutation_authority_workspace_token(&authority.workspace_root, &authority.taxonomy_path).map_err(map_error)?;
    let mutation_root = mutation_authority_relative(&authority.workspace_root, &authority.mutation_root).map_err(map_error)?;
    let taxonomy_path = mutation_authority_relative(&authority.workspace_root, &authority.taxonomy_path).map_err(map_error)?;
    let dependency_paths = [authority.taxonomy_path.clone()];
    let dependencies = dependency_paths.iter().map(|path| mutation_leaf_include_path(path)).collect::<Result<Vec<_>, _>>().map_err(map_error)?;
    let source_filename = &authority.source_filename;
    let descriptor_filename = &authority.descriptor_filename;
    let mutation_payload_facet = &authority.mutation_payload_facet;
    let owner_layout = if let Some(operations) = &authority.domain_operations {
        let entries = operations.iter().map(|(owner, identity)| quote! { ::protocol::MutationDomainOperation { owner: #owner, semantic_kind: #identity } });
        quote! { ::protocol::MutationOwnerLayout::DomainOperations(&[#(#entries),*]) }
    } else {
        quote! { ::protocol::MutationOwnerLayout::Flat }
    };
    let components = if authority.component_roots.is_empty() { vec![(mutation_root.clone(), authority.domain_operations.clone())] } else { authority.component_roots.clone() };
    let mut component_completeness = Vec::new();
    let scopes = components.iter().map(|(root, operations)| {
        let layout = if let Some(operations) = operations {
            let entries = operations.iter().map(|(owner, identity)| quote! { ::protocol::MutationDomainOperation { owner: #owner, semantic_kind: #identity } });
            for (owner, _) in operations {
                component_completeness.push(quote! {
                    let mut found = false;
                    let mut index = 0;
                    while index < descriptors.len() {
                        if ::semio_framework_os_command::str_eq(descriptors[index].owner, #owner) { found = true; }
                        index += 1;
                    }
                    assert!(found, "Mutations requires every explicitly registered component domain operation");
                });
            }
            quote! { ::protocol::MutationOwnerLayout::DomainOperations(&[#(#entries),*]) }
        } else { quote! { ::protocol::MutationOwnerLayout::Flat } };
        quote! {
            match (::protocol::MutationLeafSourceScope {
                workspace_token: [#(#workspace_token),*], mutation_root: #root, owner_layout: #layout,
                taxonomy_path: #taxonomy_path, mutation_payload_facet: #mutation_payload_facet,
                source_filename: #source_filename, descriptor_filename: #descriptor_filename,
            }).validate() {
                Ok(scope) => scope,
                Err(_) => panic!("Mutations requires a valid aggregate component source scope"),
            }
        }
    }).collect::<Vec<_>>();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let aggregate_ty = quote! { #name #ty_generics };
    let mut diff_arms = Vec::new();
    let mut inverse_arms = Vec::new();
    let mut timestamp_arms = Vec::new();
    let mut descriptor_arms = Vec::new();
    let mut semantics_arms = Vec::new();
    let mut label_arms = Vec::new();
    let mut target_arms = Vec::new();
    let mut may_emit_foreign_steps_arms = Vec::new();
    let mut foreign_source_arms = Vec::new();
    let mut input_schema_arms = Vec::new();
    let mut inverse_rows_arms = Vec::new();
    let mut payload_value_arms = Vec::new();
    let mut with_payload_value_arms = Vec::new();
    let mut from_payload_value_arms = Vec::new();
    let mut input_schemas = Vec::new();
    let mut input_schema_documents = Vec::new();
    let mut kind_consts = Vec::new();
    let mut leaf_descriptors = Vec::new();
    let mut leaf_checks = Vec::new();
    let mut conversions = Vec::new();
    let mut register_calls = Vec::new();

    for (index, variant) in data.variants.iter().enumerate() {
        let variant_ident = &variant.ident;
        let Fields::Unnamed(unnamed) = &variant.fields else {
            return Err(syn::Error::new_spanned(variant, "Mutations requires every variant to wrap exactly one direct MutationKind payload"));
        };
        if unnamed.unnamed.len() != 1 {
            return Err(syn::Error::new_spanned(variant, "Mutations requires every variant to wrap exactly one direct MutationKind payload"));
        }
        if variant.attrs.iter().chain(unnamed.unnamed[0].attrs.iter()).any(|attr| attr.path().is_ident("cfg") || attr.path().is_ident("cfg_attr")) {
            return Err(syn::Error::new_spanned(variant, "Mutations does not permit conditional leaf metadata"));
        }
        let payload_ty = &unnamed.unnamed[0].ty;
        if !matches!(payload_ty, Type::Path(path) if path.qself.is_none()) {
            return Err(syn::Error::new_spanned(payload_ty, "Mutations requires a direct leaf type path"));
        }
        let expected_variant = variant_ident.to_string();
        let expected_kebab = to_kebab(&expected_variant);
        let kind = quote! { <#payload_ty as ::semio_framework_os_command::MutationKind<#snapshot_ty, #aggregate_ty>> };
        let leaf = quote! { <#payload_ty as ::protocol::MutationLeaf> };
        diff_arms.push(quote! { Self::#variant_ident(payload) => #kind::diff(payload, base) });
        inverse_arms.push(quote! { Self::#variant_ident(payload) => #kind::inverse(payload, base) });
        timestamp_arms.push(quote! { Self::#variant_ident(payload) => #kind::timestamp(payload) });
        descriptor_arms.push(quote! { Self::#variant_ident(_) => &<Self as ::protocol::Mutation<#snapshot_ty>>::DESCRIPTORS[#index] });
        semantics_arms.push(quote! { Self::#variant_ident(_) => &#kind::SEMANTICS });
        label_arms.push(quote! { Self::#variant_ident(payload) => #kind::label(payload) });
        target_arms.push(quote! { Self::#variant_ident(payload) => #kind::target(payload) });
        may_emit_foreign_steps_arms.push(quote! { Self::#variant_ident(payload) => #kind::may_emit_foreign_steps(payload) });
        foreign_source_arms.push(quote! { Self::#variant_ident(payload) => #kind::foreign_step_source(payload, base, index) });
        input_schema_arms.push(quote! { Self::#variant_ident(payload) => #leaf::input_schema(payload) });
        inverse_rows_arms.push(quote! { Self::#variant_ident(payload) => #leaf::inverse_rows(payload) });
        payload_value_arms.push(quote! { Self::#variant_ident(payload) => #leaf::input_value(payload) });
        with_payload_value_arms.push(quote! { Self::#variant_ident(payload) => #leaf::with_input_value(payload, value).map(Self::#variant_ident) });
        from_payload_value_arms.push(quote! { if kind == #kind::SEMANTICS.kind { return #leaf::from_input_value(value).map(Self::#variant_ident); } });
        input_schemas.push(quote! { #leaf::PAYLOAD_SCHEMA });
        input_schema_documents.push(quote! { #leaf::PAYLOAD_SCHEMA_DOCUMENTS });
        kind_consts.push(quote! { #kind::SEMANTICS });
        leaf_descriptors.push(quote! { #leaf::DESCRIPTOR });
        leaf_checks.push(quote! { const {
            assert!(::semio_framework_os_command::str_eq(#kind::SEMANTICS.kind, #expected_kebab), "Mutations semantic kind must match its variant");
            assert!(::semio_framework_os_command::is_approved_verb(#kind::SEMANTICS.verb), "Mutations requires an approved semantic verb");
            assert!(::semio_framework_os_command::str_eq(#leaf::DESCRIPTOR.aggregate_variant, #expected_variant), "Mutations descriptor variant must match its wrapped leaf");
            assert!(::semio_framework_os_command::str_eq(#leaf::DESCRIPTOR.semantic_kind, #kind::SEMANTICS.kind), "Mutations descriptor and semantic kind must agree");
            let mut scope_index = 0;
            let mut matched = false;
            while scope_index < SCOPES.len() {
                if let Ok(()) = SCOPES[scope_index].validate_leaf(&#leaf::DESCRIPTOR, &#leaf::PROVENANCE) { matched = true; break; }
                scope_index += 1;
            }
            assert!(matched, "Mutations leaf source must match its aggregate workspace and declared component owner");
        }; });
        conversions.push(quote! {
            impl #impl_generics ::core::convert::From<#payload_ty> for #aggregate_ty #where_clause {
                fn from(payload: #payload_ty) -> Self {
                    let _ = <Self as ::protocol::Mutation<#snapshot_ty>>::DESCRIPTORS;
                    Self::#variant_ident(payload)
                }
            }
        });
        register_calls.push(quote! {
            ::semio_framework_os_command::MutationDescriptor::new(
                ::protocol::SchemaId(format!("{}#{}", #schema, #kind::SEMANTICS.kind)),
                ::protocol::SchemaVersion(1),
                state_class,
                #leaf::DESCRIPTOR,
                #kind::SEMANTICS,
            )?
        });
    }

    let register_fn_ident = syn::Ident::new(&format!("register_{}_descriptors", to_kebab(&name.to_string()).replace('-', "_")), name.span());
    let payload_law = mutation_payload_law_test(name, &snapshot_ty, authority, &input.generics);
    let eager_check = input.generics.params.is_empty().then(|| quote! {
        const _: () = { let _ = <#name as ::protocol::Mutation<#snapshot_ty>>::DESCRIPTORS; };
    });
    Ok(quote! {
        #(const _: &str = ::core::include_str!(#dependencies);)*
        #eager_check
        #(#conversions)*

        impl #impl_generics ::protocol::Mutation<#snapshot_ty> for #aggregate_ty #where_clause {
            type Diff = #diff_ty;
            const DESCRIPTORS: &'static [::protocol::MutationLeafDescriptor] = {
                const SCOPES: &[::protocol::ValidatedMutationLeafSourceScope] = &[#(#scopes),*];
                #(#leaf_checks)*
                let descriptors: &'static [::protocol::MutationLeafDescriptor] = &[#(#leaf_descriptors),*];
                #(#component_completeness)*
                match ::protocol::validate_mutation_leaf_descriptor_roster_uniqueness(#mutation_root, descriptors, #owner_layout) {
                    Ok(()) => descriptors,
                    Err(_) => panic!("Mutations requires a unique and complete direct leaf descriptor roster"),
                }
            };
            fn descriptor(&self) -> &'static ::protocol::MutationLeafDescriptor {
                match self { #(#descriptor_arms),* }
            }
            fn diff(&self, base: &#snapshot_ty) -> ::protocol::MutationOutcome<Self::Diff> {
                let _ = <Self as ::protocol::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#diff_arms),* }
            }
            fn inverse(&self, base: &#snapshot_ty) -> Result<Vec<Self>, ::semio_framework_value::ValueError> {
                let _ = <Self as ::protocol::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#inverse_arms),* }
            }
            fn timestamp(&self) -> Option<::protocol::HybridLogicalTimestamp> {
                let _ = <Self as ::protocol::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#timestamp_arms),* }
            }
            fn conflict_target(&self) -> Vec<String> {
                let _ = <Self as ::protocol::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#target_arms),* }
            }
            fn may_emit_foreign_steps(&self) -> bool {
                let _ = <Self as ::protocol::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#may_emit_foreign_steps_arms),* }
            }
            fn foreign_step_source<'a>(&'a self, base: &'a #snapshot_ty, index: usize) -> Result<Option<::protocol::ForeignStepSource<'a>>, ::semio_framework_value::ValueError> {
                let _ = <Self as ::protocol::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#foreign_source_arms),* }
            }
            fn inverse_rows(&self) -> usize {
                match self { #(#inverse_rows_arms),* }
            }
            const INPUT_SCHEMAS: &'static [&'static str] = &[#(#input_schemas),*];
            const INPUT_SCHEMA_DOCUMENTS: &'static [&'static [&'static str]] = &[#(#input_schema_documents),*];
            fn input_schema(&self) -> ::core::option::Option<&'static str> {
                match self { #(#input_schema_arms),* }
            }
            fn payload_value(&self) -> ::semio_framework_value::DslValue {
                match self { #(#payload_value_arms),* }
            }
            fn with_payload_value(&self, value: ::semio_framework_value::DslValue) -> ::core::result::Result<Self, ::semio_framework_value::ValueError> {
                match self { #(#with_payload_value_arms),* }
            }
            fn from_payload_value(kind: &str, value: ::semio_framework_value::DslValue) -> ::core::result::Result<Self, ::semio_framework_value::ValueError> {
                #(#from_payload_value_arms)*
                ::core::result::Result::Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,::std::format!("{kind} is no leaf kind of {}", ::core::stringify!(#name))))
            }
            #retire_cold
        }

        impl #impl_generics ::semio_framework_os_command::SemanticMutation<#snapshot_ty> for #aggregate_ty #where_clause {
            fn kinds() -> &'static [::semio_framework_os_command::SemanticDescriptor] {
                let _ = <Self as ::protocol::Mutation<#snapshot_ty>>::DESCRIPTORS;
                &[#(#kind_consts),*]
            }
            fn semantics(&self) -> &'static ::semio_framework_os_command::SemanticDescriptor {
                let _ = <Self as ::protocol::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#semantics_arms),* }
            }
            fn label(&self) -> ::semio_framework_ui_locale::LocalizedLabel {
                let _ = <Self as ::protocol::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#label_arms),* }
            }
            fn target(&self) -> Vec<String> {
                let _ = <Self as ::protocol::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#target_arms),* }
            }
        }

        /// 🪪️ Registers the validated leaf roster during owner startup.
        pub fn #register_fn_ident #impl_generics (
            state_class: ::semio_framework_schema_state::StateClass,
        ) -> ::core::result::Result<(), ::semio_framework_os_command::MutationDescriptorError> #where_clause {
            let _ = <#aggregate_ty as ::protocol::Mutation<#snapshot_ty>>::DESCRIPTORS;
            let descriptors = [#(#register_calls),*];
            ::semio_framework_os_command::register_mutation_descriptors(descriptors)
        }

        #payload_law
    })
}

/// ⚖️ The `#[cfg(test)]` editable-payload law `#[derive(Mutations)]` emits for a non-generic aggregate: every leaf publishes one
/// payload schema (`::protocol::mutation_input_schema_failures`), and every committed fixture under the
/// aggregate's owner directory (the one holding its `🧬️schema`) that decodes as the aggregate and, when the aggregate's own file
/// defines a top-level `demo_mutation_cases() -> Vec<Aggregate>`, every demo case is labelled in every locale
/// (`mutation_label_failures`) and rebuilds itself from its own editable payload or, when inert, refuses to
/// (`mutation_payload_round_trip_failures`), every operation answers its schema-declared inverse rows and every `perTarget` leaf
/// is admitted at the one-item ceiling and refused past it as `mutation.too-large` (`mutation_inverse_rows_declaration_failures`),
/// and every fixture case's inverse fits the leaf's declared rows
/// (`mutation_inverse_rows_failures`, design §20.5) — walked over every subset of the owner's standard, since a subset's
/// fixtures replay through the aggregate too. Nothing is emitted when the owner directory cannot be addressed from the
/// compiling crate.
fn mutation_payload_law_test(name: &syn::Ident, snapshot_ty: &Type, authority: &MutationAggregateSourceAuthority, generics: &syn::Generics) -> proc_macro2::TokenStream {
    if !generics.params.is_empty() {
        return quote! {};
    }
    let parent = authority.mutation_root.parent();
    let owner = match parent {
        Some(schema) if schema.file_name().and_then(|segment| segment.to_str()) == Some("🧬️schema") => schema.parent(),
        other => other,
    };
    let (Some(owner), Ok(manifest)) = (owner, std::env::var("CARGO_MANIFEST_DIR")) else { return quote! {} };
    let Some(relative) = mutation_relative_path(Path::new(&manifest), owner) else { return quote! {} };
    let subsets = owner.parent().filter(|subsets| subsets.file_name().and_then(|segment| segment.to_str()) == Some("🪆️subsets")).unwrap_or(owner);
    let Some(footprint_relative) = mutation_relative_path(Path::new(&manifest), subsets) else { return quote! {} };
    let source = crate::compiler_resources::read_to_string(authority.mutation_root.join(&authority.source_filename)).unwrap_or_default();
    let signature = format!("demo_mutation_cases() -> Vec<{name}>");
    let demo_cases = source.lines().any(|line| ["fn ", "pub fn ", "pub(crate) fn "].iter().any(|prefix| line.strip_prefix(prefix).is_some_and(|rest| rest.starts_with(&signature)))).then(|| quote! { ops.extend(demo_mutation_cases()); });
    let test_ident = syn::Ident::new(&format!("semio_payload_law_{}", to_kebab(&name.to_string()).replace('-', "_")), name.span());
    quote! {
        #[cfg(test)]
        #[test]
        fn #test_ident() {
            let root = ::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join(#relative);
            let (mut ops, _) = ::semio_framework_os_command::mutation_fixture_ops::<#name>(&root);
            #demo_cases
            let count = ops.len();
            let mut failures = ::semio_framework_os_command::mutation_input_schema_failures::<#snapshot_ty, #name>();
            failures.extend(::semio_framework_os_command::mutation_label_failures::<#snapshot_ty, #name>(&ops));
            failures.extend(::semio_framework_os_store_one_item::mutation_inverse_rows_declaration_failures::<#snapshot_ty, #name>(&ops));
            failures.extend(::semio_framework_os_command::mutation_payload_round_trip_failures::<#snapshot_ty, #name>(ops));
            let (footprint_failures, cases) = ::semio_framework_os_command::fold::mutation_inverse_rows_failures::<#snapshot_ty, #name>(&::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join(#footprint_relative));
            failures.extend(footprint_failures);
            assert!(failures.is_empty(), "{} breaches of the editable-payload and fold-footprint laws over {} {} operations and {} fixture cases: {:#?}", failures.len(), count, ::core::stringify!(#name), cases, failures);
        }
    }
}

/// 🧭️ `to` relative to `from` (both canonical), in `/` segments — `None` when they share no root or a segment is not UTF-8.
fn mutation_relative_path(from: &Path, to: &Path) -> Option<String> {
    let (from, to) = (mutation_authority_canonical(from).ok()?, mutation_authority_canonical(to).ok()?);
    let (from, to): (Vec<_>, Vec<_>) = (from.components().collect(), to.components().collect());
    let shared = from.iter().zip(&to).take_while(|(left, right)| left == right).count();
    if shared == 0 {
        return None;
    }
    let mut segments: Vec<String> = std::iter::repeat_n("..".to_string(), from.len() - shared).collect();
    for component in &to[shared..] {
        segments.push(component.as_os_str().to_str()?.to_string());
    }
    Some(segments.join("/"))
}
//#endregion 🔖️Mutations
