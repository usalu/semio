//! 🧬️ Subject adapter of the schema-conformance case: the typed twins and `menagerie_issues`, `species_issues` and `ensemble_issues` of the `pets` crate judge every committed document.
//!
//! A document is judged by decoding it into its typed twin — which refuses a wrong JSON type, a missing or
//! undeclared member, a literal outside its enumeration and a tuple of the wrong length like the schema does — and
//! then by the owned validator. A document the twin refuses is rejected; the rule documents keep a sound structure,
//! so the twin decodes every one of them and the validator's findings are projected.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/✅️validation/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use pets::serde_json::{self, json, Map, Value};
    use pets::{ensemble_issues, menagerie_issues, species_issues, Ensemble, Issue, Menagerie, Species};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🧬️schema-conformance/🔣️.json";

    /// 🧫️ The committed vectors.
    fn vectors(ctx: &Context) -> Result<Value, String> {
        serde_json::from_slice(&ctx.input_bytes(VECTORS)?).map_err(|error| format!("{VECTORS}: {error}"))
    }

    /// 🗂️ One group of the committed vectors.
    fn group<'a>(document: &'a Value, name: &str) -> Result<&'a Vec<Value>, String> {
        document.get(name).and_then(Value::as_array).ok_or_else(|| format!("{VECTORS} carries no {name} group"))
    }

    /// 🔎️ The document of a vector: inline, or the value its JSON pointer reaches in the vectors.
    fn document_of<'a>(committed: &'a Value, vector: &'a Value) -> Result<&'a Value, String> {
        match vector.get("pointer").and_then(Value::as_str) {
            Some(pointer) => committed.pointer(pointer).ok_or_else(|| format!("{pointer} reaches nothing in {VECTORS}")),
            None => vector.get("document").ok_or_else(|| format!("vector {vector} carries neither a document nor a pointer")),
        }
    }

    /// ⚖️ The findings of the owned validator of the definition a vector names; `None` when the typed twin refuses the document.
    fn issues_of(vector: &Value, document: &Value) -> Option<Vec<Issue>> {
        match vector.get("definition").and_then(Value::as_str) {
            Some("Species") => serde_json::from_value::<Species>(document.clone()).ok().map(|species| species_issues(&species)),
            Some("Ensemble") => serde_json::from_value::<Ensemble>(document.clone()).ok().map(|ensemble| ensemble_issues(&ensemble)),
            _ => serde_json::from_value::<Menagerie>(document.clone()).ok().map(|menagerie| menagerie_issues(&menagerie)),
        }
    }

    /// 🗝️ The projection keyed by vector id.
    fn keyed(vectors: &[Value], answer: impl Fn(&Value) -> Result<Value, String>) -> Result<Outcome, String> {
        let mut projection = Map::new();
        for vector in vectors {
            let id = vector.get("id").and_then(Value::as_str).ok_or_else(|| format!("vector {vector} carries no id"))?;
            projection.insert(id.to_string(), answer(vector)?);
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }

    /// ✅️ Whether every accepted document decodes and has no finding.
    pub fn accepted_documents(ctx: &Context) -> Result<Outcome, String> {
        let committed = vectors(ctx)?;
        keyed(group(&committed, "accepted")?, |vector| Ok(json!(issues_of(vector, document_of(&committed, vector)?).is_some_and(|issues| issues.is_empty()))))
    }

    /// 🚫️ Whether a structurally broken document is accepted: never, by the twin or by the validator.
    pub fn structural_rejections(ctx: &Context) -> Result<Outcome, String> {
        let committed = vectors(ctx)?;
        keyed(group(&committed, "structural")?, |vector| Ok(json!(issues_of(vector, document_of(&committed, vector)?).is_some_and(|issues| issues.is_empty()))))
    }

    /// 📜️ The findings of every rule document, pointer and code, in code point order.
    pub fn rule_violations(ctx: &Context) -> Result<Outcome, String> {
        let committed = vectors(ctx)?;
        keyed(group(&committed, "rules")?, |vector| {
            let issues = issues_of(vector, document_of(&committed, vector)?).ok_or_else(|| format!("the typed twin refuses the rule document {}", vector["id"]))?;
            Ok(json!(issues))
        })
    }
}

/// 🧭️ Subject role only — the oracle is python-jsonschema in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("accepted-documents", subject::accepted_documents).subject("structural-rejections", subject::structural_rejections).subject("rule-violations", subject::rule_violations);
    built
}
