//! 🔒 Shared finite-value, index, permutation, and rebasing mechanics for material/animation leaves.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GltfMaterialAnimationFailure {
    pub code: &'static str,
    pub path: String,
    pub detail: &'static str,
}

pub fn index<T>(items: &[T], value: usize, path: impl Into<String>) -> Result<(), GltfMaterialAnimationFailure> {
    (value < items.len()).then_some(()).ok_or(GltfMaterialAnimationFailure { code: "gltf.mutation.index-out-of-range", path: path.into(), detail: "the addressed index must exist" })
}

pub fn position(length: usize, value: usize, path: impl Into<String>) -> Result<(), GltfMaterialAnimationFailure> {
    (value <= length).then_some(()).ok_or(GltfMaterialAnimationFailure { code: "gltf.mutation.position-out-of-range", path: path.into(), detail: "the insertion position must be within the relation" })
}

pub fn finite(value: f64, path: impl Into<String>) -> Result<(), GltfMaterialAnimationFailure> {
    value.is_finite().then_some(()).ok_or(GltfMaterialAnimationFailure { code: "gltf.mutation.non-finite-value", path: path.into(), detail: "the numeric value must be finite" })
}

pub fn permutation(order: &[usize], length: usize, path: impl Into<String>) -> Result<(), GltfMaterialAnimationFailure> {
    let path = path.into();
    if order.len() != length {
        return Err(GltfMaterialAnimationFailure { code: "gltf.mutation.invalid-permutation", path, detail: "the order must cover every current member exactly once" });
    }
    let mut seen = vec![false; length];
    for &value in order {
        if value >= length || std::mem::replace(&mut seen[value], true) {
            return Err(GltfMaterialAnimationFailure { code: "gltf.mutation.invalid-permutation", path: path.clone(), detail: "the order must cover every current member exactly once" });
        }
    }
    Ok(())
}

pub fn rebase_move(reference: usize, from: usize, to: usize) -> usize {
    if reference == from {
        to
    } else if from < to && (from < reference && reference <= to) {
        reference - 1
    } else if to < from && (to <= reference && reference < from) {
        reference + 1
    } else {
        reference
    }
}
