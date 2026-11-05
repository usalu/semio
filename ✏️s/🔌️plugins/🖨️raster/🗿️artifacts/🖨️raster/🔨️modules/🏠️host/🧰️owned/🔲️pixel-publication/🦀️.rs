//! 📬️ One private pixel publication retains image custody, exact restoration and capacity-safe asset replacement.
use super::*;
#[path="📜️demands/🦀️.rs"]mod demands;
use semio_s_artifact_raster_raster_model::{RasterImagePayload,RasterLayerMask,RasterPixelContent,RasterTransform,raster_image_child_id,RASTER_IMAGE_CHILD_ID_BYTES};
use semio_s_artifact_raster_raster_mutations_pixels::{add_layer_asset,remove_layer_asset,change_layer_pixels,change_layer_mask,transform_image};

#[derive(semio_framework_value::RetireOwned)]
struct PublicationOwned {
    result:Option<RasterPixelPreparedImage>,
    payload:Option<RasterImagePayload>,
    inverse_payload:Option<RasterImagePayload>,
    layer_id:String,
    previous:Option<String>,
    mask:Option<RasterLayerMask>,
    width:Option<u32>,
    height:Option<u32>,
    transform:RasterTransform,
    pending:Option<(String,RasterAssetChild)>,
    inverse:Vec<RasterMutation>,
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Compute,Capture,References,Root,ReserveInverse,Detach,Remove,AddRoot,Add,Restore,RemoveFinal,ChildRoot,Child,Release,ReserveMap,Insert,Attach,Ready}

pub(super) struct RasterPixelMutationWork {
    result:Option<RasterPixelResultWork>,
    owned:std::mem::ManuallyDrop<Option<PublicationOwned>>,
    retirement:std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    locator:RasterLayerLocator,
    phase:Phase,
    inverse:bool,
    release:bool,
    detach:bool,
    closing:bool,
}

fn copied(value:&str)->Result<String,&'static str>{raster_exact_string_from_parts(&[value.as_bytes()])}
fn mask_with_key(mask:&RasterLayerMask,image_key:Option<String>)->RasterLayerMask{RasterLayerMask{enabled:mask.enabled,linked:mask.linked,invert:mask.invert,width:mask.width,height:mask.height,image_key,transform:mask.transform.clone()}}

impl RasterPixelMutationWork {
    pub(super) fn new(inverse:bool)->Self{Self{result:Some(RasterPixelResultWork::new()),owned:std::mem::ManuallyDrop::new(Some(PublicationOwned{result:None,payload:None,inverse_payload:None,layer_id:String::new(),previous:None,mask:None,width:None,height:None,transform:RasterTransform::default(),pending:None,inverse:Vec::new()})),retirement:std::mem::ManuallyDrop::new(None),locator:RasterLayerLocator::new(),phase:Phase::Compute,inverse,release:false,detach:false,closing:false}}

    fn data(&self)->Result<&PublicationOwned,&'static str>{self.owned.as_ref().ok_or("raster-store.pixel-publication-owner")}

    fn ready(&self)->Result<&RasterPixelPreparedImage,&'static str>{self.data()?.result.as_ref().ok_or("raster-store.pixel-publication-result")}

    fn admit(cx:&mut dyn RasterWork,copy:usize,capacity:usize)->bool{
        if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return false;}
        let demand=semio_framework_value::RetirementDemand{copy_bytes:copy,capacity_bytes:capacity,depth:1,..Default::default()};
        if raster_short(cx.retained_grant(),demand){return false;}cx.consume_fuel(1);true
    }

    fn receipt(cx:&mut dyn RasterWork,copy:usize,capacity:usize)->Result<(),&'static str>{cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,..Default::default()}).map_err(|_|"raster-store.pixel-publication-receipt")}

    pub(super) fn step(&mut self,base:&RasterSnapshot,candidate:&mut RasterSnapshot,operation:&RasterMutation,address:RasterLayerAddress,retirement:&mut RasterRetirementSlot,cx:&mut dyn RasterWork)->Result<bool,&'static str>{
        if self.closing{return Err("raster-store.pixel-publication-closing");}
        if cx.should_yield(){return Ok(false);}
        match self.phase{
            Phase::Compute=>{let result=self.result.as_mut().ok_or("raster-store.pixel-result-owner")?;if result.step(base,operation,cx)?{self.owned.as_mut().unwrap().result=result.take();self.phase=Phase::Capture;}},
            Phase::Capture=>{
                if !self.ready()?.changed{self.phase=Phase::Ready;return Ok(true);}
                let(layer_id,target)=pixel_target(operation).ok_or("raster-store.pixel-publication-operation")?;
                let layer=RasterLayerLocator::node_at(base,address).ok_or("raster-store.pixel-publication-layer")?;
                let(mask,previous,width,height,transform)=match(target,layer){
                    ("mask",RasterLayerNode::Pixel{mask:Some(mask),transform,..}|RasterLayerNode::Group{mask:Some(mask),transform,..})=>(Some(mask),mask.image_key.as_deref(),None,None,transform),
                    ("pixels",RasterLayerNode::Pixel{image_key,width,height,transform,..})=>(None,image_key.as_deref(),*width,*height,transform),
                    _=>return Err("raster-store.pixel-publication-target"),
                };
                let bytes=layer_id.len()+previous.map_or(0,str::len)+mask.and_then(|mask|mask.image_key.as_ref()).map_or(0,String::len);
                if !Self::admit(cx,bytes+size_of::<RasterTransform>(),bytes){return Ok(false);}
                let data=self.owned.as_mut().unwrap();data.layer_id=copied(layer_id)?;data.previous=previous.map(copied).transpose()?;data.mask=mask.cloned();data.width=width;data.height=height;data.transform=transform.clone();
                Self::receipt(cx,bytes+size_of::<RasterTransform>(),data.layer_id.capacity()+data.previous.as_ref().map_or(0,String::capacity)+data.mask.as_ref().and_then(|mask|mask.image_key.as_ref()).map_or(0,String::capacity))?;self.phase=Phase::References;
            },
            Phase::References=>{
                let data=self.owned.as_ref().ok_or("raster-store.pixel-publication-owner")?;let Some(previous)=data.previous.as_deref()else{self.phase=Phase::Root;return Ok(false);};
                if Some(previous)==self.ready()?.key.as_deref(){self.phase=Phase::Root;return Ok(false);}
                let layer_id=&data.layer_id;let target=pixel_target(operation).ok_or("raster-store.pixel-target")?.1;
                if self.locator.step_where(base,|node|{let own=RasterLayerLocator::node_id(node)==layer_id;let pixels=matches!(node,RasterLayerNode::Pixel{image_key:Some(key),..}if key==previous)&&!(own&&target=="pixels");let mask=match node{RasterLayerNode::Pixel{mask:Some(mask),..}|RasterLayerNode::Group{mask:Some(mask),..}=>mask.image_key.as_deref()==Some(previous)&&!(own&&target=="mask"),_=>false};pixels||mask},cx)?{self.release=self.locator.found.is_none();self.detach=self.release&&!self.ready()?.reuse_destination&&base.assets.len()==semio_s_artifact_raster_raster_model::RASTER_OWNED_MAP_CAPACITY;self.phase=Phase::Root;}
            },
            Phase::Root=>{
                if self.ready()?.reuse_destination{self.phase=Phase::ReserveInverse;return Ok(false);}
                let demand=RasterImagePayload::owned_birth_demands();if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth<demand.depth||raster_short(cx.retained_grant(),demand){return Ok(false);}
                let data=self.owned.as_mut().unwrap();let image=data.result.as_mut().and_then(|result|result.image.take()).ok_or("raster-store.pixel-image-owner")?;let turn=RasterImagePayload::from_owned(image,cx.retained_grant());
                cx.consume_fuel(turn.work);match turn.result{Ok((payload,receipt))=>{data.payload=Some(payload);cx.consume_retained(receipt).map_err(|_|"raster-store.pixel-root-receipt")?;self.phase=Phase::ReserveInverse;},Err((error,image))=>{data.result.as_mut().unwrap().image=Some(image);cx.consume_retained(error.retained_progress()).map_err(|_|"raster-store.pixel-root-error-receipt")?;return Err("raster-store.pixel-root-refused");}}
            },
            Phase::ReserveInverse=>{
                if !self.inverse{self.phase=Phase::ChildRoot;return Ok(false);}
                let capacity=4*size_of::<RasterMutation>();if !Self::admit(cx,0,capacity){return Ok(false);}let data=self.owned.as_mut().unwrap();data.inverse=empty_vec(4);Self::receipt(cx,0,data.inverse.capacity()*size_of::<RasterMutation>())?;self.phase=Phase::Detach;
            },
            Phase::Detach=>{if self.detach{if !self.push_attachment(None,None,false,cx)?{return Ok(false);}}self.phase=Phase::Remove;},
            Phase::Remove=>{
                if self.detach{let bytes=self.ready()?.key.as_ref().ok_or("raster-store.pixel-key")?.len();if !Self::admit(cx,bytes,bytes){return Ok(false);}let key=copied(self.ready()?.key.as_ref().unwrap())?;let cap=key.capacity();self.owned.as_mut().unwrap().inverse.push(RasterMutation::RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset{asset_id:key}));Self::receipt(cx,bytes,cap)?;}self.phase=Phase::AddRoot;
            },
            Phase::AddRoot=>{
                if !self.release{self.phase=Phase::Restore;return Ok(false)}let demand=RasterImagePayload::alias_demands();if cx.fuel_remaining()<2||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth<demand.depth||raster_short(cx.retained_grant(),demand){return Ok(false)}let previous=self.data()?.previous.as_ref().ok_or("raster-store.pixel-previous")?;let owner=base.assets.get(previous).and_then(|child|child.local_owner::<semio_s_artifact_raster_raster_model::SemioImageSnapshot>()).ok_or("raster-store.pixel-inverse-source")?;cx.consume_fuel(1);let turn=RasterImagePayload::from_shared(owner,cx.retained_grant());cx.consume_fuel(turn.work);match turn.result{Ok((asset,receipt))=>{self.owned.as_mut().unwrap().inverse_payload=Some(asset);cx.consume_retained(receipt).map_err(|_|"raster-store.pixel-inverse-root-receipt")?;},Err((error,owner))=>{self.owned.as_mut().unwrap().inverse_payload=Some(<RasterImagePayload as semio_framework_value::owned::NativeReceiveMap<std::sync::Arc<semio_s_artifact_raster_raster_model::SemioImageSnapshot>>>::from_native(owner));cx.consume_retained(error.retained_progress()).map_err(|_|"raster-store.pixel-inverse-root-error-receipt")?;return Err("raster-store.pixel-inverse-root")}}self.phase=Phase::Add;
            },
            Phase::Add=>{
                let previous=self.data()?.previous.as_ref().ok_or("raster-store.pixel-previous")?;let bytes=previous.len()+size_of::<RasterMutation>();if !Self::admit(cx,bytes,previous.len()){return Ok(false)}let asset_id=copied(previous)?;let cap=asset_id.capacity();let data=self.owned.as_mut().unwrap();let asset=data.inverse_payload.take().ok_or("raster-store.pixel-inverse-payload")?;data.inverse.push(RasterMutation::AddLayerAsset(add_layer_asset::AddLayerAsset{asset_id,asset}));Self::receipt(cx,bytes,cap)?;self.phase=Phase::Restore;
            },
            Phase::Restore=>{
                if !self.push_attachment(Some(()),Some(self.detach),matches!(operation,RasterMutation::TransformImage(_)),cx)?{return Ok(false);}
                self.phase=if !self.detach&&!self.ready()?.reuse_destination{Phase::RemoveFinal}else{Phase::ChildRoot};
            },
            Phase::RemoveFinal=>{
                let bytes=self.ready()?.key.as_ref().ok_or("raster-store.pixel-key")?.len();if !Self::admit(cx,bytes,bytes){return Ok(false);}let key=copied(self.ready()?.key.as_ref().unwrap())?;let cap=key.capacity();self.owned.as_mut().unwrap().inverse.push(RasterMutation::RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset{asset_id:key}));Self::receipt(cx,bytes,cap)?;self.phase=Phase::ChildRoot;
            },
            Phase::ChildRoot=>{
                if self.ready()?.reuse_destination{self.phase=Phase::Release;return Ok(false)}let copy=size_of::<(String,RasterAssetChild)>();if cx.fuel_remaining()<2||!Self::admit(cx,copy,0){return Ok(false)}let payload=self.data()?.payload.as_ref().ok_or("raster-store.pixel-payload")?;let turn=payload.owner(cx.retained_grant());cx.consume_fuel(turn.work);match turn.result{Ok((owner,mut receipt))=>{let child=RasterAssetChild::new(String::new(),semio_framework_artifact_reference::ArtifactRef{artifact_id:String::new(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:String::new(),standard:String::new(),subset:String::new()}}).with_local_owner(owner);self.owned.as_mut().unwrap().pending=Some((String::new(),child));receipt.copied_bytes=receipt.copied_bytes.checked_add(copy).ok_or("raster-store.pixel-child-root-copy-overflow")?;cx.consume_retained(receipt).map_err(|_|"raster-store.pixel-child-root-receipt")?;},Err(error)=>{cx.consume_retained(error.retained_progress()).map_err(|_|"raster-store.pixel-child-root-error-receipt")?;return Err("raster-store.pixel-child-root")}}self.phase=Phase::Child;
            },
            Phase::Child=>{
                if self.ready()?.reuse_destination{self.phase=Phase::Release;return Ok(false);}
                let key_bytes=self.ready()?.key.as_ref().ok_or("raster-store.pixel-key")?.len();let digest=self.ready()?.digest.ok_or("raster-store.pixel-digest")?;
                let bytes=key_bytes*2+6+RASTER_IMAGE_CHILD_ID_BYTES+b"s.stdio.semio".len()+b"v1".len()+b"image".len();if !Self::admit(cx,bytes+size_of::<RasterAssetChild>(),bytes){return Ok(false);}
                let data=self.owned.as_mut().unwrap();let key=data.result.as_ref().unwrap().key.as_ref().unwrap();let(map_key,child)=data.pending.as_mut().ok_or("raster-store.pixel-child-root")?;*map_key=copied(key)?;child.child_id=raster_image_child_id(digest);child.target=semio_framework_artifact_reference::ArtifactRef{artifact_id:raster_exact_string_from_parts(&[key.as_bytes(),b"-image"])?,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:copied("s.stdio.semio")?,standard:copied("v1")?,subset:copied("image")?}};
                let capacity=map_key.capacity()+child.child_id.capacity()+child.target.artifact_id.capacity()+child.target.dialect.artifact_kind.capacity()+child.target.dialect.standard.capacity()+child.target.dialect.subset.capacity();Self::receipt(cx,bytes+size_of::<RasterAssetChild>(),capacity)?;self.phase=Phase::Release;
            },
            Phase::Release=>{
                if self.release{if !Self::admit(cx,size_of::<(String,RasterAssetChild)>(),0){return Ok(false);}let previous=self.data()?.previous.as_ref().ok_or("raster-store.pixel-previous")?;let mut removed=candidate.assets.remove_entry(previous).ok_or("raster-store.pixel-release-missing")?;retirement.put(RasterDisplaced::AssetEntry(removed.take()));Self::receipt(cx,size_of::<(String,RasterAssetChild)>(),0)?;}self.phase=Phase::ReserveMap;
            },
            Phase::ReserveMap=>{
                if self.ready()?.reuse_destination{self.phase=Phase::Attach;return Ok(false);}
                let key=self.data()?.pending.as_ref().ok_or("raster-store.pixel-child")?.0.as_str();if candidate.assets.page_required_for_insert(key){let bytes=RasterOwnedMap::<RasterAssetChild>::conservative_page_credit_bytes();if !Self::admit(cx,0,bytes){return Ok(false);}candidate.assets.admit_one_page()?;Self::receipt(cx,0,bytes)?;}self.phase=Phase::Insert;
            },
            Phase::Insert=>{
                if !Self::admit(cx,size_of::<(String,RasterAssetChild)>(),0){return Ok(false);}let(key,child)=self.owned.as_mut().unwrap().pending.take().ok_or("raster-store.pixel-child")?;match candidate.assets.insert_pre_admitted(key,child){Ok(RasterOwnedMapInsert::Inserted)=>{},Ok(RasterOwnedMapInsert::Replaced(mut previous))=>{retirement.put(RasterDisplaced::AssetEntry(previous.take()));return Err("raster-store.pixel-destination-changed");},Err(rejected)=>{retirement.put(RasterDisplaced::AssetEntry((rejected.key,rejected.value)));return Err(rejected.reason);}}Self::receipt(cx,size_of::<(String,RasterAssetChild)>(),0)?;self.phase=Phase::Attach;
            },
            Phase::Attach=>{
                let bytes=self.ready()?.key.as_ref().ok_or("raster-store.pixel-key")?.len();if !Self::admit(cx,bytes,bytes){return Ok(false);}let data=self.owned.as_ref().unwrap();let key=copied(data.result.as_ref().unwrap().key.as_ref().unwrap())?;let cap=key.capacity();let layer=RasterLayerLocator::node_at_mut(candidate,address).ok_or("raster-store.pixel-attachment-target")?;
                let previous=if data.mask.is_some(){let(RasterLayerNode::Pixel{mask:Some(mask),..}|RasterLayerNode::Group{mask:Some(mask),..})=layer else{return Err("raster-store.pixel-mask-lost")};mask.image_key.replace(key)}else{let RasterLayerNode::Pixel{image_key,width,height,transform,..}=layer else{return Err("raster-store.pixel-layer-lost")};if let RasterMutation::TransformImage(value)=operation{let before=data.result.as_ref().unwrap().before.as_ref().ok_or("raster-store.pixel-before")?;let image=data.payload.as_ref().map(RasterImagePayload::image).or_else(||data.result.as_ref().unwrap().image.as_ref()).ok_or("raster-store.pixel-output")?;let reshape=transform_image::reshape(value,&data.transform,(data.width,data.height),(before.width,before.height),(image.width,image.height));*width=Some(reshape.width);*height=Some(reshape.height);*transform=reshape.transform;}image_key.replace(key)};
                if let Some(previous)=previous{retirement.put(RasterDisplaced::String(previous));}Self::receipt(cx,bytes,cap)?;self.phase=Phase::Ready;return Ok(true);
            },
            Phase::Ready=>return Ok(true),
        }
        Ok(false)
    }

    fn push_attachment(&mut self,restore:Option<()>,detached:Option<bool>,transform:bool,cx:&mut dyn RasterWork)->Result<bool,&'static str>{
        let data=self.data()?;let result=data.result.as_ref().ok_or("raster-store.pixel-result")?;let next=if restore.is_some(){data.previous.as_deref()}else{None};let expected=if detached==Some(true){None}else{result.key.as_deref()};let bytes=data.layer_id.len()+next.map_or(0,str::len)+expected.map_or(0,str::len);if !Self::admit(cx,bytes+size_of::<RasterTransform>(),bytes){return Ok(false);}
        let layer_id=copied(&data.layer_id)?;let next=next.map(copied).transpose()?;let expected=expected.map(copied).transpose()?;let capacity=layer_id.capacity()+next.as_ref().map_or(0,String::capacity)+expected.as_ref().map_or(0,String::capacity);
        let mutation=if let Some(mask)=&data.mask{let prior=mask_with_key(mask,expected);let replacement=restore.map(|_|mask_with_key(mask,next));RasterMutation::ChangeLayerMask(change_layer_mask::ChangeLayerMask{layer_id,expected:if detached==Some(true){None}else{Some(prior)},mask:replacement})}else{RasterMutation::ChangeLayerPixels(change_layer_pixels::ChangeLayerPixels{layer_id,expected_image_key:expected,content:RasterPixelContent{image_key:next,width:data.width,height:data.height},transform:transform.then(||data.transform.clone())})};
        self.owned.as_mut().unwrap().inverse.push(mutation);Self::receipt(cx,bytes+size_of::<RasterTransform>(),capacity)?;Ok(true)
    }

    pub(super) fn take_inverse(&mut self)->Option<Vec<RasterMutation>>{if self.phase!=Phase::Ready{return None;}Some(std::mem::take(&mut self.owned.as_mut()?.inverse))}
    pub(super) fn output_ready(&self)->bool{self.phase==Phase::Ready}
    pub(super) fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{if let Some(result)=self.result.as_ref(){return result.close_demands(body);}if let Some(active)=self.retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(active,body);}if self.owned.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.owned);}Ok(Default::default())}
    pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep>{self.closing=true;if grant.maximum_items==0||grant.maximum_depth==0{return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}if let Some(result)=self.result.as_mut(){if result.terminal_is_empty(){self.result.take();return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1};}return result.close_step(grant);}if self.retirement.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant)}else{store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.owned,&mut self.retirement,grant)}}
    pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.result.is_none()&&self.owned.is_none()&&self.retirement.is_none()}
}

impl Drop for RasterPixelMutationWork{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"pixel publication must hand off or retire every owner before Drop");}}
