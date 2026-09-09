use super::*;

/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every one of those spellings must also appear in one
/// of the nine subset-scoped catalogs (`note-1-any` plus the eight mutation-owning subsets this
/// enum's 33 variants were split across in ticket
/// `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`).
/// The framework reads the catalogs and never the enum, so this is the only thing standing
/// between a renamed variant and a mutation catalog that silently measures a vocabulary the code
/// no longer has.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <NoteMutation as protocol::SemanticMutation<NoteSnapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared NoteMutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = concat!(
        include_str!("../../../../🔮️oracle/🔣️.json"),
        include_str!("../../../../../🖋️ink/🔮️oracle/🔣️.json"),
        include_str!("../../../../../📝️text/🔮️oracle/🔣️.json"),
        include_str!("../../../../../🧮️math/🔮️oracle/🔣️.json"),
        include_str!("../../../../../📊️table/🔮️oracle/🔣️.json"),
        include_str!("../../../../../🖼️asset/🔮️oracle/🔣️.json"),
        include_str!("../../../../../🧱️block/🔮️oracle/🔣️.json"),
        include_str!("../../../../../🎨️canvas/🔮️oracle/🔣️.json"),
        include_str!("../../../../../📜️document/🔮️oracle/🔣️.json"),
    );
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in one of the committed subset oracle manifests' catalogs");
    }
}
