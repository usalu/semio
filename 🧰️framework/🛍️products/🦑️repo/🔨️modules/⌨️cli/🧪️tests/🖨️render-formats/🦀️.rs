//! 🦀️ Rust subject for the stream-rendering case. Gated behind the `sut` feature like every
//! subject, so the oracle role never links the crate under test.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_cli::repo_cli;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️Helpers
    /// 📥️ The committed vectors every scenario reads.
    fn vectors(ctx: &Context) -> Result<Vec<Json>, String> {
        Ok(ctx.fixture_json("local://📡️event-streams.json")?.array("vectors"))
    }

    /// 🔢️ A numeric member, zero when absent.
    fn number(value: &Json, key: &str) -> i64 {
        match value.get(key) {
            Some(Json::Number(number)) => *number as i64,
            _ => 0,
        }
    }

    /// ☑️ A boolean member, false when absent.
    fn flag(value: &Json, key: &str) -> bool {
        matches!(value.get(key), Some(Json::Bool(true)))
    }

    /// 🖨️ Renders one vector in a given format.
    fn render(vector: &Json, format: &str, is_tty: bool) -> Result<Json, String> {
        let events = vector.get("events").cloned().unwrap_or(Json::Array(Vec::new())).to_string();
        let encoded = repo_cli::render_stream_json(&events, format, is_tty, flag(vector, "verbose"), number(vector, "elapsedMs"))?;
        parse_json(&encoded)
    }
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    /// 🖨️ Every vector renders exactly the stated bytes.
    pub fn every_stream_renders_its_stated_bytes(ctx: &Context) -> Result<Outcome, String> {
        let mut rows: Vec<Json> = Vec::new();
        for vector in vectors(ctx)? {
            let stated = vector.get("expect").cloned().ok_or_else(|| format!("{}: no expectation", vector.str("id")))?;
            let actual = render(&vector, &vector.str("format"), flag(&vector, "isTty"))?;
            for member in ["out", "err"] {
                if actual.str(member) != stated.str(member) {
                    return Err(format!("{}: {member} expected {:?} got {:?}", vector.str("id"), stated.str(member), actual.str(member)));
                }
            }
            if number(&actual, "exitCode") != number(&stated, "exitCode") {
                return Err(format!("{}: exit code expected {} got {}", vector.str("id"), number(&stated, "exitCode"), number(&actual, "exitCode")));
            }
            rows.push(Json::Object(vec![("id".to_string(), Json::String(vector.str("id"))), ("rendered".to_string(), actual)]));
        }
        Ok(Outcome::projection(Json::Object(vec![("rendered".to_string(), Json::Array(rows))])))
    }

    /// 🚫️ No escape sequence survives when the output is not a terminal.
    pub fn no_colour_without_a_terminal(ctx: &Context) -> Result<Outcome, String> {
        let mut rows: Vec<Json> = Vec::new();
        for vector in vectors(ctx)? {
            if vector.str("format") != "text" {
                continue;
            }
            let actual = render(&vector, "text", false)?;
            let body = format!("{}{}", actual.str("out"), actual.str("err"));
            if body.contains('\u{1b}') {
                return Err(format!("{}: an escape sequence survived a non-terminal render", vector.str("id")));
            }
            rows.push(Json::Object(vec![("id".to_string(), Json::String(vector.str("id"))), ("plain".to_string(), Json::String(body))]));
        }
        Ok(Outcome::projection(Json::Object(vec![("plain".to_string(), Json::Array(rows))])))
    }

    /// 🔚️ Every renderer reports the exit code the terminal event carried.
    pub fn the_exit_code_follows_the_done_event(ctx: &Context) -> Result<Outcome, String> {
        let mut rows: Vec<Json> = Vec::new();
        for vector in vectors(ctx)? {
            let mut wanted = 0i64;
            for event in vector.array("events") {
                if let Some(done) = event.get("done") {
                    wanted = number(done, "exit_code");
                }
            }
            for format in ["json", "text", "md"] {
                let actual = number(&render(&vector, format, false)?, "exitCode");
                if actual != wanted {
                    return Err(format!("{}: {format} reported {actual}, the done event carried {wanted}", vector.str("id")));
                }
            }
            rows.push(Json::Object(vec![("id".to_string(), Json::String(vector.str("id"))), ("exitCode".to_string(), Json::Number(wanted as f64))]));
        }
        Ok(Outcome::projection(Json::Object(vec![("codes".to_string(), Json::Array(rows))])))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("every-stream-renders-its-stated-bytes", subject::every_stream_renders_its_stated_bytes)
        .subject("no-colour-without-a-terminal", subject::no_colour_without_a_terminal)
        .subject("the-exit-code-follows-the-done-event", subject::the_exit_code_follows_the_done_event);
    adapter
}
//#endregion 🔖️Registration
