/// 🧮️ Original member catalog constructor and cancellation System allocation laws.

#[test]
fn member_catalog_constructor_birth_and_retirement_match_original_system_allocations() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for bounded in [false, true] {
        let (birth, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| if bounded { bounded_artifact_store_owners_birth_demand::<DemoSnapshot, DemoMutation>().unwrap() } else { semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new().member_store_owners_birth_demand().unwrap() });
        assert_eq!((allocated, released), (0, 0));
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: birth.capacity_bytes, maximum_depth: birth.depth, maximum_copy_bytes: 0, maximum_release_bytes: 0 };
        let ((owners, progress), allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| if bounded { bounded_artifact_store_owners::<DemoSnapshot, DemoMutation>(grant).unwrap_or_else(|_| panic!("funded bounded original sources")) } else { semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new().member_store_owners(grant).unwrap_or_else(|_| panic!("funded original sources")) });
        let mut owners = owners;
        assert_eq!((allocated, released), (birth.capacity_bytes, 0));
        assert_eq!(progress.retained_capacity_bytes, allocated);
        assert!(progress.fits(grant));
        assert!(!owners.constructor_is_complete());
        let mut born = allocated;
        let mut physical = 0;
        let mut turns = 0;
        while !owners.uninstalled_owners_terminal_is_empty() && turns < 1024 {
            let (demand, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| {
                let copy = owners.uninstalled_owners_demands(0).unwrap().copy_bytes;
                owners.uninstalled_owners_demands(copy).unwrap()
            });
            assert_eq!((allocated, released), (0, 0));
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            for row in fixture["cases"].as_array().unwrap().iter().filter(|row| !row["born"].as_bool().unwrap()) {
                let items = row["items"].as_u64().unwrap() as usize;
                let denied = if items == 0 { RetainedCloneGrant { maximum_items: 0, ..grant } }
                    else if demand.capacity_bytes > 0 { RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant } }
                    else if demand.release_bytes > 0 { RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant } }
                    else { continue };
                let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| owners.close_uninstalled_owners_step(denied).unwrap());
                assert_eq!(step.progress(), RetainedCloneProgress::default());
                assert_eq!((allocated, released), (0, 0));
                assert_eq!(owners.uninstalled_owners_demands(grant.maximum_copy_bytes).unwrap(), demand);
            }
            let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| owners.close_uninstalled_owners_step(grant).unwrap());
            let progress = step.progress();
            assert!(progress.fits(grant));
            assert_eq!((allocated, released), (progress.retained_capacity_bytes, progress.released_bytes));
            assert!(progress.copied_items > 0 || matches!(step, RetainedCloneStep::Complete(_)), "an exact current owner grant must advance");
            if matches!(step, RetainedCloneStep::Complete(_)) { assert!(owners.uninstalled_owners_terminal_is_empty()); }
            born += allocated;
            physical += released;
            turns += 1;
        }
        assert!(owners.uninstalled_owners_terminal_is_empty());
        assert_eq!(physical, born);
        let (_, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| drop(owners));
        assert_eq!((allocated, released), (0, 0));
        println!("[DEBUG] original member catalog bounded={bounded} sourcebirth={} fundedticketbirth={} fundedphysical={physical} turns={turns} neutral0/onebelow preserved terminalDrop=0", birth.capacity_bytes, born - birth.capacity_bytes);
    }
}

#[test]
fn member_catalog_initial_open_cancellation_pays_original_catalog_and_page_before_drop() {
    let (request, request_born, request_freed) = semio_framework_trace_memory_testing::observe_backing(|| {
        let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "original 雪".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "v1".into(), subset: "first".into() } };
        let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: 3 }).unwrap();
        pages.admit_page(OwnedSchemaDecodePage::try_from_slice(&[1, 97, 83]).unwrap()).unwrap();
        pages.seal().unwrap();
        MemberOpenRequest::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), 1000, expected, None, pages, ActorId("actor 雪".into())).admit(1).unwrap_or_else(|_| panic!("admitted original request"))
    });
    let outstanding = request_born.checked_sub(request_freed).expect("original request keeps exact live allocations");
    type Open = InitialMemberStoreOpen<RetainedTestMembers, DemoSnapshot, DemoMutation,semio_framework_os_store_testing_doubles::DemoMemberStoreProducer<DemoMutation>>;
    let (birth, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| Open::begin_birth_demand(&request,&semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()).unwrap());
    assert_eq!((allocated, released), (0, 0));
    let mut caller=OriginalMemberCaller::new();let mut admission=MemberOpenSourceAdmission::new(request);let policy=caller.policy;
    let((ready,receipt),allocated,released)=semio_framework_trace_memory_testing::observe_backing(||caller.run(policy,|parent,cx|admission.admit_source(parent,cx).unwrap()));assert!(!ready);assert_eq!((allocated,released),(0,0));assert_eq!(receipt,RetainedCloneProgress{copied_items:1,..Default::default()});
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: birth.capacity_bytes, maximum_depth: birth.depth, maximum_copy_bytes: 0, maximum_release_bytes: 0 };
    let(((open,raw),receipt),allocated,released)=semio_framework_trace_memory_testing::observe_backing(||caller.run(grant,|_parent,cx|{let(open,raw)=Open::begin(&mut admission,cx,grant,semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<DemoMutation>::new()).unwrap();semio_framework_os_store_member_open::receive_member_close(cx,Ok(RetainedCloneStep::Progress(raw))).unwrap();(open,raw)}));assert_eq!(raw,receipt);assert!(admission.terminal_is_empty());
    let mut open = open;
    assert_eq!((allocated, released), (birth.capacity_bytes, 0));
    let initial_born=outstanding+allocated;
    let(added,physical,turns)=retire_original_member_open(&mut open,&mut caller);
    let born=initial_born+added;
    assert_eq!(physical,born,"original request and catalog birth must all return under unchanged source policy");
    let (final_bytes, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| MemberOpenOperation::terminal_drop_byte_demand(&open).expect("terminal original open declares no remaining physical allocation"));
    assert_eq!((allocated, released, final_bytes), (0, 0, 0));
    let (_, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| drop(open));
    assert_eq!((allocated, released), (0, 0), "original page release is already paid in its exact current turn");
    caller.close();
    println!("[DEBUG] original member begin sourcebirth={} originalrequest={outstanding} allfundedbirth={born} exactfundedphysical={physical} turns={turns} terminalDrop=0", birth.capacity_bytes);
}

