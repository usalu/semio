//! 🧵️ UTF8-preserving transport through existing bounded UI value pages.
use super::*;

impl UiValue {
    /// 🪡️ Validates a finite wire sequence before reconstructing its exact UTF8 source.
    pub fn join_text_chunks<'a>(chunks: impl IntoIterator<Item = &'a str>, maximum_bytes: usize) -> Result<String, &'static str> {
        if maximum_bytes > UI_TEXT_MAX_BYTES * UI_VALUE_MAX_ITEMS { return Err("UI text source byte limit exceeded"); }
        let mut source = String::new();
        let mut count = 0usize;
        for chunk in chunks {
            count += 1;
            if count > UI_VALUE_MAX_ITEMS || chunk.len() > UI_TEXT_MAX_BYTES || source.len().checked_add(chunk.len()).is_none_or(|bytes| bytes > maximum_bytes) { return Err("UI text chunk byte limit exceeded"); }
            source.push_str(chunk);
        }
        if count == 0 { return Err("UI text chunks cannot be empty"); }
        Ok(source)
    }

    /// 📦️ Carries text as bounded scalar pages without enlarging UI text or collection capacities.
    pub fn try_text_chunks(value: &str, maximum_bytes: usize) -> Result<Self, &'static str> {
        if value.len() > maximum_bytes || maximum_bytes > UI_TEXT_MAX_BYTES * UI_VALUE_MAX_ITEMS { return Err("UI text source byte limit exceeded"); }
        let mut chunks = UiListBuilder::try_new().ok_or("UI text chunk collection admission failed")?;
        let mut start = 0usize;
        loop {
            let mut end = start.saturating_add(UI_TEXT_MAX_BYTES).min(value.len());
            while !value.is_char_boundary(end) { end -= 1; }
            let text = UiText::try_from_str(&value[start..end]).ok_or("UI text chunk admission failed")?;
            chunks.push(UiValue::Text(text)).map_err(|_| "UI text chunk collection capacity exceeded")?;
            if end == value.len() { break; }
            start = end;
        }
        Ok(UiValue::List(chunks.finish()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn utf8_text_chunk_transport_matches_shared_vectors() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            let source = row["unit"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap() as usize);
            let value = UiValue::try_text_chunks(&source, row["maximumBytes"].as_u64().unwrap() as usize).unwrap();
            let independent = serde_json::to_value(&value).unwrap();
            let chunks: Vec<_> = independent.as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect();
            assert_eq!(chunks.concat(), source);
            assert_eq!(UiValue::join_text_chunks(chunks.iter().copied(), row["maximumBytes"].as_u64().unwrap() as usize).unwrap(), source);
            assert!(chunks.iter().all(|chunk| chunk.len() <= UI_TEXT_MAX_BYTES));
            if let Some(expected) = row.get("bytes") { assert_eq!(serde_json::json!(chunks.iter().map(|chunk| chunk.len()).collect::<Vec<_>>()), *expected); }
        }
        for row in fixture["refusals"].as_array().unwrap() {
            let source = row["unit"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap() as usize);
            assert!(UiValue::try_text_chunks(&source, row["maximumBytes"].as_u64().unwrap() as usize).is_err());
        }
    }
}
