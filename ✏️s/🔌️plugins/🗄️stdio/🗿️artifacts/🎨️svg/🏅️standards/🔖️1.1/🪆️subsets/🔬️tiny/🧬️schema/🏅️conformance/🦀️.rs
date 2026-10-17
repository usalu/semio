//! 🛡️ Owned SVG profile validation with caller supplied progress.
use crate::schema::snapshot::{SvgSnapshot,SvgAttr,SvgNode};
use semio_framework_diagnostic::{Diagnostic,FaultCode,Severity,TextSpan};
use semio_framework_value::ValueError;
    const BLOCKED_ELEMENTS: &[&str] = &["style", "script", "symbol", "marker", "clipPath", "mask", "pattern", "linearGradient", "radialGradient", "stop", "filter", "cursor", "textPath", "tspan", "tref", "view"];

    const BLOCKED_ATTRS: &[&str] = &["style", "opacity", "fill-opacity", "stroke-opacity", "clip-path", "mask", "filter"];

    fn local_name(name: &str) -> &str {
        name.rsplit(':').next().unwrap_or(name)
    }

    fn is_blocked_element(name: &str) -> bool {
        let ln = local_name(name);
        BLOCKED_ELEMENTS.contains(&ln) || ln.starts_with("fe")
    }

    fn is_external_href(value: &str) -> bool {
        let v = value.trim();
        !v.starts_with('#') && (v.contains("://") || v.starts_with("//"))
    }

    pub const CODE_ELEMENT: &str = "stdio.svg.tiny.blocklisted-element";
    pub const CODE_ATTRIBUTE: &str = "stdio.svg.tiny.blocklisted-attribute";
    pub const CODE_BASE_PROFILE: &str = "stdio.svg.tiny.base-profile";
    pub const CODE_EXTERNAL_HREF: &str = "stdio.svg.tiny.external-href";

    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: semio_framework_diagnostic::FaultScope::default() }
    }

    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: semio_framework_diagnostic::FaultScope::default() }
    }



    fn root_attrs(root: &SvgNode) -> &[SvgAttr] {
        match root {
            SvgNode::Element { attrs, .. } => attrs.as_slice(),
            _ => &[],
        }
    }

    /// 🛡️ Traverses owned SVG values under semantic progress.
    pub fn check_svg_tiny_conformance_with(snapshot:&SvgSnapshot,control:&mut dyn FnMut(usize,usize)->Result<(),ValueError>)->Result<Vec<Diagnostic>,ValueError>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        let mut out=Vec::new();let mut count=0usize;control(0,0)?;let Some(root)=&snapshot.doc.root else{return Ok(out)};
        let mut pending=vec![std::slice::from_ref(root).iter()];while let Some(nodes)=pending.last_mut(){let Some(node)=nodes.next()else{pending.pop();continue;};count=count.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "SVG validation count overflow"))?;if count%256==0{control(count,0)?;}
            if let SvgNode::Element{name,attrs,children}=node{if is_blocked_element(name){out.push(hard(CODE_ELEMENT,format!("element <{name}> is outside SVG Tiny 1.1's vocabulary -- REC-SVGMobile-20030114 excludes it")));}for a in attrs{count=count.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "SVG validation count overflow"))?;if count%256==0||a.value.owned_size()>65536||a.name.len()>65536{control(count,0)?;}let ln=local_name(&a.name);if BLOCKED_ATTRS.contains(&ln){out.push(hard(CODE_ATTRIBUTE,format!("attribute '{}' on <{name}> is forbidden anywhere in SVG Tiny 1.1",a.name)));}if ln=="href"&&a.value.text().is_some_and(is_external_href){out.push(soft(CODE_EXTERNAL_HREF,format!("<{name}> {}=\"{}\" looks like an external document reference -- SVG Tiny 1.1 restricts references to the same document",a.name,a.value.text().unwrap_or(""))));}}pending.push(children.iter());}
        }
        if let SvgNode::Element{name,..}=root{let attrs=root_attrs(root);let base_profile_ok=attrs.iter().any(|a|a.name=="baseProfile"&&a.value.text()==Some("tiny"));let version_ok=attrs.iter().any(|a|a.name=="version"&&a.value.text()==Some("1.1"));if !base_profile_ok||!version_ok{out.push(soft(CODE_BASE_PROFILE,format!("root <{name}> is missing baseProfile=\"tiny\"/version=\"1.1\" -- SVG Tiny 1.1 documents should declare their profile")));}}
        control(count,count)?;Ok(out)
    }

    /// ✅️ Checks the owned profile without a physical codec.
    pub fn check_svg_tiny_conformance(snapshot:&SvgSnapshot)->Vec<Diagnostic>{check_svg_tiny_conformance_with(snapshot,&mut |_,_|Ok(())).expect("owned SVG validation count")}
    /// 🚦️ Binds physical SQLite progress to pure owned profile validation.
    pub fn check_svg_tiny_conformance_controlled(snapshot:&SvgSnapshot,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<Vec<Diagnostic>,semio_framework_value::ValueError>{check_svg_tiny_conformance_with(snapshot,&mut |completed,total|control.checkpoint(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot,completed,total))}



//#region 🧩️ArtifactItems
pub mod artifact_items {
/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_SVG_DOCUMENT_SCHEMA: &str = "stdio.svg";
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
/// 🧬️ Artifact schema descriptor id.
pub const SVG_ARTIFACT_SCHEMA_ID: &str = "s.stdio.svg";
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("../../../../../../📜️artifact-definition.json");
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{STDIO_SVG_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.svg".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Svg", "Svg"),
        source_format: STDIO_SVG_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Document },
        schema: STDIO_SVG_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems4

//#region 🧩️ArtifactItems8
pub mod artifact_items_8 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}
}
//#endregion 🧩️ArtifactItems8
