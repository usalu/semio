//! 🧯️ The original Flow error lends its complete typed payload to the prepaid domain frame.
use super::{FlowCoreError,dag};
use dag::DagError;
use semio_framework_job::{OriginalDomainProjection,OriginalDomainFailure,OriginalDomainView as View};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress,Number};
fn absent()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Flow error projection ordinal is absent")}
impl FlowCoreError{
 fn original_variant(&self)->&'static str{match self{Self::Json(_)=>"json",Self::WidgetIdExists(_)=>"widgetIdExists",Self::UnknownWidget(_)=>"unknownWidget",Self::UnknownNeuronWidget(_)=>"unknownNeuronWidget",Self::NotVariadicInput(_)=>"notVariadicInput",Self::NotVariadicOutput(_)=>"notVariadicOutput",Self::NotNeuron(_)=>"notNeuron",Self::WidgetNotNeuron(_)=>"widgetNotNeuron",Self::MaxInputPortsReached(_)=>"maxInputPortsReached",Self::MaxOutputPortsReached(_)=>"maxOutputPortsReached",Self::UnknownInputPort(_)=>"unknownInputPort",Self::UnknownOutputPort(_)=>"unknownOutputPort",Self::NoOutputPort(_)=>"noOutputPort",Self::NoInputPort(_)=>"noInputPort",Self::UnknownSynapse(_)=>"unknownSynapse",Self::UnknownWidgetLayout(_)=>"unknownWidgetLayout",Self::UnknownCluster(_)=>"unknownCluster",Self::WidgetNotCluster(_)=>"widgetNotCluster",Self::UnknownSharedPayload(_)=>"unknownSharedPayload",Self::HistoryRefused(_)=>"historyRefused",Self::SelfConnection=>"selfConnection",Self::SelfInsertion=>"selfInsertion",Self::CycleWouldBeCreated=>"cycleWouldBeCreated",Self::ConnectionAlreadyExists=>"connectionAlreadyExists",Self::CollapseNeedsTwoWidgets=>"collapseNeedsTwoWidgets",Self::CollapseUnknownWidgets=>"collapseUnknownWidgets",Self::CollapseContainsClusters=>"collapseContainsClusters",Self::MinInputPorts{..}=>"minInputPorts",Self::MinOutputPorts{..}=>"minOutputPorts",Self::IncompatiblePortTypes{..}=>"incompatiblePortTypes",Self::Dag(_)=>"dag"}}
 fn original_text(&self)->Option<&str>{match self{Self::Json(value)|Self::WidgetIdExists(value)|Self::UnknownWidget(value)|Self::UnknownNeuronWidget(value)|Self::NotVariadicInput(value)|Self::NotVariadicOutput(value)|Self::NotNeuron(value)|Self::WidgetNotNeuron(value)|Self::MaxInputPortsReached(value)|Self::MaxOutputPortsReached(value)|Self::UnknownInputPort(value)|Self::UnknownOutputPort(value)|Self::NoOutputPort(value)|Self::NoInputPort(value)|Self::UnknownSynapse(value)|Self::UnknownWidgetLayout(value)|Self::UnknownCluster(value)|Self::WidgetNotCluster(value)|Self::UnknownSharedPayload(value)|Self::HistoryRefused(value)=>Some(value),_=>None}}
}
fn dag_variant(error:&DagError)->&'static str{match error{DagError::SnapshotRootNotObject=>"snapshotRootNotObject",DagError::SchemaMismatch=>"schemaMismatch",DagError::NodesMissing=>"nodesMissing",DagError::UnknownAlignMode(_)=>"unknownAlignMode",DagError::UnknownWidget(_)=>"unknownWidget",DagError::CanvasTheme(_)=>"canvasTheme",DagError::GridFactorOutOfRange=>"gridFactorOutOfRange",DagError::Json(_)=>"json"}}
fn dag_text(error:&DagError)->Option<&str>{match error{DagError::UnknownAlignMode(value)|DagError::UnknownWidget(value)|DagError::CanvasTheme(value)|DagError::Json(value)=>Some(value),_=>None}}
#[derive(Clone,Copy)]
enum Node<'a>{Flow(&'a FlowCoreError),Dag(&'a DagError),Text(&'a str),UInt(usize)}
impl<'a> Node<'a>{
 fn count(self)->Option<usize>{match self{Self::Flow(FlowCoreError::MinInputPorts{..}|FlowCoreError::MinOutputPorts{..})=>Some(3),Self::Flow(FlowCoreError::IncompatiblePortTypes{..})=>Some(5),Self::Flow(FlowCoreError::Dag(_))=>Some(2),Self::Flow(error)=>Some(1+usize::from(error.original_text().is_some())),Self::Dag(error)=>Some(1+usize::from(dag_text(error).is_some())),_=>None}}
 fn view(self)->Result<View<'a>,ValueError>{Ok(match self{Self::Text(value)=>View::IntrinsicText(value),Self::UInt(value)=>View::IntrinsicNumber(Number::UInt(u64::try_from(value).map_err(|_|absent())?)),node=>View::IntrinsicObject(node.count().ok_or_else(absent)?)})}
 fn key(self,index:usize)->Result<&'a str,ValueError>{
  if self.count().is_none_or(|length|index>=length){return Err(absent())}
  if index==0{return Ok("variant")}
  match self{
   Self::Flow(FlowCoreError::Dag(_))=>Ok("dag"),
   Self::Flow(FlowCoreError::MinInputPorts{..}|FlowCoreError::MinOutputPorts{..})=>["variant","widget","min"].get(index).copied().ok_or_else(absent),
   Self::Flow(FlowCoreError::IncompatiblePortTypes{..})=>["variant","source","sourceType","target","targetType"].get(index).copied().ok_or_else(absent),
   Self::Flow(_)|Self::Dag(_)=>Ok("value"),_=>Err(absent())
  }
 }
 fn child(self,index:usize)->Result<Self,ValueError>{
  if self.count().is_none_or(|length|index>=length){return Err(absent())}
  Ok(match self{
   Self::Flow(error) if index==0=>Self::Text(error.original_variant()),
   Self::Dag(error) if index==0=>Self::Text(dag_variant(error)),
   Self::Flow(FlowCoreError::Dag(error))=>Self::Dag(error),
   Self::Flow(FlowCoreError::MinInputPorts{widget,min}|FlowCoreError::MinOutputPorts{widget,min})=>if index==1{Self::Text(widget)}else{Self::UInt(*min)},
   Self::Flow(FlowCoreError::IncompatiblePortTypes{source,source_type,target,target_type})=>Self::Text([source.as_str(),source_type.as_str(),target.as_str(),target_type.as_str()][index-1]),
   Self::Flow(error)=>Self::Text(error.original_text().ok_or_else(absent)?),
   Self::Dag(error)=>Self::Text(dag_text(error).ok_or_else(absent)?),_=>return Err(absent())
  })
 }
}
fn locate<'a>(source:&'a FlowCoreError,path:&[usize])->Result<Node<'a>,ValueError>{if path.len()>2{return Err(absent())}let mut node=Node::Flow(source);for &index in path{node=node.child(index)?}Ok(node)}
impl OriginalDomainProjection for FlowCoreError{
 fn projection_identity(&self)->usize{self as*const Self as usize}
 fn projection_view(&self,path:&[usize])->Result<View<'_>,ValueError>{locate(self,path)?.view()}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{locate(self,path)?.key(index)}
}
impl OriginalDomainFailure for FlowCoreError{
 fn domain_code(&self)->&str{match self{Self::Json(_)=>"flow.json",Self::WidgetIdExists(_)=>"flow.widget-id-exists",Self::UnknownWidget(_)=>"flow.unknown-widget",Self::UnknownNeuronWidget(_)=>"flow.unknown-neuron-widget",Self::NotVariadicInput(_)=>"flow.not-variadic-input",Self::NotVariadicOutput(_)=>"flow.not-variadic-output",Self::NotNeuron(_)=>"flow.not-neuron",Self::WidgetNotNeuron(_)=>"flow.widget-not-neuron",Self::MaxInputPortsReached(_)=>"flow.max-input-ports-reached",Self::MaxOutputPortsReached(_)=>"flow.max-output-ports-reached",Self::UnknownInputPort(_)=>"flow.unknown-input-port",Self::UnknownOutputPort(_)=>"flow.unknown-output-port",Self::NoOutputPort(_)=>"flow.no-output-port",Self::NoInputPort(_)=>"flow.no-input-port",Self::UnknownSynapse(_)=>"flow.unknown-synapse",Self::UnknownWidgetLayout(_)=>"flow.unknown-widget-layout",Self::UnknownCluster(_)=>"flow.unknown-cluster",Self::WidgetNotCluster(_)=>"flow.widget-not-cluster",Self::UnknownSharedPayload(_)=>"flow.unknown-shared-payload",Self::HistoryRefused(_)=>"flow.history-refused",Self::SelfConnection=>"flow.self-connection",Self::SelfInsertion=>"flow.self-insertion",Self::CycleWouldBeCreated=>"flow.cycle-would-be-created",Self::ConnectionAlreadyExists=>"flow.connection-already-exists",Self::CollapseNeedsTwoWidgets=>"flow.collapse-needs-two-widgets",Self::CollapseUnknownWidgets=>"flow.collapse-unknown-widgets",Self::CollapseContainsClusters=>"flow.collapse-contains-clusters",Self::MinInputPorts{..}=>"flow.min-input-ports",Self::MinOutputPorts{..}=>"flow.min-output-ports",Self::IncompatiblePortTypes{..}=>"flow.incompatible-port-types",Self::Dag(_)=>"flow.dag"}}
 fn domain_message(&self)->&str{
  if let Some(value)=self.original_text(){return value}
  match self{
   Self::Dag(error)=>dag_text(error).unwrap_or_else(||match error{DagError::SnapshotRootNotObject=>"snapshot root must be object",DagError::SchemaMismatch=>"schema must be dag.hostDocument",DagError::NodesMissing=>"nodes array missing",DagError::GridFactorOutOfRange=>"gridFactor must be finite and in (0, 1e6]",_=>unreachable!()}),
   Self::MinInputPorts{widget,..}|Self::MinOutputPorts{widget,..}=>widget,
   Self::IncompatiblePortTypes{source,..}=>source,
   Self::SelfConnection=>"cannot connect widget to itself",Self::SelfInsertion=>"cannot insert widget between itself",Self::CycleWouldBeCreated=>"connection would create cycle",Self::ConnectionAlreadyExists=>"connection already exists",Self::CollapseNeedsTwoWidgets=>"select at least two widgets to collapse",Self::CollapseUnknownWidgets=>"selection contains unknown widgets",Self::CollapseContainsClusters=>"cannot collapse clusters",_=>unreachable!()
  }
 }
 fn domain_retained_progress(&self)->RetainedCloneProgress{Default::default()}
}
#[cfg(test)]
#[path="🧪️.rs"]
mod tests;
