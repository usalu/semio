//! 🧮️ Prepares decoded glTF geometry input at the physical accessor boundary.
use crate::schema::snapshot::{GltfSnapshot,GltfDecodedSnapshot};
use crate::standards::v2_0::subsets::any::schema::inferences::{GltfInference,GltfGeometricInference};

/// ⛽️ Reports each accessor boundary and refuses cancellation before decoding the next owner.
pub fn project_gltf_inference<'a>(snapshot:&'a GltfSnapshot,progress:&mut impl FnMut(usize,usize)->bool)->Result<GltfDecodedSnapshot<'a>,String> {
    let mut accessors=Vec::with_capacity(snapshot.document.accessors.len());
    let total=snapshot.document.accessors.len()+snapshot.buffers.len();
    for index in 0..snapshot.document.accessors.len() {
        if !progress(index,total) { return Err("glTF inference projection cancelled".into()); }
        accessors.push(crate::standards::v2_0::subsets::any::io::decode_accessor(&snapshot.document,&snapshot.buffers,index));
    }
    let mut buffer_fingerprints=Vec::with_capacity(snapshot.buffers.len());
    for (index,bytes) in snapshot.buffers.iter().enumerate() {
        if !progress(snapshot.document.accessors.len()+index,total) { return Err("glTF inference projection cancelled".into()); }
        buffer_fingerprints.push(format!("buffer:{index}:{}",crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::byte_fingerprint(bytes)));
    }
    if !progress(total,total) { return Err("glTF inference projection cancelled".into()); }
    Ok(GltfDecodedSnapshot { document:&snapshot.document,accessors,buffer_fingerprints })
}

/// 🧠️ Projects the exact authored snapshot before invoking the pure inference DAG.
pub fn compute_gltf_inference(snapshot:&GltfSnapshot)->GltfGeometricInference {
    let input=project_gltf_inference(snapshot,&mut |_,_|true).expect("uncancelled glTF inference projection");
    crate::standards::v2_0::subsets::any::schema::inferences::compute_gltf_inference(&input)
}
impl protocol::Inference<GltfSnapshot> for GltfInference {
    fn infer(snapshot: &GltfSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { geometry: compute_gltf_inference(snapshot) }
    
        })
    }
}


#[cfg(test)]
#[path = "🧪️tests/🧮️decoded-accessor/🦀️.rs"]
mod projection_tests;
