//! 🧪 Checks the retained page tree against a nested PDF and a cyclic variation.

use super::*;
use crate::standards::v1_7::subsets::base::modules::lexer::parse_indirect_at;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfIndirectObject;

const PDF: &[u8] = include_bytes!("../../../../../🧪️tests/🧫️fixtures/🌳️page-tree/🧪️nested-inheritance.pdf");

pub(crate) fn objects() -> Vec<PdfIndirectObject> {
    (1..=13).map(|number| {
        let header = format!("\n{number} 0 obj\n");
        let offset = PDF.windows(header.len()).position(|bytes| bytes == header.as_bytes()).unwrap() + 1;
        let (id, value) = parse_indirect_at(PDF, offset).unwrap();
        PdfIndirectObject { id, value }
    }).collect()
}

fn reference(num: u32) -> ObjRef { ObjRef { num, gen: 0 } }

struct Borrowed<'a> {
    objects: &'a [PdfIndirectObject],
    reads: usize,
}

impl ObjectSource for Borrowed<'_> {
    fn get(&mut self, _: ObjRef) -> Option<PdfObject> { panic!("page traversal must not clone a retained source object"); }

    fn inspect(&mut self, reference: ObjRef, read: &mut dyn FnMut(Option<&PdfObject>)) {
        self.reads += 1;
        read(self.objects.get(reference.num as usize - 1).filter(|object| object.id == reference).map(|object| &object.value));
    }
}

fn grant(event: PageTreeEvent) -> CosGrant {
    CosGrant {
        work: 1,
        allocation_bytes: if let PageTreeEvent::NeedsAllocation { bytes } = event { bytes } else { 0 },
        release_bytes: if let PageTreeEvent::NeedsRelease { bytes } = event { bytes } else { 0 },
    }
}

fn account(retained: &mut usize, receipt: &PageTreeReceipt, grant: CosGrant) {
    assert!(receipt.work <= grant.work && receipt.allocated_bytes <= grant.allocation_bytes && receipt.released_bytes <= grant.release_bytes);
    *retained += receipt.allocated_bytes;
    *retained -= receipt.released_bytes;
}

fn close(cursor: &mut PageTreeCursor, mut retained: usize) {
    cursor.begin_close();
    assert_eq!(cursor.close_step(CosGrant::default()).work, 0);
    let mut allowance = grant(PageTreeEvent::More);
    for _ in 0..128 {
        let receipt = cursor.close_step(allowance);
        account(&mut retained, &receipt, allowance);
        let event = receipt.event.unwrap();
        if event == PageTreeEvent::Closed { assert_eq!(retained, 0); return; }
        allowance = grant(event);
    }
    panic!("page traversal retained index backing");
}

#[test]
fn visits_nested_pages_in_document_order_with_nearest_declarations() {
    for cycle in [false, true] {
        let mut objects = objects();
        if cycle {
            let PdfObject::Array(children) = &mut objects[7].value else { unreachable!() };
            children.extend([PdfObject::Ref(reference(2)), PdfObject::Ref(reference(6))]);
        }
        let mut source = Borrowed { objects: &objects, reads: 0 };
        let mut walker = PageTreeCursor::new(reference(2));
        let mut leaves = Vec::new();
        let mut resource_owners = Vec::new();
        let mut retained = 0;
        let mut allowance = grant(PageTreeEvent::More);
        for _ in 0..4096 {
            let reads = source.reads;
            let receipt = walker.advance(&mut source, allowance, false);
            assert!(source.reads - reads <= receipt.work);
            account(&mut retained, &receipt, allowance);
            let event = receipt.event.unwrap();
            match event {
                PageTreeEvent::Page(leaf) => leaves.push(leaf),
                PageTreeEvent::BranchResources(attribute) => resource_owners.push(attribute.owner.num),
                PageTreeEvent::Complete => break,
                _ => {}
            }
            allowance = grant(event);
        }
        assert_eq!(leaves.iter().map(|leaf| leaf.reference.num).collect::<Vec<_>>(), [6, 5, 3]);
        assert_eq!(resource_owners, [2, 4]);
        let owners = leaves.iter().map(|leaf| {
            let inherited = leaf.inheritance;
            [inherited.media_box, inherited.crop_box, inherited.rotate, inherited.resources].map(|entry| entry.map_or(0, |entry| entry.owner.num))
        }).collect::<Vec<_>>();
        assert_eq!(owners, [[6, 4, 6, 4], [2, 4, 4, 4], [2, 0, 0, 2]]);
        close(&mut walker, retained);
    }
}

#[test]
fn cancelled_page_walk_retires_the_visited_index_without_reading_more_nodes() {
    let objects = objects();
    let mut source = Borrowed { objects: &objects, reads: 0 };
    let mut walker = PageTreeCursor::new(reference(2));
    let mut retained = 0;
    let mut allowance = grant(PageTreeEvent::More);
    for _ in 0..512 {
        let receipt = walker.advance(&mut source, allowance, false);
        account(&mut retained, &receipt, allowance);
        let event = receipt.event.unwrap();
        if matches!(event, PageTreeEvent::BranchResources(_)) {
            let reads = source.reads;
            let cancelled = walker.advance(&mut source, allowance, true);
            assert_eq!(cancelled.event, Ok(PageTreeEvent::Cancelled));
            assert_eq!(cancelled.work + cancelled.allocated_bytes + cancelled.released_bytes, 0);
            assert_eq!(source.reads, reads);
            assert!(retained > 0);
            close(&mut walker, retained);
            return;
        }
        allowance = grant(event);
    }
    panic!("page tree did not reach its first branch");
}
