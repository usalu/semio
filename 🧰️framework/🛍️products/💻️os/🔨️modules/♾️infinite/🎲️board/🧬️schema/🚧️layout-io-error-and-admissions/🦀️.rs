//! 🚧️ Layout IO error and admissions of the board schema: the layout IO error, the defaulted value and the force, tree, DAG and redraw option admissions. See [`super`].

use super::*;

#[derive(Debug)]
pub enum LayoutIoError {Admission(ValueError),Layout(LayoutError)}

impl std::fmt::Display for LayoutIoError {fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Admission(e)=>write!(f,"{e}"),Self::Layout(e)=>write!(f,"{e}")}}}

impl std::error::Error for LayoutIoError {}

impl From<ValueError> for LayoutIoError{fn from(e:ValueError)->Self{Self::Admission(e)}}

impl From<LayoutError> for LayoutIoError{fn from(e:LayoutError)->Self{Self::Layout(e)}}

struct Defaulted<T>(Option<T>);

impl<T> Default for Defaulted<T>{fn default()->Self{Self(None)}}

impl<T> Defaulted<T>{pub fn unwrap_or(self,value:T)->T{self.0.unwrap_or(value)}}

impl<T:semio_framework_value::FromValue> semio_framework_value::FromValue for Defaulted<T>{
 fn from_value(value:semio_framework_value::DslValue)->Result<Self,ValueError>{T::from_value(value).map(|value|Self(Some(value)))}
 fn from_value_controlled(value:&semio_framework_value::DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{T::from_value_controlled(value,control).map(|value|Self(Some(value)))}
 fn default_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;Ok(Self(None))}
 fn retire_decoded(self){if let Some(value)=self.0{T::retire_decoded(value)}}
}

#[derive(FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ForceAdmission {
 #[value(default)]
 pub(super) iterations:Defaulted<u32>,
 #[value(default)]
 pub(super) ideal_edge_length:Defaulted<f64>,
 #[value(default)]
 pub(super) repulsion_strength:Defaulted<f64>,
 #[value(default)]
 pub(super) spring_strength:Defaulted<f64>,
 #[value(default)]
 pub(super) gravity:Defaulted<f64>,
 #[value(default)]
 pub center_x:Option<f64>,
 #[value(default)]
 pub center_y:Option<f64>,
 #[value(default)]
 pub(super) time_step:Defaulted<f64>,
 #[value(default)]
 pub(super) velocity_damping:Defaulted<f64>,
 #[value(default)]
 pub(super) max_speed:Defaulted<f64>,
 #[value(default)]
 pub random_seed:Defaulted<u64>,
 #[value(default)]
 pub(super) barnes_hut_theta:Defaulted<f64>,
 #[value(default)]
 pub(super) pairwise_repulsion_max_bodies:Defaulted<u32>,
 #[value(default)]
 pub locked_node_ids:Defaulted<Vec<String>>,
}

impl ForceAdmission {pub fn admit(self,control:&mut NativeDecodeControl<'_>)->Result<ForceGraphLayoutOptions,ValueError>{
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
 pub layer_spacing:Defaulted<f64>,
 #[value(default)]
 pub sibling_gap:Defaulted<f64>,
 #[value(default)]
 pub direction:Defaulted<String>,
 #[value(default)]
 pub center_x:Option<f64>,
 #[value(default)]
 pub center_y:Option<f64>,
 #[value(default)]
 pub locked_node_ids:Defaulted<Vec<String>>,
}

impl TreeAdmission {pub fn admit(self,control:&mut NativeDecodeControl<'_>)->Result<HierarchicalTreeLayoutOptions,ValueError>{
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
 pub layer_spacing:Defaulted<f64>,
 #[value(default)]
 pub sibling_gap:Defaulted<f64>,
 #[value(default)]
 pub orientation:Defaulted<DagLayoutOrientation>,
 #[value(default)]
 pub center_x:Option<f64>,
 #[value(default)]
 pub center_y:Option<f64>,
}

impl DagAdmission {pub fn admit(self,control:&mut NativeDecodeControl<'_>)->Result<DagLayoutOptions,ValueError>{
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
 pub mode:String,
 #[value(default)] pub center_x:Option<f64>,
 #[value(default)] pub center_y:Option<f64>,
 #[value(default)] pub random_seed:Option<u64>,
 #[value(default)] pub redraw_handles_after:bool,
 #[value(default)] pub locked_node_ids:Vec<String>,
 #[value(default)] pub force_graph:Option<ForceAdmission>,
 #[value(default)] pub hierarchical_tree:Option<TreeAdmission>,
}

/// 📥️ Defaults are admitted in IO before publishing typed semantic options.
pub fn decode_redraw_options_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<RedrawLayoutOptions,ValueError>{let raw:RedrawAdmission=semio_framework_pack_json::from_json_str_controlled(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?;Ok(RedrawLayoutOptions{mode:raw.mode,center_x:raw.center_x,center_y:raw.center_y,random_seed:raw.random_seed,redraw_handles_after:raw.redraw_handles_after,locked_node_ids:raw.locked_node_ids,force_graph:raw.force_graph.map(|v|v.admit(control)).transpose()?,hierarchical_tree:raw.hierarchical_tree.map(|v|v.admit(control)).transpose()?})}

/// 📥️ Layered options use the same declared missing-field defaults under byte admission.
pub fn decode_dag_options_json(source:&str,control:&mut NativeDecodeControl<'_>)->Result<DagLayoutOptions,ValueError>{let raw:DagAdmission=semio_framework_pack_json::from_json_str_controlled(source,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?;raw.admit(control)}
