//! 🏗️ Document, diff and composite expansion of the dsl derive expansion: the dsl document, diff text and diff binary expansions, the composite attribute parser, the composite mutation expansion and the kebab helper. See [`super`].

use super::*;

/// ✉️ Owns artifact envelope metadata independently from canonical Record fields.
pub fn expand_dsl_document(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if !matches!(input.data, Data::Struct(_)) {
        return syn::Error::new_spanned(&input, "DslArtifact only supports structs").to_compile_error().into();
    }
    let mut id: Option<String> = None;
    let mut extension: Option<String> = None;
    for attribute in &input.attrs {
        if !attribute.path().is_ident("artifact") { continue; }
        if let Err(error) = attribute.parse_nested_meta(|meta| {
            let target = if meta.path.is_ident("id") { &mut id } else if meta.path.is_ident("extension") { &mut extension } else { return Err(meta.error("unsupported artifact attribute")); };
            if target.is_some() { return Err(meta.error("duplicate artifact attribute")); }
            let value: syn::LitStr = meta.value()?.parse()?;
            *target = Some(value.value());
            Ok(())
        }) { return error.to_compile_error().into(); }
    }
    let envelope_id = match id.or_else(|| extension.clone()) {
        Some(id) => id,
        None => return syn::Error::new_spanned(&input, "DslArtifact requires #[artifact(id = \"plugin.artifact\")] or #[artifact(extension = \"...\")]").to_compile_error().into(),
    };
    let suffix = extension.as_deref().unwrap_or_else(|| envelope_id.rsplit('.').next().unwrap_or(&envelope_id));
    let name = &input.ident;
    quote! {
        impl #name {
            pub const __DSL_ENVELOPE_ID: &'static str = #envelope_id;
            pub const __DSL_EXTENSION: &'static str = #suffix;
            /// 🏭️ Forwards the actual DslRecord controlled metadata factories.
            pub fn __artifact_record_spec_producer() -> ::semio_framework_dsl_record::RecordSpecProducer {
                Self::__dsl_spec_producer()
            }
            /// 🫳️ Binds the same DslRecord authored static declaration.
            pub fn __artifact_borrowed_record_spec_producer() -> ::semio_framework_dsl_record::BorrowedRecordSpecProducer {
                ::semio_framework_dsl_record::BorrowedRecordSpecProducer::of::<Self>()
            }
        }
    }.into()
}

/// 📝️ Generates diff text I/O at its representation owner.
pub fn expand_diff_text(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Type);
    diff_text_tokens(&name).into()
}

#[doc(hidden)] pub fn diff_text_tokens(name: &Type) -> proc_macro2::TokenStream {
    quote! {
        impl ::semio_framework_os_kernel::DiffText for #name {
            fn print_diff(&self) -> String {
                ::semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), ::semio_framework_dsl_record::JoinMode::Inline)
            }
            fn parse_diff(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
                let record = ::semio_framework_dsl_record::parse_exact(line, &Self::__dsl_spec(), &::semio_framework_dsl_record::ParseOptions { limits: ::semio_framework_diagnostic::Limits::default(), mode: ::semio_framework_dsl_record::SourceMode::Inline })?;
                Self::__dsl_from_record(&record)
            }
        }
    }
}

/// 💾️ Generates diff binary I/O at its representation owner.
pub fn expand_diff_binary(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Type);
    diff_binary_tokens(&name).into()
}

#[doc(hidden)] pub fn diff_binary_tokens(name: &Type) -> proc_macro2::TokenStream {
    quote! {
        impl ::semio_framework_os_kernel::DiffBinary for #name {
            fn encode_diff(&self) -> Result<Vec<u8>, ::semio_framework_os_kernel::ProtocolError> {
                ::semio_framework_os_kernel::os_pack::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), &::semio_framework_os_kernel::os_store::PackEncodeOptions::default()).map_err(::semio_framework_os_kernel::ProtocolError::from)
            }
            fn decode_diff(bytes: &[u8]) -> Result<Self, ::semio_framework_os_kernel::ProtocolError> {
                let (record, _report) = ::semio_framework_os_kernel::os_pack::pack_rt::decode_document(bytes, &Self::__dsl_spec(), &::semio_framework_os_kernel::os_store::PackDecodeOptions::default()).map_err(::semio_framework_os_kernel::ProtocolError::from)?;
                Self::__dsl_from_record(&record).map_err(|error| ::semio_framework_os_kernel::ProtocolError::Malformed { what: "diff record", offset: 0, detail: error.to_string() })
            }
        }
    }
}

//#region 🔖️CompositeMutation
/// 🌉️ `#[composite(snapshot = ..., op = ...)]` container attrs for
/// `#[derive(CompositeMutation)]` — see that macro's doc.
#[derive(Default)]
struct CompositeAttrs {
    pub(super) snapshot: Option<Type>,
    op: Option<Type>,
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
#[doc(hidden)] pub fn parse_composite_attrs(input: &DeriveInput) -> syn::Result<CompositeAttrs> {
    let mut out = CompositeAttrs::default();
    for attr in &input.attrs {
        if !attr.path().is_ident("composite") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("snapshot") {
                if out.snapshot.is_some() { return Err(meta.error("duplicate composite snapshot")); }
                out.snapshot = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("op") {
                if out.op.is_some() { return Err(meta.error("duplicate composite op")); }
                out.op = Some(meta.value()?.parse()?);
            } else { return Err(meta.error("unsupported composite attribute")); }
            Ok(())
        })?;
    }
    Ok(out)
}

pub fn expand_derive_composite_mutation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_composite_mutation(&input) {
        Ok(expanded) => expanded.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[doc(hidden)] pub fn expand_composite_mutation(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = input.ident.clone();
    let attrs = parse_composite_attrs(input)?;
    let (Some(snapshot_ty), Some(op_ty)) = (attrs.snapshot, attrs.op) else {
        return Err(syn::Error::new_spanned(input, "#[derive(CompositeMutation)] requires #[composite(snapshot = YourSnapshot, op = YourOp)]"));
    };

    let expected_kebab = to_kebab(&name.to_string());
    let assert_kind_message = format!("#[derive(CompositeMutation)]: {}'s CompositeMutationKind::SEMANTICS.kind must equal \"{}\" (its own kebab form)", name, expected_kebab);
    let assert_verb_message = format!("#[derive(CompositeMutation)]: {}'s CompositeMutationKind::SEMANTICS.verb must be one of protocol::APPROVED_VERBS", name);

    let expanded = quote! {
        const _: () = assert!(::semio_framework_os_kernel::str_eq(<#name as ::semio_framework_os_kernel::CompositeMutationKind<#snapshot_ty, #op_ty>>::SEMANTICS.kind, #expected_kebab), #assert_kind_message);
        const _: () = assert!(::semio_framework_os_kernel::is_approved_verb(<#name as ::semio_framework_os_kernel::CompositeMutationKind<#snapshot_ty, #op_ty>>::SEMANTICS.verb), #assert_verb_message);

        impl ::semio_framework_os_kernel::MutationKind<#snapshot_ty, #op_ty> for #name {
            const SEMANTICS: ::semio_framework_os_kernel::SemanticDescriptor = <#name as ::semio_framework_os_kernel::CompositeMutationKind<#snapshot_ty, #op_ty>>::SEMANTICS;
            fn diff(&self, base: &#snapshot_ty) -> ::semio_framework_os_kernel::MutationOutcome<<#op_ty as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::Diff> {
                ::semio_framework_os_kernel::os_spr::fold::fold_plan_diff(self, base)
            }
            fn inverse(&self, base: &#snapshot_ty) -> Result<Vec<#op_ty>, ::semio_framework_value::ValueError> {
                ::semio_framework_os_kernel::os_spr::fold::fold_plan_inverse(self, base)
            }
            fn label(&self) -> ::semio_framework_ui_locale::LocalizedLabel {
                ::semio_framework_os_kernel::CompositeMutationKind::label(self)
            }
            fn timestamp(&self) -> Option<::semio_framework_os_kernel::HybridLogicalTimestamp> {
                ::semio_framework_os_kernel::CompositeMutationKind::timestamp(self)
            }
            fn target(&self) -> Vec<String> {
                ::semio_framework_os_kernel::CompositeMutationKind::target(self)
            }
            fn may_emit_foreign_steps(&self) -> bool {
                ::semio_framework_os_kernel::CompositeMutationKind::may_emit_foreign_steps(self)
            }
            fn foreign_step_source<'a>(&'a self, base: &'a #snapshot_ty, index: usize) -> Result<Option<::semio_framework_os_kernel::ForeignStepSource<'a>>, ::semio_framework_value::ValueError> {
                ::semio_framework_os_kernel::CompositeMutationKind::foreign_step_source(self, base, index)
            }
        }
    };
    Ok(expanded)
}
//#endregion 🔖️CompositeMutation

//#region 🔖️VariantHelpers
/// 🔡️ Converts a Rust identifier (`PascalCase`/`camelCase`/`snake_case`, any mix) into
/// lowercase `kebab-case` — the unified syntax law's key/keyword/tag convention. Falls back to
/// this whenever no explicit `#[dsl(key = "...")]` override is given, for variant keywords,
/// record field keys, and `DslScalar` variant tags alike, so `SetCamera` -> `set-camera`,
/// `airtightness_n50` -> `airtightness-n50`, `HTTPServer` -> `http-server`.
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
pub(super) fn to_kebab(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    let mut out = String::with_capacity(name.len() + 4);
    for (i, &c) in chars.iter().enumerate() {
        if c == '_' || c == '-' {
            if !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
            continue;
        }
        if c.is_uppercase() {
            let prev = if i == 0 { None } else { chars.get(i - 1).copied() };
            let next = chars.get(i + 1).copied();
            // A new word starts at an uppercase letter that follows a lowercase/digit
            // (`SetCamera` -> boundary before `C`) OR that follows another uppercase letter but
            // is itself followed by a lowercase one (`HTTPServer` -> boundary before the `S` that
            // starts "Server", not between every letter of the "HTTP" acronym).
            let boundary = match prev {
                Some(p) if p.is_lowercase() || p.is_ascii_digit() => true,
                Some(p) if p.is_uppercase() => next.is_some_and(|n| n.is_lowercase()),
                _ => false,
            };
            if boundary && !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}
//#endregion 🔖️VariantHelpers
