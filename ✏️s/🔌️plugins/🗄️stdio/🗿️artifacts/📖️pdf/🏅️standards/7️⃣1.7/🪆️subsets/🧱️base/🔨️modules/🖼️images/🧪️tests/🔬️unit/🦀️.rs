use super::*;

#[test]
fn pdf_semantic_body_image_words_match_neutral_native_and_independent_lopdf() {
    use crate::standards::v1_7::subsets::base::{io::encode_pdf,schema::snapshot::{PdfSnapshot,PdfPage,PdfColorSpace}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){let integer=|key:&str|row[key].as_u64().unwrap() as u32;let values=row["values"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u32).collect::<Vec<_>>();let packed=row["packed"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect::<Vec<_>>();let (width,height,components,bits)=(integer("width"),integer("height"),integer("components"),integer("bits"));
        assert_eq!(pack_image_samples(width,height,components,bits,&values).unwrap(),packed);assert_eq!(unpack_image_samples(width,height,components,bits,&packed).unwrap(),values);
        let mut snapshot=PdfSnapshot::default();snapshot.pages.push(PdfPage::new(20.0,20.0));snapshot.images.push(PdfImage::samples("I",width,height,if components==3{PdfColorSpace::DeviceRgb}else{PdfColorSpace::DeviceGray},bits,values));let bytes=encode_pdf(&snapshot).unwrap();let reference=semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_image_sample_streams(&bytes).unwrap();assert_eq!(reference,vec![(bits,packed)]);
        println!("[DEBUG] PDF logical image body {} independent native image stream exact",row["name"].as_str().unwrap());
    }
    assert!(unpack_image_samples(3,2,1,1,&[160]).is_err());assert!(pack_image_samples(1,1,1,4,&[16]).is_err());
}

#[test]
fn pdf_semantic_body_foreign_artifact_custody_is_explicit_and_independent() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfSnapshot,PdfIndirectObject,ObjRef};
    use crate::standards::v1_7::subsets::base::io::foreign_artifacts::admit_pdf_artifact;
    let mut snapshot=PdfSnapshot::default();let object=PdfObject::Stream {dict:Vec::new(),data:vec![255,216,255,217],filters:vec![PdfStreamFilter::Dct {color_transform:None}]};let reference=admit_pdf_artifact(&mut snapshot,"s.stdio.jpeg",object.clone()).unwrap();assert_eq!(NativePdfArtifactResources::from_objects(&snapshot.objects).resolve(&reference).unwrap(),object);assert!(NativePdfArtifactResources::default().resolve(&reference).is_err());let bytes=snapshot.objects[0].value.clone();snapshot.objects.clear();snapshot.objects.push(PdfIndirectObject{id:ObjRef{num:99,gen:0},value:bytes});assert_eq!(NativePdfArtifactResources::from_objects(&snapshot.objects).resolve(&reference).unwrap(),object);println!("[DEBUG] PDF foreign image reference refuses unadmitted custody and survives independent native object identity");
}
