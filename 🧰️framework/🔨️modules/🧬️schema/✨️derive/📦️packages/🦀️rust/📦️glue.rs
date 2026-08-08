//! ✨️ `semio_framework_schema_derive` — `#[derive(ArtifactSchema)]` with `#[artifact_schema]` / `#[state]`.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta};

//#region 🔖️Helpers
/// 🐪 snake_case ident → canonical camelCase JSON field name.
fn snake_to_camel(name: &str) -> String {
    let mut out = String::new();
    let mut upper = false;
    for ch in name.chars() {
        if ch == '_' {
            upper = true;
            continue;
        }
        if upper {
            out.extend(ch.to_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    out
}

fn parse_artifact_id(input: &DeriveInput) -> syn::Result<String> {
    for attr in &input.attrs {
        if !attr.path().is_ident("artifact_schema") {
            continue;
        }
        let mut id = None;
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("id") {
                let value: syn::LitStr = meta.value()?.parse()?;
                id = Some(value.value());
                Ok(())
            } else {
                Err(meta.error("unsupported artifact_schema attribute"))
            }
        })?;
        return id.ok_or_else(|| syn::Error::new_spanned(attr, "artifact_schema requires id = \"…\""));
    }
    Err(syn::Error::new_spanned(&input.ident, "missing #[artifact_schema(id = \"…\")]"))
}

fn parse_state_class(field: &syn::Field) -> syn::Result<String> {
    for attr in &field.attrs {
        if !attr.path().is_ident("state") {
            continue;
        }
        let Meta::List(list) = &attr.meta else {
            return Err(syn::Error::new_spanned(attr, "expected #[state(persistent|shared_ui|local_ui|preview|effect)]"));
        };
        let tokens = list.tokens.to_string().replace(' ', "");
        let variant = match tokens.as_str() {
            "persistent" => "Persistent",
            "shared_ui" => "SharedUi",
            "local_ui" => "LocalUi",
            "preview" => "Preview",
            "effect" => "Effect",
            other => {
                return Err(syn::Error::new_spanned(
                    attr,
                    format!("unknown state class `{other}`"),
                ))
            }
        };
        return Ok(variant.to_string());
    }
    Err(syn::Error::new_spanned(field, "missing #[state(…)] on field"))
}
//#endregion 🔖️Helpers

//#region 🔖️ArtifactSchema
/// ✨️ Derives [`ArtifactSchemaFields`] from `#[artifact_schema]` / `#[state]` annotations.
#[proc_macro_derive(ArtifactSchema, attributes(artifact_schema, state))]
pub fn derive_artifact_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_artifact_schema(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand_artifact_schema(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let id = parse_artifact_id(input)?;
    let ident = &input.ident;
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(ident, "ArtifactSchema only supports structs"));
    };
    let Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new_spanned(ident, "ArtifactSchema requires named fields"));
    };

    let mut field_entries = Vec::new();
    for field in &fields.named {
        let name = field.ident.as_ref().ok_or_else(|| syn::Error::new_spanned(field, "named field required"))?;
        let camel = snake_to_camel(&name.to_string());
        let variant = parse_state_class(field)?;
        let variant_ident = syn::Ident::new(&variant, name.span());
        field_entries.push(quote! {
            (#camel, ::semio_framework_schema::StateClass::#variant_ident)
        });
    }

    let id_lit = syn::LitStr::new(&id, ident.span());
    Ok(quote! {
        impl ::semio_framework_schema::ArtifactSchemaFields for #ident {
            fn artifact_schema_id() -> &'static str {
                #id_lit
            }
            fn field_states() -> &'static [(&'static str, ::semio_framework_schema::StateClass)] {
                &[#(#field_entries),*]
            }
        }
    })
}
//#endregion 🔖️ArtifactSchema
