use std::collections::HashSet;

/// 🧭️ Authored source encoding admitted by an explicitly selected codec.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpaceFixtureFormat {
    Json,
    Dsl,
}

impl std::str::FromStr for SpaceFixtureFormat {
    type Err = String;

    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format {
            "json" => Ok(Self::Json),
            "dsl" => Ok(Self::Dsl),
            _ => Err(format!("unsupported fixture source format {format}")),
        }
    }
}

/// 🧫️ Opaque fixture identity and its authored source input.
pub struct SpaceFixtureSource {
    pub slug: String,
    pub format: SpaceFixtureFormat,
    pub codec: String,
    pub text: String,
}

/// 🔐️ Owned boundary around source decoding into a snapshot document.
pub struct SpaceFixtureCodec {
    pub id: &'static str,
    pub decode: fn(SpaceFixtureFormat, &str) -> Result<String, String>,
}

/// 🧪️ Prepares every source before any registry mutation can occur.
pub fn prepare_space_fixture_sources(sources: &[SpaceFixtureSource], codecs: &[SpaceFixtureCodec]) -> Result<Vec<(String, String)>, String> {
    if sources.is_empty() {
        return Err("fixture sources are empty".into());
    }
    let mut codec_ids = HashSet::new();
    for codec in codecs {
        if codec.id.trim().is_empty() || !codec_ids.insert(codec.id) {
            return Err("fixture codec identity is empty or repeated".into());
        }
    }
    let mut slugs = HashSet::new();
    let mut documents = Vec::with_capacity(sources.len());
    for source in sources {
        if source.slug.trim().is_empty() || !slugs.insert(source.slug.as_str()) {
            return Err("fixture slug is empty or repeated".into());
        }
        if source.text.trim().is_empty() {
            return Err("fixture source text is empty".into());
        }
        let codec = codecs.iter().find(|codec| codec.id == source.codec).ok_or_else(|| "fixture codec is missing".to_string())?;
        let document = (codec.decode)(source.format, &source.text)?;
        let value: store::DslValue = semio_framework_pack_json::from_json_str(&document, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("fixture document is invalid: {error}"))?;
        if !matches!(value, store::DslValue::Object(ref entries) if !entries.is_empty()) {
            return Err("fixture document is empty or is not an object".into());
        }
        documents.push((source.slug.clone(), document));
    }
    Ok(documents)
}

/// 📥️ Atomically admits the complete caller-selected source set.
pub fn register_space_fixture_sources(sources: &[SpaceFixtureSource], codecs: &[SpaceFixtureCodec]) -> Result<(), String> {
    semio_framework_os::register_os_fixture_documents(prepare_space_fixture_sources(sources, codecs)?)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
