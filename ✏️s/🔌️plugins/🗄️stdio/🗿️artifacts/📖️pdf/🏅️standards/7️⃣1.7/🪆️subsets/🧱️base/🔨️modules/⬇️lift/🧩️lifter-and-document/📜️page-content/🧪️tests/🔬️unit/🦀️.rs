use crate::standards::v1_7::subsets::base::modules::lift::{project_content_bytes, PageContentBarrier, PageContentEvent, PageContentReceipt, PageProgramCursor};
use crate::standards::v1_7::subsets::base::schema::graph_source::ObjectSource;
use crate::standards::v1_7::subsets::base::io::foreign_artifacts::NativePdfArtifactResources;
use crate::standards::v1_7::subsets::base::modules::content::{parse_content, NoFonts};
use crate::standards::v1_7::subsets::base::modules::lexer::CosGrant;
use crate::standards::v1_7::subsets::base::schema::snapshot::{stream_bytes, ObjRef, PdfDictEntry, PdfObject, PdfOp};

struct BorrowedContent(Vec<PdfObject>);

impl ObjectSource for BorrowedContent {
    fn get(&mut self, _: ObjRef) -> Option<PdfObject> { panic!("content payload should be borrowed") }

    fn inspect(&mut self, reference: ObjRef, read: &mut dyn FnMut(Option<&PdfObject>)) {
        read(self.0.get(reference.num as usize));
    }
}

fn stream(data: &[u8]) -> PdfObject { PdfObject::Stream { dict: Vec::new(), data: stream_bytes(&(data)), filters: Vec::new() } }

#[test]
fn single_page_stream_reaches_the_parser_as_its_exact_octets() {
    let mut objects = BorrowedContent(vec![stream(b"BT (Hello PDF) Tj ET")]);
    let contents = PdfObject::Ref(ObjRef { num: 0, gen: 0 });
    project_content_bytes(&mut objects, Some(&contents), |data| {
        assert_eq!(data, b"BT (Hello PDF) Tj ET");
    });
}

#[test]
fn referenced_content_arrays_join_mixed_members_in_original_order() {
    let first = ObjRef { num: 1, gen: 0 };
    let third = ObjRef { num: 2, gen: 0 };
    let mut objects = BorrowedContent(vec![
        PdfObject::Array(vec![PdfObject::Ref(first), stream(b"30 m 75"), PdfObject::Ref(third)]),
        stream(b"20"),
        stream(b"90 l S"),
    ]);
    let contents = PdfObject::Ref(ObjRef { num: 0, gen: 0 });
    project_content_bytes(&mut objects, Some(&contents), |data| {
        assert_eq!(data, b"20\n30 m 75\n90 l S");
    });
}

#[derive(Default)]
struct Account {
    acquired: usize,
    released: usize,
    allocation: usize,
    release: usize,
}

impl Account {
    fn grant(&self) -> CosGrant {
        CosGrant { work: 1, allocation_bytes: self.allocation, release_bytes: self.release }
    }

    fn record(&mut self, turn: PageContentReceipt) -> PageContentEvent {
        assert!(turn.work <= 1);
        assert!(turn.allocated_bytes <= self.allocation);
        assert!(turn.released_bytes <= self.release);
        self.acquired += turn.allocated_bytes;
        self.released += turn.released_bytes;
        let event = turn.event.unwrap();
        match event {
            PageContentEvent::NeedsAllocation { bytes } => self.allocation = bytes,
            PageContentEvent::NeedsRelease { bytes } => self.release = bytes,
            _ => { self.allocation = 0; self.release = 0; }
        }
        event
    }

    fn retire(&mut self, cursor: &mut PageProgramCursor) {
        cursor.begin_close();
        for _ in 0..10_000 {
            if self.record(cursor.close_step(self.grant())) == PageContentEvent::Closed { return; }
        }
        panic!("page program did not release all backing");
    }
}

fn reference(num: u32) -> ObjRef { ObjRef { num, gen: 0 } }

fn fixture() -> (&'static str, &'static str) {
    include_str!("🧪️page-projection.txt").split_once("\n%% STREAM\n").unwrap()
}

fn page(contents: PdfObject) -> PdfObject {
    PdfObject::Dict(vec![
        PdfDictEntry::new("Type", PdfObject::Name("Page".into())),
        PdfDictEntry::new("Contents", contents),
    ])
}

fn split_page(left: &[u8], right: &[u8]) -> BorrowedContent {
    BorrowedContent(vec![
        page(PdfObject::Ref(reference(1))),
        PdfObject::Array(vec![PdfObject::Ref(reference(2)), PdfObject::Ref(reference(3))]),
        stream(left),
        stream(right),
    ])
}

fn project(source: &mut BorrowedContent) -> (Vec<PdfOp>, Account) {
    let mut cursor = PageProgramCursor::new(reference(0));
    let mut account = Account::default();
    assert!(cursor.take_program().is_none());
    for _ in 0..10_000 {
        let event = account.record(cursor.advance(source, account.grant(), false));
        if event == PageContentEvent::Complete {
            let commands = cursor.take_program().unwrap();
            assert!(cursor.take_program().is_none());
            return (commands, account);
        }
        assert!(matches!(event, PageContentEvent::More | PageContentEvent::NeedsAllocation { .. } | PageContentEvent::NeedsRelease { .. }));
    }
    panic!("controlled page program did not finish");
}

#[test]
fn controlled_page_projection_matches_reference_across_borrowed_and_split_streams() {
    let (left, right) = fixture();
    let joined = format!("{left}\n{right}");
    let mut split = split_page(left.as_bytes(), right.as_bytes());
    let mut single = BorrowedContent(vec![page(PdfObject::Ref(reference(1))), stream(joined.as_bytes())]);
    let (split_ops, split_account) = project(&mut split);
    let (single_ops, single_account) = project(&mut single);
    let mut resources = NativePdfArtifactResources::from_objects(&[]);
    let reference_ops = parse_content(joined.as_bytes(), &NoFonts, &mut resources).unwrap();
    assert_eq!(split_ops, reference_ops);
    assert_eq!(single_ops, reference_ops);
    assert!(split_ops.iter().any(|op| matches!(op, PdfOp::MoveTo { x: 20.0, y: 10.0 })));
    assert!(split_ops.iter().any(|op| matches!(op, PdfOp::SetDash { array, phase } if array == &[6., 2.] && *phase == 1.)));
    assert_eq!(split_account.acquired, single_account.acquired, "paged stream octets are copied once into the joined backing whether the page holds one stream or two");
    assert_eq!(split_account.released, single_account.released);
    assert!(single_account.acquired >= joined.len());
    let transferred = split_ops.capacity() * std::mem::size_of::<PdfOp>() + split_ops.iter().map(|op| match op {
        PdfOp::SetDash { array, .. } | PdfOp::SetFillColor { components: array } => array.capacity() * std::mem::size_of::<f64>(),
        PdfOp::SetFillColorSpace { name } => name.capacity(),
        _ => 0,
    }).sum::<usize>();
    assert_eq!(split_account.acquired, split_account.released + transferred);
}

#[test]
fn controlled_page_projection_releases_split_and_migration_backing_on_cancellation() {
    let (left, right) = fixture();
    for stop in 0..512 {
        let mut graph = split_page(left.as_bytes(), right.as_bytes());
        let mut cursor = PageProgramCursor::new(reference(0));
        let mut account = Account::default();
        for _ in 0..stop {
            if account.record(cursor.advance(&mut graph, account.grant(), false)) == PageContentEvent::Complete { break; }
        }
        assert_eq!(account.record(cursor.advance(&mut graph, account.grant(), true)), PageContentEvent::Cancelled);
        assert!(cursor.take_program().is_none());
        account.retire(&mut cursor);
        assert_eq!(account.acquired, account.released, "stop at page-content turn {stop}");
    }
}

#[test]
fn controlled_page_projection_retires_partial_program_before_cold_resource_fallback() {
    let mut graph = split_page(b"q 17 w 20 30 m", b"/AuthoredFont 12 Tf (text) Tj Q");
    let mut cursor = PageProgramCursor::new(reference(0));
    let mut account = Account::default();
    for _ in 0..10_000 {
        let event = account.record(cursor.advance(&mut graph, account.grant(), false));
        if let PageContentEvent::Deferred(reason) = event {
            assert_eq!(reason, PageContentBarrier::ResourceContext);
            assert!(cursor.take_program().is_none());
            assert_eq!(account.acquired, account.released);
            return;
        }
        assert!(matches!(event, PageContentEvent::More | PageContentEvent::NeedsAllocation { .. } | PageContentEvent::NeedsRelease { .. }));
    }
    panic!("font-bound page did not reach its cold projection barrier");
}
