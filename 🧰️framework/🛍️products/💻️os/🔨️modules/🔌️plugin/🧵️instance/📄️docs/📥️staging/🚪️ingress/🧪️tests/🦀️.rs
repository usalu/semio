//! 🪢️ Head-reachability law of the genesis member roster, run over the language-agnostic corpus `head-reachability.json`; the TypeScript twin `🟦️.ts` validates the same corpus against `graphlib`.
use super::retain_head_reachable_members;
use protocol::{DocumentArchiveArtifactRef, DocumentArchiveOwnerRef, OwnedDocumentMemberPackEntry};

fn reference(id: &str) -> DocumentArchiveArtifactRef {
    DocumentArchiveArtifactRef { artifact_id: id.into(), artifact_kind: "kind".into(), standard: "std".into(), subset: "sub".into() }
}

#[test]
fn roster_keeps_exactly_the_head_reachable_members() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("head-reachability.json")).expect("corpus");
    for case in corpus["cases"].as_array().expect("cases") {
        let text = |value: &serde_json::Value, key: &str| value[key].as_str().expect(key).to_owned();
        let root = text(case, "root");
        let head: Vec<(String, String, String)> = case["head"].as_array().unwrap().iter().map(|row| (text(row, "slot"), text(row, "childId"), text(row, "artifactId"))).collect();
        let projected: Vec<(&str, &str, &str)> = head.iter().map(|(slot, child, artifact)| (slot.as_str(), child.as_str(), artifact.as_str())).collect();
        let mut members: Vec<OwnedDocumentMemberPackEntry> = case["members"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(index, row)| OwnedDocumentMemberPackEntry {
                ordinal: index as u32,
                reference: reference(&text(row, "artifactId")),
                owner: DocumentArchiveOwnerRef { parent: reference(&text(row, "parent")), slot: text(row, "slot"), child_id: text(row, "childId") },
                envelope_pack: Vec::new(),
            })
            .collect();
        retain_head_reachable_members(&mut members, &root, &projected);
        let kept: Vec<&str> = members.iter().map(|entry| entry.reference.artifact_id.as_str()).collect();
        let expected: Vec<String> = case["survivors"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_owned()).collect();
        assert_eq!(kept, expected, "{}", text(case, "name"));
    }
}
