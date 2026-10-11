//! 🧵️ Compiles schema wire roles into direct immutable native field projections.
use super::*;

struct FieldRole { key:String, reference:proc_macro2::TokenStream, present:proc_macro2::TokenStream, flatten:bool }

enum Wire { Direct, Decimal, HexWord, Flatten }
#[derive(Clone, Copy, PartialEq)]
enum Declared { Decimal, HexWord, Tree }

/// 🧬️ Recognizes the intrinsic octet serializer family (`pack::value::bytes[::optional]`), whose JSON projection is the plain array of numbers (or `null`) the native `Vec<u8>`/`Option<Vec<u8>>` roles already emit.
fn octets_kind(path:&str,leaf:&str)->Option<&'static str> {
    let path=path.trim_start_matches("::");
    ["pack::value::bytes","semio_framework_value::bytes"].iter().find_map(|root|{
        let rest=path.strip_prefix(root)?;
        let(kind,rest)=match rest.strip_prefix("::optional"){Some(rest)=>("optional",rest),None=>("plain",rest)};
        (rest==leaf).then_some(kind)
    })
}
fn canonical_field_role(attrs:&[syn::Attribute])->syn::Result<Option<Declared>> {
    let mut declared=None;
    for attr in attrs.iter().filter(|attr|attr.path().is_ident("canonical_json")) {
        attr.parse_nested_meta(|meta|{
            let role=if meta.path.is_ident("decimal_string"){Declared::Decimal}else if meta.path.is_ident("hex_word"){Declared::HexWord}else if meta.path.is_ident("tree"){Declared::Tree}else{return Err(meta.error("canonical_json field attribute supports only decimal_string, hex_word and tree"));};
            if declared.replace(role).is_some_and(|previous|previous!=role){return Err(meta.error("canonical_json field attribute declares one wire role"));}
            Ok(())
        })?;
    }
    Ok(declared)
}
fn wire(attrs:&FieldAttrs,declared:Option<Declared>)->syn::Result<Wire> {
    let refuse=|message:&str|Err(syn::Error::new(proc_macro2::Span::call_site(),message.to_string()));
    if attrs.flatten{
        let custom=attrs.effective_serialize_with().is_some()||attrs.serialize_controlled_with.is_some();
        return match(declared,custom){
            (None,false)|(Some(Declared::Tree),_)=>Ok(Wire::Flatten),
            _=>refuse("canonical flatten requires the field's own tree; a custom serializer must declare #[canonical_json(tree)]"),
        };
    }
    match(attrs.effective_serialize_with(),attrs.serialize_controlled_with.as_deref(),declared){
        (_,_,Some(Declared::HexWord))=>Ok(Wire::HexWord),
        (_,_,Some(Declared::Tree))=>Ok(Wire::Direct),
        (None,None,None)=>Ok(Wire::Direct),
        (None,_,_)=>refuse("canonical field declares a controlled or decimal serializer without the plain value serializer it must equal"),
        (Some(_),_,Some(Declared::Decimal))=>Ok(Wire::Decimal),
        (Some(direct),controlled,None)=>match octets_kind(&direct,"::to_value"){
            Some(kind)if controlled.is_none_or(|path|octets_kind(path,"::to_value_controlled")==Some(kind))=>Ok(Wire::Direct),
            _=>refuse("canonical field requires a direct native role; custom serializers need #[canonical_json(decimal_string)], #[canonical_json(hex_word)] or the intrinsic pack::value::bytes octet role"),
        },
    }
}
fn binary32(ty:&syn::Type)->bool {matches!(ty,syn::Type::Path(path)if path.qself.is_none()&&path.path.is_ident("f32"))}
fn type_argument(ty:&syn::Type,name:&str)->Option<syn::Type> {
    let syn::Type::Path(path)=ty else{return None};
    let segment=path.path.segments.last().filter(|segment|path.qself.is_none()&&segment.ident==name)?;
    let syn::PathArguments::AngleBracketed(arguments)=&segment.arguments else{return None};
    match arguments.args.iter().next()?{syn::GenericArgument::Type(ty)=>Some(ty.clone()),_=>None}
}
fn hex_word_reference(ty:&syn::Type,reference:&proc_macro2::TokenStream,owner:&syn::Path)->syn::Result<proc_macro2::TokenStream> {
    let view=|ty:&syn::Type|->Option<proc_macro2::TokenStream>{
        if binary32(ty){return Some(quote!(ArtifactCanonicalHexWordF32));}
        if matches!(ty,syn::Type::Array(array)if binary32(&array.elem)){return Some(quote!(ArtifactCanonicalHexWordArray));}
        type_argument(ty,"Vec").filter(binary32).map(|_|quote!(ArtifactCanonicalHexWordList))
    };
    let refuse=||syn::Error::new_spanned(ty,"canonical_json(hex_word) requires f32, [f32; N], Vec<f32> or an Option of them");
    match type_argument(ty,"Option") {
        Some(inner)=>{let view=view(&inner).ok_or_else(refuse)?;Ok(quote!(match #reference{Some(value)=>#owner::#view::from_ref(value) as &dyn #owner::ArtifactCanonicalJsonTree,None=>&() as &dyn #owner::ArtifactCanonicalJsonTree}))}
        None=>{let view=view(ty).ok_or_else(refuse)?;Ok(quote!(#owner::#view::from_ref(#reference)))}
    }
}
fn presence(attrs:&FieldAttrs,reference:&proc_macro2::TokenStream)->syn::Result<proc_macro2::TokenStream> {
    if attrs.skip{return Ok(quote!(false));}
    if attrs.flatten&&attrs.skip_serializing_if.is_some(){return Err(syn::Error::new(proc_macro2::Span::call_site(),"canonical flatten has no presence predicate; the flattened record owns its absent members"));}
    match attrs.skip_serializing_if.as_deref(){
        None=>Ok(quote!(true)),
        Some("Option::is_none")=>Ok(quote!(!(#reference).is_none())),
        Some("Vec::is_empty"|"String::is_empty"|"PagedList::is_empty"|"PagedUtf8::is_empty")=>Ok(quote!(!(#reference).is_empty())),
        Some(predicate)=>{let predicate:syn::Path=syn::parse_str(predicate)?;Ok(quote!(!#predicate(#reference)))}
    }
}
fn absent()->proc_macro2::TokenStream {quote!(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvariantViolated,"canonical schema field ordinal is absent"))}
fn roles(fields:Vec<FieldRole>,owner:&syn::Path)->(proc_macro2::TokenStream,proc_macro2::TokenStream,proc_macro2::TokenStream){
    let mut counts=Vec::new();let mut children=Vec::new();let mut keys=Vec::new();
    let error=absent();
    let flattened=quote!(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvariantViolated,"canonical flattened field is not an object"));
    for field in fields {
        let FieldRole{key,reference,present,flatten}=field;
        if flatten {
            let length=quote!(match #owner::ArtifactCanonicalJsonTree::canonical_tree_node(#reference)?{#owner::ArtifactCanonicalJsonNode::Object(length)=>length,#owner::ArtifactCanonicalJsonNode::Null=>0,_=>return Err(#flattened)});
            counts.push(length.clone());
            children.push(quote!({let length=#length;if ordinal<length{return #owner::ArtifactCanonicalJsonTree::canonical_tree_child(#reference,ordinal);}ordinal-=length;}));
            keys.push(quote!({let length=#length;if ordinal<length{return #owner::ArtifactCanonicalJsonTree::canonical_tree_key(#reference,ordinal);}ordinal-=length;}));
            continue;
        }
        counts.push(quote!(usize::from(#present)));
        children.push(quote!(if #present {if ordinal==0{return Ok(#reference);}ordinal-=1;}));
        keys.push(quote!(if #present {if ordinal==0{return Ok(#key.into());}ordinal-=1;}));
    }
    (quote!(Ok(#owner::ArtifactCanonicalJsonNode::Object(0 #(+ #counts)*))),quote!(#(#children)* Err(#error)),quote!(#(#keys)* Err(#error)))
}
fn named_roles(fields:&Fields,container:&ContainerAttrs,root:Option<&proc_macro2::TokenStream>,owner:&syn::Path)->syn::Result<Vec<FieldRole>> {
    let mut out=Vec::new();
    let Fields::Named(syntax)=fields else{return Err(syn::Error::new_spanned(fields,"canonical field roles require named fields"));};
    for(field,syntax)in named_fields(fields,container)?.into_iter().zip(&syntax.named) {
        if field.attrs.skip{continue;}
        let ident=&field.ident;
        let reference=if let Some(root)=root{quote!(&#root.#ident)}else{quote!(#ident)};
        let present=presence(&field.attrs,&reference)?;
        let reference=match wire(&field.attrs,canonical_field_role(&syntax.attrs)?)?{Wire::Direct=>reference,Wire::HexWord=>hex_word_reference(&syntax.ty,&reference,owner)?,Wire::Flatten=>reference,Wire::Decimal=>{let decimal=if matches!(&syntax.ty,syn::Type::Path(path)if path.path.is_ident("i64")){quote!(ArtifactCanonicalDecimalI64)}else{quote!(ArtifactCanonicalDecimalU64)};quote!(#owner::#decimal::from_ref(#reference))}};
        let flatten=field.attrs.flatten;
        out.push(FieldRole{key:field.wire_name,reference,present,flatten});
    }
    Ok(out)
}
fn tag_role(key:&str,wire:&str)->FieldRole {FieldRole{key:key.into(),reference:quote!({static KIND:&str=#wire;&KIND}),present:quote!(true),flatten:false}}

pub(super) fn expand(input:&DeriveInput)->syn::Result<proc_macro2::TokenStream>{
    let mut owner=None;
    for attr in input.attrs.iter().filter(|attr|attr.path().is_ident("canonical_json")) {
        attr.parse_nested_meta(|meta|{if !meta.path.is_ident("owner"){return Err(meta.error("canonical_json requires an explicit first-party owner path"));}owner=Some(meta.value()?.parse::<syn::Path>()?);Ok(())})?;
    }
    let owner=owner.ok_or_else(||syn::Error::new_spanned(input,"canonical_json(owner = first_party_path) is required"))?;
    let attrs=parse_container_attrs(&input.attrs)?;
    let error=absent();
    let mut generics=input.generics.clone();for parameter in generics.type_params_mut(){parameter.bounds.push(syn::parse_quote!(#owner::ArtifactCanonicalJsonTree));}
    let name=&input.ident;
    let mut views=Vec::new();
    let(node,child,key)=match &input.data {
        Data::Struct(data)=>{
            let transparent=attrs.transparent||matches!(&data.fields,Fields::Unnamed(fields)if fields.unnamed.len()==1);
            if transparent {
                if data.fields.len()!=1{return Err(syn::Error::new_spanned(input,"canonical transparent owner requires exactly one original field"));}
                let member=data.fields.iter().next().unwrap().ident.as_ref().map(|ident|quote!(#ident)).unwrap_or_else(||quote!(0));
                (quote!(#owner::ArtifactCanonicalJsonTree::canonical_tree_node(&self.#member)),quote!(#owner::ArtifactCanonicalJsonTree::canonical_tree_child(&self.#member,ordinal)),quote!(#owner::ArtifactCanonicalJsonTree::canonical_tree_key(&self.#member,ordinal)))
            }else{roles(named_roles(&data.fields,&attrs,Some(&quote!(self)),&owner)?,&owner)}
        }
        Data::Enum(data)=>{
            let mut nodes=Vec::new();let mut children=Vec::new();let mut keys=Vec::new();
            for variant in &data.variants {
                let name=&variant.ident;let variant_attrs=parse_variant_attrs(&variant.attrs)?;let wire=variant_wire_name(&name.to_string(),&variant_attrs.rename,&attrs.rename_all);
                let(pattern,n,c,k)=match &variant.fields {
                    Fields::Unit=>{
                        if let Some(tag)=attrs.tag.as_deref(){let(n,c,k)=roles(vec![tag_role(tag,&wire)],&owner);(quote!(Self::#name),n,c,k)}
                        else{(quote!(Self::#name),quote!(Ok(#owner::ArtifactCanonicalJsonNode::String(#wire))),quote!(Err(#error)),quote!(Err(#error)))}
                    }
                    Fields::Unnamed(fields)if fields.unnamed.len()==1=>{
                        let pattern=quote!(Self::#name(payload));
                        if let Some(tag)=attrs.tag.as_deref(){
                            if let Some(content)=attrs.content.as_deref(){let(n,c,k)=roles(vec![tag_role(tag,&wire),FieldRole{key:content.into(),reference:quote!(payload),present:quote!(true),flatten:false}],&owner);(pattern,n,c,k)}
                            else{
                                let n=quote!(match #owner::ArtifactCanonicalJsonTree::canonical_tree_node(payload)?{#owner::ArtifactCanonicalJsonNode::Object(length)=>Ok(#owner::ArtifactCanonicalJsonNode::Object(length.checked_add(1).ok_or_else(||#error)?)),_=>Err(#error)});
                                let c=quote!(if ordinal==0{static KIND:&str=#wire;return Ok(&KIND);}#owner::ArtifactCanonicalJsonTree::canonical_tree_child(payload,ordinal-1));
                                let k=quote!(if ordinal==0{return Ok(#tag.into());}#owner::ArtifactCanonicalJsonTree::canonical_tree_key(payload,ordinal-1));
                                (pattern,n,c,k)
                            }
                        }else{let(n,c,k)=roles(vec![FieldRole{key:wire.clone(),reference:quote!(payload),present:quote!(true),flatten:false}],&owner);(pattern,n,c,k)}
                    }
                    Fields::Named(fields)if attrs.tag.is_some()&&attrs.content.is_none()=>{
                        let container=ContainerAttrs{rename_all:attrs.field_rename_all(&variant_attrs),..Default::default()};
                        let mut projected=named_roles(&variant.fields,&container,None,&owner)?;projected.insert(0,tag_role(attrs.tag.as_ref().unwrap(),&wire));
                        let bindings:Vec<_>=fields.named.iter().map(|field|field.ident.as_ref().unwrap()).collect();let(n,c,k)=roles(projected,&owner);
                        (quote!(Self::#name{#(#bindings),*}),n,c,k)
                    }
                    Fields::Named(fields)if attrs.tag.is_none()=>{
                        let view=quote::format_ident!("__{}{}CanonicalView",input.ident,name);
                        let container=ContainerAttrs{rename_all:attrs.field_rename_all(&variant_attrs),..Default::default()};
                        let projected=named_roles(&variant.fields,&container,None,&owner)?;
                        let bindings:Vec<_>=fields.named.iter().map(|field|field.ident.as_ref().unwrap()).collect();let(vn,vc,vk)=roles(projected,&owner);
                        let(implementation,arguments,where_clause)=generics.split_for_impl();let enum_name=&input.ident;
                        let mismatch=quote!(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvariantViolated,"canonical variant view differs from its enum variant"));
                        views.push(quote!(
                            #[repr(transparent)]
                            #[allow(dead_code)]
                            struct #view #generics (#enum_name #arguments) #where_clause;
                            impl #implementation #owner::ArtifactCanonicalJsonTree for #view #arguments #where_clause {
                                #[allow(unused_variables)]
                                fn canonical_tree_node(&self)->Result<#owner::ArtifactCanonicalJsonNode<'_>,::semio_framework_value::ValueError>{match &self.0{#enum_name::#name{#(#bindings),*}=>{#vn}_=>Err(#mismatch)}}
                                #[allow(unused_variables)]
                                fn canonical_tree_child(&self,mut ordinal:usize)->Result<&dyn #owner::ArtifactCanonicalJsonTree,::semio_framework_value::ValueError>{match &self.0{#enum_name::#name{#(#bindings),*}=>{#vc}_=>Err(#mismatch)}}
                                #[allow(unused_variables)]
                                fn canonical_tree_key(&self,mut ordinal:usize)->Result<#owner::ArtifactCanonicalJsonText<'_>,::semio_framework_value::ValueError>{match &self.0{#enum_name::#name{#(#bindings),*}=>{#vk}_=>Err(#mismatch)}}
                            }
                        ));
                        let reference=quote!(unsafe{&*(self as *const Self as *const #view #arguments)});
                        (quote!(Self::#name{..}),quote!(Ok(#owner::ArtifactCanonicalJsonNode::Object(1))),quote!(if ordinal==0{Ok(#reference)}else{Err(#error)}),quote!(if ordinal==0{Ok(#wire.into())}else{Err(#error)}))
                    }
                    _=>return Err(syn::Error::new_spanned(variant,"canonical variant requires persistent record payload ownership rather than an anonymous object view")),
                };
                nodes.push(quote!(#pattern=>{#n}));children.push(quote!(#pattern=>{#c}));keys.push(quote!(#pattern=>{#k}));
            }
            (quote!(match self{#(#nodes),*}),quote!(match self{#(#children),*}),quote!(match self{#(#keys),*}))
        }
        _=>return Err(syn::Error::new_spanned(input,"canonical field roles require a record or variant owner")),
    };
    let(implementation,arguments,where_clause)=generics.split_for_impl();
    Ok(quote!(#(#views)* impl #implementation #owner::ArtifactCanonicalJsonTree for #name #arguments #where_clause {
        fn canonical_tree_node(&self)->Result<#owner::ArtifactCanonicalJsonNode<'_>,::semio_framework_value::ValueError>{#node}
        fn canonical_tree_child(&self,mut ordinal:usize)->Result<&dyn #owner::ArtifactCanonicalJsonTree,::semio_framework_value::ValueError>{#child}
        fn canonical_tree_key(&self,mut ordinal:usize)->Result<#owner::ArtifactCanonicalJsonText<'_>,::semio_framework_value::ValueError>{#key}
    }))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
