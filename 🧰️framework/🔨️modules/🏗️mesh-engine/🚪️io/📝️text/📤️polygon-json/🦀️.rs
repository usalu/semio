//! 📤️ Borrows polygon fields through Pack's measured native JSON writer.

use super::*;
use semio_framework_pack_json::{JsonBorrowContext,JsonBorrowNoContext,JsonBorrowNode,JsonBorrowedWriteCursor,
    JsonNativeBorrow,JsonNativeSource,JsonWriteNode,json_borrow_absent,json_borrow_native};
use semio_framework_value::{NativeEncodeControl,RetainedAllocationEffect,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;

const COPY_BYTES:usize=4096;

struct Map<'a,V>(&'a HistoryFoldIndex<String,V>);
impl<V> Map<'_,V>{fn is_empty(&self)->bool{self.0.is_empty()}}
impl<V:JsonNativeBorrow> JsonNativeBorrow for Map<'_,V>{
    fn json_node<'a>(&'a self,mode:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{
        if mode!=0{return Err(json_borrow_absent());}
        Ok(JsonWriteNode::Object(self.0.len()))
    }
    fn json_child<'a>(&'a self,mode:u8,index:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,ValueError>{
        self.0.ranked(index).filter(|_|mode==0).map(|(_,value)|json_borrow_native(value)).ok_or_else(json_borrow_absent)
    }
    fn json_key<'a>(&'a self,mode:u8,index:usize,_:&'a dyn JsonBorrowContext)->Result<&'a str,ValueError>{
        self.0.ranked(index).filter(|_|mode==0).map(|(name,_)|name.as_str()).ok_or_else(json_borrow_absent)
    }
}

#[derive(semio_framework_value::NativeJsonBorrow)]
struct PolygonView<'a> {
    vertices:&'a Vec<[f32;3]>,
    faces:&'a Vec<Vec<u32>>,
    #[value(skip_serializing_if="Map::is_empty")]
    attributes:Map<'a,MeshAttribute>,
    #[value(skip_serializing_if="Map::is_empty")]
    materials:Map<'a,pack::value::DslValue>,
    #[value(skip_serializing_if="Map::is_empty")]
    textures:Map<'a,MeshTexture>,
    #[value(skip_serializing_if="surface_empty")]
    surface:&'a Vec<PolygonSurfaceSample>,
}

fn surface_empty(rows:&&Vec<PolygonSurfaceSample>)->bool {rows.is_empty()}

impl<'a> From<&'a PolygonMeshSource> for PolygonView<'a>{
    fn from(source:&'a PolygonMeshSource)->Self{
        Self{vertices:&source.vertices,faces:&source.faces,attributes:Map(&source.attributes),
            materials:Map(&source.materials),textures:Map(&source.textures),surface:&source.surface}
    }
}

/// 🧵️ Retains polygon JSON measurement, allocation and writing across funded turns.
#[derive(semio_framework_value_derive::RetireOwned)]
pub struct PolygonJsonExport {
    source:PolygonMeshSource,
    writer:JsonBorrowedWriteCursor,
    measured:usize,
    cancelled:bool,
}

impl PolygonJsonExport {
    /// 🌱️ Moves the captured source into an empty writer.
    pub fn new(source:PolygonMeshSource)->Self{
        Self{source,writer:JsonBorrowedWriteCursor::new(),measured:0,cancelled:false}
    }

    /// 🛑️ Prevents partial text from being published.
    pub fn cancel(&mut self){self.cancelled=true;}

    /// 📊️ Measures progress through the two passes without inspecting source payloads.
    pub fn progress(&self)->f32{
        let (bytes,writing)=self.writer.progress();
        if writing {0.5+0.49*(bytes as f32/self.measured.max(1) as f32)}
        else {0.49*(1.0-1.0/(1.0+bytes as f32/65536.0))}
    }

    /// 📏️ Quotes the next native formatting or output-allocation event.
    pub fn demand(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if self.cancelled{return Ok(Default::default());}
        let view=PolygonView::from(&self.source);
        self.writer.normal_step_demands_with_copy(&JsonNativeSource{root:&view,context:&JsonBorrowNoContext},copy.min(COPY_BYTES))
    }

    /// 🪜️ Advances one settled event and transfers the final text once.
    pub fn step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<(Option<String>,RetainedCloneProgress),ValueError>{
        if self.cancelled{return Err(ValueError::literal(ValueRefusalKind::Canceled,"polygon JSON export cancelled"));}
        let mut grant=effect.grant();
        grant.maximum_copy_bytes=grant.maximum_copy_bytes.min(COPY_BYTES);
        let demand=self.demand(grant.maximum_copy_bytes)?;
        if !retained_clone_grant_admits(grant,demand)||!effect.performed(1)?{return Ok((None,Default::default()));}
        if let Some(bytes)=self.writer.measured_bytes(){self.measured=bytes;}
        let view=PolygonView::from(&self.source);
        let mut accepted=|_|true;
        let mut control=NativeEncodeControl::new_retained(&mut accepted);
        let result=self.writer.step(&JsonNativeSource{root:&view,context:&JsonBorrowNoContext},1,&mut control,grant);
        let receipt=self.writer.normal_step_progress();
        if receipt.retained_capacity_bytes!=0{*effect.actual()=Some(receipt.retained_capacity_bytes);}
        result.map(|text|(text,receipt))
    }
}
