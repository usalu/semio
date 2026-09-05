use std::collections::HashSet;
use std::sync::OnceLock;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MeshAsset {
    pub url: String,
    pub source: String,
    pub path: String,
}

fn object<'a>(value: &'a serde_json::Value, keys: &[&str]) -> Result<&'a serde_json::Map<String, serde_json::Value>, String> {
    let row = value.as_object().ok_or("Mesh catalog object required")?;
    if row.len() != keys.len() || keys.iter().any(|key| !row.contains_key(*key)) {
        return Err("Mesh catalog fields do not match the schema".into());
    }
    Ok(row)
}

fn path(value: &serde_json::Value, extension: &str) -> Result<String, String> {
    let value = value.as_str().ok_or("Mesh catalog path required")?;
    let stem = value.strip_suffix(extension).ok_or("Mesh catalog path extension required")?;
    if stem.is_empty() || stem.split('/').any(|part| part.is_empty() || part.chars().any(|ch| matches!(ch, '.' | '\\' | '%' | '?' | '#') || ch.is_control())) {
        return Err(format!("Unsafe mesh catalog path: {value}"));
    }
    Ok(value.into())
}

fn public_url(value: &serde_json::Value) -> Result<String, String> {
    let url = value.as_str().ok_or("Mesh public URL required")?;
    let leaf = url.strip_prefix("/mesh/").ok_or("Mesh public URL required")?;
    path(&serde_json::Value::String(leaf.into()), ".glb")?;
    if leaf.contains('/') {
        return Err("Mesh public identity must be a single explicit URL key".into());
    }
    Ok(url.into())
}

fn rows(value: &serde_json::Value) -> Result<&Vec<serde_json::Value>, String> {
    value.as_array().ok_or_else(|| "Mesh catalog rows required".into())
}

/// 🧭️ Admits the language-neutral source and delivery authority without inferring filesystem names.
pub fn parse_mesh_delivery_catalog(input: &str, read_catalog: impl Fn(&str) -> Result<String, String>) -> Result<Vec<MeshAsset>, String> {
    let input: serde_json::Value = serde_json::from_str(input).map_err(|error| error.to_string())?;
    let authority = object(&input, &["$schema", "version", "collections", "entries"])?;
    if authority["version"].as_u64() != Some(1) || !authority["$schema"].is_string() {
        return Err("Unsupported mesh delivery schema".into());
    }
    let mut result = Vec::new();
    let mut urls = HashSet::new();
    let mut sources = HashSet::new();
    let mut paths = HashSet::new();
    let mut catalogs = HashSet::new();
    let mut admit = |entry: MeshAsset| -> Result<(), String> {
        if !urls.insert(entry.url.clone()) || !sources.insert(entry.source.clone()) || !paths.insert(entry.path.clone()) {
            return Err(format!("Duplicate mesh identity: {}", entry.url));
        }
        result.push(entry);
        Ok(())
    };
    for value in rows(&authority["collections"])? {
        let collection = object(value, &["catalog", "root", "output"])?;
        let catalog_path = path(&collection["catalog"], ".json")?;
        if !catalogs.insert(catalog_path.clone()) {
            return Err(format!("Duplicate mesh source catalog: {catalog_path}"));
        }
        let root = path(&collection["root"], "")?;
        let output = path(&collection["output"], "")?;
        let source: serde_json::Value = serde_json::from_str(&read_catalog(&catalog_path)?).map_err(|error| error.to_string())?;
        let source = object(&source, &["$schema", "version", "entries"])?;
        if source["version"].as_u64() != Some(1) || !source["$schema"].is_string() || rows(&source["entries"])?.is_empty() {
            return Err("Unsupported mesh source schema".into());
        }
        for value in rows(&source["entries"])? {
            let entry = object(value, &["url", "path"])?;
            let leaf = path(&entry["path"], ".glb")?;
            admit(MeshAsset { url: public_url(&entry["url"])?, source: format!("{root}/{leaf}"), path: format!("{output}/{leaf}") })?;
        }
    }
    for value in rows(&authority["entries"])? {
        let entry = object(value, &["url", "source", "path"])?;
        admit(MeshAsset { url: public_url(&entry["url"])?, source: path(&entry["source"], ".glb")?, path: path(&entry["path"], ".glb")? })?;
    }
    Ok(result)
}

static CATALOG: OnceLock<Result<Vec<MeshAsset>, String>> = OnceLock::new();

/// 🔎️ Current public mesh IDs resolve through the sole explicit catalog; malformed IDs remain errors.
pub fn resolve_mesh_asset(url: &str) -> Result<&'static MeshAsset, String> {
    let catalog = CATALOG.get_or_init(|| parse_mesh_delivery_catalog(include_str!("📇️catalog.json"), |path| match path {
        "🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/📇️catalog.json" => Ok(include_str!("../🌱️metabolism/🎨️representation/📇️catalog.json").into()),
        _ => Err(format!("Unknown mesh source catalog: {path}")),
    }));
    catalog.as_ref().map_err(Clone::clone)?.iter().find(|entry| entry.url == url).ok_or_else(|| format!("Unknown mesh asset: {url}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neutral_mesh_catalog_agrees_with_independent_serde_projection() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🔣️.json")).unwrap();
        let catalog = parse_mesh_delivery_catalog(&fixture["delivery"].to_string(), |path| Ok(fixture["catalogs"][path].to_string())).unwrap();
        let actual: Vec<_> = catalog.iter().map(|entry| serde_json::json!({ "url": entry.url, "source": entry.source, "path": entry.path })).collect();
        assert_eq!(serde_json::Value::Array(actual), fixture["expected"]);
        for url in fixture["unknown"].as_array().unwrap() {
            assert!(!catalog.iter().any(|entry| Some(entry.url.as_str()) == url.as_str()));
        }
        assert_eq!(resolve_mesh_asset("/mesh/🧊️capsule_J.glb").unwrap().path, "🌱️metabolism/💊️capsules/🪝️j/🧊️capsule_J.glb");
        assert!(resolve_mesh_asset("/mesh/🧊️ellipsoid-🧊️capsule_J.glb").is_err());
    }

    #[test]
    fn hostile_mesh_catalog_is_rejected_without_alias_or_path_fallback() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🔣️.json")).unwrap();
        for key in ["url", "source", "path"] {
            let mut input = fixture["delivery"].clone();
            let mut extra = serde_json::json!({ "url": "/mesh/🛖️hut.glb", "source": "🛖️hut/🧊️shape.glb", "path": "🛖️hut/🧊️shape.glb" });
            extra[key] = input["entries"][0][key].clone();
            input["entries"].as_array_mut().unwrap().push(extra);
            assert!(parse_mesh_delivery_catalog(&input.to_string(), |path| Ok(fixture["catalogs"][path].to_string())).is_err());
        }
        for path in ["../🧊️shape.glb", "/🧊️shape.glb", "🏠️house//🧊️shape.glb", "🏠️house/%2e%2e/🧊️shape.glb", "🏠️house\\🧊️shape.glb", "🏠️house/./🧊️shape.glb"] {
            let mut input = fixture["delivery"].clone();
            input["entries"][0]["path"] = path.into();
            assert!(parse_mesh_delivery_catalog(&input.to_string(), |path| Ok(fixture["catalogs"][path].to_string())).is_err());
        }
        let mut input = fixture["delivery"].clone();
        input["entries"][0]["alias"] = "/mesh/old.glb".into();
        assert!(parse_mesh_delivery_catalog(&input.to_string(), |path| Ok(fixture["catalogs"][path].to_string())).is_err());
        assert!(parse_mesh_delivery_catalog(&fixture["delivery"].to_string(), |_| Err("Unknown source".into())).is_err());
    }
}
