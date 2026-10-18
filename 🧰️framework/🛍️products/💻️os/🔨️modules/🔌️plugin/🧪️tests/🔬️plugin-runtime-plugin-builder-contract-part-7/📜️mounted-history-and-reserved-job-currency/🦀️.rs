/// 📜️ Mounted history and reserved job currency: the checkpoint restart owner, mounted command history pages, the private child frame and the spare backing of a reserved job currency.

    #[test]
    fn checkpoint_restart_mode_requires_its_exact_concrete_factory_owner() {
        let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/⏳️completion/🔣️.json"))).unwrap();
        assert_eq!(<TestCommand as ::protocol::OpBinary>::TOOL_JOB_IDS, fixture["restartAuthority"]["generatedToolIds"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect::<Vec<_>>());
        assert_eq!(TestApp::<false>::bounded_first_step_tool_proofs().len() as u64, fixture["restartAuthority"]["defaultProofs"].as_u64().unwrap());
        assert_eq!(TestApp::<true>::bounded_first_step_tool_proofs().len() as u64, fixture["restartAuthority"]["retainedProofs"].as_u64().unwrap());
        assert_ne!(ToolOwnerWitness::of::<TestApp<false>>(), ToolOwnerWitness::of::<TestApp<true>>());
        assert_eq!(std::any::TypeId::of::<<TestRestartFactory<true, 2> as ArtifactOwnedToolJobFactory>::Owner>(), std::any::TypeId::of::<TestApp<true>>());
        assert_eq!(<TestRestartFactory<true, 2> as ArtifactOwnedToolJobFactory>::TOOL_IDS, &[fixture["restartAuthority"]["tool"].as_str().unwrap()]);
    }
    #[semio_framework_async_macros::async_test]
    async fn mounted_command_history_original_pages_retain_exact_app_close_authority() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = Box::pin(contract_app_raw(mounted_policy, &mut identity)).await;
        semio_framework_plugin_tests_02::child_test_hosts::child_unit_2::test_mounted_command_history_pages(&mut app);
    }
    #[semio_framework_async_macros::async_test]
    async fn mounted_command_history_document_replacement_keeps_original_held_rows() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = Box::pin(contract_app_raw(mounted_policy, &mut identity)).await;
        app.stores.test_store_mut().await.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 1 })], transaction: None }, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.unwrap();
        semio_framework_plugin_tests_02::child_test_hosts::child_unit_2::test_mounted_command_history_replacement(&mut app);
    }
    #[semio_framework_async_macros::async_test]
    async fn mounted_private_child_frame_retains_exact_admission_and_close_authority() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = Box::pin(contract_app_raw(mounted_policy, &mut identity)).await;
        semio_framework_plugin_tests_02::publication_mounted_test_hosts::publication_mounted_unit_1::test_mounted_private_child_frame(&mut app);
        semio_framework_plugin_testing_laws::artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

/// 👷️ Original reserved route returns physical backing separately from logical payload work.
#[test]
fn original_reserved_job_currency_law_retains_spare_backing_until_exact_release(){
 use semio_framework_plugin_job::reserved_jobs::FrameworkSetSelectionModeJob;
 use semio_framework_job::{InteractiveJob,InteractiveJobCloseStep};
 use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/👷️reserved/🔣️.json")).unwrap();
 let source:RetainedCloneGrant=serde_json::from_value(law["caller"]["source"].clone()).unwrap();let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(law["caller"]["operation"].as_u64().unwrap()),semio_framework_job::Generation(law["caller"]["generation"].as_u64().unwrap()),source,||Some(1000),law["caller"]["deadline"].as_u64().unwrap(),law["caller"]["fuel"].as_u64().unwrap());assert_eq!(law["caller"]["clock"],1000);
 let (raw,birth)=observe(||{let mut raw=Vec::with_capacity(law["backingBytes"].as_u64().unwrap()as usize);raw.resize(law["payloadBytes"].as_u64().unwrap()as usize,0x72);raw});
 let pointer=raw.as_ptr();let capacity=raw.capacity();assert_eq!((birth.requested_bytes,birth.released_bytes),(capacity,0));
 let mut job=FrameworkSetSelectionModeJob::new(raw,1);job.begin_close();
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:capacity,maximum_depth:1};
 for refused in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant}]{let(step,event)=observe(||caller.turn(refused,|cx|job.close_step(cx)).unwrap());assert_eq!(step,InteractiveJobCloseStep::Pending{progress:Default::default()});assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert_eq!(job.raw.as_ptr(),pointer);assert_eq!(job.raw.len(),8193);}
 let(step,event)=observe(||caller.turn(RetainedCloneGrant{maximum_depth:0,..grant},|cx|job.close_step(cx)));assert!(matches!(step,Err(ref failure) if failure.primary_value().is_some_and(|cause|cause.kind==semio_framework_value::ValueRefusalKind::DepthLimit)));assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert_eq!(job.raw.as_ptr(),pointer);
 for expected in law["turns"].as_array().unwrap().iter().take(3){let(step,event)=observe(||caller.turn(grant,|cx|job.close_step(cx)).unwrap());let progress=step.progress();assert_eq!(progress.copied_items,expected["copiedItems"].as_u64().unwrap()as usize);assert_eq!(progress.copied_bytes,expected["copiedBytes"].as_u64().unwrap()as usize);assert_eq!(progress.retained_capacity_bytes,0);assert_eq!(progress.released_bytes,0);assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert_eq!(job.raw.capacity(),capacity);assert!(!job.terminal_is_empty());}
 let(step,event)=observe(||caller.turn(RetainedCloneGrant{maximum_release_bytes:capacity-1,..grant},|cx|job.close_step(cx)).unwrap());assert_eq!(step,InteractiveJobCloseStep::Pending{progress:Default::default()});assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert_eq!(job.raw.as_ptr(),pointer);assert_eq!(job.raw.capacity(),capacity);
 let(step,event)=observe(||caller.turn(grant,|cx|job.close_step(cx)).unwrap());assert_eq!(step,InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()}});assert_eq!((event.requested_bytes,event.released_bytes),(0,capacity));assert_eq!(job.raw.capacity(),0);assert!(job.terminal_is_empty());
 let(step,event)=observe(||caller.turn(grant,|cx|job.close_step(cx)).unwrap());assert_eq!(step,InteractiveJobCloseStep::Complete{progress:Default::default()});assert_eq!((event.requested_bytes,event.released_bytes),(0,0));
 caller.retire(source);
}
