use super::*;
#[semio_framework_async_macros::async_test]
async fn demo_source_nonempty() {
    assert!(!PRIMARY_TEXT.is_empty());
    let _ = source();
}

#[semio_framework_async_macros::async_test]
async fn native_demo_matches_the_authored_workbook() {
    use crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_xlsx;
    use calamine::{Data, Reader, Xlsx};
    use pack::snapshot::ArtifactDsl;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
    use std::io::Cursor;

    let bytes = include_bytes!("../../🖼️assets/📕️example.xlsx");
    let native = decode_xlsx(bytes).unwrap().project_workbook().unwrap();
    let authored = XlsxSnapshot::parse_dsl(PRIMARY_TEXT).unwrap().project_workbook().unwrap();
    assert_eq!(native, authored);

    let mut independent: Xlsx<_> = calamine::open_workbook_from_rs(Cursor::new(bytes.as_slice())).unwrap();
    assert_eq!(independent.sheet_names(), ["Sheet1", "Totals"]);
    let worksheet = independent.worksheet_range("Sheet1").unwrap();
    assert_eq!(worksheet.get_value((0, 0)), Some(&Data::String("Name".into())));
    assert_eq!(worksheet.get_value((2, 1)), Some(&Data::Float(95.5)));
    assert_eq!(independent.worksheet_formula("Sheet1").unwrap().get_value((2, 1)).map(String::as_str), Some("SUM(B2:B2)"));
}
