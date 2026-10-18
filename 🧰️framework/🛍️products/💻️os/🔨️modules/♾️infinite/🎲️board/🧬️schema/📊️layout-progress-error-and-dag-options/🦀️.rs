//! 📊️ Layout progress, error and DAG options of the board schema: the layout progress, error and control and the DAG layout orientation and options. See [`super`].

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct LayoutProgress {pub completed:u64,pub limit:u64}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum LayoutError {Cancelled,WorkLimit,Schema(String),InvalidOptions(String)}

impl std::fmt::Display for LayoutError {fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {match self{Self::Schema(value)=>write!(f,"schema refused: {value}"),other=>write!(f,"{other:?}")}}}

impl std::error::Error for LayoutError {}

pub struct LayoutControl<'a>{pub limit:u64,pub completed:u64,pub progress:&'a mut dyn FnMut(LayoutProgress)->bool}

impl<'a> LayoutControl<'a>{
 pub fn new(limit:u64,progress:&'a mut dyn FnMut(LayoutProgress)->bool)->Self{Self{limit,completed:0,progress}}
 pub fn advance(&mut self,work:u64)->Result<(),LayoutError>{let next=self.completed.checked_add(work).filter(|next|*next<=self.limit).ok_or(LayoutError::WorkLimit)?;if !(self.progress)(LayoutProgress{completed:next,limit:self.limit}){return Err(LayoutError::Cancelled);}self.completed=next;Ok(())}
}

/// 🧭️ Tree layout flow direction for layered DAG positions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum DagLayoutOrientation {
    #[default]
    LeftRight,
    TopBottom,
}

/// 🌲️ Layered DAG layout options for snapshot JSON. `ToValue`/`FromValue` added
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass) so
/// `FlowHost::reorganize` can route through `pack::json::from_json_str` instead of
/// `serde_json::from_str` — `FlowCoreError` only has `From<pack::json::JsonError>`, not
/// `From<serde_json::Error>` (see its own docstring), so the old call never actually compiled
/// with the `?` operator once that conversion impl was dropped.
#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase",deny_unknown_fields)]
pub struct DagLayoutOptions {
    #[value(default = "default_dag_layer_spacing")]
    pub layer_spacing: f64,
    #[value(default = "default_dag_sibling_gap")]
    pub sibling_gap: f64,
    #[value(default)]
    pub orientation: DagLayoutOrientation,
    #[value(default)]
    pub center_x: Option<f64>,
    #[value(default)]
    pub center_y: Option<f64>,
}

fn default_dag_layer_spacing() -> f64 {
    120.0
}

fn default_dag_sibling_gap() -> f64 {
    40.0
}

impl Default for DagLayoutOptions {
    fn default() -> Self {
        Self { layer_spacing: default_dag_layer_spacing(), sibling_gap: default_dag_sibling_gap(), orientation: DagLayoutOrientation::default(), center_x: None, center_y: None }
    }
}
