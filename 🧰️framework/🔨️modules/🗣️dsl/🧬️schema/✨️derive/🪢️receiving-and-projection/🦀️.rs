//! 🪢️ Receiving and projection of the record derive: the typed receiving fragments and the ordinary record projection expansion with its input record. See [`super`].

use super::*;

fn receiving_fields(fields:&Fields,construct:proc_macro2::TokenStream,frame:syn::Ident)->proc_macro2::TokenStream{
 let plans=plan_fields(fields);let declared=plans.iter().filter(|plan|!plan.skip).map(|plan|plan.id);let required=plans.iter().filter(|plan|!plan.skip).map(|plan|!plan.optional);let members=plans.iter().map(|plan|{let ident=&plan.ident;let ty=&plan.field_ty;quote!{#ident:Option<#ty>}});let empty=plans.iter().map(|plan|{let ident=&plan.ident;quote!{#ident:None}});
 let bindings=plans.iter().map(|plan|{
  let FieldPlan{ident,id,kind,field_ty,block,..}=plan;
  if plan.skip{return quote!{slot.as_mut().unwrap().#ident=Some(None);}}
  let expression=match kind{
   FieldKind::OptionScalar(inner)=>quote!{if matches!(value,::semio_framework_dsl_record::FieldValue::Absent){None}else{Some(meter.take(<#inner as ::semio_framework_dsl_record::DslReceivingField>::receive_value(value,original))?) }},
   _=>quote!{meter.take(<#field_ty as ::semio_framework_dsl_record::DslReceivingField>::receive_value(value,original))?},
  };
  let expression=if *block{quote!{{let value=match value{::semio_framework_dsl_record::FieldValue::Block(inner)=>inner.as_ref(),::semio_framework_dsl_record::FieldValue::Absent=>value,_=>return Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected original receiving block").into())};#expression}}}else{expression};
  quote!{let value=record.get(#id).unwrap_or(&::semio_framework_dsl_record::FieldValue::Absent);slot.as_mut().unwrap().#ident=Some(#expression);}
 });
 let inits=plans.iter().map(|plan|{let ident=&plan.ident;quote!{#ident:slot.as_mut().unwrap().#ident.take().unwrap()}});
 let output=if matches!(fields,Fields::Unit){construct}else{quote!{#construct{#(#inits),*}}};
 quote!{::semio_framework_dsl_record::receiving::receiving_turn(|meter|{::semio_framework_dsl_record::receiving::owned::declared_record(record,&[#((#declared,#required)),*],owner)?;#[derive(::semio_framework_value::RetireOwned)]struct #frame{#(#members),*}meter.take(owner.receive_nested::<#frame,Self,::semio_framework_dsl_record::receiving::NativeSnapshotFailure>(|slot,original|::semio_framework_dsl_record::receiving::receiving_turn(|meter|{*slot=Some(#frame{#(#empty),*});#(#bindings)*::semio_framework_dsl_record::receiving::work(original,::std::mem::size_of::<Self>(),1)?;meter.performed();Ok(#output)})))})}
}

pub(super) fn emit_receiving(input:DeriveInput)->proc_macro2::TokenStream{
 let name=&input.ident;
 if let Data::Enum(data)=&input.data{if !data.variants.is_empty()&&data.variants.iter().all(|variant|matches!(variant.fields,Fields::Unit)){
  let arms=data.variants.iter().enumerate().map(|(ordinal,variant)|{let ident=&variant.ident;let ordinal=ordinal as u32;quote!{#ordinal=>Ok(#name::#ident)}}).collect::<Vec<_>>();
  let arms_owned=arms.clone();
  return quote!{
   impl ::semio_framework_dsl_record::DslReceivingField for #name{fn receive_value(value:&::semio_framework_dsl_record::FieldValue,owner:&mut ::semio_framework_dsl_record::receiving::NativeSnapshotDecodeOwner<'_,'_>)->::semio_framework_value::RetainedWorkOutcome<Self,::semio_framework_dsl_record::receiving::NativeSnapshotFailure>{::semio_framework_dsl_record::receiving::receiving_turn(|meter|{::semio_framework_dsl_record::receiving::work(owner,::core::mem::size_of::<Self>(),1)?;meter.performed();let result=match value{::semio_framework_dsl_record::FieldValue::Enum(ordinal)=>match *ordinal{#(#arms,)* _=>Err(::semio_framework_dsl_record::receiving::owned::invalid())},_=>Err(::semio_framework_dsl_record::receiving::owned::invalid())};result.map_err(Into::into)})}}
   impl ::semio_framework_dsl_record::receiving::owned::scalar_sealed::Sealed for #name{}
   impl ::semio_framework_dsl_record::receiving::owned::OwnedScalar for #name{fn creation_bytes()->usize{::core::mem::size_of::<Self>()}fn take_source(source:&mut Option<::semio_framework_dsl_record::FieldValue>)->Result<Self,::semio_framework_value::ValueError>{let Some(::semio_framework_dsl_record::FieldValue::Enum(ordinal))=source.as_ref()else{return Err(::semio_framework_dsl_record::receiving::owned::invalid())};let output:Result<Self,::semio_framework_value::ValueError>=match *ordinal{#(#arms_owned,)* _=>Err(::semio_framework_dsl_record::receiving::owned::invalid())};let output=output?;source.take();Ok(output)}}
   impl ::semio_framework_dsl_record::receiving::owned::DslOwnedReceivingField for #name{type Cursor=::semio_framework_dsl_record::receiving::owned::ScalarReceiving<Self>;fn receiving_cursor()->Self::Cursor{::core::default::Default::default()}}
  };
 }}
 let name=&input.ident;
 let synchronous=match &input.data{
  Data::Struct(data)=>{
   if !matches!(data.fields,Fields::Named(_)|Fields::Unit){return syn::Error::new_spanned(&input,"DslReceiving records require named fields").to_compile_error()}
   let body=receiving_fields(&data.fields,quote!{Self},quote::format_ident!("__{}ReceivingFrame",name));
   quote!{impl ::semio_framework_dsl_record::DslReceivingRecord for #name{fn receive_record(record:&::semio_framework_dsl_record::RecordValue,owner:&mut ::semio_framework_dsl_record::receiving::NativeSnapshotDecodeOwner<'_,'_>)->::semio_framework_value::RetainedWorkOutcome<Self,::semio_framework_dsl_record::receiving::NativeSnapshotFailure>{#body}}impl ::semio_framework_dsl_record::DslReceivingField for #name{fn receive_value(value:&::semio_framework_dsl_record::FieldValue,owner:&mut ::semio_framework_dsl_record::receiving::NativeSnapshotDecodeOwner<'_,'_>)->::semio_framework_value::RetainedWorkOutcome<Self,::semio_framework_dsl_record::receiving::NativeSnapshotFailure>{match value{::semio_framework_dsl_record::FieldValue::Record(record)=><Self as ::semio_framework_dsl_record::DslReceivingRecord>::receive_record(record,owner),_=>::semio_framework_value::RetainedWorkOutcome{result:Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvalidValue,"expected original receiving record").into()),work:0}}}}}
  },
  Data::Enum(data)=>{
   let arms=data.variants.iter().map(|variant|{let attrs=parse_field_attrs(&variant.attrs);let ident=&variant.ident;let keyword=attrs.key.unwrap_or_else(||to_kebab(&ident.to_string()));let body=match &variant.fields{Fields::Unnamed(fields)if fields.unnamed.len()==1=>{let ty=&fields.unnamed[0].ty;quote!{owner.receive_nested::<#ty,Self,::semio_framework_dsl_record::receiving::NativeSnapshotFailure>(|slot,original|::semio_framework_dsl_record::receiving::receiving_turn(|meter|{*slot=Some(meter.take(<#ty as ::semio_framework_dsl_record::DslReceivingRecord>::receive_record(record,original))?);::semio_framework_dsl_record::receiving::work(original,::std::mem::size_of::<Self>(),1)?;meter.performed();Ok(Self::#ident(slot.take().unwrap()))}))}},fields=>receiving_fields(fields,quote!{Self::#ident},quote::format_ident!("__{}{}ReceivingFrame",name,ident))};quote!{#keyword=>{#body}}});
   quote!{impl ::semio_framework_dsl_record::DslReceivingVariants for #name{fn receive_named_record(keyword:&str,record:&::semio_framework_dsl_record::RecordValue,owner:&mut ::semio_framework_dsl_record::receiving::NativeSnapshotDecodeOwner<'_,'_>)->::semio_framework_value::RetainedWorkOutcome<Self,::semio_framework_dsl_record::receiving::NativeSnapshotFailure>{match keyword{#(#arms,)*_=>::semio_framework_value::RetainedWorkOutcome{result:Err(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvalidValue,"unknown original receiving variant").into()),work:0}}}}}
  },
  _=>syn::Error::new_spanned(&input,"DslReceiving requires a record or tagged variants").to_compile_error(),
 };let owned=owned_receiving::emit_owned_receiving(&input);quote!{#synchronous #owned}
}

pub(super) struct ProjectionInput { pub(super) spec: syn::Ident, pub(super) to: syn::Ident, pub(super) from: syn::Ident, pub(super) declaration: DeriveInput }

impl syn::parse::Parse for ProjectionInput {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let spec = input.parse()?; input.parse::<syn::Token![,]>()?;
        let to = input.parse()?; input.parse::<syn::Token![,]>()?;
        let from = input.parse()?; input.parse::<syn::Token![;]>()?;
        Ok(Self { spec, to, from, declaration: input.parse()? })
    }
}

pub(super) fn emit_projection(input: DeriveInput, spec: syn::Ident, to: syn::Ident, from: syn::Ident) -> proc_macro2::TokenStream {
    if let Some(error)=custody_field_error(&input){return error;}
    let name = input.ident.clone();
    let container = parse_container_attrs(&input);
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "RecordProjection only supports structs").to_compile_error();
    };
    let (spec_exprs, to_value_stmts, from_value_stmts, field_idents) = record_codegen(&data.fields);
    let field_initializers = field_inits(&field_idents);
    let keyword_expr = match &container.keyword {
        Some(k) => quote! { Some(#k.to_string()) },
        None => quote! { None },
    };
    let layout_expr = if container.lines_layout {
        quote! { ::semio_framework_dsl_record::RecordLayout::Lines }
    } else {
        quote! { ::semio_framework_dsl_record::RecordLayout::Inline }
    };

    let borrowed=borrowed_record_owner_codegen(&input,false);
    let expanded = quote! {
        #borrowed
        impl #name {
            pub fn #spec() -> ::semio_framework_dsl_record::RecordSpec {
                ::semio_framework_dsl_record::RecordSpec::new_owned(#keyword_expr, #layout_expr, vec![ #(#spec_exprs),* ])
            }
            pub fn #to(&self) -> ::semio_framework_dsl_record::RecordValue {
                let mut record = ::semio_framework_dsl_record::RecordValue::default();
                #(#to_value_stmts)*
                record
            }
            pub fn #from(record: &::semio_framework_dsl_record::RecordValue) -> Result<Self, ::semio_framework_dsl_record::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_initializers),* })
            }
        }

    };
    expanded
}
