use super::*;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PageLaw {
    items: usize,
    item_bytes: usize,
    page_items: usize,
    page_slots: Vec<usize>,
    page_release_bytes: Vec<usize>,
    maximum_release_bytes: usize,
}

/// 🧮️ Keeps an edit in order across page boundaries and releases each exact physical backing independently.
#[test]
fn mutation_pages_keep_order_and_bound_each_backing_release() {
    type Mutation = [u8; 256];
    let law: PageLaw = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(law.page_items, PAGE_ITEMS);
    assert_eq!(law.item_bytes, size_of::<Mutation>());

    let mut pages = ArtifactMutationPages::<Mutation>::new(law.items).unwrap();
    let directory_bytes = pages.directory_bytes();
    assert_eq!(directory_bytes, ArtifactMutationPages::<Mutation>::directory_demand(law.items).capacity_bytes);
    let mut expected = 0;
    let mut admitted_bytes = 0;
    for slots in law.page_slots {
        let demand = pages.next_page_demand().unwrap();
        assert_eq!(demand.capacity_bytes, slots * law.item_bytes);
        let grant = RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: demand.copy_bytes,
            maximum_capacity_bytes: demand.capacity_bytes,
            maximum_release_bytes: 0,
            maximum_depth: demand.depth,
        };
        assert_eq!(pages.admit_page(RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant }).unwrap(), Default::default());
        assert!(pages.push([0; 256]).is_err());
        let receipt = pages.admit_page(grant).unwrap();
        assert_eq!(receipt.retained_capacity_bytes, demand.capacity_bytes);
        admitted_bytes += receipt.retained_capacity_bytes;
        for _ in 0..slots {
            pages.push([expected as u8; 256]).unwrap();
            expected += 1;
        }
    }
    assert_eq!(pages.iter().map(|item| usize::from(item[0])).collect::<Vec<_>>(), (0..law.items).collect::<Vec<_>>());
    assert!(admitted_bytes > law.maximum_release_bytes);
    assert_eq!(pages.next_page_demand(), None);

    for expected in 0..law.items {
        assert_eq!(pages.get(0).unwrap()[0], expected as u8);
        assert_eq!(pages.get(pages.len() - 1).unwrap()[0], (law.items - 1) as u8);
        let item = pages.pop_front().unwrap();
        pages.restore_front(item);
        assert_eq!(pages.pop_front().unwrap()[0], expected as u8);
    }
    let mut released_bytes = 0;
    for release in law.page_release_bytes.into_iter().chain([directory_bytes]) {
        assert_eq!(pages.empty_release_bytes(), release);
        assert!(release <= law.maximum_release_bytes);
        pages.release_empty_backing();
        released_bytes += release;
    }
    assert!(pages.backing_is_empty());
    assert_eq!(released_bytes, admitted_bytes + directory_bytes);
}
