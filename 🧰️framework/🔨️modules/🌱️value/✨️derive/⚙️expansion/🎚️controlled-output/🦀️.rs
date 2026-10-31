//! 🎚️ Controlled (cumulative-budget) encode, decode and retirement bodies.

use quote::{quote, format_ident};
use syn::{Data, DeriveInput, Fields};
use super::{ContainerAttrs, FieldAttrs, FieldDefault, field_wire_name, parse_field_attrs, parse_variant_attrs, type_is_option, variant_wire_name};

pub(super) fn controlled_semantic_admission(body: proc_macro2::TokenStream, container: &ContainerAttrs, c: &syn::Path, custom_constructor: bool) -> syn::Result<proc_macro2::TokenStream> {
    if let Some(path) = &container.validate_controlled {
        let validator: syn::Path = syn::parse_str(path)?;
        return Ok(quote! {
            let decoded = (|| -> ::core::result::Result<Self, #c::ValueError> { #body })()?;
            let admission = control.scoped_stage(|control| { control.begin_stage(0)?; #validator(&decoded, control) });
            if let Err(error) = admission {
                <Self as #c::FromValue>::retire_decoded(decoded);
                return Err(error);
            }
            Ok(decoded)
        });
    }
    if container.validate.is_some() && !custom_constructor {
        return Ok(quote! { control.checkpoint()?; Err(#c::ValueError::literal(#c::ValueRefusalKind::UnsupportedOwner, "semantic value validation requires an explicit controlled hook or constructor")) });
    }
    Ok(body)
}

fn controlled_field_decode(ty:&syn::Type,attrs:&FieldAttrs,value:proc_macro2::TokenStream,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    if attrs.effective_deserialize_with().is_some()&&attrs.retire_with.is_none(){return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "custom controlled value conversion requires explicit retirement"))})}
    if let Some(path)=attrs.deserialize_controlled_with.clone(){
        let path:syn::Path=syn::parse_str(&path)?;return Ok(quote!{#path(#value,control)})
    }
    if attrs.effective_deserialize_with().is_some(){return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "custom value conversion has no controlled constructor"))})}
    Ok(quote!{<#ty as #c::FromValue>::from_value_controlled(#value,control)})
}

fn controlled_field_default(ty:&syn::Type,attrs:&FieldAttrs,container:&ContainerAttrs,wire:&str,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    if attrs.skip&&attrs.retire_with.is_none(){return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "skipped controlled default requires explicit retirement"))})}
    if attrs.required&&!attrs.skip{return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, format!("missing field `{}`",#wire)))})}
    if let Some(path)=&attrs.default_controlled{let path:syn::Path=syn::parse_str(path)?;return Ok(quote!{#path(control)})}
    if matches!(attrs.default,FieldDefault::Path(_))||attrs.skip{return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "custom or skipped default has no controlled constructor"))})}
    if matches!(attrs.default,FieldDefault::Bare)||container.default||type_is_option(ty){
        if attrs.effective_deserialize_with().is_some(){return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "custom value default has no controlled constructor"))})}
        return Ok(quote!{<#ty as #c::FromValue>::default_value_controlled(control)})
    }
    Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, format!("missing field `{}`",#wire)))})
}

fn controlled_field_retire(ty:&syn::Type,attrs:&FieldAttrs,value:proc_macro2::TokenStream,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    if let Some(path)=&attrs.retire_with{let path:syn::Path=syn::parse_str(path)?;return Ok(quote!{#path(#value)})}
    if attrs.skip||attrs.effective_deserialize_with().is_some(){return Ok(quote!{drop(#value)})}
    Ok(quote!{<#ty as #c::FromValue>::retire_decoded(#value)})
}

fn controlled_named_fields(fields:&syn::FieldsNamed,container:&ContainerAttrs,rename:&Option<String>,entries:proc_macro2::TokenStream,extra:&[String],constructor:proc_macro2::TokenStream,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    let mut reads=Vec::new();let mut members=Vec::new();let mut names=Vec::new();
    for field in &fields.named{let attrs=parse_field_attrs(&field.attrs)?;let ident=field.ident.as_ref().unwrap();let wire=field_wire_name(&ident.to_string(),&attrs.rename,rename);if !attrs.flatten{names.push(wire.clone())}}
    let allowed:Vec<_>=names.iter().chain(extra.iter()).collect();
    let deny=if container.deny_unknown_fields{quote!{#c::DslValue::deny_fields_controlled(#entries,&[#(#allowed),*],control)?;}}else{quote!{}};
    for field in &fields.named{
        let attrs=parse_field_attrs(&field.attrs)?;let ident=field.ident.as_ref().unwrap();let ty=&field.ty;let wire=field_wire_name(&ident.to_string(),&attrs.rename,rename);let guard=format_ident!("__owned_{}",ident);
        let missing=controlled_field_default(ty,&attrs,container,&wire,c)?;
        let expression=if attrs.skip{missing}else if attrs.flatten{
            let decode=controlled_field_decode(ty,&attrs,quote!{__remaining.get()},c)?;
            quote!{{let __remaining=<#c::DslValue as #c::FromValue>::guard_decoded(#c::DslValue::filtered_object_controlled(#entries,&[#(#names),*],control)?);#decode}}
        }else{
            let decode=controlled_field_decode(ty,&attrs,quote!{__field},c)?;
            quote!{match #c::DslValue::field_controlled(#entries,#wire,control)?{Some(__field)=>#decode,None=>#missing}}
        };
        let retire=controlled_field_retire(ty,&attrs,quote!{__value},c)?;
        if attrs.effective_deserialize_with().is_some()||attrs.skip||attrs.retire_with.is_some(){
            reads.push(quote!{let #guard:#ty=#expression.map_err(|error:#c::ValueError|error.under(#wire))?;let #guard= #c::DecodedValue::new(#guard,|__value:#ty|{#retire});control.step()?;});
        }else{
            reads.push(quote!{let #guard=<#ty as #c::FromValue>::guard_decoded(#expression.map_err(|error:#c::ValueError|error.under(#wire))?);control.step()?;});
        }
        members.push(quote!{#ident:#guard.take()});
    }
    let count=fields.named.len();
    Ok(quote!{#deny control.begin_stage(#count)?;#(#reads)* Ok(#constructor{#(#members),*})})
}

fn type_mentions_owner(ty:&syn::Type,owner:&syn::Ident)->bool {
    match ty {
        syn::Type::Path(path)=>path.path.segments.last().is_some_and(|segment| {
            segment.ident==*owner||segment.ident=="Self"||match &segment.arguments {
                syn::PathArguments::AngleBracketed(arguments)=>arguments.args.iter().any(|argument|match argument {
                    syn::GenericArgument::Type(ty)=>type_mentions_owner(ty,owner),
                    syn::GenericArgument::AssocType(binding)=>type_mentions_owner(&binding.ty,owner),
                    _=>false,
                }),
                _=>false,
            }
        }),
        syn::Type::Array(array)=>type_mentions_owner(&array.elem,owner),
        syn::Type::Slice(slice)=>type_mentions_owner(&slice.elem,owner),
        syn::Type::Tuple(tuple)=>tuple.elems.iter().any(|ty|type_mentions_owner(ty,owner)),
        syn::Type::Paren(paren)=>type_mentions_owner(&paren.elem,owner),
        syn::Type::Group(group)=>type_mentions_owner(&group.elem,owner),
        syn::Type::Reference(reference)=>type_mentions_owner(&reference.elem,owner),
        syn::Type::Ptr(pointer)=>type_mentions_owner(&pointer.elem,owner),
        _=>false,
    }
}

pub(super) fn controlled_from_body(input:&DeriveInput,container:&ContainerAttrs,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    if let Some(path)=&container.deserialize_controlled_with{let visitor:syn::Path=syn::parse_str(path).map_err(|error|syn::Error::new_spanned(input,error))?;return Ok(quote!{#visitor(value,control)});}
    let recursive=input.data.clone();
    let mentions=match &recursive{Data::Struct(data)=>data.fields.iter().any(|f|type_mentions_owner(&f.ty,&input.ident)),Data::Enum(data)=>data.variants.iter().flat_map(|v|v.fields.iter()).any(|f|type_mentions_owner(&f.ty,&input.ident)),_=>false};
    if mentions&&container.retire_with.is_none(){return Ok(quote!{control.checkpoint()?;Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "recursive value owner requires explicit controlled retirement"))})}
    match &input.data{
        Data::Struct(data) if container.transparent||matches!(&data.fields,Fields::Unnamed(f)if f.unnamed.len()==1)=>{
            let field=data.fields.iter().next().unwrap();let attrs=parse_field_attrs(&field.attrs)?;let ty=&field.ty;let decode=controlled_field_decode(ty,&attrs,quote!{value},c)?;
            Ok(if let Some(ident)=&field.ident{quote!{control.begin_stage(0)?;Ok(Self{#ident:#decode?})}}else{quote!{control.begin_stage(0)?;Ok(Self(#decode?))}})
        },
        Data::Struct(data)=>{
            let Fields::Named(fields)=&data.fields else{return Err(syn::Error::new_spanned(&data.fields,"controlled value requires named fields"))};
            let fields=controlled_named_fields(fields,container,&container.rename_all,quote!{__entries},&[],quote!{Self},c)?;
            Ok(quote!{let __entries=value.object_controlled(control)?;#fields})
        },
        Data::Enum(data)=>{
            let mut string_arms=Vec::new();let mut object_arms=Vec::new();
            for variant in &data.variants{
                let attrs=parse_variant_attrs(&variant.attrs)?;let ident=&variant.ident;let wire=variant_wire_name(&ident.to_string(),&attrs.rename,&container.rename_all);
                if matches!(variant.fields,Fields::Unit){
                    string_arms.push(quote!{#wire=>Ok(Self::#ident),});
                    let deny=if container.deny_unknown_fields{let allowed:Vec<_>=container.tag.iter().collect();quote!{#c::DslValue::deny_fields_controlled(__entries,&[#(#allowed),*],control)?;}}else{quote!{}};
                    object_arms.push(quote!{#wire=>{#deny Ok(Self::#ident)},});continue
                }
                let payload=match(&container.tag,&container.content){
                    (None,_)=>quote!{__payload},
                    (Some(_),Some(content))=>quote!{#c::DslValue::field_controlled(__entries,#content,control)?.ok_or_else(||#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "missing enum content"))?},
                    (Some(_),None)=>quote!{value},
                };
                let body=match &variant.fields{
                    Fields::Unnamed(fields)if fields.unnamed.len()==1=>{
                        let field=fields.unnamed.first().unwrap();let ty=&field.ty;
                        if let(Some(tag),None)=(&container.tag,&container.content){
                            quote!{
                                let __remaining=<#c::DslValue as #c::FromValue>::guard_decoded(#c::DslValue::filtered_object_controlled(__entries,&[#tag],control)?);
                                match <#ty as #c::FromValue>::from_value_controlled(__remaining.get(),control){
                                    Ok(payload)=>Ok(Self::#ident(payload)),
                                    Err(error)=>match __remaining.get(){#c::DslValue::Object(fields)=>match fields.as_slice(){[(key,payload)]if key=="value"=>Ok(Self::#ident(<#ty as #c::FromValue>::from_value_controlled(payload,control)?)),_=>Err(error)},_=>Err(error)}
                                }
                            }
                        }else{let attrs=parse_field_attrs(&field.attrs)?;let decode=controlled_field_decode(ty,&attrs,payload,c)?;quote!{Ok(Self::#ident(#decode?))}}
                    },
                    Fields::Named(fields)=>{
                        let extra=if container.content.is_none(){container.tag.iter().cloned().collect::<Vec<_>>()}else{Vec::new()};
                        let body=controlled_named_fields(fields,container,&container.field_rename_all(&attrs),quote!{__variant_entries},&extra,quote!{Self::#ident},c)?;
                        quote!{let __variant_entries=(#payload).object_controlled(control)?;#body}
                    },
                    _=>return Err(syn::Error::new_spanned(&variant.fields,"unsupported controlled enum fields"))
                };
                object_arms.push(quote!{#wire=>{#body},});
            }
            if container.tag.is_none()&&data.variants.iter().all(|v|matches!(v.fields,Fields::Unit)){
                return Ok(quote!{control.begin_stage(1)?;control.step()?;let #c::DslValue::String(tag)=value else{return Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "expected enum string"))};match tag.as_str(){#(#string_arms)*_=>Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "unknown enum variant"))}})
            }
            if let Some(tag)=&container.tag{
                let deny=if let(Some(content),true)=(&container.content,container.deny_unknown_fields){quote!{#c::DslValue::deny_fields_controlled(__entries,&[#tag,#content],control)?;}}else{quote!{}};
                Ok(quote!{let __entries=value.object_controlled(control)?;#deny let __tag=#c::DslValue::field_controlled(__entries,#tag,control)?.ok_or_else(||#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "missing enum tag"))?;let #c::DslValue::String(__tag)=__tag else{return Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "expected enum string tag"))};match __tag.as_str(){#(#object_arms)*_=>Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "unknown enum variant"))}})
            }else{
                Ok(quote!{control.begin_stage(0)?;if let #c::DslValue::String(tag)=value{return match tag.as_str(){#(#string_arms)*_=>Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "unknown enum variant"))}}let __entries=value.object_controlled(control)?;let[(__tag,__payload)]=__entries else{return Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "externally tagged enum requires one field"))};match __tag.as_str(){#(#object_arms)*_=>Err(#c::ValueError::new(#c::ValueRefusalKind::InvalidValue, "unknown enum variant"))}})
            }
        },
        Data::Union(_)=>Err(syn::Error::new_spanned(input,"controlled unions unsupported"))
    }
}

pub(super) fn controlled_retirement_body(input:&DeriveInput,container:&ContainerAttrs,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
    if let Some(path)=&container.retire_with{let path:syn::Path=syn::parse_str(path)?;return Ok(quote!{#path(self);})}
    let body=|fields:&Fields,constructor:proc_macro2::TokenStream|->syn::Result<proc_macro2::TokenStream>{
        let mut names=Vec::new();let mut retirements=Vec::new();
        for(index,field)in fields.iter().enumerate(){let name=field.ident.clone().unwrap_or_else(||format_ident!("__field_{index}"));let attrs=parse_field_attrs(&field.attrs)?;retirements.push(controlled_field_retire(&field.ty,&attrs,quote!{#name},c)?);names.push(name);}
        let pattern=match fields{Fields::Named(_)=>quote!{#constructor{#(#names),*}},Fields::Unnamed(_)=>quote!{#constructor(#(#names),*)},Fields::Unit=>constructor};
        Ok(quote!{#pattern=>{#(#retirements;)*}})
    };
    match &input.data{Data::Struct(data)=>{let arm=body(&data.fields,quote!{Self})?;Ok(quote!{match self{#arm}})},Data::Enum(data)=>{let arms=data.variants.iter().map(|v|{let name=&v.ident;body(&v.fields,quote!{Self::#name})}).collect::<syn::Result<Vec<_>>>()?;Ok(quote!{match self{#(#arms),*}})},Data::Union(_)=>Ok(quote!{drop(self)})}
}

fn controlled_field_encode(attrs:&FieldAttrs,value:proc_macro2::TokenStream,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
 if let Some(path)=&attrs.serialize_controlled_with{let path:syn::Path=syn::parse_str(path)?;return Ok(quote!{#path(#value,control)})}
 if attrs.effective_serialize_with().is_some(){return Ok(quote!{Err(#c::ValueError::new(#c::ValueRefusalKind::UnsupportedOwner, "custom value conversion has no controlled encoder"))})}
 Ok(quote!{#c::ToValue::to_value_controlled(#value,control)})
}

fn controlled_named_output(fields:&syn::FieldsNamed,rename:&Option<String>,source_self:bool,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
 let count=fields.named.len();let mut flags=Vec::new();let mut capacities=Vec::new();let mut pushes=Vec::new();
 for(index,field)in fields.named.iter().enumerate(){
  let ident=field.ident.as_ref().unwrap();let flag=format_ident!("__emit_{index}");let attrs=parse_field_attrs(&field.attrs)?;let wire=field_wire_name(&ident.to_string(),&attrs.rename,rename);let alias=format_ident!("__source_field_{index}");let access=if source_self{quote!{&self.#ident}}else{quote!{#alias}};
  let flatten=attrs.flatten&&source_self;
  let emit=if attrs.skip{quote!{false}}else if !flatten{if let Some(path)=&attrs.skip_serializing_if{let path:syn::Path=syn::parse_str(path)?;quote!{!#path(#access)}}else{quote!{true}}}else{quote!{true}};
  flags.push(quote!{let #flag=#emit;control.step()?;});
  if attrs.skip{pushes.push(quote!{control.step()?;});continue}
  if !flatten{capacities.push(quote!{::core::primitive::usize::from(#flag)});}
  let value=controlled_field_encode(&attrs,access,c)?;
  let push=if flatten{quote!{#c::DslValue::flatten_encoding_controlled(__output.get_mut(),#value?,control)?;}}else{quote!{#c::DslValue::push_encoding_controlled(__output.get_mut(),#wire,#value?,control)?;}};
  pushes.push(quote!{if #flag{#push}control.step()?;});
 }
 Ok(quote!{control.begin_stage(#count)?;#(#flags)*control.begin_stage(#count)?;let mut __output=#c::DslValue::object_encoding_controlled(0usize #( + #capacities )*,control)?;#(#pushes)*Ok(#c::DslValue::Object(__output.take()))})
}

pub(super) fn controlled_to_body(input:&DeriveInput,container:&ContainerAttrs,c:&syn::Path)->syn::Result<proc_macro2::TokenStream>{
 match &input.data{
  Data::Enum(data)if data.variants.is_empty()=>Ok(quote!{match *self{}}),
  Data::Struct(data)if container.transparent||matches!(&data.fields,Fields::Unnamed(fields)if fields.unnamed.len()==1)=>{
   let field=data.fields.iter().next().ok_or_else(||syn::Error::new_spanned(input,"transparent output requires field"))?;let access=if let Some(ident)=&field.ident{quote!{&self.#ident}}else{quote!{&self.0}};controlled_field_encode(&parse_field_attrs(&field.attrs)?,access,c)
  }
  Data::Struct(data)=>{let Fields::Named(fields)=&data.fields else{return Err(syn::Error::new_spanned(input,"controlled output requires named fields"))};controlled_named_output(fields,&container.rename_all,true,c)}
  Data::Enum(data)=>{
   let mut arms=Vec::new();
   for variant in &data.variants{
    let ident=&variant.ident;let attrs=parse_variant_attrs(&variant.attrs)?;let wire=variant_wire_name(&ident.to_string(),&attrs.rename,&container.rename_all);
    let(pattern,payload)=match &variant.fields{
     Fields::Unit=>(quote!{Self::#ident},None),
     Fields::Unnamed(fields)if fields.unnamed.len()==1=>{let field=fields.unnamed.first().unwrap();(quote!{Self::#ident(__payload)},Some(controlled_field_encode(&parse_field_attrs(&field.attrs)?,quote!{__payload},c)?))},
     Fields::Named(fields)=>{let idents=fields.named.iter().enumerate().map(|(index,field)|{let name=field.ident.as_ref().unwrap();let alias=format_ident!("__source_field_{index}");if parse_field_attrs(&field.attrs)?.skip{Ok(quote!{#name:_})}else{Ok(quote!{#name:#alias})}}).collect::<syn::Result<Vec<_>>>()?;(quote!{Self::#ident{#(#idents),*}},Some(controlled_named_output(fields,&container.field_rename_all(&attrs),false,c)?))},
     _=>return Err(syn::Error::new_spanned(variant,"unsupported controlled output fields"))
    };
    let body=match(&container.tag,&container.content,payload){
     (None,_,None)=>quote!{control.copy_text(#wire).map(#c::DslValue::String)},
     (None,_,Some(payload))=>quote!{let __payload=#c::DslValue::guard_encoded((||->::core::result::Result<#c::DslValue,#c::ValueError>{#payload})()?);let mut __wrapper=#c::DslValue::object_encoding_controlled(1,control)?;#c::DslValue::push_encoding_controlled(__wrapper.get_mut(),#wire,__payload.take(),control)?;Ok(#c::DslValue::Object(__wrapper.take()))},
     (Some(tag),content,payload)=>{
      let count=if payload.is_some(){2usize}else{1};let payload_push=match(payload,content){
       (None,_)=>quote!{},
       (Some(payload),Some(content))=>quote!{let __payload=#c::DslValue::guard_encoded((||->::core::result::Result<#c::DslValue,#c::ValueError>{#payload})()?);#c::DslValue::push_encoding_controlled(__wrapper.get_mut(),#content,__payload.take(),control)?;},
       (Some(payload),None)=>quote!{let __payload=#c::DslValue::guard_encoded((||->::core::result::Result<#c::DslValue,#c::ValueError>{#payload})()?);if matches!(__payload.get(),#c::DslValue::Object(_)){#c::DslValue::flatten_encoding_controlled(__wrapper.get_mut(),__payload.take(),control)?;}else{#c::DslValue::push_encoding_controlled(__wrapper.get_mut(),"value",__payload.take(),control)?;}}
      };
      quote!{let mut __wrapper=#c::DslValue::object_encoding_controlled(#count,control)?;let __tag=control.copy_text(#wire).map(#c::DslValue::String)?;#c::DslValue::push_encoding_controlled(__wrapper.get_mut(),#tag,__tag,control)?;#payload_push Ok(#c::DslValue::Object(__wrapper.take()))}
     }
    };
    arms.push(quote!{#pattern=>{#body}});
   }
   Ok(quote!{match self{#(#arms),*}})
  }
  Data::Union(_)=>Err(syn::Error::new_spanned(input,"controlled output unions unsupported"))
 }
}
