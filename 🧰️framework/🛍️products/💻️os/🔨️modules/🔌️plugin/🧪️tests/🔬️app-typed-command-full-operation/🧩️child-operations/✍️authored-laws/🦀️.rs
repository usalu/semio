//! 🧩️ Independent child-group authored label and operation producer laws.

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
        fn child_complete_actual_authored_label_producer_preserves_all_axes_and_retained_source(){
            use semio_framework_value_paged_text::paged_text::{PagedText,InlineTextBuffer};
            use semio_framework_value::{NativeEncodeControl,ErasedSnapshotRetirement,RetainedCloneGrant,RetainedCloneStep};
            let fixture:serde_json::Value=crate::fixtures::fixture_json("✍️authored-label");assert_eq!(fixture["englishBytes"],8194);assert_eq!(fixture["germanBytes"],8205);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["initialAllocationBytes"],4096);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
            fn drain<const N:usize>(paged:&mut PagedText<N>,inline:&mut InlineTextBuffer)->usize{if !inline.terminal_is_empty(){assert!(inline.close_one(1));}let mut released=0;for _ in 0..8205+128{match crate::granted::step(paged,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:64}).unwrap(){RetainedCloneStep::Complete(_)=>break,RetainedCloneStep::Progress(progress)=>{let released_items=progress.copied_items;let released_bytes=progress.released_bytes;assert!(progress.copied_bytes<=4096);assert_eq!(progress.retained_capacity_bytes,0);assert!(released_items<=1&&released_bytes<=4096);released+=released_bytes;}}}assert!(paged.terminal_is_empty()&&inline.terminal_is_empty());released}
            for large in[false,true]{let operation=crate::test_app_mutation_fixture::SetLabel{value:if large{std::iter::repeat_n('x',8181).collect()}else{fixture["shortValue"].as_str().unwrap().to_owned()}};for _terminology in semio_ui_locale::Terminology::ALL{for locale in semio_ui_locale::Locale::ALL{let(prefix,suffix)=match locale{semio_ui_locale::Locale::En=>(fixture["englishPrefix"].as_str().unwrap(),fixture["englishSuffix"].as_str().unwrap()),semio_ui_locale::Locale::De=>(fixture["germanPrefix"].as_str().unwrap(),fixture["germanSuffix"].as_str().unwrap())};
                let(mut paged,mut inline)=paged_owner::encode_label(&operation.value,paged_owner::leaked(prefix),paged_owner::leaked(suffix),65536).unwrap();let expected=prefix.as_bytes().iter().copied().chain(operation.value.as_bytes().iter().copied()).chain(suffix.as_bytes().iter().copied());
                if large{assert!(paged.is_some()&&inline.is_none());assert!(paged.as_ref().unwrap().borrow().unwrap().bytes().eq(expected));assert!(paged_owner::encode_label(&operation.value,paged_owner::leaked(prefix),paged_owner::leaked(suffix),4096).is_err(),"a 4096-byte segment policy refuses the 8194-byte label");}else{assert!(paged.is_none()&&inline.is_some());assert!(inline.as_ref().unwrap().borrow().unwrap().bytes().eq(expected));}
                if let Some(mut text)=paged.take(){let allocated=text.allocated_bytes();assert_eq!(drain(&mut text,&mut InlineTextBuffer::empty()),allocated);}if let Some(mut cell)=inline.take(){assert!(cell.close_one(1));assert!(cell.terminal_is_empty());}
            }}}
        }
        #[test]
        fn child_complete_actual_authored_label_receipt_joins_original8194_source_and_prefix(){
            use semio_framework_value::NativeEncodeControl;
            use semio_os_kernel::{os_pack::codec::PackEncodeOptions,os_spr::operation_bytes::OperationByteOutput};
            let fixture:serde_json::Value=crate::fixtures::fixture_json("🧾️label-receipt");assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
            fn encode(output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),semio_os_kernel::os_spr::ProtocolError>{output.write_bytes(b"17",control)?;for _ in 0..32{output.write_bytes(&[b' ';256],control)?;}Ok(())}
            let operation=crate::test_app_mutation_fixture::SetLabel{value:fixture["typedValue"].as_str().unwrap().to_owned()};for refused in[false,true]{let mut options=PackEncodeOptions::default();if refused{options.limits.max_segment_len=11;}let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);for(field,text)in[(reader::ChildGroupText::Owner,"root"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,"child.empty")]{owner.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}let mut receipt=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|encode(output,control)).unwrap();receipt.read_first_schema_from_source(&if refused{"schema"}else{fixture["acceptedSchema"].as_str().unwrap()},&mut control).unwrap();let mut label_failed=false;for terminology in semio_ui_locale::Terminology::ALL{for locale in semio_ui_locale::Locale::ALL{let result=receipt.produce_label(terminology,locale,operation.value.as_str(),paged_owner::authored(locale).0,paged_owner::authored(locale).1,&mut control);if refused{assert!(result.is_err());label_failed=true;break;}result.unwrap();}if label_failed{break;}}if refused{assert!(receipt.commit(&mut control).is_err());assert_eq!(owner.operations.len(),0);let view=owner.view().unwrap();assert_eq!(view.labels.len(),0);assert_eq!(view.schema.len(),11);}else{receipt.commit(&mut control).unwrap();assert_eq!(owner.operations.len(),1);assert!(owner.operations[0].iter().eq(b"17".iter().copied().chain(std::iter::repeat_n(b' ',8192))));let view=owner.view().unwrap();assert_eq!(view.labels.len(),1);for terminology in semio_ui_locale::Terminology::ALL{for locale in semio_ui_locale::Locale::ALL{let expected=match locale{semio_ui_locale::Locale::En=>"Set label to target",semio_ui_locale::Locale::De=>"Beschriftung auf target setzen"};let text=view.labels.text(0,terminology,locale).unwrap();assert_eq!(text.len(),expected.len());for(index,byte)in expected.as_bytes().iter().enumerate(){assert_eq!(text.byte(index).unwrap(),*byte)}}}}
                assert_eq!(owner.allocated_bytes(),control.owned_bytes());let allocated=owner.allocated_bytes();assert_eq!(crate::granted::drain_retained(&mut owner),allocated);
            }
        }
        #[test]
        fn child_complete_actual_authored_operation_producer_joins_original_projection_and_retained_receipt(){
            use semio_framework_value::{ErasedSnapshotRetirement,NativeEncodeControl};
            use semio_os_kernel::os_pack::codec::PackEncodeOptions;
            use std::sync::atomic::{AtomicUsize,Ordering};
            struct Observed<'a>{inner:&'a mut dyn OperationByteOutput,accepted:&'a AtomicUsize}
            impl OperationByteOutput for Observed<'_>{fn write_bytes(&mut self,bytes:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),semio_os_kernel::os_pack::PackRefusal>{for byte in bytes{self.inner.write_bytes(&[*byte],control)?;self.accepted.fetch_add(1,Ordering::Relaxed);}Ok(())}}
            let fixture:serde_json::Value=crate::fixtures::fixture_json("📨️authored-operation-source");assert_eq!(fixture["textBytes"],8194);assert_eq!(fixture["frameBytes"],8202);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
            for mode in["complete","file-one-short","depth-zero","initial4096","interior-cancel"]{
                let mutation=crate::test_app_mutation_fixture::TestMutation::from(crate::test_app_mutation_fixture::SetLabel{value:"x".repeat(8194)});let input_pointer=match &mutation{crate::test_app_mutation_fixture::TestMutation::SetLabel(leaf)=>leaf.value.as_ptr(),_=>unreachable!()};
                let accepted=AtomicUsize::new(0);let mut continue_encoding=|_:semio_framework_value::native_encoding::NativeEncodeProgress|mode!="interior-cancel"||accepted.load(Ordering::Relaxed)<256;let mut control=NativeEncodeControl::new(if mode=="initial4096"{4096}else{65536},&mut continue_encoding);let mut options=PackEncodeOptions::default();options.limits.max_file_len=if mode=="file-one-short"{8201}else{8202};if mode=="depth-zero"{options.limits.max_depth=0;}
                let mut child=paged_owner::PagedChildOwner::empty();for(field,text)in[(reader::ChildGroupText::Owner,"owner"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,"child.empty")]{child.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}
                let mut capsule=mutation.retained_pack_operation();
                let result=child.produce_owned_operation(8202,&options,&mut control,|options,output,control|semio_os_kernel::variants_binary::encode_op_into(&mut capsule,options,&mut Observed{inner:output,accepted:&accepted},control));
                match result{Ok(mut receipt)=>{assert_eq!(mode,"complete");receipt.read_first_schema_from_source(&fixture["expected"]["schema"].as_str().unwrap(),&mut control).unwrap();for terminology in semio_ui_locale::Terminology::ALL{for locale in semio_ui_locale::Locale::ALL{let crate::test_app_mutation_fixture::TestMutation::SetLabel(leaf)=&mutation else{unreachable!()};receipt.produce_label(terminology,locale,leaf.value.as_str(),paged_owner::authored(locale).0,paged_owner::authored(locale).1,&mut control).unwrap();}}receipt.commit(&mut control).unwrap();},Err(_)=>assert_ne!(mode,"complete")}
                assert_eq!(match &mutation{crate::test_app_mutation_fixture::TestMutation::SetLabel(leaf)=>leaf.value.as_ptr(),_=>unreachable!()},input_pointer);
                if mode=="complete"{assert_eq!(child.operations.len(),1);assert_eq!(child.operations[0].len(),8202);let canonical=<crate::test_app_mutation_fixture::TestMutation as protocol::OpBinary>::encode_op(&mutation).unwrap();assert_eq!(canonical.len(),8202);for(index,byte)in canonical.iter().copied().enumerate(){assert_eq!(child.operations[0].byte_at(index),Some(byte));}let prefix=[1,1,0,1,0,7,130,64];for(index,byte)in prefix.into_iter().enumerate(){assert_eq!(child.operations[0].byte_at(index),Some(byte));}for index in 8..8202{assert_eq!(child.operations[0].byte_at(index),Some(b'x'));}let schema=fixture["expected"]["schema"].as_str().unwrap();let view=child.view().unwrap();assert_eq!(view.schema.len(),schema.len());for(index,byte)in schema.bytes().enumerate(){assert_eq!(view.schema.byte(index).unwrap(),byte);}}
                else{assert_eq!(child.operations.len(),0);assert!(child.retained_partial_source().is_some());if mode=="interior-cancel"{assert_eq!(accepted.load(Ordering::Relaxed),256);assert_eq!(child.retained_partial_source().unwrap().len(),256);}assert!(child.retained_partial_source().unwrap().len()<8202);}
                let scratch=capsule.allocated_bytes();let before=semio_framework_trace::retained_heap_bytes_on_this_thread();assert_eq!(crate::granted::retire(crate::granted::NONE,|effect|capsule.retire_one(effect)).unwrap(),(false,0,0));assert_eq!(semio_framework_trace::retained_heap_bytes_on_this_thread(),before);assert_eq!(capsule.allocated_bytes(),scratch);
                let mut released=0;for _ in 0..32768{let before=semio_framework_trace::retained_heap_bytes_on_this_thread();let step=crate::granted::retire(crate::granted::PAGE,|effect|capsule.retire_one(effect)).unwrap();let after=semio_framework_trace::retained_heap_bytes_on_this_thread();assert_eq!(usize::try_from(before-after).unwrap(),step.2);assert!(step.1<=1&&step.2<=4096);released+=step.2;if !step.0{break}}assert_eq!(released,scratch);assert_eq!(capsule.allocated_bytes(),0);assert_eq!(capsule.variant_identity().unwrap_err().kind(),semio_framework_value::ValueRefusalKind::InvariantViolated);
                let allocated=child.allocated_bytes();assert_eq!(crate::granted::drain_retained(&mut child),allocated);assert!(child.terminal_is_empty());
            }
        }
    }
