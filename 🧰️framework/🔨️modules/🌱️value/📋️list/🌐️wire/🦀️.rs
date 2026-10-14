//! 🌐️ Paged-list canonical value conversion is owned by its genuine wire traits.
use crate::{DslValue,ToValue,FromValue,ValueShape,ValueEdit,ValueError,ValueRefusalKind};
use semio_framework_value_paged_list::PagedList;
pub fn append_paged_cold<T, const N: usize>(values: &mut PagedList<T, N>, value: T) -> Result<(), ValueError> {
    while !values.has_reserved_slot() {
        let required = values.next_allocation_bytes().map_err(ValueError::from)?;
        let progress = values.reserve_one(required).map_err(|error| ValueError::from(error.refusal()))?;
        if !progress.progressed {
            return Err(ValueError::new(crate::ValueRefusalKind::InvariantViolated, "paged owner exact cold allocation did not progress"));
        }
    }
    values.push_reserved(value).map_err(|_| ValueError::new(crate::ValueRefusalKind::InvariantViolated, "paged owner rejected its reserved slot"))
}

pub fn paged_value_index(segment: &str, length: usize) -> Result<usize, ValueError> {
    if segment.is_empty() || (segment.len() > 1 && segment.starts_with('0')) || !segment.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("invalid canonical array index `{segment}`")));
    }
    let index = segment.parse::<usize>().map_err(|_| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("array index `{segment}` is out of range")))?;
    (index < length).then_some(index).ok_or_else(|| ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("array index `{segment}` is out of range for length {length}")))
}

impl<T: ToValue, const N: usize> ToValue for PagedList<T, N> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(self.to_value()) };
        let index = paged_value_index(segment, self.len())?;
        self.get(index).expect("validated paged index").value_at_path(rest).map_err(|error| error.under(segment))
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, ValueError> {
        let Some((segment, rest)) = path.split_first() else { return Ok(ValueShape::Array { len: self.len() }) };
        let index = paged_value_index(segment, self.len())?;
        self.get(index).expect("validated paged index").value_shape_at_path(rest).map_err(|error| error.under(segment))
    }
}

impl<T: FromValue, const N: usize> FromValue for PagedList<T, N> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Array(items) = value else { return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("expected an array, found {value:?}"))) };
        if items.len() > N {
            return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, format!("array length {} exceeds paged capacity {N}", items.len())));
        }
        let mut output = Self::default();
        for (index, item) in items.into_iter().enumerate() {
            append_paged_cold(&mut output, T::from_value(item).map_err(|error| error.under(index))?)?;
        }
        Ok(output)
    }

    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        let Some((segment, rest)) = path.split_first() else {
            return match edit {
                ValueEdit::Set(value) => {
                    *self = Self::from_value(value)?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } | ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "paged list structural edits require the retained editor cursor")),
            };
        };
        let index = paged_value_index(segment, self.len())?;
        if rest.is_empty() {
            return match edit {
                ValueEdit::Set(value) => {
                    *self.get_mut(index).expect("validated paged index") = T::from_value(value).map_err(|error| error.under(segment))?;
                    Ok(())
                }
                ValueEdit::Insert(_) | ValueEdit::InsertAt { .. } | ValueEdit::Remove => Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "paged list structural edits require the retained editor cursor")),
            };
        }
        self.get_mut(index).expect("validated paged index").edit_value_at_path(rest, edit).map_err(|error| error.under(segment))
    }
}

