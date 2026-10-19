//! 🧹️ Lifecycle codegen of the record derive: the controlled, encoding, retirement and retained projection fragments of a field plan. See [`super`].

use super::*;

/// 🛬️ Generates explicit controlled bindings from the same authored field plans.
pub(super) fn controlled_record_codegen(fields:&Fields)->Vec<proc_macro2::TokenStream>{
    plan_fields(fields).iter().map(|plan|{
        let FieldPlan{ident,id,key,optional,kind,elem_ty,field_ty,block,..}=plan;
        if plan.skip {let local=field_local(ident);let retire=retire_field_codegen(plan,quote!{value});return quote!{let #local=::semio_framework_dsl_record::__rt::DecodedFieldOwner::new(None::<#elem_ty>,|value|{#retire});}}
        let expression=match kind {
            FieldKind::Scalar=>quote!{control.scoped_stage(|control|<#elem_ty as ::semio_framework_dsl_record::DslField>::from_value_controlled(value,control))?},
            FieldKind::Bytes64=>quote!{match value{::semio_framework_dsl_record::FieldValue::Bytes64(bytes)=>{control.step()?;control.copy_bytes(bytes)?},_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Bytes64"))}},
            FieldKind::OptionScalar(inner)=>quote!{match value{::semio_framework_dsl_record::FieldValue::Absent=>{control.step()?;None},other=>Some(control.scoped_stage(|control|<#inner as ::semio_framework_dsl_record::DslField>::from_value_controlled(other,control))?)}},
            FieldKind::VecList(inner)|FieldKind::VecTable(inner)=>quote!{match value{::semio_framework_dsl_record::FieldValue::List(items)=>::semio_framework_dsl_record::__rt::decode_list_controlled::<#inner,#field_ty>(items,control)?,_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected List"))}},
            FieldKind::VecTuple(inner)=>quote!{match value{::semio_framework_dsl_record::FieldValue::Tuple(items)=>::semio_framework_dsl_record::__rt::decode_list_controlled::<#inner,#field_ty>(items,control)?,_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Tuple"))}},
            FieldKind::MapField(inner)=>quote!{control.scoped_stage(|control|<::std::collections::BTreeMap<String,#inner> as ::semio_framework_dsl_record::DslField>::from_value_controlled(value,control))?},
            FieldKind::VecStatements(inner)=>controlled_statements(inner,field_ty),
            FieldKind::VecBlockStatements(inner)=>{let expression=controlled_statements(inner,field_ty);quote!{match value{::semio_framework_dsl_record::FieldValue::Block(inner)=>{let value=inner.as_ref();#expression},_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Block"))}}},
            FieldKind::OptionStatements(inner)=>quote!{match value{::semio_framework_dsl_record::FieldValue::Absent=>None,::semio_framework_dsl_record::FieldValue::Statements(items) if items.is_empty()=>None,::semio_framework_dsl_record::FieldValue::Statements(items) if items.len()==1=>Some(control.scoped_stage(|control|<#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record_controlled(&items[0].0,&items[0].1,control))?),_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected0or1 tagged values"))}},
            FieldKind::RequiredStatements(inner)=>quote!{match value{::semio_framework_dsl_record::FieldValue::Statements(items) if items.len()==1=>{control.charge(::std::mem::size_of::<#inner>())?;Box::new(control.scoped_stage(|control|<#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record_controlled(&items[0].0,&items[0].1,control))?)},_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected exactly1 tagged value"))}},
            FieldKind::RequiredInlineStatements(inner)=>quote!{match value{::semio_framework_dsl_record::FieldValue::Statements(items) if items.len()==1=>{control.scoped_stage(|control|<#inner as ::semio_framework_dsl_record::DslVariants>::from_named_record_controlled(&items[0].0,&items[0].1,control))?},_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected exactly1 tagged value"))}},
        };
        let expression=if *block {quote!{match value{::semio_framework_dsl_record::FieldValue::Block(inner)=>{let value=inner.as_ref();#expression},::semio_framework_dsl_record::FieldValue::Absent=>{let value=&::semio_framework_dsl_record::FieldValue::Absent;#expression},_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Block"))}}}else{expression};
        let retire=retire_field_codegen(plan,quote!{value});
        let ident=field_local(ident);
        let presence=if *optional{quote!{}}else{quote!{if matches!(value,::semio_framework_dsl_record::FieldValue::Absent){return Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvalidValue,concat!("missing required field '",#key,"'")));}}};
        quote!{let #ident=::semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.scoped_stage(|control|{let value=record.get(#id).ok_or_else(||::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,format!("missing field '{}'",#key)))?;#presence Ok::<_,::semio_framework_value::ValueError>(#expression)}).map_err(|error|error.under(#key))?,|value|{#retire});}
    }).collect()
}

/// 🛫️ Projects explicit declared fields with cumulative ownership and known workloads.
pub(super) fn encoding_record_codegen(fields:&Fields,bindings:bool)->Vec<proc_macro2::TokenStream>{
    plan_fields(fields).iter().filter(|plan|!plan.skip).map(|plan|{
        let FieldPlan{ident,id,key,kind,elem_ty,block,..}=plan;
        let source=if bindings{let local=field_local(ident);quote!{#local}}else{quote!{&self.#ident}};
        let projection=match kind{
            FieldKind::Scalar=>quote!{<#elem_ty as ::semio_framework_dsl_record::DslField>::to_value_controlled(value,control)?},
            FieldKind::Bytes64=>quote!{::semio_framework_dsl_record::FieldValue::Bytes64(control.copy_bytes(value)?)},
            FieldKind::OptionScalar(inner)=>quote!{match value{Some(value)=><#inner as ::semio_framework_dsl_record::DslField>::to_value_controlled(value,control)?,None=>::semio_framework_dsl_record::FieldValue::Absent}},
            FieldKind::VecList(_)|FieldKind::VecTable(_)=>quote!{::semio_framework_dsl_record::FieldValue::List(::semio_framework_dsl_record::native_encoding::project_list(value,control)?)},
            FieldKind::VecTuple(_)=>quote!{::semio_framework_dsl_record::FieldValue::Tuple(::semio_framework_dsl_record::native_encoding::project_list(value,control)?)},
            FieldKind::MapField(_)=>quote!{::semio_framework_dsl_record::native_encoding::project_map(value,control)?},
            FieldKind::VecStatements(_)=>quote!{::semio_framework_dsl_record::native_encoding::project_statements(value,control)?},
            FieldKind::VecBlockStatements(_)=>quote!{{control.charge(::std::mem::size_of::<::semio_framework_dsl_record::FieldValue>())?;::semio_framework_dsl_record::FieldValue::Block(Box::new(::semio_framework_dsl_record::native_encoding::project_statements(value,control)?))}},
            FieldKind::OptionStatements(_)=>quote!{match value{Some(value)=>::semio_framework_dsl_record::native_encoding::project_statements(::std::slice::from_ref(value),control)?,None=>::semio_framework_dsl_record::FieldValue::Statements(Vec::new())}},
            FieldKind::RequiredStatements(_)=>quote!{::semio_framework_dsl_record::native_encoding::project_statements(::std::slice::from_ref(value.as_ref()),control)?},
            FieldKind::RequiredInlineStatements(_)=>quote!{::semio_framework_dsl_record::native_encoding::project_statements(::std::slice::from_ref(value),control)?},
        };
        let projection=if *block{quote!{match #projection{::semio_framework_dsl_record::FieldValue::Absent=>::semio_framework_dsl_record::FieldValue::Absent,value=>{let value=::semio_framework_dsl_record::__rt::DecodedFieldOwner::new(value,::semio_framework_dsl_record::native_encoding::retire_field);control.charge(::std::mem::size_of::<::semio_framework_dsl_record::FieldValue>())?;::semio_framework_dsl_record::FieldValue::Block(Box::new(value.take()))}}}}else{projection};
        quote!{let field=control.scoped_stage(|control|{control.begin_stage(0)?;let value=#source;Ok::<_,::semio_framework_value::ValueError>(#projection)}).map_err(|error|error.under(#key))?;record.insert(#id,field)?;control.step()?;}
    }).collect()
}

/// 🧹️ Delegates each completed field to its declared field or variant retirement.
pub(super) fn retire_field_codegen(plan:&FieldPlan,value:proc_macro2::TokenStream)->proc_macro2::TokenStream{
    let FieldPlan{kind,elem_ty,..}=plan;
    match kind {
        FieldKind::Scalar=>quote!{<#elem_ty as ::semio_framework_dsl_record::DslField>::retire_decoded(#value);},
        FieldKind::Bytes64=>quote!{drop(#value);},
        FieldKind::OptionScalar(inner)=>quote!{if let Some(value)=#value{<#inner as ::semio_framework_dsl_record::DslField>::retire_decoded(value);}},
        FieldKind::VecList(inner)|FieldKind::VecTable(inner)|FieldKind::VecTuple(inner)=>quote!{for value in #value{<#inner as ::semio_framework_dsl_record::DslField>::retire_decoded(value);}},
        FieldKind::MapField(inner)=>quote!{<::std::collections::BTreeMap<String,#inner> as ::semio_framework_dsl_record::DslField>::retire_decoded(#value);},
        FieldKind::VecStatements(inner)|FieldKind::VecBlockStatements(inner)=>quote!{for value in #value{<#inner as ::semio_framework_dsl_record::DslVariants>::retire_decoded_variant(value);}},
        FieldKind::OptionStatements(inner)=>quote!{if let Some(value)=#value{<#inner as ::semio_framework_dsl_record::DslVariants>::retire_decoded_variant(value);}},
        FieldKind::RequiredStatements(inner)=>quote!{<#inner as ::semio_framework_dsl_record::DslVariants>::retire_decoded_variant(*#value);},
        FieldKind::RequiredInlineStatements(inner)=>quote!{<#inner as ::semio_framework_dsl_record::DslVariants>::retire_decoded_variant(#value);},
    }
}

/// 🌲️ Builds record retirement from explicit field plans or an authored domain lifecycle.
pub(super) fn record_retirement_codegen(fields:&Fields,retire_with:Option<&syn::Path>)->proc_macro2::TokenStream{
    if let Some(owner)=retire_with{return quote!{#owner(self);};}
    let plans=plan_fields(fields);let idents=field_inits(&plans.iter().map(|plan|plan.ident.clone()).collect::<Vec<_>>());
    let retirements=plans.iter().map(|plan|{let ident=field_local(&plan.ident);retire_field_codegen(plan,quote!{#ident})});
    quote!{let Self{#(#idents),*}=self;#(#retirements)*}
}

/// 🌿️ Builds a controlled tagged-list binding without borrowing an unchecked constructor.
fn controlled_statements(inner:&Type,field_ty:&Type)->proc_macro2::TokenStream{
    quote!{match value{::semio_framework_dsl_record::FieldValue::Statements(items)=>::semio_framework_dsl_record::__rt::decode_statements_controlled::<#inner,#field_ty>(items,control)?,_=>return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected Statements"))}}
}

/// 🔎️ Reads declared source slots directly for the retained native projection cursor.
pub(super) fn retained_record_codegen(fields:&Fields,from_bindings:bool)->proc_macro2::TokenStream{
    let plans=plan_fields(fields);let plans=plans.iter().filter(|plan|!plan.skip).collect::<Vec<_>>();let ids=plans.iter().map(|plan|plan.id);
    let arms=plans.iter().enumerate().map(|(index,plan)|{
        let FieldPlan{ident,kind,elem_ty,block,..}=plan;
        let value=if from_bindings{let local=field_local(ident);quote!{#local}}else{quote!{&self.#ident}};
        let projection=match kind{
            FieldKind::Scalar=>quote!{<#elem_ty as ::semio_framework_dsl_record::DslField>::projection_view(value,path)},
            FieldKind::OptionScalar(inner)=>quote!{match value{Some(value)=><#inner as ::semio_framework_dsl_record::DslField>::projection_view(value,path),None if path.is_empty()=>Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Absent),None=>Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::VecList(inner)|FieldKind::VecTable(inner)=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::List(value.len()))}else{<#inner as ::semio_framework_dsl_record::DslField>::projection_view(value.get(path[0]).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..])}},
            FieldKind::VecTuple(inner)=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Tuple(value.len()))}else{<#inner as ::semio_framework_dsl_record::DslField>::projection_view(value.get(path[0]).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..])}},
            FieldKind::Bytes64=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Bytes(value))}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::MapField(inner)=>quote!{<::std::collections::BTreeMap<String,#inner> as ::semio_framework_dsl_record::DslField>::projection_view(value,path)},
            FieldKind::VecStatements(inner)=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Statements(value.len()))}else{<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_view(value.get(path[0]).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..])}},
            FieldKind::VecBlockStatements(inner)=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Block)}else if path[0]==0{let path=&path[1..];if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Statements(value.len()))}else{<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_view(value.get(path[0]).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..])}}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::OptionStatements(inner)=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Statements(usize::from(value.is_some())))}else if path[0]==0{<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_view(value.as_ref().ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..])}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::RequiredStatements(inner)=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Statements(1))}else if path[0]==0{<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_view(value.as_ref(),&path[1..])}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::RequiredInlineStatements(inner)=>quote!{if path.is_empty(){Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Statements(1))}else if path[0]==0{<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_view(value,&path[1..])}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            _=>quote!{Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::UnsupportedOwner,"tagged source owner has no retained projection"))},
        };
        let projection=if *block{quote!{if path.is_empty(){match {#projection}?{::semio_framework_dsl_record::native_encoding::FieldProjectionView::Absent=>Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Absent),_=>Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Block)}}else if path[0]==0{let path=&path[1..];#projection}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}}}else{projection};
        quote!{#index=>{let value=#value;let path=&path[1..];#projection}}
    });
    quote!{if path.is_empty(){return Ok(::semio_framework_dsl_record::native_encoding::FieldProjectionView::Record(&[#(#ids),*]))}match path[0]{#(#arms,)*_=>Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}}
}

pub(super) fn retained_record_key_codegen(fields:&Fields,from_bindings:bool)->proc_macro2::TokenStream{
    let plans=plan_fields(fields);let arms=plans.iter().filter(|plan|!plan.skip).enumerate().map(|(ordinal,plan)|{
        let FieldPlan{ident,kind,elem_ty,block,..}=plan;
        let value=if from_bindings{let local=field_local(ident);quote!{#local}}else{quote!{&self.#ident}};
        let projection=match kind{
            FieldKind::Scalar=>quote!{<#elem_ty as ::semio_framework_dsl_record::DslField>::projection_key(value,path,index)},
            FieldKind::OptionScalar(inner)=>quote!{match value{Some(value)=><#inner as ::semio_framework_dsl_record::DslField>::projection_key(value,path,index),None=>Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::VecList(inner)|FieldKind::VecTable(inner)|FieldKind::VecTuple(inner)=>quote!{let child=*path.first().ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?;<#inner as ::semio_framework_dsl_record::DslField>::projection_key(value.get(child).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..],index)},
            FieldKind::MapField(inner)=>quote!{<::std::collections::BTreeMap<String,#inner> as ::semio_framework_dsl_record::DslField>::projection_key(value,path,index)},
            FieldKind::VecStatements(inner)=>quote!{if path.is_empty(){Ok(<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_identity(value.get(index).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?).0)}else{<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_key(value.get(path[0]).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..],index)}},
            FieldKind::VecBlockStatements(inner)=>quote!{if path.first()==Some(&0){let path=&path[1..];if path.is_empty(){Ok(<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_identity(value.get(index).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?).0)}else{<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_key(value.get(path[0]).ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?,&path[1..],index)}}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::OptionStatements(inner)=>quote!{let value=value.as_ref().ok_or_else(::semio_framework_dsl_record::native_encoding::projection_path_error)?;if path.is_empty()&&index==0{Ok(<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_identity(value).0)}else if path.first()==Some(&0){<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_key(value,&path[1..],index)}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::RequiredStatements(inner)=>quote!{if path.is_empty()&&index==0{Ok(<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_identity(value.as_ref()).0)}else if path.first()==Some(&0){<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_key(value.as_ref(),&path[1..],index)}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            FieldKind::RequiredInlineStatements(inner)=>quote!{if path.is_empty()&&index==0{Ok(<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_identity(value).0)}else if path.first()==Some(&0){<#inner as ::semio_framework_dsl_record::DslVariants>::projected_variant_key(value,&path[1..],index)}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}},
            _=>quote!{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())},
        };
        let projection=if *block{quote!{if path.first()==Some(&0){let path=&path[1..];#projection}else{Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}}}else{projection};
        quote!{#ordinal=>{let value=#value;let path=&path[1..];#projection}}
    });quote!{if path.is_empty(){return Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}match path[0]{#(#arms,)*_=>Err(::semio_framework_dsl_record::native_encoding::projection_path_error())}}
}

/// 🫴️ Keeps skipped custody optional so decoding creates absence without an unfinanced default owner.
pub(super) fn custody_field_error(input:&DeriveInput)->Option<proc_macro2::TokenStream>{
    let fields=match &input.data{Data::Struct(data)=>data.fields.iter().collect::<Vec<_>>(),Data::Enum(data)=>data.variants.iter().flat_map(|variant|variant.fields.iter()).collect(),_=>Vec::new()};
    fields.into_iter().find_map(|field|{let attrs=parse_field_attrs(&field.attrs);if attrs.skip&&!matches!(classify_field(&field.ty,&attrs).0,FieldKind::OptionScalar(_)|FieldKind::OptionStatements(_)){Some(syn::Error::new_spanned(field,"dsl skip requires an optional custody field").to_compile_error())}else{None}})
}
