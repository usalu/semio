//! 🎒️ A full Pack Value record borrows the mutation's ordered intrinsic fields.
use semio_framework_dsl_record::native_encoding::{FieldProjectionSource,FieldProjectionView as View,projection_path_error};
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node,ArtifactCanonicalJsonText as Text,ArtifactCanonicalJsonTree};
use semio_framework_value::{Number,ValueError,ValueRefusalKind};

#[repr(transparent)]
struct PackValue<M>(M);

pub(super) fn source<M:ArtifactCanonicalJsonTree>(value:&M)->&dyn FieldProjectionSource{
    unsafe{&*(std::ptr::from_ref(value).cast::<PackValue<M>>())}
}

impl<M:ArtifactCanonicalJsonTree> PackValue<M>{
    fn intrinsic(&self,path:&[usize])->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{
        let Some((0,ordinals))=path.split_first() else{return Err(projection_path_error())};
        let mut value:&dyn ArtifactCanonicalJsonTree=&self.0;
        for ordinal in ordinals{value=value.canonical_tree_child(*ordinal)?;}
        Ok(value)
    }
}

impl<M:ArtifactCanonicalJsonTree> FieldProjectionSource for PackValue<M>{
    fn projection_view(&self,path:&[usize])->Result<View<'_>,ValueError>{
        if path.is_empty(){return Ok(View::Record(&[1]))}
        let value=self.intrinsic(path)?;
        if let Some(bytes)=value.canonical_tree_octets(){return Ok(View::IntrinsicBytes(bytes))}
        Ok(match value.canonical_tree_node()?{
            Node::Null=>View::IntrinsicNull,
            Node::Bool(value)=>View::IntrinsicBool(value),
            Node::I64(value)=>View::IntrinsicNumber(Number::Int(value)),
            Node::U64(value)=>View::IntrinsicNumber(Number::UInt(value)),
            Node::F32(value)=>View::IntrinsicNumber(Number::Float(f64::from(value))),
            Node::F64(value)=>View::IntrinsicNumber(Number::Float(value)),
            Node::String(text)|Node::Text(Text::Contiguous(text))=>View::IntrinsicText(text),
            Node::Array(length)=>View::IntrinsicArray(length),
            Node::Object(length)=>View::IntrinsicObject(length),
            _=>return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"Pack Value requires a native intrinsic field projection")),
        })
    }

    fn projection_key(&self,path:&[usize],ordinal:usize)->Result<&str,ValueError>{
        match self.intrinsic(path)?.canonical_tree_key(ordinal)?{
            Text::Contiguous(key)=>Ok(key),
            Text::Native(_)=>Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"Pack Value requires a contiguous intrinsic key projection")),
        }
    }
}
