//! 🪪️ Both fixed revision heads are compared only after the host borrows the actual sealed command authority.
pub fn qualified_heads_current(expected_parent:&[u8;32],expected_child:&[u8;32],current_parent:&[u8;32],current_child:&[u8;32])->bool{expected_parent==current_parent&&expected_child==current_child}
#[cfg(test)]mod tests{
 fn revision(s:&str)->[u8;32]{std::array::from_fn(|i|u8::from_str_radix(&s[i*2..i*2+2],16).unwrap())}
 #[test]fn original_qualified_revision_heads_match_neutral_cases(){let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();for row in fixture["cases"].as_array().unwrap(){let value=|key:&str|revision(row[key].as_str().unwrap());assert_eq!(super::qualified_heads_current(&value("expectedParent"),&value("expectedChild"),&value("currentParent"),&value("currentChild")),row["accepted"].as_bool().unwrap());}}
}
