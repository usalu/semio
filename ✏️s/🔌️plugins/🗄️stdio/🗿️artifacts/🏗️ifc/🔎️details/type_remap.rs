//! 🗺️ Exact direct attribute maps are planned before original payload donation.
use super::*;
use semio_framework_value::{ValueError, ValueRefusalKind};
fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
#[derive(Default)]
pub(super) struct Plan {
    pub(super) map: Option<Vec<usize>>,
    phase: u8,
    target: usize,
    previous: usize,
    offset: usize,
    reuse: usize,
    supplied: usize,
    target_attribute: Option<IfcNormalizationAttribute>,
    previous_attribute: Option<IfcNormalizationAttribute>,
    comparison: super::type_metadata::Comparison,
}
impl Plan {
    pub(super) fn required_frontier(&self, previous: usize) -> Option<usize> {
        (self.phase == 2 && self.previous == previous && self.target_attribute.is_some_and(|attribute| !attribute.optional && !attribute.derived)).then_some(self.target)
    }
    pub(super) fn copy_bytes() -> usize {
        2 * std::mem::size_of::<Self>() + 2 * std::mem::size_of::<IfcTypeProjection>() + 128
    }
    pub(super) fn capacity_bytes(&self, count: usize) -> Result<usize, ValueError> {
        if self.phase == 0 { count.checked_mul(std::mem::size_of::<usize>()).ok_or_else(|| invalid("IFC retype map capacity overflow")) } else { Ok(0) }
    }
    fn finish_slot(&mut self, previous: usize) -> Result<(), ValueError> {
        self.map.as_mut().ok_or_else(|| invalid("IFC retype map owner missing"))?.push(previous);
        self.target += 1;
        self.previous = 0;
        self.offset = 0;
        self.reuse = 0;
        self.phase = 1;
        self.target_attribute = None;
        self.previous_attribute = None;
        self.comparison = Default::default();
        Ok(())
    }
    pub(super) fn advance<C: IfcDetailsProfile>(&mut self, previous_name: &str, target_name: &str, values: &[IfcValue], count: usize, supplied: Option<&[super::type_input::Input]>) -> Result<(bool, usize), ValueError> {
        match self.phase {
            0 => {
                if count >= super::type_input::INPUT || values.len() >= super::type_input::INPUT {
                    return Err(invalid("IFC type map exceeds ordinal tag range"));
                }
                let map = self.map.get_or_insert_with(Vec::new);
                map.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC retype ordinal map allocation failed"))?;
                self.phase = 1;
                return Ok((false, map.capacity() * std::mem::size_of::<usize>()));
            }
            1 => {
                if supplied.and_then(|values| values.get(self.supplied)).is_some_and(|value| value.index < self.target) || self.target == count && supplied.is_some_and(|values| values.len() != self.supplied) {
                    return Err(invalid("IFC type input cannot override a compatible source slot"));
                }
                if self.target == count {
                    return Ok((true, 0));
                }
                let attribute = C::normalization_attribute(target_name, self.target, true).map_err(|_| invalid("IFC retype target attribute unavailable"))?.ok_or_else(|| invalid("IFC retype target attribute missing"))?;
                if attribute.derived {
                    return Err(invalid("IFC retype derived normalization requires an explicit value"));
                }
                self.target_attribute = Some(attribute);
                self.phase = 2;
            }
            2 => {
                if self.previous == values.len() {
                    if let Some(value) = supplied.and_then(|values| values.get(self.supplied)).filter(|value| value.index == self.target) {
                        if self.target_attribute.ok_or_else(|| invalid("IFC retype target metadata missing"))?.optional || value.value.is_none() {
                            return Err(invalid("IFC supplied type value must fill a required target slot"));
                        }
                        self.finish_slot(super::type_input::INPUT | self.supplied)?;
                        self.supplied += 1;
                    } else {
                        if !self.target_attribute.ok_or_else(|| invalid("IFC retype target metadata missing"))?.optional {
                            return Err(invalid("IFC retype required slot needs an explicit value"));
                        }
                        self.finish_slot(usize::MAX)?;
                    }
                } else {
                    let target = self.target_attribute.ok_or_else(|| invalid("IFC retype target metadata missing"))?;
                    if matches!(values[self.previous], IfcValue::Derived) || !target.optional && matches!(values[self.previous], IfcValue::Unset) {
                        self.previous += 1;
                    } else {
                        self.previous_attribute = Some(C::normalization_attribute(previous_name, self.previous, true).map_err(|_| invalid("IFC retype source attribute unavailable"))?.ok_or_else(|| invalid("IFC retype source attribute missing"))?);
                        self.offset = 0;
                        self.phase = 3;
                    }
                }
            }
            3 => {
                let target = self.target_attribute.ok_or_else(|| invalid("IFC retype target metadata missing"))?;
                let previous = self.previous_attribute.ok_or_else(|| invalid("IFC retype source metadata missing"))?;
                let end = self.offset.saturating_add(64).min(target.name.len());
                if target.name.len() != previous.name.len() || target.name.as_bytes()[self.offset..end] != previous.name.as_bytes()[self.offset..end] {
                    self.previous += 1;
                    self.phase = 2;
                } else {
                    self.offset = end;
                    if end == target.name.len() {
                        self.comparison.start(previous.type_expr, target.type_expr);
                        self.phase = 4;
                    }
                }
            }
            4 => match self.comparison.advance()? {
                Some(false) => {
                    self.previous += 1;
                    self.phase = 2;
                }
                Some(true) => {
                    self.reuse = 0;
                    self.phase = 6;
                }
                None => {}
            },
            6 => {
                let map = self.map.as_ref().ok_or_else(|| invalid("IFC retype map owner missing"))?;
                if let Some(previous) = map.get(self.reuse) {
                    if *previous == self.previous {
                        self.previous += 1;
                        self.phase = 2;
                    } else {
                        self.reuse += 1;
                    }
                } else {
                    self.finish_slot(self.previous)?;
                }
            }
            _ => return Err(invalid("IFC retype map stage invalid")),
        }
        Ok((false, 0))
    }
    pub(super) fn metadata(&self) -> [usize; 18] {
        let comparison = self.comparison.metadata();
        [
            self.phase as usize,
            self.target,
            self.previous,
            self.offset,
            self.reuse,
            self.supplied,
            self.target_attribute.map_or(0, |value| value.name.as_ptr() as usize),
            self.target_attribute.map_or(0, |value| value.type_expr as *const _ as *const () as usize),
            self.previous_attribute.map_or(0, |value| value.name.as_ptr() as usize),
            self.previous_attribute.map_or(0, |value| value.type_expr as *const _ as *const () as usize),
            comparison[0],
            comparison[1],
            comparison[2],
            comparison[3],
            comparison[4],
            comparison[5],
            self.map.as_ref().map_or(0, Vec::len),
            self.map.as_ref().map_or(0, Vec::capacity),
        ]
    }
    pub(super) fn clear_metadata(&mut self) {
        *self = Self::default();
    }
}
