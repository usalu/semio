//! 🌍️ Real-world documents through the controlled lifter: every document either lifts exactly like the reference lifter over the same retained graph, or stops at the barrier its ledger entry names.
//!
//! @see ../../../../../../../🧪️tests/🧫️fixtures/🌍️real-world/🔣️.json — the ledger: one expected outcome per document, advanced by each lifter slice.

use crate::standards::v1_7::subsets::base::{
    io::import::decode_pdf,
    modules::{lexer::CosGrant, lift::{lift_document, ControlledLiftEvent, DocumentLiftCursor}},
    schema::{graph_source::GraphSource, snapshot::PdfSnapshot},
};
use semio_framework_test_fixtures::{fixture_bytes, fixture_text};
use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::{reference_navigation, reference_page_fonts};

/// 🧭️ Drives the controlled lifter with exactly the grant each unit quotes; `Err` names the barrier, its entry and the pages lifted before it.
fn controlled_lift(retained: &PdfSnapshot) -> Result<PdfSnapshot, String> {
    controlled_lift_measured(retained).0
}

/// 🌐️ The largest allocation one lift unit may quote in the browser: the interactive component's 512 KiB per-turn copy grant minus the decoder's per-unit copy body.
const BROWSER_UNIT_ALLOCATION: usize = 512 * 1024 - 128;

/// 🌐️ The largest release one lift unit may ask for in the browser, and the largest empty backing it may reserve (a reservation that copies nothing is bounded by the release grant, not the copy grant): the interactive component's 1 MiB per-turn release grant.
const BROWSER_UNIT_RELEASE: usize = 1024 * 1024;

/// 📏️ The largest quote a lift asked one unit for, with the stage and the page count it was asked at.
type Largest = (usize, &'static str, usize);

/// 📏️ The largest units one lift asked for: a copying allocation, an empty reservation, a release; and its release ledger: the bytes its units allocated, the bytes they credited as released, and the first unit whose credit exceeded everything allocated before it (stage, page, excess).
#[derive(Debug)]
struct Quotes { copying: Largest, empty: Largest, release: Largest, allocated: usize, released: usize, over_credit: Option<Largest> }

impl Quotes {
    /// ⚖️ Books one unit's receipt and records the first credit above the bytes allocated so far.
    fn book(&mut self, allocated: usize, released: usize, stage: (&'static str, usize)) {
        self.allocated += allocated;
        self.released += released;
        if self.released > self.allocated && self.over_credit.is_none() { self.over_credit = Some((self.released - self.allocated, stage.0, stage.1)); }
    }
}

/// 🧭️ [`controlled_lift`] that also reports the largest copying allocation, the largest empty reservation and the largest release any unit quoted (completion and close included), each with the stage and page that quoted it.
fn controlled_lift_measured(retained: &PdfSnapshot) -> (Result<PdfSnapshot, String>, Quotes) {
    let (outcome, quotes, _) = controlled_lift_driven(retained, 0);
    (outcome, quotes)
}

/// 🏃️ [`controlled_lift_measured`] that runs every body-run stage under `run` units per call (none when `run` is zero) and also counts the calls.
fn controlled_lift_driven(retained: &PdfSnapshot, run: usize) -> (Result<PdfSnapshot, String>, Quotes, usize) {
    let mut calls = 0;
    let mut source = GraphSource::new(&retained.objects);
    let mut cursor = DocumentLiftCursor::controlled();
    let mut grant = CosGrant { work: 1, allocation_bytes: 0, release_bytes: 0 };
    let mut quotes = Quotes { copying: (0, "", 0), empty: (0, "", 0), release: (0, "", 0), allocated: 0, released: 0, over_credit: None };
    loop {
        let stage = (cursor.stage_name(), cursor.pages_lifted());
        let empty = cursor.reserving_empty_slots();
        calls += 1;
        let bodies = run != 0 && cursor.runs_bodies();
        grant.work = if bodies { run } else { 1 };
        let turn = if bodies { cursor.advance_bodies(&retained.trailer, &retained.declared_version, &mut source, grant, false) } else { cursor.advance(&retained.trailer, &retained.declared_version, &mut source, grant, false) };
        assert!(turn.work <= grant.work && turn.allocated_bytes <= grant.allocation_bytes && turn.released_bytes <= grant.release_bytes, "a unit stays inside its grant");
        quotes.book(turn.allocated_bytes, turn.released_bytes, stage);
        let event = match turn.event {
            Ok(event) => event,
            Err(reason) => return (Err(reason.to_owned()), quotes, calls),
        };
        match event {
            ControlledLiftEvent::NeedsAllocation { bytes } => {
                let largest = if empty { &mut quotes.empty } else { &mut quotes.copying };
                if bytes > largest.0 { *largest = (bytes, stage.0, stage.1); }
                grant.allocation_bytes = bytes;
            }
            ControlledLiftEvent::NeedsRelease { bytes } => {
                if bytes > quotes.release.0 { quotes.release = (bytes, stage.0, stage.1); }
                grant.release_bytes = bytes;
            }
            ControlledLiftEvent::More => { grant.allocation_bytes = 0; grant.release_bytes = 0; }
            ControlledLiftEvent::Complete => return (Ok(cursor.take_result().expect("completed controlled lift")), quotes, calls),
            ControlledLiftEvent::Deferred(_) => {
                let (reason, detail, pages) = cursor.deferred_detail().expect("deferred controlled lift");
                let outcome = format!("{reason:?}:{detail}:{pages}");
                cursor.begin_close();
                let mut close = CosGrant { work: 1, allocation_bytes: 0, release_bytes: 0 };
                loop {
                    let turn = cursor.close_step(close);
                    assert!(turn.released_bytes <= close.release_bytes, "a close unit stays inside its grant");
                    quotes.book(turn.allocated_bytes, turn.released_bytes, ("close", pages));
                    match turn.event {
                        Ok(ControlledLiftEvent::Closed) => break,
                        Ok(ControlledLiftEvent::NeedsRelease { bytes }) => {
                            if bytes > quotes.release.0 { quotes.release = (bytes, "close", pages); }
                            close.release_bytes = bytes;
                        }
                        _ => close.release_bytes = 0,
                    }
                }
                assert!(cursor.take_partial().snapshot.is_some());
                return (Err(outcome), quotes, calls);
            }
            event => panic!("unexpected controlled PDF lift state {event:?}"),
        }
    }
}

/// 🌐️ LAW: every unit of every real-world lift quotes at most what one browser turn can copy into a new allocation, reserves at most what one browser turn can reserve empty, and asks for at most what one browser turn can release (completion and close included), so no document needs a backing or a retirement above the interactive grants; and no lift ever credits more release than its units allocated, so the host's lifetime release stock never runs dry; the report names the largest units of each document, their stage and page.
#[test]
fn every_real_world_lift_unit_fits_one_browser_turn() {
    let ledger: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🌍️real-world/🔣️.json")).unwrap();
    let mut report = Vec::new();
    for case in ledger["documents"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let input = fixture_bytes!(format!("../../../../{}", case["path"].as_str().unwrap()));
        let retained = decode_pdf(input).unwrap();
        let (_, quotes) = controlled_lift_measured(&retained);
        report.push((name.to_owned(), quotes));
    }
    let over: Vec<_> = report.iter().filter_map(|(name, quotes)| quotes.over_credit.map(|(bytes, stage, page)| format!("{name}: credits {bytes} bytes more release than it allocated at {stage} on page {page}"))).collect();
    assert!(over.is_empty(), "lifts credit release they were never charged: {over:?}; every document: {report:?}");
    let above: Vec<_> = report.iter().flat_map(|(name, Quotes { copying, empty, release, .. })| {
        [(copying, BROWSER_UNIT_ALLOCATION, "copied into a new allocation"), (empty, BROWSER_UNIT_RELEASE, "reserved empty"), (release, BROWSER_UNIT_RELEASE, "released")]
            .into_iter().filter(|((bytes, _, _), bound, _)| bytes > bound).map(|((bytes, stage, page), _, what)| format!("{name}: {bytes} bytes {what} in {stage} on page {page}")).collect::<Vec<_>>()
    }).collect();
    assert!(above.is_empty(), "lift units above one browser turn: {above:?}; every document: {report:?}");
}

/// 📐️ LAW: the measured extent of every form and glyph program of every real-world document counts exactly the operators the reference parser types from it, so the controlled lifter quotes those programs by their real size instead of one operator per two body bytes.
#[test]
fn measured_program_extents_count_every_operator_the_reference_parser_types() {
    use crate::standards::v1_7::subsets::base::{modules::content::content_extent, schema::{snapshot::PdfObject, stream_roles::PdfStreamRoleValue}};
    let ledger: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🌍️real-world/🔣️.json")).unwrap();
    let mut measured = 0;
    for case in ledger["documents"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let retained = decode_pdf(fixture_bytes!(format!("../../../../{}", case["path"].as_str().unwrap()))).unwrap();
        let reference = lift_document(&retained.trailer, &retained.declared_version, &mut GraphSource::new(&retained.objects)).unwrap();
        for role in reference.admitted_stream_roles.iter().filter(|role| role.identity.path.is_empty()) {
            let PdfStreamRoleValue::Operators { content } = &role.value else { continue; };
            let Some(PdfObject::Stream { data, .. }) = retained.objects.iter().find(|object| object.id == role.identity.owner).map(|object| &object.value) else { continue; };
            let extent = content_extent(&data.to_vec_owner()).unwrap_or_else(|| panic!("{name}: program {:?} measures", role.identity.owner));
            assert_eq!(extent.operators, content.len(), "{name}: program {:?}", role.identity.owner);
            measured += 1;
        }
    }
    assert!(measured > 800, "the ledger's form and glyph programs are measured ({measured})");
}

/// 📖️ LAW: an import keeps exactly the streams the lift does not interpret as their encoded file bytes: the reference lifter reads no encoded stream of any ledger document except the large sampled images it types as artifacts of their encoded body, every stream left encoded equals the file's own body (lopdf, undecoded), and the thesis keeps its application data and its large images encoded instead of inflating them.
#[test]
fn imports_keep_uninterpreted_streams_encoded_and_decode_every_stream_the_lift_reads() {
    use crate::standards::v1_7::subsets::base::schema::{graph_source::ObjectSource, snapshot::{ObjRef, PdfObject}, stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfStreamRoleKind, PdfStreamRoleValue}};
    use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::{io::foreign_artifacts::encoded_sampled_image, modules::filters::declares_logical_codec};
    fn encoded(value: Option<&PdfObject>) -> bool { matches!(value, Some(PdfObject::Stream { dict, filters, .. }) if filters.is_empty() && declares_logical_codec(dict)) }
    struct Reads<'a> { graph: GraphSource<'a>, encoded: Vec<ObjRef> }
    impl ObjectSource for Reads<'_> {
        fn get(&mut self, reference: ObjRef) -> Option<PdfObject> {
            let value = self.graph.get(reference);
            if encoded(value.as_ref()) && !self.encoded.contains(&reference) { self.encoded.push(reference); }
            value
        }
        fn inspect(&mut self, reference: ObjRef, read: &mut dyn FnMut(Option<&PdfObject>)) {
            let mut seen = false;
            self.graph.inspect(reference, &mut |value| { seen = encoded(value); read(value); });
            if seen && !self.encoded.contains(&reference) { self.encoded.push(reference); }
        }
        fn identity_of(&self, value: &PdfObject) -> Option<PdfGraphIdentity> { self.graph.identity_of(value) }
        fn role_dependencies(&self, identity: &PdfGraphIdentity) -> Vec<PdfGraphIdentity> { self.graph.role_dependencies(identity) }
        fn record_role(&mut self, value: &PdfObject, role: PdfStreamRoleValue) { self.graph.record_role(value, role) }
        fn record_role_dependencies(&mut self, value: &PdfObject, kind: PdfStreamRoleKind, dependencies: &[PdfGraphIdentity]) { self.graph.record_role_dependencies(value, kind, dependencies) }
        fn admitted_roles(&self) -> &[PdfAdmittedStreamRole] { self.graph.admitted_roles() }
        fn take_admitted_roles(&mut self) -> Vec<PdfAdmittedStreamRole> { self.graph.take_admitted_roles() }
        fn semantic_role(&mut self, value: &PdfObject, kind: PdfStreamRoleKind) -> Option<PdfStreamRoleValue> { self.graph.semantic_role(value, kind) }
        fn semantic_role_at(&mut self, identity: &PdfGraphIdentity, kind: PdfStreamRoleKind) -> Option<PdfStreamRoleValue> { self.graph.semantic_role_at(identity, kind) }
    }
    let ledger: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🌍️real-world/🔣️.json")).unwrap();
    let mut report = Vec::new();
    for case in ledger["documents"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let input = fixture_bytes!(format!("../../../../{}", case["path"].as_str().unwrap()));
        let retained = decode_pdf(input).unwrap();
        let mut reads = Reads { graph: GraphSource::new(&retained.objects), encoded: Vec::new() };
        lift_document(&retained.trailer, &retained.declared_version, &mut reads).unwrap();
        let interpreted: Vec<_> = reads.encoded.iter().filter(|reference| !retained.objects.iter().any(|object| object.id == **reference && object.value.as_dict().is_some_and(encoded_sampled_image))).collect();
        assert!(interpreted.is_empty(), "{name}: the reference lifter reads encoded streams other than artifact images {interpreted:?}");
        let bodies: std::collections::HashMap<(u32, u16), Vec<u8>> = semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_encoded_stream_bodies(input).unwrap().into_iter().collect();
        let (mut kept, mut kept_bytes, mut decoded_bytes) = (0, 0, 0);
        for object in &retained.objects {
            let PdfObject::Stream { data, .. } = &object.value else { continue; };
            if !encoded(Some(&object.value)) { decoded_bytes += data.len(); continue; }
            assert_eq!(Some(&data.to_vec_owner()), bodies.get(&(object.id.num, object.id.gen)), "{name}: encoded stream {:?} is the file's body", object.id);
            kept += 1;
            kept_bytes += data.len();
        }
        report.push(format!("{name}: {kept} streams kept encoded ({kept_bytes} bytes), {decoded_bytes} bytes decoded"));
        if name.contains("thesis") { assert!(kept > 0, "{name}: application data stays encoded: {report:?}"); }
    }
    println!("{report:#?}");
}

/// 🖼️ LAW: a page painting more images than the former 32-slot table lifts on the controlled path exactly like the reference lifter, every unit inside its grant.
#[test]
fn a_page_with_forty_images_lifts_like_the_reference_lifter() {
    use crate::standards::v1_7::subsets::base::{io::export::encode_pdf, schema::snapshot::{PdfColorSpace, PdfImage, PdfOp, PdfPage}};
    let mut page = PdfPage::new(400.0, 400.0);
    let mut document = PdfSnapshot::default();
    for index in 0..40u8 {
        let id = format!("I{index}");
        document.images.push(PdfImage::gray8(id.clone(), 2, 2, vec![index; 4]));
        page.content.push(PdfOp::PaintXObject { name: id });
    }
    document.pages.push(page);
    let retained = decode_pdf(&encode_pdf(&document).unwrap()).unwrap();
    let reference = typed(lift_document(&retained.trailer, &retained.declared_version, &mut GraphSource::new(&retained.objects)).unwrap());
    let controlled = typed(controlled_lift(&retained).unwrap_or_else(|barrier| panic!("forty images stop at {barrier}")));
    assert_eq!(controlled.images.len(), 40);
    assert_eq!(controlled, reference);
}

/// 🎯️ A document whose catalog carries an output intent without a destination profile lifts to the same typed intents as the reference lifter instead of stopping at the catalog barrier, so a saved `set-output-intent` edit opens again.
#[test]
fn output_intents_without_a_profile_lift_like_the_reference_lifter() {
    use crate::standards::v1_7::subsets::base::{io::export::encode_pdf, schema::snapshot::{PdfOutputIntent, PdfPage}};
    let mut document = PdfSnapshot::default();
    document.pages.push(PdfPage::new(200.0, 200.0));
    document.output_intents.push(PdfOutputIntent { subtype: "GTS_PDFA1".into(), condition_identifier: "sRGB IEC61966-2.1".into(), condition: None, registry_name: None, info: Some("sweep intent".into()), profile: None });
    let retained = decode_pdf(&encode_pdf(&document).unwrap()).unwrap();
    let reference = typed(lift_document(&retained.trailer, &retained.declared_version, &mut GraphSource::new(&retained.objects)).unwrap());
    let controlled = typed(controlled_lift(&retained).unwrap_or_else(|barrier| panic!("an output intent stops at {barrier}")));
    assert_eq!(controlled.output_intents, document.output_intents);
    assert_eq!(controlled, reference);
}

/// 🏃️ LAW: body runs (page programs, image bytes) lift every ledger document to exactly the typed snapshot or barrier that single units reach, every run inside its grant and no unit above one browser turn, in at most a quarter of the ledger's calls.
#[test]
fn body_runs_lift_every_real_world_document_like_single_units_in_fewer_calls() {
    let ledger: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🌍️real-world/🔣️.json")).unwrap();
    let mut report = Vec::new();
    for case in ledger["documents"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let retained = decode_pdf(fixture_bytes!(format!("../../../../{}", case["path"].as_str().unwrap()))).unwrap();
        let (single, _, unit_calls) = controlled_lift_driven(&retained, 0);
        let (run, quotes, run_calls) = controlled_lift_driven(&retained, 4096);
        assert_eq!(run.map(typed), single.map(typed), "{name}: a body-run lift reaches the single-unit outcome");
        assert!(quotes.copying.0 <= BROWSER_UNIT_ALLOCATION && quotes.empty.0 <= BROWSER_UNIT_RELEASE && quotes.release.0 <= BROWSER_UNIT_RELEASE && quotes.over_credit.is_none(), "{name}: {quotes:?}");
        report.push((name.to_owned(), unit_calls, run_calls));
    }
    let (units, runs) = report.iter().fold((0, 0), |(units, runs), (_, unit_calls, run_calls)| (units + unit_calls, runs + run_calls));
    assert!(4 * runs <= units, "body runs save at least three quarters of the ledger's calls ({runs} of {units}); every document (single units, runs): {report:?}");
    println!("body runs: {report:?}");
}

fn typed(mut snapshot: PdfSnapshot) -> PdfSnapshot {
    snapshot.objects = Vec::new();
    snapshot.trailer = Vec::new();
    snapshot
}

#[test]
fn real_world_documents_lift_like_the_reference_lifter_or_stop_at_their_ledgered_barrier() {
    let ledger: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🌍️real-world/🔣️.json")).unwrap();
    let mut report = Vec::new();
    for case in ledger["documents"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let input = fixture_bytes!(format!("../../../../{}", case["path"].as_str().unwrap()));
        let retained = decode_pdf(input).unwrap_or_else(|error| panic!("{name}: the reference reader refuses the document: {error:?}"));
        let reference = typed(lift_document(&retained.trailer, &retained.declared_version, &mut GraphSource::new(&retained.objects)).unwrap());
        let outcome = match controlled_lift(&retained) {
            Ok(controlled) => {
                let controlled = typed(controlled);
                assert_eq!(controlled.pages, reference.pages, "{name}: pages");
                assert_eq!(controlled.outlines, reference.outlines, "{name}: outlines");
                assert_eq!(controlled.named_destinations, reference.named_destinations, "{name}: named destinations");
                assert_eq!(controlled.page_labels, reference.page_labels, "{name}: page labels");
                assert_eq!(controlled.open_action, reference.open_action, "{name}: open action");
                assert_eq!(controlled.viewer_preferences, reference.viewer_preferences, "{name}: viewer preferences");
                assert_eq!(controlled.metadata, reference.metadata, "{name}: metadata");
                assert_eq!(controlled.catalog_extra, reference.catalog_extra, "{name}: catalog extras");
                assert_eq!(controlled.info, reference.info, "{name}: info");
                let (lifted, expected) = (&controlled.admitted_stream_roles, &reference.admitted_stream_roles);
                if let Some(at) = (0..lifted.len().max(expected.len())).find(|at| lifted.get(*at) != expected.get(*at)) {
                    let describe = |role: Option<&crate::standards::v1_7::subsets::base::schema::stream_roles::PdfAdmittedStreamRole>| role.map(|role| {
                        let other = if std::ptr::eq(role, lifted.get(at).unwrap_or(role)) { expected.get(at) } else { lifted.get(at) };
                        let split = other.and_then(|other| role.dependencies.iter().zip(&other.dependencies).position(|(a, b)| a != b));
                        format!("{:?} {:?} {} dependencies, first differing at {split:?}: {:?}, value equal {}", role.identity, role.value.kind(), role.dependencies.len(), split.map(|split| &role.dependencies[split]), other.is_some_and(|other| other.value == role.value))
                    });
                    panic!("{name}: stream role {at} of {}/{} differs\ncontrolled {:?}\nreference  {:?}", lifted.len(), expected.len(), describe(lifted.get(at)), describe(expected.get(at)));
                }
                assert_eq!(controlled, reference, "{name}: every typed lane");
                "equal".to_owned()
            }
            Err(barrier) => barrier,
        };
        report.push((name.to_owned(), outcome, case["expect"].as_str().unwrap().to_owned()));
    }
    let lifted: Vec<_> = report.iter().map(|(name, outcome, _)| format!("{name} => {outcome}")).collect();
    let ledgered: Vec<_> = report.iter().map(|(name, _, expect)| format!("{name} => {expect}")).collect();
    assert_eq!(lifted, ledgered, "every document stops at its ledgered barrier or lifts like the reference lifter");
}

#[test]
fn navigation_fixture_lanes_agree_with_lopdf_and_the_schema_first_spec() {
    let spec: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧭️navigation/spec.json")).unwrap();
    let input = fixture_bytes!("../../../../🧪️tests/🧫️fixtures/🧭️navigation/navigation.pdf");
    let retained = decode_pdf(input).unwrap();
    let lifted = controlled_lift(&retained).unwrap_or_else(|barrier| panic!("navigation fixture stops at {barrier}"));
    let independent = reference_navigation(input).unwrap();
    let toc = spec["toc"].as_array().unwrap();
    assert_eq!(independent.outline.len(), toc.len());
    let mut flattened = Vec::new();
    let mut stack: Vec<(usize, &[crate::standards::v1_7::subsets::base::schema::snapshot::PdfOutlineItem])> = vec![(1, lifted.outlines.as_slice())];
    while let Some((depth, items)) = stack.pop() {
        let Some((first, rest)) = items.split_first() else { continue };
        stack.push((depth, rest));
        flattened.push((depth, first.title.clone()));
        stack.push((depth + 1, first.children.as_slice()));
    }
    for ((depth, title), (expected, (reference_depth, reference_title, _))) in flattened.iter().zip(toc.iter().zip(&independent.outline)) {
        assert_eq!((*depth as u64, title.as_str()), (expected[0].as_u64().unwrap(), expected[1].as_str().unwrap()));
        assert_eq!((depth, title), (reference_depth, reference_title));
    }
    let mut named: Vec<(String, u64)> = lifted.named_destinations.iter().map(|named| match &named.destination {
        crate::standards::v1_7::subsets::base::schema::snapshot::PdfDestination::Page { page, .. } => (named.name.clone(), u64::from(*page) + 1),
        other => panic!("{other:?}"),
    }).collect();
    named.sort();
    let mut expected: Vec<(String, u64)> = spec["namedDestinations"].as_object().unwrap().iter().map(|(name, page)| (name.clone(), page.as_u64().unwrap())).collect();
    expected.sort();
    assert_eq!(named, expected);
    let mut reference_named: Vec<(String, u64)> = independent.named.iter().map(|(name, page)| (name.clone(), page.unwrap() as u64 + 1)).collect();
    reference_named.sort();
    assert_eq!(named, reference_named);
    assert_eq!(lifted.page_labels.len(), independent.labels.len());
    for (label, (start, _, prefix, first)) in lifted.page_labels.iter().zip(&independent.labels) {
        assert_eq!((i64::from(label.start_index), label.prefix.clone(), i64::from(label.start)), (*start, prefix.clone(), first.unwrap_or(1)));
    }
    assert_eq!(independent.open_page, Some(spec["openActionPage"].as_u64().unwrap() as usize - 1));
    assert!(matches!(&lifted.open_action, Some(crate::standards::v1_7::subsets::base::schema::snapshot::PdfOpenAction::Destination { destination: crate::standards::v1_7::subsets::base::schema::snapshot::PdfDestination::Page { page: 0, .. } })));
    assert_eq!(lifted.pages.iter().map(|page| page.annotations.len()).collect::<Vec<_>>(), independent.annotations);
    assert_eq!(lifted.info.title.as_deref(), spec["info"]["title"].as_str());
    assert_eq!(lifted.info.author.as_deref(), spec["info"]["author"].as_str());
    assert_eq!(lifted.info.extra.iter().map(|entry| entry.key.as_str()).collect::<Vec<_>>(), spec["info"]["extra"].as_array().unwrap().iter().map(|key| key.as_str().unwrap()).collect::<Vec<_>>());
    assert_eq!(lifted.catalog_extra.iter().map(|entry| entry.key.as_str()).collect::<Vec<_>>(), spec["catalogExtra"].as_array().unwrap().iter().map(|key| key.as_str().unwrap()).collect::<Vec<_>>());
    for key in &lifted.catalog_extra { assert!(independent.catalog_keys.contains(&key.key)); }
    assert!(lifted.metadata.as_deref().is_some_and(|text| text.contains("Navigation fixture")));
    assert!(lifted.viewer_preferences.as_ref().is_some_and(|preferences| preferences.display_doc_title));
    assert!(lifted.pages[spec["groupPage"].as_u64().unwrap() as usize - 1].group.is_some());
}

#[test]
fn reference_glyph_programs_depend_on_every_claimed_font_in_object_order() {
    use crate::standards::v1_7::subsets::base::schema::{snapshot::{ObjRef, PdfObject}, stream_roles::{PdfGraphIdentity, PdfStreamRoleValue}};
    let input = fixture_bytes!("../../../../🧪️tests/🧫️fixtures/🔤️fonts/fonts.pdf");
    let retained = decode_pdf(input).unwrap();
    let lift = || lift_document(&retained.trailer, &retained.declared_version, &mut GraphSource::new(&retained.objects)).unwrap().admitted_stream_roles;
    let first = lift();
    for run in 0..8 { assert_eq!(lift(), first, "lift {run} records the same roles in the same order"); }
    let graph = GraphSource::new(&retained.objects);
    let page = retained.objects.iter().find(|object| object.value.dict_get("Type").and_then(PdfObject::as_name) == Some("Page")).unwrap();
    let resources = match page.value.dict_get("Resources").expect("page resources") {
        PdfObject::Ref(owner) => &retained.objects.iter().find(|object| object.id == *owner).unwrap().value,
        direct => direct,
    };
    let entries = match resources.dict_get("Font").expect("page font dictionary") {
        PdfObject::Ref(owner) => retained.objects.iter().find(|object| object.id == *owner).unwrap().value.as_dict().unwrap(),
        direct => direct.as_dict().unwrap(),
    };
    let claimed: Vec<ObjRef> = entries[..=entries.iter().position(|entry| entry.key == "T3").unwrap()].iter().map(|entry| entry.value.as_ref().unwrap()).collect();
    assert!(claimed.len() > 2 && !claimed.is_sorted(), "the Type 3 font is claimed after fonts of higher and lower object numbers");
    let mut sorted = claimed.clone();
    sorted.sort_unstable();
    let glyphs: Vec<_> = first.iter().filter(|role| matches!(role.value, PdfStreamRoleValue::Operators { .. }) && role.identity.path.is_empty()).collect();
    assert_eq!(glyphs.len(), 42, "one role per glyph program of the pdfTeX Type 3 font");
    for glyph in glyphs {
        let mut generators = graph.generators_of(&glyph.identity);
        let mut inputs = graph.dependencies_of(&glyph.identity);
        for font in &sorted {
            let root = PdfGraphIdentity { owner: *font, path: Vec::new() };
            for generator in graph.generators_of(&root) { if !generators.contains(&generator) { generators.push(generator); } }
            for dependency in graph.dependencies_of(&root) { if !inputs.contains(&dependency) { inputs.push(dependency); } }
        }
        assert_eq!(glyph.dependencies, generators, "glyph {:?} stores its own identity and every claimed font root once, in object order", glyph.identity.owner);
        let derived = graph.closure_of(&glyph.dependencies);
        assert_eq!((derived.len(), derived.iter().all(|input| inputs.contains(input))), (inputs.len(), true), "glyph {:?} derives every input the full dependency list named", glyph.identity.owner);
    }
}

#[test]
fn fonts_fixture_types_every_font_kind_like_lopdf_and_the_schema_first_spec() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfFontKind, PdfFontProgram, PdfOp, PdfTextString, PdfTextArrayItem};
    let spec: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🔤️fonts/spec.json")).unwrap();
    let input = fixture_bytes!("../../../../🧪️tests/🧫️fixtures/🔤️fonts/fonts.pdf");
    let retained = decode_pdf(input).unwrap();
    let lifted = controlled_lift(&retained).unwrap_or_else(|barrier| panic!("font fixture stops at {barrier}"));
    let independent = reference_page_fonts(input).unwrap();
    let pages = spec["pages"].as_array().unwrap();
    assert_eq!((lifted.pages.len(), independent.len()), (pages.len(), pages.len()));
    assert_eq!(lifted.fonts.len() as u64, spec["fontCount"].as_u64().unwrap());
    for ((page, expected), reference) in lifted.pages.iter().zip(pages).zip(&independent) {
        let names: Vec<&str> = expected["fonts"].as_array().unwrap().iter().map(|name| name.as_str().unwrap()).collect();
        let mut declared: Vec<&str> = reference.iter().map(|(name, _, _)| name.as_str()).collect();
        declared.sort_unstable();
        let mut sorted = names.clone();
        sorted.sort_unstable();
        assert_eq!(declared, sorted, "lopdf declares the spec's fonts");
        for ((name, kind), (_, subtype, program)) in names.iter().zip(expected["kinds"].as_array().unwrap()).zip(names.iter().map(|name| reference.iter().find(|entry| entry.0 == *name).unwrap())) {
            let font = lifted.fonts.iter().find(|font| font.id == *name).unwrap_or_else(|| panic!("{name} is typed"));
            let typed = match &font.kind { PdfFontKind::Type0 { .. } => "Type0", PdfFontKind::TrueType { .. } => "TrueType", PdfFontKind::Type3 { .. } => "Type3", PdfFontKind::Type1 { .. } => "Type1" };
            assert_eq!((typed, subtype.as_str()), (kind.as_str().unwrap(), kind.as_str().unwrap()), "{name}");
            let embedded = match &font.kind {
                PdfFontKind::Type1 { program, .. } | PdfFontKind::TrueType { program, .. } => program.clone(),
                PdfFontKind::Type0 { descendant, .. } => descendant.program.clone(),
                PdfFontKind::Type3 { .. } => None,
            };
            let key = embedded.map(|program| match program { PdfFontProgram::Type1 { .. } => "FontFile", PdfFontProgram::TrueType { .. } => "FontFile2", _ => "FontFile3" });
            assert_eq!(key, program.as_deref(), "{name}: program kind agrees with lopdf");
            assert_eq!(key, spec["programs"][*name].as_str(), "{name}: program kind agrees with the spec");
        }
        let mut shown = Vec::new();
        for op in &page.content {
            let mut push = |text: &PdfTextString| if let PdfTextString::Text { text } = text { shown.push(text.clone()); };
            match op {
                PdfOp::ShowText { text } => push(text),
                PdfOp::ShowTextArray { items } => {
                    let joined: String = items.iter().filter_map(|item| match item { PdfTextArrayItem::Text { text } => Some(text.as_str()), _ => None }).collect();
                    shown.push(joined);
                }
                _ => {}
            }
        }
        for text in expected["text"].as_array().unwrap().iter().map(|text| text.as_str().unwrap()) {
            assert!(shown.iter().any(|line| line == text), "{text} decodes as text: {shown:?}");
        }
    }
    let Some(PdfFontKind::Type3 { char_procs, .. }) = lifted.fonts.iter().find(|font| font.id == "T3").map(|font| &font.kind) else { panic!("typed Type 3 font") };
    assert_eq!(char_procs.len(), 42);
    assert!(char_procs.iter().all(|glyph| glyph.content.iter().any(|op| matches!(op, PdfOp::InlineImage { .. }))), "every pdfTeX bitmap glyph paints its inline image");
}

/// 🧷️ LAW: images typed inside form XObjects (RGB, gray with a soft mask, a JPEG, an Indexed bitmap and a stencil mask; a nested form; a name two forms both author; a form shared between pages), page-level colour spaces, shadings and patterns (direct and indirect, a tiling pattern that types an image, an object under another name, a direct name another page already claimed) and a catalog interactive form (a field tree with widget-only kids, widgets with appearance streams, default resources) lift on the controlled path exactly like the reference lifter, every unit inside one browser turn, and both independent readers (lopdf, the schema-first specs) name the same images, forms, resources, fields and widgets.
#[test]
fn form_images_page_resources_and_acro_form_lift_like_the_reference_lifter_and_both_readers() {
    use crate::standards::v1_7::subsets::base::schema::{snapshot::{PdfAnnotationKind, PdfFormField, PdfFormFieldKind, PdfOp}, stream_roles::PdfStreamRoleValue};
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::{reference_form_fields, reference_page_resource_names, reference_reachable_xobjects, ReferenceField};
    fn shape(fields: &[PdfFormField]) -> Vec<ReferenceField> {
        fields.iter().map(|field| ReferenceField {
            name: field.name.clone(),
            field_type: match field.kind { PdfFormFieldKind::Button { .. } => Some("Btn"), PdfFormFieldKind::Text { .. } => Some("Tx"), PdfFormFieldKind::Choice { .. } => Some("Ch"), PdfFormFieldKind::Signature { .. } => Some("Sig"), PdfFormFieldKind::Container => None }.map(str::to_owned),
            widgets: field.widgets.len(),
            children: shape(&field.children),
        }).collect()
    }
    let strings = |value: &serde_json::Value| value.as_array().unwrap().iter().map(|item| item.as_str().unwrap().to_owned()).collect::<Vec<_>>();
    let lift = |input: &[u8], name: &str| {
        let retained = decode_pdf(input).unwrap_or_else(|error| panic!("{name}: the reference reader refuses the document: {error:?}"));
        let reference = typed(lift_document(&retained.trailer, &retained.declared_version, &mut GraphSource::new(&retained.objects)).unwrap());
        let (outcome, quotes) = controlled_lift_measured(&retained);
        let controlled = typed(outcome.unwrap_or_else(|barrier| panic!("{name} stops at {barrier}")));
        assert!(quotes.copying.0 <= BROWSER_UNIT_ALLOCATION && quotes.empty.0 <= BROWSER_UNIT_RELEASE && quotes.release.0 <= BROWSER_UNIT_RELEASE && quotes.over_credit.is_none(), "{name}: {quotes:?}");
        assert_eq!(controlled, reference, "{name}: every typed lane");
        controlled
    };

    let input = fixture_bytes!("../../../../🧪️tests/🧫️fixtures/🖼️form-images/form-images.pdf");
    let spec: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🖼️form-images/spec.json")).unwrap();
    let images = lift(input, "form images");
    assert_eq!(images.images.iter().map(|image| image.id.clone()).collect::<Vec<_>>(), strings(&spec["images"]));
    assert_eq!(images.forms.iter().map(|form| form.id.clone()).collect::<Vec<_>>(), strings(&spec["forms"]));
    assert!(images.admitted_stream_roles.iter().filter(|role| matches!(role.value, PdfStreamRoleValue::Image { .. })).count() == images.images.len(), "every image keeps its own role");
    let (objects, forms) = reference_reachable_xobjects(input).unwrap();
    assert_eq!((objects.len(), forms.len()), (spec["imageObjects"].as_array().unwrap().len(), spec["formObjects"].as_array().unwrap().len()), "lopdf reaches the same image and form objects");

    let input = fixture_bytes!("../../../../🧪️tests/🧫️fixtures/🎨️page-resources/page-resources.pdf");
    let spec: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🎨️page-resources/spec.json")).unwrap();
    let resources = lift(input, "page resources");
    assert_eq!(resources.color_spaces.iter().map(|space| space.name.clone()).collect::<Vec<_>>(), strings(&spec["colorSpaces"]));
    assert_eq!(resources.shadings.iter().map(|shading| shading.id.clone()).collect::<Vec<_>>(), strings(&spec["shadings"]));
    assert_eq!(resources.patterns.iter().map(|pattern| pattern.id.clone()).collect::<Vec<_>>(), strings(&spec["patterns"]));
    assert_eq!(resources.images.iter().map(|image| image.id.clone()).collect::<Vec<_>>(), strings(&spec["images"]), "the tiling pattern types its own image");
    for key in ["ColorSpace", "Shading", "Pattern"] {
        let declared = reference_page_resource_names(input, key).unwrap();
        for (page, expected) in declared.iter().zip(spec["declared"].as_array().unwrap()) { assert_eq!(page, &strings(&expected[key]), "lopdf {key} resources"); }
    }
    let spaces = |page: usize| resources.pages[page].content.iter().filter_map(|op| match op { PdfOp::SetFillColorSpace { name } => Some(name.as_str()), _ => None }).collect::<Vec<_>>();
    assert_eq!(spaces(0), ["CsD", "CsR", "CsS", "CsN", "Pattern", "Pattern"]);
    assert_eq!(spaces(1), ["CsR", "CsD_2"], "page 2 renames the object it knows under another name and the direct space another page claimed");

    let input = fixture_bytes!("../../../../🧪️tests/🧫️fixtures/📝️acro-form/acro-form.pdf");
    let spec: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/📝️acro-form/spec.json")).unwrap();
    let form = lift(input, "acro form");
    let acro = form.acro_form.as_ref().expect("the interactive form is typed");
    let (fields, per_page) = reference_form_fields(input).unwrap();
    assert_eq!(shape(&acro.fields), fields, "the field tree equals lopdf's");
    assert_eq!(acro.fields.iter().map(|field| field.name.as_str()).collect::<Vec<_>>(), spec["fields"].as_array().unwrap().iter().map(|field| field["name"].as_str().unwrap()).collect::<Vec<_>>());
    assert_eq!(acro.default_fonts, strings(&spec["defaultFonts"]));
    let widgets: Vec<usize> = form.pages.iter().map(|page| page.annotations.iter().filter(|annotation| matches!(annotation.kind, PdfAnnotationKind::Widget { .. })).count()).collect();
    assert_eq!(widgets, per_page);
    assert_eq!(widgets, spec["widgetsPerPage"].as_array().unwrap().iter().map(|count| count.as_u64().unwrap() as usize).collect::<Vec<_>>());
    assert!(form.pages.iter().flat_map(|page| &page.annotations).filter(|annotation| matches!(annotation.kind, PdfAnnotationKind::Widget { .. })).all(|annotation| annotation.appearance.is_some()), "every widget keeps its appearance streams");
    assert!(!form.forms.is_empty() && form.fonts.iter().any(|font| font.id == "Helv"), "the appearance streams type their forms and the shared default font");
}

/// 🪙️ What one browser turn grants a decode step: the copy grant, the retained items and the fuel (one unit per token, one per 64 body bytes the importer charges).
const TURN_COPY: usize = 1_048_463;
const TURN_ITEMS: usize = 63;
const TURN_FUEL: usize = 4096;

/// 🧮️ Drives the lift under the importer's accounting (copy charged per unit at 128 bytes, or two per body byte in a run of up to the turn's remaining copy, an allocation charged as copy unless it only reserves empty slots, a turn ending when its copy, item or fuel grant is spent or a demand cannot be met); the outcome, the turns and per stage `(calls, copy)`.
fn controlled_lift_turns(retained: &PdfSnapshot) -> (Result<PdfSnapshot, String>, usize, std::collections::BTreeMap<&'static str, (usize, usize)>) {
    use crate::standards::v1_7::subsets::base::modules::lift::ControlledDocumentLiftCursor;
    let mut source = GraphSource::new(&retained.objects);
    let mut cursor = DocumentLiftCursor::controlled();
    let (mut turns, mut remaining, mut items, mut fuel, mut demanded, mut release) = (1, TURN_COPY, TURN_ITEMS, TURN_FUEL, 0, 0);
    let mut report = std::collections::BTreeMap::new();
    loop {
        let stage = cursor.stage_name();
        let empty = cursor.reserving_empty_slots();
        let bodies = cursor.runs_bodies();
        let inline = cursor.next_inline_body_bytes();
        let unit = if bodies { 2 } else { 128.max(inline) };
        if remaining < unit || items == 0 || fuel == 0 { turns += 1; (remaining, items, fuel) = (TURN_COPY, TURN_ITEMS, TURN_FUEL); }
        let work = if bodies { (remaining.saturating_sub(demanded) / (2 * unit)).min(fuel * 64).max(ControlledDocumentLiftCursor::RUN_UNIT_MAXIMUM) } else { 1 };
        if bodies && work * unit > remaining { turns += 1; (remaining, items, fuel) = (TURN_COPY, TURN_ITEMS, TURN_FUEL); }
        let boundary = if empty { TURN_COPY * 2 } else { remaining.saturating_sub(unit * work) };
        let grant = CosGrant { work, allocation_bytes: boundary.min(16 << 20), release_bytes: release.max(TURN_COPY) };
        let turn = if bodies { cursor.advance_bodies(&retained.trailer, &retained.declared_version, &mut source, grant, false) } else { cursor.advance(&retained.trailer, &retained.declared_version, &mut source, grant, false) };
        let copied = if empty { 0 } else { turn.allocated_bytes } + turn.work * unit;
        remaining = remaining.saturating_sub(copied);
        fuel = fuel.saturating_sub(turn.work.div_ceil(if bodies { 64 } else { 1 }));
        items = items.saturating_sub(usize::from(turn.allocated_bytes != 0 || inline != 0 && turn.work != 0 && matches!(turn.event, Ok(ControlledLiftEvent::More)) || matches!(turn.event, Ok(ControlledLiftEvent::Complete))));
        let entry = report.entry(stage).or_insert((0, 0));
        *entry = (entry.0 + 1, entry.1 + copied);
        (demanded, release) = (0, 0);
        match turn.event {
            Err(reason) => return (Err(reason.to_owned()), turns, report),
            Ok(ControlledLiftEvent::NeedsAllocation { bytes }) => { demanded = bytes; turns += 1; (remaining, items, fuel) = (TURN_COPY, TURN_ITEMS, TURN_FUEL); }
            Ok(ControlledLiftEvent::NeedsRelease { bytes }) => { release = bytes; turns += 1; (remaining, items, fuel) = (TURN_COPY, TURN_ITEMS, TURN_FUEL); }
            Ok(ControlledLiftEvent::More) => {}
            Ok(ControlledLiftEvent::Complete) => return (Ok(cursor.take_result().expect("completed controlled lift")), turns, report),
            Ok(ControlledLiftEvent::Deferred(_)) => return (Err("deferred".to_owned()), turns, report),
            event => panic!("unexpected controlled PDF lift state {event:?}"),
        }
    }
}

/// 🚦️ LAW: the lift fits its share of the registered route's settlement bound (512 host turns plus 512 per MiB of file, preparation and lift each at most half of it): under the browser turn's copy, item and fuel grants every real-world document lifts to the outcome single units reach in at most that many turns.
#[test]
fn real_world_documents_lift_within_half_of_their_route_turn_bound() {
    let ledger: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🌍️real-world/🔣️.json")).unwrap();
    let mut report = Vec::new();
    for case in ledger["documents"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let input = fixture_bytes!(format!("../../../../{}", case["path"].as_str().unwrap()));
        let retained = decode_pdf(input).unwrap();
        let (outcome, turns, stages) = controlled_lift_turns(&retained);
        let (single, ..) = controlled_lift_driven(&retained, 0);
        assert_eq!(outcome.map(typed), single.map(typed), "{name}: the accounted lift reaches the single-unit outcome");
        let bound = (512 + 512 * input.len().div_ceil(1 << 20)) / 2;
        report.push((name.to_owned(), turns));
        assert!(turns <= bound, "{name}: lifted in {turns} turns, above {bound}: {stages:?}");
    }
    println!("lift turns (name, turns): {report:?}");
}

/// 🧵️ The objects a value references directly.
fn direct_references(value: &crate::standards::v1_7::subsets::base::schema::snapshot::PdfObject, found: &mut Vec<(u32, u16)>) {
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfObject;
    match value {
        PdfObject::Ref(owner) => found.push((owner.num, owner.gen)),
        PdfObject::Array(items) => items.iter().for_each(|item| direct_references(item, found)),
        PdfObject::Dict(entries) | PdfObject::Stream { dict: entries, .. } => entries.iter().for_each(|entry| direct_references(&entry.value, found)),
        _ => {}
    }
}

/// 🧬️ LAW: a role stores only the generators of its inputs (its identity, the inherited font dictionary, every claimed font root), so the stored lists stay linear in roles plus fonts instead of roles times the fonts' object closures (the thesis stored about 500 identities in each of its 770 glyph programs, 11 MB of its 55 MB); and the objects derived from those generators are exactly the objects the independent reader lopdf reaches from them, so nothing a role consumed went missing.
#[test]
fn stored_role_dependencies_are_generators_and_derive_exactly_what_lopdf_reaches() {
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_reachable_objects;
    let ledger: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🌍️real-world/🔣️.json")).unwrap();
    let (mut checked, mut stored) = (0, 0);
    for case in ledger["documents"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let input = fixture_bytes!(format!("../../../../{}", case["path"].as_str().unwrap()));
        let retained = decode_pdf(input).unwrap();
        let Ok(snapshot) = controlled_lift(&retained) else { continue };
        let graph = GraphSource::new(&retained.objects);
        let bound = 2 + snapshot.fonts.len();
        for role in &snapshot.admitted_stream_roles {
            assert!(role.dependencies.len() <= bound, "{name}: role {:?} stores {} dependencies, above identity, font dictionary and {} fonts", role.identity.owner, role.dependencies.len(), snapshot.fonts.len());
            assert!(role.dependencies.iter().all(|dependency| graph.resolve(dependency).is_some()), "{name}: every stored generator resolves");
            let mut seeds = Vec::new();
            for generator in &role.dependencies {
                if generator.path.is_empty() { seeds.push((generator.owner.num, generator.owner.gen)); } else if let Some(value) = graph.resolve(generator) { direct_references(value, &mut seeds); }
            }
            let derived: std::collections::BTreeSet<(u32, u16)> = graph.closure_of(&role.dependencies).iter().filter(|input| input.path.is_empty()).map(|input| (input.owner.num, input.owner.gen)).collect();
            let reached = reference_reachable_objects(input, &seeds).unwrap();
            assert_eq!(derived.difference(&reached).collect::<Vec<_>>(), Vec::<&(u32, u16)>::new(), "{name}: role {:?} derives objects lopdf does not reach", role.identity.owner);
            assert_eq!(reached.difference(&derived).collect::<Vec<_>>(), Vec::<&(u32, u16)>::new(), "{name}: role {:?} misses objects lopdf reaches", role.identity.owner);
            stored += role.dependencies.len();
        }
        checked += snapshot.admitted_stream_roles.len();
    }
    assert!(checked > 100, "the ledger lifts hundreds of roles, not {checked}");
    println!("roles {checked}, stored dependencies {stored}");
}
