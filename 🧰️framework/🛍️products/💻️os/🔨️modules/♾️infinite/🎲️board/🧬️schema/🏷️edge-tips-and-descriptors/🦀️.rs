//! 🏷️ Edge tips and descriptors of the board schema: the edge tip geometry, definitions and catalog, the built-in tips, the camera and node descriptors and the edge handle ids. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeTipGeometry {
    Arrow,
    FineArrow,
    Diamond,
    Circle,
    Bar,
}

#[derive(Clone, Debug)]
pub struct EdgeTipDef {
    pub geometry: EdgeTipGeometry,
    pub filled: bool,
    pub scale: f64,
}

/// 🔺️ Typed authored edge tip facts, independent of catalog serialization.
#[derive(Clone,Debug,semio_framework_value_derive::ToValue,semio_framework_value_derive::FromValue)]
pub struct EdgeTipCatalogEntry {pub id:String,#[value(default)]pub geometry:Option<EdgeTipGeometry>,#[value(default)]pub filled:Option<bool>,#[value(default)]pub scale:Option<f64>}

impl EdgeTipDef {
    pub fn from_catalog_entry(entry:&EdgeTipCatalogEntry)->Option<Self>{
        let Some(geometry)=entry.geometry else{return Self::builtin_for_id(&entry.id)};
        let filled=entry.filled.unwrap_or_else(||match geometry{EdgeTipGeometry::FineArrow|EdgeTipGeometry::Bar=>false,EdgeTipGeometry::Diamond=>!entry.id.contains("open"),_=>true});
        let scale=entry.scale.filter(|value|value.is_finite()&&*value>0.0).unwrap_or(1.0);
        Some(Self{geometry,filled,scale})
    }

    pub fn builtin_for_id(id: &str) -> Option<Self> {
        match id {
            "arrow" | "filled-arrow" => Some(Self { geometry: EdgeTipGeometry::Arrow, filled: true, scale: 1.0 }),
            "fine-arrow" => Some(Self { geometry: EdgeTipGeometry::FineArrow, filled: false, scale: 1.0 }),
            "filled-diamond" => Some(Self { geometry: EdgeTipGeometry::Diamond, filled: true, scale: 1.0 }),
            "open-diamond" => Some(Self { geometry: EdgeTipGeometry::Diamond, filled: false, scale: 1.0 }),
            _ => None,
        }
    }
}

pub fn builtin_edge_tips() -> BTreeMap<String, EdgeTipDef> {
    let ids = ["arrow", "filled-arrow", "fine-arrow", "filled-diamond", "open-diamond"];
    let mut m = BTreeMap::new();
    for id in ids {
        if let Some(def) = EdgeTipDef::builtin_for_id(id) {
            m.insert(id.to_string(), def);
        }
    }
    m
}

impl semio_framework_value::ToValue for EdgeTipGeometry{
fn to_value(&self)->semio_framework_value::DslValue{semio_framework_value::DslValue::String(match self{Self::Arrow=>"arrow",Self::FineArrow=>"fine-arrow",Self::Diamond=>"diamond",Self::Circle=>"circle",Self::Bar=>"bar"}.into())}
fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_value::DslValue,semio_framework_value::ValueError>{Ok(semio_framework_value::DslValue::String(control.copy_text(match self{Self::Arrow=>"arrow",Self::FineArrow=>"fine-arrow",Self::Diamond=>"diamond",Self::Circle=>"circle",Self::Bar=>"bar"})?))}}

impl semio_framework_value::FromValue for EdgeTipGeometry{
fn from_value(value:semio_framework_value::DslValue)->Result<Self,semio_framework_value::ValueError>{let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(usize::MAX,&mut accepted);Self::from_value_controlled(&value,&mut control)}
fn from_value_controlled(value:&semio_framework_value::DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{control.step()?;match value.as_str(){Some("arrow")=>Ok(Self::Arrow),Some("fine-arrow")=>Ok(Self::FineArrow),Some("diamond")=>Ok(Self::Diamond),Some("circle")=>Ok(Self::Circle),Some("bar")=>Ok(Self::Bar),_=>Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"Unknown edge tip geometry"))}}}

#[derive(Clone, Debug, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct CameraDescriptor {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

/// 🌱️ Owns semantic descriptor values independently of serialized syntax.
#[derive(Clone, Debug, Deserialize, Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct NodeDescriptor {
    pub id: String,
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    #[value(default)]
    pub draggable: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub selected: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub style: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub text: Option<String>,
    /// 🏷️ Runtime host encoding: catalog id from the baked icon table or inline SVG (`<?xml` / `<svg` …) parsed at detail LOD.
    #[serde(default)]
    #[value(default)]
    pub icon_kind: Option<String>,
    /// 🧩️ Semantic node-kind id for compatibility rows at `node` specificity.
    #[serde(default)]
    #[value(default)]
    pub node_kind: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub user_data: Option<semio_framework_value::DslValue>,
    #[serde(default)]
    #[value(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub locked: Option<bool>,
    #[serde(default)]
    #[value(default)]
    pub root: Option<bool>,
    #[value(default)]
    pub shape: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub radius: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub width: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub height: Option<f64>,
    #[serde(default)]
    #[value(default)]
    pub scale: Option<f64>,
}

/// 🔗️ Requires both semantic endpoint identities without serialized object lookup.
pub fn board_edge_handle_ids<'a>(source:Option<&'a str>,target:Option<&'a str>)->Option<(&'a str,&'a str)>{Some((source?,target?))}
