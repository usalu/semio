//! 🔢️ Attribute map and defaults of the glTF snapshot: the ordered accessor name codec and the spec-mandated scalar defaults with their omission predicates. See [`super`].

//#region 🔖️OrderedAttrMap
/// 🔢️ Retains ordered accessor names under cumulative ownership and cancellation.
pub(super) fn ordered_attr_map_to_value_controlled(attrs: &[(String, usize)], control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_value::DslValue, semio_framework_value::ValueError> {
    control.scoped_stage(|control| {
        control.begin_stage(attrs.len())?;
        let mut output = control.allocate_vec(attrs.len())?;
        for (key, value) in attrs { output.push((control.copy_text(key)?, semio_framework_value::ToValue::to_value_controlled(value, control)?)); control.step()?; }
        Ok(semio_framework_value::DslValue::Object(output))
    })
}

pub(super) fn ordered_attr_map_to_value(attrs: &[(String, usize)]) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::object(attrs.iter().map(|(k, v)| (k.clone(), semio_framework_value::ToValue::to_value(v))))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn ordered_attr_map_from_value(value: semio_framework_value::DslValue) -> Result<Vec<(String, usize)>, semio_framework_value::ValueError> {
    let entries = semio_framework_value::DslValue::into_object(value)?;
    entries.into_iter().map(|(k, v)| Ok((k, semio_framework_value::FromValue::from_value(v)?))).collect()
}
//#endregion 🔖️OrderedAttrMap

//#region 🔖️SpecDefaults
/// 📐️ glTF 2.0 spec-mandated scalar defaults, applied on read when the JSON key is absent and
/// omitted on write when the in-memory value still equals the default -- a documented (and, for
/// this fixture set, byte-exact) normal form: a field is either genuinely absent or explicitly
/// non-default in every real document this codec has seen.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn default_one_f64() -> f64 {
    1.0
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn is_one_f64(v: &f64) -> bool {
    *v == 1.0
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn default_zero_u64() -> u64 {
    0
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn is_zero_u64(v: &u64) -> bool {
    *v == 0
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn default_zero_usize() -> usize {
    0
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn is_zero_usize(v: &usize) -> bool {
    *v == 0
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn default_wrap() -> u64 {
    10497
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn is_default_wrap(v: &u64) -> bool {
    *v == 10497
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn default_alpha_cutoff() -> f64 {
    0.5
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn is_default_alpha_cutoff(v: &f64) -> bool {
    *v == 0.5
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn default_vec3_zero() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn is_vec3_zero(v: &[f64; 3]) -> bool {
    *v == [0.0, 0.0, 0.0]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn default_vec4_one() -> [f64; 4] {
    [1.0, 1.0, 1.0, 1.0]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn is_vec4_one(v: &[f64; 4]) -> bool {
    *v == [1.0, 1.0, 1.0, 1.0]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn is_false(v: &bool) -> bool {
    !*v
}
//#endregion 🔖️SpecDefaults
