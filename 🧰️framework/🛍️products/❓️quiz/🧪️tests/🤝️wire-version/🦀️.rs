//! 🤝️ Subject adapter of the wire-version case: `WIRE_VERSION` of the `quiz` crate and the fingerprint of the schema
//! without its prose, written by `serde_json`.
//!
//! @see ./🥒️.feature
//! @see ../../🧬️schema/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, Value};
    use quiz::WIRE_VERSION;
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const SCHEMA: [&str; 5] = ["🧰️framework", "🛍️products", "❓️quiz", "🧬️schema", "🔣️.json"];
    const PROSE: [&str; 3] = ["description", "title", "$comment"];

    /// ✍️ `node` without its prose as JSON with sorted keys and no whitespace, appended to `into`.
    fn canonical(node: &Value, into: &mut String) {
        match node {
            Value::Array(items) => {
                into.push('[');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        into.push(',');
                    }
                    canonical(item, into);
                }
                into.push(']');
            }
            Value::Object(members) => {
                let mut kept: Vec<(&String, &Value)> = members.iter().filter(|(key, value)| !(PROSE.contains(&key.as_str()) && value.is_string())).collect();
                kept.sort_by(|(left, _), (right, _)| left.cmp(right));
                into.push('{');
                for (index, (key, value)) in kept.into_iter().enumerate() {
                    if index > 0 {
                        into.push(',');
                    }
                    into.push_str(&Value::String(key.clone()).to_string());
                    into.push(':');
                    canonical(value, into);
                }
                into.push('}');
            }
            other => into.push_str(&other.to_string()),
        }
    }

    /// 🔢️ FNV-1a 64 of `bytes` as 16 lowercase hex digits.
    fn fnv1a64(bytes: &[u8]) -> String {
        let hashed = bytes.iter().fold(0xcbf2_9ce4_8422_2325_u64, |hashed, byte| (hashed ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3));
        format!("{hashed:016x}")
    }

    /// 🤝️ The wire version of the crate and the fingerprint of the schema without its prose.
    pub fn agreement(ctx: &Context) -> Result<Outcome, String> {
        let path = SCHEMA.iter().fold(ctx.repo_root.clone(), |path, segment| path.join(segment));
        let schema: Value = serde_json::from_slice(&std::fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?).map_err(|error| format!("{}: {error}", path.display()))?;
        let mut text = String::new();
        canonical(&schema, &mut text);
        Ok(Outcome::projection(parse_json(&serde_json::json!({ "wireVersion": WIRE_VERSION, "fingerprint": fnv1a64(text.as_bytes()) }).to_string())?))
    }
}

/// 🧭️ Subject role only — the oracle is Python's own JSON writer with an FNV-1a from its definition in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("agreement", subject::agreement);
    built
}
