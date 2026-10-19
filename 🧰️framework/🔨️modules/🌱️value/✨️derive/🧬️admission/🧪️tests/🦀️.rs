use semio_framework_value::{DslValue, FromValue, NativeDecodeControl, ValueError, ValueRefusalKind};

thread_local! {
    static RETIRED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static VALIDATED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[derive(Debug, semio_framework_value::FromValue)]
#[value(validate = "Self::validate", validate_controlled = "Self::validate_controlled", retire_with = "retire_probe")]
struct Probe { value: i64 }

impl Probe {
    fn validate(&self) -> Result<(), ValueError> {
        VALIDATED.with(|count| count.set(count.get() + 1));
        if self.value < 1 { return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "value must be positive")); }
        Ok(())
    }
    fn validate_controlled(&self, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
        control.step()?;
        self.validate()
    }
}

fn retire_probe(_: Probe) { RETIRED.with(|count| count.set(count.get() + 1)); }

#[derive(Debug, semio_framework_value::FromValue)]
#[value(validate = "Self::validate")]
struct ColdOnly { value: i64 }

impl ColdOnly {
    fn validate(&self) -> Result<(), ValueError> { if self.value > 0 { Ok(()) } else { Err(ValueError::literal(ValueRefusalKind::InvalidValue, "positive value required")) } }
}


trait SemanticValue { fn semantic_value(&self) -> i64; }
fn validate_shape<T: SemanticValue>(value: &T) -> Result<(), ValueError> { if value.semantic_value() > 0 { Ok(()) } else { Err(ValueError::literal(ValueRefusalKind::InvalidValue, "positive value required")) } }
fn validate_shape_controlled<T: SemanticValue>(value: &T, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> { control.step()?; validate_shape(value) }
fn retire_shape<T>(_: T) { RETIRED.with(|count| count.set(count.get() + 1)); }

#[derive(Debug, semio_framework_value::FromValue)]
#[value(transparent, validate = "validate_shape", validate_controlled = "validate_shape_controlled", retire_with = "retire_shape")]
struct TransparentProbe(i64);
impl SemanticValue for TransparentProbe { fn semantic_value(&self) -> i64 { self.0 } }

#[derive(Debug, semio_framework_value::FromValue)]
#[value(tag = "kind", rename_all = "lowercase", validate = "validate_shape", validate_controlled = "validate_shape_controlled", retire_with = "retire_shape")]
enum TaggedProbe { Value { value: i64 }, Empty }
impl SemanticValue for TaggedProbe { fn semantic_value(&self) -> i64 { match self { Self::Value { value } => *value, Self::Empty => 0 } } }

#[derive(Debug, semio_framework_value::FromValue)]
#[value(rename_all = "lowercase", validate = "validate_shape", validate_controlled = "validate_shape_controlled", retire_with = "retire_shape")]
enum ExternalProbe { Value { value: i64 }, Empty }
impl SemanticValue for ExternalProbe { fn semantic_value(&self) -> i64 { match self { Self::Value { value } => *value, Self::Empty => 0 } } }


#[derive(Debug, semio_framework_value::FromValue)]
#[value(validate = "validate_shape", validate_controlled = "validate_shape_controlled", default_controlled = "negative_default", retire_with = "retire_shape")]
struct DefaultProbe { value: i64 }
impl SemanticValue for DefaultProbe { fn semantic_value(&self) -> i64 { self.value } }
fn negative_default(control: &mut NativeDecodeControl<'_>) -> Result<DefaultProbe, ValueError> { control.checkpoint()?; Ok(DefaultProbe { value: 0 }) }

fn shape_law(fixture: &serde_json::Value, controlled: bool) {
    for row in fixture["cases"].as_array().unwrap() {
        for mode in ["transparent", "tagged", "external"] {
            let input = match mode {
                "transparent" => row["input"].get("value").cloned().unwrap_or(serde_json::Value::Null),
                "tagged" => { let mut value = row["input"].clone(); value["kind"] = "value".into(); value },
                _ => serde_json::json!({"value": row["input"]}),
            };
            RETIRED.with(|count| count.set(0));
            let value = DslValue::from(&input);
            let mut allow = |_| true;
            let mut control = NativeDecodeControl::new(10_000, &mut allow);
            let result = match mode {
                "transparent" if controlled => TransparentProbe::from_value_controlled(&value, &mut control).map(|_| ()),
                "transparent" => TransparentProbe::from_value(value).map(|_| ()),
                "tagged" if controlled => TaggedProbe::from_value_controlled(&value, &mut control).map(|_| ()),
                "tagged" => TaggedProbe::from_value(value).map(|_| ()),
                _ if controlled => ExternalProbe::from_value_controlled(&value, &mut control).map(|_| ()),
                _ => ExternalProbe::from_value(value).map(|_| ()),
            };
            assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{mode} controlled={controlled} {row}");
            assert_eq!(RETIRED.with(|count| count.get()), row["retired"].as_u64().unwrap() as usize, "{mode} controlled={controlled} {row}");
        }
    }
    RETIRED.with(|count| count.set(0));
    let unit = &fixture["unitCase"];
    let value = DslValue::from(&unit["input"]);
    let mut allow = |_| true;
    let mut control = NativeDecodeControl::new(10_000, &mut allow);
    let result = if controlled { ExternalProbe::from_value_controlled(&value, &mut control) } else { ExternalProbe::from_value(value) };
    assert_eq!(result.is_ok(), unit["admitted"].as_bool().unwrap());
    assert_eq!(RETIRED.with(|count| count.get()), unit["retired"].as_u64().unwrap() as usize, "unit enum early returns still pass semantic admission");
}

/// 🔣️ Cold semantic admission agrees with the neutral JSON Schema verdicts and retires refused roots once.
#[test]
fn semantic_admission_matches_neutral_schema() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        RETIRED.with(|count| count.set(0));
        let result = Probe::from_value(DslValue::from(&row["input"]));
        assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
        assert_eq!(RETIRED.with(|count| count.get()), row["retired"].as_u64().unwrap() as usize, "{}", row["id"]);
    }
    shape_law(&fixture, false);
    println!("[DEBUG] Value semantic admission: six neutral verdicts across named, transparent, tagged and external constructors; semantic refusals retire decoded roots once");
}

/// 🛡️ Controlled semantic admission has its own checkpoint and cannot bypass a missing controlled hook.
#[test]
fn controlled_admission_is_metered_and_refusals_retire_once() {
    for value in [0, 1] {
        RETIRED.with(|count| count.set(0));
        let mut callback = |_| true;
        let mut control = NativeDecodeControl::new(10_000, &mut callback);
        let input = DslValue::from(&serde_json::json!({"value": value}));
        let result = Probe::from_value_controlled(&input, &mut control);
        assert_eq!(result.is_ok(), value == 1);
        assert_eq!(RETIRED.with(|count| count.get()), usize::from(value == 0));
    }
    VALIDATED.with(|count| count.set(0));
    let mut callback = |_| false;
    let mut control = NativeDecodeControl::new(10_000, &mut callback);
    let input = DslValue::from(&serde_json::json!({"value": 1}));
    assert!(Probe::from_value_controlled(&input, &mut control).is_err());
    assert_eq!(VALIDATED.with(|count| count.get()), 0);
    let mut callback = |_| true;
    let mut control = NativeDecodeControl::new(10_000, &mut callback);
    assert!(ColdOnly::from_value_controlled(&input, &mut control).is_err());
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    shape_law(&fixture, true);
    RETIRED.with(|count| count.set(0));
    let mut allow = |_| true;
    let mut control = NativeDecodeControl::new(10_000, &mut allow);
    let default = DefaultProbe::default_value_controlled(&mut control);
    assert!(!default.is_ok(), "controlled default constructors cannot bypass semantic admission");
    assert_eq!(RETIRED.with(|count| count.get()), fixture["cases"][2]["retired"].as_u64().unwrap() as usize);
    println!("[DEBUG] Controlled admission covers transparent and enum constructors, honors original cancellation and requires an explicit controlled semantic hook");
}
