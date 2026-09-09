use super::*;

#[test]
fn direct_descriptor_and_catalog_bijection() {
    let kinds: Vec<_> = <PdfMutation as protocol::SemanticMutation<PdfSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations");
    let catalog: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("../../🔣️oracle.json")).unwrap()).unwrap();
    assert_eq!(catalog["mutationCatalogs"][0]["kinds"], serde_json::json!(kinds));
    {
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("📥️insert-page").join("🔣️.json")).unwrap()).unwrap();
        assert_eq!(descriptor["semanticKind"], kinds[0]);
        assert!(source.join("📥️insert-page").join("🦀️.rs").is_file());
    }
    {
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("🗑️remove-page").join("🔣️.json")).unwrap()).unwrap();
        assert_eq!(descriptor["semanticKind"], kinds[1]);
        assert!(source.join("🗑️remove-page").join("🦀️.rs").is_file());
    }
    {
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("🔀️move-page").join("🔣️.json")).unwrap()).unwrap();
        assert_eq!(descriptor["semanticKind"], kinds[2]);
        assert!(source.join("🔀️move-page").join("🦀️.rs").is_file());
    }
    {
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("📐️resize-page").join("🔣️.json")).unwrap()).unwrap();
        assert_eq!(descriptor["semanticKind"], kinds[3]);
        assert!(source.join("📐️resize-page").join("🦀️.rs").is_file());
    }
    {
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("♻️replace-page-text").join("🔣️.json")).unwrap()).unwrap();
        assert_eq!(descriptor["semanticKind"], kinds[4]);
        assert!(source.join("♻️replace-page-text").join("🦀️.rs").is_file());
    }
}
