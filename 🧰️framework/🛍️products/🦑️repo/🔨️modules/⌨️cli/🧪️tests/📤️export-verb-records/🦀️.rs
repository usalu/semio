//! 🦀️ Rust subject for the `export` verb batch case. Gated behind the `sut` feature like every
//! subject, so the oracle role never links the crate under test.

#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_cli::repo_cli;
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️Helpers
    /// 📥️ The frozen repository every scenario exports.
    fn records(ctx: &Context) -> Result<String, String> {
        Ok(ctx.fixture_json("local://🗄️repo-records.json")?.to_string())
    }

    /// 📦️ The batch the verb would append.
    fn batch(ctx: &Context) -> Result<Json, String> {
        parse_json(&repo_cli::export_records_json(&records(ctx)?)?)
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
    //#endregion 🔖️Helpers

    //#region 🔖️Scenarios
    /// 🧾️ The batch carries one count per entity kind and a sha-256 digest.
    pub fn the_batch_states_its_counts(ctx: &Context) -> Result<Outcome, String> {
        let batch = batch(ctx)?;
        let digest = batch.str("snapshot");
        if digest.len() != 64 || !digest.chars().all(|value| value.is_ascii_hexdigit()) {
            return Err(format!("the snapshot digest is not a sha-256: {digest}"));
        }
        Ok(Outcome::projection(Json::Object(vec![("counts".to_string(), batch.get("counts").cloned().unwrap_or(Json::Null)), ("digestIsSha256".to_string(), Json::Bool(true))])))
    }

    /// 🪪️ Every input id carries the snapshot digest and its entity kind, and the ids are sorted.
    pub fn every_input_id_is_namespaced_by_the_digest(ctx: &Context) -> Result<Outcome, String> {
        let batch = batch(ctx)?;
        let digest = batch.str("snapshot");
        let ids = strings(&batch, "inputIds");
        let prefix = format!("snapshot:{digest}:");
        for id in &ids {
            let Some(rest) = id.strip_prefix(&prefix) else { return Err(format!("input id {id} is not namespaced by the snapshot digest")) };
            let kind = rest.split_once(':').map(|(kind, _)| kind).unwrap_or_default();
            if !["technology", "bundle", "folder", "file", "section", "definition"].contains(&kind) {
                return Err(format!("input id {id} names the unknown entity kind {kind:?}"));
            }
        }
        let mut sorted = ids.clone();
        sorted.sort();
        if sorted != ids {
            return Err("the input ids are not sorted".to_string());
        }
        Ok(Outcome::projection(Json::Object(vec![("inputIds".to_string(), Json::Array(ids.into_iter().map(|id| Json::String(id.replace(&prefix, "snapshot:<digest>:"))).collect()))])))
    }

    /// 🔁️ Building the batch twice produces the same digest and the same ids.
    pub fn the_batch_is_deterministic(ctx: &Context) -> Result<Outcome, String> {
        let records = records(ctx)?;
        let first = repo_cli::export_records_json(&records)?;
        let second = repo_cli::export_records_json(&records)?;
        if first != second {
            return Err("the export batch is not deterministic".to_string());
        }
        Ok(Outcome::projection(Json::Object(vec![("stable".to_string(), Json::Bool(true))])))
    }

    /// 🔏️ The snapshot digest and every encoded record, stated so the two implementations compare.
    pub fn the_digest_and_the_encoded_records_agree(ctx: &Context) -> Result<Outcome, String> {
        let batch = batch(ctx)?;
        Ok(Outcome::projection(Json::Object(vec![
            ("snapshot".to_string(), Json::String(batch.str("snapshot"))),
            ("records".to_string(), Json::Array(strings(&batch, "records").into_iter().map(Json::String).collect())),
        ])))
    }
    //#endregion 🔖️Scenarios
}

//#region 🔖️Registration
/// 📤️ Registration entry point the generated host calls.
pub fn adapter() -> semio_repo_test_host::Adapter {
    let adapter = semio_repo_test_host::Adapter::new("rust");
    #[cfg(feature = "sut")]
    let adapter = adapter
        .subject("the-batch-states-its-counts", subject::the_batch_states_its_counts)
        .subject("every-input-id-is-namespaced-by-the-digest", subject::every_input_id_is_namespaced_by_the_digest)
        .subject("the-batch-is-deterministic", subject::the_batch_is_deterministic)
        .subject("the-digest-and-the-encoded-records-agree", subject::the_digest_and_the_encoded_records_agree);
    adapter
}
//#endregion 🔖️Registration
