//! 🔮️ Structured oracle registrations share the mutation vector runner's runtime data ownership.
use semio_framework_value::DslValue;
use std::collections::BTreeSet;
use std::path::Path;

/// 📋️ Missing native kinds are named only by actual registered vector entries, in caller declaration order.
pub fn mutation_catalogue_missing(kinds:&[&str],text:&str)->Result<Vec<String>,String>{
    let document=semio_framework_pack_json::parse(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|format!("mutation-catalogue.json: {error}"))?;
    let manifest=semio_framework_pack_json::to_dsl_value(&document);
    let catalogues=manifest.get("mutationCatalogs").and_then(DslValue::as_array).ok_or_else(||"mutation-catalogue.catalogs".to_string())?;
    let mut registered=BTreeSet::new();
    for catalogue in catalogues{
        let vectors=catalogue.get("vectors").and_then(DslValue::as_array).ok_or_else(||"mutation-catalogue.vectors".to_string())?;
        for vector in vectors{
            let kind=vector.get("mutationId").and_then(DslValue::as_str).filter(|kind|!kind.is_empty()).ok_or_else(||"mutation-catalogue.kind".to_string())?;
            registered.insert(kind);
        }
    }
    Ok(kinds.iter().copied().filter(|kind|!registered.contains(kind)).map(str::to_string).collect())
}
/// 🧾️ The native enum retains its catalogue proof while its unchanged manifest stays a runtime test input.
pub fn assert_mutation_catalogue(kinds:&[&str],source:&Path){
    let missing=mutation_catalogue_missing(kinds,&super::read(source)).unwrap_or_else(|error|panic!("catalogue {}: {error}",source.display()));
    assert!(missing.is_empty(),"native kinds missing from oracle catalogue {}: {missing:?}",source.display());
}
#[cfg(test)]
#[path="tests.rs"]
mod tests;
