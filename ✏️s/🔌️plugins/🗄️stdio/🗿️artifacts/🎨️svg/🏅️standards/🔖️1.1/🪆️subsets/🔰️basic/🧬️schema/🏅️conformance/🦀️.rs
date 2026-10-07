//! 🛡️ Owned SVG profile validation with caller supplied progress.
use crate::schema::snapshot::{SvgSnapshot,SvgAttr,SvgNode,SvgAttributeValue};
use semio_framework_diagnostic::{Diagnostic,FaultCode,Severity,TextSpan};
use semio_framework_value::ValueError;
use std::collections::HashMap;
    const BLOCKED_FILTER_PRIMITIVES: &[&str] = &["feConvolveMatrix", "feDisplacementMap", "feTurbulence", "feMorphology", "feDiffuseLighting", "feSpecularLighting", "feDistantLight", "fePointLight", "feSpotLight"];

    const TEXT_ELEMENTS: &[&str] = &["text", "tspan", "tref", "textPath"];

    fn local_name(name: &str) -> &str {
        name.rsplit(':').next().unwrap_or(name)
    }

    fn attr_val<'a>(attrs: &'a [SvgAttr], name: &str) -> Option<&'a str> {
        attrs.iter().find(|a| local_name(&a.name) == name).and_then(|a| a.value.text())
    }

    fn clip_path_ref_id(attrs:&[SvgAttr])->Option<&str>{attrs.iter().find(|a|a.name=="clip-path").and_then(|a|match &a.value{SvgAttributeValue::LocalReference(id)=>Some(id.as_str()),_=>None})}

    pub const CODE_FILTER_PRIMITIVE: &str = "stdio.svg.basic.blocklisted-filter-primitive";
    pub const CODE_CLIP_PATH_TEXT: &str = "stdio.svg.basic.clip-path-text";
    pub const CODE_BASE_PROFILE: &str = "stdio.svg.basic.base-profile";
    pub const CODE_NESTED_SVG: &str = "stdio.svg.basic.nested-svg";

    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: semio_framework_diagnostic::FaultScope::default() }
    }

    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: semio_framework_diagnostic::FaultScope::default() }
    }





    fn clip_path_children_by_id<'a>(elements: &[(&'a str, &'a [SvgAttr], &'a [SvgNode])]) -> HashMap<&'a str, &'a [SvgNode]> {
        elements.iter().filter(|(name, ..)| local_name(name) == "clipPath").filter_map(|(_, attrs, children)| attr_val(attrs, "id").map(|id| (id, *children))).collect()
    }



    /// 🛡️ Traverses owned SVG values under semantic progress.
    pub fn check_svg_basic_conformance_with(snapshot:&SvgSnapshot,control:&mut dyn FnMut(usize,usize)->Result<(),ValueError>)->Result<Vec<Diagnostic>,ValueError>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        fn tick(control:&mut dyn FnMut(usize,usize)->Result<(),ValueError>,count:&mut usize)->Result<(),ValueError>{*count=count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"SVG Basic validation unit overflow"))?;if *count%256==0{control(*count,0)?;}Ok(())}
        let mut out=Vec::new();let mut count=0usize;control(0,0)?;let Some(root)=&snapshot.doc.root else{return Ok(out)};let mut elements=Vec::new();let mut pending=vec![std::slice::from_ref(root).iter()];while let Some(nodes)=pending.last_mut(){let Some(node)=nodes.next()else{pending.pop();continue;};tick(control,&mut count)?;if let SvgNode::Element{name,attrs,children}=node{for _ in attrs{tick(control,&mut count)?;}elements.push((name.as_str(),attrs.as_slice(),children.as_slice()));pending.push(children.iter());}}
        let clip_paths=clip_path_children_by_id(&elements);
        for(name,attrs,_children)in &elements{tick(control,&mut count)?;if BLOCKED_FILTER_PRIMITIVES.contains(&local_name(name)){out.push(hard(CODE_FILTER_PRIMITIVE,format!("element <{name}> is an expensive raster filter primitive not supported by SVG Basic 1.1")));}if let Some(id)=clip_path_ref_id(attrs){if let Some(children)=clip_paths.get(id){let mut contains_text=false;let mut pending=vec![children.iter()];while let Some(nodes)=pending.last_mut(){let Some(node)=nodes.next()else{pending.pop();continue;};tick(control,&mut count)?;if let SvgNode::Element{name,children,..}=node{if TEXT_ELEMENTS.contains(&local_name(name)){contains_text=true;break;}pending.push(children.iter());}}if contains_text{out.push(hard(CODE_CLIP_PATH_TEXT,format!("<{name}> clip-path reference #{id} targets clipPath #{id}, which contains a text descendant -- SVG Basic 1.1 forbids clipping to text")));}}}}
        for(name,..)in elements.iter().skip(1){tick(control,&mut count)?;if local_name(name)=="svg"{out.push(soft(CODE_NESTED_SVG,format!("nested <{name}> element found below the document root -- review its viewport/clipping behavior on constrained renderers")));}}
        if let SvgNode::Element{name,attrs,..}=root{let base_profile_ok=attrs.iter().any(|a|a.name=="baseProfile"&&a.value.text()==Some("basic"));let version_ok=attrs.iter().any(|a|a.name=="version"&&a.value.text()==Some("1.1"));if !base_profile_ok||!version_ok{out.push(soft(CODE_BASE_PROFILE,format!("root <{name}> is missing baseProfile=\"basic\"/version=\"1.1\" -- SVG Basic 1.1 documents should declare their profile")));}}
        control(count,count)?;Ok(out)
    }

    /// ✅️ Checks the owned profile without a physical codec.
    pub fn check_svg_basic_conformance(snapshot:&SvgSnapshot)->Vec<Diagnostic>{check_svg_basic_conformance_with(snapshot,&mut |_,_|Ok(())).expect("owned SVG validation count")}


