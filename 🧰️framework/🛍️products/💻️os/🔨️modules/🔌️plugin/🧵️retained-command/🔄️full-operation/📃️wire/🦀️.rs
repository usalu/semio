//! 📃️ Synchronous wire views borrow unchanged publication payloads without projecting owned JSON trees.
use semio_framework::kernel::UiDirtyScope;
use semio_framework_pack_json::{JsonWriteNode,JsonWriteSource};
use semio_framework_value::{NativeEncodeControl,Number,ValueError,ValueRefusalKind};

/// 🫳️ Finite publication fields remain owned by their semantic producer throughout framing.
pub enum PublicationWire<'a>{
    LaneReceipt{generation_before:u64,generation_after:u64},
    Accepted(u64),
    Ui(&'a UiDirtyScope),
    Download{filename:&'a str,mime:&'a str,encoding:Option<&'a str>,bytes:u64},
}

/// 📏️ A refused write preserves the exact accepted prefix and its caller-owned storage.
pub struct PublicationWireWrite{pub written:usize,pub work:u64,pub result:Result<(),ValueError>}
struct CountedSource<'a,S>{source:&'a S,work:&'a std::cell::Cell<u64>}
impl<S:JsonWriteSource> JsonWriteSource for CountedSource<'_,S>{
 fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{self.work.set(self.work.get()+1);self.source.node_at_path(path)}
 fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{self.work.set(self.work.get()+1);self.source.object_key_at_path(path,index)}
}

fn absent()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"publication wire path is absent")}
fn number(value:u64)->JsonWriteNode<'static>{JsonWriteNode::Number(Number::UInt(value))}

fn ui_key(scope:&UiDirtyScope,index:usize)->Option<&'static str>{
    match scope{
        UiDirtyScope::Full|UiDirtyScope::None=>(index==0).then_some("kind"),
        UiDirtyScope::Partial{window_bodies,panel_bodies,..}=>[
            ("kind",true),("windowBodies",!window_bodies.is_empty()),("panelBodies",!panel_bodies.is_empty()),
            ("utilities",true),("tools",true),("engagements",true),("measures",true),("labels",true),
        ].into_iter().filter(|(_,present)|*present).nth(index).map(|(key,_)|key),
    }
}

fn ui_node<'a>(scope:&'a UiDirtyScope,path:&[usize])->Result<JsonWriteNode<'a>,ValueError>{
    if path.is_empty(){return Ok(JsonWriteNode::Object(match scope{UiDirtyScope::Partial{window_bodies,panel_bodies,..}=>6+usize::from(!window_bodies.is_empty())+usize::from(!panel_bodies.is_empty()),_=>1}))}
    let key=ui_key(scope,path[0]).ok_or_else(absent)?;
    match (scope,key,path){
        (UiDirtyScope::Full,"kind",[_])=>Ok(JsonWriteNode::String("full")),
        (UiDirtyScope::None,"kind",[_])=>Ok(JsonWriteNode::String("none")),
        (UiDirtyScope::Partial{..},"kind",[_])=>Ok(JsonWriteNode::String("partial")),
        (UiDirtyScope::Partial{window_bodies,..},"windowBodies",[_])=>Ok(JsonWriteNode::Array(window_bodies.len())),
        (UiDirtyScope::Partial{panel_bodies,..},"panelBodies",[_])=>Ok(JsonWriteNode::Array(panel_bodies.len())),
        (UiDirtyScope::Partial{window_bodies,..},"windowBodies",[_,index])=>window_bodies.get(*index).map(|text|JsonWriteNode::String(text)).ok_or_else(absent),
        (UiDirtyScope::Partial{panel_bodies,..},"panelBodies",[_,index])=>panel_bodies.get(*index).map(|text|JsonWriteNode::String(text)).ok_or_else(absent),
        (UiDirtyScope::Partial{utilities,..},"utilities",[_])=>Ok(JsonWriteNode::Bool(*utilities)),
        (UiDirtyScope::Partial{tools,..},"tools",[_])=>Ok(JsonWriteNode::Bool(*tools)),
        (UiDirtyScope::Partial{engagements,..},"engagements",[_])=>Ok(JsonWriteNode::Bool(*engagements)),
        (UiDirtyScope::Partial{measures,..},"measures",[_])=>Ok(JsonWriteNode::Bool(*measures)),
        (UiDirtyScope::Partial{labels,..},"labels",[_])=>Ok(JsonWriteNode::Bool(*labels)),
        _=>Err(absent()),
    }
}

impl JsonWriteSource for PublicationWire<'_>{
    fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{
        match(self,path){
            (Self::LaneReceipt{..},[])=>Ok(JsonWriteNode::Object(2)),
            (Self::LaneReceipt{generation_before,..},[0])=>Ok(number(*generation_before)),
            (Self::LaneReceipt{generation_after,..},[1])=>Ok(number(*generation_after)),
            (Self::Accepted(_),[])=>Ok(JsonWriteNode::Array(2)),
            (Self::Accepted(_),[0])=>Ok(JsonWriteNode::String("accepted")),
            (Self::Accepted(count),[1])=>Ok(number(*count)),
            (Self::Ui(scope),path)=>ui_node(scope,path),
            (Self::Download{..},[])=>Ok(JsonWriteNode::Array(4)),
            (Self::Download{filename,..},[0])=>Ok(JsonWriteNode::String(filename)),
            (Self::Download{mime,..},[1])=>Ok(JsonWriteNode::String(mime)),
            (Self::Download{encoding,..},[2])=>Ok(encoding.map_or(JsonWriteNode::Null,JsonWriteNode::String)),
            (Self::Download{bytes,..},[3])=>Ok(number(*bytes)),
            _=>Err(absent()),
        }
    }
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
        if !path.is_empty(){return Err(absent())}
        match self{
            Self::LaneReceipt{..}=>["generationBefore","generationAfter"].get(index).copied().ok_or_else(absent),
            Self::Ui(scope)=>ui_key(scope,index).ok_or_else(absent),
            _=>Err(absent()),
        }
    }
}

impl PublicationWire<'_>{
    /// ✍️ Writes a bounded prefix into already-admitted storage under the caller's original control.
    pub fn write_into(&self,output:&mut[u8],maximum_depth:usize,maximum_items:u64,control:&mut NativeEncodeControl<'_>)->PublicationWireWrite{
        let maximum=output.len()as u64;
        let mut written=0usize;let work=std::cell::Cell::new(0u64);let source=CountedSource{source:self,work:&work};
        let result=semio_framework_pack_json_source_sink::write_json_source_into(&source,maximum,maximum_depth,maximum_items,&mut|bytes:&[u8],_:&mut NativeEncodeControl<'_>|{
            let end=written.checked_add(bytes.len()).filter(|end|*end<=output.len()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"publication wire slice is full"))?;
            output[written..end].copy_from_slice(bytes);written=end;work.set(work.get()+1);Ok::<(),ValueError>(())
        },control).map(|_|());
        PublicationWireWrite{written,work:work.get(),result}
    }
}

/// 📤️ Framed fault and terminal octets bypass JSON quoting while retaining the caller's exact prefix on refusal.
pub fn write_publication_raw_into(source:&[u8],output:&mut[u8],control:&mut NativeEncodeControl<'_>)->PublicationWireWrite{
    let mut written=0usize;let work=std::cell::Cell::new(0u64);
    let result=control.scoped_stage(|control|{
        control.begin_stage(source.len())?;
        for bytes in source.chunks(64){
            control.checkpoint()?;
            let end=written.checked_add(bytes.len()).filter(|end|*end<=output.len()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"publication raw slice is full"))?;
            output[written..end].copy_from_slice(bytes);written=end;work.set(work.get()+1);control.advance(bytes.len())?;
        }
        Ok(())
    });
    PublicationWireWrite{written,work:work.get(),result}
}

