//! 🎯️ The `ToValue` and `FromValue` derive entry points.

use quote::quote;
use syn::{Data, DeriveInput, Fields};
use super::{container_crate_path, controlled_from_body, controlled_retirement_body, controlled_to_body, deny_unknown_keys, enum_edit_path_body, enum_key_at_path_body, enum_to_path_body, field_wire_name, from_value_struct_fields, generics_with_bound, named_fields, parse_container_attrs, parse_field_attrs, parse_variant_attrs, struct_edit_path_body, struct_key_at_path_body, struct_to_path_body, to_value_object_entries, variant_destructure_patterns, variant_field_from_value_read, variant_field_to_value_push, variant_wire_name};

//#region 🔖️Expand
pub fn expand_to_value(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let container = parse_container_attrs(&input.attrs)?;
    let value_crate = container_crate_path(&container);
    let generics = generics_with_bound(&input.generics, &container.bound, &quote! { #value_crate::ToValue });
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = match &input.data {
        Data::Struct(data) if container.transparent => match &data.fields {
            Fields::Named(named) if named.named.len() == 1 => {
                let ident = named.named.first().expect("checked len == 1").ident.clone().expect("named field");
                quote! { #value_crate::ToValue::to_value(&self.#ident) }
            }
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => quote! { #value_crate::ToValue::to_value(&self.0) },
            other => return Err(syn::Error::new_spanned(other, "#[value(transparent)] requires exactly one field")),
        },
        // 🆔 Single-field tuple struct (`struct Foo(pub u32);`): automatic transparent newtype —
        // see the module docs' `#[value(transparent)]` entry for why this needs no attribute.
        Data::Struct(data) if matches!(&data.fields, Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1) => {
            quote! { #value_crate::ToValue::to_value(&self.0) }
        }
        Data::Struct(data) => {
            let fields = named_fields(&data.fields, &container)?;
            let entries = to_value_object_entries(&fields, &value_crate);
            quote! {
                #entries
                #value_crate::DslValue::Object(entries)
            }
        }
        Data::Enum(data) if data.variants.is_empty() => quote! { match *self {} },
        Data::Enum(data) if container.tag.is_none() && data.variants.iter().all(|variant| matches!(variant.fields, Fields::Unit)) => {
            let arms = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                Ok(quote! { Self::#variant_ident => #value_crate::DslValue::String(#wire_variant.to_string()) })
            }).collect::<syn::Result<Vec<_>>>()?;
            quote! {
                match *self { #(#arms),* }
            }
        }
        Data::Enum(data) if container.tag.is_none() => {
            // 🏷️ Externally-tagged (serde's own default enum representation when no `#[serde(tag
            // = …)]` is present): a unit variant is still the bare wire-name string, a
            // single-unnamed-field or named-field variant becomes a one-key object
            // `{"VariantName": <payload>}`.
            let arms = data
                .variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                    let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                    let arm: syn::Result<proc_macro2::TokenStream> = match &variant.fields {
                        Fields::Unit => Ok(quote! {
                            Self::#variant_ident => #value_crate::DslValue::String(#wire_variant.to_string())
                        }),
                        Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => Ok(quote! {
                            Self::#variant_ident(payload) => #value_crate::DslValue::object([
                                (#wire_variant.to_string(), #value_crate::ToValue::to_value(payload)),
                            ])
                        }),
                        Fields::Named(named) => {
                            let push_into = quote! { content_entries };
                            let pushes = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs));
                                    variant_field_to_value_push(field, &field_attrs, &wire_name, &ident, &push_into, &value_crate)
                                })
                                .collect::<syn::Result<Vec<_>>>()?;
                            let idents = variant_destructure_patterns(named);
                            Ok(quote! {
                                Self::#variant_ident { #(#idents),* } => {
                                    let mut content_entries: Vec<(String, #value_crate::DslValue)> = Vec::new();
                                    #(#pushes)*
                                    #value_crate::DslValue::object([
                                        (#wire_variant.to_string(), #value_crate::DslValue::Object(content_entries)),
                                    ])
                                }
                            })
                        }
                        other => Err(syn::Error::new_spanned(other, "#[derive(ToValue)] externally-tagged enum variants must be unit, a single unnamed payload, or named fields")),
                    };
                    arm
                })
                .collect::<syn::Result<Vec<_>>>()?;
            quote! {
                match self { #(#arms),* }
            }
        }
        Data::Enum(data) => {
            let Some(tag) = &container.tag else {
                unreachable!("the tag.is_none() arm above already handles every tag-less enum");
            };
            let arms = data
                .variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                    let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                    let arm: syn::Result<proc_macro2::TokenStream> = match (&variant.fields, &container.content) {
                        (Fields::Unit, _) => Ok(quote! {
                            Self::#variant_ident => #value_crate::DslValue::object([(#tag.to_string(), #value_crate::DslValue::String(#wire_variant.to_string()))])
                        }),
                        (Fields::Unnamed(unnamed), Some(content)) if unnamed.unnamed.len() == 1 => Ok(quote! {
                            Self::#variant_ident(payload) => #value_crate::DslValue::object([
                                (#tag.to_string(), #value_crate::DslValue::String(#wire_variant.to_string())),
                                (#content.to_string(), #value_crate::ToValue::to_value(payload)),
                            ])
                        }),
                        (Fields::Unnamed(unnamed), None) if unnamed.unnamed.len() == 1 => Ok(quote! {
                            Self::#variant_ident(payload) => {
                                let mut entries = match #value_crate::ToValue::to_value(payload) {
                                    #value_crate::DslValue::Object(entries) => entries,
                                    other => vec![("value".to_string(), other)],
                                };
                                entries.insert(0, (#tag.to_string(), #value_crate::DslValue::String(#wire_variant.to_string())));
                                #value_crate::DslValue::Object(entries)
                            }
                        }),
                        (Fields::Named(named), Some(content)) => {
                            let push_into = quote! { content_entries };
                            let pushes = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs));
                                    variant_field_to_value_push(field, &field_attrs, &wire_name, &ident, &push_into, &value_crate)
                                })
                                .collect::<syn::Result<Vec<_>>>()?;
                            let idents = variant_destructure_patterns(named);
                            Ok(quote! {
                                Self::#variant_ident { #(#idents),* } => {
                                    let mut content_entries: Vec<(String, #value_crate::DslValue)> = Vec::new();
                                    #(#pushes)*
                                    #value_crate::DslValue::object([
                                        (#tag.to_string(), #value_crate::DslValue::String(#wire_variant.to_string())),
                                        (#content.to_string(), #value_crate::DslValue::Object(content_entries)),
                                    ])
                                }
                            })
                        }
                        (Fields::Named(named), None) => {
                            // 🛡️ `__out_entries`, not `entries` — a user field literally named `entries`
                            // (e.g. `SemioValue::Map { entries: Vec<SemioValueEntry> }`) would otherwise
                            // shadow the accumulator once `#(#idents),*` destructures it into scope, making
                            // `ToValue::to_value(#ident)` resolve to the accumulator itself (an owned
                            // `Vec<(String, DslValue)>`) instead of the field's `&Vec<SemioValueEntry>`.
                            let push_into = quote! { __out_entries };
                            let pushes = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs));
                                    variant_field_to_value_push(field, &field_attrs, &wire_name, &ident, &push_into, &value_crate)
                                })
                                .collect::<syn::Result<Vec<_>>>()?;
                            let idents = variant_destructure_patterns(named);
                            Ok(quote! {
                                Self::#variant_ident { #(#idents),* } => {
                                    let mut __out_entries: Vec<(String, #value_crate::DslValue)> = vec![(#tag.to_string(), #value_crate::DslValue::String(#wire_variant.to_string()))];
                                    #(#pushes)*
                                    #value_crate::DslValue::Object(__out_entries)
                                }
                            })
                        }
                        (other, _) => Err(syn::Error::new_spanned(other, "#[derive(ToValue)] enum variants must be unit, a single unnamed payload, or named fields")),
                    };
                    arm
                })
                .collect::<syn::Result<Vec<_>>>()?;
            quote! {
                match self { #(#arms),* }
            }
        }
        Data::Union(_) => return Err(syn::Error::new_spanned(&input.ident, "#[derive(ToValue)] does not support unions")),
    };

    let (path_body, shape_body, key_body) = match &input.data {
        Data::Struct(data) if container.transparent => match &data.fields {
            Fields::Named(named) if named.named.len() == 1 => {
                let ident = named.named.first().expect("checked len == 1").ident.clone().expect("named field");
                (
                    quote! { #value_crate::ToValue::value_at_path(&self.#ident, path) },
                    quote! { #value_crate::ToValue::value_shape_at_path(&self.#ident, path) },
                    quote! { #value_crate::ToValue::value_key_at_path(&self.#ident, path, index) },
                )
            }
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => (
                quote! { #value_crate::ToValue::value_at_path(&self.0, path) },
                quote! { #value_crate::ToValue::value_shape_at_path(&self.0, path) },
                quote! { #value_crate::ToValue::value_key_at_path(&self.0, path, index) },
            ),
            other => return Err(syn::Error::new_spanned(other, "#[value(transparent)] requires exactly one field")),
        },
        Data::Struct(data) if matches!(&data.fields, Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1) => (
            quote! { #value_crate::ToValue::value_at_path(&self.0, path) },
            quote! { #value_crate::ToValue::value_shape_at_path(&self.0, path) },
            quote! { #value_crate::ToValue::value_key_at_path(&self.0, path, index) },
        ),
        Data::Struct(data) => {
            let fields = named_fields(&data.fields, &container)?;
            (
                struct_to_path_body(&fields, &value_crate, "value_at_path"),
                struct_to_path_body(&fields, &value_crate, "value_shape_at_path"),
                struct_key_at_path_body(&fields, &value_crate),
            )
        }
        Data::Enum(data) => (
            enum_to_path_body(data, &container, &value_crate, "value_at_path")?,
            enum_to_path_body(data, &container, &value_crate, "value_shape_at_path")?,
            enum_key_at_path_body(data, &container, &value_crate)?,
        ),
        Data::Union(_) => unreachable!("union rejected above"),
    };

    let controlled_body=controlled_to_body(input,&container,&value_crate)?;

    Ok(quote! {
        impl #impl_generics #value_crate::ToValue for #name #ty_generics #where_clause {
            fn to_value(&self) -> #value_crate::DslValue {
                #body
            }
            fn to_value_controlled(&self,control:&mut #value_crate::NativeEncodeControl<'_>)->::core::result::Result<#value_crate::DslValue,#value_crate::ValueError>{
                control.scoped_depth(64,|control|control.scoped_stage(|control|{#controlled_body}))
            }

            fn value_at_path(&self, path: &[&str]) -> ::core::result::Result<#value_crate::DslValue, #value_crate::ValueError> {
                #path_body
            }

            fn value_shape_at_path(&self, path: &[&str]) -> ::core::result::Result<#value_crate::ValueShape, #value_crate::ValueError> {
                #shape_body
            }

            fn value_key_at_path(&self, path: &[&str], index: usize) -> ::core::result::Result<String, #value_crate::ValueError> {
                #key_body
            }
        }
    })
}

pub fn expand_from_value(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let container = parse_container_attrs(&input.attrs)?;
    let value_crate = container_crate_path(&container);
    let generics = generics_with_bound(&input.generics, &container.bound, &quote! { #value_crate::FromValue });
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = match &input.data {
        Data::Struct(data) if container.transparent => match &data.fields {
            Fields::Named(named) if named.named.len() == 1 => {
                let ident = named.named.first().expect("checked len == 1").ident.clone().expect("named field");
                quote! { Ok(Self { #ident: #value_crate::FromValue::from_value(value)? }) }
            }
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => quote! { Ok(Self(#value_crate::FromValue::from_value(value)?)) },
            other => return Err(syn::Error::new_spanned(other, "#[value(transparent)] requires exactly one field")),
        },
        // 🆔 Single-field tuple struct: sibling of `expand_to_value`'s identical guard above.
        Data::Struct(data) if matches!(&data.fields, Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1) => {
            quote! { Ok(Self(#value_crate::FromValue::from_value(value)?)) }
        }
        Data::Struct(data) => {
            let fields = named_fields(&data.fields, &container)?;
            let reads = from_value_struct_fields(&fields, &container, &value_crate);
            quote! {
                let __entries = #value_crate::DslValue::into_object(value)?;
                #reads
            }
        }
        Data::Enum(data) if container.tag.is_none() && data.variants.iter().all(|variant| matches!(variant.fields, Fields::Unit)) => {
            let arms = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                Ok(quote! { #wire_variant => Ok(Self::#variant_ident), })
            }).collect::<syn::Result<Vec<_>>>()?;
            quote! {
                let __s = match value { #value_crate::DslValue::String(s) => s, other => return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))) };
                match __s.as_str() {
                    #(#arms)*
                    other => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("unknown variant `{other}`"))),
                }
            }
        }
        Data::Enum(data) if container.tag.is_none() => {
            // 🏷️ Externally-tagged (serde's own default enum representation when no `#[serde(tag
            // = …)]` is present) — mirrors `expand_to_value`'s sibling arm above.
            let string_arms = data.variants.iter().filter(|variant| matches!(variant.fields, Fields::Unit)).map(|variant| {
                let variant_ident = &variant.ident;
                let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                Ok(quote! { #wire_variant => return Ok(Self::#variant_ident), })
            }).collect::<syn::Result<Vec<_>>>()?;
            let object_arms = data
                .variants
                .iter()
                .filter(|variant| !matches!(variant.fields, Fields::Unit))
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                    let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                    let arm: syn::Result<proc_macro2::TokenStream> = match &variant.fields {
                        Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                            let payload_ty = &unnamed.unnamed[0].ty;
                            Ok(quote! {
                                #wire_variant => Self::#variant_ident(<#payload_ty as #value_crate::FromValue>::from_value(__payload)?),
                            })
                        }
                        Fields::Named(named) => {
                            let field_wire_names: Vec<String> = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs))
                                })
                                .collect();
                            let deny_check = if container.deny_unknown_fields {
                                deny_unknown_keys(&quote! { __variant_entries }, &field_wire_names, &value_crate)
                            } else {
                                quote! {}
                            };
                            let entries_ident = quote! { __variant_entries };
                            let reads = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs));
                                    variant_field_from_value_read(field, &field_attrs, &wire_name, &ident, &entries_ident, &value_crate)
                                })
                                .collect::<syn::Result<Vec<_>>>()?;
                            let idents = named.named.iter().map(|field| field.ident.clone().expect("named field"));
                            Ok(quote! {
                                #wire_variant => {
                                    let __variant_entries = #value_crate::DslValue::into_object(__payload)?;
                                    #deny_check
                                    #(#reads)*
                                    Self::#variant_ident { #(#idents),* }
                                },
                            })
                        }
                        other => Err(syn::Error::new_spanned(other, "#[derive(FromValue)] externally-tagged enum variants must be unit, a single unnamed payload, or named fields")),
                    };
                    arm
                })
                .collect::<syn::Result<Vec<_>>>()?;
            quote! {
                if let #value_crate::DslValue::String(__s) = &value {
                    match __s.as_str() {
                        #(#string_arms)*
                        _ => {}
                    }
                }
                let __entries = #value_crate::DslValue::into_object(value)?;
                if __entries.len() != 1 {
                    return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected an externally-tagged enum object with exactly one key, found {} keys", __entries.len())));
                }
                let (__key, __payload) = __entries.into_iter().next().expect("checked len == 1 above");
                Ok(match __key.as_str() {
                    #(#object_arms)*
                    other => return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("unknown variant `{other}`"))),
                })
            }
        }
        Data::Enum(data) => {
            let Some(tag) = &container.tag else {
                unreachable!("the tag.is_none() arm above already handles every tag-less enum");
            };
            let arms = data
                .variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let variant_attrs = parse_variant_attrs(&variant.attrs)?;
                    let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
                    let arm: syn::Result<proc_macro2::TokenStream> = match (&variant.fields, &container.content) {
                        (Fields::Unit, Some(_)) => Ok(quote! {
                            #wire_variant => Self::#variant_ident,
                        }),
                        (Fields::Unit, None) => {
                            // 🛡️ Internally-tagged unit variant: the whole entries object is nothing
                            // but the tag, so `deny_unknown_fields` allows exactly `{tag}`.
                            let deny_check = if container.deny_unknown_fields {
                                deny_unknown_keys(&quote! { __entries }, std::slice::from_ref(tag), &value_crate)
                            } else {
                                quote! {}
                            };
                            Ok(quote! {
                                #wire_variant => { #deny_check Self::#variant_ident },
                            })
                        }
                        (Fields::Unnamed(unnamed), Some(_)) if unnamed.unnamed.len() == 1 => {
                            let payload_ty = &unnamed.unnamed[0].ty;
                            Ok(quote! {
                                #wire_variant => Self::#variant_ident(<#payload_ty as #value_crate::FromValue>::from_value(__content()?)?),
                            })
                        }
                        (Fields::Unnamed(unnamed), None) if unnamed.unnamed.len() == 1 => {
                            // 🩹 Strip the tag key before handing the object to the payload type's
                            // own `FromValue` — `expand_to_value`'s sibling arm never puts the tag
                            // INTO the payload's own entries (it prepends the tag after taking the
                            // payload's `to_value()`, so the payload never emits it either), so
                            // leaving the tag in here was a decode/encode asymmetry: a payload type
                            // that itself carries `#[value(deny_unknown_fields)]` would reject its
                            // own valid wire form because the wrapper's tag key looked unknown to it.
                            //
                            // 🪆 The `__scalar` arm is the exact inverse of `expand_to_value`'s runtime
                            // branch: a payload whose `to_value()` is NOT an object cannot be spliced
                            // beside the tag, so the encoder carries it as the single entry
                            // `{"value": <scalar>}`. Which of the two shapes a given wire object is
                            // cannot be decided from the payload TYPE at expansion time (a struct whose
                            // only field is literally named `value` produces the same key set), so the
                            // object form is attempted first and the carrier is unwrapped only when it
                            // fails — that ordering decodes both shapes correctly, where a key-shape
                            // test alone would mis-decode `struct P { value: String }`.
                            let payload_ty = &unnamed.unnamed[0].ty;
                            Ok(quote! {
                                #wire_variant => Self::#variant_ident({
                                    let __payload: Vec<(String, #value_crate::DslValue)> = __entries.iter().filter(|(__k, _)| __k != #tag).cloned().collect();
                                    match <#payload_ty as #value_crate::FromValue>::from_value(#value_crate::DslValue::Object(__payload.clone())) {
                                        ::core::result::Result::Ok(__decoded) => __decoded,
                                        ::core::result::Result::Err(__object_error) => match __payload.as_slice() {
                                            [(__k, __scalar)] if __k == "value" => <#payload_ty as #value_crate::FromValue>::from_value(__scalar.clone())?,
                                            _ => return ::core::result::Result::Err(__object_error),
                                        },
                                    }
                                }),
                            })
                        }
                        (Fields::Named(named), content_key) => {
                            let source = if content_key.is_some() {
                                quote! { __content()?.into_object()? }
                            } else {
                                quote! { __entries.clone() }
                            };
                            let field_wire_names: Vec<String> = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs))
                                })
                                .collect();
                            let deny_check = if container.deny_unknown_fields {
                                let allowed: Vec<String> = if content_key.is_some() {
                                    field_wire_names
                                } else {
                                    let mut allowed = vec![tag.clone()];
                                    allowed.extend(field_wire_names);
                                    allowed
                                };
                                deny_unknown_keys(&quote! { __variant_entries }, &allowed, &value_crate)
                            } else {
                                quote! {}
                            };
                            let entries_ident = quote! { __variant_entries };
                            let reads = named
                                .named
                                .iter()
                                .map(|field| {
                                    let field_attrs = parse_field_attrs(&field.attrs).unwrap_or_default();
                                    let ident = field.ident.clone().expect("named field");
                                    let wire_name = field_wire_name(&ident.to_string(), &field_attrs.rename, &container.field_rename_all(&variant_attrs));
                                    variant_field_from_value_read(field, &field_attrs, &wire_name, &ident, &entries_ident, &value_crate)
                                })
                                .collect::<syn::Result<Vec<_>>>()?;
                            let idents = named.named.iter().map(|field| field.ident.clone().expect("named field"));
                            Ok(quote! {
                                #wire_variant => {
                                    let __variant_entries = #source;
                                    #deny_check
                                    #(#reads)*
                                    Self::#variant_ident { #(#idents),* }
                                },
                            })
                        }
                        (other, _) => Err(syn::Error::new_spanned(other, "#[derive(FromValue)] enum variants must be unit, a single unnamed payload, or named fields")),
                    };
                    arm
                })
                .collect::<syn::Result<Vec<_>>>()?;
            let content_helper = match &container.content {
                Some(content) => quote! {
                    let __content = || -> ::core::result::Result<#value_crate::DslValue, #value_crate::ValueError> {
                        __entries.iter().find(|(k, _)| k == #content).map(|(_, v)| v.clone()).ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing content field `{}`", #content)))
                    };
                },
                None => quote! {},
            };
            // 🛡️ Adjacently-tagged outer-level `deny_unknown_fields`: the allowed key set here is
            // just `{tag, content}` regardless of which variant matches, so this check runs once,
            // before the tag is even read — unlike the internally-tagged case (no `content`),
            // where the allowed set depends on the variant and is checked per-arm above instead.
            let outer_deny_check = match (&container.content, container.deny_unknown_fields) {
                (Some(content), true) => deny_unknown_keys(&quote! { __entries }, &[tag.clone(), content.clone()], &value_crate),
                _ => quote! {},
            };
            quote! {
                let __entries = #value_crate::DslValue::into_object(value)?;
                #outer_deny_check
                let __tag = __entries.iter().find(|(k, _)| k == #tag).map(|(_, v)| v.clone()).ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing tag field `{}`", #tag)))?;
                let __tag = match __tag { #value_crate::DslValue::String(s) => s, other => return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected a string tag, found {other:?}"))) };
                #content_helper
                Ok(match __tag.as_str() {
                    #(#arms)*
                    other => return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("unknown `{}` variant `{other}`", #tag))),
                })
            }
        }
        Data::Union(_) => return Err(syn::Error::new_spanned(&input.ident, "#[derive(FromValue)] does not support unions")),
    };

    let body = if let Some(path) = &container.validate {
        let validator: syn::Path = syn::parse_str(path)?;
        quote! {
            let decoded = (|| -> ::core::result::Result<Self, #value_crate::ValueError> { #body })()?;
            if let Err(error) = #validator(&decoded) {
                <Self as #value_crate::FromValue>::retire_decoded(decoded);
                return Err(error);
            }
            Ok(decoded)
        }
    } else { body };
    let controlled_body = controlled_from_body(input, &container, &value_crate)?;
    let controlled_body = controlled_semantic_admission(controlled_body, &container, &value_crate, container.deserialize_controlled_with.is_some())?;
    let retirement_body = controlled_retirement_body(input, &container, &value_crate)?;
    let controlled_default_method = if let Some(path) = &container.default_controlled {
        let path: syn::Path = syn::parse_str(path)?;
        let body = controlled_semantic_admission(quote! { #path(control) }, &container, &value_crate, true)?;
        quote! { fn default_value_controlled(control: &mut #value_crate::NativeDecodeControl<'_>) -> Result<Self, #value_crate::ValueError> { #body } }
    } else { quote! {} };
    let edit_body = match &input.data {
        Data::Struct(data) if container.transparent => match &data.fields {
            Fields::Named(named) if named.named.len() == 1 => {
                let ident = named.named.first().expect("checked len == 1").ident.clone().expect("named field");
                quote! { #value_crate::FromValue::edit_value_at_path(&mut self.#ident, path, edit) }
            }
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => quote! { #value_crate::FromValue::edit_value_at_path(&mut self.0, path, edit) },
            other => return Err(syn::Error::new_spanned(other, "#[value(transparent)] requires exactly one field")),
        },
        Data::Struct(data) if matches!(&data.fields, Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1) => {
            quote! { #value_crate::FromValue::edit_value_at_path(&mut self.0, path, edit) }
        }
        Data::Struct(data) => {
            let fields = named_fields(&data.fields, &container)?;
            struct_edit_path_body(&fields, &value_crate)
        }
        Data::Enum(data) => enum_edit_path_body(data, &container, &value_crate)?,
        Data::Union(_) => unreachable!("union rejected above"),
    };

    Ok(quote! {
        impl #impl_generics #value_crate::FromValue for #name #ty_generics #where_clause {
            fn from_value(value: #value_crate::DslValue) -> ::core::result::Result<Self, #value_crate::ValueError> {
                #body
            }

            fn from_value_controlled(value: &#value_crate::DslValue, control: &mut #value_crate::NativeDecodeControl<'_>) -> ::core::result::Result<Self, #value_crate::ValueError> {
                control.scoped_depth(64, |control| control.scoped_stage(|control| { #controlled_body }))
            }
            #controlled_default_method
            fn retire_decoded(self) { #retirement_body }
            fn edit_value_at_path(&mut self, path: &[&str], edit: #value_crate::ValueEdit) -> ::core::result::Result<(), #value_crate::ValueError> {
                #edit_body
            }
        }
    })
}
//#endregion 🔖️Expand
