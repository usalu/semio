//#region 🔖️StateClass
// The four — and only four — state mechanisms the architecture admits, spanning the
// durability × visibility square. Carried on MutationDescriptor (protocol_command) and on wire
// envelopes (protocol_wire).

/// 🗂️ Which of the four state lanes an operation's diffs belong to. Exhaustive by
/// construction: `Artifact` = persisted shared, `Config` = persisted local-only, `Presence` =
/// ephemeral shared, `Transient` = ephemeral local-only UI state.
///
/// Draftness is a LANE property — which store a record lives in — never a field annotation, so a
/// draft artifact's fields are still [`StateClass::Artifact`]. Derivation travels on its own axis
/// (`#[derived]` / `x-semio-derived`), never as a state class: a derived field is not state at all.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StateClass {
    Artifact,
    Config,
    Presence,
    Transient,
}

/// 🌱️ Exact four-lane native value codec.
impl semio_framework_value::ToValue for StateClass {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_value::DslValue, semio_framework_value::ValueError> {
        let text = match self { Self::Artifact => "Artifact", Self::Config => "Config", Self::Presence => "Presence", Self::Transient => "Transient" };
        control.copy_text(text).map(semio_framework_value::DslValue::String)
    }
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(match self { StateClass::Artifact => "Artifact", StateClass::Config => "Config", StateClass::Presence => "Presence", StateClass::Transient => "Transient" }.to_string())
    }
}
impl semio_framework_value::FromValue for StateClass {
    fn from_value_controlled(value: &semio_framework_value::DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        control.scoped_stage(|control| { control.begin_stage(1)?; control.step() })?;
        match value { semio_framework_value::DslValue::String(text) => match text.as_str() { "Artifact" => Ok(Self::Artifact), "Config" => Ok(Self::Config), "Presence" => Ok(Self::Presence), "Transient" => Ok(Self::Transient), _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unknown StateClass variant")) }, _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected a string")) }
    }
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            semio_framework_value::DslValue::String(s) => match s.as_str() {
                "Artifact" => Ok(StateClass::Artifact),
                "Config" => Ok(StateClass::Config),
                "Presence" => Ok(StateClass::Presence),
                "Transient" => Ok(StateClass::Transient),
                other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown StateClass variant `{other}`"))),
            },
            other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}
//#endregion 🔖️StateClass

//#region 🔖️StateClassKebab
/// 🏷️ Parse the canonical kebab `x-semio-state` string into [`StateClass`].
///
/// 🌐️ One canonical state identity shared by schema leaves and mutation descriptors.
pub fn parse_state_class_kebab(value: &str) -> Option<StateClass> {
    match value {
        "artifact" => Some(StateClass::Artifact),
        "config" => Some(StateClass::Config),
        "presence" => Some(StateClass::Presence),
        "transient" => Some(StateClass::Transient),
        _ => None,
    }
}

/// 🏷️ Canonical kebab spelling of a [`StateClass`] for JSON Schema `x-semio-state`.
pub async fn state_class_kebab(class: StateClass) -> &'static str {
    match class {
        StateClass::Artifact => "artifact",
        StateClass::Config => "config",
        StateClass::Presence => "presence",
        StateClass::Transient => "transient",
    }
}
//#endregion 🔖️StateClassKebab
