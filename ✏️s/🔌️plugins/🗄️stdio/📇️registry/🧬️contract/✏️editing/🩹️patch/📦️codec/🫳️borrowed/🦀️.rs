//! 🩹️ Original patch fields expose their authored JSON occurrences without a value-tree mirror.
use super::SnapshotPatch;
use semio_framework_pack_json::{JsonWriteNode,JsonWriteSource};
use semio_framework_value::{DslValue,Number,ValueError,ValueRefusalKind};
use super::kernel::operation_bytes::OperationByteOutput;

enum Member<'source>{Text(&'source str),UInt(u64),Bool(bool),Value(&'source DslValue)}
impl SnapshotPatch{
    fn json_member(&self,index:usize)->Option<(&'static str,Member<'_>)>{
        if index==0{return Some(("operation",Member::Text(self.operation())));}
        match(self,index-1){
            (Self::Set{path,..}|Self::Insert{path,..}|Self::Remove{path}|Self::Rename{path,..}|Self::Splice{path,..},0)=>Some(("path",Member::Text(path))),
            (Self::Set{value,..}|Self::Insert{value,..},1)=>Some(("value",Member::Value(value))),
            (Self::Insert{index:Some(index),..},2)=>Some(("index",Member::UInt(*index))),
            (Self::Move{from,..},0)=>Some(("from",Member::Text(from))),
            (Self::Move{path,..},1)=>Some(("path",Member::Text(path))),
            (Self::Move{index:Some(index),..},2)=>Some(("index",Member::UInt(*index))),
            (Self::Rename{key,..},1)=>Some(("key",Member::Text(key))),
            (Self::Splice{offset,..},1)=>Some(("offset",Member::UInt(*offset))),
            (Self::Splice{remove,..},2)=>Some(("remove",Member::UInt(*remove))),
            (Self::Splice{value,..},3)=>Some(("value",Member::Value(value))),
            (Self::Splice{continued:true,..},4)=>Some(("continued",Member::Bool(true))),
            _=>None,
        }
    }
    /// ✍️ Streams the original JSON operation through the caller's complete byte and collection policy.
    pub fn encode_op_into(&self,options:&super::kernel::codec::PackEncodeOptions,output:&mut dyn super::kernel::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),super::kernel::ProtocolError>{
        super::kernel::operation_bytes::with_operation_encode_policy(options,control,|control|{
        let maximum=options.limits.max_file_len.min(super::SNAPSHOT_PATCH_MAX_BYTES as u64);
        let mut limited=super::kernel::operation_bytes::OperationByteLimitedOutput::new(output,maximum);
        semio_framework_pack_json::write_json_source_into::<_,super::kernel::PackRefusal,_>(self,maximum,usize::from(options.limits.max_depth),options.limits.max_items,&mut|bytes:&[u8],control:&mut semio_framework_value::NativeEncodeControl<'_>|limited.write_bytes(bytes,control),control)?;
        Ok(())
        })
    }
}
impl JsonWriteSource for SnapshotPatch{
    fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{
        if path.is_empty(){let mut length=0;while self.json_member(length).is_some(){length+=1;}return Ok(JsonWriteNode::Object(length));}
        let (_,member)=self.json_member(path[0]).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"patch JSON member ordinal is absent"))?;
        if let Member::Value(value)=member{return value.node_at_path(&path[1..]);}
        if path.len()!=1{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"patch scalar has no child ordinal"));}
        Ok(match member{Member::Text(value)=>JsonWriteNode::String(value),Member::UInt(value)=>JsonWriteNode::Number(Number::UInt(value)),Member::Bool(value)=>JsonWriteNode::Bool(value),Member::Value(_)=>unreachable!()})
    }
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
        if path.is_empty(){return self.json_member(index).map(|(key,_)|key).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"patch JSON key ordinal is absent"));}
        match self.json_member(path[0]){Some((_,Member::Value(value)))=>value.object_key_at_path(&path[1..],index),_=>Err(ValueError::new(ValueRefusalKind::InvariantViolated,"patch JSON key owner is absent"))}
    }
}

impl semio_framework_plugin::plugin_app_close_prelude::store::ArtifactCanonicalJson for SnapshotPatch {
    fn canonical_json_node(&self, path: &[usize]) -> Result<semio_framework_plugin::plugin_app_close_prelude::store::ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
        use semio_framework_plugin::plugin_app_close_prelude::store::ArtifactCanonicalJsonNode as N;
        Ok(match self.node_at_path(path)? {
            JsonWriteNode::Null => N::Null,
            JsonWriteNode::Bool(value) => N::Bool(value),
            JsonWriteNode::Number(Number::Int(value)) => N::I64(value),
            JsonWriteNode::Number(Number::UInt(value)) => N::U64(value),
            JsonWriteNode::Number(Number::Float(value)) => N::F64(value),
            JsonWriteNode::String(value) => N::String(value),
            JsonWriteNode::NativeString(_) => return Err("authored SnapshotPatch str/DslValue source cannot yield native UTF8 text".into()),
            JsonWriteNode::Array(length) => N::Array(length),
            JsonWriteNode::Object(length) => N::Object(length),
        })
    }
    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<semio_framework_plugin::plugin_app_close_prelude::store::ArtifactCanonicalJsonText<'_>, semio_framework_value::ValueError> { self.object_key_at_path(path, index).map(Into::into) }
}

#[path="../📥️decode/🫳️borrowed/🦀️.rs"]
mod borrowed_operation_read;
pub use borrowed_operation_read::SnapshotPatchReadCursor;
