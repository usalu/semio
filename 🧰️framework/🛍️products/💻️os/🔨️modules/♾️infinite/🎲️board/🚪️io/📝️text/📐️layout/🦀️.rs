//! 🔤️ Physical layout JSON admission and emission with independently owned controls.
use crate::infinite::board::schema::{layout::*,layout_inferences::{force,redraw}};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueError};
#[derive(Debug)]
pub enum LayoutIoError {Admission(ValueError),Layout(LayoutError)}
impl std::fmt::Display for LayoutIoError {fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Admission(e)=>write!(f,"{e}"),Self::Layout(e)=>write!(f,"{e}")}}}
impl std::error::Error for LayoutIoError {}
impl From<ValueError> for LayoutIoError{fn from(e:ValueError)->Self{Self::Admission(e)}}
impl From<LayoutError> for LayoutIoError{fn from(e:LayoutError)->Self{Self::Layout(e)}}
/// 📸️ Both physical inputs are admitted before the pure redraw pipeline executes.
pub fn redraw_snapshot_json(source:&str,options:&str,decode:&mut NativeDecodeControl<'_>,work:&mut LayoutControl<'_>,encode:&mut NativeEncodeControl<'_>)->Result<String,LayoutIoError>{
 let mut snapshot=crate::infinite::board::io::text::snapshot::decode_board_snapshot_json(source,decode)?;
 let opts=decode_redraw_options_json(options,decode)?;
 redraw::redraw_layout(&mut snapshot,&opts,work)?;Ok(semio_framework_pack_json::to_json_string_controlled(&snapshot,encode)?)
}
/// ⚛️ Explicit physical force-only entry, with optional empty option text admitted as declared defaults.
pub fn force_snapshot_json(source:&str,options:&str,decode:&mut NativeDecodeControl<'_>,work:&mut LayoutControl<'_>,encode:&mut NativeEncodeControl<'_>)->Result<String,LayoutIoError>{
 let mut snapshot=crate::infinite::board::io::text::snapshot::decode_board_snapshot_json(source,decode)?;
 let opts:ForceGraphLayoutOptions=if options.trim().is_empty(){ForceGraphLayoutOptions::default()}else{{let admission:ForceAdmission=semio_framework_pack_json::from_json_str_controlled(options,semio_framework_pack_json::JsonMemberPolicy::Reject,decode)?;admission.admit(decode)?}};
 if snapshot.schema.as_str()=="board.ports.directed.v1"{force::apply_ported_force_layout(&mut snapshot,&opts,work)?;}else{force::apply_force_layout(&mut snapshot,&opts,work)?;}
 Ok(semio_framework_pack_json::to_json_string_controlled(&snapshot,encode)?)
}
/// 🔗️ Physical snap entry shares the exclusive snapshot admission contract.
pub fn snap_snapshot_json(source:&str,decode:&mut NativeDecodeControl<'_>,work:&mut LayoutControl<'_>,encode:&mut NativeEncodeControl<'_>)->Result<String,LayoutIoError>{let mut snapshot=crate::infinite::board::io::text::snapshot::decode_board_snapshot_json(source,decode)?;redraw::snap_edge_handles(&mut snapshot,work)?;Ok(semio_framework_pack_json::to_json_string_controlled(&snapshot,encode)?)}

use semio_framework_value_derive::FromValue;
struct Defaulted<T>(Option<T>);
impl<T> Default for Defaulted<T>{fn default()->Self{Self(None)}}
impl<T> Defaulted<T>{fn unwrap_or(self,value:T)->T{self.0.unwrap_or(value)}}
impl<T:semio_framework_value::FromValue> semio_framework_value::FromValue for Defaulted<T>{
 fn from_value(value:semio_framework_value::DslValue)->Result<Self,ValueError>{T::from_value(value).map(|value|Self(Some(value)))}
 fn from_value_controlled(value:&semio_framework_value::DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{T::from_value_controlled(value,control).map(|value|Self(Some(value)))}
 fn default_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;Ok(Self(None))}
 fn retire_decoded(self){if let Some(value)=self.0{T::retire_decoded(value)}}
}
#[derive(FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
struct ForceAdmission {
 #[value(default)]
 iterations:Defaulted<u32>,
 #[value(default)]
 ideal_edge_length:Defaulted<f64>,
 #[value(default)]
 repulsion_strength:Defaulted<f64>,
 #[value(default)]
 spring_strength:Defaulted<f64>,
 #[value(default)]
 gravity:Defaulted<f64>,
 #[value(default)]
 center_x:Option<f64>,
 #[value(default)]
 center_y:Option<f64>,
 #[value(default)]
 time_step:Defaulted<f64>,
 #[value(default)]
 velocity_damping:Defaulted<f64>,
 #[value(default)]
 max_speed:Defaulted<f64>,
 #[value(default)]
 random_seed:Defaulted<u64>,
 #[value(default)]
 barnes_hut_theta:Defaulted<f64>,
 #[value(default)]
 pairwise_repulsion_max_bodies:Defaulted<u32>,
 #[value(default)]
 locked_node_ids:Defaulted<Vec<String>>,
}
impl ForceAdmission {fn admit(self,control:&mut NativeDecodeControl<'_>)->Result<ForceGraphLayoutOptions,ValueError>{
 control.checkpoint()?;let defaults=ForceGraphLayoutOptions::default();Ok(ForceGraphLayoutOptions {
 iterations:self.iterations.unwrap_or(defaults.iterations),
 ideal_edge_length:self.ideal_edge_length.unwrap_or(defaults.ideal_edge_length),
 repulsion_strength:self.repulsion_strength.unwrap_or(defaults.repulsion_strength),
 spring_strength:self.spring_strength.unwrap_or(defaults.spring_strength),
 gravity:self.gravity.unwrap_or(defaults.gravity),
 center_x:self.center_x,
 center_y:self.center_y,
 time_step:self.time_step.unwrap_or(defaults.time_step),
 velocity_damping:self.velocity_damping.unwrap_or(defaults.velocity_damping),
 max_speed:self.max_speed.unwrap_or(defaults.max_speed),
 random_seed:self.random_seed.unwrap_or(defaults.random_seed),
 barnes_hut_theta:self.barnes_hut_theta.unwrap_or(defaults.barnes_hut_theta),
 pairwise_repulsion_max_bodies:self.pairwise_repulsion_max_bodies.unwrap_or(defaults.pairwise_repulsion_max_bodies),
 locked_node_ids:self.locked_node_ids.unwrap_or(defaults.locked_node_ids),
 })}}
#[derive(FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
struct TreeAdmission {
 #[value(default)]
 layer_spacing:Defaulted<f64>,
 #[value(default)]
 sibling_gap:Defaulted<f64>,
 #[value(default)]
 direction:Defaulted<String>,
 #[value(default)]
 center_x:Option<f64>,
 #[value(default)]
 center_y:Option<f64>,
 #[value(default)]
 locked_node_ids:Defaulted<Vec<String>>,
}
impl TreeAdmission {fn admit(self,control:&mut NativeDecodeControl<'_>)->Result<HierarchicalTreeLayoutOptions,ValueError>{
 let direction=match self.direction.0{Some(value)=>value,None=>{control.charge("downwards".len())?;control.checkpoint()?;default_direction()}};Ok(HierarchicalTreeLayoutOptions {
 layer_spacing:self.layer_spacing.unwrap_or(default_tree_layer_spacing()),
 sibling_gap:self.sibling_gap.unwrap_or(default_tree_sibling_gap()),
 direction:direction,
 center_x:self.center_x,
 center_y:self.center_y,
 locked_node_ids:self.locked_node_ids.unwrap_or(Vec::new()),
 })}}
#[derive(FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
struct DagAdmission {
 #[value(default)]
 layer_spacing:Defaulted<f64>,
 #[value(default)]
 sibling_gap:Defaulted<f64>,
 #[value(default)]
 orientation:Defaulted<DagLayoutOrientation>,
 #[value(default)]
 center_x:Option<f64>,
 #[value(default)]
 center_y:Option<f64>,
}
impl DagAdmission {fn admit(self,control:&mut NativeDecodeControl<'_>)->Result<DagLayoutOptions,ValueError>{
 control.checkpoint()?;let defaults=DagLayoutOptions::default();Ok(DagLayoutOptions {
 layer_spacing:self.layer_spacing.unwrap_or(defaults.layer_spacing),
 sibling_gap:self.sibling_gap.unwrap_or(defaults.sibling_gap),
 orientation:self.orientation.unwrap_or(defaults.orientation),
 center_x:self.center_x,
 center_y:self.center_y,
 })}}
#[derive(FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
struct RedrawAdmission {
 mode:String,
 #[value(default)] center_x:Option<f64>,
 #[value(default)] center_y:Option<f64>,
 #[value(default)] random_seed:Option<u64>,
 #[value(default)] redraw_handles_after:bool,
 #[value(default)] locked_node_ids:Vec<String>,
 #[value(default)] force_graph:Option<ForceAdmission>,
 #[value(default)] hierarchical_tree:Option<TreeAdmission>,
}
/// 📥️ Defaults are admitted in IO before publishing typed semantic options.
pub fn decode_redraw_options_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<RedrawLayoutOptions,ValueError>{let raw:RedrawAdmission=semio_framework_pack_json::from_json_str_controlled(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?;Ok(RedrawLayoutOptions{mode:raw.mode,center_x:raw.center_x,center_y:raw.center_y,random_seed:raw.random_seed,redraw_handles_after:raw.redraw_handles_after,locked_node_ids:raw.locked_node_ids,force_graph:raw.force_graph.map(|v|v.admit(control)).transpose()?,hierarchical_tree:raw.hierarchical_tree.map(|v|v.admit(control)).transpose()?})}
/// 📥️ Layered options use the same declared missing-field defaults under byte admission.
pub fn decode_dag_options_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<DagLayoutOptions,ValueError>{let raw:DagAdmission=semio_framework_pack_json::from_json_str_controlled(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?;raw.admit(control)}
