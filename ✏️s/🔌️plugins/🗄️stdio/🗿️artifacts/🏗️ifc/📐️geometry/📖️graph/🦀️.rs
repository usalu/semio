//! 🌐️ Borrowed, standard-neutral representation graph reads.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeometryStandard { Ifc2x3, Ifc4 }

/// 🔤️ Reads intrinsic source values without building another artifact's snapshot.
pub trait GeometryValue {
    fn as_reference(&self) -> Option<u64>;
    fn as_str(&self) -> Option<&str>;
    fn as_enum(&self) -> Option<&str>;
    fn as_real(&self) -> Option<f64>;
    fn as_integer(&self) -> Option<i64>;
    fn as_aggregate(&self) -> Option<&[Self]> where Self: Sized;
    fn as_typed(&self)->Option<(&str,&[Self])> where Self:Sized;
    fn is_absent(&self) -> bool;
    fn is_unset(&self) -> bool;
    fn is_derived(&self) -> bool;
}

/// 🧩️ One original type constituent and its borrowed positional values.
pub struct GeometryConstituent<'a, V> {pub name: &'a str, pub args: &'a [V]}

/// 🆔️ One original instance and its primary constituent, retaining complex cardinality.
pub struct GeometryEntity<'a, V> {pub id: u64, pub name: &'a str, pub args: &'a [V], pub constituent_count: usize}

impl<V> Copy for GeometryEntity<'_, V> {}
impl<V> Clone for GeometryEntity<'_, V> {fn clone(&self) -> Self { *self }}

/// 📚️ An indexed native graph with explicit standard capabilities.
pub trait GeometryGraph {
    type Value: GeometryValue;
    fn standard(&self) -> GeometryStandard;
    fn len(&self) -> usize;
    fn source_identity(&self) -> usize;
    fn entity(&self, index: usize) -> Option<GeometryEntity<'_, Self::Value>>;
    fn constituent(&self, instance: usize, index: usize) -> Option<GeometryConstituent<'_, Self::Value>>;
}

#[path="🪧️part21/🦀️.rs"]
mod part21;
