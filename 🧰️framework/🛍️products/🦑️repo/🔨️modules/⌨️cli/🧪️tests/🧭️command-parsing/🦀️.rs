//! 🦀️ Rust subject for the argv → command projection case. Gated behind the `sut` feature like
//! every subject, so the oracle role never links the crate under test.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_cli::repo_cli;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️Helpers
    /// 📥️ The committed vectors every scenario reads.
    fn vectors(ctx: &Context) -> Result<Vec<Json>, String> {
        Ok(ctx.fixture_json("local://🔣️argv-vectors.json")?.array("vectors"))
    }

    /// 📜️ A JSON string array as a `Vec<String>`.
    fn strings(value: &Json, key: &str) -> Vec<String> {
        value
            .array(key)
            .into_iter()
            .map(|item| match item {
                Json::String(text) => text,
                other => other.to_string(),
            })
            .collect()
    }

    /// 🧾️ One projection reduced to the members its vector states, so a vector expresses an intent
    /// rather than the whole default flag table.
    fn narrow(projection: &Json, stated: &Json) -> Json {
        let mut flags: Vec<(String, Json)> = Vec::new();
        if let Some(Json::Object(wanted)) = stated.get("flags") {
            let actual = projection.get("flags").cloned().unwrap_or(Json::Null);
            for (name, _) in wanted {
                flags.push((name.clone(), actual.get(name).cloned().unwrap_or(Json::Null)));
            }
        }
        Json::Object(vec![
            ("path".to_string(), projection.get("path").cloned().unwrap_or(Json::Null)),
            ("positional".to_string(), projection.get("positional").cloned().unwrap_or(Json::Null)),
            ("flags".to_string(), Json::Object(flags)),
            ("help".to_string(), projection.get("help").cloned().unwrap_or(Json::Null)),
        ])
    }
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    /// 🧭️ Every accepting vector projects into its stated path, operands and flags.
    pub fn argv_projects_into_a_command(ctx: &Context) -> Result<Outcome, String> {
        let mut rows: Vec<Json> = Vec::new();
        for vector in vectors(ctx)? {
            let Some(stated) = vector.get("expect") else { continue };
            let argv = strings(&vector, "argv");
            let encoded = repo_cli::projection(&argv).map_err(|message| format!("{}: unexpected refusal {message}", vector.str("id")))?;
            let actual = narrow(&parse_json(&encoded)?, stated);
            if actual.to_string() != stated.to_string() {
                return Err(format!("{}: expected {} got {}", vector.str("id"), stated.to_string(), actual.to_string()));
            }
            rows.push(Json::Object(vec![("id".to_string(), Json::String(vector.str("id"))), ("projection".to_string(), actual)]));
        }
        Ok(Outcome::projection(Json::Object(vec![("accepted".to_string(), Json::Array(rows))])))
    }

    /// ❌️ Every refusing vector carries its stated message verbatim.
    pub fn refusals_carry_their_verbatim_message(ctx: &Context) -> Result<Outcome, String> {
        let mut rows: Vec<Json> = Vec::new();
        for vector in vectors(ctx)? {
            if vector.get("error").is_none() {
                continue;
            }
            let argv = strings(&vector, "argv");
            let stated = vector.str("error");
            match repo_cli::projection(&argv) {
                Ok(_) => return Err(format!("{}: expected the refusal {stated}", vector.str("id"))),
                Err(message) if message == stated => rows.push(Json::Object(vec![("id".to_string(), Json::String(vector.str("id"))), ("message".to_string(), Json::String(message))])),
                Err(message) => return Err(format!("{}: expected {stated} got {message}", vector.str("id"))),
            }
        }
        Ok(Outcome::projection(Json::Object(vec![("refused".to_string(), Json::Array(rows))])))
    }

    /// 🔁️ Parsing the same argv twice produces the same projection.
    pub fn parsing_is_idempotent(ctx: &Context) -> Result<Outcome, String> {
        let mut rows: Vec<Json> = Vec::new();
        for vector in vectors(ctx)? {
            if vector.get("expect").is_none() {
                continue;
            }
            let argv = strings(&vector, "argv");
            if repo_cli::projection(&argv)? != repo_cli::projection(&argv)? {
                return Err(format!("{}: parsing is not idempotent", vector.str("id")));
            }
            rows.push(Json::String(vector.str("id")));
        }
        Ok(Outcome::projection(Json::Object(vec![("stable".to_string(), Json::Array(rows))])))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("argv-projects-into-a-command", subject::argv_projects_into_a_command)
        .subject("refusals-carry-their-verbatim-message", subject::refusals_carry_their_verbatim_message)
        .subject("parsing-is-idempotent", subject::parsing_is_idempotent);
    adapter
}
//#endregion 🔖️Registration
