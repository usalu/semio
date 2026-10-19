//! 🌿️ Variant helpers of the record derive: the kebab-case conversion and the hygienic field locals and initializers. See [`super`].

use super::*;

//#region 🔖️VariantHelpers
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

/// 🔒️ Reserved local for one authored field: generated statements bind authored fields only under
/// `__semio_field_<name>`, so no authored name (`field`, `record`, `control`, `value`, …) can shadow a
/// generated local or be captured by one. Law: `🗣️dsl/🧪️tests/🧪️hygienic-bindings`.
pub(super) fn field_local(ident: &syn::Ident) -> syn::Ident {
    quote::format_ident!("__semio_field_{}", ident)
}

/// 🧷️ Struct/variant initializers `name: __semio_field_name` over the reserved locals.
pub(super) fn field_inits(idents: &[syn::Ident]) -> Vec<proc_macro2::TokenStream> {
    idents.iter().map(|ident| { let local = field_local(ident); quote! { #ident: #local } }).collect()
}

/// 🏗️ Like the `to_value` half of `record_codegen`, but reading from bare local bindings
/// (`ident`) instead of `self.ident` — what a `match self { Variant { fields... } => ... }` arm
/// needs, since enum variant fields aren't reached through `self.field` syntax.
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
pub(super) fn record_codegen_to_value_from_bindings(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    let plans = plan_fields(fields);
    plans
        .iter()
        .filter(|plan|!plan.skip)
        .map(|plan| {
            let FieldPlan { ident, id, kind, block, .. } = plan;
            let ident = &field_local(ident);
            let to_value_expr: proc_macro2::TokenStream = match kind {
                FieldKind::Scalar => quote! { ::semio_framework_dsl_record::DslField::to_value(#ident) },
                FieldKind::Bytes64 => quote! { ::semio_framework_dsl_record::FieldValue::Bytes64(#ident.clone()) },
                FieldKind::OptionScalar(_) => quote! {
                    match #ident {
                        Some(v) => ::semio_framework_dsl_record::DslField::to_value(v),
                        None => ::semio_framework_dsl_record::FieldValue::Absent,
                    }
                },
                FieldKind::VecList(_) | FieldKind::VecTable(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::semio_framework_dsl_record::DslField::to_value(v)); }
                        ::semio_framework_dsl_record::FieldValue::List(__items)
                    }
                },
                FieldKind::VecTuple(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::semio_framework_dsl_record::DslField::to_value(v)); }
                        ::semio_framework_dsl_record::FieldValue::Tuple(__items)
                    }
                },
                FieldKind::VecStatements(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::semio_framework_dsl_record::DslVariants::to_named_record(v)); }
                        ::semio_framework_dsl_record::FieldValue::Statements(__items)
                    }
                },
                FieldKind::VecBlockStatements(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::semio_framework_dsl_record::DslVariants::to_named_record(v)); }
                        ::semio_framework_dsl_record::FieldValue::Block(Box::new(::semio_framework_dsl_record::FieldValue::Statements(__items)))
                    }
                },
                FieldKind::MapField(_) => quote! {
                    {
                        let mut __entries = Vec::with_capacity(#ident.len());
                        for (k, v) in #ident.iter() { __entries.push((k.clone(), ::semio_framework_dsl_record::DslField::to_value(v))); }
                        ::semio_framework_dsl_record::FieldValue::Map(__entries)
                    }
                },
                FieldKind::OptionStatements(_) => quote! {
                    ::semio_framework_dsl_record::FieldValue::Statements(match #ident {
                        Some(v) => vec![::semio_framework_dsl_record::DslVariants::to_named_record(v)],
                        None => vec![],
                    })
                },
                FieldKind::RequiredStatements(_) => quote! { ::semio_framework_dsl_record::FieldValue::Statements(vec![::semio_framework_dsl_record::DslVariants::to_named_record(#ident.as_ref())]) },
                FieldKind::RequiredInlineStatements(_) => quote! { ::semio_framework_dsl_record::FieldValue::Statements(vec![::semio_framework_dsl_record::DslVariants::to_named_record(#ident)]) },
            };
            let to_value_expr = if *block {
                quote! {
                    match #to_value_expr {
                        ::semio_framework_dsl_record::FieldValue::Absent => ::semio_framework_dsl_record::FieldValue::Absent,
                        other => ::semio_framework_dsl_record::FieldValue::Block(Box::new(other)),
                    }
                }
            } else {
                to_value_expr
            };
            quote! { record.fields.insert(#id, #to_value_expr); }
        })
        .collect()
}
//#endregion 🔖️VariantHelpers
