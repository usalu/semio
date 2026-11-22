use semio_os_channel_archive_ingress::{folder_archive_load::*, DocumentArchiveIngressSeal};

fn hex(text: &str) -> Vec<u8> {
    (0..text.len() / 2).map(|at| u8::from_str_radix(&text[2 * at..2 * at + 2], 16).unwrap()).collect()
}

fn key(row: &serde_json::Value) -> FolderArchiveLoadKey {
    let field = |name: &str| row["key"][name].as_u64().unwrap();
    FolderArchiveLoadKey { instance: field("instance") as u32, activation: field("activation"), lifetime: field("lifetime"), sequence: field("sequence") }
}

#[test]
fn folder_archive_load_wire_matches_the_language_neutral_law() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📬️folder-load-wire/🔣️.json")).unwrap();
    for row in law["requests"].as_array().unwrap() {
        let name = row["name"].as_str().unwrap();
        let wire = hex(row["wireHex"].as_str().unwrap());
        let bytes = row.get("bytes").map(|value| hex(value.as_str().unwrap())).unwrap_or_default();
        let (document, schema) = (row["document"].as_str().unwrap_or(""), row["schema"].as_str().unwrap_or(""));
        let action = match row["action"].as_str().unwrap() {
            "begin" => FolderArchiveLoadAction::Begin { seal: DocumentArchiveIngressSeal { bytes: row["extent"].as_u64().unwrap() as usize, sha256: hex(row["sha256"].as_str().unwrap()).try_into().unwrap() }, document, schema },
            "page" => FolderArchiveLoadAction::Page { offset: row["offset"].as_u64().unwrap() as u32, bytes: &bytes },
            "finish" => FolderArchiveLoadAction::Finish,
            "publish" => FolderArchiveLoadAction::Publish,
            _ => FolderArchiveLoadAction::Cancel,
        };
        let request = FolderArchiveLoadRequest { key: key(row), action };
        assert!(FolderArchiveLoadRequest::is_wire(&wire), "{name}");
        assert_eq!(request.encode().expect(name), wire, "{name}");
        assert_eq!(FolderArchiveLoadRequest::decode(&wire).expect(name), request, "{name}");
        let mut truncated = wire.clone();
        truncated.truncate(wire.len() - 1);
        assert!(FolderArchiveLoadRequest::decode(&truncated).is_none() || matches!(action, FolderArchiveLoadAction::Page { .. } | FolderArchiveLoadAction::Begin { .. }), "{name}");
    }
    for row in law["replies"].as_array().unwrap() {
        let name = row["reply"].as_str().unwrap();
        let wire = hex(row["wireHex"].as_str().unwrap());
        let padding = vec![0u8; row["length"].as_u64().unwrap() as usize];
        let action = match row["action"].as_str().unwrap() {
            "begin" => FolderArchiveLoadAction::Begin { seal: DocumentArchiveIngressSeal { bytes: 6, sha256: [0; 32] }, document: "d", schema: "s" },
            "page" => FolderArchiveLoadAction::Page { offset: row["offset"].as_u64().unwrap() as u32, bytes: &padding },
            "finish" => FolderArchiveLoadAction::Finish,
            "publish" => FolderArchiveLoadAction::Publish,
            _ => FolderArchiveLoadAction::Cancel,
        };
        let request = FolderArchiveLoadRequest { key: key(row), action };
        let reply = match name {
            "pending" => FolderArchiveLoadReply::Pending,
            "admitted" => FolderArchiveLoadReply::Admitted { activation: row["activation"].as_u64().unwrap(), lifetime: row["lifetime"].as_u64().unwrap() },
            "received" => FolderArchiveLoadReply::Received { through: row["through"].as_u64().unwrap() as u32 },
            "verified" => FolderArchiveLoadReply::Verified,
            "submitted" => FolderArchiveLoadReply::Submitted,
            "closed" => FolderArchiveLoadReply::Closed,
            _ => FolderArchiveLoadReply::Refused,
        };
        assert!(FolderArchiveLoadReply::is_wire(&wire), "{name}");
        let mut encoded = Vec::with_capacity(FolderArchiveLoadReply::MAXIMUM_BYTES);
        assert!(reply.encode_into(request, &mut encoded), "{name}");
        assert_eq!(encoded, wire, "{name}");
        assert_eq!(FolderArchiveLoadReply::decode(request, &wire), Some(reply), "{name}");
    }
}
