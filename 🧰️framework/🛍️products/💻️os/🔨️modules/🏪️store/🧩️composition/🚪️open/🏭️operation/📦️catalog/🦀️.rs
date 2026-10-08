/// 🧮️ Original member catalog constructor and cancellation System allocation laws.

#[test]
fn member_catalog_constructor_birth_and_retirement_match_original_system_allocations() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for bounded in [false, true] {
        let (price, allocated, released) = crate::test_allocation::observe_backing(|| if bounded { bounded_artifact_store_owners_birth_bytes::<DemoSnapshot, DemoMutation>() } else { <DemoSnapshot as MemberStoreOwner<DemoMutation>>::member_store_owners_birth_bytes() });
        assert_eq!((allocated, released), (0, 0));
        let (owners, allocated, released) = crate::test_allocation::observe_backing(|| if bounded { bounded_artifact_store_owners::<DemoSnapshot, DemoMutation>() } else { demo_closable_store_owners() });
        let mut owners = owners;
        assert_eq!((allocated, released), (price, 0));
        let mut physical = 0;
        let mut turns = 0;
        while !owners.uninstalled_owners_terminal_is_empty() && turns < 1024 {
            let (demand, allocated, released) = crate::test_allocation::observe_backing(|| owners.next_close_byte_demand());
            assert_eq!((allocated, released), (0, 0));
            for row in fixture["cases"].as_array().unwrap().iter().filter(|row| !row["born"].as_bool().unwrap()) {
                let items = row["items"].as_u64().unwrap() as usize;
                if items == 1 && demand == 0 { continue; }
                let bytes = if items == 0 { demand } else { demand - 1 };
                let (_, allocated, released) = crate::test_allocation::observe_backing(|| owners.close_uninstalled_owners_step(items, bytes).unwrap());
                assert_eq!((allocated, released), (0, 0));
            }
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| owners.close_uninstalled_owners_step(1, demand).unwrap());
            assert_eq!(allocated, 0);
            match step { SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1); assert_eq!(released_bytes, released); assert!(released <= demand); }, SnapshotRetirementStep::Complete => { assert!(owners.uninstalled_owners_terminal_is_empty()); assert_eq!(released, 0); }, SnapshotRetirementStep::Blocked => panic!("original exact catalog grant must progress") }
            physical += released;
            turns += 1;
        }
        assert!(owners.uninstalled_owners_terminal_is_empty());
        assert_eq!(physical, price);
        let (_, allocated, released) = crate::test_allocation::observe_backing(|| drop(owners));
        assert_eq!((allocated, released), (0, 0));
        println!("[DEBUG] original member catalog bounded={bounded} pure-query=0 exact-birth={price} fundedphysical={physical} turns={turns} neutral0/onebelow unchanged terminalDrop=0");
    }
}

#[test]
fn member_catalog_initial_open_cancellation_pays_original_catalog_and_page_before_drop() {
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "original 雪".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "v1".into(), subset: "first".into() } };
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: 3 }).unwrap();
    pages.admit_page(OwnedSchemaDecodePage::try_from_slice(&[1, 97, 83]).unwrap()).unwrap();
    pages.seal().unwrap();
    let request = MemberOpenRequest::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), 1000, expected, None, pages, ActorId("actor 雪".into())).admit(1).unwrap_or_else(|_| panic!("admitted original request"));
    type Open = InitialMemberStoreOpen<RetainedTestMembers, DemoSnapshot, DemoMutation>;
    let (price, allocated, released) = crate::test_allocation::observe_backing(|| Open::begin_birth_bytes(&request).unwrap());
    assert_eq!((allocated, released), (0, 0));
    let (open, allocated, released) = crate::test_allocation::observe_backing(|| Open::begin(request).unwrap_or_else(|_| panic!("original funded begin")));
    let mut open = open;
    assert_eq!((allocated, released), (price, 0));
    let mut physical = 0;
    let mut turns = 0;
    while !ErasedSnapshotRetirement::terminal_is_empty(&open) && turns < 1024 {
        let (demand, allocated, released) = crate::test_allocation::observe_backing(|| ErasedSnapshotRetirement::next_close_byte_demand(&open));
        assert_eq!((allocated, released), (0, 0));
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| ErasedSnapshotRetirement::close_step(&mut open, 1, demand).unwrap());
        assert_eq!(allocated, 0, "cancellation cannot construct replacement owners");
        match step { SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1); assert_eq!(released_bytes, released); assert!(released <= demand); }, SnapshotRetirementStep::Complete => { assert!(ErasedSnapshotRetirement::terminal_is_empty(&open)); assert_eq!(released, 0); }, SnapshotRetirementStep::Blocked => panic!("original exact cancellation grant must progress") }
        physical += released;
        turns += 1;
    }
    assert!(ErasedSnapshotRetirement::terminal_is_empty(&open));
    let (final_bytes, allocated, released) = crate::test_allocation::observe_backing(|| MemberOpenOperation::terminal_drop_byte_demand(&open).expect("terminal original open declares its complete final allocation"));
    assert_eq!((allocated, released), (0, 0));
    assert_eq!(final_bytes, OWNED_SCHEMA_DECODE_PAGE_BYTES);
    let (_, allocated, released) = crate::test_allocation::observe_backing(|| drop(open));
    assert_eq!((allocated, released), (0, final_bytes), "original final page release must match its same-turn whole declared grant");
    physical += released;
    assert!(physical >= price);
    println!("[DEBUG] original member begin exact-birth={price} originalcatalog/page funded-close={physical} turns={turns} cancellation-birth=0 final-paid-page={final_bytes}");
}

