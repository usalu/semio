//! 🔁️ Closed refusal transport admits every owned byte through the actual caller control.
use crate::{DslValue, NativeDecodeControl, NativeEncodeControl, ValueError, ValueRefusalKind, FromValue, ToValue, retained_clone::RetainedCloneProgress};

fn kind(text: &str) -> Result<ValueRefusalKind, ValueError> {
    ValueRefusalKind::from_wire(text).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"unknown refusal kind"))
}
impl ValueRefusalKind {
    /// 🛫️ Owns the canonical wire spelling under the caller's cumulative encoding admission.
    pub fn to_value_controlled(&self, control: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        control.copy_text(self.as_str()).map(DslValue::String)
    }
    /// 🛬️ Admits one borrowed closed kind without constructing or classifying error prose.
    pub fn from_value_controlled(value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        control.scoped_stage(|control| {
            control.begin_stage(1)?;
            let DslValue::String(text) = value else { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected refusal kind text")) };
            let result = kind(text)?; control.step()?; Ok(result)
        })
    }
}
impl crate::ToValue for ValueRefusalKind {
    fn to_value(&self) -> DslValue { DslValue::String(self.as_str().into()) }
    fn to_value_controlled(&self, control: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { ValueRefusalKind::to_value_controlled(self, control) }
}
impl crate::FromValue for ValueRefusalKind {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::String(text) = value else { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected refusal kind text")) };
        kind(&text)
    }
    fn from_value_controlled(value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> { ValueRefusalKind::from_value_controlled(value, control) }
}

impl ValueError {
    /// 🛫️ Builds the exact three-member native record without an unchecked codec or default control.
    pub fn to_value_controlled(&self, control: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        control.scoped_depth(64, |control| control.scoped_stage(|control| {
            control.begin_stage(3)?;
            let mut entries = DslValue::object_encoding_controlled(3, control)?;
            let kind = self.kind.to_value_controlled(control)?;
            DslValue::push_encoding_controlled(entries.get_mut(), "kind", kind, control)?; control.step()?;
            let message = DslValue::String(control.copy_text(&self.message)?);
            DslValue::push_encoding_controlled(entries.get_mut(), "message", message, control)?; control.step()?;
            let progress = control.scoped_stage(|control| { control.begin_stage(4)?; self.retained_progress().to_value_controlled(control) })?;
            DslValue::push_encoding_controlled(entries.get_mut(), "retainedProgress", progress, control)?; control.step()?;
            Ok(DslValue::Object(entries.take()))
        }))
    }
    /// 🛬️ Constructs the closed refusal after validating every field, retaining actual control errors.
    pub fn from_value_controlled(value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        control.scoped_depth(64, |control| control.scoped_stage(|control| {
            control.begin_stage(3)?;
            let DslValue::Object(entries) = value else { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected refusal record")) };
            if entries.len() != 3 { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected exactly kind, message and retainedProgress")) }
            let (mut kind_value, mut message_value, mut retained_value) = (None, None, None);
            for (key, child) in entries {
                match key.as_str() {
                    "kind" if kind_value.is_none() => kind_value = Some(child),
                    "message" if message_value.is_none() => message_value = Some(child),
                    "retainedProgress" if retained_value.is_none() => retained_value = Some(child),
                    _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown or duplicate refusal member")),
                }
                control.step()?;
            }
            let kind_value = kind_value.ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "missing refusal kind"))?;
            let kind = ValueRefusalKind::from_value_controlled(kind_value, control)?;
            let Some(DslValue::String(message)) = message_value else { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected refusal message text")) };
            let retained = retained_value.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "missing refusal retained receipt"))?;
            let progress = control.scoped_stage(|control| { control.begin_stage(4)?; RetainedCloneProgress::from_value_controlled(retained, control) })?;
            Ok(Self::new(kind, control.copy_text(message)?).with_retained_progress(progress))
        }))
    }
}
