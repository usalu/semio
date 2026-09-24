//! 🦀️ Rust side of the variable-coercion case. Coerces every ROOT selection's arguments the way the
//! executor does — variables first, declared defaults only for argument names that are absent.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{Context, Json, Outcome};
    use semio_framework_repo_graphql::serde_json::{Map, Value as SerdeJson};
    use semio_framework_repo_graphql::{coerce_arguments, parse};

    pub fn to_host(value: &SerdeJson) -> Json {
        match value {
            SerdeJson::Null => Json::Null,
            SerdeJson::Bool(flag) => Json::Bool(*flag),
            SerdeJson::Number(number) => Json::Number(number.as_f64().unwrap_or_default()),
            SerdeJson::String(text) => Json::String(text.clone()),
            SerdeJson::Array(items) => Json::Array(items.iter().map(to_host).collect()),
            SerdeJson::Object(fields) => Json::Object(fields.iter().map(|(name, item)| (name.clone(), to_host(item))).collect()),
        }
    }

    struct Case {
        id: String,
        source: String,
        variables: Map<String, SerdeJson>,
        defaults: Map<String, SerdeJson>,
    }

    fn corpus(ctx: &Context) -> Result<Vec<Case>, String> {
        let bytes = ctx.fixture_bytes("shared://🔀️variable-coercion/🔣️coercions.json")?;
        let text = String::from_utf8(bytes).map_err(|error| error.to_string())?;
        let parsed: SerdeJson = semio_framework_repo_graphql::serde_json::from_str(&text).map_err(|error| error.to_string())?;
        let cases = parsed.get("cases").and_then(SerdeJson::as_array).ok_or("corpus has no cases array")?;
        cases
            .iter()
            .map(|entry| {
                Ok(Case {
                    id: entry.get("id").and_then(SerdeJson::as_str).ok_or("case has no id")?.to_string(),
                    source: entry.get("source").and_then(SerdeJson::as_str).ok_or("case has no source")?.to_string(),
                    variables: entry.get("variables").and_then(SerdeJson::as_object).cloned().unwrap_or_default(),
                    defaults: entry.get("defaults").and_then(SerdeJson::as_object).cloned().unwrap_or_default(),
                })
            })
            .collect()
    }

    fn coerce(ctx: &Context, only_with_defaults: bool) -> Result<Outcome, String> {
        let rows = corpus(ctx)?
            .into_iter()
            .filter(|case| !only_with_defaults || !case.defaults.is_empty())
            .map(|case| {
                let document = parse(&case.source).map_err(|error| format!("{}: {error}", case.id))?;
                let selections = document
                    .selections
                    .iter()
                    .map(|selection| {
                        let args = coerce_arguments(&selection.arguments, &case.defaults, &case.variables);
                        let arguments = args.iter().map(|(name, value)| Json::Object(vec![("name".to_string(), Json::String(name.clone())), ("value".to_string(), to_host(value))])).collect::<Vec<_>>();
                        Json::Object(vec![("key".to_string(), Json::String(selection.key().to_string())), ("arguments".to_string(), Json::Array(arguments))])
                    })
                    .collect::<Vec<_>>();
                Ok(Json::Object(vec![("id".to_string(), Json::String(case.id)), ("selections".to_string(), Json::Array(selections))]))
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(Outcome::projection(Json::Object(vec![("cases".to_string(), Json::Array(rows))])))
    }

    pub fn arguments_resolve_against_variables(ctx: &Context) -> Result<Outcome, String> {
        coerce(ctx, false)
    }

    pub fn defaults_fill_only_absent_arguments(ctx: &Context) -> Result<Outcome, String> {
        coerce(ctx, true)
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let adapter = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("arguments-resolve-against-variables", subject::arguments_resolve_against_variables)
        .subject("defaults-fill-only-absent-arguments", subject::defaults_fill_only_absent_arguments);
    adapter
}
//#endregion 🔖️Registration
