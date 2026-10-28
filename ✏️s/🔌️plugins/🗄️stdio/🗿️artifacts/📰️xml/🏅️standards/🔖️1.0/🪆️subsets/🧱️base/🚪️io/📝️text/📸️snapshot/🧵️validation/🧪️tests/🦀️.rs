use super::*;

fn finish(cursor: &mut XmlValidationCursor, source: &[u8], grant: usize) -> XmlValidationStep {
    let mut turns=0;
    loop {
        let step=cursor.advance(source,grant);
        assert!(step.work<=grant);
        turns+=1;
        assert!(turns<source.len().saturating_mul(1100)+2000);
        if step.done || step.error.is_some() { return step; }
    }
}

#[test]
fn portable_xml_validation_contract() {
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧪️fixtures/🔣️.json")).unwrap();
    for row in corpus["cases"].as_array().unwrap() {
        let source=if let Some(bytes)=row["bytes"].as_array() { bytes.iter().map(|byte|byte.as_u64().unwrap() as u8).collect::<Vec<_>>() } else { row["xml"].as_str().unwrap().as_bytes().to_vec() };
        for grant in [1,7,64,4096] {
            let mut cursor=XmlValidationCursor::new();
            let step=finish(&mut cursor,&source,grant);
            assert_eq!(step.error.is_none(),row["valid"].as_bool().unwrap(),"{} {:?}",row["id"],step.error);
            if let Some(error)=row["error"].as_str() { assert_eq!(step.error,Some(error),"{}",row["id"]); }
            if let Some(root)=row["root"].as_str() { let span=cursor.root_name().unwrap(); assert_eq!(&source[span.start..span.end],root.as_bytes()); }
            cursor.close(); assert!(cursor.terminal_is_empty());
        }
    }
}

#[test]
fn bounded_xml_limits_comparisons_and_cancellation() {
    let long=format!("<root>{}</root>","text 😀 &amp; ".repeat(10000));
    for grant in [1,7,64,4096] { let mut cursor=XmlValidationCursor::new(); assert!(finish(&mut cursor,long.as_bytes(),grant).done); cursor.close(); assert!(cursor.terminal_is_empty()); }
    for cut in [0,1,20,50] {
        let mut cursor=XmlValidationCursor::new(); for _ in 0..cut { cursor.advance(long.as_bytes(),7); }
        cursor.close(); assert!(cursor.terminal_is_empty()); assert_eq!(cursor.advance(long.as_bytes(),7).error,Some("xml.validation.closed"));
    }
    for depth in [256,257] {
        let source=format!("{}{}","<a>".repeat(depth),"</a>".repeat(depth)); let mut cursor=XmlValidationCursor::new();
        assert_eq!(finish(&mut cursor,source.as_bytes(),7).error.is_none(),depth==256); cursor.close();
    }
    for count in [1024,1025] {
        let attributes=(0..count).map(|i|format!("a{i}=\"\"")).collect::<Vec<_>>().join(" ");
        let source=format!("<a {attributes}/>"); let mut cursor=XmlValidationCursor::new();
        assert_eq!(finish(&mut cursor,source.as_bytes(),4096).error.is_none(),count==1024); cursor.close();
    }
    let name="a".repeat(10000); let source=format!("<{name} {name}=\"\" {name}=\"\"/>"); let mut cursor=XmlValidationCursor::new();
    assert_eq!(finish(&mut cursor,source.as_bytes(),7).error,Some("xml.validation.duplicate-attribute")); cursor.close();
    let first=b"<a/>".to_vec(); let second=first.clone(); let mut cursor=XmlValidationCursor::new();
    assert_eq!(cursor.advance(&first,0).work,0); assert_eq!(cursor.advance(&second,1).error,Some("xml.validation.source")); cursor.close();
}

#[test]
fn root_attribute_source_spans_are_exact() {
    let source="<Document DOMVersion=\"8&#46;0\" note=\"Grüße 😀\"/>".as_bytes();
    let mut cursor=XmlValidationCursor::new(); let mut attributes=Vec::new();
    loop { let step=cursor.advance(source,1); assert!(step.error.is_none()); if let Some(XmlValidationEvent::Attribute { depth,name,value })=step.event { assert_eq!(depth,1); attributes.push((&source[name.start..name.end],&source[value.start..value.end])); } if step.done { break; } }
    assert_eq!(attributes,vec![(b"DOMVersion".as_slice(),b"8&#46;0".as_slice()),(b"note".as_slice(),"Grüße 😀".as_bytes())]); cursor.close();
}
