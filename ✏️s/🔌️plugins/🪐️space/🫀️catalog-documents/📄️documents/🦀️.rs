use std::collections::HashSet;

/// 🧭️ Authored source encoding admitted by an explicitly selected codec.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpaceDocumentFormat {
    Json,
    Dsl,
}

impl std::str::FromStr for SpaceDocumentFormat {
    type Err = String;

    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format {
            "json" => Ok(Self::Json),
            "dsl" => Ok(Self::Dsl),
            _ => Err(format!("unsupported document source format {format}")),
        }
    }
}

/// 📄️ Opaque document identity and its authored source input.
pub struct SpaceDocumentSource {
    pub slug: String,
    pub format: SpaceDocumentFormat,
    pub codec: String,
    pub text: String,
}

/// 🔐️ Owned boundary around source decoding into a snapshot document.
pub struct SpaceDocumentCodec {
    pub id: &'static str,
    pub decode: fn(SpaceDocumentFormat, &str) -> Result<String, String>,
}

/// 🧪️ Decodes a complete caller-owned batch without changing runtime state.
pub fn prepare_space_document_sources(sources: &[SpaceDocumentSource], codecs: &[SpaceDocumentCodec]) -> Result<Vec<(String, String)>, String> {
    if sources.is_empty() {
        return Err("document sources are empty".into());
    }
    let mut codec_ids = HashSet::new();
    for codec in codecs {
        if codec.id.trim().is_empty() || !codec_ids.insert(codec.id) {
            return Err("document codec identity is empty or repeated".into());
        }
    }
    let mut slugs = HashSet::new();
    let mut documents = Vec::with_capacity(sources.len());
    for source in sources {
        if source.slug.trim().is_empty() || !slugs.insert(source.slug.as_str()) {
            return Err("document slug is empty or repeated".into());
        }
        if source.text.trim().is_empty() {
            return Err("document source text is empty".into());
        }
        let codec = codecs.iter().find(|codec| codec.id == source.codec).ok_or_else(|| "document codec is missing".to_string())?;
        let document = (codec.decode)(source.format, &source.text)?;
        let value: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(&document, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("document document is invalid: {error}"))?;
        if !matches!(value, semio_framework_value::DslValue::Object(ref entries) if !entries.is_empty()) {
            return Err("document document is empty or is not an object".into());
        }
        documents.push((source.slug.clone(), document));
    }
    Ok(documents)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
