//! 🧭️ Typed path, key and edit bodies for structs and enums.

use quote::quote;
use syn::Fields;
use super::{ContainerAttrs, NamedField, VariantAttrs, check_variant_field_attrs_supported, field_wire_name, parse_field_attrs, parse_variant_attrs, type_is_option, variant_path_bindings, variant_wire_name};

//#region 🧭️TypedPath
fn field_to_path_call(
    field: &NamedField,
    access: &proc_macro2::TokenStream,
    path: &proc_macro2::TokenStream,
    method: &syn::Ident,
    value_crate: &syn::Path,
) -> proc_macro2::TokenStream {
    match field.attrs.effective_serialize_with() {
        Some(serializer) => {
            let serializer: syn::Path = syn::parse_str(&serializer).expect("valid serialize_with path");
            quote! { #value_crate::ToValue::#method(&#serializer(#access), #path) }
        }
        None => quote! { #value_crate::ToValue::#method(#access, #path) },
    }
}

pub(super) fn struct_to_path_body(fields: &[NamedField], value_crate: &syn::Path, method: &str) -> proc_macro2::TokenStream {
    let method = syn::Ident::new(method, proc_macro2::Span::call_site());
    let direct_arms = fields.iter().filter(|field| !field.attrs.skip && !field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_to_path_call(field, &quote! { &self.#ident }, &quote! { __rest }, &method, value_crate);
        let omitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
            let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
            quote! {
                if #predicate(&self.#ident) {
                    return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                }
            }
        });
        quote! {
            #wire_name => {
                #omitted
                #call.map_err(|error| error.under(__segment))
            }
        }
    });
    let flatten_attempts = fields.iter().filter(|field| !field.attrs.skip && field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let call = field_to_path_call(field, &quote! { &self.#ident }, &quote! { path }, &method, value_crate);
        quote! {
            if let Ok(value) = #call {
                return Ok(value);
            }
        }
    });
    let root = if method == "value_shape_at_path" {
        let normal_counts = fields.iter().filter(|field| !field.attrs.skip && !field.attrs.flatten).map(|field| {
            let ident = &field.ident;
            match &field.attrs.skip_serializing_if {
                Some(predicate) => {
                    let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
                    quote! { if !#predicate(&self.#ident) { __len += 1; } }
                }
                None => quote! { __len += 1; },
            }
        });
        let flattened_counts = fields.iter().filter(|field| !field.attrs.skip && field.attrs.flatten).map(|field| {
            let ident = &field.ident;
            let call = field_to_path_call(field, &quote! { &self.#ident }, &quote! { &[] }, &method, value_crate);
            quote! {
                match #call? {
                    #value_crate::ValueShape::Object { len } => __len = __len.checked_add(len).ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::OwnershipLimit, "object length overflow"))?,
                    _ => return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "flattened field is not an object")),
                }
            }
        });
        quote! {
            if path.is_empty() {
                let mut __len = 0usize;
                #(#normal_counts)*
                #(#flattened_counts)*
                return Ok(#value_crate::ValueShape::Object { len: __len });
            }
        }
    } else {
        quote! {
            if path.is_empty() {
                return Ok(#value_crate::ToValue::to_value(self));
            }
        }
    };
    quote! {
        #root
        let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
        match *__segment {
            #(#direct_arms,)*
            _ => {
                #(#flatten_attempts)*
                Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)))
            }
        }
    }
}

fn field_to_key_call(field: &NamedField, access: &proc_macro2::TokenStream, path: &proc_macro2::TokenStream, value_crate: &syn::Path) -> proc_macro2::TokenStream {
    match field.attrs.effective_serialize_with() {
        Some(serializer) => {
            let serializer: syn::Path = syn::parse_str(&serializer).expect("valid serialize_with path");
            quote! { #value_crate::ToValue::value_key_at_path(&#serializer(#access), #path, index) }
        }
        None => quote! { #value_crate::ToValue::value_key_at_path(#access, #path, index) },
    }
}

pub(super) fn struct_key_at_path_body(fields: &[NamedField], value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let root_steps = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let admitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
            let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
            quote! { !#predicate(&self.#ident) }
        }).unwrap_or_else(|| quote! { true });
        if field.attrs.flatten {
            let shape = field_to_path_call(field, &quote! { &self.#ident }, &quote! { &[] }, &syn::Ident::new("value_shape_at_path", proc_macro2::Span::call_site()), value_crate);
            let key = field_to_key_call(field, &quote! { &self.#ident }, &quote! { &[] }, value_crate);
            quote! {
                if #admitted {
                    let #value_crate::ValueShape::Object { len } = #shape? else {
                        return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "flattened field is not an object"));
                    };
                    if index < __offset + len {
                        let index = index - __offset;
                        return #key;
                    }
                    __offset += len;
                }
            }
        } else {
            quote! {
                if #admitted {
                    if index == __offset { return Ok(#wire_name.to_owned()); }
                    __offset += 1;
                }
            }
        }
    });
    let direct_arms = fields.iter().filter(|field| !field.attrs.skip && !field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_to_key_call(field, &quote! { &self.#ident }, &quote! { __rest }, value_crate);
        let omitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
            let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
            quote! {
                if #predicate(&self.#ident) {
                    return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                }
            }
        });
        quote! {
            #wire_name => {
                #omitted
                #call.map_err(|error| error.under(__segment))
            }
        }
    });
    let flatten_attempts = fields.iter().filter(|field| !field.attrs.skip && field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let call = field_to_key_call(field, &quote! { &self.#ident }, &quote! { path }, value_crate);
        quote! { if let Ok(key) = #call { return Ok(key); } }
    });
    quote! {
        if path.is_empty() {
            let mut __offset = 0usize;
            #(#root_steps)*
            return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {__offset}")));
        }
        let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
        match *__segment {
            #(#direct_arms,)*
            _ => {
                #(#flatten_attempts)*
                Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)))
            }
        }
    }
}

fn field_edit_call(
    field: &NamedField,
    access: &proc_macro2::TokenStream,
    path: &proc_macro2::TokenStream,
    edit: &proc_macro2::TokenStream,
    value_crate: &syn::Path,
) -> proc_macro2::TokenStream {
    match (field.attrs.effective_serialize_with(), field.attrs.effective_deserialize_with()) {
        (Some(serializer), Some(deserializer)) => {
            let serializer: syn::Path = syn::parse_str(&serializer).expect("valid serialize_with path");
            let deserializer: syn::Path = syn::parse_str(&deserializer).expect("valid deserialize_with path");
            quote! {
                {
                    let mut __field_value = #serializer(&*#access);
                    #value_crate::FromValue::edit_value_at_path(&mut __field_value, #path, #edit)?;
                    let __replacement = #deserializer(__field_value)?;
                    *#access = __replacement;
                    Ok::<(), #value_crate::ValueError>(())
                }
            }
        }
        (Some(_), None) => quote! {
            Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::UnsupportedOwner, "custom wire field has no matching decoder for a typed-path edit"))
        },
        (None, Some(deserializer)) => {
            let deserializer: syn::Path = syn::parse_str(&deserializer).expect("valid deserialize_with path");
            quote! {
                if #path.is_empty() {
                    match #edit {
                        #value_crate::ValueEdit::Set(value) => {
                            let __replacement = #deserializer(value)?;
                            *#access = __replacement;
                            Ok::<(), #value_crate::ValueError>(())
                        }
                        #value_crate::ValueEdit::Insert(_) | #value_crate::ValueEdit::InsertAt { .. } => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot insert a required field")),
                        #value_crate::ValueEdit::Remove => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot remove a required field")),
                    }
                } else {
                    Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::UnsupportedOwner, "custom wire field has no matching encoder for a nested typed-path edit"))
                }
            }
        }
        (None, None) => quote! { #value_crate::FromValue::edit_value_at_path(#access, #path, #edit) },
    }
}

pub(super) fn struct_edit_path_body(fields: &[NamedField], value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let direct_arms = fields.iter().filter(|field| !field.attrs.skip && !field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_edit_call(field, &quote! { &mut self.#ident }, &quote! { __rest }, &quote! { edit }, value_crate);
        quote! { #wire_name => #call.map_err(|error| error.under(__segment)) }
    });
    let flatten_attempts = fields.iter().filter(|field| !field.attrs.skip && field.attrs.flatten).map(|field| {
        let ident = &field.ident;
        let call = field_edit_call(field, &quote! { &mut self.#ident }, &quote! { path }, &quote! { edit.clone() }, value_crate);
        quote! {
            if #call.is_ok() {
                return Ok(());
            }
        }
    });
    quote! {
        if path.is_empty() {
            return match edit {
                #value_crate::ValueEdit::Set(value) => {
                    let replacement = <Self as #value_crate::FromValue>::from_value(value)?;
                    *self = replacement;
                    Ok(())
                }
                #value_crate::ValueEdit::Insert(_) | #value_crate::ValueEdit::InsertAt { .. } => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot insert at the record root")),
                #value_crate::ValueEdit::Remove => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot remove the record root")),
            };
        }
        let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
        match *__segment {
            #(#direct_arms,)*
            _ => {
                #(#flatten_attempts)*
                Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)))
            }
        }
    }
}

fn variant_named_fields(named: &syn::FieldsNamed, container: &ContainerAttrs, variant: &VariantAttrs) -> syn::Result<Vec<NamedField>> {
    named
        .named
        .iter()
        .map(|field| {
            let attrs = parse_field_attrs(&field.attrs)?;
            check_variant_field_attrs_supported(field, &attrs)?;
            let ident = field.ident.clone().expect("named field");
            let wire_name = field_wire_name(&ident.to_string(), &attrs.rename, &container.field_rename_all(variant));
            Ok(NamedField { ident, wire_name, attrs, is_option: type_is_option(&field.ty) })
        })
        .collect()
}

fn named_variant_edit_dispatch(fields: &[NamedField], path: &proc_macro2::TokenStream, edit: &proc_macro2::TokenStream, value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let arms = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_edit_call(field, &quote! { #ident }, &quote! { __rest }, edit, value_crate);
        quote! { #wire_name => #call.map_err(|error| error.under(__segment)) }
    });
    quote! {
        let (__segment, __rest) = #path.split_first().ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot structurally replace an enum variant payload"))?;
        match *__segment {
            #(#arms,)*
            _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment))),
        }
    }
}

fn named_variant_to_path_dispatch(fields: &[NamedField], path: &proc_macro2::TokenStream, method: &str, value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
    let arms = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_to_path_call(field, &quote! { #ident }, &quote! { __rest }, &method_ident, value_crate);
        let omitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
            let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
            quote! {
                if #predicate(#ident) {
                    return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                }
            }
        });
        quote! {
            #wire_name => {
                #omitted
                #call.map_err(|error| error.under(__segment))
            }
        }
    });
    let root = if method == "value_shape_at_path" {
        let counts = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
            let ident = &field.ident;
            match &field.attrs.skip_serializing_if {
                Some(predicate) => {
                    let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
                    quote! { if !#predicate(#ident) { __len += 1; } }
                }
                None => quote! { __len += 1; },
            }
        });
        quote! {
            if #path.is_empty() {
                let mut __len = 0usize;
                #(#counts)*
                return Ok(#value_crate::ValueShape::Object { len: __len });
            }
        }
    } else {
        let pushes = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
            let ident = &field.ident;
            let wire_name = &field.wire_name;
            let value = match field.attrs.effective_serialize_with() {
                Some(serializer) => {
                    let serializer: syn::Path = syn::parse_str(&serializer).expect("valid serialize_with path");
                    quote! { #serializer(#ident) }
                }
                None => quote! { #value_crate::ToValue::to_value(#ident) },
            };
            match &field.attrs.skip_serializing_if {
                Some(predicate) => {
                    let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
                    quote! { if !#predicate(#ident) { __entries.push((#wire_name.to_owned(), #value)); } }
                }
                None => quote! { __entries.push((#wire_name.to_owned(), #value)); },
            }
        });
        quote! {
            if #path.is_empty() {
                let mut __entries = Vec::new();
                #(#pushes)*
                return Ok(#value_crate::DslValue::Object(__entries));
            }
        }
    };
    quote! {
        #root
        let (__segment, __rest) = #path.split_first().expect("non-empty variant path checked above");
        match *__segment {
            #(#arms,)*
            _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment))),
        }
    }
}

fn named_variant_key_dispatch(fields: &[NamedField], path: &proc_macro2::TokenStream, value_crate: &syn::Path) -> proc_macro2::TokenStream {
    let root_steps = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let admitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
            let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
            quote! { !#predicate(#ident) }
        }).unwrap_or_else(|| quote! { true });
        quote! {
            if #admitted {
                if index == __offset { return Ok(#wire_name.to_owned()); }
                __offset += 1;
            }
        }
    });
    let arms = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
        let ident = &field.ident;
        let wire_name = &field.wire_name;
        let call = field_to_key_call(field, &quote! { #ident }, &quote! { __rest }, value_crate);
        quote! { #wire_name => #call.map_err(|error| error.under(__segment)) }
    });
    quote! {
        if #path.is_empty() {
            let mut __offset = 0usize;
            #(#root_steps)*
            return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {__offset}")));
        }
        let (__segment, __rest) = #path.split_first().expect("non-empty path checked above");
        match *__segment {
            #(#arms,)*
            _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment))),
        }
    }
}

pub(super) fn enum_key_at_path_body(data: &syn::DataEnum, container: &ContainerAttrs, value_crate: &syn::Path) -> syn::Result<proc_macro2::TokenStream> {
    if data.variants.is_empty() {
        return Ok(quote! { match *self {} });
    }
    let arms = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let variant_attrs = parse_variant_attrs(&variant.attrs)?;
            let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
            match &variant.fields {
                Fields::Unit => {
                    let dispatch = if let Some(tag) = &container.tag {
                        quote! {
                            if path.is_empty() {
                                return if index == 0 { Ok(#tag.to_owned()) } else { Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 1"))) };
                            }
                            Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("variant `{}` has no object child at the requested path", #wire_variant)))
                        }
                    } else {
                        quote! { Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected an object, found unit variant `{}`", #wire_variant))) }
                    };
                    Ok(quote! { Self::#variant_ident => { #dispatch } })
                }
                Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                    let dispatch = if container.tag.is_none() {
                        quote! {
                            if path.is_empty() {
                                return if index == 0 { Ok(#wire_variant.to_owned()) } else { Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 1"))) };
                            }
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #value_crate::ToValue::value_key_at_path(payload, __rest, index).map_err(|error| error.under(__segment))
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        quote! {
                            if path.is_empty() {
                                return match index {
                                    0 => Ok(#tag.to_owned()),
                                    1 => Ok(#content.to_owned()),
                                    _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 2"))),
                                };
                            }
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected an object below `{}`, found `{}`", #content, __segment)));
                            }
                            #value_crate::ToValue::value_key_at_path(payload, __rest, index).map_err(|error| error.under(__segment))
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        quote! {
                            if path.is_empty() {
                                if index == 0 { return Ok(#tag.to_owned()); }
                                return match #value_crate::ToValue::value_shape_at_path(payload, &[])? {
                                    #value_crate::ValueShape::Object { len } if index <= len => #value_crate::ToValue::value_key_at_path(payload, &[], index - 1),
                                    #value_crate::ValueShape::Object { len } => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {}", len + 1))),
                                    _ if index == 1 => Ok("value".to_owned()),
                                    _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 2"))),
                                };
                            }
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "enum tag is not an object"));
                            }
                            if matches!(#value_crate::ToValue::value_shape_at_path(payload, &[])?, #value_crate::ValueShape::Object { .. }) {
                                return #value_crate::ToValue::value_key_at_path(payload, path, index);
                            }
                            if !path.first().is_some_and(|segment| *segment == "value") {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", path[0])));
                            }
                            #value_crate::ToValue::value_key_at_path(payload, &path[1..], index)
                        }
                    };
                    Ok(quote! { Self::#variant_ident(payload) => { #dispatch } })
                }
                Fields::Named(named) => {
                    let fields = variant_named_fields(named, container, &variant_attrs)?;
                    let (bindings, fields) = variant_path_bindings(&fields);
                    let pattern = quote! { Self::#variant_ident { #(#bindings),* } };
                    let dispatch = if container.tag.is_none() {
                        let named = named_variant_key_dispatch(&fields, &quote! { __rest }, value_crate);
                        quote! {
                            if path.is_empty() {
                                return if index == 0 { Ok(#wire_variant.to_owned()) } else { Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 1"))) };
                            }
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #named
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let named = named_variant_key_dispatch(&fields, &quote! { __rest }, value_crate);
                        quote! {
                            if path.is_empty() {
                                return match index {
                                    0 => Ok(#tag.to_owned()), 1 => Ok(#content.to_owned()),
                                    _ => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length 2"))),
                                };
                            }
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("expected an object below `{}`, found `{}`", #content, __segment)));
                            }
                            #named
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let root_steps = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
                            let ident = &field.ident;
                            let wire_name = &field.wire_name;
                            let admitted = field.attrs.skip_serializing_if.as_ref().map(|predicate| {
                                let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
                                quote! { !#predicate(#ident) }
                            }).unwrap_or_else(|| quote! { true });
                            quote! {
                                if #admitted {
                                    if index == __offset { return Ok(#wire_name.to_owned()); }
                                    __offset += 1;
                                }
                            }
                        });
                        let named_path = named_variant_key_dispatch(&fields, &quote! { path }, value_crate);
                        quote! {
                            if path.is_empty() {
                                if index == 0 { return Ok(#tag.to_owned()); }
                                let mut __offset = 1usize;
                                #(#root_steps)*
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {__offset}")));
                            }
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "enum tag is not an object"));
                            }
                            #named_path
                        }
                    };
                    Ok(quote! { #pattern => { #dispatch } })
                }
                other => Err(syn::Error::new_spanned(other, "#[derive(ToValue)] enum variants must be unit, a single unnamed payload, or named fields")),
            }
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! { match self { #(#arms),* } })
}

pub(super) fn enum_to_path_body(data: &syn::DataEnum, container: &ContainerAttrs, value_crate: &syn::Path, method: &str) -> syn::Result<proc_macro2::TokenStream> {
    if data.variants.is_empty() {
        return Ok(quote! { match *self {} });
    }
    let method_ident = syn::Ident::new(method, proc_macro2::Span::call_site());
    let shape = method == "value_shape_at_path";
    let tag_value = |wire_variant: &str| {
        if shape {
            quote! { Ok(#value_crate::ValueShape::String) }
        } else {
            quote! { Ok(#value_crate::DslValue::String(#wire_variant.to_owned())) }
        }
    };
    let arms = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let variant_attrs = parse_variant_attrs(&variant.attrs)?;
            let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
            match &variant.fields {
                Fields::Unit => {
                    let tag_result = tag_value(&wire_variant);
                    let root = if shape {
                        if container.tag.is_some() {
                            quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::Object { len: 1 }); } }
                        } else {
                            quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::String); } }
                        }
                    } else {
                        quote! {}
                    };
                    let dispatch = if let Some(tag) = &container.tag {
                        quote! {
                            #root
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment == #tag && __rest.is_empty() {
                                #tag_result
                            } else {
                                Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("variant `{}` has no child `{}`", #wire_variant, __segment)))
                            }
                        }
                    } else {
                        quote! { #root Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("variant `{}` has no children", #wire_variant))) }
                    };
                    Ok(quote! { Self::#variant_ident => { #dispatch } })
                }
                Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                    let call = quote! { #value_crate::ToValue::#method_ident(payload, __rest) };
                    let dispatch = if container.tag.is_none() {
                        let root = if shape { quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::Object { len: 1 }); } } } else { quote! {} };
                        quote! {
                            #root
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #call.map_err(|error| error.under(__segment))
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let tag_result = tag_value(&wire_variant);
                        let root = if shape { quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::Object { len: 2 }); } } } else { quote! {} };
                        quote! {
                            #root
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment == #tag && __rest.is_empty() {
                                return #tag_result;
                            }
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                            }
                            #call.map_err(|error| error.under(__segment))
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let tag_result = tag_value(&wire_variant);
                        let root = if shape {
                            quote! {
                                if path.is_empty() {
                                    return match #value_crate::ToValue::value_shape_at_path(payload, &[])? {
                                        #value_crate::ValueShape::Object { len } => Ok(#value_crate::ValueShape::Object { len: len.checked_add(1).ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::OwnershipLimit, "object length overflow"))? }),
                                        _ => Ok(#value_crate::ValueShape::Object { len: 2 }),
                                    };
                                }
                            }
                        } else {
                            quote! {}
                        };
                        quote! {
                            #root
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                if path.len() == 1 { return #tag_result; }
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "enum tag has no children"));
                            }
                            if matches!(#value_crate::ToValue::value_shape_at_path(payload, &[])?, #value_crate::ValueShape::Object { .. }) {
                                return #value_crate::ToValue::#method_ident(payload, path);
                            }
                            if !path.first().is_some_and(|segment| *segment == "value") {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", path[0])));
                            }
                            #value_crate::ToValue::#method_ident(payload, &path[1..])
                        }
                    };
                    Ok(quote! { Self::#variant_ident(payload) => { #dispatch } })
                }
                Fields::Named(named) => {
                    let fields = variant_named_fields(named, container, &variant_attrs)?;
                    let (bindings, fields) = variant_path_bindings(&fields);
                    let pattern = quote! { Self::#variant_ident { #(#bindings),* } };
                    let dispatch = if container.tag.is_none() {
                        let named_dispatch = named_variant_to_path_dispatch(&fields, &quote! { __rest }, method, value_crate);
                        let root = if shape { quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::Object { len: 1 }); } } } else { quote! {} };
                        quote! {
                            #root
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #named_dispatch
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let tag_result = tag_value(&wire_variant);
                        let named_dispatch = named_variant_to_path_dispatch(&fields, &quote! { __rest }, method, value_crate);
                        let root = if shape { quote! { if path.is_empty() { return Ok(#value_crate::ValueShape::Object { len: 2 }); } } } else { quote! {} };
                        quote! {
                            #root
                            let (__segment, __rest) = path.split_first().expect("non-empty path checked above");
                            if *__segment == #tag && __rest.is_empty() { return #tag_result; }
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                            }
                            #named_dispatch
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let tag_result = tag_value(&wire_variant);
                        let named_dispatch = named_variant_to_path_dispatch(&fields, &quote! { path }, method, value_crate);
                        let root = if shape {
                            let counts = fields.iter().filter(|field| !field.attrs.skip).map(|field| {
                                let ident = &field.ident;
                                match &field.attrs.skip_serializing_if {
                                    Some(predicate) => {
                                        let predicate: syn::Path = syn::parse_str(predicate).expect("valid skip_serializing_if path");
                                        quote! { if !#predicate(#ident) { __len += 1; } }
                                    }
                                    None => quote! { __len += 1; },
                                }
                            });
                            quote! {
                                if path.is_empty() {
                                    let mut __len = 1usize;
                                    #(#counts)*
                                    return Ok(#value_crate::ValueShape::Object { len: __len });
                                }
                            }
                        } else {
                            quote! {}
                        };
                        quote! {
                            #root
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                if path.len() == 1 { return #tag_result; }
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "enum tag has no children"));
                            }
                            #named_dispatch
                        }
                    };
                    Ok(quote! { #pattern => { #dispatch } })
                }
                other => Err(syn::Error::new_spanned(other, "#[derive(ToValue)] enum variants must be unit, a single unnamed payload, or named fields")),
            }
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let root = if shape {
        quote! {}
    } else {
        quote! { return Ok(#value_crate::ToValue::to_value(self)); }
    };
    Ok(quote! {
        if path.is_empty() { #root }
        match self { #(#arms),* }
    })
}

pub(super) fn enum_edit_path_body(data: &syn::DataEnum, container: &ContainerAttrs, value_crate: &syn::Path) -> syn::Result<proc_macro2::TokenStream> {
    if data.variants.is_empty() {
        return Ok(quote! { match *self {} });
    }
    let arms = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let variant_attrs = parse_variant_attrs(&variant.attrs)?;
            let wire_variant = variant_wire_name(&variant_ident.to_string(), &variant_attrs.rename, &container.rename_all);
            match &variant.fields {
                Fields::Unit => Ok(quote! {
                    Self::#variant_ident => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("variant `{}` has no editable payload", #wire_variant)))
                }),
                Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                    let dispatch = if container.tag.is_none() {
                        quote! {
                            let (__segment, __rest) = path.split_first().ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "missing external variant path"))?;
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #value_crate::FromValue::edit_value_at_path(payload, __rest, edit).map_err(|error| error.under(__segment))
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        quote! {
                            let (__segment, __rest) = path.split_first().ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "missing adjacent enum path"))?;
                            if *__segment == #tag {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot edit an enum tag in place"));
                            }
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                            }
                            #value_crate::FromValue::edit_value_at_path(payload, __rest, edit).map_err(|error| error.under(__segment))
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        quote! {
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot edit an enum tag in place"));
                            }
                            match #value_crate::FromValue::edit_value_at_path(payload, path, edit.clone()) {
                                Ok(()) => Ok(()),
                                Err(object_error) if path.first().is_some_and(|segment| *segment == "value") => {
                                    #value_crate::FromValue::edit_value_at_path(payload, &path[1..], edit).map_err(|_| object_error)
                                }
                                Err(error) => Err(error),
                            }
                        }
                    };
                    Ok(quote! { Self::#variant_ident(payload) => { #dispatch } })
                }
                Fields::Named(named) => {
                    let fields = variant_named_fields(named, container, &variant_attrs)?;
                    let (bindings, fields) = variant_path_bindings(&fields);
                    let pattern = quote! { Self::#variant_ident { #(#bindings),* } };
                    let dispatch = if container.tag.is_none() {
                        let named_dispatch = named_variant_edit_dispatch(&fields, &quote! { __rest }, &quote! { edit }, value_crate);
                        quote! {
                            let (__segment, __rest) = path.split_first().ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "missing external variant path"))?;
                            if *__segment != #wire_variant {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("active variant `{}` has no child `{}`", #wire_variant, __segment)));
                            }
                            #named_dispatch
                        }
                    } else if let Some(content) = &container.content {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let named_dispatch = named_variant_edit_dispatch(&fields, &quote! { __rest }, &quote! { edit }, value_crate);
                        quote! {
                            let (__segment, __rest) = path.split_first().ok_or_else(|| #value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "missing adjacent enum path"))?;
                            if *__segment == #tag {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot edit an enum tag in place"));
                            }
                            if *__segment != #content {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, format!("missing object key `{}`", __segment)));
                            }
                            #named_dispatch
                        }
                    } else {
                        let tag = container.tag.as_ref().expect("tagged enum");
                        let named_dispatch = named_variant_edit_dispatch(&fields, &quote! { path }, &quote! { edit }, value_crate);
                        quote! {
                            if path.first().is_some_and(|segment| *segment == #tag) {
                                return Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot edit an enum tag in place"));
                            }
                            #named_dispatch
                        }
                    };
                    Ok(quote! { #pattern => { #dispatch } })
                }
                other => Err(syn::Error::new_spanned(other, "#[derive(FromValue)] enum variants must be unit, a single unnamed payload, or named fields")),
            }
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! {
        if path.is_empty() {
            return match edit {
                #value_crate::ValueEdit::Set(value) => {
                    let replacement = <Self as #value_crate::FromValue>::from_value(value)?;
                    *self = replacement;
                    Ok(())
                }
                #value_crate::ValueEdit::Insert(_) | #value_crate::ValueEdit::InsertAt { .. } => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot insert at the enum root")),
                #value_crate::ValueEdit::Remove => Err(#value_crate::ValueError::new(#value_crate::ValueRefusalKind::InvalidValue, "cannot remove the enum root")),
            };
        }
        match self { #(#arms),* }
    })
}
//#endregion 🧭️TypedPath
