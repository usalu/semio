//#region 🧪️NeutralPageLaws
use crate::byte_page::{ActorBytePage, ACTOR_BYTE_PAGE_BYTES};

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️fixture.json")).unwrap() }

#[test]
fn actor_byte_page_matches_shared_boundaries_and_little_endian_words() {
    let fixture = fixture();
    assert_eq!(ACTOR_BYTE_PAGE_BYTES, fixture["maximumBytes"].as_u64().unwrap() as usize);
    for row in fixture["vectors"].as_array().unwrap() {
        let len = row["length"].as_u64().unwrap() as usize;
        let source: Vec<_> = (0..len).map(|index| ((index * 37 + 11) % 256) as u8).collect();
        let page = ActorBytePage::try_copy_from(&source).unwrap();
        assert_eq!(page.as_slice(), source);
        assert_eq!(page.len(), len);
        assert_eq!(page.is_empty(), len == 0);
        let mut words = Vec::new();
        for block in 0..64 {
            for (index, word) in page.block(block).unwrap().words().iter().enumerate() {
                let offset = block * 64 + index * 8;
                let oracle = std::array::from_fn(|byte| source.get(offset + byte).copied().unwrap_or(0));
                assert_eq!(*word, u64::from_le_bytes(oracle));
                words.push(*word);
            }
        }
        assert_eq!(words[0].to_string(), row["firstWord"].as_str().unwrap());
        assert_eq!(words[len.saturating_sub(1) / 8].to_string(), row["lastUsedWord"].as_str().unwrap());
        assert!(page.block(64).is_none());
        assert!(page.block(usize::MAX).is_none());
        assert!(page.storage()[len..].iter().all(|byte| *byte == 0));
    }
}

#[test]
fn actor_byte_page_rejects_noncanonical_tail_and_oversize_before_use() {
    for row in fixture()["padding"].as_array().unwrap() {
        let mut bytes = [0; ACTOR_BYTE_PAGE_BYTES];
        bytes[row["byteOffset"].as_u64().unwrap() as usize] = row["value"].as_u64().unwrap() as u8;
        assert_eq!(ActorBytePage::try_from_array(bytes, row["length"].as_u64().unwrap() as u32).is_ok(), row["accepted"].as_bool().unwrap());
    }
    assert!(ActorBytePage::try_from_array([0; ACTOR_BYTE_PAGE_BYTES], 4097).is_err());
    assert!(ActorBytePage::try_from_array([0; ACTOR_BYTE_PAGE_BYTES], u32::MAX).is_err());
    assert!(ActorBytePage::try_copy_from(&[0; ACTOR_BYTE_PAGE_BYTES + 1]).is_err());
}

#[test]
fn actor_byte_page_has_fixed_backing_and_copies_only_the_selected_input() {
    assert!(!std::mem::needs_drop::<ActorBytePage>());
    assert_eq!(std::mem::size_of::<ActorBytePage>(), ACTOR_BYTE_PAGE_BYTES + 2);
    let mut bytes = [91; ACTOR_BYTE_PAGE_BYTES + 16];
    let page = ActorBytePage::try_copy_from(&bytes[8..12]).unwrap();
    bytes.fill(0);
    assert_eq!(page.as_slice(), [91; 4]);
    assert!(page.storage()[4..].iter().all(|byte| *byte == 0));
}
//#endregion 🧪️NeutralPageLaws
