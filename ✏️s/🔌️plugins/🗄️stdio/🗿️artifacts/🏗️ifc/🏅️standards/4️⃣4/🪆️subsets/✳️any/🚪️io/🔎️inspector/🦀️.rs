//! 🪟️ IFC4 semantic inspector over shared typed details controls and framework interaction state.
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::{semantics::{self, SemanticField, SemanticFieldState, SemanticProjection}, snapshot::IfcSnapshot};
use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{self, IfcMutation};
use semio_s_artifact_stdio_contract::editing::{self, SnapshotDetailPathSegment as Segment, SnapshotDetailsProvider, SnapshotDetailValue, SnapshotDetailPresentation, SnapshotDetailItemCapabilities, SnapshotEditEvent};
use semio_framework_plugin::{app::InteractionView, plugin_app_close_prelude as ui, ActionId, Buildable, BuiltNode, HasBase, HasChildren, TreeWindows, UiAssemblyResult, UiMapBuilder, UiText, UiValue, WindowKindDefinition, WindowKit, TreeWindowKit};
use semio_framework_ui_locale::{Locale, LocalizedLabel};
use semio_framework_value::DslValue;

pub const BODY_KEY: &str = "s.stdio.ifc4.inspector";
pub fn window_definition(editable: bool) -> WindowKindDefinition {
    let mut definition = editing::snapshot_details_window_definition();
    definition.id = BODY_KEY.into(); definition.body_key = BODY_KEY.into(); definition.label = LocalizedLabel::native("IFC Inspector", "IFC-Inspektor"); definition.icon_id = "scan-search".into();
    if !editable { definition.actions.clear(); }
    definition
}
fn fault(reason: impl Into<String>) -> semio_framework_plugin::Fault { semio_framework_plugin::Fault::from(reason.into()) }
fn selected(interaction: &InteractionView<'_>) -> Option<u64> { interaction.selection(semantics::DOMAIN).ids.first().and_then(|id| semantics::parse_semantic_identity(id)) }
fn semantic_path(path: &str) -> Option<(u64, &str)> {
    let (identity, field) = path.strip_prefix('/')?.split_once('/')?;
    if field.is_empty() || field.contains('/') || field.contains('~') { return None; }
    Some((semantics::parse_semantic_identity(identity)?, field))
}
pub fn semantic_edit_controlled(event: &SnapshotEditEvent, snapshot: &IfcSnapshot,tick:&mut dyn FnMut()->Result<(),String>) -> Result<Option<Vec<IfcMutation>>, semio_framework_plugin::Fault> {
    let path = match event { SnapshotEditEvent::SetValue {path,..} | SnapshotEditEvent::RemoveValue {path} | SnapshotEditEvent::InsertValue {path,..} | SnapshotEditEvent::MoveValue {path,..} | SnapshotEditEvent::RenameKey {path,..} => path, SnapshotEditEvent::ReplaceSource {..} => return Ok(None) };
    if !path.starts_with("/ifc-") { return Ok(None); }
    let (id, field) = semantic_path(path).ok_or_else(|| fault("semantic-invalid-path"))?;
    let value = match event { SnapshotEditEvent::SetValue {value,..} => value, SnapshotEditEvent::RemoveValue {..} => &DslValue::Null, _ => return Err(fault("semantic-unsupported-edit")) };
    let edit = semantics::edit_semantic_controlled(snapshot,id,field,value,tick).map_err(fault)?;
    let mut current=None;for entity in &snapshot.entities{tick().map_err(fault)?;if entity.id==id{current=entity.args.get(edit.index);break;}}
    Ok(Some(if current == Some(&edit.value) {Vec::new()} else {vec![IfcMutation::SetEntityArg(mutations::set_entity_arg::SetEntityArg {id:edit.id,index:edit.index,value:edit.value})]}))
}
pub fn semantic_edit(event:&SnapshotEditEvent,snapshot:&IfcSnapshot)->Result<Option<Vec<IfcMutation>>,semio_framework_plugin::Fault>{semantic_edit_controlled(event,snapshot,&mut ||Ok(()))}
pub fn semantic_edit_admission(event: &SnapshotEditEvent, snapshot: &IfcSnapshot) -> Option<bool> {
    let path = match event { SnapshotEditEvent::SetValue {path,..} | SnapshotEditEvent::RemoveValue {path} | SnapshotEditEvent::InsertValue {path,..} | SnapshotEditEvent::MoveValue {path,..} | SnapshotEditEvent::RenameKey {path,..} => path, SnapshotEditEvent::ReplaceSource {..} => return None };
    if !path.starts_with("/ifc-"){return None;}
    let admitted=path.len()<=128&&semantic_path(path).is_some_and(|(_,field)|semantics::editable_field(field))&&match event{SnapshotEditEvent::SetValue{value:DslValue::String(value),..}=>value.len()<=semantics::MAXIMUM_TEXT_BYTES,SnapshotEditEvent::SetValue{value:DslValue::Null,..}|SnapshotEditEvent::RemoveValue{..}=>true,_=>false};
    let _=snapshot;
    Some(admitted)
}
struct Provider<'a> {projection: &'a SemanticProjection, editable: bool}
impl Provider<'_> {
    fn field(&self,path:&[Segment])->Option<&SemanticField> {let [Segment::Key(identity),Segment::Key(key)]=path else{return None};if identity!=&semantics::semantic_identity(self.projection.id){return None;}self.projection.fields.iter().find(|field|field.descriptor.id==*key)}
}
impl SnapshotDetailsProvider for Provider<'_> {
    fn value(&self,path:&[Segment])->Option<SnapshotDetailValue> {
        match path {[]=>Some(SnapshotDetailValue::Object),[Segment::Key(identity)] if identity==&semantics::semantic_identity(self.projection.id)=>Some(SnapshotDetailValue::Object),_=>self.field(path).map(|field|field.value.clone().map_or(SnapshotDetailValue::Null,SnapshotDetailValue::String))}
    }
    fn child_count(&self,path:&[Segment])->usize {match path {[]=>1,[Segment::Key(identity)] if identity==&semantics::semantic_identity(self.projection.id)=>self.projection.fields.len(),_=>0}}
    fn object_key(&self,path:&[Segment],index:usize)->Option<String> {match path {[] if index==0=>Some(semantics::semantic_identity(self.projection.id)),[Segment::Key(identity)] if identity==&semantics::semantic_identity(self.projection.id)=>self.projection.fields.get(index).map(|field|field.descriptor.id.clone()),_=>None}}
    fn presentation(&self,path:&[Segment],locale:Locale)->Option<SnapshotDetailPresentation> {
        if let [Segment::Key(identity)]=path {return Some(SnapshotDetailPresentation{label:format!("{} #{}",self.projection.entity_type,self.projection.id),description:Some(identity.clone())});}
        let field=self.field(path)?;let label=match locale{Locale::En=>&field.descriptor.label_en,Locale::De=>&field.descriptor.label_de};
        let status=match (locale,field.state,field.descriptor.editable){(Locale::En,SemanticFieldState::TooLarge,_)=>"Open full Details to edit this long value",(Locale::De,SemanticFieldState::TooLarge,_)=>"Langen Wert in vollständigen Details bearbeiten",(Locale::En,SemanticFieldState::Invalid,_)=>"Value does not match this IFC attribute",(Locale::De,SemanticFieldState::Invalid,_)=>"Wert entspricht nicht diesem IFC-Attribut",(Locale::En,_,false)=>"Read only; follow the reference below",(Locale::De,_,false)=>"Schreibgeschützt; Referenz unten folgen",(Locale::En,_,true)=>"Editable IFC attribute",(Locale::De,_,true)=>"Bearbeitbares IFC-Attribut"};
        Some(SnapshotDetailPresentation{label:label.clone(),description:Some(status.into())})
    }
    fn item_capabilities(&self,path:&[Segment])->SnapshotDetailItemCapabilities {let field=self.field(path);let allowed=field.is_some_and(|field|self.editable&&field.descriptor.editable&&matches!(field.state,SemanticFieldState::Valid|SemanticFieldState::Unset));SnapshotDetailItemCapabilities{edit:allowed,rename:false,remove:allowed&&field.is_some_and(|field|field.descriptor.optional&&field.state!=SemanticFieldState::Unset),reorder:false}}
    fn allows_untyped_creation(&self,_path:&[Segment])->bool {false}
    fn allows_collection_insert(&self,_path:&[Segment])->bool {false}
    fn creation_template(&self,path:&[Segment],collection_item:bool)->Option<DslValue> {(!collection_item&&self.field(path).is_some_and(|field|field.descriptor.editable&&field.descriptor.optional)).then(||DslValue::String(String::new()))}
}
fn ui_error(code:&'static str)->semio_framework_plugin::PluginAssemblyError {semio_framework_plugin::PluginAssemblyError::new(code,"IFC semantic inspector admission failed")}
fn message(value:&str)->UiAssemblyResult<BuiltNode>{ui::text(ui::Label(UiText::clipped(value))).try_build().map_err(|_|ui_error("ifc.inspector.message"))}
fn text(value:&str)->UiAssemblyResult<UiText>{UiText::try_from_str(value).ok_or_else(||ui_error("ifc.inspector.text"))}
fn navigation_button(id:u64,controller:&str,locale:Locale,prefix:&str)->UiAssemblyResult<BuiltNode>{
    let targets=semio_framework_pack_json::to_string(&semio_framework_pack_json::json!([{ "granularity":semantics::GRANULARITY,"id":semantics::semantic_identity(id)}]));
    let mut args=UiMapBuilder::try_new().ok_or_else(||ui_error("ifc.inspector.args"))?;
    for (key,value) in [("domainId",semantics::DOMAIN),("targets",targets.as_str()),("merge","replace"),("method","pick")]{args.try_insert(key.to_string(),UiValue::Text(text(value)?)).map_err(|_|ui_error("ifc.inspector.arg"))?;}
    let label=match locale{Locale::En=>format!("Open #{id}"),Locale::De=>format!("#{id} öffnen")};
    ui::button(ui::Label(text(label.as_str())?)).try_id(format!("ifc-{prefix}-{id}")).map_err(|_|ui_error("ifc.inspector.link-id"))?.try_on_with(ui::Trigger::Activate,ActionId::try_v1(controller,semio_framework_plugin::INTERACTION_SELECT_ACTION_ID).ok_or_else(||ui_error("ifc.inspector.link-action"))?,UiValue::Map(args.finish())).map_err(|_|ui_error("ifc.inspector.link-binding"))?.try_build().map_err(|_|ui_error("ifc.inspector.link"))
}
pub fn render(doc:&semio_framework_plugin::ArtifactView<'_,IfcSnapshot>,interaction:&InteractionView<'_>,view:&semio_framework_plugin::ViewModel,controller:&str,editable:bool,cx:&mut semio_framework_job::StepContext<'_>)->UiAssemblyResult<BuiltNode>{
    let Some(id)=selected(interaction) else{return message(match view.locale{Locale::En=>"Select an IFC product to inspect its attributes",Locale::De=>"IFC-Produkt auswählen, um Attribute zu prüfen"});};
    let _=cx.set_stage("ifc-inspector");
    let projection=semantics::project_semantics(doc.snapshot,id,&mut ||{if cx.is_cancelled(){return Err("cancelled".into());}if cx.should_yield(){return Err("limited".into());}cx.consume_fuel(1);Ok(())});
    if cx.is_cancelled(){return message(match view.locale{Locale::En=>"Inspection cancelled",Locale::De=>"Inspektion abgebrochen"});}if cx.should_yield(){return message(match view.locale{Locale::En=>"Inspection reached its request limit",Locale::De=>"Inspektion hat ihr Anforderungslimit erreicht"});}
    let projection=match projection{Ok(projection)=>projection,Err(reason)=>return message(match(view.locale,reason.as_str()){(Locale::En,"semantic-entity-missing")=>"The selected entity no longer exists",(Locale::De,"semantic-entity-missing")=>"Die ausgewählte Entität existiert nicht mehr",(Locale::En,"cancelled")=>"Inspection cancelled",(Locale::De,"cancelled")=>"Inspektion abgebrochen",(Locale::En,"limited")=>"Inspection reached its request limit",(Locale::De,"limited")=>"Inspektion hat ihr Anforderungslimit erreicht",(Locale::En,_)=>"The selected entity cannot be inspected",(Locale::De,_)=>"Die ausgewählte Entität kann nicht geprüft werden"})};
    let windows=TreeWindows::for_body(view,BODY_KEY);let revision=semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?;
    let details=editing::render_snapshot_details_provider_revisioned(&Provider{projection:&projection,editable},revision,view.locale,controller,&windows)?;
    let mut children=vec![details];
    for (prefix,ids,en,de) in [("forward",&projection.forward,"References","Referenzen"),("back",&projection.back,"Referenced By","Referenziert von")]{
        let heading=match view.locale{Locale::En=>en,Locale::De=>de};
        children.push(semio_framework_plugin::tree_window_indexed_section(&windows,&format!("ifc-{prefix}"),ui::Label(text(heading)?),true,ids.len(),|index|navigation_button(ids[index],controller,view.locale,prefix))?);
    }
    for id in &projection.missing{children.push(message(&match view.locale{Locale::En=>format!("Missing reference #{id}"),Locale::De=>format!("Fehlende Referenz #{id}")})?);}
    ui::column().try_id("ifc-semantic-inspector").map_err(|_|ui_error("ifc.inspector.id"))?.try_children(children).map_err(|_|ui_error("ifc.inspector.children"))?.try_build().map_err(|_|ui_error("ifc.inspector.root"))
}

pub fn layout(main_window:&str,editable:bool)->semio_framework_plugin::WindowLayout {
    let mut layout=editing::snapshot_details_split_layout(main_window,"Main");
    if let semio_framework_plugin::WindowLayoutRoot::Axis(axis)=&mut layout.root {
        if let Some(semio_framework_plugin::WindowLayoutChild::Stack(stack))=axis.children.last_mut(){
            if !editable{stack.children.clear();}
            stack.active_window_kind_id=Some(BODY_KEY.into());
            stack.children.insert(0,semio_framework_plugin::WindowLayoutWindowNode{kind:"window".into(),window_kind_id:BODY_KEY.into(),title:None,instance_id:None,template_id:None,corner:None});
        }
    }
    layout
}
pub fn topology(snapshot:&IfcSnapshot)->protocol::InteractionTopology {
    let ordered=snapshot.entities.iter().map(|entity|protocol::TopologyNode{id:semantics::semantic_identity(entity.id),granularity:semantics::GRANULARITY.into(),parent:None}).collect();
    let mut domains=std::collections::BTreeMap::new();domains.insert(semantics::DOMAIN.into(),protocol::DomainTopology{ordered});protocol::InteractionTopology{domains}
}
pub fn install_interaction(definition:&mut semio_framework_plugin::AppDefinition){
    definition.interactions.push(semio_framework_plugin::InteractionDefinition{id:semantics::DOMAIN.into(),label:LocalizedLabel::native("IFC Entities","IFC-Entitäten"),granularities:vec![semio_framework_plugin::GranularityDefinition{id:semantics::GRANULARITY.into(),label:LocalizedLabel::native("Entity","Entität"),icon_id:"box".into()}],hierarchy:semio_framework_plugin::HierarchyProvider::Topology,hover:semio_framework_plugin::HoverSpec::default(),selection:semio_framework_plugin::SelectionSpec{modes:vec![semio_framework_plugin::SelectionMode::Multiple,semio_framework_plugin::SelectionMode::Single],methods:vec![semio_framework_plugin::SelectionMethod::Pick],merges:vec![semio_framework_plugin::MergeMode::Replace,semio_framework_plugin::MergeMode::Invertive],transitive:false,broadcast:true}});
    for window in &mut definition.window_kinds {if window.id==BODY_KEY||window.id==semio_framework_plugin::MeshWindowKit::KIND_ID{window.interactions=vec![semio_framework_plugin::InteractionRef::new(semantics::DOMAIN)];}}
}
