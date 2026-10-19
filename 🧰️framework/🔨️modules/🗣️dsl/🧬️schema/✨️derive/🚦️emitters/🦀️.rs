//! 🚦️ Emitters of the record derive: the record, scalar and tagged variant expansions. See [`super`].

use super::*;

pub(super) fn emit_record(input: DeriveInput) -> proc_macro2::TokenStream {
    if let Some(error)=custody_field_error(&input){return error;}
    let name = input.ident.clone();
    let container = parse_container_attrs(&input);
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslRecord only supports structs").to_compile_error();
    };
    let (spec_exprs, to_value_stmts, from_value_stmts, field_idents) = record_codegen(&data.fields);
    let field_initializers = field_inits(&field_idents);
    let controlled_stmts=controlled_record_codegen(&data.fields);
    let schema_stmts=schema_record_codegen(&data.fields);let schema_count=schema_stmts.len();
    let encoding_stmts=encoding_record_codegen(&data.fields,false);
    let encoding_count=encoding_stmts.len();
    let retained_projection=retained_record_codegen(&data.fields,false);
    let retained_keys=retained_record_key_codegen(&data.fields,false);
    let controlled_fields=field_idents.iter().map(|ident|{let local=field_local(ident);quote!{#ident:#local.take()}});
    let retirement=record_retirement_codegen(&data.fields,container.retire_with.as_ref());
    let schema_keyword_expr=match &container.keyword{Some(keyword)=>quote!{Some(#keyword)},None=>quote!{None}};
    let keyword_expr = match &container.keyword {
        Some(k) => quote! { Some(#k.to_string()) },
        None => quote! { None },
    };
    let layout_expr = if container.lines_layout {
        quote! { ::semio_framework_dsl_record::RecordLayout::Lines }
    } else {
        quote! { ::semio_framework_dsl_record::RecordLayout::Inline }
    };

    let borrowed=borrowed_record_owner_codegen(&input,true);
    let expanded = quote! {
        #borrowed
        impl #name {
            pub fn __dsl_spec() -> ::semio_framework_dsl_record::RecordSpec {
                ::semio_framework_dsl_record::RecordSpec::new_owned(#keyword_expr, #layout_expr, vec![ #(#spec_exprs),* ])
            }
            /// 🏭️ Owns exactly the declared metadata fields under either native control.
            pub fn __dsl_spec_controlled<C: ::semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<::semio_framework_dsl_record::RecordSpec,::semio_framework_value::ValueError>{
                control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(#schema_count)?;let mut fields=control.allocate_vec(#schema_count)?;#(#schema_stmts)*::semio_framework_dsl_record::producer::record(#schema_keyword_expr,#layout_expr,fields,control)}))
            }
            /// 🪆️ Retains lazy owner factories without constructing their metadata.
            pub fn __dsl_spec_producer()->::semio_framework_dsl_record::RecordSpecProducer{::semio_framework_dsl_record::RecordSpecProducer{ordinary:Self::__dsl_spec,decoding:|control|Self::__dsl_spec_controlled(control),encoding:|control|Self::__dsl_spec_controlled(control)}}
            pub fn __dsl_to_record(&self) -> ::semio_framework_dsl_record::RecordValue {
                let mut record = ::semio_framework_dsl_record::RecordValue::default();
                #(#to_value_stmts)*
                record
            }
            /// 🛫️ Projects complete named fields under caller-owned output admission.
            pub fn __dsl_to_record_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<::semio_framework_dsl_record::RecordValue,::semio_framework_value::ValueError>{
                control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(#encoding_count)?;let mut record=::semio_framework_dsl_record::native_encoding::EncodedRecord::new(#encoding_count,control)?;#(#encoding_stmts)*Ok::<_,::semio_framework_value::ValueError>(record.take())}))
            }
            pub fn __dsl_from_record(record: &::semio_framework_dsl_record::RecordValue) -> Result<Self, ::semio_framework_dsl_record::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_initializers),* })
            }
            /// 🛬️ Binds owned typed fields with cumulative allocation and interior cancellation.
            pub fn __dsl_from_record_controlled(record:&::semio_framework_dsl_record::RecordValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{
                control.checkpoint()?;
                #(#controlled_stmts)*
                Ok(Self{#(#controlled_fields),*})
            }
        }

        impl ::semio_framework_dsl_record::DslField for #name {
            fn projection_view(&self,path:&[usize])->Result<::semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,::semio_framework_value::ValueError>{#retained_projection}
            fn projection_key(&self,path:&[usize],index:usize)->Result<&str,::semio_framework_value::ValueError>{#retained_keys}
            fn retire_decoded(self){#retirement}
            fn shape() -> ::semio_framework_dsl_record::Shape {
                ::semio_framework_dsl_record::Shape::Record(Self::__dsl_spec_producer())
            }
            fn shape_controlled<C: ::semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<::semio_framework_dsl_record::Shape,::semio_framework_value::ValueError>{control.checkpoint()?;Ok(::semio_framework_dsl_record::Shape::Record(Self::__dsl_spec_producer()))}
            fn to_value(&self) -> ::semio_framework_dsl_record::FieldValue {
                ::semio_framework_dsl_record::FieldValue::Record(self.__dsl_to_record())
            }
            fn from_value(value: &::semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
                match value {
                    ::semio_framework_dsl_record::FieldValue::Record(record) => Self::__dsl_from_record(record).map_err(|e| e.message),
                    other => Err(format!("expected Record, found {other:?}")),
                }
            }
            fn from_value_controlled(value:&::semio_framework_dsl_record::FieldValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{
                match value{::semio_framework_dsl_record::FieldValue::Record(record)=>Self::__dsl_from_record_controlled(record,control),_=>Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Record"))}
            }
            fn from_record_controlled(record:&::semio_framework_dsl_record::RecordValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{Self::__dsl_from_record_controlled(record,control)}
            fn to_value_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<::semio_framework_dsl_record::FieldValue,::semio_framework_value::ValueError>{Self::__dsl_to_record_controlled(self,control).map(::semio_framework_dsl_record::FieldValue::Record)}
            fn to_record_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<::semio_framework_dsl_record::RecordValue,::semio_framework_value::ValueError>{Self::__dsl_to_record_controlled(self,control)}
        }
    };
    expanded.into()
}

pub(super) fn emit_scalar(input: DeriveInput) -> proc_macro2::TokenStream {
    let name = input.ident.clone();
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslScalar only supports unit-variant enums").to_compile_error();
    };
    let mut variant_tags = Vec::new();
    let mut schema_tags=Vec::new();
    let mut match_to_ordinal = Vec::new();
    let mut match_from_ordinal = Vec::new();
    for (ordinal, variant) in data.variants.iter().enumerate() {
        if !matches!(variant.fields, Fields::Unit) {
            return syn::Error::new_spanned(variant, "DslScalar only supports unit variants").to_compile_error();
        }
        let attrs = parse_field_attrs(&variant.attrs);
        let variant_ident = variant.ident.clone();
        let tag = attrs.key.unwrap_or_else(|| to_kebab(&variant_ident.to_string()));
        let ordinal = ordinal as u32;
        variant_tags.push(quote! { (#tag.to_string(), #ordinal) });
        schema_tags.push(quote!{variants.push((control.copy_text(#tag)?,#ordinal));control.step()?;});
        match_to_ordinal.push(quote! { #name::#variant_ident => #ordinal });
        match_from_ordinal.push(quote! { #ordinal => Ok(#name::#variant_ident) });
    }

    let schema_count=schema_tags.len();
    let receiver=borrowed_enum_receiver(data);
    let borrowed=borrowed_scalar_owner_codegen(&input);
    let expanded = quote! {
        #borrowed
        impl ::semio_framework_dsl_record::DslField for #name {
            fn shape() -> ::semio_framework_dsl_record::Shape {
                ::semio_framework_dsl_record::Shape::Enum(vec![ #(#variant_tags),* ])
            }
            fn shape_controlled<C: ::semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<::semio_framework_dsl_record::Shape,::semio_framework_value::ValueError>{control.scoped_stage(|control|{control.begin_stage(#schema_count)?;let mut variants=control.allocate_vec(#schema_count)?;#(#schema_tags)*Ok(::semio_framework_dsl_record::Shape::Enum(variants))})}
            fn to_value(&self) -> ::semio_framework_dsl_record::FieldValue {
                ::semio_framework_dsl_record::FieldValue::Enum(match #receiver { #(#match_to_ordinal),* })
            }
            fn projection_view(&self,path:&[usize])->Result<::semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,::semio_framework_value::ValueError>{if !path.is_empty(){return Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Enum(match #receiver{#(#match_to_ordinal),*}))}
            fn from_value(value: &::semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
                match value {
                    ::semio_framework_dsl_record::FieldValue::Enum(ordinal) => match *ordinal {
                        #(#match_from_ordinal,)*
                        other => Err(format!("unknown enum ordinal {other}")),
                    },
                    other => Err(format!("expected Enum, found {other:?}")),
                }
            }
            fn from_value_controlled(value:&::semio_framework_dsl_record::FieldValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{control.step()?;match value{::semio_framework_dsl_record::FieldValue::Enum(ordinal)=>match *ordinal{#(#match_from_ordinal,)*other=>Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown enum ordinal {other}")))},_=>Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected declared Enum"))}}
            fn to_value_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<::semio_framework_dsl_record::FieldValue,::semio_framework_value::ValueError>{control.step()?;Ok(::semio_framework_dsl_record::FieldValue::Enum(match #receiver{#(#match_to_ordinal),*}))}
        }
    };
    expanded.into()
}

/// 🕳️ Borrows inhabited variants and dereferences only an impossible empty enum value.
pub(super) fn borrowed_enum_receiver(data: &syn::DataEnum) -> proc_macro2::TokenStream {
    if data.variants.is_empty() { quote!(*self) } else { quote!(self) }
}

fn dsl_variants_codegen(name: &syn::Ident, data: &syn::DataEnum, retire_with:Option<&syn::Path>) -> proc_macro2::TokenStream {
    let receiver=borrowed_enum_receiver(data);
    let mut variants_exprs = Vec::new();
    let mut schema_variant_stmts=Vec::new();
    let mut schema_methods=Vec::new();
    let mut to_named_arms = Vec::new();
    let mut encoding_arms=Vec::new();
    let mut from_named_arms = Vec::new();
    let mut controlled_arms=Vec::new();
    let mut retirement_arms=Vec::new();
    let mut variant_identity_arms=Vec::new();let mut variant_projection_arms=Vec::new();let mut variant_key_arms=Vec::new();

    for (variant_index,variant) in data.variants.iter().enumerate() {
        let attrs = parse_field_attrs(&variant.attrs);
        let variant_ident = variant.ident.clone();
        let keyword = attrs.key.clone().unwrap_or_else(|| to_kebab(&variant_ident.to_string()));
        let fields = &variant.fields;

        if let Fields::Unnamed(unnamed) = fields {
            if unnamed.unnamed.len() == 1 {
                let inner_ty = &unnamed.unnamed[0].ty;
                variants_exprs.push(quote!{(#keyword.to_string(),::semio_framework_dsl_record::__rt::newtype_variant_producer::<#inner_ty>())});
                schema_variant_stmts.push(quote!{variants.push((control.copy_text(#keyword)?,::semio_framework_dsl_record::__rt::newtype_variant_producer::<#inner_ty>()));control.step()?;});
                to_named_arms.push(quote! {
                    #name::#variant_ident(inner) => (#keyword.to_string(), ::semio_framework_dsl_record::__rt::newtype_variant_to_record(inner))
                });
                from_named_arms.push(quote! {
                    #keyword => Ok(#name::#variant_ident(::semio_framework_dsl_record::__rt::newtype_variant_from_record::<#inner_ty>(record)?))
                });
                encoding_arms.push(quote!{#name::#variant_ident(inner)=>{let keyword=control.copy_text(#keyword)?;let record=<#inner_ty as ::semio_framework_dsl_record::DslField>::to_record_controlled(inner,control)?;Ok((keyword,record))}});
                controlled_arms.push(quote!{#keyword=>Ok(#name::#variant_ident(control.scoped_stage(|control|<#inner_ty as ::semio_framework_dsl_record::DslField>::from_record_controlled(record,control))?))});
                retirement_arms.push(quote!{#name::#variant_ident(inner)=><#inner_ty as ::semio_framework_dsl_record::DslField>::retire_decoded(inner)});
                variant_identity_arms.push(quote!{#name::#variant_ident(_)=>(#keyword,#variant_index,::semio_framework_dsl_record::__rt::newtype_variant_producer::<#inner_ty>())});
                variant_projection_arms.push(quote!{#name::#variant_ident(inner)=>{if !matches!(<#inner_ty as ::semio_framework_dsl_record::DslField>::projection_view(inner,&[])?,::semio_framework_dsl_record::native_encoding::FieldProjectionView::Record(_)){return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"newtype variant requires an original Record source"));}<#inner_ty as ::semio_framework_dsl_record::DslField>::projection_view(inner,path)}});
                variant_key_arms.push(quote!{#name::#variant_ident(inner)=>{if !matches!(<#inner_ty as ::semio_framework_dsl_record::DslField>::projection_view(inner,&[])?,::semio_framework_dsl_record::native_encoding::FieldProjectionView::Record(_)){return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"newtype variant requires an original Record source"));}<#inner_ty as ::semio_framework_dsl_record::DslField>::projection_key(inner,path,index)}});

                continue;
            }
        }

        let (spec_exprs, _to_value_stmts, from_value_stmts, field_idents) = record_codegen(fields);
        let controlled_stmts=controlled_record_codegen(fields);
        let controlled_fields=field_idents.iter().map(|ident|{let local=field_local(ident);quote!{#ident:#local.take()}});

        let schema_name=quote::format_ident!("__dsl_variant_spec_{}",variant_index);let controlled_name=quote::format_ident!("__dsl_variant_spec_{}_controlled",variant_index);let producer_name=quote::format_ident!("__dsl_variant_spec_{}_producer",variant_index);
        let schema_stmts=schema_record_codegen(fields);let schema_count=schema_stmts.len();
        schema_methods.push(quote!{
            fn #schema_name()->::semio_framework_dsl_record::RecordSpec{::semio_framework_dsl_record::RecordSpec::new_owned(Some(#keyword.to_string()),::semio_framework_dsl_record::RecordLayout::Inline,vec![#(#spec_exprs),*])}
            fn #controlled_name<C: ::semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<::semio_framework_dsl_record::RecordSpec,::semio_framework_value::ValueError>{control.scoped_depth(64,|control|control.scoped_stage(|control|{control.begin_stage(#schema_count)?;let mut fields=control.allocate_vec(#schema_count)?;#(#schema_stmts)*::semio_framework_dsl_record::producer::record(Some(#keyword),::semio_framework_dsl_record::RecordLayout::Inline,fields,control)}))}
            fn #producer_name()->::semio_framework_dsl_record::RecordSpecProducer{::semio_framework_dsl_record::RecordSpecProducer{ordinary:Self::#schema_name,decoding:|control|Self::#controlled_name(control),encoding:|control|Self::#controlled_name(control)}}
        });
        variants_exprs.push(quote!{(#keyword.to_string(),Self::#producer_name())});
        schema_variant_stmts.push(quote!{variants.push((control.copy_text(#keyword)?,Self::#producer_name()));control.step()?;});

        let field_binds: Vec<proc_macro2::TokenStream> = field_idents.iter().map(|f| { let local = field_local(f); quote! { #f: #local } }).collect();
        let to_value_stmts_for_variant: Vec<proc_macro2::TokenStream> = record_codegen_to_value_from_bindings(fields);
        let is_unit = matches!(fields, Fields::Unit);
        let match_pattern = if is_unit {
            quote! { #name::#variant_ident }
        } else {
            quote! { #name::#variant_ident { #(#field_binds),* } }
        };
        let construct_expr = if is_unit {
            quote! { #name::#variant_ident }
        } else {
            let inits = field_inits(&field_idents);
            quote! { #name::#variant_ident { #(#inits),* } }
        };
        to_named_arms.push(quote! {
            #match_pattern => {
                let mut record = ::semio_framework_dsl_record::RecordValue::default();
                #(#to_value_stmts_for_variant)*
                (#keyword.to_string(), record)
            }
        });
        from_named_arms.push(quote! {
            #keyword => {
                #(#from_value_stmts)*
                Ok(#construct_expr)
            }
        });
        let encoding_stmts=encoding_record_codegen(fields,true);let encoding_count=encoding_stmts.len();
        encoding_arms.push(quote!{#match_pattern=>{control.begin_stage(#encoding_count)?;let mut record=::semio_framework_dsl_record::native_encoding::EncodedRecord::new(#encoding_count,control)?;#(#encoding_stmts)*let keyword=control.copy_text(#keyword)?;Ok((keyword,record.take()))}});
        let controlled_construct=if is_unit{quote!{#name::#variant_ident}}else{quote!{#name::#variant_ident{#(#controlled_fields),*}}};
        controlled_arms.push(quote!{#keyword=>{#(#controlled_stmts)* Ok(#controlled_construct)}});
        let plans=plan_fields(fields);let retirements=plans.iter().map(|plan|{let ident=field_local(&plan.ident);retire_field_codegen(plan,quote!{#ident})});
        retirement_arms.push(quote!{#match_pattern=>{#(#retirements)*}});
        let projected=retained_record_codegen(fields,true);let projected_key=retained_record_key_codegen(fields,true);
        variant_identity_arms.push(quote!{#match_pattern=>(#keyword,#variant_index,Self::#producer_name())});
        variant_projection_arms.push(quote!{#match_pattern=>{#projected}});
        variant_key_arms.push(quote!{#match_pattern=>{#projected_key}});

    }

    let retirement=if let Some(owner)=retire_with{quote!{#owner(self);}}else{quote!{match self{#(#retirement_arms),*}}};

    let schema_variant_count=schema_variant_stmts.len();
    quote! {
        impl #name{#(#schema_methods)*}
        impl ::semio_framework_dsl_record::DslVariants for #name {
            fn retire_decoded_variant(self){#retirement}
            fn projected_variant_identity(&self)->(&'static str,usize,::semio_framework_dsl_record::RecordSpecProducer){match #receiver{#(#variant_identity_arms),*}}
            fn projected_variant_view(&self,path:&[usize])->Result<::semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,::semio_framework_value::ValueError>{match #receiver{#(#variant_projection_arms),*}}
            fn projected_variant_key(&self,path:&[usize],index:usize)->Result<&str,::semio_framework_value::ValueError>{match #receiver{#(#variant_key_arms),*}}
            fn variants() -> Vec<(String, ::semio_framework_dsl_record::RecordSpecProducer)> {
                vec![ #(#variants_exprs),* ]
            }
            fn variants_controlled<C: ::semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<Vec<(String,::semio_framework_dsl_record::RecordSpecProducer)>,::semio_framework_value::ValueError>{control.scoped_stage(|control|{control.begin_stage(#schema_variant_count)?;let mut variants=control.allocate_vec(#schema_variant_count)?;#(#schema_variant_stmts)*Ok(variants)})}
            fn to_named_record(&self) -> (String, ::semio_framework_dsl_record::RecordValue) {
                match #receiver { #(#to_named_arms),* }
            }
            fn to_named_record_controlled(&self,control:&mut ::semio_framework_value::NativeEncodeControl<'_>)->Result<(String,::semio_framework_dsl_record::RecordValue),::semio_framework_value::ValueError>{
                control.scoped_depth(64,|control|control.scoped_stage(|control|match #receiver{#(#encoding_arms),*}))
            }
            fn from_named_record(keyword: &str, record: &::semio_framework_dsl_record::RecordValue) -> Result<Self, ::semio_framework_dsl_record::TextError> {
                match keyword {
                    #(#from_named_arms,)*
                    other => Err(::semio_framework_dsl_record::TextError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown keyword '{other}'"),::semio_framework_dsl_record::TextSpan::at(1,1))),
                }
            }
            fn from_named_record_controlled(keyword:&str,record:&::semio_framework_dsl_record::RecordValue,control:&mut ::semio_framework_value::NativeDecodeControl<'_>)->Result<Self,::semio_framework_value::ValueError>{
                control.step()?;
                match keyword{#(#controlled_arms,)*_=>Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"unknown declared keyword"))}
            }
        }
    }
}

pub(super) fn emit_enum(input: DeriveInput) -> proc_macro2::TokenStream {
    if let Some(error)=custody_field_error(&input){return error;}
    let name = input.ident.clone();
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslEnum only supports enums").to_compile_error();
    };
    let container=parse_container_attrs(&input);
    let ordinary=dsl_variants_codegen(&name,data,container.retire_with.as_ref());
    let borrowed=borrowed_variants_owner_codegen(&name,data);
    let field = if container.variant_field { variant_field_codegen(&name) } else { quote!{} };
    quote!{#ordinary #borrowed #field}
}


fn variant_field_codegen(name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl ::semio_framework_dsl_record::BorrowedDslField for #name {
            const SHAPE: ::semio_framework_dsl_record::BorrowedShape = ::semio_framework_dsl_record::BorrowedShape::Statements(<Self as ::semio_framework_dsl_record::BorrowedDslVariants>::VARIANTS);
        }
        impl ::semio_framework_dsl_record::DslField for #name {
            fn shape() -> ::semio_framework_dsl_record::Shape { ::semio_framework_dsl_record::Shape::Statements(<Self as ::semio_framework_dsl_record::DslVariants>::variants()) }
            fn shape_controlled<C: ::semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<::semio_framework_dsl_record::Shape, ::semio_framework_value::ValueError> { <Self as ::semio_framework_dsl_record::DslVariants>::variants_controlled(control).map(::semio_framework_dsl_record::Shape::Statements) }
            fn to_value(&self) -> ::semio_framework_dsl_record::FieldValue { ::semio_framework_dsl_record::FieldValue::Statements(vec![<Self as ::semio_framework_dsl_record::DslVariants>::to_named_record(self)]) }
            fn from_value(value: &::semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
                match value {
                    ::semio_framework_dsl_record::FieldValue::Statements(items) if items.len() == 1 => <Self as ::semio_framework_dsl_record::DslVariants>::from_named_record(&items[0].0, &items[0].1).map_err(|error| error.message),
                    _ => Err("declared variant field requires exactly one named record".into()),
                }
            }
            fn projection_view(&self, path: &[usize]) -> Result<::semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>, ::semio_framework_value::ValueError> {
                match path.split_first() {
                    None => Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Statements(1)),
                    Some((0, rest)) => <Self as ::semio_framework_dsl_record::DslVariants>::projected_variant_view(self, rest),
                    _ => Err(::semio_framework_dsl_record::native_encoding::projection_path_error()),
                }
            }
            fn projection_key(&self, path: &[usize], index: usize) -> Result<&str, ::semio_framework_value::ValueError> {
                match path.split_first() {
                    None if index == 0 => Ok(<Self as ::semio_framework_dsl_record::DslVariants>::projected_variant_identity(self).0),
                    Some((0, rest)) => <Self as ::semio_framework_dsl_record::DslVariants>::projected_variant_key(self, rest, index),
                    _ => Err(::semio_framework_dsl_record::native_encoding::projection_path_error()),
                }
            }
            fn to_value_controlled(&self, control: &mut ::semio_framework_value::NativeEncodeControl<'_>) -> Result<::semio_framework_dsl_record::FieldValue, ::semio_framework_value::ValueError> { ::semio_framework_dsl_record::native_encoding::project_statements::<Self, [Self]>(::std::slice::from_ref(self), control) }
            fn from_value_controlled(value: &::semio_framework_dsl_record::FieldValue, control: &mut ::semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ::semio_framework_value::ValueError> {
                control.checkpoint()?;
                match value {
                    ::semio_framework_dsl_record::FieldValue::Statements(items) if items.len() == 1 => control.scoped_stage(|control| {
                        control.begin_stage(1)?;
                        let decoded = ::semio_framework_value::DecodedValue::new(control.scoped_stage(|control| {
                            control.begin_stage(0)?;
                            <Self as ::semio_framework_dsl_record::DslVariants>::from_named_record_controlled(&items[0].0, &items[0].1, control)
                        })?, <Self as ::semio_framework_dsl_record::DslVariants>::retire_decoded_variant);
                        control.step()?;
                        Ok(decoded.take())
                    }),
                    _ => Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvalidValue, "declared variant field requires exactly one named record")),
                }
            }
            fn retire_decoded(self) { <Self as ::semio_framework_dsl_record::DslVariants>::retire_decoded_variant(self); }
        }
    }
}

