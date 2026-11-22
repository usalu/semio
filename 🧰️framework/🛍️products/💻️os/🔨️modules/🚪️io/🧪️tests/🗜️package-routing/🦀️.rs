//! 🗜️ Package content check and picked-file routing law driven by the language-agnostic `package-routing` fixture the TypeScript twin also drives; the `zip` crate is the third-party writer that builds every archive, stored and deflated.

use semio_os_io::{inspect_package, route_picked_file, FormatDescriptor, PackageClaim, PackageDialect, PackageSniffCancelled, PackageSource, PackageVerdict, PickedFileCatalog, PickedFileRefusal, PickedFileRoute, PACKAGE_TAIL_BYTES};
use serde_json::Value;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../📺️renderer/🧑‍🎨engine/🧪️tests/🗜️package-routing/🧫️fixtures/🔣️.json")).expect("package routing fixture")
}

fn claim(fixture: &Value) -> PackageClaim {
    let strings = |value: &Value| value.as_array().unwrap().iter().map(|item| item.as_str().unwrap().to_string()).collect::<Vec<_>>();
    PackageClaim {
        main_part_content_types: strings(&fixture["claim"]["mainPartContentTypes"]),
        dialects: fixture["claim"]["dialects"].as_array().unwrap().iter().map(|row| PackageDialect { artifact_ref: row["artifactRef"].as_str().unwrap().into(), namespace: row["namespace"].as_str().unwrap().into() }).collect(),
    }
}

struct Bytes {
    bytes: Vec<u8>,
    read: AtomicU64,
    cancelled: AtomicBool,
}

impl Bytes {
    fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, read: AtomicU64::new(0), cancelled: AtomicBool::new(false) }
    }
}

impl PackageSource for Bytes {
    fn size(&self) -> u64 {
        self.bytes.len() as u64
    }
    fn read(&self, start: u64, end: u64) -> Option<Vec<u8>> {
        self.read.fetch_add(end - start, Ordering::Relaxed);
        self.bytes.get(start as usize..(end as usize).min(self.bytes.len())).map(<[u8]>::to_vec)
    }
    fn cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}

fn archive(row: &Value, method: zip::CompressionMethod) -> Vec<u8> {
    if let Some(raw) = row["raw"].as_str() {
        return raw.chars().map(|character| character as u32 as u8).collect();
    }
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    for (name, text) in row["entries"].as_object().unwrap() {
        let mut text = text.as_str().unwrap().to_string();
        if row["padPart"]["path"].as_str() == Some(name) {
            text = format!("{text}<!-- {} -->", "x".repeat(row["padPart"]["bytes"].as_u64().unwrap() as usize));
        }
        writer.start_file(name.as_str(), zip::write::SimpleFileOptions::default().compression_method(method)).unwrap();
        writer.write_all(text.as_bytes()).unwrap();
    }
    let mut bytes = writer.finish().unwrap().into_inner();
    if let Some(cut) = row["cutTail"].as_u64() {
        bytes.truncate(bytes.len() - cut as usize);
    }
    bytes
}

fn verdict(row: &Value) -> PackageVerdict {
    match row["verdict"]["kind"].as_str().unwrap() {
        "match" => PackageVerdict::Match { artifact_ref: row["verdict"]["artifactRef"].as_str().map(str::to_string) },
        _ => PackageVerdict::Mismatch,
    }
}

#[test]
fn every_fixture_case_gets_its_verdict_from_stored_and_deflated_archives() {
    let fixture = fixture();
    let claim = claim(&fixture);
    for method in [zip::CompressionMethod::Stored, zip::CompressionMethod::Deflated] {
        for row in fixture["cases"].as_array().unwrap() {
            let id = row["id"].as_str().unwrap();
            assert_eq!(inspect_package(&Bytes::new(archive(row, method)), &claim), Ok(verdict(row)), "{id} ({method:?})");
        }
    }
}

#[test]
fn only_the_tail_the_directory_and_two_entries_are_read() {
    let fixture = fixture();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["padPart"].is_object()).unwrap();
    let source = Bytes::new(archive(row, zip::CompressionMethod::Stored));
    assert_eq!(inspect_package(&source, &claim(&fixture)), Ok(verdict(row)));
    assert!(source.bytes.len() > 200_000);
    assert!(source.read.load(Ordering::Relaxed) < PACKAGE_TAIL_BYTES + 8192 + 4096);
}

#[test]
fn content_types_beyond_the_bound_are_refused_and_cancellation_is_not_a_mismatch() {
    let fixture = fixture();
    let claim = claim(&fixture);
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    writer.start_file("[Content_Types].xml", options).unwrap();
    writer.write_all(format!("<Types><!-- {} --></Types>", "x".repeat(2 * 1024 * 1024 + 1)).as_bytes()).unwrap();
    writer.start_file("xl/workbook.xml", options).unwrap();
    writer.write_all(b"<workbook/>").unwrap();
    assert_eq!(inspect_package(&Bytes::new(writer.finish().unwrap().into_inner()), &claim), Ok(PackageVerdict::Mismatch));
    let cancelled = Bytes::new(archive(&fixture["cases"][0], zip::CompressionMethod::Deflated));
    cancelled.cancelled.store(true, Ordering::Relaxed);
    assert_eq!(inspect_package(&cancelled, &claim), Err(PackageSniffCancelled));
}

struct Catalog(PackageClaim, bool);

impl PickedFileCatalog for Catalog {
    fn descriptors(&self) -> Vec<FormatDescriptor> {
        vec![FormatDescriptor { kind_id: "f.sheet".into(), short_id: "sheet".into(), aliases: vec![], mimes: vec!["application/x-sheet".into()], extensions: vec![".sheet".into()], name: "sheet".into(), full_name: "sheet".into(), neutral: true, dir_name: "sheet".into(), is_binary: true }]
    }
    fn artifact_ref_of(&self, _kind_id: &str) -> Option<String> {
        Some("owner@ecma/*".into())
    }
    fn package_of(&self, kind_id: &str) -> Option<PackageClaim> {
        (self.1 && kind_id == "f.sheet").then(|| self.0.clone())
    }
}

#[test]
fn routing_follows_the_package_dialect_and_refuses_contradicting_content() {
    let fixture = fixture();
    let catalog = Catalog(claim(&fixture), true);
    let case = |id: &str| fixture["cases"].as_array().unwrap().iter().find(|row| row["id"] == id).unwrap().clone();
    let route = |id: &str, catalog: &Catalog| route_picked_file(Some(catalog), "Budget.SHEET", None, &|_| Vec::new(), Some(&Bytes::new(archive(&case(id), zip::CompressionMethod::Deflated)))).unwrap();
    let routed = |route: PickedFileRoute| match route {
        PickedFileRoute::Routed { artifact_ref, .. } => artifact_ref,
        PickedFileRoute::Refused(reason) => panic!("refused {}", reason.as_str()),
    };
    assert_eq!(routed(route("strict workbook", &catalog)), "s.stdio.xlsx@ecma-376/strict");
    assert_eq!(routed(route("transitional workbook", &catalog)), "s.stdio.xlsx@ecma-376/transitional");
    assert_eq!(routed(route("workbook in an undeclared namespace is still a workbook", &catalog)), "owner@ecma/*");
    for id in ["a word processing package is no workbook", "plain text is no zip", "zip without content types"] {
        assert_eq!(route(id, &catalog), PickedFileRoute::Refused(PickedFileRefusal::ContentMismatch), "{id}");
    }
    assert_eq!(routed(route("plain text is no zip", &Catalog(catalog.0.clone(), false))), "owner@ecma/*", "no package claim, no content read");
    assert_eq!(route_picked_file(None, "Budget.sheet", None, &|_| Vec::new(), None), Ok(PickedFileRoute::Refused(PickedFileRefusal::FormatCatalogUnavailable)));
    assert_eq!(route_picked_file(Some(&catalog), "Budget.unknown", None, &|_| Vec::new(), None), Ok(PickedFileRoute::Refused(PickedFileRefusal::FormatUnresolved)));
}
