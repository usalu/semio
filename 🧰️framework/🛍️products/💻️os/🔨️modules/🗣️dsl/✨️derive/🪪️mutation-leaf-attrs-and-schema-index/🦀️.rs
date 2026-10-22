//! 🪪️ Mutation leaf attrs and schema index of the dsl derive expansion: the leaf attribute parser, the portable and workspace paths, the withdraw only check, the leaf expansion with its inverse rows, the payload schema path and the schema document index. See [`super`].

use quote::quote;
use super::{BTreeMap, Data, DeriveInput, Fields, HashSet, MUTATION_SCHEMA_INDEX_LOCATOR, MUTATION_SCHEMA_INDEX_SCHEMA, MutationLeafJson, MutationSourceAuthority, Path, PathBuf, TokenStream, emit_mutation_leaf_descriptor, fs, mutation_authority_locator, mutation_authority_no_follow, mutation_authority_relative, mutation_authority_workspace_root, mutation_source_authority, parse_macro_input, parse_mutation_leaf_descriptor};

//#region 🪪️MutationLeaf
#[derive(Debug)]
#[doc(hidden)] pub struct MutationLeafAttrs { pub(super) contract: syn::Path, pub(super) payload: Option<syn::Ident>, pub(super) input_schema: Option<syn::Path> }

impl MutationLeafAttrs {
    /// 🧷️ Collects the parsed `mutation_leaf` attribute arguments of one derive input.
    #[doc(hidden)] pub fn new(contract: syn::Path, payload: Option<syn::Ident>, input_schema: Option<syn::Path>) -> Self {
        Self { contract, payload, input_schema }
    }
}

#[doc(hidden)] pub fn parse_mutation_leaf_attrs(input: &DeriveInput) -> syn::Result<MutationLeafAttrs> {
    let mut contract = None;
    let mut payload = None;
    let mut input_schema = None;
    let mut found = false;
    for attribute in &input.attrs {
        if !attribute.path().is_ident("mutation_leaf") { continue; }
        if found { return Err(syn::Error::new_spanned(attribute, "duplicate mutation_leaf attribute")); }
        found = true;
        if matches!(&attribute.meta, syn::Meta::Path(_)) { continue; }
        attribute.parse_nested_meta(|meta| {
            if meta.path.is_ident("payload") {
                if payload.is_some() { return Err(meta.error("duplicate mutation_leaf payload")); }
                payload = Some(meta.value()?.parse::<syn::Ident>()?);
                return Ok(());
            }
            if meta.path.is_ident("input_schema") {
                if input_schema.is_some() { return Err(meta.error("duplicate mutation_leaf input_schema")); }
                input_schema = Some(meta.value()?.parse::<syn::Path>()?);
                return Ok(());
            }
            if !meta.path.is_ident("contract") { return Err(meta.error("unsupported mutation_leaf attribute")); }
            if contract.is_some() { return Err(meta.error("duplicate mutation_leaf contract")); }
            let path: syn::Path = meta.value()?.parse()?;
            if path.leading_colon.is_none() || path.segments.is_empty() || path.segments.iter().any(|segment| !matches!(segment.arguments, syn::PathArguments::None) || matches!(segment.ident.to_string().as_str(), "self" | "super" | "crate")) { return Err(meta.error("mutation_leaf contract must be an absolute non-generic Rust path without self, super, or crate segments")); }
            contract = Some(path);
            Ok(())
        })?;
    }
    if !found { return Err(syn::Error::new_spanned(input, "MutationLeaf requires #[mutation_leaf(contract = ::protocol)]")); }
    let contract = contract.ok_or_else(|| syn::Error::new_spanned(input, "MutationLeaf requires mutation_leaf contract"))?;
    if let Some(variant) = &payload {
        let Data::Enum(data) = &input.data else { return Err(syn::Error::new_spanned(variant, "mutation_leaf payload names a variant of an enum leaf")); };
        let wraps_one = data.variants.iter().find(|candidate| candidate.ident == *variant).is_some_and(|candidate| matches!(&candidate.fields, Fields::Unnamed(fields) if fields.unnamed.len() == 1));
        if !wraps_one { return Err(syn::Error::new_spanned(variant, "mutation_leaf payload names a variant that wraps exactly one payload")); }
        if let Some(path) = &input_schema { return Err(syn::Error::new_spanned(path, "mutation_leaf input_schema and payload are exclusive: a wrapped leaf's input schema is its payload variant's")); }
    }
    Ok(MutationLeafAttrs::new(contract, payload, input_schema))
}

#[doc(hidden)] pub fn mutation_leaf_portable_path(path: &Path) -> Result<String, String> { path.to_str().map(|path| path.replace('\\', "/")).filter(|path| !path.is_empty()).ok_or_else(|| "metadata path is not UTF-8".to_string()) }

/// 🧭️ Canonical path without the Windows verbatim prefix, so a canonical root and a canonical member share one prefix on every platform.
pub(super) fn mutation_authority_canonical(path: &Path) -> Result<PathBuf, String> {
    let canonical = fs::canonicalize(path).map_err(|error| error.to_string())?;
    Ok(match canonical.to_str().and_then(|value| value.strip_prefix(r"\\?\")) { Some(plain) => PathBuf::from(plain), None => canonical })
}

#[doc(hidden)] pub fn mutation_authority_workspace_token(workspace_root: &Path, taxonomy_path: &Path) -> Result<[u8; 32], String> {
    let workspace_root = mutation_authority_canonical(workspace_root)?;
    let taxonomy_path = mutation_authority_canonical(taxonomy_path)?;
    let workspace = mutation_leaf_portable_path(&workspace_root)?;
    let taxonomy = mutation_authority_relative(&workspace_root, &taxonomy_path)?;
    let mut input = b"semio.mutation-source-provenance/v1\0".to_vec();
    for value in [workspace.as_bytes(), taxonomy.as_bytes()] { let length = u64::try_from(value.len()).map_err(|_| "metadata token component exceeds u64".to_string())?; input.extend_from_slice(&length.to_be_bytes()); input.extend_from_slice(value); }
    Ok(semio_framework_hash::Sha256::digest(&input))
}

#[doc(hidden)] pub fn mutation_leaf_workspace_token(authority: &MutationSourceAuthority) -> Result<[u8; 32], String> { mutation_authority_workspace_token(&authority.workspace_root, &authority.taxonomy_path) }

pub(super) fn mutation_leaf_include_path(path: &Path) -> Result<String, String> { mutation_leaf_portable_path(path) }

/// 🚪️ The editing surface of a withdraw-only leaf (descriptor `editable: false`, design §22.20): `input_schema` is always `None`, so
/// the history editor never opens on it, and `with_input_value` refuses every edited payload — the editable-payload law
/// (`mutation_payload_round_trip_failures`) holds an inert leaf to that. `from_input_value` stays the trait's: feature rows and
/// fixtures still decode the leaf from its payload. Such a leaf declares neither `payload` nor `input_schema` — both name an
/// editable payload.
#[doc(hidden)] pub fn mutation_leaf_withdraw_only(name: &syn::Ident, descriptor: &MutationLeafJson, attrs: &MutationLeafAttrs) -> syn::Result<Option<proc_macro2::TokenStream>> {
    if descriptor.editable { return Ok(None); }
    if let Some(variant) = &attrs.payload { return Err(syn::Error::new_spanned(variant, "a withdraw-only leaf (descriptor editable: false) declares no mutation_leaf payload")); }
    if let Some(path) = &attrs.input_schema { return Err(syn::Error::new_spanned(path, "a withdraw-only leaf (descriptor editable: false) declares no mutation_leaf input_schema")); }
    Ok(Some(quote! {
        fn input_schema(&self) -> ::core::option::Option<&'static str> {
            ::core::option::Option::None
        }
        fn with_input_value(&self, _value: ::semio_framework_value::DslValue) -> ::core::result::Result<Self, ::semio_framework_value::ValueError> {
            ::core::result::Result::Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, ::std::format!("{} is withdraw-only", ::core::stringify!(#name))))
        }
    }))
}

pub fn expand_mutation_leaf(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if matches!(input.data, Data::Union(_)) { return syn::Error::new_spanned(&input, "MutationLeaf does not support unions").to_compile_error().into(); }
    let attrs = match parse_mutation_leaf_attrs(&input) { Ok(attrs) => attrs, Err(error) => return error.to_compile_error().into() };
    let source = match input.ident.span().unwrap().local_file() { Some(source) => source, None => return syn::Error::new_spanned(&input, "MutationLeaf requires a local source file").to_compile_error().into() };
    let compiler_cwd = match std::env::current_dir() { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error.to_string()).to_compile_error().into() };
    let authority = match mutation_source_authority(&source, &compiler_cwd) { Ok(authority) => authority, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf source authority failed: {error}")).to_compile_error().into() };
    let raw_descriptor = match crate::compiler_resources::read(&authority.descriptor_path) { Ok(raw) => raw, Err(error) => return syn::Error::new_spanned(&input, error.to_string()).to_compile_error().into() };
    let descriptor = match parse_mutation_leaf_descriptor(&raw_descriptor, &authority) { Ok(descriptor) => descriptor, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf descriptor failed: {error}")).to_compile_error().into() };
    let workspace_token = match mutation_leaf_workspace_token(&authority) { Ok(token) => token, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf provenance failed: {error}")).to_compile_error().into() };
    let mutation_root = match mutation_authority_relative(&authority.workspace_root, &authority.mutation_root) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let source_path = match mutation_authority_relative(&authority.workspace_root, &authority.source_path) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let descriptor_path = match mutation_authority_relative(&authority.workspace_root, &authority.descriptor_path) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let taxonomy_path = match mutation_authority_relative(&authority.workspace_root, &authority.taxonomy_path) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let payload_schema_path = match mutation_leaf_payload_schema_path(&authority, &descriptor.payload_schema) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf payload schema failed: {error}")).to_compile_error().into() };
    let inverse_rows = match mutation_leaf_inverse_rows(&payload_schema_path) { Ok(rows) => rows, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf x-semio-inverse-rows failed: {error}")).to_compile_error().into() };
    let referenced_documents = match mutation_leaf_referenced_documents(&payload_schema_path).and_then(|paths| paths.iter().map(|path| mutation_leaf_include_path(path)).collect::<Result<Vec<_>, _>>()) { Ok(paths) => paths, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf referenced schema documents failed: {error}")).to_compile_error().into() };
    let dependency_paths = [authority.taxonomy_path.clone(), authority.descriptor_path.clone(), payload_schema_path];
    let dependency_paths: Result<Vec<_>, _> = dependency_paths.iter().map(|path| mutation_leaf_include_path(path)).collect();
    let dependency_paths = match dependency_paths { Ok(paths) => paths, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let [taxonomy_dependency, descriptor_dependency, payload_schema_dependency]: [String; 3] = match dependency_paths.try_into() { Ok(paths) => paths, Err(_) => unreachable!() };
    let name = &input.ident;
    let contract = &attrs.contract;
    let editable = attrs.payload.as_ref().map(|variant| quote! {
        fn input_schema(&self) -> ::core::option::Option<&'static str> {
            match self { Self::#variant(_) => ::core::option::Option::Some(<Self as #contract::MutationLeaf>::PAYLOAD_SCHEMA), _ => ::core::option::Option::None }
        }
        fn input_value(&self) -> ::semio_framework_value::DslValue {
            match self { Self::#variant(payload) => ::semio_framework_value::ToValue::to_value(payload), _ => ::semio_framework_value::ToValue::to_value(self) }
        }
        fn with_input_value(&self, value: ::semio_framework_value::DslValue) -> ::core::result::Result<Self, ::semio_framework_value::ValueError> {
            match self {
                Self::#variant(_) => ::semio_framework_value::FromValue::from_value(value).map(Self::#variant),
                _ => ::core::result::Result::Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,::std::format!("{} is editable only as {}", ::core::stringify!(#name), ::core::stringify!(#variant)))),
            }
        }
        fn from_input_value(value: ::semio_framework_value::DslValue) -> ::core::result::Result<Self, ::semio_framework_value::ValueError> {
            ::semio_framework_value::FromValue::from_value(value).map(Self::#variant)
        }
    });
    let instance_schema = attrs.input_schema.as_ref().map(|path| quote! {
        fn input_schema(&self) -> ::core::option::Option<&'static str> {
            #path(self)
        }
    });
    let withdraw_only = match mutation_leaf_withdraw_only(&input.ident, &descriptor, &attrs) { Ok(tokens) => tokens, Err(error) => return error.to_compile_error().into() };
    let inverse_rows = mutation_leaf_inverse_rows_body(&inverse_rows, attrs.payload.as_ref());
    let owner = &authority.owner;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let descriptor = emit_mutation_leaf_descriptor(contract, &descriptor);
    let workspace_token = workspace_token.iter();
    quote! {
        const _: &str = ::core::include_str!(#taxonomy_dependency);
        const _: &str = ::core::include_str!(#descriptor_dependency);
        impl #impl_generics #contract::MutationLeaf for #name #ty_generics #where_clause {
            const DESCRIPTOR: #contract::MutationLeafDescriptor = #descriptor;
            const PROVENANCE: #contract::MutationSourceProvenance = #contract::MutationSourceProvenance { workspace_token: [#(#workspace_token),*], mutation_root: #mutation_root, owner: #owner, source_path: #source_path, descriptor_path: #descriptor_path, taxonomy_path: #taxonomy_path };
            const PAYLOAD_SCHEMA: &'static str = ::core::include_str!(#payload_schema_dependency);
            const PAYLOAD_SCHEMA_DOCUMENTS: &'static [&'static str] = &[#(::core::include_str!(#referenced_documents)),*];
            #editable
            #instance_schema
            #withdraw_only
            fn inverse_rows(&self) -> usize {
                #inverse_rows
            }
        }
    }.into()
}

/// 🧾️ A leaf payload schema's `x-semio-inverse-rows` (design §20.5): `fixed` rows plus `perTarget` rows per item of each named
/// array field, or one `bounded` constant; absent, one row.
struct MutationLeafInverseRows {
    fixed: usize,
    per_target: Vec<(String, usize)>,
}

/// 📖️ Reads `x-semio-inverse-rows` from the root of the payload schema at `payload_schema`.
fn mutation_leaf_inverse_rows(payload_schema: &Path) -> Result<MutationLeafInverseRows, String> {
    let raw = crate::compiler_resources::read(payload_schema).map_err(|error| error.to_string())?;
    let value: serde_json::Value = serde_json::from_slice(&raw).map_err(|error| format!("malformed payload schema: {error}"))?;
    let Some(rows) = value.get("x-semio-inverse-rows") else { return Ok(MutationLeafInverseRows { fixed: 1, per_target: Vec::new() }) };
    let object = rows.as_object().filter(|object| !object.is_empty()).ok_or_else(|| "x-semio-inverse-rows must be a nonempty object".to_string())?;
    if object.keys().any(|key| !["fixed", "perTarget", "bounded"].contains(&key.as_str())) { return Err("x-semio-inverse-rows admits only fixed, perTarget and bounded".to_string()); }
    if object.contains_key("bounded") && object.len() != 1 { return Err("x-semio-inverse-rows bounded excludes fixed and perTarget".to_string()); }
    let count = |key: &str| object.get(key).map(|value| value.as_u64().map(|count| count as usize).ok_or_else(|| format!("x-semio-inverse-rows {key} must be a nonnegative integer"))).transpose();
    let fixed = count("bounded")?.or(count("fixed")?).unwrap_or(0);
    let per_target = match object.get("perTarget") {
        None => Vec::new(),
        Some(fields) => fields
            .as_object()
            .filter(|fields| !fields.is_empty())
            .ok_or_else(|| "x-semio-inverse-rows perTarget must be a nonempty object".to_string())?
            .iter()
            .map(|(field, rows)| rows.as_u64().filter(|rows| *rows >= 1).map(|rows| (to_snake_ascii(field), rows as usize)).ok_or_else(|| format!("x-semio-inverse-rows perTarget {field} must be a positive integer")))
            .collect::<Result<_, _>>()?,
    };
    Ok(MutationLeafInverseRows { fixed, per_target })
}

/// 🐍️ The Rust field a camelCase payload property names.
fn to_snake_ascii(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 4);
    for character in value.chars() {
        if character.is_ascii_uppercase() {
            out.push('_');
            out.push(character.to_ascii_lowercase());
        } else {
            out.push(character);
        }
    }
    out
}

/// 🧮️ The generated `MutationLeaf::inverse_rows` body: the fixed rows plus each `perTarget` field's length times its rows, read
/// from the leaf itself or from its `payload` variant (any other variant declares the fixed rows).
fn mutation_leaf_inverse_rows_body(rows: &MutationLeafInverseRows, payload: Option<&syn::Ident>) -> proc_macro2::TokenStream {
    let fixed = proc_macro2::Literal::usize_unsuffixed(rows.fixed);
    if rows.per_target.is_empty() {
        return quote! { #fixed };
    }
    let terms = rows.per_target.iter().map(|(field, count)| {
        let field = syn::Ident::new(field, proc_macro2::Span::call_site());
        let count = proc_macro2::Literal::usize_unsuffixed(*count);
        quote! { + #count * payload.#field.len() }
    });
    match payload {
        None => quote! { let payload = self; #fixed #(#terms)* },
        Some(variant) => quote! { match self { Self::#variant(payload) => #fixed #(#terms)*, _ => #fixed } },
    }
}

/// 🧬️ The descriptor's `payloadSchema`, resolved beside the descriptor: a normalized relative path of portable
/// segments that stays inside the leaf and names a regular file reached without a symlink.
#[doc(hidden)] pub fn mutation_leaf_payload_schema_path(authority: &MutationSourceAuthority, payload_schema: &str) -> Result<PathBuf, String> {
    let leaf = authority.descriptor_path.parent().ok_or_else(|| "descriptor has no leaf directory".to_string())?;
    if payload_schema.starts_with('/') || payload_schema.contains('\\') || payload_schema.contains('\0') { return Err("payloadSchema is not a relative portable path".to_string()); }
    let mut path = leaf.to_path_buf();
    for segment in payload_schema.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." || segment.eq_ignore_ascii_case("compose") { return Err("payloadSchema has a rejected path segment".to_string()); }
        path.push(segment);
    }
    mutation_authority_no_follow(&authority.workspace_root, &path, false)?;
    Ok(path)
}

/// 🔗️ Every schema document the payload schema at `payload_schema` references by an absolute `$id` (fragment stripped),
/// transitively, among the `🧬️schema` JSON documents of its search tree ([`mutation_schema_search_root`]) recorded in the
/// generated schema index — what the runtime publishes beside the leaf (`MutationLeaf::PAYLOAD_SCHEMA_DOCUMENTS`), so an input
/// that `$ref`s another facet of its scope resolves. A reference the index does not hold is left to the OS-wide registry, which
/// framework scopes publish themselves. Sorted, never the payload schema itself.
fn mutation_leaf_referenced_documents(payload_schema: &Path) -> Result<Vec<PathBuf>, String> {
    let index = mutation_schema_document_index(&mutation_schema_search_root(payload_schema))?;
    let (own, mut pending) = mutation_schema_document_references(payload_schema)?;
    let mut seen: HashSet<String> = own.into_iter().collect();
    let mut found = Vec::new();
    while let Some(id) = pending.pop() {
        if !seen.insert(id.clone()) {
            continue;
        }
        let Some(path) = index.get(&id) else { continue };
        pending.extend(mutation_schema_document_references(path)?.1);
        found.push(path.clone());
    }
    found.sort();
    Ok(found)
}

/// 🧭️ The tree a leaf's referenced documents are looked up in: its plugin (the directory under `🔌️plugins`, which bundles every
/// artifact a leaf may reference across), else its framework module (the nearest directory under `🔨️modules`), else the module
/// owning its `🧬️schema/🧬️mutations` root.
fn mutation_schema_search_root(payload_schema: &Path) -> PathBuf {
    let under = |parent: &str| payload_schema.ancestors().find(|ancestor| ancestor.parent().and_then(Path::file_name).is_some_and(|name| name == parent));
    let owner = payload_schema.ancestors().find(|ancestor| ancestor.file_name().is_some_and(|name| name == "🧬️schema") && ancestor.join("🧬️mutations").is_dir()).and_then(Path::parent);
    under("🔌️plugins").or_else(|| under("🔨️modules")).or(owner).or_else(|| payload_schema.parent()).unwrap_or(payload_schema).to_path_buf()
}

/// 🧫️ Recognizes example collections while preserving domain modules whose names match collection roles.
fn mutation_schema_example_collection(path: &Path) -> bool {
    path.ancestors().any(|directory| directory.file_name().is_some_and(|name| ["🧫️fixtures", "🧪️fixtures", "🧪️tests"].iter().any(|collection| name == *collection)) && !directory.parent().and_then(Path::file_name).is_some_and(|name| name == "🔨️modules"))
}

/// 🗂️ The `$id` documents of `root` from the generated schema index. The index file is the only directory-freshness input:
/// adding a schema rewrites it, and rustc tracks that one file instead of every walked directory.
#[doc(hidden)] pub fn mutation_schema_document_index(root: &Path) -> Result<std::rc::Rc<BTreeMap<String, PathBuf>>, String> {
    let workspace = mutation_authority_workspace_root(root)?;
    let index = mutation_schema_index(&workspace)?;
    if mutation_schema_example_collection(root) { return Ok(std::rc::Rc::new(BTreeMap::new())); }
    let key = mutation_authority_relative(&workspace, root)?;
    index.get(&key).cloned().ok_or_else(|| format!("schema document index has no root {key}; run repo run @semio-tech/dsl-derive-rs:generate-schema-index"))
}

type MutationSchemaIndex = BTreeMap<String, std::rc::Rc<BTreeMap<String, PathBuf>>>;

type CachedSchemaIndex = Option<(PathBuf, std::rc::Rc<MutationSchemaIndex>)>;

/// 📖️ Loads and caches the generated index, tracking the one file through the compiler resource observer.
fn mutation_schema_index(workspace: &Path) -> Result<std::rc::Rc<MutationSchemaIndex>, String> {
    thread_local! { static INDEX: std::cell::RefCell<CachedSchemaIndex> = const { std::cell::RefCell::new(None) }; }
    if let Some(cached) = INDEX.with(|current| current.borrow().as_ref().filter(|(root, _)| root == workspace).map(|(_, index)| index.clone())) { return Ok(cached); }
    let path = mutation_authority_locator(workspace, MUTATION_SCHEMA_INDEX_LOCATOR)?;
    mutation_authority_no_follow(workspace, &path, false).map_err(|error| format!("schema document index {MUTATION_SCHEMA_INDEX_LOCATOR}: {error}; run repo run @semio-tech/dsl-derive-rs:generate-schema-index"))?;
    let raw = crate::compiler_resources::read(&path).map_err(|error| error.to_string())?;
    let parsed = std::rc::Rc::new(parse_mutation_schema_document_index(workspace, &raw)?);
    INDEX.with(|current| *current.borrow_mut() = Some((workspace.to_path_buf(), parsed.clone())));
    Ok(parsed)
}

/// 🧬️ Parses `MutationSchemaDocumentIndexV1` into absolute document paths keyed by search root and `$id`.
fn parse_mutation_schema_document_index(workspace: &Path, raw: &[u8]) -> Result<MutationSchemaIndex, String> {
    let value: serde_json::Value = serde_json::from_slice(raw).map_err(|error| error.to_string())?;
    let object = value.as_object().filter(|object| object.len() == 2 && object.contains_key("schema") && object.contains_key("roots")).ok_or_else(|| "schema document index must contain schema and roots".to_string())?;
    if object.get("schema").and_then(serde_json::Value::as_str) != Some(MUTATION_SCHEMA_INDEX_SCHEMA) { return Err("schema document index declares another schema".to_string()); }
    let roots = object.get("roots").and_then(serde_json::Value::as_object).ok_or_else(|| "schema document index roots must be an object".to_string())?;
    let mut parsed = BTreeMap::new();
    for (root, documents) in roots {
        if root.is_empty() || root.contains('\\') || root.starts_with('/') || root.split('/').any(|segment| segment.is_empty() || segment == "." || segment == "..") { return Err(format!("schema document index root {root} is not a relative path")); }
        let documents = documents.as_object().ok_or_else(|| format!("schema document index root {root} must map documents"))?;
        let mut resolved = BTreeMap::new();
        for (id, path) in documents {
            if id.is_empty() { return Err("schema document index $id must be nonempty".to_string()); }
            let path = path.as_str().filter(|path| !path.is_empty()).ok_or_else(|| format!("schema document index path for {id} must be a string"))?;
            if !path.starts_with(&format!("{root}/")) { return Err(format!("schema document index path {path} escapes root {root}")); }
            resolved.insert(id.clone(), mutation_authority_locator(workspace, path)?);
        }
        parsed.insert(root.clone(), std::rc::Rc::new(resolved));
    }
    Ok(parsed)
}

/// 🔗️ A schema document's own `$id` and every absolute document id its `$ref`s name (fragments stripped).
fn mutation_schema_document_references(path: &Path) -> Result<(Option<String>, Vec<String>), String> {
    fn collect(value: &serde_json::Value, into: &mut Vec<String>) {
        match value {
            serde_json::Value::Object(entries) => {
                for (key, value) in entries {
                    match (key.as_str(), value) {
                        ("$ref", serde_json::Value::String(reference)) if !reference.starts_with('#') => into.push(reference.split('#').next().unwrap_or_default().to_string()),
                        _ => collect(value, into),
                    }
                }
            }
            serde_json::Value::Array(items) => items.iter().for_each(|item| collect(item, into)),
            _ => {}
        }
    }
    let raw = crate::compiler_resources::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let document: serde_json::Value = serde_json::from_slice(&raw).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut references = Vec::new();
    collect(&document, &mut references);
    references.retain(|reference| !reference.is_empty());
    Ok((document.get("$id").and_then(serde_json::Value::as_str).map(str::to_string), references))
}
//#endregion 🪪️MutationLeaf
