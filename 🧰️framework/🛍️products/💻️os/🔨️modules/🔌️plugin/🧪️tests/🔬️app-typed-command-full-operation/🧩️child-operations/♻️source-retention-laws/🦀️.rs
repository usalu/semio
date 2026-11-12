//! 🧩️ Independent child-group typed and paged source physical retention laws.

extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as pack;
extern crate semio_os_kernel as dsl;
use semio_framework_value_testing_allocator as _;
use semio_plugin_decl_emission::ChildEmit;
#[path="../../../../🧪️tests/🖥️test-app-mutations/🦀️.rs"]
mod test_app_mutation_fixture;

#[cfg(test)]
#[path="../🧫️fixture-json/🦀️.rs"]
mod fixtures;
#[cfg(test)]
#[path="../🏦️granted/🦀️.rs"]
mod granted;
#[cfg(test)]
#[path="../../../../../../../../🔨️modules/🌱️value/📝️text/📦️paged/🧪️tests/native-rig.rs"]
mod rig;
#[cfg(test)]
mod child_complete_group_candidate_tests{
        mod paged_encoder{include!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🛫️encoder/🦀️.rs");}
        mod paged_owner{include!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🫙️owner/🦀️.rs");}
        mod reader{include!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/📖️reader/🦀️.rs");}
        use semio_os_kernel::os_spr::operation_bytes::{OperationByteOutput};
        use semio_framework_value::{ErasedSnapshotRetirement,NativeEncodeControl};
        #[test]
        fn child_complete_actual_authored_typed_source_event_observer_matches_system_backing(){
            let(mut bytes,allocated)=semio_framework_trace::observe_heap_allocations_on_this_thread(||vec![b'x';8194]);assert_eq!((allocated.requested_bytes,allocated.released_bytes),(8194,0));assert!(!allocated.overflowed);
            let(_,shortened)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bytes.truncate(4096));assert_eq!((shortened.requested_bytes,shortened.released_bytes),(0,0));assert_eq!(bytes.capacity(),8194);
            let(_,resized)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bytes.shrink_to_fit());assert_eq!(bytes.capacity(),4096);assert_eq!((resized.requested_bytes,resized.released_bytes,resized.largest_release_bytes),(4096,8194,8194));assert!(!resized.overflowed);
            let(_,released)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(bytes));assert_eq!((released.requested_bytes,released.released_bytes,released.largest_release_bytes),(0,4096,4096));
            let(pages,allocated)=semio_framework_trace::observe_heap_allocations_on_this_thread(||(vec![0u8;4096],vec![0u8;4096]));assert_eq!((allocated.requested_bytes,allocated.released_bytes),(8192,0));let(inner,outer)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(pages)).1);assert_eq!(inner,outer);assert_eq!((outer.requested_bytes,outer.released_bytes,outer.largest_release_bytes),(0,8192,4096));assert!(!outer.overflowed);
        }
        #[test]
        fn child_complete_actual_authored_typed_source_physical8194_grant4096(){
            use semio_framework_value::retirement::controlled::ControlledRetirement;
            use semio_framework_value::retained_clone::RetainedCloneGrant;
            let fixture:serde_json::Value=crate::fixtures::fixture_json("♻️typed-source-physical");
            let backing_bytes=fixture["textBytes"].as_u64().unwrap() as usize;
            let work_bytes=fixture["maximumBytes"].as_u64().unwrap() as usize;
            let admission=fixture["maximumAllocationBytes"].as_u64().unwrap() as usize;
            assert_eq!((backing_bytes,work_bytes,admission),(8194,4096,65536));assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["textByte"],b'x');
            let mutation=crate::test_app_mutation_fixture::SetLabel{value:"x".repeat(backing_bytes)};
            let backing=mutation.value.capacity();assert_eq!(backing,backing_bytes);
            let pointer=mutation.value.as_ptr();
            let (mut owner,construction)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ControlledRetirement::retain(mutation.value));
            assert_eq!(owner.original().unwrap().as_ptr(),pointer);assert_eq!(construction.released_bytes,0);assert!(!construction.overflowed);
            let (step,zero)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::granted::step(&mut owner,Default::default()).unwrap());
            assert_eq!(step.progress(),Default::default());assert_eq!((zero.requested_bytes,zero.released_bytes),(0,0));assert!(!owner.terminal_is_empty());
            let ((),birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{for _ in 0..64{let demand=owner.next_demand(work_bytes).unwrap();if demand.release_bytes>0{break}crate::granted::step(&mut owner,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:0,maximum_depth:demand.depth.max(1)}).unwrap();}});
            assert!(!birth.overflowed&&birth.released_bytes==0);
            assert_eq!(owner.next_demand(work_bytes).unwrap().release_bytes,backing,"a String retires release-only once its cursor is born: its bytes are never logically drained");
            for grant in fixture["physicalRelease"]["deniedBytes"].as_array().unwrap(){
                let bytes=grant.as_u64().unwrap() as usize;let demand=owner.next_demand(work_bytes).unwrap();assert_eq!(demand.release_bytes,backing);
                let (step,observed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::granted::step(&mut owner,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:bytes,maximum_depth:demand.depth.max(1)}).unwrap());
                assert_eq!(step.progress(),Default::default());assert!(!owner.terminal_is_empty());assert_eq!(owner.next_demand(work_bytes).unwrap().release_bytes,backing);assert_eq!((observed.requested_bytes,observed.released_bytes),(0,0));assert!(!observed.overflowed);
            }
            let demand=owner.next_demand(work_bytes).unwrap();
            assert_eq!(demand.release_bytes,fixture["physicalRelease"]["demandBytes"].as_u64().unwrap() as usize);assert!(demand.release_bytes<=admission);
            let (step,released)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::granted::step(&mut owner,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)}).unwrap());
            let reported=step.progress().released_bytes;assert_eq!(reported,fixture["physicalRelease"]["reportedBytes"].as_u64().unwrap() as usize);
            assert_eq!((released.requested_bytes,released.released_bytes,released.largest_release_bytes),(0,reported,reported));assert!(!released.overflowed);assert_eq!(reported,backing);
            let ((),terminal)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{for _ in 0..64{if owner.terminal_is_empty(){break}crate::granted::step(&mut owner,crate::granted::PAGE).unwrap();}});
            assert!(owner.terminal_is_empty());assert_eq!(terminal.requested_bytes,0);assert!(!terminal.overflowed);
            let ((),scaffold)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));
            assert_eq!(scaffold.requested_bytes,0);assert!(!scaffold.overflowed);
            assert_eq!(construction.requested_bytes+birth.requested_bytes,terminal.released_bytes+scaffold.released_bytes);
        }
        #[test]
        fn child_complete_actual_authored_paged_source8194_frame8202_physical4096(){
            use semio_framework_value_paged_text::paged_text::TextReadSource;
            use semio_framework_value::{NativeEncodeControl,ErasedSnapshotRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneStep}};
            use semio_os_kernel::os_pack::codec::PackEncodeOptions;
            struct Original;impl TextReadSource for Original{fn byte_len(&self)->usize{8194}fn byte_at(&self,index:usize)->Option<u8>{(index<8194).then_some(b'x')}}
            struct Observed<'a>{inner:&'a mut dyn OperationByteOutput,accepted:&'a std::sync::atomic::AtomicUsize}
            impl OperationByteOutput for Observed<'_>{fn write_bytes(&mut self,bytes:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),semio_os_kernel::os_pack::PackRefusal>{for byte in bytes{self.inner.write_bytes(&[*byte],control)?;self.accepted.fetch_add(1,std::sync::atomic::Ordering::Relaxed);}Ok(())}}
            fn settle(owner:&mut dyn ErasedSnapshotRetirement)->usize{let mut total=0;for _ in 0..100000{let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::granted::step(owner,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_release_bytes:4096,maximum_depth:64,..Default::default()}).unwrap());let progress=match step{RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress};assert!(!event.overflowed);assert_eq!(event.requested_bytes,0);assert_eq!(event.released_bytes,progress.released_bytes);assert!(progress.copied_items<=1&&event.released_bytes<=4096&&event.largest_release_bytes<=4096);total+=event.released_bytes;if owner.terminal_is_empty(){break}}assert!(owner.terminal_is_empty());total}
            let fixture:serde_json::Value=crate::fixtures::fixture_json("🌱️authored-paged-source");assert_eq!(fixture["textBytes"],8194);assert_eq!(fixture["frameBytes"],8202);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["metadataWindowBytes"],512);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
            for mode in fixture["modes"].as_array().unwrap().iter().map(|mode|mode.as_str().unwrap()){
                let mut source=crate::test_app_mutation_fixture::SetLabel::source_owner();let accepted=std::sync::atomic::AtomicUsize::new(0);let mut allow=|_|mode!="interior256"||accepted.load(std::sync::atomic::Ordering::Relaxed)<256;let mut control=NativeEncodeControl::new(if mode=="initial4096"{4096}else{65536},&mut allow);
                let(result,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||paged_owner::encode_paged(&Original,control.maximum_bytes()).and_then(|paged|{source.value=Some(paged);control.charge(source.allocated_bytes())}));assert!(!event.overflowed);assert_eq!(event.requested_bytes-event.released_bytes,source.allocated_bytes());assert_eq!(source.allocated_bytes(),control.owned_bytes());let original=source.allocated_bytes();
                if mode=="initial4096"{assert!(result.is_err());assert!(source.value_view().is_err());assert_eq!(settle(&mut source),original);continue}result.unwrap();let original_byte=source.value_view().unwrap().byte_at(0);assert_eq!(original_byte,Some(b'x'));
                let(mut capsule,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.retained_pack_operation());assert_eq!((birth.requested_bytes,birth.released_bytes),(0,0));assert_eq!(capsule.variant_identity().unwrap(),("set-label",1));
                if mode=="complete"{let mut measurement=semio_os_kernel::os_spr::operation_bytes::OperationByteMeasurement::new(8202);let window=control.owned_bytes()+512;let(result,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||control.scoped_maximum(window,|control|Ok::<_,semio_framework_value::ValueError>(semio_os_kernel::variants_binary::encode_op_into(&mut capsule,&PackEncodeOptions::default(),&mut measurement,control))));result.unwrap().unwrap();assert_eq!(measurement.exact_length().unwrap(),8202);assert_eq!((event.requested_bytes,event.released_bytes),(0,0));}
                let mut child=paged_owner::PagedChildOwner::empty();let mut options=PackEncodeOptions::default();options.limits.max_file_len=if mode=="file-one-short"{8201}else{8202};if mode=="depth-zero"{options.limits.max_depth=0;}
                for(field,text)in[(reader::ChildGroupText::Owner,"owner"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,"child.empty")]{child.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}
                let result=child.produce_owned_operation(8202,&options,&mut control,|options,output,control|semio_os_kernel::variants_binary::encode_op_into(&mut capsule,options,&mut Observed{inner:output,accepted:&accepted},control));
                match result{Ok(mut receipt)=>{assert_eq!(mode,"complete");receipt.read_first_schema_from_source(&"plugin.testkit.document",&mut control).unwrap();for terminology in semio_ui_locale::Terminology::ALL{for locale in semio_ui_locale::Locale::ALL{receipt.produce_label(terminology,locale,Original,paged_owner::authored(locale).0,paged_owner::authored(locale).1,&mut control).unwrap();}}receipt.commit(&mut control).unwrap();},Err(_)=>assert_ne!(mode,"complete")}
                assert_eq!(source.allocated_bytes(),original);assert_eq!(source.value_view().unwrap().byte_at(0),original_byte);assert!(control.owned_bytes()<=65536);
                if mode=="complete"{assert_eq!(child.operations.len(),1);assert_eq!(child.operations[0].len(),8202);for(index,byte)in[1,1,0,1,0,7,130,64].into_iter().enumerate(){assert_eq!(child.operations[0].byte_at(index),Some(byte));}for index in 8..8202{assert_eq!(child.operations[0].byte_at(index),Some(b'x'));}let view=child.view().unwrap();assert_eq!(view.schema.len(),"plugin.testkit.document".len());for terminology in semio_ui_locale::Terminology::ALL{for(locale_index,locale)in semio_ui_locale::Locale::ALL.into_iter().enumerate(){let label=view.labels.text(0,terminology,locale).unwrap();let prefix=fixture["labelPrefixes"][locale_index].as_str().unwrap();let suffix=fixture["labelSuffixes"][locale_index].as_str().unwrap();assert_eq!(label.len(),prefix.len()+8194+suffix.len());for(index,byte)in prefix.bytes().chain(std::iter::repeat_n(b'x',8194)).chain(suffix.bytes()).enumerate(){assert_eq!(label.byte(index).unwrap(),byte);}}}}
                else{assert_eq!(child.operations.len(),0);if mode=="interior256"{assert_eq!(accepted.load(std::sync::atomic::Ordering::Relaxed),256);assert_eq!(child.retained_partial_source().unwrap().len(),256);}}
                let scratch=capsule.allocated_bytes();let mut released=0;for _ in 0..32768{let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::granted::retire(crate::granted::PAGE,|effect|capsule.retire_one(effect)).unwrap());assert_eq!(event.requested_bytes,0);assert_eq!(event.released_bytes,step.2);assert!(step.1<=1&&event.released_bytes<=4096&&event.largest_release_bytes<=4096);released+=event.released_bytes;if !step.0{break}}assert_eq!(released,scratch);drop(capsule);let child_bytes=child.allocated_bytes();assert_eq!(settle(&mut child),child_bytes);assert_eq!(settle(&mut source),original);
                let(_,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{drop(child);drop(source)});assert_eq!((event.requested_bytes,event.released_bytes),(0,0));
            }
        }
    }
