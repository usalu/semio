use super::*;

#[test]
fn paged_text_extent_preserves_neutral_line_and_unicode_semantics() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in corpus["cases"].as_array().unwrap() {
        let text = row["content"].as_str().unwrap();
        let size = row["size"].as_f64().unwrap();
        let expected: [f64; 2] = serde_json::from_value(row["extent"].clone()).unwrap();
        let paged = semio_framework_value::paged::PagedUtf8::<{usize::MAX}>::try_from_str(text).unwrap();
        assert_eq!(drawing_text_fallback_extent(text, size), expected);
        assert_eq!(drawing_text_fallback_extent(&paged, size), expected);
    }
    for row in corpus["chunkCases"].as_array().unwrap() {
        let text = format!("{}{}", row["prefix"].as_str().unwrap().repeat(row["prefixRepeat"].as_u64().unwrap() as usize), row["suffix"].as_str().unwrap());
        let paged = semio_framework_value::paged::PagedUtf8::<{usize::MAX}>::try_from_str(&text).unwrap();
        assert_eq!(drawing_text_fallback_extent(&paged, row["size"].as_f64().unwrap()), serde_json::from_value::<[f64;2]>(row["extent"].clone()).unwrap());
    }
    eprintln!("[DEBUG] native paged text extents preserve independent line fixtures and CRLF chunk boundaries");
}
