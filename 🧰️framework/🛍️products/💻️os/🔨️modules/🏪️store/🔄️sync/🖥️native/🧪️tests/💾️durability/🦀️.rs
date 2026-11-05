use super::*;
#[test]
fn failed_folder_flush_restores_the_previous_record_or_reports_both_failures() {
    let root = std::path::PathBuf::from(std::env::var("SEMIO_TEST_ARTIFACT_DIR").expect("owned test output"));
    for row in include_str!("flush.csv").lines().skip(1).map(|line| line.split(',').collect::<Vec<_>>()) {
        let persistent = row[1] == "1";
        let folder = root.join(format!("flush-{}-{persistent}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        let storage = FolderEventLogStorage::new(folder.clone());
        let event = |pack: &[u8]| FolderEvent { kind: DOCUMENT_PUT_EVENT, updated_at_ms: 1, key: "a".into(), metadata: "schema".into(), primary: pack.into(), secondary: b"spr".to_vec() };
        storage.append(&event(b"old")).expect("baseline");
        let calls = std::cell::Cell::new(0);
        let outcome = storage.append_checked_with_sync(&event(b"new"), Some(Some((b"old", b"spr"))), None, |file| {
            let call = calls.get(); calls.set(call + 1);
            if call == 0 || persistent { Err(std::io::Error::other(if call == 0 { "original flush" } else { "rollback flush" })) } else { file.sync_data() }
        });
        let error = outcome.expect_err("failed flush cannot acknowledge").to_string();
        assert!(error.contains("original flush"));
        assert_eq!(error.contains("uncertain"), row[2] == "1");
        if persistent { assert!(error.contains("rollback flush")); }
        assert_eq!(storage.events().unwrap().last().unwrap().primary, row[3].as_bytes());
        std::fs::remove_dir_all(folder).unwrap();
    }
}
#[test]
fn newest_folder_archive_rebases_exactly_and_preserves_members() {
    semio_framework_async::block_on(async {
        let root = std::path::PathBuf::from(std::env::var("SEMIO_TEST_ARTIFACT_DIR").expect("owned output"));
        let folder = root.join(format!("archive-{}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        let storage = FolderEventLogStorage::new(folder.clone());
        storage.write("a", "schema", b"old", b"spr0").await.unwrap();
        let mut carrier = semio_framework_os_spr::DocumentArchivePack { parent_pack: b"new".to_vec(), parent_spr: b"spr1".to_vec(), members: vec![semio_framework_os_spr::OwnedDocumentMemberPackEntry { envelope_pack: b"child".to_vec(), ..Default::default() }] };
        let archive = semio_framework_os_spr::encode_document_archive_bytes(&carrier).unwrap();
        storage.write_archive("a", "schema", &archive).await.unwrap();
        assert_eq!(storage.read("a").await.unwrap(), Some((b"new".to_vec(), b"spr1".to_vec())));
        assert!(!storage.write_if_current("a", "schema", Some((b"old", b"spr0")), b"bad", b"bad").await.unwrap());
        assert!(storage.write_snapshot_if_current("a", "schema", (b"new", b"spr1"), None, b"bad", b"bad").await.unwrap().is_none());
        let published = storage.write_snapshot_if_current("a", "schema", (b"new", b"spr1"), Some(&archive), b"next", b"spr2").await.unwrap().unwrap().unwrap();
        let observed = semio_framework_os_spr::decode_document_archive_bytes(&published).await.unwrap();
        assert_eq!(observed.members, carrier.members);
        assert_eq!(observed.parent_pack, b"next");
        assert_eq!(storage.read_archive("a").await.unwrap(), Some(published.clone()));
        carrier.parent_pack = b"next".to_vec(); carrier.parent_spr = b"spr2".to_vec(); carrier.members[0].envelope_pack = b"peer child".to_vec();
        let peer = semio_framework_os_spr::encode_document_archive_bytes(&carrier).unwrap();
        storage.write_archive("a", "schema", &peer).await.unwrap();
        assert!(storage.write_snapshot_if_current("a", "schema", (b"next", b"spr2"), Some(&published), b"stale", b"stale").await.unwrap().is_none());
        assert_eq!(storage.read_archive("a").await.unwrap(), Some(peer));
        storage.write("a", "schema", b"pair", b"spr3").await.unwrap();
        assert_eq!(storage.read_archive("a").await.unwrap(), None);
        assert_eq!(storage.read("a").await.unwrap(), Some((b"pair".to_vec(), b"spr3".to_vec())));
        std::fs::remove_dir_all(folder).unwrap();
    });
}
#[test]
fn folder_pair_archive_order_matches_neutral_vectors() {
    semio_framework_async::block_on(async {
        let folder = std::path::PathBuf::from(std::env::var("SEMIO_TEST_ARTIFACT_DIR").unwrap()).join(format!("order-{}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        let storage = FolderEventLogStorage::new(folder.clone());
        for row in include_str!("archive.csv").lines().skip(1).map(|line| line.split(',').collect::<Vec<_>>()) {
            if row[1] == "pair" { storage.write("a", "schema", row[2].as_bytes(), row[3].as_bytes()).await.unwrap(); }
            else {
                let carrier = semio_framework_os_spr::DocumentArchivePack { parent_pack: row[2].as_bytes().to_vec(), parent_spr: row[3].as_bytes().to_vec(), members: vec![semio_framework_os_spr::OwnedDocumentMemberPackEntry { envelope_pack: row[4].as_bytes().to_vec(), ..Default::default() }] };
                storage.write_archive("a", "schema", &semio_framework_os_spr::encode_document_archive_bytes(&carrier).unwrap()).await.unwrap();
            }
            assert_eq!(storage.read("a").await.unwrap().unwrap().0, row[5].as_bytes());
            assert_eq!(storage.read_archive("a").await.unwrap().is_some(), row[6] == "1");
        }
        std::fs::remove_dir_all(folder).unwrap();
    });
}
