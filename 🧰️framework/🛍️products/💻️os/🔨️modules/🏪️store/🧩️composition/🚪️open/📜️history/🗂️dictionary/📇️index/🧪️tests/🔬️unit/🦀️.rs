
use super::*;

fn retire(index: &mut RetainedDictionaryIndex, grant: usize) -> usize {
    let mut bytes = 0;
    let mut pages = 0;
    let expected = index.allocated_pages();
    for _ in 0..4096 {
        match index.close_step(1, grant) {
            DictionaryIndexClose::Complete => {
                assert_eq!(pages, expected);
                assert!(index.terminal_is_empty());
                return bytes;
            }
            DictionaryIndexClose::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= grant);
                pages += released_items;
                bytes += released_bytes;
                assert_eq!(index.lookup(0), Err(DictionaryIndexError::State));
            }
        }
    }
    panic!("bounded range retirement failed to converge");
}

#[test]
fn retained_dictionary_range_index_never_publishes_partial_delta_and_retires_two_pages() {
    assert_eq!(size_of::<[DictionaryRange; PAGE_ENTRIES]>(), PAGE_BYTES);
    for grant in [1, 7, 4096] {
        let mut index = RetainedDictionaryIndex::new(10_000, 8192, 1048576).unwrap();
        index.begin_delta(0, 65).unwrap();
        for entry in 0..65 {
            index.append(DictionaryRange { offset: 32 + entry, length: 1 }).unwrap();
            assert_eq!(index.visible_entries(), 0);
            assert_eq!(index.lookup(entry as usize), Err(DictionaryIndexError::Malformed));
        }
        assert_eq!(index.allocated_pages(), 2);
        index.publish_delta().unwrap();
        assert_eq!(index.visible_entries(), 65);
        assert_eq!(index.lookup(64), Ok(DictionaryRange { offset: 96, length: 1 }));
        assert_eq!(index.dictionary_bytes(), 65);
        index.begin_delta(65, 2).unwrap();
        index.append(DictionaryRange { offset: 100, length: 1 }).unwrap();
        index.reject_record();
        assert_eq!(index.visible_entries(), 65);
        assert_eq!(index.lookup(0), Err(DictionaryIndexError::Malformed));
        assert_eq!(retire(&mut index, grant), 2048);
    }
}

#[test]
fn retained_dictionary_range_index_caps_all_deltas_and_retains_late_rejections() {
    for grant in [1, 7, 4096] {
        let mut index = RetainedDictionaryIndex::new(100, 3, 4).unwrap();
        index.begin_delta(0, 1).unwrap();
        index.append(DictionaryRange { offset: 32, length: 3 }).unwrap();
        index.publish_delta().unwrap();
        index.begin_delta(1, 2).unwrap();
        index.append(DictionaryRange { offset: 35, length: 1 }).unwrap();
        assert_eq!(index.append(DictionaryRange { offset: 36, length: 1 }), Err(DictionaryIndexError::Capacity));
        assert_eq!(index.visible_entries(), 1);
        assert_eq!(index.dictionary_bytes(), 4);
        assert_eq!(retire(&mut index, grant), 1024);
        for range in [DictionaryRange { offset: 99, length: 2 }, DictionaryRange { offset: u64::MAX, length: 1 }] {
            let mut index = RetainedDictionaryIndex::new(100, 1, 1048576).unwrap();
            index.begin_delta(0, 1).unwrap();
            assert_eq!(index.append(range), Err(DictionaryIndexError::Malformed));
            assert_eq!(retire(&mut index, grant), 0);
        }
        let mut index = RetainedDictionaryIndex::new(100, 1, 1048576).unwrap();
        assert_eq!(index.begin_delta(1, 1), Err(DictionaryIndexError::Malformed));
        assert_eq!(retire(&mut index, grant), 0);
    }
}
