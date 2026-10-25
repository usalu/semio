//! 📦️ Container codecs of the value codec: options, slices, vectors, deques, fixed arrays, boxes, hash and ordered sets, string-keyed maps, hash maps, the dynamic value, phantom data and the pair and triple tuples. See [`super`].

use super::*;

//#region 🔖️Containers
impl<T: ToValue> ToValue for Option<T> {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{c.scoped_depth(64,|c|match self{Some(value)=>value.to_value_controlled(c),None=>().to_value_controlled(c)})}
    fn to_value(&self) -> DslValue {
        match self {
            Some(value) => value.to_value(),
            None => DslValue::Null,
        }
    }

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        match (path.is_empty(), self) {
            (true, _) => Ok(self.to_value()),
            (false, Some(value)) => value.value_at_path(path),
            (false, None) => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("null has no child `{}`", path[0]))),
        }
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        match (path.is_empty(), self) {
            (true, None) => Ok(ValueShape::Null),
            (true, Some(value)) => value.value_shape_at_path(path),
            (false, Some(value)) => value.value_shape_at_path(path),
            (false, None) => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("null has no child `{}`", path[0]))),
        }
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        self.as_ref().ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected an object, found null"))?.value_key_at_path(path, index)
    }
}

impl<T: FromValue> FromValue for Option<T> {
fn from_value_controlled(v:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{match v{DslValue::Null=>{controlled::scalar(c)?;Ok(None)},v=>T::from_value_controlled(v,c).map(Some)}}
fn default_value_controlled(c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)?;Ok(None)}
fn retire_decoded(self){if let Some(v)=self{T::retire_decoded(v)}}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Null => Ok(None),
            other => T::from_value(other).map(Some),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        if path.is_empty() {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(value) | ValueEdit::InsertAt { value, .. } if self.is_none() => {
                    *self = Some(T::from_value(value)?);
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot insert an already-present optional value")),
                ValueEdit::Remove if self.is_some() => {
                    *self = None;
                    Ok(())
                }
                ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot remove an absent optional value")),
            };
        }
        self.as_mut().ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("null has no child `{}`", path[0])))?.edit_value_at_path(path, edit)
    }
}

impl<T: ToValue> ToValue for [T] {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::sequence(self.iter(),c)}
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        let index = path_index(segment, self.len(), false)?;
        self[index].value_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Array { len: self.len() }) };
        let index = path_index(segment, self.len(), false)?;
        self[index].value_shape_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        let (segment, rest) = path.split_first().ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected an object, found an array"))?;
        let item_index = path_index(segment, self.len(), false)?;
        self[item_index].value_key_at_path(rest, index).map_err(|error| error.under(segment))
    }
}
impl<T:ToValue+Send+Sync+'static> ToValue for semio_framework_value_shared_issuance::SealedShared<T>{
    fn to_value(&self)->DslValue{self.get().to_value()}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{self.get().to_value_controlled(control)}
    fn value_at_path(&self,path:&[&str])->Result<DslValue,ValueError>{self.get().value_at_path(path)}
    fn value_shape_at_path(&self,path:&[&str])->Result<ValueShape,ValueError>{self.get().value_shape_at_path(path)}
    fn value_key_at_path(&self,path:&[&str],index:usize)->Result<String,ValueError>{self.get().value_key_at_path(path,index)}
    fn to_object_key_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{self.get().to_object_key_controlled(control)}
}

impl<T: ToValue> ToValue for Vec<T> {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::sequence(self.iter(),c)}
    fn to_value(&self) -> DslValue {
        self.as_slice().to_value()
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        self.as_slice().value_at_path(path)
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        self.as_slice().value_shape_at_path(path)
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        self.as_slice().value_key_at_path(path, index)
    }
}

impl<T: FromValue> FromValue for Vec<T> {
fn from_value_controlled(v:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::sequence(v,c)}
fn default_value_controlled(c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)?;Ok(Vec::new())}
fn retire_decoded(self){for v in self{T::retire_decoded(v)}}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Array(items) => items.into_iter().enumerate().map(|(index, item)| T::from_value(item).map_err(|error| error.under(index))).collect(),
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an array, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot insert at the array root")),
                ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot remove the array root")),
            };
        };
        if !rest.is_empty() {
            let index = path_index(segment, self.len(), false)?;
            return self[index].edit_value_at_path(rest, edit).map_err(|error| error.under(segment));
        }
        match edit {
            ValueEdit::Set(value) => {
                let index = path_index(segment, self.len(), false)?;
                self[index] = T::from_value(value).map_err(|error| error.under(segment))?;
                Ok(())
            }
            ValueEdit::InsertAt { .. } => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "positioned insertion requires an object parent")),
            ValueEdit::Insert(value) => {
                let index = path_index(segment, self.len(), true)?;
                let value = T::from_value(value).map_err(|error| error.under(segment))?;
                self.insert(index, value);
                Ok(())
            }
            ValueEdit::Remove => {
                let index = path_index(segment, self.len(), false)?;
                self.remove(index);
                Ok(())
            }
        }
    }
}

/// 📐️ Same plain-JSON-array wire shape as `Vec<T>` — a `VecDeque` field (e.g. `semio-framework-
/// actor`'s per-lane mailbox rings) round-trips identically to its `Vec` twin, just ring-backed in memory.
impl<T: ToValue> ToValue for std::collections::VecDeque<T> {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::sequence(self.iter(),c)}
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        let index = path_index(segment, self.len(), false)?;
        self[index].value_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Array { len: self.len() }) };
        let index = path_index(segment, self.len(), false)?;
        self[index].value_shape_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        let (segment, rest) = path.split_first().ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected an object, found an array"))?;
        let item_index = path_index(segment, self.len(), false)?;
        self[item_index].value_key_at_path(rest, index).map_err(|error| error.under(segment))
    }
}

impl<T: FromValue> FromValue for std::collections::VecDeque<T> {
fn from_value_controlled(v:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let values=Vec::<T>::from_value_controlled(v,c)?.guard_decoded();c.charge(values.get().len().checked_mul(size_of::<T>()).ok_or_else(||ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "deque size overflow"))?)?;Ok(values.take().into())}
fn default_value_controlled(c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)?;Ok(Self::new())}
fn retire_decoded(self){for v in self{T::retire_decoded(v)}}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Array(items) => items.into_iter().enumerate().map(|(index, item)| T::from_value(item).map_err(|error| error.under(index))).collect(),
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an array, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot insert at the array root")),
                ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot remove the array root")),
            };
        };
        if !rest.is_empty() {
            let index = path_index(segment, self.len(), false)?;
            return self[index].edit_value_at_path(rest, edit).map_err(|error| error.under(segment));
        }
        match edit {
            ValueEdit::Set(value) => {
                let index = path_index(segment, self.len(), false)?;
                self[index] = T::from_value(value).map_err(|error| error.under(segment))?;
            }
            ValueEdit::InsertAt { .. } => return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "positioned insertion requires an object parent")),
            ValueEdit::Insert(value) => {
                let index = path_index(segment, self.len(), true)?;
                let value = T::from_value(value).map_err(|error| error.under(segment))?;
                self.insert(index, value);
            }
            ValueEdit::Remove => {
                let index = path_index(segment, self.len(), false)?;
                self.remove(index);
            }
        }
        Ok(())
    }
}

/// 🔧️ A fixed-size array encodes exactly like a `Vec<T>` (a plain JSON array) — the length is
/// carried by `N`, not the wire, so decode rejects any array whose length doesn't match `N`
/// (matches what a fixed-size `[T; N]` field means: this many, no more, no fewer).
impl<T: ToValue, const N: usize> ToValue for [T; N] {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::sequence(self.iter(),c)}
    fn to_value(&self) -> DslValue {
        self.as_slice().to_value()
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        self.as_slice().value_at_path(path)
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        self.as_slice().value_shape_at_path(path)
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        self.as_slice().value_key_at_path(path, index)
    }
}

impl<T: FromValue, const N: usize> FromValue for [T; N] {
fn from_value_controlled(v:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let DslValue::Array(items)=v else{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected array"))};if items.len()!=N{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "fixed array length mismatch"))}let values=Vec::<T>::from_value_controlled(v,c)?;Ok(values.try_into().ok().expect("length checked"))}
fn retire_decoded(self){for v in self{T::retire_decoded(v)}}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Array(items) => {
                let found = items.len();
                let decoded: Vec<T> = items.into_iter().enumerate().map(|(index, item)| T::from_value(item).map_err(|error| error.under(index))).collect::<Result<_, _>>()?;
                decoded.try_into().map_err(|_| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an array of length {N}, found {found}")))
            }
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an array, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot insert into a fixed array")),
                ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot remove a fixed array")),
            };
        };
        let index = path_index(segment, N, false)?;
        match (rest.is_empty(), edit) {
            (true, ValueEdit::Set(value)) => {
                self[index] = T::from_value(value).map_err(|error| error.under(segment))?;
                Ok(())
            }
            (true, ValueEdit::Insert(_)) | (true, ValueEdit::InsertAt { .. }) => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot insert into a fixed array")),
            (true, ValueEdit::Remove) => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot remove a fixed array")),
            (false, edit) => self[index].edit_value_at_path(rest, edit).map_err(|error| error.under(segment)),
        }
    }
}

impl<T: ToValue> ToValue for Box<T> {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{c.scoped_depth(64,|c|(**self).to_value_controlled(c))}
    fn to_value(&self) -> DslValue {
        (**self).to_value()
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        (**self).value_at_path(path)
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        (**self).value_shape_at_path(path)
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        (**self).value_key_at_path(path, index)
    }
}

impl<T: FromValue> FromValue for Box<T> {
fn from_value_controlled(v:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{c.charge(size_of::<T>())?;T::from_value_controlled(v,c).map(Box::new)}
fn retire_decoded(self){T::retire_decoded(*self)}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        T::from_value(value).map(Box::new)
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        if path.is_empty() {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = Box::new(T::from_value(value)?);
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot insert at the boxed value root")),
                ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot remove the boxed value root")),
            };
        }
        (**self).edit_value_at_path(path, edit)
    }
}

/// 🧺️ An unordered set uses an array wire shape, collapsing duplicates without an iteration contract.
impl<T: ToValue + Eq + std::hash::Hash, S: std::hash::BuildHasher> ToValue for std::collections::HashSet<T,S> {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::sequence(self.iter(),c)}
    fn to_value(&self)->DslValue{DslValue::Array(self.iter().map(ToValue::to_value).collect())}
    fn value_shape_at_path(&self,path:&[&str])->Result<ValueShape,ValueError>{
        if path.is_empty(){Ok(ValueShape::Array{len:self.len()})}else{Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "unordered set members have no stable wire index"))}
    }
    fn value_at_path(&self,path:&[&str])->Result<DslValue,ValueError>{
        if path.is_empty(){Ok(self.to_value())}else{Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "unordered set members have no stable wire index"))}
    }
}

impl<T: FromValue + Eq + std::hash::Hash, S: ControlledValueHasher> FromValue for std::collections::HashSet<T,S> {
    fn from_value(value:DslValue)->Result<Self,ValueError>{
        let DslValue::Array(items)=value else{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an array, found {value:?}")))};
        let mut output=Self::with_hasher(S::default()).guard_decoded();
        for(index,value)in items.into_iter().enumerate(){
            let child=T::from_value(value).map_err(|error|error.under(index))?.guard_decoded();
            if !output.get().contains(child.get()){output.get_mut().insert(child.take());}
        }
        Ok(output.take())
    }
    fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::hash_set(value,control)}
    fn default_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{Ok(Self::with_hasher(S::from_value_controlled(control)?))}
    fn retire_decoded(self){for value in self{T::retire_decoded(value)}}
}

/// 🌳️ A `BTreeSet<T>` encodes exactly like a `Vec<T>` (a plain JSON array), in the set's own sorted
/// iteration order — matches `serde`'s own `BTreeSet` representation.
impl<T: ToValue + Ord> ToValue for std::collections::BTreeSet<T> {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::sequence(self.iter(),c)}
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        let index = path_index(segment, self.len(), false)?;
        self.iter().nth(index).expect("validated set index").value_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Array { len: self.len() }) };
        let index = path_index(segment, self.len(), false)?;
        self.iter().nth(index).expect("validated set index").value_shape_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        let (segment, rest) = path.split_first().ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected an object, found an array"))?;
        let item_index = path_index(segment, self.len(), false)?;
        self.iter().nth(item_index).expect("validated set index").value_key_at_path(rest, index).map_err(|error| error.under(segment))
    }
}

impl<T: FromValue + Ord> FromValue for std::collections::BTreeSet<T> {
fn from_value_controlled(v:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::set(v,c)}
fn default_value_controlled(c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)?;Ok(Self::new())}
fn retire_decoded(self){for v in self{T::retire_decoded(v)}}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Array(items) => items.into_iter().enumerate().map(|(index, item)| T::from_value(item).map_err(|error| error.under(index))).collect(),
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an array, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot insert at the set root")),
                ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot remove the set root")),
            };
        };
        if !rest.is_empty() {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "nested edits inside ordered set members require replacing the complete member"));
        }
        let mut values: Vec<T> = std::mem::take(self).into_iter().collect();
        let result = (|| -> Result<(), ValueError> {
            match edit {
                ValueEdit::Set(value) => {
                    let index = path_index(segment, values.len(), false)?;
                    let replacement = T::from_value(value).map_err(|error| error.under(segment))?;
                    let ordered = index.checked_sub(1).is_none_or(|previous| values[previous] < replacement) && values.get(index + 1).is_none_or(|next| replacement < *next);
                    if !ordered {
                        return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "ordered set replacement changes the member's wire index"));
                    }
                    values[index] = replacement;
                    Ok(())
                }
                ValueEdit::InsertAt { .. } => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "positioned insertion requires an object parent")),
            ValueEdit::Insert(value) => {
                    let index = path_index(segment, values.len(), true)?;
                    let value = T::from_value(value).map_err(|error| error.under(segment))?;
                    let ordered = index.checked_sub(1).is_none_or(|previous| values[previous] < value) && values.get(index).is_none_or(|next| value < *next);
                    if !ordered {
                        return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "ordered set insertion index does not match the member's sort order"));
                    }
                    values.insert(index, value);
                    Ok(())
                }
                ValueEdit::Remove => {
                    let index = path_index(segment, values.len(), false)?;
                    values.remove(index);
                    Ok(())
                }
            }
        })();
        *self = values.into_iter().collect();
        result
    }
}

impl<T: ToValue> ToValue for std::collections::BTreeMap<String, T> {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::map(self.iter(),c)}
    fn to_value(&self) -> DslValue {
        DslValue::object(self.iter().map(|(key, value)| (key.clone(), value.to_value())))
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((key, rest)) = path.split_first() else { return Ok(self.to_value()) };
        self.get(*key).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{key}`")))?.value_at_path(rest).map_err(|error| error.under(key))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((key, rest)) = path.split_first() else { return Ok(ValueShape::Object { len: self.len() }) };
        self.get(*key).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{key}`")))?.value_shape_at_path(rest).map_err(|error| error.under(key))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        if path.is_empty() {
            return self.keys().nth(index).cloned().ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {}", self.len())));
        }
        let (key, rest) = path.split_first().expect("non-empty path checked above");
        self.get(*key).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{key}`")))?.value_key_at_path(rest, index).map_err(|error| error.under(key))
    }
}

impl<T: FromValue> FromValue for std::collections::BTreeMap<String, T> {
fn from_value_controlled(v:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::map(v,c)}
fn default_value_controlled(c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)?;Ok(Self::new())}
fn retire_decoded(self){for (_,v) in self{T::retire_decoded(v)}}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Object(entries) => entries.into_iter().map(|(key, value)| T::from_value(value).map(|value| (key.clone(), value)).map_err(|error| error.under(key))).collect(),
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an object, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((key, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot insert at the map root")),
                ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot remove the map root")),
            };
        };
        if !rest.is_empty() {
            return self.get_mut(*key).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{key}`")))?.edit_value_at_path(rest, edit).map_err(|error| error.under(key));
        }
        match edit {
            ValueEdit::Set(value) => {
                let replacement = T::from_value(value).map_err(|error| error.under(key))?;
                let target = self.get_mut(*key).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{key}`")))?;
                *target = replacement;
            }
            ValueEdit::Insert(value) | ValueEdit::InsertAt { value, .. } => {
                if self.contains_key(*key) {
                    return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("object key `{key}` already exists")));
                }
                self.insert((*key).to_owned(), T::from_value(value).map_err(|error| error.under(key))?);
            }
            ValueEdit::Remove => {
                self.remove(*key).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{key}`")))?;
            }
        }
        Ok(())
    }
}

impl ToValue for DslValue {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::intrinsic(self,c)}
    fn to_value(&self) -> DslValue {
        self.clone()
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.clone()) };
        match self {
            DslValue::Array(items) => {
                let index = path_index(segment, items.len(), false)?;
                items[index].value_at_path(rest).map_err(|error| error.under(segment))
            }
            DslValue::Object(entries) => {
                let index = object_entry_index(entries, segment)?.ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?;
                entries[index].1.value_at_path(rest).map_err(|error| error.under(segment))
            }
            _ => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("value has no child `{segment}`"))),
        }
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::of(self)) };
        match self {
            DslValue::Array(items) => {
                let index = path_index(segment, items.len(), false)?;
                items[index].value_shape_at_path(rest).map_err(|error| error.under(segment))
            }
            DslValue::Object(entries) => {
                let index = object_entry_index(entries, segment)?.ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?;
                entries[index].1.value_shape_at_path(rest).map_err(|error| error.under(segment))
            }
            _ => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("value has no child `{segment}`"))),
        }
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        if path.is_empty() {
            let DslValue::Object(entries) = self else { return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an object, found {self:?}"))) };
            return entries.get(index).map(|(key, _)| key.clone()).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {}", entries.len())));
        }
        let (segment, rest) = path.split_first().expect("non-empty path checked above");
        match self {
            DslValue::Array(items) => {
                let item_index = path_index(segment, items.len(), false)?;
                items[item_index].value_key_at_path(rest, index).map_err(|error| error.under(segment))
            }
            DslValue::Object(entries) => {
                let entry_index = object_entry_index(entries, segment)?.ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?;
                entries[entry_index].1.value_key_at_path(rest, index).map_err(|error| error.under(segment))
            }
            _ => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("value has no child `{segment}`"))),
        }
    }
}

impl FromValue for DslValue {
fn from_value_controlled(v:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::intrinsic(v,c)}
fn default_value_controlled(c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)?;Ok(Self::Null)}
fn retire_decoded(self){controlled::retire_intrinsic(self)}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        Ok(value)
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = value;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot insert at the value root")),
                ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot remove the value root")),
            };
        };
        match self {
            DslValue::Array(items) => {
                if !rest.is_empty() {
                    let index = path_index(segment, items.len(), false)?;
                    return items[index].edit_value_at_path(rest, edit).map_err(|error| error.under(segment));
                }
                match edit {
                    ValueEdit::Set(value) => {
                        let index = path_index(segment, items.len(), false)?;
                        items[index] = value;
                    }
                    ValueEdit::InsertAt { .. } => return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "positioned insertion requires an object parent")),
                    ValueEdit::Insert(value) => {
                        let index = path_index(segment, items.len(), true)?;
                        items.insert(index, value);
                    }
                    ValueEdit::Remove => {
                        let index = path_index(segment, items.len(), false)?;
                        items.remove(index);
                    }
                }
                Ok(())
            }
            DslValue::Object(entries) => {
                let index = object_entry_index(entries, segment)?;
                if !rest.is_empty() {
                    return entries[index.ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?]
                        .1
                        .edit_value_at_path(rest, edit)
                        .map_err(|error| error.under(segment));
                }
                match edit {
                    ValueEdit::Set(value) => entries[index.ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?].1 = value,
                    ValueEdit::Insert(value) => {
                        if index.is_some() {
                            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("object key `{segment}` already exists")));
                        }
                        entries.push(((*segment).to_owned(), value));
                    }
                    ValueEdit::InsertAt { index: position, value } => {
                        if index.is_some() || position > entries.len() {
                            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "ordered insertion requires an absent key and an in-range position"));
                        }
                        entries.insert(position, ((*segment).to_owned(), value));
                    }
                    ValueEdit::Remove => {
                        entries.remove(index.ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?);
                    }
                }
                Ok(())
            }
            _ => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("value has no child `{segment}`"))),
        }
    }
}

/// 👻️ `PhantomData<T>` is zero-sized and carries no data regardless of `T` — matches `serde`'s own
/// blanket impl (encodes as a unit value, decodes from anything). Unconditional on `T` (no `T:
/// ToValue`/`FromValue` bound) so a generic struct with a `PhantomData<SomeUnrelatedType>` marker
/// field never forces that unrelated type to implement these traits too.
impl<T: ?Sized> ToValue for std::marker::PhantomData<T> {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{().to_value_controlled(c)}
    fn to_value(&self) -> DslValue {
        DslValue::Null
    }
}

impl<T: ?Sized> FromValue for std::marker::PhantomData<T> {
fn from_value_controlled(v:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{<()>::from_value_controlled(v,c)?;Ok(Self)}
fn default_value_controlled(c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(c)?;Ok(Self)}

    fn from_value(_value: DslValue) -> Result<Self, ValueError> {
        Ok(std::marker::PhantomData)
    }
}

/// 🔗️ A 2-tuple encodes as a fixed-length array — the same shape `serde_json` gives a Rust tuple.
impl<A: ToValue, B: ToValue> ToValue for (A, B) {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{c.scoped_depth(64,|c|c.scoped_stage(|c|{c.begin_stage(2)?;let mut output=Vec::<DslValue>::guard_decoded(c.allocate_vec(2)?);output.get_mut().push(self.0.to_value_controlled(c)?);c.step()?;output.get_mut().push(self.1.to_value_controlled(c)?);c.step()?;Ok(DslValue::Array(output.take()))}))}
    fn to_value(&self) -> DslValue {
        DslValue::Array(vec![self.0.to_value(), self.1.to_value()])
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        match path_index(segment, 2, false)? {
            0 => self.0.value_at_path(rest),
            1 => self.1.value_at_path(rest),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Array { len: 2 }) };
        match path_index(segment, 2, false)? {
            0 => self.0.value_shape_at_path(rest),
            1 => self.1.value_shape_at_path(rest),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        let (segment, rest) = path.split_first().ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected an object, found an array"))?;
        match path_index(segment, 2, false)? {
            0 => self.0.value_key_at_path(rest, index),
            1 => self.1.value_key_at_path(rest, index),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }
}

impl<A: FromValue, B: FromValue> FromValue for (A, B) {
fn from_value_controlled(v:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let DslValue::Array(items)=v else{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected tuple array"))};if items.len()!=2{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "tuple length mismatch"))}let v0=A::from_value_controlled(&items[0],c).map_err(|e|e.under(0))?.guard_decoded();let v1=B::from_value_controlled(&items[1],c).map_err(|e|e.under(1))?.guard_decoded();Ok((v0.take(),v1.take()))}
fn retire_decoded(self){A::retire_decoded(self.0);B::retire_decoded(self.1);}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Array(items) if items.len() == 2 => {
                let mut iter = items.into_iter();
                let a = A::from_value(iter.next().expect("len == 2")).map_err(|error| error.under(0))?;
                let b = B::from_value(iter.next().expect("len == 2")).map_err(|error| error.under(1))?;
                Ok((a, b))
            }
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected a 2-element array, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot insert into a fixed tuple")),
                ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot remove a fixed tuple")),
            };
        };
        if rest.is_empty() && !matches!(edit, ValueEdit::Set(_)) {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot change the length of a fixed tuple"));
        }
        match path_index(segment, 2, false)? {
            0 => self.0.edit_value_at_path(rest, edit),
            1 => self.1.edit_value_at_path(rest, edit),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }
}

/// 🧩️ A 3-tuple encodes as a fixed-length array — the same shape `serde_json` gives a Rust tuple.
impl<A: ToValue, B: ToValue, C: ToValue> ToValue for (A, B, C) {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{c.scoped_depth(64,|c|c.scoped_stage(|c|{c.begin_stage(3)?;let mut output=Vec::<DslValue>::guard_decoded(c.allocate_vec(3)?);output.get_mut().push(self.0.to_value_controlled(c)?);c.step()?;output.get_mut().push(self.1.to_value_controlled(c)?);c.step()?;output.get_mut().push(self.2.to_value_controlled(c)?);c.step()?;Ok(DslValue::Array(output.take()))}))}
    fn to_value(&self) -> DslValue {
        DslValue::Array(vec![self.0.to_value(), self.1.to_value(), self.2.to_value()])
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        match path_index(segment, 3, false)? {
            0 => self.0.value_at_path(rest),
            1 => self.1.value_at_path(rest),
            2 => self.2.value_at_path(rest),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Array { len: 3 }) };
        match path_index(segment, 3, false)? {
            0 => self.0.value_shape_at_path(rest),
            1 => self.1.value_shape_at_path(rest),
            2 => self.2.value_shape_at_path(rest),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        let (segment, rest) = path.split_first().ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected an object, found an array"))?;
        match path_index(segment, 3, false)? {
            0 => self.0.value_key_at_path(rest, index),
            1 => self.1.value_key_at_path(rest, index),
            2 => self.2.value_key_at_path(rest, index),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }
}

impl<A: FromValue, B: FromValue, C: FromValue> FromValue for (A, B, C) {
fn from_value_controlled(v:&DslValue,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let DslValue::Array(items)=v else{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected tuple array"))};if items.len()!=3{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "tuple length mismatch"))}let v0=A::from_value_controlled(&items[0],c).map_err(|e|e.under(0))?.guard_decoded();let v1=B::from_value_controlled(&items[1],c).map_err(|e|e.under(1))?.guard_decoded();let v2=C::from_value_controlled(&items[2],c).map_err(|e|e.under(2))?.guard_decoded();Ok((v0.take(),v1.take(),v2.take()))}
fn retire_decoded(self){A::retire_decoded(self.0);B::retire_decoded(self.1);C::retire_decoded(self.2);}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Array(items) if items.len() == 3 => {
                let mut iter = items.into_iter();
                let a = A::from_value(iter.next().expect("len == 3")).map_err(|error| error.under(0))?;
                let b = B::from_value(iter.next().expect("len == 3")).map_err(|error| error.under(1))?;
                let c = C::from_value(iter.next().expect("len == 3")).map_err(|error| error.under(2))?;
                Ok((a, b, c))
            }
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected a 3-element array, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot insert into a fixed tuple")),
                ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot remove a fixed tuple")),
            };
        };
        if rest.is_empty() && !matches!(edit, ValueEdit::Set(_)) {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot change the length of a fixed tuple"));
        }
        match path_index(segment, 3, false)? {
            0 => self.0.edit_value_at_path(rest, edit),
            1 => self.1.edit_value_at_path(rest, edit),
            2 => self.2.edit_value_at_path(rest, edit),
            _ => unreachable!("validated tuple index"),
        }
        .map_err(|error| error.under(segment))
    }
}

/// 🗺️ A `HashMap<K, V>` encodes as an object with `K` stringified into the key — matches
/// `serde_json`'s own behavior for non-`String` map keys (JSON objects only have string keys), so
/// `HashMap<u64, _>` etc. round-trip on the same wire shape a pre-conversion `serde_json::Value`
/// would have produced. Iteration order is unspecified, same as `serde_json` gives for a `HashMap`.
impl<K: ToString + ToValue, V: ToValue> ToValue for std::collections::HashMap<K, V> {
fn to_value_controlled(&self,c:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{encoding::map(self.iter(),c)}
    fn to_value(&self) -> DslValue {
        DslValue::object(self.iter().map(|(key, value)| (key.to_string(), value.to_value())))
    }


    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        let mut matches = self.iter().filter(|(key, _)| key.to_string() == *segment);
        let (_, value) = matches.next().ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?;
        if matches.next().is_some() {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("multiple map keys encode as `{segment}`")));
        }
        value.value_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Object { len: self.len() }) };
        let mut matches = self.iter().filter(|(key, _)| key.to_string() == *segment);
        let (_, value) = matches.next().ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?;
        if matches.next().is_some() {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("multiple map keys encode as `{segment}`")));
        }
        value.value_shape_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, ValueError> {
        if path.is_empty() {
            let key = self.keys().nth(index).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("object key index {index} is out of range for length {}", self.len())))?.to_string();
            if self.keys().filter(|candidate| candidate.to_string() == key).count() != 1 {
                return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("multiple map keys encode as `{key}`")));
            }
            return Ok(key);
        }
        let (segment, rest) = path.split_first().expect("non-empty path checked above");
        let mut matches = self.iter().filter(|(key, _)| key.to_string() == *segment);
        let (_, value) = matches.next().ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?;
        if matches.next().is_some() {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("multiple map keys encode as `{segment}`")));
        }
        value.value_key_at_path(rest, index).map_err(|error| error.under(segment))
    }
}

impl<K: std::str::FromStr + std::hash::Hash + Eq + FromValue, V: FromValue> FromValue for std::collections::HashMap<K, V>
where
    K::Err: std::fmt::Display,
{
    fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::hash_map(value,control)}
    fn default_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::scalar(control)?;Ok(Self::new())}
    fn retire_decoded(self){for(key,value)in self{K::retire_decoded(key);V::retire_decoded(value)}}

    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Object(entries) => entries
                .into_iter()
                .map(|(key, value)| {
                    let parsed_key = key.parse::<K>().map_err(|error| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("invalid map key {key:?}: {error}")))?;
                    V::from_value(value).map(|value| (parsed_key, value)).map_err(|error| error.under(key))
                })
                .collect(),
            other => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an object, found {other:?}"))),
        }
    }


    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = <Self as FromValue>::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot insert at the map root")),
                ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "cannot remove the map root")),
            };
        };
        let key = segment.parse::<K>().map_err(|error| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("invalid map key `{segment}`: {error}")))?;
        if !rest.is_empty() {
            return self.get_mut(&key).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?.edit_value_at_path(rest, edit).map_err(|error| error.under(segment));
        }
        match edit {
            ValueEdit::Set(value) => {
                let replacement = V::from_value(value).map_err(|error| error.under(segment))?;
                let target = self.get_mut(&key).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?;
                *target = replacement;
            }
            ValueEdit::Insert(value) | ValueEdit::InsertAt { value, .. } => {
                if self.contains_key(&key) {
                    return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("object key `{segment}` already exists")));
                }
                self.insert(key, V::from_value(value).map_err(|error| error.under(segment))?);
            }
            ValueEdit::Remove => {
                self.remove(&key).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("missing object key `{segment}`")))?;
            }
        }
        Ok(())
    }
}
//#endregion 🔖️Containers
