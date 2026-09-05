//! 🧬️ Owned wire protocol of the repository test platform: the plan a host receives, the result it
//! emits, and the minimal JSON reader/writer both are expressed in. Deliberately dependency-free —
//! a test host must never be the reason an external crate enters the graph.

//#region 🔖️Json
/// 🔣️ Owned JSON value. The whole protocol is expressed in this type so no external serialization
/// crate is reachable from a test host.
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl Json {
    /// 🔎️ Object member lookup; `None` for a non-object or an absent key.
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Object(entries) => entries.iter().find(|(name, _)| name == key).map(|(_, value)| value),
            _ => None,
        }
    }

    /// 🔎️ String value, or `""` for anything else — plans are validated before a host ever sees them.
    pub fn str(&self, key: &str) -> String {
        match self.get(key) {
            Some(Json::String(value)) => value.clone(),
            _ => String::new(),
        }
    }

    /// 🔎️ Array members, or an empty slice.
    pub fn array(&self, key: &str) -> Vec<Json> {
        match self.get(key) {
            Some(Json::Array(items)) => items.clone(),
            _ => Vec::new(),
        }
    }

    /// 🧵️ Serializes to compact JSON with escaped control characters and `\u` escapes where required.
    pub fn to_string(&self) -> String {
        let mut out = String::new();
        self.write(&mut out);
        out
    }

    fn write(&self, out: &mut String) {
        match self {
            Json::Null => out.push_str("null"),
            Json::Bool(value) => out.push_str(if *value { "true" } else { "false" }),
            Json::Number(value) => {
                if value.is_finite() && value.fract() == 0.0 && value.abs() < 1e15 {
                    out.push_str(&format!("{}", *value as i64));
                } else if value.is_finite() {
                    out.push_str(&format!("{}", value));
                } else {
                    out.push_str("null");
                }
            }
            Json::String(value) => write_string(value, out),
            Json::Array(items) => {
                out.push('[');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    item.write(out);
                }
                out.push(']');
            }
            Json::Object(entries) => {
                out.push('{');
                for (index, (key, value)) in entries.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    write_string(key, out);
                    out.push(':');
                    value.write(out);
                }
                out.push('}');
            }
        }
    }
}

fn write_string(value: &str, out: &mut String) {
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

/// 🔣️ Parses one JSON document. Errors carry a byte offset so a malformed plan fails loudly.
pub fn parse_json(source: &str) -> Result<Json, String> {
    let bytes: Vec<char> = source.chars().collect();
    let mut cursor = 0usize;
    let value = parse_value(&bytes, &mut cursor)?;
    skip_whitespace(&bytes, &mut cursor);
    if cursor != bytes.len() {
        return Err(format!("trailing input at char {}", cursor));
    }
    Ok(value)
}

fn skip_whitespace(bytes: &[char], cursor: &mut usize) {
    while *cursor < bytes.len() && matches!(bytes[*cursor], ' ' | '\t' | '\n' | '\r') {
        *cursor += 1;
    }
}

fn parse_value(bytes: &[char], cursor: &mut usize) -> Result<Json, String> {
    skip_whitespace(bytes, cursor);
    match bytes.get(*cursor) {
        None => Err("unexpected end of input".to_string()),
        Some('n') => expect_literal(bytes, cursor, "null", Json::Null),
        Some('t') => expect_literal(bytes, cursor, "true", Json::Bool(true)),
        Some('f') => expect_literal(bytes, cursor, "false", Json::Bool(false)),
        Some('"') => parse_string(bytes, cursor).map(Json::String),
        Some('[') => parse_array(bytes, cursor),
        Some('{') => parse_object(bytes, cursor),
        Some(_) => parse_number(bytes, cursor),
    }
}

fn expect_literal(bytes: &[char], cursor: &mut usize, literal: &str, value: Json) -> Result<Json, String> {
    for expected in literal.chars() {
        if bytes.get(*cursor) != Some(&expected) {
            return Err(format!("expected {} at char {}", literal, cursor));
        }
        *cursor += 1;
    }
    Ok(value)
}

fn parse_string(bytes: &[char], cursor: &mut usize) -> Result<String, String> {
    if bytes.get(*cursor) != Some(&'"') {
        return Err(format!("expected string at char {}", cursor));
    }
    *cursor += 1;
    let mut out = String::new();
    loop {
        match bytes.get(*cursor) {
            None => return Err("unterminated string".to_string()),
            Some('"') => {
                *cursor += 1;
                return Ok(out);
            }
            Some('\\') => {
                *cursor += 1;
                let escape = *bytes.get(*cursor).ok_or("unterminated escape")?;
                *cursor += 1;
                match escape {
                    '"' => out.push('"'),
                    '\\' => out.push('\\'),
                    '/' => out.push('/'),
                    'b' => out.push('\u{8}'),
                    'f' => out.push('\u{c}'),
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    'u' => {
                        let code = read_hex4(bytes, cursor)?;
                        if (0xD800..0xDC00).contains(&code) {
                            if bytes.get(*cursor) == Some(&'\\') && bytes.get(*cursor + 1) == Some(&'u') {
                                *cursor += 2;
                                let low = read_hex4(bytes, cursor)?;
                                let combined = 0x10000 + ((code - 0xD800) << 10) + (low - 0xDC00);
                                out.push(char::from_u32(combined).ok_or("invalid surrogate pair")?);
                            } else {
                                return Err("lone high surrogate".to_string());
                            }
                        } else {
                            out.push(char::from_u32(code).ok_or("invalid escape")?);
                        }
                    }
                    other => return Err(format!("unknown escape \\{}", other)),
                }
            }
            Some(ch) => {
                out.push(*ch);
                *cursor += 1;
            }
        }
    }
}

fn read_hex4(bytes: &[char], cursor: &mut usize) -> Result<u32, String> {
    let mut code = 0u32;
    for _ in 0..4 {
        let digit = bytes.get(*cursor).and_then(|ch| ch.to_digit(16)).ok_or("bad \\u escape")?;
        code = code * 16 + digit;
        *cursor += 1;
    }
    Ok(code)
}

fn parse_array(bytes: &[char], cursor: &mut usize) -> Result<Json, String> {
    *cursor += 1;
    let mut items = Vec::new();
    skip_whitespace(bytes, cursor);
    if bytes.get(*cursor) == Some(&']') {
        *cursor += 1;
        return Ok(Json::Array(items));
    }
    loop {
        items.push(parse_value(bytes, cursor)?);
        skip_whitespace(bytes, cursor);
        match bytes.get(*cursor) {
            Some(',') => *cursor += 1,
            Some(']') => {
                *cursor += 1;
                return Ok(Json::Array(items));
            }
            _ => return Err(format!("expected , or ] at char {}", cursor)),
        }
    }
}

fn parse_object(bytes: &[char], cursor: &mut usize) -> Result<Json, String> {
    *cursor += 1;
    let mut entries = Vec::new();
    skip_whitespace(bytes, cursor);
    if bytes.get(*cursor) == Some(&'}') {
        *cursor += 1;
        return Ok(Json::Object(entries));
    }
    loop {
        skip_whitespace(bytes, cursor);
        let key = parse_string(bytes, cursor)?;
        skip_whitespace(bytes, cursor);
        if bytes.get(*cursor) != Some(&':') {
            return Err(format!("expected : at char {}", cursor));
        }
        *cursor += 1;
        entries.push((key, parse_value(bytes, cursor)?));
        skip_whitespace(bytes, cursor);
        match bytes.get(*cursor) {
            Some(',') => *cursor += 1,
            Some('}') => {
                *cursor += 1;
                return Ok(Json::Object(entries));
            }
            _ => return Err(format!("expected , or }} at char {}", cursor)),
        }
    }
}

fn parse_number(bytes: &[char], cursor: &mut usize) -> Result<Json, String> {
    let start = *cursor;
    if bytes.get(*cursor) == Some(&'-') {
        *cursor += 1;
    }
    while matches!(bytes.get(*cursor), Some(ch) if ch.is_ascii_digit() || *ch == '.' || *ch == 'e' || *ch == 'E' || *ch == '+' || *ch == '-') {
        *cursor += 1;
    }
    let text: String = bytes[start..*cursor].iter().collect();
    text.parse::<f64>().map(Json::Number).map_err(|_| format!("invalid number {:?} at char {}", text, start))
}
//#endregion 🔖️Json

//#region 🔖️Digest
const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// #⃣ Owned SHA-256, truncated to 32 hex characters — byte-identical to the coordinator's `digest()`
/// so a Rust host's `projectionHash` is directly comparable with a TypeScript host's.
pub fn digest(input: &[u8]) -> String {
    sha256_hex(input)[..32].to_string()
}

/// #⃣ The FULL 64-character SHA-256 hex. Protocol v2 addresses fixture blobs and result artifacts by
/// content, and a truncated digest is not a content address — the store's whole safety argument is
/// that a blob's name IS its content.
pub fn sha256_hex(input: &[u8]) -> String {
    let mut state: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let mut message = input.to_vec();
    let bit_len = (input.len() as u64) * 8;
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in message.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i * 4], chunk[i * 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) = (state[0], state[1], state[2], state[3], state[4], state[5], state[6], state[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(SHA256_K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        for (slot, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *slot = slot.wrapping_add(value);
        }
    }
    state.iter().map(|word| format!("{:08x}", word)).collect::<String>()
}
//#endregion 🔖️Digest

//#region 🔖️Plan
/// 🧫️ One immutable fixture the coordinator resolved for this case.
#[derive(Debug, Clone)]
pub struct Fixture {
    pub uri: String,
    pub scope: String,
    pub name: String,
    pub path: String,
    pub digest: String,
}

/// 🥒️ One planned scenario, already expanded and level-filtered by the coordinator.
#[derive(Debug, Clone)]
pub struct Scenario {
    pub id: String,
    pub name: String,
    pub level: String,
    pub mode: String,
    pub seed: String,
    pub steps: Vec<(String, String)>,
    /// 📜️ Doc strings attached to this scenario's steps, in step order. A feature carries its own
    /// vectors, so an adapter reads its input from here rather than hard-coding it.
    pub doc_strings: Vec<String>,
    /// 📊️ Data tables attached to this scenario's steps, in step order — header row first.
    pub data_tables: Vec<Vec<Vec<String>>>,
}

/// 🪆️ The smallest owning subset a case is scoped to. A case with no target is UNSCOPED, and Protocol
/// v2 reports that rather than letting a host widen itself to the whole artifact.
#[derive(Debug, Clone, Default)]
pub struct SubsetTarget {
    pub artifact: String,
    pub standard: String,
    pub subset: String,
}

/// 📋️ The owned execution plan. A host never parses a feature file — it executes exactly this.
#[derive(Debug, Clone)]
pub struct Plan {
    pub schema_version: u32,
    pub baseline_sha: String,
    pub owner: String,
    pub case: String,
    pub capability: String,
    pub comparison: String,
    /// ⚖️ The multi-artifact, externally-probed pipeline this case compares under; empty for a
    /// projection-only case.
    pub comparison_pipeline: String,
    pub tolerance_profile: String,
    pub target: Option<SubsetTarget>,
    pub mutation_manifest_digest: String,
    pub feature_hash: String,
    pub level: String,
    pub role: String,
    pub implementation: String,
    pub platform: String,
    pub work_dir: String,
    pub output_dir: String,
    /// 📦️ Where this host writes its produced artifact bundle — separate from `work_dir`, so a
    /// mutable scratch copy is never mistaken for a result.
    pub artifact_dir: String,
    pub results_path: String,
    pub subject_raw_inputs: Vec<(String, String)>,
    pub fixtures: Vec<Fixture>,
    pub scenarios: Vec<Scenario>,
}

impl Plan {
    /// 📋️ Reads a coordinator-written plan file.
    pub fn from_json(value: &Json) -> Plan {
        let fixtures = value.array("fixtures").iter().map(|entry| Fixture { uri: entry.str("uri"), scope: entry.str("scope"), name: entry.str("name"), path: entry.str("path"), digest: entry.str("digest") }).collect();
        let scenarios = value
            .array("scenarios")
            .iter()
            .map(|entry| Scenario {
                id: entry.str("id"),
                name: entry.str("name"),
                level: entry.str("level"),
                mode: entry.str("mode"),
                seed: entry.str("seed"),
                steps: entry.array("steps").iter().map(|step| (step.str("keyword"), step.str("text"))).collect(),
                doc_strings: entry
                    .array("steps")
                    .iter()
                    .filter_map(|step| match step.get("docString") {
                        Some(Json::String(text)) => Some(text.clone()),
                        _ => None,
                    })
                    .collect(),
                data_tables: entry
                    .array("steps")
                    .iter()
                    .filter_map(|step| match step.get("dataTable") {
                        Some(Json::Array(rows)) => Some(
                            rows.iter()
                                .map(|row| match row {
                                    Json::Array(cells) => cells
                                        .iter()
                                        .map(|cell| match cell {
                                            Json::String(text) => text.clone(),
                                            _ => String::new(),
                                        })
                                        .collect(),
                                    _ => Vec::new(),
                                })
                                .collect(),
                        ),
                        _ => None,
                    })
                    .collect(),
            })
            .collect();
        let target = value.get("target").and_then(|entry| match entry {
            Json::Object(_) => Some(SubsetTarget { artifact: entry.str("artifact"), standard: entry.str("standard"), subset: entry.str("subset") }),
            _ => None,
        });
        let subject_raw_inputs = match value.get("subjectRawInputs") {
            Some(Json::Object(entries)) => entries
                .iter()
                .filter_map(|(implementation, path)| match path {
                    Json::String(path) => Some((implementation.clone(), path.clone())),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        };
        Plan {
            schema_version: 2,
            baseline_sha: value.str("baselineSha"),
            owner: value.str("owner"),
            case: value.str("case"),
            capability: value.str("capability"),
            comparison: value.str("comparison"),
            comparison_pipeline: value.str("comparisonPipeline"),
            tolerance_profile: value.str("toleranceProfile"),
            target,
            mutation_manifest_digest: value.str("mutationManifestDigest"),
            feature_hash: value.str("featureHash"),
            level: value.str("level"),
            role: value.str("role"),
            implementation: value.str("implementation"),
            platform: value.str("platform"),
            work_dir: value.str("workDir"),
            output_dir: value.str("outputDir"),
            artifact_dir: value.str("artifactDir"),
            results_path: value.str("resultsPath"),
            subject_raw_inputs,
            fixtures,
            scenarios,
        }
    }

    /// 🧫️ Absolute path of a resolved fixture; an undeclared URI is an error, never a silent default.
    pub fn fixture(&self, uri: &str) -> Result<std::path::PathBuf, String> {
        let fixture = self.fixtures.iter().find(|entry| entry.uri == uri).ok_or_else(|| format!("fixture {} is not part of this plan — declare it in the feature file", uri))?;
        Ok(std::path::PathBuf::from(&fixture.path))
    }

    /// 📦️ Absolute path this host writes one named result artifact to, creating parent directories.
    pub fn artifact(&self, role: &str, filename: &str) -> Result<std::path::PathBuf, String> {
        let dir = std::path::PathBuf::from(if self.artifact_dir.is_empty() { self.work_dir.clone() } else { self.artifact_dir.clone() }).join(role);
        std::fs::create_dir_all(&dir).map_err(|error| format!("cannot create artifact directory {}: {error}", dir.display()))?;
        Ok(dir.join(filename))
    }
}
//#endregion 🔖️Plan

//#region 🔖️Outcome
/// 📦️ One file this handler produced, addressed by ROLE so no comparison stage ever names a path.
#[derive(Debug, Clone)]
pub struct ResultArtifact {
    pub role: String,
    pub path: String,
    pub media_type: String,
}

/// 🏭️ Proof a SUBJECT handler reached production dispatch. Its ABSENCE is how a vector-replay adapter
/// is detected — a replayed expectation and a computed one are otherwise indistinguishable on the wire.
#[derive(Debug, Clone)]
pub struct ProductionDispatch {
    pub operation: String,
    pub bridge_version: u32,
}

/// 🎯️ What one scenario handler produces: an artifact BUNDLE plus the projection the profile compares.
pub struct Outcome {
    pub raw: Option<Vec<u8>>,
    pub projection: Json,
    pub artifacts: Vec<ResultArtifact>,
    pub production_dispatch: Option<ProductionDispatch>,
    pub diagnostics: Vec<(String, String)>,
}

impl Outcome {
    /// 🎯️ Projection-only outcome, for behaviours with no serialized artifact.
    pub fn projection(projection: Json) -> Outcome {
        Outcome { raw: None, projection, artifacts: Vec::new(), production_dispatch: None, diagnostics: Vec::new() }
    }

    /// 🎯️ Outcome carrying both the produced bytes and their semantic projection.
    pub fn with_raw(raw: Vec<u8>, projection: Json) -> Outcome {
        Outcome { raw: Some(raw), projection, artifacts: Vec::new(), production_dispatch: None, diagnostics: Vec::new() }
    }

    /// 📦️ Adds one produced file to the bundle under its role.
    pub fn artifact(mut self, role: &str, path: &std::path::Path, media_type: &str) -> Outcome {
        self.artifacts.push(ResultArtifact { role: role.to_string(), path: path.display().to_string(), media_type: media_type.to_string() });
        self
    }

    /// 🏭️ Records that this outcome came out of PRODUCTION dispatch rather than a committed vector.
    pub fn dispatched(mut self, operation: &str, bridge_version: u32) -> Outcome {
        self.production_dispatch = Some(ProductionDispatch { operation: operation.to_string(), bridge_version });
        self
    }
}
//#endregion 🔖️Outcome
