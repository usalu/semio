//! 📹️ Record field codegen of the record derive: the field plans and the shared record and schema field fragments. See [`super`].

use super::*;

//#region 🔖️RecordCodegen
pub(super) struct FieldPlan {
    pub(super) skip: bool,
    pub(super) ident: syn::Ident,
    pub(super) id: u16,
    pub(super) key: String,
    pub(super) positional: Option<u16>,
    pub(super) optional: bool,
    pub(super) kind: FieldKind,
    pub(super) elem_ty: Type,
    pub(super) field_ty: Type,
    /// 🧱️ `#[dsl(block)]` on a field whose `FieldKind` doesn't already imply its own `{ }` wrapping
    /// (`VecBlockStatements` handles that itself) — wraps whatever shape that kind would otherwise
    /// produce in `Shape::Block`, e.g. a single nested `#[derive(DslRecord)]` field printed as a
    /// bare `camera { x=0 y=0 zoom=1 }` line instead of a `camera=...` attribute.
    pub(super) block: bool,
    /// 📏️ `#[dsl(unit = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    pub(super) unit: Option<String>,
    /// 📐️ `#[dsl(angle = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    pub(super) angle: Option<String>,
    /// 🔗️ `#[dsl(refs = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    pub(super) refs: Option<String>,
    /// 📚️ `#[dsl(defines = "...")]` — sets `FieldSpec.defines`, independent of `Shape`.
    pub(super) defines: Option<String>,
    /// 💎️ `#[dsl(lang = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    pub(super) lang: Option<String>,
    pub(super) lang_from: Option<String>,
    /// 🧐️ `#[dsl(coord)]`, only meaningful for `FieldKind::Scalar`/`OptionScalar` on an array type.
    pub(super) coord: bool,
    /// 📁️ `#[dsl(dir)]`, ditto.
    pub(super) dir: bool,
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
pub(super) fn plan_fields(fields: &Fields) -> Vec<FieldPlan> {
    let mut positional_counter: u16 = 0;
    let mut out = Vec::new();
    for (index, field) in fields.iter().enumerate() {
        let attrs = parse_field_attrs(&field.attrs);
        let ident = field.ident.clone().expect("dsl_derive only supports named fields");
        let (kind, elem_ty) = classify_field(&field.ty, &attrs);
        let key = attrs.key.clone().unwrap_or_else(|| to_kebab(&ident.to_string()));
        let optional = !attrs.required && matches!(kind, FieldKind::OptionScalar(_) | FieldKind::OptionStatements(_));
        let positional = if attrs.positional && !attrs.skip {
            let p = positional_counter;
            positional_counter += 1;
            Some(p)
        } else {
            None
        };
        let block = attrs.block && !matches!(kind, FieldKind::VecBlockStatements(_));
        out.push(FieldPlan {
            skip: attrs.skip,
            ident,
            id: index as u16,
            key,
            positional,
            optional,
            kind,
            elem_ty,
            field_ty: field.ty.clone(),
            block,
            unit: attrs.unit.clone(),
            angle: attrs.angle.clone(),
            refs: attrs.refs.clone(),
            defines: attrs.defines.clone(),
            lang: attrs.lang.clone(),
            lang_from: attrs.lang_from.clone(),
            coord: attrs.coord,
            dir: attrs.dir,
        });
    }
    out
}

/// 🏗️ Builds the shared generic record and variant field fragments.
/// bodies: the `RecordSpec` field-spec expressions, the struct→`RecordValue` conversion, and the
/// `RecordValue`→struct conversion.
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
pub(super) fn record_codegen(fields: &Fields) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<syn::Ident>) {
    let plans = plan_fields(fields);
    let mut spec_exprs = Vec::new();
    let mut to_value_stmts = Vec::new();
    let mut from_value_stmts = Vec::new();
    let mut field_idents = Vec::new();

    for plan in &plans {
        let FieldPlan { ident, id, key, positional, optional, kind, elem_ty, field_ty, block, unit, angle, refs, defines, lang, lang_from, coord, dir, .. } = plan;
        if plan.skip {
            field_idents.push(ident.clone());
            let local=field_local(ident);
            from_value_stmts.push(quote!{let #local=None;});
            continue;
        }
        let quantity_shape_override: Option<proc_macro2::TokenStream> = if let Some(symbol) = unit {
            Some(quote! { ::semio_framework_dsl_record::Shape::Quantity(::semio_framework_dsl_record::__rt::unit_for_derive(#symbol)) })
        } else if let Some(symbol) = angle {
            Some(quote! { ::semio_framework_dsl_record::Shape::Angle(::semio_framework_dsl_record::__rt::unit_for_derive(#symbol)) })
        } else if let Some(kind) = refs {
            Some(quote! { ::semio_framework_dsl_record::Shape::Ref(#kind) })
        } else if let Some(from) = lang_from {
            let embed_lang_key = plans.iter().find(|p| p.ident == from.as_str()).map_or_else(|| to_kebab(from), |p| p.key.clone());
            Some(quote! { ::semio_framework_dsl_record::Shape::EmbedFrom(#embed_lang_key) })
        } else if let Some(l) = lang {
            Some(quote! { ::semio_framework_dsl_record::Shape::Embed(#l) })
        } else if *coord {
            Some(quote! { ::semio_framework_dsl_record::Shape::Coord(3) })
        } else if *dir {
            Some(quote! { ::semio_framework_dsl_record::Shape::Dir })
        } else {
            None
        };
        let defines_expr = match defines {
            Some(kind) => quote! { .defines(#kind) },
            None => quote! {},
        };
        field_idents.push(ident.clone());
        let pos_expr = match positional {
            Some(p) => quote! { .positional(#p as u8) },
            None => quote! {},
        };
        let opt_expr = if *optional {
            quote! { .optional() }
        } else {
            quote! {}
        };

        let (shape_expr, to_value_expr, from_value_expr): (proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream) = match kind {
            FieldKind::Scalar => (
                quantity_shape_override.clone().unwrap_or_else(|| quote! { <#elem_ty as ::semio_framework_dsl_record::DslField>::shape() }),
                quote! { ::semio_framework_dsl_record::DslField::to_value(&self.#ident) },
                quote! { <#elem_ty as ::semio_framework_dsl_record::DslField>::from_value(value).map_err(|message|::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,message,::semio_framework_dsl_record::TextSpan::at(1,1)))? },
            ),
            FieldKind::Bytes64 => (
                quote! { ::semio_framework_dsl_record::Shape::Bytes64 },
                quote! { ::semio_framework_dsl_record::FieldValue::Bytes64(self.#ident.clone()) },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Bytes64(bytes) => bytes.clone(),
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Bytes64, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::OptionScalar(inner) => (
                quantity_shape_override.clone().unwrap_or_else(|| quote! { <#inner as ::semio_framework_dsl_record::DslField>::shape() }),
                quote! {
                    match &self.#ident {
                        Some(v) => ::semio_framework_dsl_record::DslField::to_value(v),
                        None => ::semio_framework_dsl_record::FieldValue::Absent,
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Absent => None,
                        other => Some(<#inner as ::semio_framework_dsl_record::DslField>::from_value(other).map_err(|message|::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,message,::semio_framework_dsl_record::TextSpan::at(1,1)))?),
                    }
                },
            ),
            FieldKind::VecList(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::List(Box::new(<#inner as ::semio_framework_dsl_record::DslField>::shape())) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::semio_framework_dsl_record::DslField::to_value(v)); }
                        ::semio_framework_dsl_record::FieldValue::List(__items)
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::List(items) => {
                            <#field_ty as ::semio_framework_dsl_record::DslSequence<#inner>>::from_decoded(items.iter().map(|v| <#inner as ::semio_framework_dsl_record::DslField>::from_value(v).map_err(|message|::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,message,::semio_framework_dsl_record::TextSpan::at(1,1)))))?
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected List, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::VecTable(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Table(<#inner>::__dsl_spec_producer()) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::semio_framework_dsl_record::DslField::to_value(v)); }
                        ::semio_framework_dsl_record::FieldValue::List(__items)
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::List(items) => {
                            <#field_ty as ::semio_framework_dsl_record::DslSequence<#inner>>::from_decoded(items.iter().map(|v| <#inner as ::semio_framework_dsl_record::DslField>::from_value(v).map_err(|message|::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,message,::semio_framework_dsl_record::TextSpan::at(1,1)))))?
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected List, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::VecTuple(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Tuple(Box::new(<#inner as ::semio_framework_dsl_record::DslField>::shape()), None) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::semio_framework_dsl_record::DslField::to_value(v)); }
                        ::semio_framework_dsl_record::FieldValue::Tuple(__items)
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Tuple(items) => {
                            <#field_ty as ::semio_framework_dsl_record::DslSequence<#inner>>::from_decoded(items.iter().map(|v| <#inner as ::semio_framework_dsl_record::DslField>::from_value(v).map_err(|message|::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,message,::semio_framework_dsl_record::TextSpan::at(1,1)))))?
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Tuple, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::VecStatements(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Statements(<#inner as ::semio_framework_dsl_record::DslVariants>::variants()) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::semio_framework_dsl_record::DslVariants::to_named_record(v)); }
                        ::semio_framework_dsl_record::FieldValue::Statements(__items)
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Statements(items) => {
                            <#field_ty as ::semio_framework_dsl_record::DslSequence<#inner>>::from_decoded(items.iter().map(|(keyword, record)| <#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record(keyword, record)))?
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Statements, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::VecBlockStatements(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Block(Box::new(::semio_framework_dsl_record::Shape::Statements(<#inner as ::semio_framework_dsl_record::DslVariants>::variants()))) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::semio_framework_dsl_record::DslVariants::to_named_record(v)); }
                        ::semio_framework_dsl_record::FieldValue::Block(Box::new(::semio_framework_dsl_record::FieldValue::Statements(__items)))
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Block(inner_value) => match inner_value.as_ref() {
                            ::semio_framework_dsl_record::FieldValue::Statements(items) => {
                                <#field_ty as ::semio_framework_dsl_record::DslSequence<#inner>>::from_decoded(items.iter().map(|(keyword, record)| <#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record(keyword, record)))?
                            }
                            other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Statements inside Block, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                        },
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Block, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::MapField(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Map(Box::new(<#inner as ::semio_framework_dsl_record::DslField>::shape())) },
                quote! {
                    {
                        let mut __entries = Vec::with_capacity(self.#ident.len());
                        for (k, v) in self.#ident.iter() { __entries.push((k.clone(), ::semio_framework_dsl_record::DslField::to_value(v))); }
                        ::semio_framework_dsl_record::FieldValue::Map(__entries)
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Map(entries) => {
                            let mut __out = ::std::collections::BTreeMap::new();
                            for (k, v) in entries.iter() { __out.insert(k.clone(), <#inner as ::semio_framework_dsl_record::DslField>::from_value(v).map_err(|message|::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,message,::semio_framework_dsl_record::TextSpan::at(1,1)))?); }
                            __out
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Map, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::OptionStatements(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Statements(<#inner as ::semio_framework_dsl_record::DslVariants>::variants()) },
                quote! {
                    ::semio_framework_dsl_record::FieldValue::Statements(match &self.#ident {
                        Some(v) => vec![::semio_framework_dsl_record::DslVariants::to_named_record(v)],
                        None => vec![],
                    })
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Absent => None,
                        ::semio_framework_dsl_record::FieldValue::Statements(items) if items.is_empty() => None,
                        ::semio_framework_dsl_record::FieldValue::Statements(items) if items.len() == 1 => {
                            Some(<#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record(&items[0].0, &items[0].1)?)
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected 0 or 1 tagged values, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::RequiredStatements(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Statements(<#inner as ::semio_framework_dsl_record::DslVariants>::variants()) },
                quote! { ::semio_framework_dsl_record::FieldValue::Statements(vec![::semio_framework_dsl_record::DslVariants::to_named_record(self.#ident.as_ref())]) },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Statements(items) if items.len() == 1 => {
                            Box::new(<#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record(&items[0].0, &items[0].1)?)
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected exactly 1 tagged value, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
            FieldKind::RequiredInlineStatements(inner) => (
                quote! { ::semio_framework_dsl_record::Shape::Statements(<#inner as ::semio_framework_dsl_record::DslVariants>::variants()) },
                quote! { ::semio_framework_dsl_record::FieldValue::Statements(vec![::semio_framework_dsl_record::DslVariants::to_named_record(&self.#ident)]) },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Statements(items) if items.len() == 1 => {
                            <#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record(&items[0].0, &items[0].1)?
                        }
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected exactly 1 tagged value, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            ),
        };

        let (shape_expr, to_value_expr, from_value_expr) = if *block {
            (
                quote! { ::semio_framework_dsl_record::Shape::Block(Box::new(#shape_expr)) },
                quote! {
                    match #to_value_expr {
                        ::semio_framework_dsl_record::FieldValue::Absent => ::semio_framework_dsl_record::FieldValue::Absent,
                        other => ::semio_framework_dsl_record::FieldValue::Block(Box::new(other)),
                    }
                },
                quote! {
                    match value {
                        ::semio_framework_dsl_record::FieldValue::Block(inner) => { let value = inner.as_ref(); #from_value_expr },
                        ::semio_framework_dsl_record::FieldValue::Absent => { let value = &::semio_framework_dsl_record::FieldValue::Absent; #from_value_expr },
                        other => return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("expected Block, found {other:?}"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                    }
                },
            )
        } else {
            (shape_expr, to_value_expr, from_value_expr)
        };

        spec_exprs.push(quote! {
            ::semio_framework_dsl_record::FieldSpec::new(#id, #key, #shape_expr) #pos_expr #opt_expr #defines_expr
        });
        to_value_stmts.push(quote! {
            record.fields.insert(#id, #to_value_expr);
        });
        let local = field_local(ident);
                let presence=if *optional{quote!{}}else{quote!{if matches!(value,::semio_framework_dsl_record::FieldValue::Absent){return Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,concat!("missing required field '",#key,"'"),::semio_framework_dsl_record::TextSpan::at(1,1)));}}};
from_value_stmts.push(quote! {
            let #local = {
                let value = record.get(#id).ok_or_else(|| ::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("missing field '{}'", #key),::semio_framework_dsl_record::TextSpan::at(1,1)))?;
                                #presence
#from_value_expr
            };
        });
    }

    (spec_exprs, to_value_stmts, from_value_stmts, field_idents)
}

/// 🏭️ Emits declared field metadata without borrowing an ordinary allocating shape factory.
pub(super) fn schema_record_codegen(fields:&Fields)->Vec<proc_macro2::TokenStream>{
    let plans=plan_fields(fields);
    plans.iter().filter(|plan|!plan.skip).map(|plan|{
        let FieldPlan{id,key,positional,optional,kind,elem_ty,block,unit,angle,refs,defines,lang,lang_from,coord,dir,..}=plan;
        let refinement=if let Some(symbol)=unit{Some(quote!{::semio_framework_dsl_record::Shape::Quantity(::semio_framework_dsl_record::__rt::unit_for_derive(#symbol))})}else if let Some(symbol)=angle{Some(quote!{::semio_framework_dsl_record::Shape::Angle(::semio_framework_dsl_record::__rt::unit_for_derive(#symbol))})}else if let Some(kind)=refs{Some(quote!{::semio_framework_dsl_record::Shape::Ref(#kind)})}else if let Some(from)=lang_from{let key=plans.iter().find(|plan|plan.ident==from.as_str()).map_or_else(||to_kebab(from),|plan|plan.key.clone());Some(quote!{::semio_framework_dsl_record::Shape::EmbedFrom(#key)})}else if let Some(language)=lang{Some(quote!{::semio_framework_dsl_record::Shape::Embed(#language)})}else if *coord{Some(quote!{::semio_framework_dsl_record::Shape::Coord(3)})}else if *dir{Some(quote!{::semio_framework_dsl_record::Shape::Dir})}else{None};
        let shape=match kind{
            FieldKind::Scalar=>refinement.unwrap_or_else(||quote!{<#elem_ty as ::semio_framework_dsl_record::DslField>::shape_controlled(control)?}),
            FieldKind::OptionScalar(inner)=>refinement.unwrap_or_else(||quote!{<#inner as ::semio_framework_dsl_record::DslField>::shape_controlled(control)?}),
            FieldKind::Bytes64=>quote!{::semio_framework_dsl_record::Shape::Bytes64},
            FieldKind::VecList(inner)=>quote!{::semio_framework_dsl_record::Shape::List(::semio_framework_dsl_record::producer::boxed(<#inner as ::semio_framework_dsl_record::DslField>::shape_controlled(control)?,control)?)},
            FieldKind::VecTuple(inner)=>quote!{::semio_framework_dsl_record::Shape::Tuple(::semio_framework_dsl_record::producer::boxed(<#inner as ::semio_framework_dsl_record::DslField>::shape_controlled(control)?,control)?,None)},
            FieldKind::VecTable(inner)=>quote!{::semio_framework_dsl_record::Shape::Table(<#inner>::__dsl_spec_producer())},
            FieldKind::MapField(inner)=>quote!{::semio_framework_dsl_record::Shape::Map(::semio_framework_dsl_record::producer::boxed(<#inner as ::semio_framework_dsl_record::DslField>::shape_controlled(control)?,control)?)},
            FieldKind::VecStatements(inner)|FieldKind::OptionStatements(inner)|FieldKind::RequiredStatements(inner)|FieldKind::RequiredInlineStatements(inner)=>quote!{::semio_framework_dsl_record::Shape::Statements(<#inner as ::semio_framework_dsl_record::DslVariants>::variants_controlled(control)?)},
            FieldKind::VecBlockStatements(inner)=>quote!{::semio_framework_dsl_record::Shape::Block(::semio_framework_dsl_record::producer::boxed(::semio_framework_dsl_record::Shape::Statements(<#inner as ::semio_framework_dsl_record::DslVariants>::variants_controlled(control)?),control)?)},
        };
        let shape=if *block{quote!{::semio_framework_dsl_record::Shape::Block(::semio_framework_dsl_record::producer::boxed(#shape,control)? )}}else{shape};
        let position=positional.map(|position|quote!{.positional(#position as u8)});let optional=if *optional{quote!{.optional()}}else{quote!{}};let defines=defines.as_ref().map(|kind|quote!{.defines(#kind)});
        quote!{let field=::semio_framework_dsl_record::producer::field(#id,#key,#shape,control)? #position #optional #defines;fields.push(field);control.step()?;}
    }).collect()
}
//#endregion 🔖️RecordCodegen
