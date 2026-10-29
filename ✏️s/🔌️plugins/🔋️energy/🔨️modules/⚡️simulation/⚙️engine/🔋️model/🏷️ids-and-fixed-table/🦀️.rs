//! 🏷️ The entity identifier, the glazing pane limit and the fixed-capacity table.

use super::*;

// #region 🔖️Ids
/// 🆔️ Stable internal entity identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EntityId(pub u32);

// 🌱️ Hand-written, not derived — `#[derive(ToValue, FromValue)]` only supports named-field
// structs, not a tuple struct like this one (see `semio-framework-value-derive`'s own docstring).
impl ToValue for EntityId {
    fn to_value_controlled(&self,c:&mut semio_framework_value::native_encoding::NativeEncodeControl<'_>)->Result<DslValue,ValueError>{self.0.to_value_controlled(c)}
    fn to_value(&self) -> DslValue {
        self.0.to_value()
    }
}
impl FromValue for EntityId {
    fn from_value_controlled(v:&DslValue,c:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,ValueError>{u32::from_value_controlled(v,c).map(Self)}
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        u32::from_value(value).map(EntityId)
    }
}

/// 📽️ Scalar id snapshot projection and retained clone: the id is a fixed-size copy root, owned by its defining crate.
impl semio_framework_value_snapshot_projection::snapshot_projection::ImmutableRootProjection for EntityId{
 fn root_projection_demand(_:semio_framework_value::RetainedCloneRef<'_,Self>)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<Self>()*2,depth:1,..Default::default()})}
 fn project_root(original:semio_framework_value::RetainedCloneRef<'_,Self>,grant:semio_framework_value::RetainedCloneGrant)->Result<Option<(Self,semio_framework_value::RetainedCloneProgress)>,semio_framework_value::ValueError>{let demand=Self::root_projection_demand(original)?;Ok(semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand).then(||(*original.get(),semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
}
impl semio_framework_value::retained_clone::RetainedClone for EntityId{
 type Cursor=semio_framework_value::retained_clone::ScalarCursor<Self>;
 fn retained_clone_cursor()->Self::Cursor{Default::default()}
}
/// 🛬️ Scalar id native receiving: one `u32` leaf read through the original receiving owner.
impl<I:semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource+?Sized> semio_framework_value_native_owned::native_decoding::owned::tree::NativeScalarValue<I> for EntityId{
 fn read(original:&I)->Result<Self,semio_framework_value::ValueError>{<u32 as semio_framework_value_native_owned::native_decoding::owned::tree::NativeScalarValue<I>>::read(original).map(Self)}
 fn read_copy_bytes(_: &I)->Result<usize,semio_framework_value::ValueError>{Ok(std::mem::size_of::<Self>())}
}
impl<I:semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource+?Sized> semio_framework_value_native_owned::native_decoding::NativeReceive<I> for EntityId{
 type Receiver=semio_framework_value_native_owned::native_decoding::owned::tree::NativeScalarReceiver<I,Self>;
 fn receive_birth_demand()->semio_framework_value::RetirementDemand{Self::Receiver::birth()}
 fn begin_receive(grant:semio_framework_value::RetainedCloneGrant)->Result<(Self::Receiver,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>{Self::Receiver::begin(grant)}
}

impl EntityId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

/// 🔗️ `EntityId` as a mutation-payload field — a bare unsigned number on every surface, so an
/// `id`-addressed mutation leaf can carry it directly instead of flattening it to `u32` by hand.
/// Hand-written for the same tuple-struct reason as the `ToValue` pair above.
impl semio_framework_dsl_record::DslField for EntityId {
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::UInt
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::UInt(u64::from(self.0))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        <u32 as semio_framework_dsl_record::DslField>::from_value(value).map(EntityId)
    }
}

semio_framework_value::artifact_retire_leaf!(EntityId);

impl semio_framework_pack_json::ArtifactCanonicalJsonTree for EntityId {
    fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, ValueError> {
        Ok(semio_framework_pack_json::ArtifactCanonicalJsonNode::U64(u64::from(self.0)))
    }
}
// #endregion 🔖️Ids

// #region 🔖️GlazingLimits
/// 🪟️ Largest glazing stack one glazing construction may alternate with gas gaps — the stack the
/// window heat balance (`crate::fenestration`) solves.
pub const MAX_PANES: usize = 4;
// #endregion 🔖️GlazingLimits

// #region 🔖️FixedTable
/// 🧺️ One-time admitted stable table with sorted deterministic lookup and no post-admission growth.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FixedTable<K, V> {
    slots: Box<[Option<(K, V)>]>,
    len: usize,
    admitted: bool,
    faulted: bool,
}

impl<K, V> Default for FixedTable<K, V> {
    fn default() -> Self {
        Self { slots: Box::default(), len: 0, admitted: false, faulted: false }
    }
}

// 🌱️ Hand-written, not derived — `#[derive(ToValue, FromValue)]` does not auto-infer
// per-type-parameter bounds (unlike `serde_derive`), so a generic struct like this one needs its
// bound spelled out explicitly; hand-writing is simpler here than threading a `#[value(bound =
// "…")]` container attribute through. Mirrors every field verbatim, `slots` as a JSON-like array
// of either `null` (an unoccupied slot) or a 2-element `[key, value]` pair.
impl<K: ToValue, V: ToValue> ToValue for FixedTable<K, V> {
    fn to_value(&self) -> DslValue {
        DslValue::object([
            (
                "slots".to_string(),
                DslValue::Array(
                    self.slots
                        .iter()
                        .map(|slot| match slot {
                            Some((key, value)) => DslValue::Array(vec![key.to_value(), value.to_value()]),
                            None => DslValue::Null,
                        })
                        .collect(),
                ),
            ),
            ("len".to_string(), self.len.to_value()),
            ("admitted".to_string(), self.admitted.to_value()),
            ("faulted".to_string(), self.faulted.to_value()),
        ])
    }
}
impl<K: FromValue, V: FromValue> FromValue for FixedTable<K, V> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = DslValue::into_object(value)?;
        let field = |key: &str| entries.iter().find(|(k, _)| k == key).map_or(DslValue::Null, |(_, v)| v.clone());
        let slots_array = match field("slots") {
            DslValue::Array(items) => items,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an array, found {other:?}")).under("slots")),
        };
        let slots: Vec<Option<(K, V)>> = slots_array
            .into_iter()
            .enumerate()
            .map(|(index, item)| -> Result<Option<(K, V)>, ValueError> {
                match item {
                    DslValue::Null => Ok(None),
                    DslValue::Array(pair) if pair.len() == 2 => {
                        let mut iter = pair.into_iter();
                        let key = K::from_value(iter.next().expect("checked len == 2")).map_err(|error| error.under("0"))?;
                        let value = V::from_value(iter.next().expect("checked len == 2")).map_err(|error| error.under("1"))?;
                        Ok(Some((key, value)))
                    }
                    other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected null or a 2-element array, found {other:?}"))),
                }
                .map_err(|error| error.under(index))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            slots: slots.into_boxed_slice(),
            len: usize::from_value(field("len")).map_err(|error| error.under("len"))?,
            admitted: bool::from_value(field("admitted")).map_err(|error| error.under("admitted"))?,
            faulted: bool::from_value(field("faulted")).map_err(|error| error.under("faulted"))?,
        })
    }
}

/// ♻️ A table retires slot by slot exactly as the boxed slice it owns, as a vector of optional entries.
impl<K: semio_framework_value::retirement::RetireOwned, V: semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetireOwned for FixedTable<K, V> {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.slots.into_vec())
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&Vec::<Option<(K, V)>>::new())
    }
    fn controlled_retirement_supported() -> bool {
        <Vec<Option<(K, V)>> as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixedTableError {
    AlreadyAdmitted,
    Allocation,
    Overflow,
    Unordered,
}

impl<K: Ord, V> FixedTable<K, V> {
    pub fn admit(&mut self, observed_capacity: usize) -> Result<(), FixedTableError> {
        if self.admitted {
            self.faulted = true;
            return Err(FixedTableError::AlreadyAdmitted);
        }
        let mut slots = Vec::new();
        if slots.try_reserve_exact(observed_capacity).is_err() {
            self.faulted = true;
            return Err(FixedTableError::Allocation);
        }
        slots.resize_with(observed_capacity, || None);
        self.slots = slots.into_boxed_slice();
        self.admitted = true;
        Ok(())
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, FixedTableError> {
        if !self.admitted {
            self.faulted = true;
            return Err(FixedTableError::Overflow);
        }
        match self.occupied().binary_search_by(|slot| slot.as_ref().expect("occupied fixed slot").0.cmp(&key)) {
            Ok(index) => Ok(Some(std::mem::replace(&mut self.slots[index].as_mut().expect("occupied fixed slot").1, value))),
            Err(index) if index == self.len && self.len < self.slots.len() => {
                self.slots[index] = Some((key, value));
                self.len += 1;
                Ok(None)
            }
            Err(_) => {
                self.faulted = true;
                Err(FixedTableError::Unordered)
            }
        }
    }

    pub fn insert_stable(&mut self, key: K, value: V) -> Result<(), FixedTableError> {
        if !self.admitted || self.len >= self.slots.len() {
            self.faulted = true;
            return Err(FixedTableError::Overflow);
        }
        self.slots[self.len] = Some((key, value));
        self.len += 1;
        Ok(())
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.occupied().binary_search_by(|slot| slot.as_ref().expect("occupied fixed slot").0.cmp(key)).ok().map(|index| &self.slots[index].as_ref().expect("occupied fixed slot").1)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let index = self.occupied().binary_search_by(|slot| slot.as_ref().expect("occupied fixed slot").0.cmp(key)).ok()?;
        Some(&mut self.slots[index].as_mut().expect("occupied fixed slot").1)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.slots[..self.len].iter().map(|slot| {
            let (key, value) = slot.as_ref().expect("occupied fixed slot");
            (key, value)
        })
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.slots[..self.len].iter().map(|slot| &slot.as_ref().expect("occupied fixed slot").1)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    pub fn faulted(&self) -> bool {
        self.faulted
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        self.slots[self.len].take()
    }

    pub fn last_mut(&mut self) -> Option<(&mut K, &mut V)> {
        let (key, value) = self.slots.get_mut(self.len.checked_sub(1)?)?.as_mut()?;
        Some((key, value))
    }

    /// 🔗️ Borrows an original keyed slot without an iterator walk or copied value.
    pub fn entry_index(&self,index:usize)->Option<(&K,&V)>{if index>=self.len{return None}self.slots.get(index)?.as_ref().map(|(key,value)|(key,value))}

    pub fn get_index(&self, index: usize) -> Option<&V> {
        self.slots.get(index)?.as_ref().map(|(_, value)| value)
    }

    pub fn get_index_mut(&mut self, index: usize) -> Option<&mut V> {
        self.slots.get_mut(index)?.as_mut().map(|(_, value)| value)
    }

    fn occupied(&self) -> &[Option<(K, V)>] {
        &self.slots[..self.len]
    }
}
// #endregion 🔖️FixedTable
impl semio_framework_dsl_record::BorrowedDslField for EntityId { const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::UInt; }

impl semio_framework_value_persistent::RetainedEqualityElement for EntityId{fn inline_eq(left:&Self,right:&Self)->bool{left.0==right.0}}
impl semio_framework_value_persistent::RetainedEquality for EntityId{type Equality=semio_framework_value_persistent::retained_equality::RetainedInlineEquality<Self>;const INLINE_PAIR_BYTES:Option<usize>=Some(2*std::mem::size_of::<Self>());fn equality_cursor()->Self::Equality{Self::Equality::new()}fn inline_equal(left:&Self,right:&Self)->Option<bool>{Some(left.0==right.0)}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod native_receiving_tests;
