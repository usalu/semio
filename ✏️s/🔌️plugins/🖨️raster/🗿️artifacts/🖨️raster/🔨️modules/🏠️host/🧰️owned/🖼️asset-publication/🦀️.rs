//! 🖼️ Asset leaves hash actual immutable images incrementally and restore them through granted root aliases.
use super::*;
use semio_s_artifact_raster_raster_model::{RasterImagePayload,RasterImageContentHash,raster_image_child_id,RASTER_IMAGE_CHILD_ID_BYTES,SemioImageSnapshot};
use semio_s_artifact_raster_raster_mutations_pixels::{add_layer_asset,remove_layer_asset};

#[derive(Default,semio_framework_value::RetireOwned)]
struct AssetOwned{pending:Option<(String,RasterAssetChild)>,inverse_payload:Option<RasterImagePayload>,inverse:Vec<RasterMutation>}

pub(super) struct RasterAssetMutationWork{
    owned:std::mem::ManuallyDrop<Option<AssetOwned>>,
    retirement:std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    hash:Option<RasterImageContentHash>,
    digest:Option<[u8;8]>,
    phase:u8,
    inverse:bool,
    closing:bool,
}

impl RasterAssetMutationWork{
    pub(super) fn new(inverse:bool)->Self{Self{owned:std::mem::ManuallyDrop::new(Some(AssetOwned::default())),retirement:std::mem::ManuallyDrop::new(None),hash:Some(RasterImageContentHash::new()),digest:None,phase:0,inverse,closing:false}}

    fn key(operation:&RasterMutation)->Result<&str,&'static str>{match operation{RasterMutation::AddLayerAsset(value)=>Ok(&value.asset_id),RasterMutation::RemoveLayerAsset(value)=>Ok(&value.asset_id),_=>Err("raster-store.asset-operation")}}

    fn demands(&self,candidate:&RasterSnapshot,operation:&RasterMutation,body:usize)->Result<semio_framework_value::RetirementDemand,&'static str>{let key=Self::key(operation)?;let(copy,capacity)=match self.phase{0=>(key.len(),0),1=>(body.clamp(1,32768),0),2=>if self.inverse{(0,size_of::<RasterMutation>())}else{(0,0)},3=>(key.len()+size_of::<RasterMutation>(),key.len()),5=>if candidate.assets.page_required_for_insert(key){(0,RasterOwnedMap::<RasterAssetChild>::conservative_page_credit_bytes())}else{(0,0)},6|9=>(size_of::<(String,RasterAssetChild)>(),0),10=>{let bytes=key.len()*2+6+RASTER_IMAGE_CHILD_ID_BYTES+b"s.stdio.semio".len()+b"v1".len()+b"image".len();(bytes+size_of::<RasterAssetChild>(),bytes)},_ =>(0,0)};Ok(semio_framework_value::RetirementDemand{copy_bytes:copy,capacity_bytes:capacity,depth:1,..Default::default()})}

    fn admit(cx:&mut dyn RasterWork,copy:usize,capacity:usize)->bool{if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return false;}let demand=semio_framework_value::RetirementDemand{copy_bytes:copy,capacity_bytes:capacity,depth:1,..Default::default()};if raster_short(cx.retained_grant(),demand){return false;}cx.consume_fuel(1);true}
    fn receipt(cx:&mut dyn RasterWork,copy:usize,capacity:usize)->Result<(),&'static str>{cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,..Default::default()}).map_err(|_|"raster-store.asset-receipt")}

    pub(super) fn step(&mut self,base:&RasterSnapshot,candidate:&mut RasterSnapshot,operation:&RasterMutation,retirement:&mut RasterRetirementSlot,cx:&mut dyn RasterWork)->Result<bool,&'static str>{
        if self.closing{return Err("raster-store.asset-closing");}let key=Self::key(operation)?;
        if key.len()>RASTER_OWNED_FIELD_BYTES{return Err("raster-store.asset-key-capacity");}
        match self.phase{
            0=>{
                if !Self::admit(cx,key.len(),0){return Ok(false);}let exists=base.assets.contains_key(key);Self::receipt(cx,key.len(),0)?;
                match operation{RasterMutation::AddLayerAsset(_)if exists=>{self.phase=7;return Ok(true);},RasterMutation::RemoveLayerAsset(_)if !exists=>return Err("raster-store.mutation-asset-missing"),RasterMutation::AddLayerAsset(_)=>self.phase=1,_=>self.phase=2}
            },
            1=>{
                let RasterMutation::AddLayerAsset(value)=operation else{return Err("raster-store.asset-hash-operation")};let bytes=cx.retained_grant().maximum_copy_bytes.min(32768).min(cx.fuel_remaining()as usize).max(1);if !Self::admit(cx,bytes,0){return Ok(false);}let hash=self.hash.as_mut().ok_or("raster-store.asset-hash")?;let progress=hash.advance(value.asset.image(),bytes)?;cx.consume_fuel(progress.processed_bytes.saturating_sub(1)as u64);Self::receipt(cx,progress.processed_bytes,0)?;if progress.done{self.digest=self.hash.take().unwrap().finish();self.phase=2;}
            },
            2=>{
                if !self.inverse{self.phase=4;return Ok(false);}let capacity=size_of::<RasterMutation>();if !Self::admit(cx,0,capacity){return Ok(false);}let data=self.owned.as_mut().unwrap();data.inverse=empty_vec(1);Self::receipt(cx,0,data.inverse.capacity()*size_of::<RasterMutation>())?;self.phase=if matches!(operation,RasterMutation::RemoveLayerAsset(_)){8}else{3};
            },
            3=>{
                if !Self::admit(cx,key.len()+size_of::<RasterMutation>(),key.len()){return Ok(false);}let asset_id=raster_exact_string_from_parts(&[key.as_bytes()])?;let capacity=asset_id.capacity();let mutation=match operation{
                    RasterMutation::AddLayerAsset(_)=>RasterMutation::RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset{asset_id}),
                    RasterMutation::RemoveLayerAsset(_)=>{let asset=self.owned.as_mut().unwrap().inverse_payload.take().ok_or("raster-store.asset-inverse-root")?;RasterMutation::AddLayerAsset(add_layer_asset::AddLayerAsset{asset_id,asset})},
                    _=>return Err("raster-store.asset-inverse-operation"),
                };self.owned.as_mut().unwrap().inverse.push(mutation);Self::receipt(cx,key.len()+size_of::<RasterMutation>(),capacity)?;self.phase=4;
            },
            4=>{
                self.phase=if matches!(operation,RasterMutation::AddLayerAsset(_)){9}else{6};return Ok(false);
            },
            5=>{if candidate.assets.page_required_for_insert(key){let bytes=RasterOwnedMap::<RasterAssetChild>::conservative_page_credit_bytes();if !Self::admit(cx,0,bytes){return Ok(false);}candidate.assets.admit_one_page()?;Self::receipt(cx,0,bytes)?;}self.phase=6;},
            6=>{
                if !Self::admit(cx,size_of::<(String,RasterAssetChild)>(),0){return Ok(false);}match operation{
                    RasterMutation::AddLayerAsset(_)=>{let(key,child)=self.owned.as_mut().unwrap().pending.take().ok_or("raster-store.asset-child")?;match candidate.assets.insert_pre_admitted(key,child){Ok(RasterOwnedMapInsert::Inserted)=>{},Ok(RasterOwnedMapInsert::Replaced(mut old))=>{retirement.put(RasterDisplaced::AssetEntry(old.take()));return Err("raster-store.asset-destination-changed");},Err(rejected)=>{retirement.put(RasterDisplaced::AssetEntry((rejected.key,rejected.value)));return Err(rejected.reason);}}},
                    RasterMutation::RemoveLayerAsset(_)=>{let mut removed=candidate.assets.remove_entry(key).ok_or("raster-store.mutation-asset-missing")?;retirement.put(RasterDisplaced::AssetEntry(removed.take()));},
                    _=>return Err("raster-store.asset-apply-operation"),
                }Self::receipt(cx,size_of::<(String,RasterAssetChild)>(),0)?;self.phase=7;return Ok(true);
            },
            7=>return Ok(true),
            8=>{
                let demand=RasterImagePayload::alias_demands();if cx.fuel_remaining()<2||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth<demand.depth||raster_short(cx.retained_grant(),demand){return Ok(false)}let owner=base.assets.get(key).and_then(|child|child.local_owner::<SemioImageSnapshot>()).ok_or("raster-store.asset-inverse-unmaterialized")?;cx.consume_fuel(1);let turn=RasterImagePayload::from_shared(owner,cx.retained_grant());cx.consume_fuel(turn.work);match turn.result{Ok((payload,receipt))=>{self.owned.as_mut().unwrap().inverse_payload=Some(payload);cx.consume_retained(receipt).map_err(|_|"raster-store.asset-inverse-root-receipt")?;},Err((error,owner))=>{self.owned.as_mut().unwrap().inverse_payload=Some(<RasterImagePayload as semio_framework_value::owned::NativeReceiveMap<std::sync::Arc<SemioImageSnapshot>>>::from_native(owner));cx.consume_retained(error.retained_progress()).map_err(|_|"raster-store.asset-inverse-root-error-receipt")?;return Err("raster-store.asset-inverse-root")}}self.phase=3;
            },
            9=>{
                let RasterMutation::AddLayerAsset(value)=operation else{return Err("raster-store.asset-image-operation")};let copy=size_of::<(String,RasterAssetChild)>();if cx.fuel_remaining()<2||!Self::admit(cx,copy,0){return Ok(false)}let turn=value.asset.owner(cx.retained_grant());cx.consume_fuel(turn.work);match turn.result{Ok((owner,mut receipt))=>{let child=RasterAssetChild::new(String::new(),semio_framework_artifact_reference::ArtifactRef{artifact_id:String::new(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:String::new(),standard:String::new(),subset:String::new()}}).with_local_owner(owner);self.owned.as_mut().unwrap().pending=Some((String::new(),child));receipt.copied_bytes=receipt.copied_bytes.checked_add(copy).ok_or("raster-store.asset-child-copy-overflow")?;cx.consume_retained(receipt).map_err(|_|"raster-store.asset-child-root-receipt")?;},Err(error)=>{cx.consume_retained(error.retained_progress()).map_err(|_|"raster-store.asset-child-root-error-receipt")?;return Err("raster-store.asset-child-root")}}self.phase=10;
            },
            10=>{
                let bytes=key.len()*2+6+RASTER_IMAGE_CHILD_ID_BYTES+b"s.stdio.semio".len()+b"v1".len()+b"image".len();if !Self::admit(cx,bytes+size_of::<RasterAssetChild>(),bytes){return Ok(false)}let(map_key,child)=self.owned.as_mut().unwrap().pending.as_mut().ok_or("raster-store.asset-child-root")?;*map_key=raster_exact_string_from_parts(&[key.as_bytes()])?;child.child_id=raster_image_child_id(self.digest.ok_or("raster-store.asset-digest")?);child.target=semio_framework_artifact_reference::ArtifactRef{artifact_id:raster_exact_string_from_parts(&[key.as_bytes(),b"-image"])?,dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:raster_exact_string_from_parts(&[b"s.stdio.semio"])?,standard:raster_exact_string_from_parts(&[b"v1"])?,subset:raster_exact_string_from_parts(&[b"image"])?}};let capacity=map_key.capacity()+child.child_id.capacity()+child.target.artifact_id.capacity()+child.target.dialect.artifact_kind.capacity()+child.target.dialect.standard.capacity()+child.target.dialect.subset.capacity();Self::receipt(cx,bytes+size_of::<RasterAssetChild>(),capacity)?;self.phase=5;
            },
            _=>return Err("raster-store.asset-phase"),
        }Ok(false)
    }

    pub(super) fn take_inverse(&mut self)->Option<Vec<RasterMutation>>{if self.phase!=7{return None;}Some(std::mem::take(&mut self.owned.as_mut()?.inverse))}
    pub(super) fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{if let Some(active)=self.retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(active,body);}if self.owned.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.owned);}Ok(Default::default())}
    pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep>{self.closing=true;if self.retirement.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant)}else{store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.owned,&mut self.retirement,grant)}}
    pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.owned.is_none()&&self.retirement.is_none()}
}
impl Drop for RasterAssetMutationWork{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"asset publication must hand off or retire every owner before Drop");}}

pub(super) enum RasterRetainedMutationWork{Pixel(RasterPixelMutationWork),Asset(RasterAssetMutationWork)}
impl RasterRetainedMutationWork{
    pub(super) fn new(operation:&RasterMutation,inverse:bool)->Self{if pixel_target(operation).is_some(){Self::Pixel(RasterPixelMutationWork::new(inverse))}else{Self::Asset(RasterAssetMutationWork::new(inverse))}}
    pub(super) fn step(&mut self,base:&RasterSnapshot,candidate:&mut RasterSnapshot,operation:&RasterMutation,address:Option<RasterLayerAddress>,retirement:&mut RasterRetirementSlot,cx:&mut dyn RasterWork)->Result<bool,&'static str>{if cx.should_yield(){return Ok(false);}match self{Self::Pixel(work)=>work.step(base,candidate,operation,address.ok_or("raster-store.pixel-address")?,retirement,cx),Self::Asset(work)=>work.step(base,candidate,operation,retirement,cx)}}
    pub(super) fn demands(&self,base:&RasterSnapshot,candidate:&RasterSnapshot,operation:&RasterMutation,address:Option<RasterLayerAddress>,body:usize)->Result<semio_framework_value::RetirementDemand,&'static str>{match self{Self::Pixel(work)=>work.demands(base,candidate,operation,address.ok_or("raster-store.pixel-address")?,body),Self::Asset(work)=>work.demands(candidate,operation,body)}}
    pub(super) fn take_inverse(&mut self)->Option<Vec<RasterMutation>>{match self{Self::Pixel(work)=>work.take_inverse(),Self::Asset(work)=>work.take_inverse()}}
    pub(super) fn output_ready(&self)->bool{match self{Self::Pixel(work)=>work.output_ready(),Self::Asset(work)=>work.phase==7}}
    pub(super) fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{match self{Self::Pixel(work)=>work.close_demands(body),Self::Asset(work)=>work.close_demands(body)}}
    pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep>{match self{Self::Pixel(work)=>work.close_step(grant),Self::Asset(work)=>work.close_step(grant)}}
    pub(super) fn terminal_is_empty(&self)->bool{match self{Self::Pixel(work)=>work.terminal_is_empty(),Self::Asset(work)=>work.terminal_is_empty()}}
}
