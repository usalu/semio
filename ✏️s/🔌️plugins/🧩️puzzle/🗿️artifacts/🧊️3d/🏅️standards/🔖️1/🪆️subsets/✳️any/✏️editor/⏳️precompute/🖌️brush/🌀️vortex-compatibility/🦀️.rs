//! 🌀️ Vortex compatibility of the brush: the vortex context, the port shape rules, the attraction compatibility and the stack pairs.

use super::*;

//#region 🔖️VortexContext
#[derive(Debug, Clone)]
pub struct AttractionVortexContext {
    pub object_kind: Option<String>,
    pub vortex_kind: Option<String>,
}

#[derive(Clone, value_derive::ToValue, value_derive::FromValue)]
pub struct BrushFillVortexTarget {
    pub full_id: String,
    pub object_id: String,
    pub object_kind: Option<String>,
    pub vortex_kind: Option<String>,
    pub vortex_index: usize,
}

pub trait BrushCatalogView {
    fn objects(&self) -> &[ObjectKind];
    fn vortices(&self) -> &[VortexKindCatalog];
    fn cables(&self) -> &[CableKindCatalog];
}

impl BrushCatalogView for KindCatalogBundle {
    fn objects(&self) -> &[ObjectKind] {
        &self.objects
    }

    fn vortices(&self) -> &[VortexKindCatalog] {
        &self.vortices
    }

    fn cables(&self) -> &[CableKindCatalog] {
        &self.cables
    }
}

pub trait BrushSceneView {
    fn object_count(&self) -> usize;
    fn find_object_kind(&self, kind_id: &str) -> Option<&EngineSceneObject>;
}

impl BrushSceneView for EngineSceneSnapshot {
    fn object_count(&self) -> usize {
        self.objects.len()
    }

    fn find_object_kind(&self, kind_id: &str) -> Option<&EngineSceneObject> {
        self.objects.iter().find(|object| object.object_kind.as_deref() == Some(kind_id))
    }
}
//#endregion 🔖️VortexContext

//#region 🔖️Compatibility
pub fn puzzle3d_vortex_port_shape(vortex_kind: &str) -> Option<&'static str> {
    if vortex_kind.contains(" circular ") {
        Some("circular")
    } else if vortex_kind.contains(" rectangular ") {
        Some("rectangular")
    } else {
        None
    }
}

pub fn puzzle3d_vortex_port_shapes_compatible(source: &str, target: &str) -> bool {
    match (puzzle3d_vortex_port_shape(source), puzzle3d_vortex_port_shape(target)) {
        (None, _) | (_, None) => true,
        (Some(a), Some(b)) => a == b,
    }
}

pub fn puzzle3d_single_letter_port_family(vortex_kind: &str) -> Option<char> {
    let head = vortex_kind.split('-').next()?;
    if head.len() == 1 {
        let ch = head.chars().next()?;
        if ch.is_ascii_lowercase() {
            return Some(ch);
        }
    }
    None
}

pub fn puzzle3d_single_letter_port_families_compatible(source: &str, target: &str) -> bool {
    match (puzzle3d_single_letter_port_family(source), puzzle3d_single_letter_port_family(target)) {
        (None, _) | (_, None) => true,
        (Some(a), Some(b)) => a == b,
    }
}

fn catalog_vortex_by_id<'a>(catalogs: &'a impl BrushCatalogView, vortex_kind: &str) -> Option<&'a VortexKindCatalog> {
    catalogs.vortices().iter().find(|v| v.id == vortex_kind)
}

fn catalog_cable_by_id<'a>(catalogs: &'a impl BrushCatalogView, cable_kind: &str) -> Option<&'a CableKindCatalog> {
    catalogs.cables().iter().find(|w| w.id == cable_kind)
}

pub fn resolve_cable_kind_for_vortex(vortex_kind: &str, catalogs: &impl BrushCatalogView) -> String {
    catalog_vortex_by_id(catalogs, vortex_kind).and_then(|v| v.default_cable_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| DEFAULT_CABLE_KIND_ID.to_string())
}

pub fn resolve_attraction_kind_for_cable(cable_kind: &str, catalogs: &impl BrushCatalogView) -> String {
    catalog_cable_by_id(catalogs, cable_kind).and_then(|c| c.default_attraction_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default()
}

pub fn compat_pair_matches(rule: &KindCompatEntry, a: &str, b: &str) -> bool {
    (rule.source == a && rule.target == b) || (rule.bidirectional && rule.source == b && rule.target == a)
}

pub fn specificity_rank(spec: Option<&str>) -> i32 {
    match spec {
        Some("general") => 0,
        Some("object") => 1,
        Some("attraction") => 2,
        Some("cable") => 3,
        Some("vortex") => 4,
        _ => 4,
    }
}

pub fn attraction_gesture_rule_applies(rule: &KindCompatEntry, attracting: &AttractionVortexContext, attracted: &AttractionVortexContext, catalogs: &impl BrushCatalogView) -> bool {
    let cable_src = resolve_cable_kind_for_vortex(attracting.vortex_kind.as_deref().unwrap_or(""), catalogs);
    let cable_tgt = resolve_cable_kind_for_vortex(attracted.vortex_kind.as_deref().unwrap_or(""), catalogs);
    let attraction_src = resolve_attraction_kind_for_cable(&cable_src, catalogs);
    let attraction_tgt = resolve_attraction_kind_for_cable(&cable_tgt, catalogs);
    let sn = attracting.object_kind.as_deref().unwrap_or("");
    let tn = attracted.object_kind.as_deref().unwrap_or("");
    let sv = attracting.vortex_kind.as_deref().unwrap_or("");
    let tv = attracted.vortex_kind.as_deref().unwrap_or("");
    match rule.specificity.as_deref().unwrap_or("vortex") {
        "general" => compat_pair_matches(rule, sv, tv),
        "object" => compat_pair_matches(rule, sn, tn),
        "attraction" => compat_pair_matches(rule, &attraction_src, &attraction_tgt),
        "vortex" => compat_pair_matches(rule, sv, tv),
        "cable" => compat_pair_matches(rule, &cable_src, &cable_tgt),
        _ => compat_pair_matches(rule, sv, tv),
    }
}

pub fn vortices_attraction_compatible_for_drag(attracting: &AttractionVortexContext, attracted: &AttractionVortexContext, rules: &[KindCompatEntry], catalogs: &impl BrushCatalogView) -> bool {
    let sv = attracting.vortex_kind.as_deref().unwrap_or("");
    let tv = attracted.vortex_kind.as_deref().unwrap_or("");
    if !puzzle3d_vortex_port_shapes_compatible(sv, tv) {
        return false;
    }
    if !puzzle3d_single_letter_port_families_compatible(sv, tv) {
        return false;
    }
    if rules.is_empty() {
        return true;
    }
    let mut matched: Vec<&KindCompatEntry> = rules.iter().filter(|r| attraction_gesture_rule_applies(r, attracting, attracted, catalogs)).collect();
    if matched.is_empty() {
        return false;
    }
    if matched.iter().any(|r| r.important) {
        matched.retain(|r| r.important);
    } else {
        let max_rank = matched.iter().map(|r| specificity_rank(r.specificity.as_deref())).max().unwrap_or(4);
        matched.retain(|r| specificity_rank(r.specificity.as_deref()) == max_rank);
    }
    !matched.is_empty()
}
//#endregion 🔖️Compatibility

//#region 🔖️StackPairs
pub fn brush_stack_vortex_base(vortex_kind: &str) -> Option<&str> {
    if let Some(base) = vortex_kind.strip_suffix(" bottom") {
        Some(base)
    } else if let Some(base) = vortex_kind.strip_suffix(" top") {
        Some(base)
    } else {
        None
    }
}

pub fn brush_stack_bottom_top_pair(source: &str, target: &str) -> bool {
    let (Some(sb), Some(tb)) = (brush_stack_vortex_base(source), brush_stack_vortex_base(target)) else {
        return false;
    };
    source.ends_with(" bottom") && target.ends_with(" top") && sb == tb
}

pub fn brush_stack_top_bottom_pair(source: &str, target: &str) -> bool {
    let (Some(sb), Some(tb)) = (brush_stack_vortex_base(source), brush_stack_vortex_base(target)) else {
        return false;
    };
    source.ends_with(" top") && target.ends_with(" bottom") && sb == tb
}

pub fn brush_stack_mate_pair(source: &str, target: &str) -> bool {
    if !puzzle3d_vortex_port_shapes_compatible(source, target) {
        return false;
    }
    brush_stack_bottom_top_pair(source, target) || brush_stack_top_bottom_pair(source, target)
}
//#endregion 🔖️StackPairs
