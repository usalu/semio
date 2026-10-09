//! 🧵️ Compiles schema wire roles into direct immutable native field projections.
use super::*;

struct FieldRole { key:String, reference:proc_macro2::TokenStream, present:proc_macro2::TokenStream }

fn presence(attrs:&FieldAttrs,reference:&proc_macro2::TokenStream)->syn::Result<proc_macro2::TokenStream> {
    if attrs.serialize_with.is_some()||attrs.with.is_some()||attrs.flatten{return Err(syn::Error::new(proc_macro2::Span::call_site(),"canonical field requires a direct native role; custom serializers and flattened views are not field owners"));}
    if attrs.skip{return Ok(quote!(false));}
    match attrs.skip_serializing_if.as_deref(){
        None=>Ok(quote!(true)),
        Some("Option::is_none")=>Ok(quote!(!(#reference).is_none())),
        Some("Vec::is_empty"|"String::is_empty"|"PagedList::is_empty"|"PagedUtf8::is_empty")=>Ok(quote!(!(#reference).is_empty())),
        Some(_)=>Err(syn::Error::new(proc_macro2::Span::call_site(),"canonical presence requires a native optional or empty-owner predicate")),
    }
}
fn absent()->proc_macro2::TokenStream {quote!(::semio_framework_value::ValueError::literal(::semio_framework_value::ValueRefusalKind::InvariantViolated,"canonical schema field ordinal is absent"))}
fn roles(fields:Vec<FieldRole>,owner:&syn::Path)->(proc_macro2::TokenStream,proc_macro2::TokenStream,proc_macro2::TokenStream){
    let mut counts=Vec::new();let mut children=Vec::new();let mut keys=Vec::new();
    for field in fields {
        let FieldRole{key,reference,present}=field;
        counts.push(quote!(usize::from(#present)));
        children.push(quote!(if #present {if ordinal==0{return Ok(#reference);}ordinal-=1;}));
        keys.push(quote!(if #present {if ordinal==0{return Ok(#key.into());}ordinal-=1;}));
    }
    let error=absent();
    (quote!(Ok(#owner::ArtifactCanonicalJsonNode::Object(0 #(+ #counts)*))),quote!(#(#children)* Err(#error)),quote!(#(#keys)* Err(#error)))
}
fn named_roles(fields:&Fields,container:&ContainerAttrs,root:Option<&proc_macro2::TokenStream>)->syn::Result<Vec<FieldRole>> {
    let mut out=Vec::new();
    for field in named_fields(fields,container)? {
        if field.attrs.skip{continue;}
        let ident=&field.ident;
        let reference=if let Some(root)=root{quote!(&#root.#ident)}else{quote!(#ident)};
        let present=presence(&field.attrs,&reference)?;
        out.push(FieldRole{key:field.wire_name,reference,present});
    }
    Ok(out)
}
fn tag_role(key:&str,wire:&str)->FieldRole {FieldRole{key:key.into(),reference:quote!({static KIND:&str=#wire;&KIND}),present:quote!(true)}}

pub(super) fn expand(input:&DeriveInput)->syn::Result<proc_macro2::TokenStream>{
    let mut owner=None;
    for attr in input.attrs.iter().filter(|attr|attr.path().is_ident("canonical_json")) {
        attr.parse_nested_meta(|meta|{if !meta.path.is_ident("owner"){return Err(meta.error("canonical_json requires an explicit first-party owner path"));}owner=Some(meta.value()?.parse::<syn::Path>()?);Ok(())})?;
    }
    let owner=owner.ok_or_else(||syn::Error::new_spanned(input,"canonical_json(owner = first_party_path) is required"))?;
    let attrs=parse_container_attrs(&input.attrs)?;
    let error=absent();
    let(node,child,key)=match &input.data {
        Data::Struct(data)=>{
            let transparent=attrs.transparent||matches!(&data.fields,Fields::Unnamed(fields)if fields.unnamed.len()==1);
            if transparent {
                if data.fields.len()!=1{return Err(syn::Error::new_spanned(input,"canonical transparent owner requires exactly one original field"));}
                let member=data.fields.iter().next().unwrap().ident.as_ref().map(|ident|quote!(#ident)).unwrap_or_else(||quote!(0));
                (quote!(#owner::ArtifactCanonicalJsonTree::canonical_tree_node(&self.#member)),quote!(#owner::ArtifactCanonicalJsonTree::canonical_tree_child(&self.#member,ordinal)),quote!(#owner::ArtifactCanonicalJsonTree::canonical_tree_key(&self.#member,ordinal)))
            }else{roles(named_roles(&data.fields,&attrs,Some(&quote!(self)))?,&owner)}
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
                            if let Some(content)=attrs.content.as_deref(){let(n,c,k)=roles(vec![tag_role(tag,&wire),FieldRole{key:content.into(),reference:quote!(payload),present:quote!(true)}],&owner);(pattern,n,c,k)}
                            else{
                                let n=quote!(match #owner::ArtifactCanonicalJsonTree::canonical_tree_node(payload)?{#owner::ArtifactCanonicalJsonNode::Object(length)=>Ok(#owner::ArtifactCanonicalJsonNode::Object(length.checked_add(1).ok_or_else(||#error)?)),_=>Err(#error)});
                                let c=quote!(if ordinal==0{static KIND:&str=#wire;return Ok(&KIND);}#owner::ArtifactCanonicalJsonTree::canonical_tree_child(payload,ordinal-1));
                                let k=quote!(if ordinal==0{return Ok(#tag.into());}#owner::ArtifactCanonicalJsonTree::canonical_tree_key(payload,ordinal-1));
                                (pattern,n,c,k)
                            }
                        }else{let(n,c,k)=roles(vec![FieldRole{key:wire.clone(),reference:quote!(payload),present:quote!(true)}],&owner);(pattern,n,c,k)}
                    }
                    Fields::Named(fields)if attrs.tag.is_some()&&attrs.content.is_none()=>{
                        let container=ContainerAttrs{rename_all:attrs.field_rename_all(&variant_attrs),..Default::default()};
                        let mut projected=named_roles(&variant.fields,&container,None)?;projected.insert(0,tag_role(attrs.tag.as_ref().unwrap(),&wire));
                        let bindings:Vec<_>=fields.named.iter().map(|field|field.ident.as_ref().unwrap()).collect();let(n,c,k)=roles(projected,&owner);
                        (quote!(Self::#name{#(#bindings),*}),n,c,k)
                    }
                    _=>return Err(syn::Error::new_spanned(variant,"canonical variant requires persistent record payload ownership rather than an anonymous object view")),
                };
                nodes.push(quote!(#pattern=>{#n}));children.push(quote!(#pattern=>{#c}));keys.push(quote!(#pattern=>{#k}));
            }
            (quote!(match self{#(#nodes),*}),quote!(match self{#(#children),*}),quote!(match self{#(#keys),*}))
        }
        _=>return Err(syn::Error::new_spanned(input,"canonical field roles require a record or variant owner")),
    };
    let mut generics=input.generics.clone();for parameter in generics.type_params_mut(){parameter.bounds.push(syn::parse_quote!(#owner::ArtifactCanonicalJsonTree));}
    let(implementation,arguments,where_clause)=generics.split_for_impl();let name=&input.ident;
    Ok(quote!(impl #implementation #owner::ArtifactCanonicalJsonTree for #name #arguments #where_clause {
        fn canonical_tree_node(&self)->Result<#owner::ArtifactCanonicalJsonNode<'_>,::semio_framework_value::ValueError>{#node}
        fn canonical_tree_child(&self,mut ordinal:usize)->Result<&dyn #owner::ArtifactCanonicalJsonTree,::semio_framework_value::ValueError>{#child}
        fn canonical_tree_key(&self,mut ordinal:usize)->Result<#owner::ArtifactCanonicalJsonText<'_>,::semio_framework_value::ValueError>{#key}
    }))
}
