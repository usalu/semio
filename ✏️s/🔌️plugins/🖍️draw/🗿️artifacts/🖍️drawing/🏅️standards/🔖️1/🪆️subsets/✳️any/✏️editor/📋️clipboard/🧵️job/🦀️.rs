//! 🧵️ Reserved clipboard verbs retain the original read, progressive codecs and one atomic publication.
use super::*;
use semio_framework_plugin::{ArtifactReservedToolJobRequest,ArtifactReservedToolInput,EditorApp,Emit,Effect,EphemeralEmit,ArtifactReservedJob};
use crate::host::outcome::{DrawingJobOutcomes,DrawingJobTurn};
use semio_framework_job::{InteractiveJob,InteractiveJobCloseStep,JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeView,StepContext};
use semio_framework_value::{DslValue,FromValue,ValueError,RetireOwned,NativeDecodeControl,native_decoding::NativeDecodeContinuation,NativeEncodeControl,NativeEncodeContinuation,retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneSource,RetainedCloneStep}};
use semio_framework_value::retirement::RetireOwned as RetireAuthority;
type App=EditorApp<super::super::DrawingPlayApp>;
type Clone<T>=crate::host::owned::DrawingNativeCloneAuthority<T>;
type NativeText=semio_framework_value::paged::PagedUtf8<{usize::MAX}>;
type NativeRetired=semio_framework_value::list::PagedList<NativeText,{MAX_NODES*MAX_NODES+MAX_TASKS}>;
struct ClipboardScene<'a>{packet:&'a DrawingClipboard,selected:&'a[String]}
impl crate::schema::scene_preparation::DrawingSceneSource for ClipboardScene<'_>{
 fn root_count(&self)->usize{self.packet.roots.len()}
 fn root(&self,index:usize)->Option<&DrawingLayerNode>{self.packet.roots.get(index)}
 fn asset_count(&self)->usize{self.packet.assets.len()}
 fn asset_at(&self,index:usize)->Option<(&dyn semio_framework_value::paged::Utf8Text,&DrawingImageAsset)>{self.packet.assets.entry_at(index).map(|(key,value)|(key as &dyn semio_framework_value::paged::Utf8Text,value))}
 fn asset(&self,key:&str)->Option<&DrawingImageAsset>{self.packet.assets.get(key)}
 fn root_visible(&self,index:usize)->bool{self.packet.roots.get(index).is_some_and(|node|self.selected.iter().any(|id|layer_base(node).id==*id))}
}
use crate::host::owned::{DrawingLayerAddress,DrawingLayerLocator,DrawingLayerCloneWorkAuthority};
#[path="📋️metadata/🦀️.rs"]
mod metadata;
struct TreeCursor{address:DrawingLayerAddress,children:[usize;MAX_DEPTH],base:usize,entered:bool,complete:bool,all_roots:bool}
impl TreeCursor{
    fn new(address:DrawingLayerAddress,all_roots:bool)->Self{Self{base:address.length,address,children:[0;MAX_DEPTH],entered:false,complete:false,all_roots}}
    fn step(&mut self,source:&DrawingSnapshot)->Result<Option<DrawingLayerAddress>,semio_framework::Fault>{self.step_with(source.layers.len(),|address|DrawingLayerLocator::node_at(source,address))}
    fn packet(&mut self,packet:&DrawingClipboard)->Result<Option<DrawingLayerAddress>,semio_framework::Fault>{self.step_with(packet.roots.len(),|address|packet_node(packet,address))}
    fn step_with<'a>(&mut self,root_count:usize,lookup:impl Fn(DrawingLayerAddress)->Option<&'a DrawingLayerNode>)->Result<Option<DrawingLayerAddress>,semio_framework::Fault>{
        if self.complete{return Ok(None);}if !self.entered{self.entered=true;return Ok(Some(self.address));}
        loop{
            let node=lookup(self.address).ok_or_else(||fault(refused()))?;
            let depth=self.address.length-1;
            if let DrawingLayerNode::Group(group)=node{let index=self.children[depth];if index<group.children.len(){if self.address.length==MAX_DEPTH{return Err(fault(refused()));}self.children[depth]+=1;self.address.indices[self.address.length]=index;self.address.length+=1;self.children[self.address.length-1]=0;return Ok(Some(self.address));}}
            if self.address.length>self.base{self.address.length-=1;continue;}
            if self.all_roots{let index=self.address.indices[0]+1;if index<root_count{self.address.indices[0]=index;self.children[0]=0;return Ok(Some(self.address));}}
            self.complete=true;return Ok(None);
        }
    }
}
const COPY_NATIVE_BYTES:usize=MAX_NODES*MAX_ID_BYTES*8+MAX_TASKS*std::mem::size_of::<usize>()*MAX_DEPTH;
pub(super) const MAX_NATIVE_BYTES:usize=COPY_NATIVE_BYTES*2+MAX_BYTES*std::mem::size_of::<DslValue>()*2;
/// 📖️ Borrows the sealed snapshot through its original clone source; the field path keeps sibling workspace fields borrowable.
macro_rules! source_doc{($workspace:expr)=>{$workspace.source.as_ref().expect("Drawing clipboard seals its source before any read").borrow().get()}}
#[derive(RetireOwned)]
struct Workspace{source:Option<RetainedCloneSource<DrawingSnapshot,store::SnapshotRead<DrawingSnapshot>>>,unsealed:Option<store::SnapshotRead<DrawingSnapshot>>,completion:Option<semio_framework_plugin::ArtifactToolCompletion<App>>,args:Option<DslValue>,interaction:protocol::InteractionState,hover:semio_framework_plugin::app::InteractionHoverState,wire:Vec<u8>,packet:Option<DrawingClipboard>,encoder:Option<semio_framework_pack_json::JsonWriteCursor<encode::ClipboardJsonSource>>,parser:Option<semio_framework_pack_json::JsonGrammarCursor<DslValue>>,hydration:Option<decode::ClipboardHydration>,placement:Option<crate::schema::scene_placement::DrawingScenePlacementJob>,text:String,buffer_admitted:usize,roots:Vec<String>,retired_ids:Vec<String>,selected:Vec<String>,assets:Vec<String>,mutations:Vec<DrawingMutation>,effects:Vec<Effect>,interaction_writes:Vec<semio_framework_plugin::InteractionWrite>,interaction_targets:Vec<protocol::InteractionTarget>,fragment:Option<ClipboardFragment>,pending_asset:Option<(String,DrawingImageAsset)>,rejected_completion:Option<super::super::completion::RejectedCompletion>,parent:Option<String>,addresses:metadata::Table<DrawingLayerAddress>,visited:metadata::Table<()>,removed:metadata::Table<()>,identities:metadata::Table<String>,asset_identities:metadata::Table<String>,cycle_active:metadata::Table<()>,cycle_done:metadata::Table<()>,cycle_stack:Vec<(DrawingLayerAddress,usize)>,pending_native:NativeText,rejected_native:Option<NativeText>,retired_native:NativeRetired,target:NativeText}
pub(crate) struct DrawingClipboardJob{owner:ControlledRetirement<Workspace>,outcomes:DrawingJobOutcomes,tool:String,layer_clone:Option<Clone<DrawingLayerNode>>,asset_clone:Option<Clone<DrawingImageAsset>>,encoding:Option<NativeEncodeContinuation>,decoding:Option<NativeDecodeContinuation>,placement:PastePlacement,phase:u8,at:usize,closing:bool,complete:bool,locator:Option<DrawingLayerLocator>,footprint:Option<DrawingLayerCloneWorkAuthority>,source_root:Option<DrawingLayerAddress>,path:[usize;MAX_DEPTH],children:[usize;MAX_DEPTH],depth:usize,entered:bool,reference:usize,admitted_bytes:usize,normalize_other:usize,asset_append:usize,tree:Option<TreeCursor>,asset_search:usize,located_asset:Option<usize>,ordinal:usize,destination:[f64;6],inverse:[f64;6],offset:[f64;2],insertion:usize,scratch:[u8;MAX_ID_BYTES],material:[u8;MAX_ID_BYTES*2+64],mapped:[u8;76]}
fn fault(error:impl std::fmt::Display)->semio_framework::Fault{semio_framework::Fault::from(error.to_string())}
fn refused()->ValueError{ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Invalid drawing clipboard input / Ungültige Zeichnungsdaten in der Zwischenablage")}
fn property<'a>(value:&'a mut DslValue,key:&str)->Option<&'a mut DslValue>{let DslValue::Object(entries)=value else{return None;};entries.iter_mut().find(|(name,_)|name==key).map(|(_,value)|value)}
fn world_at(source:&DrawingSnapshot,mut address:DrawingLayerAddress)->Result<[f64;6],semio_framework::Fault>{let mut matrix=[1.0,0.0,0.0,1.0,0.0,0.0];let length=address.length;for depth in 1..=length{address.length=depth;let node=DrawingLayerLocator::node_at(source,address).ok_or_else(||fault(refused()))?;matrix=crate::schema::geometry::multiply(matrix,crate::schema::drawing_transform_to_matrix(&layer_base(node).transform));}if !matrix.iter().all(|value|value.is_finite()){return Err(fault(refused()));}Ok(matrix)}
fn packet_node(packet:&DrawingClipboard,address:DrawingLayerAddress)->Option<&DrawingLayerNode>{let mut node=packet.roots.get(address.indices[0])?;for index in &address.indices[1..address.length]{let DrawingLayerNode::Group(group)=node else{return None;};node=group.children.get(*index)?;}Some(node)}
fn packet_node_mut(packet:&mut DrawingClipboard,address:DrawingLayerAddress)->Option<&mut DrawingLayerNode>{let mut node=packet.roots.get_mut(address.indices[0])?;for index in &address.indices[1..address.length]{let DrawingLayerNode::Group(group)=node else{return None;};node=group.children.get_mut(*index)?;}Some(node)}
fn replace_native(target:&mut NativeText,text:&str,pending:&mut NativeText,rejected:&mut Option<NativeText>,retired:&mut NativeRetired,admitted:&mut usize,cx:&mut StepContext<'_>)->Result<(),semio_framework::Fault>{
    if text.is_empty()||text.len()>MAX_ID_BYTES||!pending.is_empty()||rejected.is_some(){return Err(fault(refused()));}
    let mut accepted=|_|!cx.is_cancelled();let mut control=NativeDecodeControl::new(MAX_NATIVE_BYTES.checked_sub(*admitted).ok_or_else(||fault(refused()))?,&mut accepted);
    let result=(||{while !retired.has_reserved_slot(){let bytes=retired.next_allocation_bytes().map_err(fault)?;control.charge(bytes).map_err(fault)?;retired.reserve_one(bytes).map_err(|error|fault(error.refusal()))?;}
        let mut at=0;while at<text.len(){let mut end=(at+1024).min(text.len());while !text.is_char_boundary(end){end-=1;}pending.try_push_str_controlled(&text[at..end],&mut control).map_err(fault)?;at=end;}
        let original=std::mem::replace(target,std::mem::take(pending));retired.push_reserved(original).map_err(|original|{*rejected=Some(original);fault(refused())})?;Ok(())
    })();*admitted=(*admitted).checked_add(control.owned_bytes()).ok_or_else(||fault(refused()))?;result
}
fn identity_text<'a>(source:&semio_framework_value::paged::PagedUtf8<{usize::MAX}>,scratch:&'a mut[u8;MAX_ID_BYTES])->Result<&'a str,semio_framework::Fault>{if source.is_empty()||source.len()>MAX_ID_BYTES{return Err(fault(refused()));}let mut at=0;for chunk in source.chunks(){let end=at+chunk.len();if end>source.len(){return Err(fault(refused()));}scratch[at..end].copy_from_slice(chunk.as_bytes());at=end;}if at!=source.len(){return Err(fault(refused()));}std::str::from_utf8(&scratch[..at]).map_err(fault)}
struct FixedText<'a>{bytes:&'a mut[u8],at:usize}
impl std::fmt::Write for FixedText<'_>{fn write_str(&mut self,text:&str)->std::fmt::Result{let end=self.at.checked_add(text.len()).filter(|end|*end<=self.bytes.len()).ok_or(std::fmt::Error)?;self.bytes[self.at..end].copy_from_slice(text.as_bytes());self.at=end;Ok(())}}
fn fresh_identity<'a>(kind:crate::schema::identity::DrawingIdentityKind,document:&NativeText,id:&str,ordinal:usize,material:&mut[u8;MAX_ID_BYTES*2+64],output:&'a mut[u8;76],cx:&mut StepContext<'_>)->Result<&'a str,semio_framework::Fault>{
 let mut callback=|_|!cx.is_cancelled();let mut control=semio_framework_value::NativeEncodeControl::new(0,&mut callback);
 crate::standards::v1::subsets::any::io::text::identity::paste::prepare_paste_identity(kind,document,id,ordinal,material,output,&mut control).map_err(fault)
}

fn canonical_operation(value:&NativeText)->bool{value.len()<=16&&["union","difference","intersection","xor"].iter().any(|candidate|value==*candidate)}
fn initial_vec<T>(target:&mut Vec<T>,count:usize,admitted:&mut usize,cx:&mut StepContext<'_>)->Result<(),semio_framework::Fault>{if !target.is_empty()||target.capacity()!=0{return Err(fault(refused()));}let mut accepted=|_|!cx.is_cancelled();let mut control=NativeDecodeControl::new(MAX_NATIVE_BYTES.checked_sub(*admitted).ok_or_else(||fault(refused()))?,&mut accepted);let result=control.allocate_vec(count).map_err(fault);*admitted=(*admitted).checked_add(control.owned_bytes()).ok_or_else(||fault(refused()))?;*target=result?;Ok(())}
fn copy_identity(source:&str,admitted:&mut usize,cx:&mut StepContext<'_>)->Result<String,semio_framework::Fault>{if source.is_empty()||source.len()>MAX_ID_BYTES{return Err(fault(refused()));}let mut accepted=|_|!cx.is_cancelled();let mut control=NativeDecodeControl::new(MAX_NATIVE_BYTES.checked_sub(*admitted).ok_or_else(||fault(refused()))?,&mut accepted);let result=control.copy_text(source).map_err(fault);*admitted=(*admitted).checked_add(control.owned_bytes()).ok_or_else(||fault(refused()))?;result}
impl DrawingClipboardJob{
    pub(crate) fn new(request:ArtifactReservedToolJobRequest<App>)->Result<Self,semio_framework::Fault>{
        let ArtifactReservedToolInput::Action{args,interaction,hover}=request.input else{return Err(fault("Clipboard requires an action input"));};
        let owner=ControlledRetirement::new(Workspace{source:None,unsealed:Some(request.snapshot_read),completion:Some(request.completion),args,interaction,hover,wire:request.raw_wire,packet:None,encoder:None,parser:None,hydration:None,placement:None,text:String::new(),buffer_admitted:0,roots:Vec::new(),retired_ids:Vec::new(),selected:Vec::new(),assets:Vec::new(),mutations:Vec::new(),effects:Vec::new(),interaction_writes:Vec::new(),interaction_targets:Vec::new(),fragment:None,pending_asset:None,rejected_completion:None,parent:None,addresses:metadata::Table::new(),visited:metadata::Table::new(),removed:metadata::Table::new(),identities:metadata::Table::new(),asset_identities:metadata::Table::new(),cycle_active:metadata::Table::new(),cycle_done:metadata::Table::new(),cycle_stack:Vec::new(),pending_native:Default::default(),rejected_native:None,retired_native:Default::default(),target:Default::default()}).map_err(|(error,_)|fault(error))?;
        Ok(Self{owner,outcomes:DrawingJobOutcomes::new(),tool:request.tool_id,layer_clone:None,asset_clone:None,encoding:None,decoding:None,placement:Default::default(),phase:0,at:0,closing:false,complete:false,locator:None,footprint:None,source_root:None,path:[0;MAX_DEPTH],children:[0;MAX_DEPTH],depth:0,entered:false,reference:0,admitted_bytes:0,normalize_other:0,asset_append:0,tree:None,asset_search:0,located_asset:None,ordinal:0,destination:[1.0,0.0,0.0,1.0,0.0,0.0],inverse:[1.0,0.0,0.0,1.0,0.0,0.0],offset:[0.0;2],insertion:0,scratch:[0;MAX_ID_BYTES],material:[0;MAX_ID_BYTES*2+64],mapped:[0;76]})
    }
    fn prepare(&mut self,cx:&mut StepContext<'_>)->Result<(),semio_framework::Fault>{
        let workspace=self.owner.original_mut().ok_or_else(||fault("Clipboard owner is closing"))?;
        if self.tool=="paste"{
            let args=workspace.args.as_mut().ok_or_else(||fault(refused()))?;
            self.placement.anchor=match args.get("anchor").and_then(DslValue::as_str){None|Some("original")=>PasteAnchor::Original,Some("middle")=>PasteAnchor::Middle,Some("centroid")=>PasteAnchor::Centroid,Some("bottomLeft")=>PasteAnchor::BottomLeft,Some("bottomRight")=>PasteAnchor::BottomRight,Some("topLeft")=>PasteAnchor::TopLeft,Some("topRight")=>PasteAnchor::TopRight,_=>return Err(fault(refused()))};
            if let Some(value)=args.get("position"){let DslValue::Array(values)=value else{return Err(fault(refused()));};if values.len()!=3{return Err(fault(refused()));}let mut position=[0.0;3];for(index,value)in values.iter().enumerate(){position[index]=value.as_f64().filter(|value|value.is_finite()).ok_or_else(||fault(refused()))?;}self.placement.position=Some(position);}
            if let Some(parent)=property(args,"parentId"){let DslValue::String(parent)=parent else{return Err(fault(refused()));};if parent.len()>4096{return Err(fault(refused()));}workspace.parent=Some(std::mem::take(parent));}
            let fragment=property(args,"fragment").ok_or_else(||fault(refused()))?;
            if fragment.get("schema").and_then(DslValue::as_str)!=Some(SCHEMA){return Err(fault("Unsupported drawing clipboard schema / Unbekanntes Zeichnungsformat der Zwischenablage"));}
            let media=fragment.get("mediaType").ok_or_else(||fault(refused()))?;
            if media.get("class").and_then(DslValue::as_str)!=Some("twoD")||media.get("form").and_then(DslValue::as_str)!=Some("design"){return Err(fault("Incompatible drawing clipboard media / Unpassender Medientyp der Zwischenablage"));}
            let Some(DslValue::String(text))=property(fragment,"dslText")else{return Err(fault(refused()));};if text.len()>MAX_BYTES{return Err(fault("Drawing clipboard exceeds its declared byte capacity / Zeichnungszwischenablage überschreitet die zugelassene Größe"));}workspace.text=std::mem::take(text);
            workspace.parser=Some(semio_framework_pack_json::JsonGrammarCursor::new(semio_framework_pack_json::JsonMemberPolicy::Reject));self.phase=5;
        }else{
            let selection=workspace.interaction.selection.get(super::super::DRAWING_INTERACTION_DOMAIN).ok_or_else(||fault(ClipboardError::EmptySelection))?;
            if selection.ids.len()>MAX_NODES{return Err(fault(refused()));}if selection.ids.is_empty(){return Err(fault(ClipboardError::EmptySelection));}
            initial_vec(&mut workspace.roots,MAX_NODES,&mut workspace.buffer_admitted,cx)?;initial_vec(&mut workspace.assets,MAX_NODES,&mut workspace.buffer_admitted,cx)?;initial_vec(&mut workspace.selected,selection.ids.len(),&mut workspace.buffer_admitted,cx)?;initial_vec(&mut workspace.retired_ids,MAX_NODES*2,&mut workspace.buffer_admitted,cx)?;
            workspace.packet=Some(DrawingClipboard{schema:copy_identity(SCHEMA,&mut workspace.buffer_admitted,cx)?,roots:Vec::new(),selected:Vec::new(),assets:Default::default()});self.phase=12;self.at=0;
        }
        Ok(())
    }
    /// 🧬️ Seals the captured snapshot read as the original clone source before any layer or asset is copied; true while the seal is still owed.
    fn seal_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,semio_framework::Fault>{
        let workspace=self.owner.original_mut().ok_or_else(||fault("Clipboard owner is closing"))?;
        if workspace.unsealed.is_none(){return Ok(false);}
        type Source=RetainedCloneSource<DrawingSnapshot,store::SnapshotRead<DrawingSnapshot>>;
        let grant=cx.retained_grant();
        if grant.maximum_items==0||grant.maximum_depth<1||grant.maximum_copy_bytes<Source::constructor_copy_bytes()||grant.maximum_capacity_bytes<Source::constructor_capacity_bytes::<()>(){return Ok(true);}
        let read=workspace.unsealed.take().unwrap();
        match read.admit_retained_clone_source(grant){
            Ok((source,progress))=>{workspace.source=Some(source);cx.consume_retained(progress).map_err(fault)?;}
            Err((error,read))=>{workspace.unsealed=Some(read);return Err(fault(error.into_message()));}
        }
        Ok(true)
    }
    fn advance(&mut self,cx:&mut StepContext<'_>)->Result<bool,semio_framework::Fault>{
        if self.seal_source(cx)?{cx.consume_fuel(1);return Ok(false);}
        if self.phase==0{self.prepare(cx)?;cx.consume_fuel(1);return Ok(false);}
        let workspace=self.owner.original_mut().ok_or_else(||fault("Clipboard owner is closing"))?;
        match self.phase{
            1=>{
                cx.set_stage("clipboard-dependencies");
                if self.at==workspace.roots.len(){self.phase=10;self.at=0;self.normalize_other=0;return Ok(false);}
                if self.source_root.is_none(){
                    if self.locator.is_none(){replace_native(&mut workspace.target,&workspace.roots[self.at],&mut workspace.pending_native,&mut workspace.rejected_native,&mut workspace.retired_native,&mut workspace.buffer_admitted,cx)?;self.locator=Some(DrawingLayerLocator::new());}
                    let locator=self.locator.as_mut().unwrap();if !locator.step(source_doc!(workspace),&workspace.target,cx).map_err(fault)?{return Ok(false);}let address=locator.found().ok_or_else(||fault(refused()))?;workspace.addresses.insert(&workspace.roots[self.at],address)?;self.source_root=Some(address);self.locator=None;self.depth=0;self.entered=false;self.children=[0;MAX_DEPTH];return Ok(false);
                }
                let mut address=self.source_root.unwrap();if address.length+self.depth>MAX_DEPTH{return Err(fault(refused()));}address.indices[address.length..address.length+self.depth].copy_from_slice(&self.path[..self.depth]);address.length+=self.depth;
                let node=DrawingLayerLocator::node_at(source_doc!(workspace),address).ok_or_else(||fault(refused()))?;
                if !self.entered{
                    let id=&layer_base(node).id;let id=identity_text(id,&mut self.scratch)?;if workspace.visited.insert_key(&id)?&&workspace.visited.len()>MAX_NODES{return Err(fault(refused()));}
                    let asset=match node{DrawingLayerNode::Image(value)=>Some(&value.image_key),DrawingLayerNode::Trace(value)=>Some(&value.source_key),_=>None};if let Some(asset)=asset{let id=identity_text(asset,&mut self.scratch)?;if !workspace.assets.iter().any(|value|value==id){if workspace.assets.len()==MAX_NODES{return Err(fault(refused()));}workspace.assets.push(copy_identity(id,&mut workspace.buffer_admitted,cx)?);}}
                    self.entered=true;self.reference=0;self.children[self.depth]=0;return Ok(false);
                }else if let DrawingLayerNode::Boolean(boolean)=node{
                    if boolean.children.len()>MAX_NODES{return Err(fault(refused()));}if let Some(id)=boolean.children.get(self.reference){let id=identity_text(id,&mut self.scratch)?;if !workspace.roots.iter().any(|value|value==id){if workspace.roots.len()==MAX_NODES{return Err(fault(refused()));}workspace.roots.push(copy_identity(id,&mut workspace.buffer_admitted,cx)?);}self.reference+=1;return Ok(false);}
                }
                if let DrawingLayerNode::Group(group)=node{let index=self.children[self.depth];if index<group.children.len(){if address.length==MAX_DEPTH{return Err(fault(refused()));}self.children[self.depth]+=1;self.path[self.depth]=index;self.depth+=1;self.entered=false;return Ok(false);}}
                if self.depth==0{self.at+=1;self.source_root=None;self.entered=false;}else{self.depth-=1;self.entered=true;}
            },
            2=>{
                cx.set_stage("clipboard-layer-copy");
                if self.at==workspace.roots.len(){self.phase=3;self.at=0;return Ok(false);}
                let id=&workspace.roots[self.at];let address=workspace.addresses[id];DrawingLayerLocator::node_at(source_doc!(workspace),address).ok_or_else(||fault(refused()))?;let source_ref=workspace.source.as_ref().unwrap().borrow().project(1,move|snapshot|DrawingLayerLocator::node_at(snapshot,address).expect("sealed clipboard layer remains addressable"));let source=source_ref.get();
                if self.layer_clone.is_none(){let footprint=self.footprint.get_or_insert_with(DrawingLayerCloneWorkAuthority::new);if !footprint.step(source,cx).map_err(fault)?{return Ok(false);}let totals=footprint.totals().ok_or_else(||fault(refused()))?;self.admitted_bytes=self.admitted_bytes.checked_add(totals.bytes).filter(|bytes|*bytes<=COPY_NATIVE_BYTES).ok_or_else(||fault(refused()))?;self.footprint=None;self.layer_clone=Some(Clone::new());}
                let copier=self.layer_clone.get_or_insert_with(Clone::new);if !copier.step(source_ref,cx).map_err(fault)?{return Ok(false);}
                let mut node=copier.take().ok_or_else(||fault(refused()))?;
                let mut matrix=world_at(source_doc!(workspace),address)?;
                if matches!(node,DrawingLayerNode::Boolean(_)){let parent=if address.length>1{world_at(source_doc!(workspace),DrawingLayerAddress{length:address.length-1,..address})?}else{[1.0,0.0,0.0,1.0,0.0,0.0]};matrix=crate::schema::geometry::multiply(matrix,invert(parent).map_err(fault)?);}
                layer_base_mut(&mut node).transform=crate::schema::geometry::affine::drawing_matrix_to_transform(matrix);workspace.packet.as_mut().unwrap().roots.push(node);self.layer_clone=None;self.at+=1;
            },
            3=>{
                cx.set_stage("clipboard-asset-copy");
                if self.at==workspace.assets.len(){let packet=workspace.packet.take().unwrap();workspace.encoder=Some(semio_framework_pack_json::JsonWriteCursor::new(encode::ClipboardJsonSource{packet}));self.phase=4;return Ok(false);}
                let id=&workspace.assets[self.at];if self.located_asset.is_none(){let Some((key,_))=source_doc!(workspace).assets.entry_at(self.asset_search)else{return Err(fault("Copied image asset no longer exists / Kopierte Bildressource ist nicht mehr vorhanden"));};let index=self.asset_search;self.asset_search+=1;if key.len()==id.len()&&key==id.as_str(){self.located_asset=Some(index);}return Ok(false);}let located=self.located_asset.unwrap();source_doc!(workspace).assets.entry_at(located).ok_or_else(||fault(refused()))?;let asset_ref=workspace.source.as_ref().unwrap().borrow().project(2,move|snapshot|snapshot.assets.entry_at(located).expect("sealed clipboard asset remains addressable").1);let asset=asset_ref.get();if self.asset_clone.is_none(){self.admitted_bytes=self.admitted_bytes.checked_add(asset.samples.allocated_bytes()).and_then(|bytes|bytes.checked_add(std::mem::size_of::<DrawingImageAsset>()+id.len())).filter(|bytes|*bytes<=COPY_NATIVE_BYTES).ok_or_else(||fault(refused()))?;}
                if workspace.pending_asset.is_none(){let copier=self.asset_clone.get_or_insert_with(Clone::new);if !copier.step(asset_ref,cx).map_err(fault)?{return Ok(false);}let key=copy_identity(id,&mut workspace.buffer_admitted,cx)?;let asset=copier.take().ok_or_else(||fault(refused()))?;workspace.pending_asset=Some((key,asset));}let mut accepted=|_|!cx.is_cancelled();let mut control=NativeDecodeControl::new(MAX_NATIVE_BYTES.checked_sub(workspace.buffer_admitted).ok_or_else(||fault(refused()))?,&mut accepted);let result=workspace.packet.as_mut().unwrap().assets.append_candidate_step(&mut workspace.pending_asset,&mut self.asset_append,&mut control);workspace.buffer_admitted=workspace.buffer_admitted.checked_add(control.owned_bytes()).ok_or_else(||fault(refused()))?;if !result.map_err(fault)?{return Ok(false);}self.asset_clone=None;self.asset_search=0;self.located_asset=None;self.at+=1;
            },
            4=>{
                cx.set_stage("clipboard-encode");let grant=cx.retained_grant();let mut accepted=|_|!cx.is_cancelled();let mut control=if let Some(receipt)=self.encoding.take(){NativeEncodeControl::resume(receipt,&mut accepted).map_err(fault)?}else{NativeEncodeControl::new(MAX_BYTES,&mut accepted)};
                let output=workspace.encoder.as_mut().unwrap().step(32,&mut control,grant).map_err(fault)?;self.encoding=Some(control.pause().map_err(fault)?);
                if let Some(output)=output{workspace.text=output;self.phase=if self.tool=="cut"{20}else{8};self.at=0;}
            },
            5=>{
                cx.set_stage("clipboard-parse");let grant=cx.retained_grant();let mut accepted=|_|!cx.is_cancelled();let mut control=if let Some(receipt)=self.decoding.take(){NativeDecodeControl::resume(receipt,&mut accepted).map_err(fault)?}else{NativeDecodeControl::new(MAX_NATIVE_BYTES,&mut accepted)};
                let value=workspace.parser.as_mut().unwrap().step(&workspace.text,32,&mut control,grant).map_err(fault)?;self.decoding=Some(control.pause().map_err(fault)?);if let Some(value)=value{workspace.hydration=Some(decode::ClipboardHydration::new(value));self.phase=6;}
            },
            6=>{cx.set_stage("clipboard-hydrate");let mut accepted=|_|!cx.is_cancelled();let mut control=NativeDecodeControl::resume(self.decoding.take().ok_or_else(||fault(refused()))?,&mut accepted).map_err(fault)?;let done=workspace.hydration.as_mut().unwrap().step(&mut control).map_err(fault)?;self.decoding=Some(control.pause().map_err(fault)?);if done{let packet=&mut workspace.hydration.as_mut().unwrap().packet;workspace.packet=Some(std::mem::replace(packet,DrawingClipboard{schema:String::new(),roots:Vec::new(),selected:Vec::new(),assets:Default::default()}));self.phase=29;}},
            8=>{self.phase=37;self.at=0;}
            37=>{
                cx.set_stage("clipboard-publication-admission");
                if self.tool!="paste"{initial_vec(&mut workspace.effects,1,&mut workspace.buffer_admitted,cx)?;workspace.fragment=Some(ClipboardFragment{schema:String::new(),media_type:media_type(),dsl_text:String::new(),pack_bytes:None,source_app:String::new(),label:String::new()});}
                if self.tool!="copy"{initial_vec(&mut workspace.interaction_writes,1,&mut workspace.buffer_admitted,cx)?;initial_vec(&mut workspace.interaction_targets,if self.tool=="paste"{workspace.selected.len()}else{0},&mut workspace.buffer_admitted,cx)?;}
                self.phase=38;self.at=0;
            },
            38=>{
                cx.set_stage("clipboard-publication-metadata");
                if let Some(fragment)=workspace.fragment.as_mut(){match self.at{0=>fragment.schema=copy_identity(SCHEMA,&mut workspace.buffer_admitted,cx)?,1=>fragment.source_app=copy_identity(super::super::DRAWING_PLAY_CONTROLLER_ID,&mut workspace.buffer_admitted,cx)?,2=>{use std::fmt::Write;let mut text=FixedText{bytes:&mut self.material,at:0};write!(&mut text,"{} layers / Ebenen",workspace.selected.len()).map_err(fault)?;let label=std::str::from_utf8(&text.bytes[..text.at]).map_err(fault)?;fragment.label=copy_identity(label,&mut workspace.buffer_admitted,cx)?;},_=>{fragment.dsl_text=std::mem::take(&mut workspace.text);workspace.effects.push(Effect::ClipboardWrite{fragment:workspace.fragment.take().unwrap()});self.phase=39;self.at=0;return Ok(false);}}self.at+=1;}else{self.phase=39;self.at=0;}
            },
            39=>{
                cx.set_stage("clipboard-selection-publication");
                if self.tool=="paste"&&self.at<workspace.selected.len(){let granularity=copy_identity(super::super::DRAWING_INTERACTION_GRANULARITY,&mut workspace.buffer_admitted,cx)?;let id=std::mem::take(&mut workspace.selected[self.at]);workspace.interaction_targets.push(protocol::InteractionTarget{granularity,id});self.at+=1;return Ok(false);}
                if self.tool!="copy"{let domain=copy_identity(super::super::DRAWING_INTERACTION_DOMAIN,&mut workspace.buffer_admitted,cx)?;workspace.interaction_writes.push(semio_framework_plugin::InteractionWrite{domain,targets:std::mem::take(&mut workspace.interaction_targets),merge:protocol::MergeMode::Replace});}return Ok(true);
            },
            29=>{
                cx.set_stage("clipboard-destination");
                if source_doc!(workspace).id.len()>MAX_ID_BYTES{return Err(fault(refused()));}
                if let Some(parent)=workspace.parent.as_ref(){
                    if self.locator.is_none(){replace_native(&mut workspace.target,parent,&mut workspace.pending_native,&mut workspace.rejected_native,&mut workspace.retired_native,&mut workspace.buffer_admitted,cx)?;self.locator=Some(DrawingLayerLocator::new());}
                    let locator=self.locator.as_mut().unwrap();if !locator.step(source_doc!(workspace),&workspace.target,cx).map_err(fault)?{return Ok(false);}
                    let address=locator.found().ok_or_else(||fault("Paste destination no longer exists / Zielgruppe existiert nicht mehr"))?;
                    let Some(DrawingLayerNode::Group(group))=DrawingLayerLocator::node_at(source_doc!(workspace),address)else{return Err(fault("Paste destination must be a group / Einfügeziel muss eine Gruppe sein"));};
                    self.insertion=group.children.len();
                    for depth in 1..=address.length{let node=DrawingLayerLocator::node_at(source_doc!(workspace),DrawingLayerAddress{length:depth,..address}).ok_or_else(||fault(refused()))?;if layer_base(node).locked{return Err(fault("Unlock the destination group before pasting / Zielgruppe vor dem Einfügen entsperren"));}}
                    self.destination=world_at(source_doc!(workspace),address)?;self.inverse=invert(self.destination).map_err(fault)?;self.locator=None;
                }else{self.insertion=source_doc!(workspace).layers.len();}
                let packet=workspace.packet.as_ref().unwrap();if packet.schema!=SCHEMA||packet.roots.is_empty()||packet.roots.len()>MAX_NODES||packet.selected.is_empty()||packet.selected.len()>MAX_NODES||packet.assets.len()>MAX_NODES{return Err(fault(refused()));}
                initial_vec(&mut workspace.roots,MAX_NODES,&mut workspace.buffer_admitted,cx)?;initial_vec(&mut workspace.selected,packet.selected.len(),&mut workspace.buffer_admitted,cx)?;initial_vec(&mut workspace.mutations,packet.assets.len()+packet.roots.len(),&mut workspace.buffer_admitted,cx)?;
                self.tree=Some(TreeCursor::new(DrawingLayerAddress{length:1,indices:[0;64]},true));self.phase=30;self.at=0;
            },
            30=>{
                cx.set_stage("clipboard-identities");
                let packet=workspace.packet.as_ref().unwrap();
                let Some(address)=self.tree.as_mut().unwrap().packet(packet)?else{self.tree=None;initial_vec(&mut workspace.cycle_stack,workspace.roots.len(),&mut workspace.buffer_admitted,cx)?;self.phase=38;self.at=0;return Ok(false);};
                let node=packet_node(packet,address).ok_or_else(||fault(refused()))?;let base=layer_base(node);
                if base.id.is_empty()||base.id.len()>MAX_ID_BYTES||workspace.roots.len()==MAX_NODES||!crate::schema::drawing_transform_to_matrix(&base.transform).iter().all(|value|value.is_finite()){return Err(fault(refused()));}
                let id=identity_text(&base.id,&mut self.scratch)?;if workspace.addresses.contains_key(&id){return Err(fault("Clipboard identities must be unique / Zwischenablagekennungen müssen eindeutig sein"));}workspace.addresses.insert(&id,address)?;workspace.roots.push(copy_identity(id,&mut workspace.buffer_admitted,cx)?);
            },
            31=>{
                cx.set_stage("clipboard-fresh-identities");
                if self.at==workspace.roots.len(){self.phase=33;self.at=0;self.ordinal=0;return Ok(false);}
                let id=&workspace.roots[self.at];
                if self.locator.is_none(){let mapped=fresh_identity(crate::schema::identity::DrawingIdentityKind::Layer,&source_doc!(workspace).id,id,self.ordinal,&mut self.material,&mut self.mapped,cx)?;if workspace.removed.contains(mapped){self.ordinal=self.ordinal.checked_add(1).ok_or_else(||fault(refused()))?;return Ok(false);}replace_native(&mut workspace.target,mapped,&mut workspace.pending_native,&mut workspace.rejected_native,&mut workspace.retired_native,&mut workspace.buffer_admitted,cx)?;self.locator=Some(DrawingLayerLocator::new());}
                let locator=self.locator.as_mut().unwrap();if !locator.step(source_doc!(workspace),&workspace.target,cx).map_err(fault)?{return Ok(false);}
                if locator.found().is_some(){self.ordinal=self.ordinal.checked_add(1).ok_or_else(||fault(refused()))?;self.locator=None;return Ok(false);}
                let mapped=identity_text(&workspace.target,&mut self.scratch)?;workspace.removed.insert_key(&mapped)?;workspace.identities.insert_text(id,&mapped)?;self.locator=None;self.ordinal=0;self.at+=1;
            },
            38=>{
                cx.set_stage("clipboard-reference-validation");
                let packet=workspace.packet.as_ref().unwrap();
                if workspace.cycle_stack.is_empty(){if self.at==workspace.roots.len(){self.phase=31;self.at=0;return Ok(false);}let id=&workspace.roots[self.at];self.at+=1;if workspace.cycle_done.contains(id){return Ok(false);}workspace.cycle_active.insert_key(id)?;workspace.cycle_stack.push((workspace.addresses[id],0));return Ok(false);}
                let (address,edge)=*workspace.cycle_stack.last().unwrap();let node=packet_node(packet,address).ok_or_else(||fault(refused()))?;
                let next=match node{DrawingLayerNode::Group(group)=>{if address.length==MAX_DEPTH&&!group.children.is_empty(){return Err(fault(refused()));}group.children.get(edge).map(|_|{let mut next=address;next.indices[next.length]=edge;next.length+=1;next})},DrawingLayerNode::Boolean(boolean)=>{if boolean.children.len()>MAX_NODES||!canonical_operation(&boolean.operation){return Err(fault(refused()));}if let Some(id)=boolean.children.get(edge){if id.is_empty()||id.len()>MAX_ID_BYTES{return Err(fault(refused()));}Some(*workspace.addresses.get(identity_text(id,&mut self.scratch)?).ok_or_else(||fault("Clipboard Boolean references must be included / Booleanreferenzen müssen in der Zwischenablage enthalten sein"))?)}else{None}},_=>None};
                if let Some(next)=next{workspace.cycle_stack.last_mut().unwrap().1+=1;let id=identity_text(&layer_base(packet_node(packet,next).ok_or_else(||fault(refused()))?).id,&mut self.scratch)?;if workspace.cycle_active.contains(&id){return Err(fault("Clipboard Boolean dependencies contain a cycle / Booleanabhängigkeiten der Zwischenablage enthalten einen Zyklus"));}if workspace.cycle_done.contains(&id){return Ok(false);}if workspace.cycle_stack.len()==MAX_NODES{return Err(fault(refused()));}
                    if workspace.cycle_stack.len()==workspace.cycle_stack.capacity(){return Err(fault(refused()));}
                    workspace.cycle_active.insert_key(&id)?;workspace.cycle_stack.push((next,0));
                }else{let id=identity_text(&layer_base(node).id,&mut self.scratch)?;workspace.cycle_active.remove(&id);workspace.cycle_done.insert_key(&id)?;workspace.cycle_stack.pop();}
            },
            33=>{
                cx.set_stage("clipboard-asset-identities");
                let packet=workspace.packet.as_ref().unwrap();let Some((id,asset))=packet.assets.entry_at(self.at)else{self.tree=Some(TreeCursor::new(DrawingLayerAddress{length:1,indices:[0;64]},true));self.source_root=None;self.phase=32;self.at=0;return Ok(false);};
                if id.is_empty()||id.len()>MAX_ID_BYTES||asset.width==0||asset.height==0||(asset.width as usize).checked_mul(asset.height as usize)!=Some(asset.samples.len()){return Err(fault("Clipboard image dimensions and samples are invalid / Bildmaße und Bilddaten der Zwischenablage sind ungültig"));}
                let mapped=fresh_identity(crate::schema::identity::DrawingIdentityKind::ImageAsset,&source_doc!(workspace).id,id,self.ordinal,&mut self.material,&mut self.mapped,cx)?;
                if let Some((key,_))=source_doc!(workspace).assets.entry_at(self.asset_search){self.asset_search+=1;if key.len()==mapped.len()&&key==mapped{self.ordinal=self.ordinal.checked_add(1).ok_or_else(||fault(refused()))?;self.asset_search=0;}return Ok(false);}if workspace.asset_identities.values().any(|value|value==mapped){self.ordinal=self.ordinal.checked_add(1).ok_or_else(||fault(refused()))?;self.asset_search=0;return Ok(false);}
                workspace.asset_identities.insert_text(id,&mapped)?;self.ordinal=0;self.asset_search=0;self.at+=1;
            },
            32=>{
                cx.set_stage("clipboard-reference-remap");
                let packet=workspace.packet.as_mut().unwrap();
                if self.source_root.is_none(){let Some(address)=self.tree.as_mut().unwrap().packet(packet)?else{self.tree=None;self.phase=36;self.at=0;return Ok(false);};self.source_root=Some(address);self.reference=0;let node=packet_node_mut(packet,address).ok_or_else(||fault(refused()))?;let id=identity_text(&layer_base(node).id,&mut self.scratch)?;let mapped=workspace.identities.get(id).ok_or_else(||fault(refused()))?;replace_native(&mut layer_base_mut(node).id,mapped,&mut workspace.pending_native,&mut workspace.rejected_native,&mut workspace.retired_native,&mut workspace.buffer_admitted,cx)?;
                    match node{DrawingLayerNode::Image(image)=>{let id=identity_text(&image.image_key,&mut self.scratch)?;let mapped=workspace.asset_identities.get(id).ok_or_else(||fault("Clipboard image asset is missing / Bilddaten der Zwischenablage fehlen"))?;replace_native(&mut image.image_key,mapped,&mut workspace.pending_native,&mut workspace.rejected_native,&mut workspace.retired_native,&mut workspace.buffer_admitted,cx)?;},DrawingLayerNode::Trace(trace)=>{let id=identity_text(&trace.source_key,&mut self.scratch)?;let mapped=workspace.asset_identities.get(id).ok_or_else(||fault("Clipboard image asset is missing / Bilddaten der Zwischenablage fehlen"))?;replace_native(&mut trace.source_key,mapped,&mut workspace.pending_native,&mut workspace.rejected_native,&mut workspace.retired_native,&mut workspace.buffer_admitted,cx)?;},DrawingLayerNode::Boolean(boolean)=>{if !canonical_operation(&boolean.operation)||boolean.children.len()>MAX_NODES{return Err(fault("Unsupported clipboard Boolean operation / Unbekannte Booleanoperation der Zwischenablage"));}},_=>{}}return Ok(false);}
                if let Some(DrawingLayerNode::Boolean(boolean))=packet_node_mut(packet,self.source_root.unwrap()){if let Some(id)=boolean.children.get_mut(self.reference){if id.is_empty()||id.len()>MAX_ID_BYTES{return Err(fault(refused()));}let original=identity_text(id,&mut self.scratch)?;let mapped=workspace.identities.get(original).ok_or_else(||fault("Clipboard Boolean references must be included / Booleanreferenzen müssen in der Zwischenablage enthalten sein"))?;replace_native(id,mapped,&mut workspace.pending_native,&mut workspace.rejected_native,&mut workspace.retired_native,&mut workspace.buffer_admitted,cx)?;self.reference+=1;return Ok(false);}}
                self.source_root=None;
            },
            36=>{
                cx.set_stage("clipboard-selection-remap");
                let packet=workspace.packet.as_ref().unwrap();if let Some(id)=packet.selected.get(self.at){if !workspace.visited.insert_key(id)?{return Err(fault("Clipboard selection must be unique / Zwischenablageauswahl muss eindeutig sein"));}let mapped=workspace.identities.get(id).ok_or_else(||fault("Clipboard selection is incomplete / Zwischenablageauswahl ist unvollständig"))?;workspace.selected.push(copy_identity(mapped,&mut workspace.buffer_admitted,cx)?);self.at+=1;}else{self.phase=34;}
            },
            34=>{
                cx.set_stage("clipboard-placement");
                let mut bounds=None;let mut centroid=[0.0;2];
                if self.placement.position.is_some()&&!matches!(self.placement.anchor,PasteAnchor::Original){let source=ClipboardScene{packet:workspace.packet.as_ref().unwrap(),selected:&workspace.selected};
                    if workspace.placement.is_none(){workspace.placement=Some(crate::schema::scene_placement::DrawingScenePlacementJob::new(&source,super::super::geometry_session::limits(),super::super::geometry_session::algorithms(),0.05).map_err(fault)?);return Ok(false);}
                    let job=workspace.placement.as_mut().unwrap();if !job.advance(&source,1).map_err(fault)?.done{return Ok(false);}let output=job.result().map_err(fault)?;bounds=output.bounds;centroid=output.centroid;
                }

                let [left,top,right,bottom]=bounds.unwrap_or([0.0;4]);let anchor=match self.placement.anchor{PasteAnchor::Original=>[0.0,0.0],PasteAnchor::Middle=>[(left+right)/2.0,(top+bottom)/2.0],PasteAnchor::Centroid=>centroid,PasteAnchor::BottomLeft=>[left,bottom],PasteAnchor::BottomRight=>[right,bottom],PasteAnchor::TopLeft=>[left,top],PasteAnchor::TopRight=>[right,top]};
                self.offset=self.placement.position.map(|position|[position[0]-anchor[0],position[1]-anchor[1]]).unwrap_or([16.0,16.0]);if !self.offset.iter().all(|value|value.is_finite()){return Err(fault("Paste placement must be finite / Einfügeposition muss endlich sein"));}self.phase=35;self.at=0;
            },
            35=>{
                cx.set_stage("clipboard-mutation-plan");
                let packet=workspace.packet.as_mut().unwrap();
                if let Some((id,_))=packet.assets.entry_at(self.at){let mapped=workspace.asset_identities.get(id).ok_or_else(||fault(refused()))?;replace_native(&mut workspace.target,mapped,&mut workspace.pending_native,&mut workspace.rejected_native,&mut workspace.retired_native,&mut workspace.buffer_admitted,cx)?;let asset=std::mem::replace(packet.assets.entry_at_mut(self.at).ok_or_else(||fault(refused()))?.1,DrawingImageAsset{width:0,height:0,samples:Default::default()});workspace.mutations.push(DrawingMutation::ImportImageAsset(crate::mutations::ImportImageAsset{asset_id:std::mem::take(&mut workspace.target),asset}));self.at+=1;return Ok(false);}
                if let Some(node)=packet.roots.last(){let local=crate::schema::drawing_transform_to_matrix(&layer_base(node).transform);let translated=crate::schema::geometry::multiply([1.0,0.0,0.0,1.0,self.offset[0],self.offset[1]],local);let mut matrix=crate::schema::geometry::multiply(self.inverse,translated);if matches!(node,DrawingLayerNode::Boolean(_)){matrix=crate::schema::geometry::multiply(matrix,self.destination);}if !matrix.iter().all(|value|value.is_finite()){return Err(fault(refused()));}workspace.buffer_admitted=workspace.buffer_admitted.checked_add(std::mem::size_of::<DrawingLayerNode>()).filter(|bytes|*bytes<=MAX_NATIVE_BYTES).ok_or_else(||fault(refused()))?;let parent_id=if let Some(parent)=workspace.parent.as_ref(){replace_native(&mut workspace.target,parent,&mut workspace.pending_native,&mut workspace.rejected_native,&mut workspace.retired_native,&mut workspace.buffer_admitted,cx)?;Some(std::mem::take(&mut workspace.target))}else{None};let mut node=packet.roots.pop().unwrap();layer_base_mut(&mut node).transform=crate::schema::geometry::affine::drawing_matrix_to_transform(matrix);workspace.mutations.push(crate::mutations::create_layer(parent_id,Some(self.insertion),node));return Ok(false);}self.phase=8;
            },
            20=>{
                cx.set_stage("clipboard-cut-locks");
                if workspace.mutations.capacity()==0{initial_vec(&mut workspace.mutations,workspace.selected.len(),&mut workspace.buffer_admitted,cx)?;}
                if self.at==workspace.selected.len(){self.tree=if source_doc!(workspace).layers.is_empty(){None}else{let mut address=DrawingLayerAddress{length:1,indices:[0;64]};address.indices[0]=0;Some(TreeCursor::new(address,true))};self.source_root=None;self.phase=21;return Ok(false);}
                if self.tree.is_none(){let address=workspace.addresses[&workspace.selected[self.at]];for depth in 1..=address.length{let ancestor=DrawingLayerLocator::node_at(source_doc!(workspace),DrawingLayerAddress{length:depth,..address}).ok_or_else(||fault(refused()))?;if layer_base(ancestor).locked{return Err(fault("Unlock selected layers before cutting / Ausgewählte Ebenen vor dem Ausschneiden entsperren"));}}self.tree=Some(TreeCursor::new(address,false));}
                if let Some(address)=self.tree.as_mut().unwrap().step(source_doc!(workspace))?{let node=DrawingLayerLocator::node_at(source_doc!(workspace),address).ok_or_else(||fault(refused()))?;let base=layer_base(node);if base.locked{return Err(fault("Unlock selected descendants before cutting / Ausgewählte Unterebenen vor dem Ausschneiden entsperren"));}if base.id.is_empty()||base.id.len()>MAX_ID_BYTES{return Err(fault(refused()));}workspace.removed.insert_key(identity_text(&base.id,&mut self.scratch)?)?;}
                else{replace_native(&mut workspace.target,&workspace.selected[self.at],&mut workspace.pending_native,&mut workspace.rejected_native,&mut workspace.retired_native,&mut workspace.buffer_admitted,cx)?;workspace.mutations.push(crate::mutations::delete_layer(std::mem::take(&mut workspace.target)));self.at+=1;self.tree=None;}
            },
            21=>{
                cx.set_stage("clipboard-cut-references");
                if self.source_root.is_none(){let Some(tree)=self.tree.as_mut()else{self.phase=8;return Ok(false);};let Some(address)=tree.step(source_doc!(workspace))?else{self.tree=None;self.phase=8;return Ok(false);};self.source_root=Some(address);self.reference=0;return Ok(false);}
                let node=DrawingLayerLocator::node_at(source_doc!(workspace),self.source_root.unwrap()).ok_or_else(||fault(refused()))?;
                let base=layer_base(node);if base.id.len()>MAX_ID_BYTES{return Err(fault(refused()));}
                if base.id.is_empty()||base.id.len()>MAX_ID_BYTES||!workspace.removed.contains(identity_text(&base.id,&mut self.scratch)?){if let DrawingLayerNode::Boolean(boolean)=node{if let Some(id)=boolean.children.get(self.reference){if !id.is_empty()&&id.len()<=MAX_ID_BYTES&&workspace.removed.contains(identity_text(id,&mut self.scratch)?){return Err(fault("A Boolean outside the selection still uses these layers / Ein Boolean außerhalb der Auswahl verwendet diese Ebenen"));}self.reference+=1;return Ok(false);}}}
                self.source_root=None;
            },
            10|11=>{cx.set_stage("clipboard-normalize");let ids=if self.phase==10{&mut workspace.roots}else{&mut workspace.selected};if self.at==ids.len(){if self.phase==10{self.phase=11;self.at=0;}else{let packet=workspace.packet.as_mut().unwrap();initial_vec(&mut packet.roots,workspace.roots.len(),&mut workspace.buffer_admitted,cx)?;initial_vec(&mut packet.selected,workspace.selected.len(),&mut workspace.buffer_admitted,cx)?;self.phase=13;self.at=0;}self.normalize_other=0;return Ok(false);}if self.normalize_other==ids.len(){self.at+=1;self.normalize_other=0;return Ok(false);}let address=workspace.addresses[&ids[self.at]];let other=workspace.addresses[&ids[self.normalize_other]];if self.at!=self.normalize_other&&other.length<address.length&&address.indices[..other.length]==other.indices[..other.length]{workspace.retired_ids.push(ids.remove(self.at));self.normalize_other=0;}else{self.normalize_other+=1;}},
            12=>{cx.set_stage("clipboard-selection");let ids=&workspace.interaction.selection[super::super::DRAWING_INTERACTION_DOMAIN].ids;if self.at==ids.len(){self.phase=1;self.at=0;return Ok(false);}let id=&ids[self.at];if id.is_empty()||id.len()>MAX_ID_BYTES{return Err(fault(refused()));}if !workspace.selected.contains(id){workspace.selected.push(copy_identity(id,&mut workspace.buffer_admitted,cx)?);workspace.roots.push(copy_identity(id,&mut workspace.buffer_admitted,cx)?);}self.at+=1;},
            13=>{cx.set_stage("clipboard-selected-identities");if self.at==workspace.selected.len(){self.phase=2;self.at=0;return Ok(false);}let id=copy_identity(&workspace.selected[self.at],&mut workspace.buffer_admitted,cx)?;workspace.packet.as_mut().unwrap().selected.push(id);self.at+=1;},
            _=>return Err(fault(refused()))
        };cx.consume_fuel(1);Ok(false)
    }
    fn publish(&mut self,result:Result<(),semio_framework::Fault>)->DrawingJobTurn{
        let workspace=self.owner.original_mut().unwrap();let emit=result.map(|()|{
            let mut emit=Emit::mutations(std::mem::take(&mut workspace.mutations));
            emit.effects=std::mem::take(&mut workspace.effects);emit.interaction_writes=std::mem::take(&mut workspace.interaction_writes);emit
        });
        let failed=emit.is_err();let Some(completion)=workspace.completion.as_ref()else{return self.outcomes.fault("drawing.clipboard-completion-missing");};
        if let Err(rejected)=completion.complete(emit,EphemeralEmit::default()){
            workspace.rejected_completion=Some(super::super::completion::RejectedCompletion::new(rejected));
            return self.outcomes.fault("drawing.clipboard-completion-rejected");
        }
        self.complete=true;if failed{self.outcomes.fault("drawing.clipboard-refused")}else{DrawingJobTurn::Complete}
    }
    fn demands(&self,copy:usize)->Result<RetainedCloneGrant,ValueError>{if !self.outcomes.terminal_is_empty(){let demand=self.outcomes.retirement_demands()?;return Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth});}if let Some(owner)=self.layer_clone.as_ref(){return owner.close_demands(copy);}if let Some(owner)=self.asset_clone.as_ref(){return owner.close_demands(copy);}Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:self.owner.next_copy_byte_demand()?,maximum_capacity_bytes:self.owner.next_capacity_byte_demand(copy)?,maximum_release_bytes:self.owner.next_release_byte_demand()?,maximum_depth:self.owner.next_depth_demand()?})}
}
impl DrawingClipboardJob{
    fn turn(&mut self,cx:&mut StepContext<'_>)->DrawingJobTurn{
        if self.outcomes.faulted(){return DrawingJobTurn::Fault;}
        if self.closing||cx.is_cancelled(){return DrawingJobTurn::Cancelled;}if self.complete{return DrawingJobTurn::Complete;}
        while !cx.should_yield(){if cx.is_cancelled(){return DrawingJobTurn::Cancelled;}let result=self.advance(cx);cx.consume_fuel(1);match result{Ok(true)=>return self.publish(Ok(())),Ok(false)=>{},Err(error)=>return self.publish(Err(error))}}
        DrawingJobTurn::Yield
    }
}
impl InteractiveJob for DrawingClipboardJob{
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,ValueError>{let turn=self.turn(cx);self.outcomes.lend(turn,cx)}
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{self.outcomes.borrow_outcome(descriptor)}
    fn begin_close(&mut self){self.closing=true;}
    fn close_step(&mut self,grant:RetainedCloneGrant)->InteractiveJobCloseStep{
        if !self.closing{return InteractiveJobCloseStep::Blocked;}
        if !self.outcomes.terminal_is_empty(){return self.outcomes.close_job_step(grant);}
        if self.locator.is_some()||self.footprint.is_some()||self.tree.is_some(){if grant.maximum_items==0{return InteractiveJobCloseStep::Pending{progress:Default::default()};}self.locator=None;self.footprint=None;self.tree=None;return InteractiveJobCloseStep::Pending{progress:semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,..Default::default()}};}
        let step=if let Some(owner)=self.layer_clone.as_mut(){let step=owner.close_step(grant);if owner.terminal_is_empty(){self.layer_clone=None;}step}else if let Some(owner)=self.asset_clone.as_mut(){let step=owner.close_step(grant);if owner.terminal_is_empty(){self.asset_clone=None;}step}else if !self.owner.terminal_is_empty(){self.owner.step(grant)}else{

            self.encoding=None;self.decoding=None;return InteractiveJobCloseStep::Complete{progress:Default::default()};
        };match step{Ok(RetainedCloneStep::Progress(progress))|Ok(RetainedCloneStep::Complete(progress))=>InteractiveJobCloseStep::Pending{progress},Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}}
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.maximum_copy_bytes)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.maximum_capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.maximum_release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.maximum_depth)}
    fn terminal_is_empty(&self)->bool{self.closing&&self.owner.terminal_is_empty()&&self.layer_clone.is_none()&&self.asset_clone.is_none()&&self.encoding.is_none()&&self.decoding.is_none()&&self.locator.is_none()&&self.footprint.is_none()&&self.tree.is_none()&&self.outcomes.terminal_is_empty()}
}
impl ArtifactReservedJob for DrawingClipboardJob {}
