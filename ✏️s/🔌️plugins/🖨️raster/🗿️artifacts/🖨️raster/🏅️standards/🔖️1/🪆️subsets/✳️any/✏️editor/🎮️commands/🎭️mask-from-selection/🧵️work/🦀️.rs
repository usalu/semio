//! 🧵️ Granted selection validation, raw mask construction, hashing and atomic adoption.
use super::*;
use std::mem::size_of;
use semio_os_kernel as store;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use semio_s_artifact_stdio_semio_image::schema::snapshot::{SemioImageSnapshot,SemioImageFrame,SemioColorspace,STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
use semio_s_artifact_raster_raster_model::{RasterImageContentHash,RasterImagePayload,raster_image_key,RASTER_IMAGE_KEY_BYTES};

#[derive(semio_framework_value::RetireOwned)]
struct MaskOwned {layer_id:String,source_key:Option<String>,prior:Option<RasterLayerMask>,image:Option<SemioImageSnapshot>,key:String,rows:Vec<RasterMutation>}
#[derive(Clone,Copy,PartialEq)]
enum Phase {Discover,Bind,Header,Reserve,Pixels,Hash,Key,References,Destination,Plan,RowsReserve,Rows,Ready,Taken}
#[derive(Clone,Copy)]
enum Row {Unused,Detach,Remove,Add,AttachPrior,AttachEmpty}

/// 🎭️ Private mask work borrows the retained command's captured immutable roots until closure.
/// Callers keep both original roots unchanged and alive through every advance and close call.
pub struct MaskSelectionWork {
    owned:std::mem::ManuallyDrop<Option<MaskOwned>>,
    retirement:std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    base:usize,config:usize,width:u32,height:u32,display_width:u32,display_height:u32,
    span_count:usize,span_depth:usize,pixel:usize,span:usize,active:Option<(usize,usize,u8)>,
    phase:Phase,hash:Option<RasterImageContentHash>,digest:Option<[u8;8]>,
    path:[usize;33],locked:[bool;33],depth:usize,compare:usize,bind_bytes:usize,bind_capacity:usize,references:usize,plan:[Row;4],plan_len:usize,row:usize,closing:bool,failed:bool,verified:usize,destination_verified:bool,
}

fn refused(code:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,code)}
fn funded(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{
    grant.maximum_items>0&&grant.maximum_depth>=demand.depth&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes
}
fn progress(bytes:usize,capacity:usize)->RetainedCloneProgress{RetainedCloneProgress {copied_items:1,copied_bytes:bytes,retained_capacity_bytes:capacity,..Default::default()}}
fn mask_key(mask:Option<&RasterLayerMask>)->Option<&str>{mask.and_then(|mask|mask.image_key.as_deref())}
fn header<'a>(owned:&MaskOwned,node:&'a RasterLayerNode,document:&RasterSnapshot,config:&RasterConfig)->Result<(&'a Option<RasterLayerMask>,u32,u32,u32,u32),ValueError>{
    let RasterLayerNode::Pixel {image_key,mask,width,height,..}=node else{return Err(refused("raster.mask-requires-pixel-layer"));};
    if image_key!=&owned.source_key{return Err(refused("raster.mask-image-conflict"));}
    let (intrinsic_width,intrinsic_height)=if let Some(key)=image_key{
        let image=document.assets.get(key).and_then(|asset|asset.local_owner::<SemioImageSnapshot>()).ok_or_else(||refused("raster.mask-image-unavailable"))?;
        let count=semio_framework_pixels::editing::validate_extent(image.width,image.height).map_err(|_|refused("raster.mask-extent-invalid"))?;
        if image.frames.first().is_none_or(|frame|frame.rgba8.len()!=count*4){return Err(refused("raster.mask-source-frame-invalid"));}
        (image.width,image.height)
    }else{(width.unwrap_or(512),height.unwrap_or(512))};
    let count=semio_framework_pixels::editing::validate_extent(intrinsic_width,intrinsic_height).map_err(|_|refused("raster.mask-extent-invalid"))?;
    let selection=config.pixel_selection.as_ref().ok_or_else(||refused("raster.mask-selection-missing"))?;
    if selection.layer_id!=owned.layer_id||selection.target!="pixels"||(selection.width,selection.height)!=(intrinsic_width,intrinsic_height)||selection.spans.len()>count||selection.spans.maximum_end()>count as u64{return Err(refused("raster.mask-selection-conflict"));}
    let display_width=width.unwrap_or(intrinsic_width);let display_height=height.unwrap_or(intrinsic_height);
    semio_framework_pixels::editing::validate_extent(display_width,display_height).map_err(|_|refused("raster.mask-extent-invalid"))?;
    if mask_key(mask.as_ref()).is_some_and(|key|!document.assets.contains_key(key)){return Err(refused("raster.mask-prior-image-unavailable"));}
    Ok((mask,intrinsic_width,intrinsic_height,display_width,display_height))
}
fn at<'a>(layers:&'a [RasterLayerNode],path:&[usize])->Option<&'a RasterLayerNode>{
    let mut siblings=layers;
    for (depth,index)in path.iter().copied().enumerate(){let node=siblings.get(index)?;if depth+1==path.len(){return Some(node);}let RasterLayerNode::Group {children,..}=node else{return None;};siblings=children;}
    None
}

impl MaskSelectionWork {
    pub fn begin_demands(command:&MaskFromSelection,_document:&RasterSnapshot,_config:&RasterConfig)->Result<RetirementDemand,ValueError>{
        let bytes=command.layer_id.len().checked_add(command.expected_image_key.as_ref().map_or(0,String::len)).ok_or_else(||refused("raster.mask-header-overflow"))?;
        if command.layer_id.is_empty()||command.layer_id.len()>semio_s_artifact_raster_raster_model::RASTER_OWNED_FIELD_BYTES||command.expected_image_key.as_ref().is_some_and(|key|key.len()>semio_s_artifact_raster_raster_model::RASTER_OWNED_FIELD_BYTES){return Err(refused("raster.mask-command-invalid"));}
        Ok(RetirementDemand {copy_bytes:bytes,capacity_bytes:bytes,depth:1,..Default::default()})
    }
    pub fn begin(command:&MaskFromSelection,document:&RasterSnapshot,config:&RasterConfig,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        let demand=Self::begin_demands(command,document,config)?;if !funded(grant,demand){return Ok(None);}
        let owned=MaskOwned {layer_id:command.layer_id.clone(),source_key:command.expected_image_key.clone(),prior:None,image:None,key:String::new(),rows:Vec::new()};
        let capacity=owned.layer_id.capacity()+owned.source_key.as_ref().map_or(0,String::capacity);
        let receipt=progress(demand.copy_bytes,capacity);
        Ok(Some((Self {owned:std::mem::ManuallyDrop::new(Some(owned)),retirement:std::mem::ManuallyDrop::new(None),base:document as *const _ as usize,config:config as *const _ as usize,width:0,height:0,display_width:0,display_height:0,span_count:0,span_depth:1,pixel:0,span:0,active:None,phase:Phase::Discover,hash:None,digest:None,path:[0;33],locked:[false;33],depth:0,compare:0,bind_bytes:0,bind_capacity:0,references:0,plan:[Row::Unused;4],plan_len:0,row:0,closing:false,failed:false,verified:0,destination_verified:false},receipt)))
    }
    fn owned(&self)->Result<&MaskOwned,ValueError>{self.owned.as_ref().ok_or_else(||refused("raster.mask-owner-missing"))}
    fn count(&self)->usize{self.width as usize*self.height as usize}
    fn row_bytes(&self,row:Row)->Result<usize,ValueError>{let owned=self.owned()?;let old=mask_key(owned.prior.as_ref()).map_or(0,str::len);Ok(match row{Row::Unused=>0,Row::Detach=>owned.layer_id.len()+old,Row::Remove=>old,Row::Add=>owned.key.len(),Row::AttachPrior=>owned.layer_id.len()+old+owned.key.len(),Row::AttachEmpty=>owned.layer_id.len()+owned.key.len()})}
    pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.closing||self.failed{return Err(refused("raster.mask-owner-unavailable"));}
        let mut demand=RetirementDemand {depth:1,..Default::default()};
        match self.phase {
            Phase::Discover=>demand.copy_bytes=body.clamp(2,32768),
            Phase::Bind=>{demand.copy_bytes=self.bind_bytes;demand.capacity_bytes=self.bind_capacity;}
            Phase::Header=>{demand.copy_bytes=STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.len();demand.capacity_bytes=demand.copy_bytes+size_of::<SemioImageFrame>();}
            Phase::Reserve=>demand.capacity_bytes=self.count()*4,
            Phase::Pixels=>{demand.depth=self.span_depth;let size=size_of::<semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::RasterSelectionSpan>();let reads=self.span_count-self.span-usize::from(self.active.is_some());let minimum=if self.active.is_none()&&reads>0{size+4}else{4};demand.copy_bytes=(self.count()*4-self.pixel*4+reads*size).min(body.min(32768).max(minimum));}
            Phase::Hash=>demand.copy_bytes=body.clamp(1,32768),
            Phase::Key=>{demand.copy_bytes=RASTER_IMAGE_KEY_BYTES;demand.capacity_bytes=RASTER_IMAGE_KEY_BYTES;}
            Phase::References=>demand.copy_bytes=mask_key(self.owned()?.prior.as_ref()).map_or(0,str::len)*2,
            Phase::Destination=>demand.copy_bytes=(self.count()*4-self.verified).min((body.min(32768)/2).max(1))*2,
            Phase::RowsReserve=>demand.capacity_bytes=self.plan_len*size_of::<RasterMutation>(),
            Phase::Rows=>{let row=self.plan[self.row];demand.copy_bytes=self.row_bytes(row)?;demand.capacity_bytes=demand.copy_bytes+if matches!(row,Row::Add){RasterImagePayload::owned_birth_demands().capacity_bytes}else{0};}
            _=>{}
        }
        Ok(demand)
    }
    fn original(&self,document:&RasterSnapshot,config:&RasterConfig)->Result<(),ValueError>{
        if self.base!=document as *const _ as usize||self.config!=config as *const _ as usize{return Err(refused("raster.mask-original-changed"));}
        Ok(())
    }
    fn walk_next(&mut self,document:&RasterSnapshot)->Result<bool,ValueError>{
        let node=at(&document.layers,&self.path[..=self.depth]).ok_or_else(||refused("raster.mask-layer-walk-invalid"))?;
        if let RasterLayerNode::Group {children,locked,..}=node{if !children.is_empty(){if self.depth==32{return Err(refused("raster.mask-layer-depth"));}let inherited=self.locked[self.depth]||*locked;self.depth+=1;self.path[self.depth]=0;self.locked[self.depth]=inherited;return Ok(true);}}
        self.path[self.depth]+=1;
        while at(&document.layers,&self.path[..=self.depth]).is_none(){if self.depth==0{return Ok(false);}self.depth-=1;self.path[self.depth]+=1;}
        Ok(true)
    }
    fn discover(&mut self,document:&RasterSnapshot,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        let node=at(&document.layers,&self.path[..=self.depth]).ok_or_else(||refused("raster.mask-layer-missing"))?;
        let id=semio_s_artifact_raster_raster_model::layer_node_id(node);let target=&self.owned()?.layer_id;
        let mut bytes=0;let mut equal=id.len()==target.len();
        if equal{let end=id.len().min(self.compare+grant.maximum_copy_bytes.min(32768)/2);equal=id.as_bytes()[self.compare..end]==target.as_bytes()[self.compare..end];bytes=(end-self.compare)*2;self.compare=end;if equal&&end<id.len(){return Ok(progress(bytes,0));}}
        if equal{
            if self.locked[self.depth]||semio_s_artifact_raster_raster_model::layer_locked(node){return Err(refused("raster.mask-layer-protected").with_retained_progress(progress(bytes,0)));}
            let RasterLayerNode::Pixel {mask,..}=node else{return Err(refused("raster.mask-requires-pixel-layer").with_retained_progress(progress(bytes,0)));};
            let capacity=mask_key(mask.as_ref()).map_or(0,str::len);if capacity>semio_s_artifact_raster_raster_model::RASTER_OWNED_FIELD_BYTES{return Err(refused("raster.mask-prior-field-limit").with_retained_progress(progress(bytes,0)));}let owned=self.owned()?;let bytes=capacity.checked_add(owned.layer_id.len()*2).and_then(|bytes|bytes.checked_add(owned.source_key.as_ref().map_or(0,String::len)*2)).ok_or_else(||refused("raster.mask-header-overflow"))?;self.bind_capacity=capacity;self.bind_bytes=bytes;self.phase=Phase::Bind;
        }else{self.compare=0;if !self.walk_next(document).map_err(|error|error.with_retained_progress(progress(bytes,0)))?{return Err(refused("raster.mask-layer-missing").with_retained_progress(progress(bytes,0)));}}
        Ok(progress(bytes,0))
    }
    fn next_reference(&mut self,document:&RasterSnapshot)->Result<(),ValueError>{
        let node=at(&document.layers,&self.path[..=self.depth]).ok_or_else(||refused("raster.mask-layer-walk-invalid"))?;
        if let Some(previous)=mask_key(self.owned()?.prior.as_ref()){
            let count=match node{RasterLayerNode::Pixel {image_key,mask,..}=>usize::from(image_key.as_deref()==Some(previous))+usize::from(mask_key(mask.as_ref())==Some(previous)),RasterLayerNode::Group {mask,..}=>usize::from(mask_key(mask.as_ref())==Some(previous)),_=>0};self.references+=count;
        }
        if !self.walk_next(document)?{self.phase=Phase::Plan;}
        Ok(())
    }
    fn plan(&mut self,document:&RasterSnapshot)->Result<(),ValueError>{
        let owned=self.owned()?;let previous=mask_key(owned.prior.as_ref());let existing=document.assets.contains_key(&owned.key);
        if existing&&!self.destination_verified{
            let image=self.destination(document)?;
            if image.schema!=STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA||image.width!=self.width||image.height!=self.height||image.colorspace!=SemioColorspace::Rgba||image.bit_depth!=8||image.icc.is_some()||!image.metadata.is_empty()||image.frames.len()!=1||image.frames[0].delay_ms!=0||image.frames[0].rgba8.len()!=self.count()*4{return Err(refused("raster.mask-destination-conflict"));}
            self.phase=Phase::Destination;return Ok(());
        }
        let unchanged=owned.prior.as_ref().is_some_and(|mask|mask.enabled&&mask.linked&&!mask.invert&&mask.width==Some(self.display_width)&&mask.height==Some(self.display_height)&&mask.image_key.as_deref()==Some(owned.key.as_str())&&mask.transform==RasterTransform::default());
        if unchanged{self.phase=Phase::Ready;return Ok(());}
        let release=self.references==1&&previous.is_some_and(|key|key!=owned.key&&document.assets.contains_key(key));
        let full=!existing&&document.assets.len()>=RASTER_OWNED_MAP_CAPACITY;
        if full&&!release{return Err(refused("raster.mask-asset-capacity"));}
        let rows=if full{[Row::Detach,Row::Remove,Row::Add,Row::AttachEmpty]}else{[if existing{Row::Unused}else{Row::Add},Row::AttachPrior,if release{Row::Remove}else{Row::Unused},Row::Unused]};
        self.plan_len=0;for row in rows{if !matches!(row,Row::Unused){self.plan[self.plan_len]=row;self.plan_len+=1;}}
        self.phase=Phase::RowsReserve;Ok(())
    }
    fn build_row(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        use semio_s_artifact_raster_raster_mutations_pixels::{add_layer_asset,change_layer_mask,remove_layer_asset};
        let row=self.plan[self.row];let bytes=self.row_bytes(row)?;let width=self.display_width;let height=self.display_height;let owned=self.owned.as_mut().ok_or_else(||refused("raster.mask-owner-missing"))?;let mut receipt=progress(bytes,bytes);
        let mutation=match row {
            Row::Detach=>RasterMutation::ChangeLayerMask(change_layer_mask::ChangeLayerMask {layer_id:owned.layer_id.clone(),expected:owned.prior.clone(),mask:None}),
            Row::Remove=>RasterMutation::RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset {asset_id:mask_key(owned.prior.as_ref()).ok_or_else(||refused("raster.mask-prior-missing"))?.into()}),
            Row::Add=>{
                let image=owned.image.take().ok_or_else(||refused("raster.mask-result-missing"))?;let child=RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(bytes),..grant};
                let turn=RasterImagePayload::from_owned(image,child);let(asset,p)=match turn.result{Ok(result)=>result,Err((error,image))=>{owned.image=Some(image);return Err(error.with_retained_progress(progress(0,0)));}};receipt.retained_capacity_bytes+=p.retained_capacity_bytes;
                RasterMutation::AddLayerAsset(add_layer_asset::AddLayerAsset {asset_id:owned.key.clone(),asset})
            }
            Row::AttachPrior=>RasterMutation::ChangeLayerMask(change_layer_mask::ChangeLayerMask {layer_id:owned.layer_id.clone(),expected:owned.prior.clone(),mask:Some(replacement(&owned.key,width,height))}),
            Row::AttachEmpty=>RasterMutation::ChangeLayerMask(change_layer_mask::ChangeLayerMask {layer_id:owned.layer_id.clone(),expected:None,mask:Some(replacement(&owned.key,width,height))}),
            Row::Unused=>return Err(refused("raster.mask-row-invalid")),
        };
        owned.rows.push(mutation);self.row+=1;if self.row==self.plan_len{self.phase=Phase::Ready;}Ok(receipt)
    }
    fn destination(&self,document:&RasterSnapshot)->Result<std::sync::Arc<SemioImageSnapshot>,ValueError>{document.assets.get(&self.owned()?.key).and_then(|child|child.local_owner::<SemioImageSnapshot>()).ok_or_else(||refused("raster.mask-destination-unavailable"))}
    pub fn advance(&mut self,document:&RasterSnapshot,config:&RasterConfig,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.failed{return Err(refused("raster.mask-owner-failed"));}let result=self.advance_one(document,config,grant);if result.is_err(){self.failed=true;}result
    }
    fn advance_one(&mut self,document:&RasterSnapshot,config:&RasterConfig,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let demand=self.demands(grant.maximum_copy_bytes)?;if !funded(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}self.original(document,config)?;
        let mut receipt=progress(0,0);
        match self.phase {
            Phase::Discover=>receipt=self.discover(document,grant)?,
            Phase::Bind=>{
                let node=at(&document.layers,&self.path[..=self.depth]).ok_or_else(||refused("raster.mask-layer-missing"))?;
                let (prior,width,height,display_width,display_height)=header(self.owned()?,node,document,config).map_err(|error|error.with_retained_progress(progress(self.bind_bytes-self.bind_capacity,0)))?;
                self.owned.as_mut().unwrap().prior=prior.clone();receipt=progress(self.bind_bytes,self.owned()?.prior.as_ref().and_then(|mask|mask.image_key.as_ref()).map_or(0,String::capacity));
                self.width=width;self.height=height;self.display_width=display_width;self.display_height=display_height;self.span_count=config.pixel_selection.as_ref().unwrap().spans.len();self.span_depth=if self.span_count==0{1}else{config.pixel_selection.as_ref().unwrap().spans.rank_observation_demand(0)?.depth};self.phase=Phase::Header;
            }
            Phase::Header=>{
                let mut schema=String::new();schema.try_reserve_exact(STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.len()).map_err(|_|refused("raster.mask-schema-reservation"))?;schema.push_str(STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA);
                self.owned.as_mut().unwrap().image=Some(SemioImageSnapshot {schema,width:self.width,height:self.height,colorspace:SemioColorspace::Rgba,bit_depth:8,frames:Vec::new(),icc:None,metadata:Vec::new()});
                let image=self.owned.as_mut().unwrap().image.as_mut().unwrap();image.frames.try_reserve_exact(1).map_err(|_|refused("raster.mask-frame-reservation").with_retained_progress(progress(demand.copy_bytes,image.schema.capacity())))?;
                image.frames.push(SemioImageFrame {delay_ms:0,rgba8:Vec::new()});receipt=progress(demand.copy_bytes,image.schema.capacity()+image.frames.capacity()*size_of::<SemioImageFrame>());self.phase=Phase::Reserve;
            }
            Phase::Reserve=>{let count=self.count()*4;let pixels=&mut self.owned.as_mut().unwrap().image.as_mut().unwrap().frames[0].rgba8;pixels.try_reserve_exact(count).map_err(|_|refused("raster.mask-pixel-reservation"))?;receipt.retained_capacity_bytes=pixels.capacity();self.phase=Phase::Pixels;}
            Phase::Pixels=>{
                let budget=grant.maximum_copy_bytes.min(32768);let size=size_of::<semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::RasterSelectionSpan>();
                while self.pixel<self.count()&&budget-receipt.copied_bytes>=4{
                    if self.active.is_none()&&self.span<self.span_count{
                        if budget-receipt.copied_bytes<size+4{break;}
                        let span=config.pixel_selection.as_ref().unwrap().spans.get(self.span).ok_or_else(||refused("raster.mask-span-missing").with_retained_progress(receipt))?;
                        self.active=Some((span.start as usize,(span.start+span.length) as usize,span.coverage as u8));receipt.copied_bytes+=size;
                    }
                    let mut end=self.count().min(self.pixel+(budget-receipt.copied_bytes)/4);
                    if let Some((_,last,_))=self.active{end=end.min(last);}
                    let pixels=&mut self.owned.as_mut().unwrap().image.as_mut().unwrap().frames[0].rgba8;
                    while self.pixel<end{let coverage=self.active.filter(|(start,_,_)|self.pixel>=*start).map_or(0,|(_,_,coverage)|coverage);pixels.extend_from_slice(&[255,255,255,coverage]);self.pixel+=1;receipt.copied_bytes+=4;}
                    if self.active.is_some_and(|(_,last,_)|self.pixel==last){self.active=None;self.span+=1;}else{break;}
                }
                if self.pixel==self.width as usize*self.height as usize{self.hash=Some(RasterImageContentHash::new());self.phase=Phase::Hash;}
            }
            Phase::Hash=>{let result=self.hash.as_mut().unwrap().advance(self.owned.as_ref().unwrap().image.as_ref().unwrap(),grant.maximum_copy_bytes.min(32768)).map_err(refused)?;receipt.copied_bytes=result.processed_bytes;if result.done{self.digest=self.hash.take().unwrap().finish();self.phase=Phase::Key;}}
            Phase::Key=>{let key=raster_image_key(self.digest.ok_or_else(||refused("raster.mask-hash-incomplete"))?);receipt=progress(RASTER_IMAGE_KEY_BYTES,key.capacity());self.owned.as_mut().unwrap().key=key;self.path=[0;33];self.locked=[false;33];self.depth=0;self.phase=Phase::References;}
            Phase::References=>{self.next_reference(document).map_err(|error|error.with_retained_progress(progress(demand.copy_bytes,0)))?;receipt.copied_bytes=demand.copy_bytes;}
            Phase::Destination=>{
                let image=self.destination(document)?;let end=(self.count()*4).min(self.verified+grant.maximum_copy_bytes.min(32768)/2);let start=self.verified;
                let destination=image.frames.first().and_then(|frame|frame.rgba8.get(start..end)).ok_or_else(||refused("raster.mask-destination-conflict"))?;
                let expected=&self.owned.as_ref().unwrap().image.as_ref().unwrap().frames[0].rgba8[start..end];receipt.copied_bytes=(end-start)*2;
                if destination!=expected{return Err(refused("raster.mask-destination-conflict").with_retained_progress(receipt));}
                self.verified=end;if end==self.count()*4{self.destination_verified=true;self.phase=Phase::Plan;}
            }
            Phase::Plan=>self.plan(document)?,
            Phase::RowsReserve=>{let rows=&mut self.owned.as_mut().unwrap().rows;rows.try_reserve_exact(self.plan_len).map_err(|_|refused("raster.mask-row-reservation"))?;receipt.retained_capacity_bytes=rows.capacity()*size_of::<RasterMutation>();self.phase=Phase::Rows;}
            Phase::Rows=>receipt=self.build_row(grant)?,
            Phase::Ready=>{},
            Phase::Taken=>return Err(refused("raster.mask-output-already-taken")),
        }
        if !receipt.fits(grant){return Err(refused("raster.mask-resource-receipt").with_retained_progress(receipt));}
        Ok(if self.phase==Phase::Ready{RetainedCloneStep::Complete(receipt)}else{RetainedCloneStep::Progress(receipt)})
    }
    pub fn take_emit(&mut self)->Option<Emit<RasterMutation,RasterConfigMutation>>{if self.closing||self.failed||self.phase!=Phase::Ready{return None;}self.phase=Phase::Taken;Some(Emit::mutations(std::mem::take(&mut self.owned.as_mut()?.rows)))}
    pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.hash.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<RasterImageContentHash>>(),depth:1,..Default::default()});}if let Some(active)=self.retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(active,body);}if self.owned.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.owned);}Ok(Default::default())}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{self.closing=true;if self.hash.is_some(){let demand=RetirementDemand{copy_bytes:size_of::<Option<RasterImageContentHash>>(),depth:1,..Default::default()};if !funded(grant,demand){return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}self.hash=None;return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress(demand.copy_bytes,0))),work:1};}if self.retirement.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant)}else{store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.owned,&mut self.retirement,grant)}}
    pub fn terminal_is_empty(&self)->bool{self.closing&&self.owned.is_none()&&self.retirement.is_none()}
}

fn replacement(key:&str,width:u32,height:u32)->RasterLayerMask{RasterLayerMask {enabled:true,linked:true,invert:false,width:Some(width),height:Some(height),image_key:Some(key.into()),transform:RasterTransform::default()}}
impl Drop for MaskSelectionWork {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"selection mask must hand off or retire every owner before Drop");}}

#[derive(Default)]
pub struct MaskFromSelectionWork {candidate:Option<MaskSelectionWork>,complete:bool}
impl<A:RasterWorkApp> ArtifactCommandWork<A> for MaskFromSelectionWork {
    fn tool_id(&self)->&'static str{"maskFromSelection"}
    fn extent(&self,command:&RasterCommand,_snapshot:&RasterSnapshot,_interaction:&protocol::InteractionState,_context:Option<&semio_plugin::app::ArtifactOwnedToolJobContext<A>>)->Option<usize>{matches!(command,RasterCommand::MaskFromSelection(_)).then_some(1)}
    fn work_demands(&self,input:&ArtifactCommandInputs<'_,A>,body:usize)->Result<RetirementDemand,ValueError>{if let Some(candidate)=&self.candidate{return candidate.demands(body);}let RasterCommand::MaskFromSelection(command)=input.command else{return Err(refused("raster.mask-work-mismatch"));};MaskSelectionWork::begin_demands(command,input.snapshot,input.config)}
    fn step(&mut self,input:&ArtifactCommandInputs<'_,A>,cx:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactCommandWorkStep<A>,semio_framework_job::OriginalTurnFailure>{
        if cx.is_cancelled(){return Err(refused("raster.mask-cancelled").into());}if self.complete{return Err(refused("raster.mask-work-complete").into());}if cx.should_yield(){return Ok(ArtifactCommandWorkStep::Pending {stage:"mask-yield",preview:b""});}
        let RasterCommand::MaskFromSelection(command)=input.command else{return Err(refused("raster.mask-work-mismatch").into());};
        let result=if let Some(candidate)=self.candidate.as_mut(){candidate.advance(input.snapshot,input.config,cx.retained_grant())}else{MaskSelectionWork::begin(command,input.snapshot,input.config,cx.retained_grant()).map(|started|{if let Some((candidate,receipt))=started{self.candidate=Some(candidate);RetainedCloneStep::Progress(receipt)}else{RetainedCloneStep::Progress(Default::default())}})};
        cx.consume_fuel(1);let step=match result{Ok(step)=>step,Err(error)=>return Err(semio_framework_job::OriginalTurnFailure::debit_original(error,cx))};let receipt=step.progress();cx.consume_retained(receipt)?;
        if matches!(step,RetainedCloneStep::Complete(_)){self.complete=true;return Ok(ArtifactCommandWorkStep::Complete(self.candidate.as_mut().unwrap().take_emit().ok_or_else(||refused("raster.mask-output-missing"))?));}
        Ok(if receipt==RetainedCloneProgress::default(){ArtifactCommandWorkStep::Pending {stage:"mask-capacity",preview:b""}}else{ArtifactCommandWorkStep::Progress {stage:"mask-create",preview:br#"{"en":"Creating selection mask","de":"Auswahlmaske wird erstellt"}"#}})
    }
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        let Some(candidate)=self.candidate.as_mut()else{return Ok(semio_framework_job::InteractiveJobCloseStep::Complete {progress:Default::default()});};let turn=candidate.close_step(cx.retained_grant());cx.consume_fuel(turn.work);let step=turn.result.map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;let progress=step.progress();if candidate.terminal_is_empty(){self.candidate=None;Ok(semio_framework_job::InteractiveJobCloseStep::Complete {progress})}else{Ok(semio_framework_job::InteractiveJobCloseStep::Pending {progress})}
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.candidate.as_ref().map(|candidate|candidate.close_demands(32768)).transpose()?.unwrap_or_default().copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.candidate.as_ref().map(|candidate|candidate.close_demands(body)).transpose()?.unwrap_or_default().capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.candidate.as_ref().map(|candidate|candidate.close_demands(32768)).transpose()?.unwrap_or_default().release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.candidate.as_ref().map(|candidate|candidate.close_demands(32768)).transpose()?.unwrap_or_default().depth)}
    fn terminal_frame_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn terminal_is_empty(&self)->bool{self.candidate.is_none()}
}
