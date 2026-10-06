use crate::standards::v6_0::subsets::document::io::sqlite::snapshot::tests::*;

#[test]
fn storage_word_widths_and_chunk_ordinals_survive_projection() {
    let snapshot = fixture();
    let database = project(&snapshot);
    let restored = restore(&database).expect("restore");
    assert_eq!(restored.ifds[0].storage.offsets_kind, TiffFieldType::Short);
    assert_eq!(restored.ifds[0].storage.byte_counts_kind, TiffFieldType::Long);
    assert_eq!(restored.ifds[0].storage.chunks, vec![vec![1, 2], vec![3, 4, 5]]);
}

#[test]
fn chunk_ordinals_must_be_contiguous() {
    let mut database = project(&fixture());
    database.table_mut("tiff_chunk").expect("chunk table").rows[1].values[2] = SqliteValue::Integer(4);
    assert!(restore(&database).is_err());
}
