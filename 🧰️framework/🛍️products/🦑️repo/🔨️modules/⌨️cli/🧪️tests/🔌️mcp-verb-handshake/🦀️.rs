//! 🦀️ Rust subject for the `mcp` verb handshake. Gated behind the `sut` feature like every
//! subject, so the oracle role never links the crate under test.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_cli::repo_cli;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use std::io::Write;
    use std::path::PathBuf;
    use std::process::{Command, Stdio};

    //#region 🔖️Helpers
    /// 📥️ The committed conversation every scenario reads.
    fn conversation(ctx: &Context) -> Result<Json, String> {
        ctx.fixture_json("shared://🔌️mcp-verb-handshake/🤝️handshake.json")
    }

    /// 🔎️ The built `semio` binary, release first, then debug.
    fn binary() -> Result<PathBuf, String> {
        let name = if cfg!(windows) { "semio.exe" } else { "semio" };
        let root = std::env::current_dir().map_err(|error| error.to_string())?;
        for profile in ["release", "debug"] {
            let candidate = root.join("target").join(profile).join(name);
            if candidate.is_file() {
                return Ok(candidate);
            }
        }
        Err(format!("neither target/release/{name} nor target/debug/{name} exists — build the `semio` binary before running this case"))
    }

    /// 📃️ The string members of an array.
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

    /// ▶️ Runs the stated conversation against `semio mcp` and returns its response lines.
    fn run_conversation(document: &Json) -> Result<Vec<Json>, String> {
        let binary = binary()?;
        let mut child = Command::new(&binary)
            .arg("mcp")
            .env(document.str("profileEnvironment"), document.str("profile"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| format!("{}: {error}", binary.display()))?;
        {
            let stdin = child.stdin.as_mut().ok_or("the server accepted no standard input")?;
            for request in strings(document, "requests") {
                writeln!(stdin, "{request}").map_err(|error| error.to_string())?;
            }
        }
        let output = child.wait_with_output().map_err(|error| error.to_string())?;
        let body = String::from_utf8_lossy(&output.stdout).to_string();
        let mut responses = Vec::new();
        for line in body.lines().filter(|line| !line.trim().is_empty()) {
            responses.push(parse_json(line)?);
        }
        if responses.is_empty() {
            return Err(format!("the server answered nothing; its standard error was {:?}", String::from_utf8_lossy(&output.stderr)));
        }
        Ok(responses)
    }

    /// 🔎️ The response carrying a given identifier.
    fn response(responses: &[Json], id: f64) -> Option<&Json> {
        responses.iter().find(|response| matches!(response.get("id"), Some(Json::Number(value)) if *value == id))
    }

    /// 📛️ The `name` member of every object in an array member of a result.
    fn names(result: &Json, key: &str) -> Vec<String> {
        result.array(key).iter().map(|entry| entry.str("name")).collect()
    }
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    /// 🤝️ The verb answers initialize, tools/list and resources/list.
    pub fn the_verb_completes_the_handshake(ctx: &Context) -> Result<Outcome, String> {
        let document = conversation(ctx)?;
        let expect = document.get("expect").cloned().ok_or("the conversation states no expectation")?;
        let responses = run_conversation(&document)?;

        let initialize = response(&responses, 1.0).ok_or("no initialize response")?;
        let result = initialize.get("result").cloned().ok_or_else(|| format!("initialize failed: {}", initialize.to_string()))?;
        if result.str("protocolVersion") != expect.str("protocolVersion") {
            return Err(format!("protocol version {:?} != {:?}", result.str("protocolVersion"), expect.str("protocolVersion")));
        }
        let server = result.get("serverInfo").cloned().ok_or("no serverInfo")?;
        if server.str("name") != expect.str("serverName") || server.str("version") != expect.str("serverVersion") {
            return Err(format!("serverInfo {} does not match the stated one", server.to_string()));
        }
        let capabilities = match result.get("capabilities") {
            Some(Json::Object(entries)) => entries.iter().map(|(name, _)| name.clone()).collect::<Vec<String>>(),
            _ => Vec::new(),
        };
        let mut sorted = capabilities.clone();
        sorted.sort();
        if sorted != strings(&expect, "capabilities") {
            return Err(format!("capabilities {sorted:?} != {:?}", strings(&expect, "capabilities")));
        }

        let tools = names(&response(&responses, 2.0).ok_or("no tools/list response")?.get("result").cloned().ok_or("tools/list failed")?, "tools");
        if tools != strings(&expect, "tools") {
            return Err(format!("tools {tools:?} != {:?}", strings(&expect, "tools")));
        }
        let listed = response(&responses, 3.0).ok_or("no resources/list response")?.get("result").cloned().ok_or("resources/list failed")?;
        let resources: Vec<String> = listed.array("resources").iter().map(|entry| entry.str("uri")).collect();
        if resources != strings(&expect, "resources") {
            return Err(format!("resources {resources:?} != {:?}", strings(&expect, "resources")));
        }

        Ok(Outcome::projection(Json::Object(vec![
            ("protocolVersion".to_string(), Json::String(result.str("protocolVersion"))),
            ("serverName".to_string(), Json::String(server.str("name"))),
            ("capabilities".to_string(), Json::Array(sorted.into_iter().map(Json::String).collect())),
            ("tools".to_string(), Json::Array(tools.into_iter().map(Json::String).collect())),
            ("resources".to_string(), Json::Array(resources.into_iter().map(Json::String).collect())),
        ])))
    }

    /// 🙈️ An initialize request carrying an unknown member is still accepted.
    pub fn initialize_ignores_unknown_members(ctx: &Context) -> Result<Outcome, String> {
        let document = conversation(ctx)?;
        let responses = run_conversation(&document)?;
        let initialize = response(&responses, 1.0).ok_or("no initialize response")?;
        if initialize.get("error").is_some() {
            return Err(format!("initialize refused an unknown member: {}", initialize.to_string()));
        }
        Ok(Outcome::projection(Json::Object(vec![("lenient".to_string(), Json::Bool(true))])))
    }

    /// 🏜️ `--dry-run` initializes the verb and exits without serving.
    pub fn the_dry_run_starts_no_server(_ctx: &Context) -> Result<Outcome, String> {
        let code = repo_cli::run(&["mcp".to_string(), "--dry-run".to_string()]);
        if code != 0 {
            return Err(format!("--dry-run exited with {code}"));
        }
        Ok(Outcome::projection(Json::Object(vec![("exitCode".to_string(), Json::Number(0.0))])))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("the-verb-completes-the-handshake", subject::the_verb_completes_the_handshake)
        .subject("initialize-ignores-unknown-members", subject::initialize_ignores_unknown_members)
        .subject("the-dry-run-starts-no-server", subject::the_dry_run_starts_no_server);
    adapter
}
//#endregion 🔖️Registration
