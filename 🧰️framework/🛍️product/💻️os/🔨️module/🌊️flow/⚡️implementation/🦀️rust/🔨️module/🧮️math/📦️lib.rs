//! ➕️ Flow math module: schema-dispatched arithmetic operators.

use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, FieldSpec, Operation, OperatorImpl, OperatorInfo, Registry, Schema, Value, VariadicSpec};
use std::cell::Cell;

// #region 🔖️Add
/// ➕️ Adds numbers, points, or vectors.
pub struct Add;

impl Operation for Add {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        if let Some(items) = input.get("items").and_then(|v| v.as_dictionary()) {
            return add_items(items);
        }
        let a = read_dict(input, "a")?;
        let b = read_dict(input, "b")?;
        match a.schema() {
            Some("point") => Ok(channel_output("sum", xyz_dictionary("point", read_xyz(a)? + read_xyz(b)?))),
            Some("vector") => Ok(channel_output("sum", xyz_dictionary("vector", read_xyz(a)? + read_xyz(b)?))),
            _ => Ok(channel_output("sum", number_dictionary(read_value_number(a)? + read_value_number(b)?))),
        }
    }
}
// #endregion 🔖️Add

// #region 🔖️Subtract
/// ➖️ Subtracts numbers, points, or vectors.
pub struct Subtract;

impl Operation for Subtract {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let a = read_dict(input, "a")?;
        let b = read_dict(input, "b")?;
        match a.schema() {
            Some("point") => Ok(channel_output("difference", xyz_dictionary("point", read_xyz(a)? - read_xyz(b)?))),
            Some("vector") => Ok(channel_output("difference", xyz_dictionary("vector", read_xyz(a)? - read_xyz(b)?))),
            _ => Ok(channel_output("difference", number_dictionary(read_value_number(a)? - read_value_number(b)?))),
        }
    }
}
// #endregion 🔖️Subtract

// #region 🔖️Move
/// 🚚️ Moves a point or vector by a vector.
pub struct Move;

impl Operation for Move {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let subject = read_dict(input, "subject")?;
        let vector = read_dict(input, "vector")?;
        let schema = subject.schema().unwrap_or("vector");
        let result = xyz_dictionary(schema, read_xyz(subject)? + read_xyz(vector)?);
        if schema == "point" {
            Ok(Dictionary::new().insert("point", Value::Dictionary(result)).insert("vector", Value::null()))
        } else {
            Ok(Dictionary::new().insert("vector", Value::Dictionary(result)).insert("point", Value::null()))
        }
    }
}
// #endregion 🔖️Move

// #region 🔖️Scalar
/// ✖️ Multiplies two numbers.
pub struct Multiply;

impl Operation for Multiply {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("product", number_dictionary(read_channel_number(input, "a")? * read_channel_number(input, "b")?)))
    }
}

/// ➗️ Divides a by b.
pub struct Divide;

impl Operation for Divide {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let b = read_channel_number(input, "b")?;
        if b.abs() < f64::EPSILON {
            return Err(EvalError::InvalidInput("divide by zero".into()));
        }
        Ok(channel_output("quotient", number_dictionary(read_channel_number(input, "a")? / b)))
    }
}

/// ⚡️ Raises a to the power of b.
pub struct Power;

impl Operation for Power {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("power", number_dictionary(read_channel_number(input, "a")?.powf(read_channel_number(input, "b")?))))
    }
}

/// 🧮️ Remainder of a divided by b.
pub struct Modulo;

impl Operation for Modulo {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let b = read_channel_number(input, "b")?;
        if b.abs() < f64::EPSILON {
            return Err(EvalError::InvalidInput("modulo by zero".into()));
        }
        Ok(channel_output("modulo", number_dictionary(read_channel_number(input, "a")? % b)))
    }
}

/// ↔ Negates a number.
pub struct Negate;

impl Operation for Negate {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("negated", number_dictionary(-read_channel_number(input, "number")?)))
    }
}

/// 📏️ Absolute value.
pub struct Abs;

impl Operation for Abs {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("absolute", number_dictionary(read_channel_number(input, "number")?.abs())))
    }
}

/// √ Square root.
pub struct Sqrt;

impl Operation for Sqrt {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("root", number_dictionary(read_channel_number(input, "number")?.sqrt())))
    }
}

/// ⬇️ Minimum of two numbers.
pub struct Min;

impl Operation for Min {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("minimum", number_dictionary(read_channel_number(input, "a")?.min(read_channel_number(input, "b")?))))
    }
}

/// ⬆️ Maximum of two numbers.
pub struct Max;

impl Operation for Max {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("maximum", number_dictionary(read_channel_number(input, "a")?.max(read_channel_number(input, "b")?))))
    }
}

/// ⬇️ Floor of a number.
pub struct Floor;

impl Operation for Floor {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("floor", number_dictionary(read_channel_number(input, "number")?.floor())))
    }
}

/// ⬆️ Ceiling of a number.
pub struct Ceil;

impl Operation for Ceil {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("ceiling", number_dictionary(read_channel_number(input, "number")?.ceil())))
    }
}

/// ⭕️ Rounds a number.
pub struct Round;

impl Operation for Round {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("rounded", number_dictionary(read_channel_number(input, "number")?.round())))
    }
}

/// 〰 Sine in radians.
pub struct Sin;

impl Operation for Sin {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("sine", number_dictionary(read_channel_number(input, "number")?.sin())))
    }
}

/// 〰 Cosine in radians.
pub struct Cos;

impl Operation for Cos {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("cosine", number_dictionary(read_channel_number(input, "number")?.cos())))
    }
}

/// 〰 Tangent in radians.
pub struct Tan;

impl Operation for Tan {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("tangent", number_dictionary(read_channel_number(input, "number")?.tan())))
    }
}

/// 🗺️ Remaps a value from one range to another.
pub struct Remap;

impl Operation for Remap {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let value = read_channel_number(input, "value")?;
        let from_min = read_channel_number(input, "fromMin")?;
        let from_max = read_channel_number(input, "fromMax")?;
        let to_min = read_channel_number(input, "toMin")?;
        let to_max = read_channel_number(input, "toMax")?;
        let span = from_max - from_min;
        if span.abs() < f64::EPSILON {
            return Err(EvalError::InvalidInput("remap span is zero".into()));
        }
        Ok(channel_output("remapped", number_dictionary(to_min + ((value - from_min) / span) * (to_max - to_min))))
    }
}

/// 🎲️ Seeded or entropy-backed random number in [min, max].
pub struct Random;

impl Operation for Random {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let min = read_channel_number(input, "min")?;
        let max = read_channel_number(input, "max")?;
        let seed = read_channel_number(input, "seed").ok().map(f64::to_bits);
        Ok(channel_output("random", number_dictionary(min + (max - min) * next_random_unit(seed))))
    }
}

/// ➡️ Forwards the number input unchanged.
pub struct PassThrough;

impl Operation for PassThrough {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("number", number_dictionary(read_channel_number(input, "number")?)))
    }
}

/// ∑ Sums all numbers in a list dictionary.
pub struct Sum;

impl Operation for Sum {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let list = read_dict(input, "list")?;
        let mut total = 0.0;
        for index in list_indices(list) {
            let value = list.get(&index.to_string()).and_then(|v| v.as_dictionary()).map(read_value_number).transpose()?.ok_or_else(|| EvalError::MissingInput(index.to_string()))?;
            total += value;
        }
        Ok(channel_output("sum", number_dictionary(total)))
    }
}
// #endregion 🔖️Scalar

// #region 🔖️Helpers
#[derive(Clone, Copy, Debug, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

thread_local! {
    static ENTROPY_SEED: Cell<u64> = const { Cell::new(0) };
}

fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

fn xyz_dictionary(schema: &str, value: Vec3) -> Dictionary {
    Dictionary::with_schema(schema).insert("x", Value::Atom(Atom::Decimal(value.x))).insert("y", Value::Atom(Atom::Decimal(value.y))).insert("z", Value::Atom(Atom::Decimal(value.z)))
}

fn add_items(items: &Dictionary) -> Result<Dictionary, EvalError> {
    let mut indices: Vec<usize> = items.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
    indices.sort_unstable();
    let first = indices.first().ok_or_else(|| EvalError::MissingInput("items".into()))?;
    let first_dict = read_dict(items, &first.to_string())?;
    match first_dict.schema() {
        Some("point") | Some("vector") => {
            let mut total = Vec3::new(0.0, 0.0, 0.0);
            for index in indices {
                total = total + read_xyz(read_dict(items, &index.to_string())?)?;
            }
            Ok(channel_output("sum", xyz_dictionary(first_dict.schema().unwrap_or("vector"), total)))
        }
        _ => {
            let mut total = 0.0;
            for index in indices {
                total += read_value_number(read_dict(items, &index.to_string())?)?;
            }
            Ok(channel_output("sum", number_dictionary(total)))
        }
    }
}

fn read_dict<'a>(input: &'a Dictionary, key: &str) -> Result<&'a Dictionary, EvalError> {
    input.get(key).and_then(|v| v.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_channel_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    read_value_number(read_dict(input, key)?)
}

fn read_value_number(input: &Dictionary) -> Result<f64, EvalError> {
    input.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("value".into()))
}

fn read_xyz(input: &Dictionary) -> Result<Vec3, EvalError> {
    Ok(Vec3::new(read_field_number(input, "x")?, read_field_number(input, "y")?, read_field_number(input, "z")?))
}

fn read_field_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn list_indices(list: &Dictionary) -> Vec<usize> {
    let mut indices: Vec<usize> = list.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
    indices.sort_unstable();
    indices
}

fn splitmix64(seed: u64) -> u64 {
    let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn random_unit(seed: u64) -> f64 {
    splitmix64(seed) as f64 / u64::MAX as f64
}

fn entropy_seed() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        (js_sys::Math::random() * u64::MAX as f64) as u64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).map(|duration| duration.as_nanos() as u64).unwrap_or(0)
    }
}

fn next_random_unit(explicit_seed: Option<u64>) -> f64 {
    if let Some(seed) = explicit_seed {
        return random_unit(seed);
    }
    ENTROPY_SEED.with(|cell| {
        let seed = cell.get();
        let next = if seed == 0 { entropy_seed() } else { splitmix64(seed) };
        cell.set(next);
        random_unit(next)
    })
}

fn number_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::number_default(id, 0.0, &[operator_id])
}

fn sum_out() -> ChannelSpec {
    ChannelSpec::named("S", "Sum", "sum", "Sum")
}

fn difference_out() -> ChannelSpec {
    ChannelSpec::named("D", "Dif", "difference", "Difference")
}

fn product_out() -> ChannelSpec {
    ChannelSpec::named("P", "Prd", "product", "Product")
}

fn quotient_out() -> ChannelSpec {
    ChannelSpec::named("Q", "Quo", "quotient", "Quotient")
}

fn power_out() -> ChannelSpec {
    ChannelSpec::named("P", "Pow", "power", "Power")
}

fn modulo_out() -> ChannelSpec {
    ChannelSpec::named("M", "Mod", "modulo", "Modulo")
}

fn negated_out() -> ChannelSpec {
    ChannelSpec::named("N", "Neg", "negated", "Negated")
}

fn absolute_out() -> ChannelSpec {
    ChannelSpec::named("A", "Abs", "absolute", "Absolute")
}

fn root_out() -> ChannelSpec {
    ChannelSpec::named("R", "Roo", "root", "Root")
}

fn floor_out() -> ChannelSpec {
    ChannelSpec::named("F", "Flr", "floor", "Floor")
}

fn ceiling_out() -> ChannelSpec {
    ChannelSpec::named("C", "Ceil", "ceiling", "Ceiling")
}

fn rounded_out() -> ChannelSpec {
    ChannelSpec::named("R", "Rnd", "rounded", "Rounded")
}

fn sine_out() -> ChannelSpec {
    ChannelSpec::named("S", "Sin", "sine", "Sine")
}

fn cosine_out() -> ChannelSpec {
    ChannelSpec::named("C", "Cos", "cosine", "Cosine")
}

fn tangent_out() -> ChannelSpec {
    ChannelSpec::named("T", "Tan", "tangent", "Tangent")
}

fn number_out() -> ChannelSpec {
    ChannelSpec::named("N", "Num", "number", "Number")
}

fn minimum_out() -> ChannelSpec {
    ChannelSpec::named("Mi", "Min", "minimum", "Minimum")
}

fn maximum_out() -> ChannelSpec {
    ChannelSpec::named("Ma", "Max", "maximum", "Maximum")
}

fn remapped_out() -> ChannelSpec {
    ChannelSpec::named("R", "Rem", "remapped", "Remapped")
}

fn random_out() -> ChannelSpec {
    ChannelSpec::named("R", "Rnd", "random", "Random")
}

fn vector_out() -> ChannelSpec {
    ChannelSpec::named("V", "Vec", "vector", "Vector")
}

fn point_out() -> ChannelSpec {
    ChannelSpec::named("P", "Pnt", "point", "Point")
}

fn move_out() -> Vec<ChannelSpec> {
    vec![point_out(), vector_out()]
}

fn schema(id: &str, name: &str) -> Schema {
    Schema {
        id: id.into(),
        module: "math".into(),
        name: name.into(),
        icon: "emoji:🧭️".into(),
        summary: format!("{name} with x, y, z decimal fields"),
        fields: vec![FieldSpec::decimal_default("x", 0.0), FieldSpec::decimal_default("y", 0.0), FieldSpec::decimal_default("z", 0.0)],
    }
}

fn operator_info(id: &str, name: &str, abbreviation: &str, summary: &str, inputs: Vec<ChannelSpec>, outputs: Vec<ChannelSpec>) -> OperatorInfo {
    OperatorInfo { id: id.into(), module: "math".into(), name: name.into(), abbreviation: abbreviation.into(), icon: "emoji:➕️".into(), summary: summary.into(), inputs, outputs, ..Default::default() }
}

fn register_simple(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operation>, schemas: Vec<&str>, produces: &[&str]) {
    registry.register_operator(info, vec![OperatorImpl { schemas: schemas.into_iter().map(str::to_string).collect(), operation }], produces);
}

#[cfg(any(test, target_arch = "wasm32"))]
fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖️Helpers

/// 📦️ Registers all math schemas and operators.
pub fn register(registry: &mut Registry) {
    registry.register_schema(schema("point", "Point"));
    registry.register_schema(schema("vector", "Vector"));

    let scalar = vec![number_channel("a", "math.add"), number_channel("b", "math.add")];
    let sum_output = vec![sum_out()];
    registry.register_operator(
        operator_info("math.add", "Add", "Add", "Adds numbers, points, or vectors", scalar.clone(), sum_output.clone()),
        vec![
            OperatorImpl { schemas: vec!["number".into(), "number".into()], operation: Box::new(Add) },
            OperatorImpl { schemas: vec!["point".into(), "point".into()], operation: Box::new(Add) },
            OperatorImpl { schemas: vec!["vector".into(), "vector".into()], operation: Box::new(Add) },
        ],
        &["number", "point", "vector"],
    );
    registry.register_operator(
        OperatorInfo { variadic_input: Some(VariadicSpec { slot_key: "items".into(), min: 2, max: None }), ..operator_info("math.addVariadic", "Add Variadic", "Add", "Adds any number of numbers, points, or vectors", vec![], sum_output.clone()) },
        vec![
            OperatorImpl { schemas: vec!["number".into(), "number".into()], operation: Box::new(Add) },
            OperatorImpl { schemas: vec!["point".into(), "point".into()], operation: Box::new(Add) },
            OperatorImpl { schemas: vec!["vector".into(), "vector".into()], operation: Box::new(Add) },
        ],
        &["number", "point", "vector"],
    );
    let subtract_scalar = vec![number_channel("a", "math.subtract"), number_channel("b", "math.subtract")];
    registry.register_operator(
        operator_info("math.subtract", "Subtract", "Sub", "Subtracts numbers, points, or vectors", subtract_scalar.clone(), vec![difference_out()]),
        vec![
            OperatorImpl { schemas: vec!["number".into(), "number".into()], operation: Box::new(Subtract) },
            OperatorImpl { schemas: vec!["point".into(), "point".into()], operation: Box::new(Subtract) },
            OperatorImpl { schemas: vec!["vector".into(), "vector".into()], operation: Box::new(Subtract) },
        ],
        &["number", "point", "vector"],
    );
    let binary_scalar = vec![number_channel("a", "math.multiply"), number_channel("b", "math.multiply")];
    register_simple(registry, operator_info("math.multiply", "Multiply", "Mul", "Multiplies two numbers", binary_scalar.clone(), vec![product_out()]), Box::new(Multiply), vec!["number", "number"], &["number"]);
    register_simple(registry, operator_info("math.divide", "Divide", "Div", "Divides a by b", binary_scalar.clone(), vec![quotient_out()]), Box::new(Divide), vec!["number", "number"], &["number"]);
    register_simple(registry, operator_info("math.power", "Power", "Pow", "Raises a to the power of b", binary_scalar.clone(), vec![power_out()]), Box::new(Power), vec!["number", "number"], &["number"]);
    register_simple(registry, operator_info("math.modulo", "Modulo", "Mod", "Remainder of a divided by b", binary_scalar, vec![modulo_out()]), Box::new(Modulo), vec!["number", "number"], &["number"]);

    for (id, name, abbreviation, summary, output, operation) in [
        ("math.negate", "Negate", "Neg", "Negates a number", vec![negated_out()], Box::new(Negate) as Box<dyn Operation>),
        ("math.abs", "Abs", "Abs", "Absolute value", vec![absolute_out()], Box::new(Abs)),
        ("math.sqrt", "Sqrt", "Sqrt", "Square root", vec![root_out()], Box::new(Sqrt)),
        ("math.floor", "Floor", "Flr", "Floor of a number", vec![floor_out()], Box::new(Floor)),
        ("math.ceil", "Ceil", "Ceil", "Ceiling of a number", vec![ceiling_out()], Box::new(Ceil)),
        ("math.round", "Round", "Rnd", "Rounds a number", vec![rounded_out()], Box::new(Round)),
        ("math.sin", "Sin", "Sin", "Sine in radians", vec![sine_out()], Box::new(Sin)),
        ("math.cos", "Cos", "Cos", "Cosine in radians", vec![cosine_out()], Box::new(Cos)),
        ("math.tan", "Tan", "Tan", "Tangent in radians", vec![tangent_out()], Box::new(Tan)),
        ("math.passThrough", "PassThrough", "Pass", "Forwards a number", vec![number_out()], Box::new(PassThrough)),
    ] {
        register_simple(registry, operator_info(id, name, abbreviation, summary, vec![number_channel("number", id)], output), operation, vec!["number"], &["number"]);
    }

    register_simple(registry, operator_info("math.min", "Min", "Min", "Minimum of two numbers", vec![number_channel("a", "math.min"), number_channel("b", "math.min")], vec![minimum_out()]), Box::new(Min), vec!["number", "number"], &["number"]);
    register_simple(registry, operator_info("math.max", "Max", "Max", "Maximum of two numbers", vec![number_channel("a", "math.max"), number_channel("b", "math.max")], vec![maximum_out()]), Box::new(Max), vec!["number", "number"], &["number"]);
    register_simple(
        registry,
        operator_info(
            "math.remap",
            "Remap",
            "Map",
            "Remaps a value from one range to another",
            vec![number_channel("value", "math.remap"), number_channel("fromMin", "math.remap"), number_channel("fromMax", "math.remap"), number_channel("toMin", "math.remap"), number_channel("toMax", "math.remap")],
            vec![remapped_out()],
        ),
        Box::new(Remap),
        vec!["number", "number", "number", "number", "number"],
        &["number"],
    );
    register_simple(
        registry,
        operator_info(
            "math.random",
            "Random",
            "Rnd",
            "Random number in range with optional seed",
            vec![number_channel("seed", "math.random"), number_channel("min", "math.random"), ChannelSpec::number_default("max", 1.0, &["math.random"])],
            vec![random_out()],
        ),
        Box::new(Random),
        vec!["number", "number", "number"],
        &["number"],
    );
    register_simple(registry, operator_info("math.sum", "Sum", "Sum", "Sums numbers in a list dictionary", vec![ChannelSpec::list("list", &["math.sum"])], sum_output.clone()), Box::new(Sum), vec!["list"], &["number"]);
    registry.register_operator(
        operator_info("math.move", "Move", "Move", "Moves a point or vector by a vector", vec![ChannelSpec::requires("subject", &["math.move"]), ChannelSpec::requires("vector", &["math.move"])], move_out()),
        vec![OperatorImpl { schemas: vec!["point".into(), "vector".into()], operation: Box::new(Move) }, OperatorImpl { schemas: vec!["vector".into(), "vector".into()], operation: Box::new(Move) }],
        &["point", "vector"],
    );
    registry.finalize();
}

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_module_wasm::{build_manifest_json, evaluate_json, FlowModuleCommand, FlowModuleSetting};

    #[test]
    fn add_sums_number_dictionaries() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("a", Value::Dictionary(number_dictionary(3.0))).insert("b", Value::Dictionary(number_dictionary(1.1)));
        let out = reg.dispatch("math.add", &input).unwrap();
        let sum = out.get("sum").and_then(|v| v.as_dictionary()).expect("sum channel");
        assert_eq!(sum.schema(), Some("number"));
        assert_eq!(sum.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(4.1));
    }

    #[test]
    fn construct_vector_uses_xyz_channels() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("x", Value::Dictionary(number_dictionary(1.0))).insert("y", Value::Dictionary(number_dictionary(2.0))).insert("z", Value::Dictionary(number_dictionary(3.0)));
        let out = reg.dispatch("math.vector", &input).unwrap();
        let vector = out.get("vector").and_then(|v| v.as_dictionary()).expect("vector channel");
        assert_eq!(vector.schema(), Some("vector"));
        assert_eq!(vector.get("z").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    }

    #[test]
    fn schema_component_round_trips_vector() {
        let reg = module_registry();
        let built = reg.dispatch("math.vector", &Dictionary::new().insert("x", Value::Dictionary(number_dictionary(1.0))).insert("y", Value::Dictionary(number_dictionary(2.0))).insert("z", Value::Dictionary(number_dictionary(3.0)))).unwrap();
        let vector = built.get("vector").and_then(|value| value.as_dictionary()).expect("vector");
        let deconstructed = reg.dispatch("math.vector", &Dictionary::new().insert("vector", Value::Dictionary(vector.clone()))).unwrap();
        assert_eq!(deconstructed.get("y").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(2.0));
    }

    #[test]
    fn move_translates_point() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("subject", Value::Dictionary(xyz_dictionary("point", Vec3::new(1.0, 2.0, 3.0)))).insert("vector", Value::Dictionary(xyz_dictionary("vector", Vec3::new(4.0, 5.0, 6.0))));
        let out = reg.dispatch("math.move", &input).unwrap();
        let point = out.get("point").and_then(|v| v.as_dictionary()).expect("point channel");
        assert_eq!(point.schema(), Some("point"));
        assert_eq!(point.get("x").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(5.0));
    }

    #[test]
    fn manifest_lists_math_operators_and_schemas() {
        let json = build_manifest_json(
            "math",
            "Math",
            "0.2.0",
            &module_registry(),
            vec!["onStartup".into()],
            vec![],
            vec![FlowModuleCommand { id: "math.showHelp".into(), title: "Math: Show Help".into() }],
            vec![FlowModuleSetting { id: "math.defaultPrecision".into(), setting_type: "number".into(), default: serde_json::json!(1), description: "Decimal places for number preview".into() }],
        );
        assert!(json.contains("flow.module"));
        assert!(json.contains("math.vector"));
        assert!(json.contains("vector"));
    }

    #[test]
    fn evaluate_json_adds_numbers() {
        let reg = module_registry();
        let input = Dictionary::new().insert("a", Value::Dictionary(number_dictionary(2.0))).insert("b", Value::Dictionary(number_dictionary(1.0)));
        let out_json = evaluate_json(&reg, "math.add", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        let sum = out.get("sum").and_then(|v| v.as_dictionary()).expect("sum channel");
        assert_eq!(sum.schema(), Some("number"));
        assert_eq!(sum.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(3.0));
    }

    #[test]
    fn random_is_deterministic_with_seed() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("seed", Value::Dictionary(number_dictionary(42.0))).insert("min", Value::Dictionary(number_dictionary(0.0))).insert("max", Value::Dictionary(number_dictionary(1.0)));
        let first = reg.dispatch("math.random", &input).unwrap();
        let second = reg.dispatch("math.random", &input).unwrap();
        let first_value = first.get("random").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
        let second_value = second.get("random").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
        assert_eq!(first_value, second_value);
    }

    #[test]
    fn divide_rejects_zero() {
        let mut reg = Registry::new();
        register(&mut reg);
        let input = Dictionary::new().insert("a", Value::Dictionary(number_dictionary(1.0))).insert("b", Value::Dictionary(number_dictionary(0.0)));
        assert!(reg.dispatch("math.divide", &input).is_err());
    }
}
// #endregion 🔖️Tests

// #region 🔖️WasmExt
#[cfg(all(target_arch = "wasm32", feature = "standalone-wasm"))]
mod wasm_ext {
    use super::module_registry;
    use flow_module_wasm::{build_manifest_json, command_json, evaluate_json, FlowModuleCommand, FlowModuleSetting};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn manifest() -> String {
        build_manifest_json(
            "math",
            "Math",
            "0.2.0",
            &module_registry(),
            vec!["onStartup".into()],
            vec![],
            vec![FlowModuleCommand { id: "math.showHelp".into(), title: "Math: Show Help".into() }],
            vec![FlowModuleSetting { id: "math.defaultPrecision".into(), setting_type: "number".into(), default: serde_json::json!(1), description: "Decimal places for number preview".into() }],
        )
    }

    #[wasm_bindgen]
    pub fn evaluate(kind_id: &str, input_json: &str) -> String {
        evaluate_json(&module_registry(), kind_id, input_json)
    }

    #[wasm_bindgen]
    pub fn command(command_id: &str, args_json: &str) -> String {
        command_json(command_id, args_json)
    }

    #[wasm_bindgen]
    pub fn activate() {}

    #[wasm_bindgen]
    pub fn deactivate() {}
}
// #endregion 🔖️WasmExt
