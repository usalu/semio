//! 🔖️ Graph kinds: node, handle and element vocabulary with the property-bag value bridges.

use super::*;

// #region 🔖️PropertyValue
/// 🧾️ Converts a fixture `userData` value into a typed property bag.
pub fn property_bag_from_value(value: &semio_framework_value::DslValue) -> PropertyBag {
    let semio_framework_value::DslValue::Object(entries) = value.clone() else {
        return PropertyBag::default();
    };
    entries.into_iter().filter_map(|(k, v)| semio_framework_value::FromValue::from_value(v).ok().map(|pv| (k, pv))).collect()
}

/// 🧾️ Serializes a property bag back to a value for fixture export.
pub fn property_bag_to_value(bag: &PropertyBag) -> Option<semio_framework_value::DslValue> {
    if bag.is_empty() {
        None
    } else {
        let entries: Vec<(String, semio_framework_value::DslValue)> = bag.iter().map(|(k, v)| (k.clone(), semio_framework_value::ToValue::to_value(v))).collect();
        Some(semio_framework_value::DslValue::Object(entries))
    }
}
// #endregion 🔖️PropertyValue

// #region 🔖️Kinds

/// 🌉️ `geometry::Point` bridge (`📐️geometry` is a different owner's module — RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// 26/09/01 splits this ticket by module, so `Point` itself is out of scope here); hand-written
/// rather than a derive since we cannot add `#[derive(ToValue, FromValue)]` to `Point`'s own
/// definition. `Node::center`/`Handle` below name this via `#[value(with = "point_bridge")]`.
mod point_bridge {
    pub fn to_value(p: &super::Point) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object([
            ("x".to_string(), semio_framework_value::ToValue::to_value(&p.x)),
            ("y".to_string(), semio_framework_value::ToValue::to_value(&p.y)),
        ])
    }
    pub fn from_value(value: semio_framework_value::DslValue) -> Result<super::Point, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(entries) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Point"));
        };
        let x = entries.iter().find(|(k, _)| k == "x").map(|(_, v)| v.clone()).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `x`"))?;
        let y = entries.iter().find(|(k, _)| k == "y").map(|(_, v)| v.clone()).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `y`"))?;
        Ok(super::Point {
            x: <f64 as semio_framework_value::FromValue>::from_value(x).map_err(|e| e.under("x"))?,
            y: <f64 as semio_framework_value::FromValue>::from_value(y).map_err(|e| e.under("y"))?,
        })
    }
}

/// 🔵️ Circle or axis-aligned rectangle node body.
// 🧬️ `semio_framework_value_derive::{ToValue, FromValue}` additive (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
// 26/09/01) — never had `serde`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum NodeShape {
    #[default]
    Circle,
    Rectangle,
}

/// 🪝️ Port direction for directed edge wiring.
// 🧬️ `semio_framework_value_derive::{ToValue, FromValue}` additive, see `NodeShape` above.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum HandleRole {
    Source,
    Target,
    #[default]
    Any,
}

/// 🏷️ Semantic kind and property payload shared by graph elements.
// 🧬️ `semio_framework_value_derive::{ToValue, FromValue}` additive, see `NodeShape` above; `PropertyBag` (`🛂️manifest`) already covered.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct ElementSemantics {
    pub kind: Option<String>,
    pub properties: PropertyBag,
}

/// 🟠️ Retained node state with world-space center and shape extents.
// 🧬️ `semio_framework_value_derive::{ToValue, FromValue}` additive, see `NodeShape` above; `center` bridges through
// `point_bridge` (`geometry::Point` is a different owner's module).
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct Node {
    pub id: NodeId,
    #[value(with = "point_bridge")]
    pub center: Point,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub shape: NodeShape,
    pub draggable: bool,
    pub kind: Option<String>,
    pub label: Option<String>,
    pub properties: PropertyBag,
}

/// 🟣️ Tangent handle anchored to a node at a polar angle.
// 🧬️ `semio_framework_value_derive::{ToValue, FromValue}` additive, see `Node` above.
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct Handle {
    pub angle: f64,
    pub id: HandleId,
    pub node_id: NodeId,
    pub radius: f64,
    pub role: HandleRole,
    pub kind: Option<String>,
    /// 🔤️ Value schemas this port declares — an output lists what it may carry, an input what it
    /// accepts, and a board refuses a wire whose two declared sets are disjoint. Empty is undeclared,
    /// which stays connectable.
    pub value_types: Vec<String>,
    pub properties: PropertyBag,
}

/// 🪢️ Retained edge with typed endpoints.
pub type GraphEdge<E> = CoreEdge<E>;
semio_framework_value::artifact_retire_leaf!(NodeShape,HandleRole);
semio_framework_value::artifact_retire_struct!(ElementSemantics {kind,properties});
semio_framework_value::artifact_retire_struct!(Handle {angle,id,node_id,radius,role,kind,value_types,properties});
impl<E:Copy+Send+'static> semio_framework_value::retirement::RetireOwned for CoreEdge<E>{
 type RetirementCursor=semio_framework_value::retirement::Leaf<Self>;
 fn retirement_seed(self)->Self::RetirementCursor{semio_framework_value::retirement::leaf(self)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value::retirement::leaf_birth_bytes::<Self>())}
 fn controlled_retirement_supported()->bool{true}
 fn retirement_element_copy_bytes()->usize{size_of::<Self>()}
}
/// ♻️ Mirrors a node with its foreign `Point` center as a plain coordinate pair so the node retires through the canonical record cursor.
#[doc(hidden)]
pub struct NodeParts{id:NodeId,center:(f64,f64),radius:f64,width:f64,height:f64,shape:NodeShape,draggable:bool,kind:Option<String>,label:Option<String>,properties:PropertyBag}
semio_framework_value::artifact_retire_struct!(NodeParts {id,center,radius,width,height,shape,draggable,kind,label,properties});
impl semio_framework_value::retirement::RetireOwned for Node{
 type RetirementCursor=<NodeParts as semio_framework_value::retirement::RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{semio_framework_value::retirement::RetireOwned::retirement_ready(&self.kind)&&semio_framework_value::retirement::RetireOwned::retirement_ready(&self.label)&&semio_framework_value::retirement::RetireOwned::retirement_ready(&self.properties)}
 fn retirement_seed(self)->Self::RetirementCursor{let Self{id,center,radius,width,height,shape,draggable,kind,label,properties}=self;semio_framework_value::retirement::RetireOwned::retirement_seed(NodeParts{id,center:(center.x,center.y),radius,width,height,shape,draggable,kind,label,properties})}
 fn controlled_retirement_supported()->bool{<NodeParts as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()}
}
// #endregion 🔖️Kinds
