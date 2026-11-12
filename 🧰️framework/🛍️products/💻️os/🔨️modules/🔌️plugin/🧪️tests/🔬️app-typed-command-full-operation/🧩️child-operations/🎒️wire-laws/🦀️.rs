//! 🧩️ Independent child-group wire, paged owner and source-candidate laws.

extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as pack;
extern crate semio_os_kernel as dsl;
use semio_framework_value_testing_allocator as _;
use semio_plugin_decl_emission::ChildEmit;

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
        mod candidate{include!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🧩️projection/🦀️.rs");}
        mod reader{include!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/📖️reader/🦀️.rs");}
        use semio_os_kernel::os_spr::operation_bytes::{OwnedOperationBytes,OperationByteOutput};
        use semio_framework_value::{ErasedSnapshotRetirement,NativeEncodeControl};
        use semio_framework_value_persistent::list::PagedList;
        use semio_ui_locale::LocalizedLabel;
        use candidate::{ChildGroupSources,ChildGroupSource};
        struct Group{owner:String,slot:String,child_id:String,schema:String,ops:PagedList<OwnedOperationBytes,2>,labels:PagedList<LocalizedLabel,2>}
        struct Groups(Vec<Group>);
        impl ChildGroupSources for Groups{
            fn len(&self)->usize{self.0.len()}
            fn group_at(&self,index:usize)->Option<ChildGroupSource<'_>>{self.0.get(index).map(|group|ChildGroupSource{owner:&group.owner,slot:&group.slot,child_id:&group.child_id,schema:&group.schema,operations:&group.ops,labels:&group.labels})}
        }
        fn groups(fixture:&serde_json::Value)->Groups{
            let mut groups=Vec::new();
            for value in fixture["groups"].as_array().unwrap(){
                let mut ops=PagedList::<OwnedOperationBytes,2>::empty();let mut labels=PagedList::<LocalizedLabel,2>::empty();
                while !ops.has_reserved_slot(){assert!(ops.reserve_one_cold(4096).unwrap().progressed)}
                while !labels.has_reserved_slot(){assert!(labels.reserve_one_cold(4096).unwrap().progressed)}
                let mut bytes=OwnedOperationBytes::try_new(8194,65536).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);
                for byte in value["ops"][0].as_array().unwrap(){bytes.write_bytes(&[byte.as_u64().unwrap()as u8],&mut encoding).unwrap();}
                assert!(ops.push_reserved(bytes).is_ok());
                let label=LocalizedLabel::from_fn(|terminology,locale|value["labels"][0][terminology.as_str()][locale.as_str()].as_str().unwrap().to_owned());
                assert!(labels.push_reserved(label).is_ok());
                groups.push(Group{owner:value["owner"].as_str().unwrap().into(),slot:value["slot"].as_str().unwrap().into(),child_id:value["child_id"].as_str().unwrap().into(),schema:value["op_schema"].as_str().unwrap().into(),ops,labels});
            }
            Groups(groups)
        }
        struct Visitor<'a>{expected:&'a Groups,group:usize,operation:usize,byte:usize,label:usize,partial:Option<OwnedOperationBytes>,complete:PagedList<OwnedOperationBytes,4>}
        impl<'a> Visitor<'a>{
            fn new(expected:&'a Groups)->Self{Self{expected,group:0,operation:0,byte:0,label:0,partial:None,complete:PagedList::empty()}}
            fn close(&mut self){if let Some(source)=self.partial.as_mut(){crate::granted::drain_bytes(source);}self.partial.take();while let Some(source)=self.complete.get_mut(self.complete.len().saturating_sub(1)){crate::granted::drain_bytes(source);self.complete.pop();}while !self.complete.terminal_is_empty(){assert!(self.complete.release_empty_page_leased(4096).unwrap().progressed)}}
        }
        impl reader::ChildGroupDecodeVisitor for Visitor<'_>{
            fn begin_groups(&mut self,count:usize,_:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(),semio_os_kernel::os_pack::PackRefusal>{assert_eq!(count,self.expected.0.len());Ok(())}
            fn begin_group(&mut self,index:usize,_:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(),semio_os_kernel::os_pack::PackRefusal>{self.group=index;Ok(())}
            fn text(&mut self,field:reader::ChildGroupText,source:semio_os_kernel::os_pack::codec::ByteSpan<'_>,_:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(),semio_os_kernel::os_pack::PackRefusal>{let group=&self.expected.0[self.group];let expected=match field{reader::ChildGroupText::Owner=>&group.owner,reader::ChildGroupText::Slot=>&group.slot,reader::ChildGroupText::ChildId=>&group.child_id,reader::ChildGroupText::Schema=>&group.schema};assert!(source.iter().eq(expected.bytes()));Ok(())}
            fn begin_operations(&mut self,count:usize,_:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(),semio_os_kernel::os_pack::PackRefusal>{assert_eq!(count,self.expected.0[self.group].ops.len());Ok(())}
            fn begin_operation(&mut self,index:usize,count:usize,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(),semio_os_kernel::os_pack::PackRefusal>{
                self.operation=index;self.byte=0;assert!(self.partial.is_none());assert_eq!(count,self.expected.0[self.group].ops[index].len());
                self.partial=Some(OwnedOperationBytes::try_new(count.max(1),control.maximum_bytes()).unwrap());Ok(())
            }
            fn operation_byte(&mut self,byte:u8,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(),semio_os_kernel::os_pack::PackRefusal>{
                assert_eq!(self.expected.0[self.group].ops[self.operation].byte_at(self.byte),Some(byte));
                let source=self.partial.as_mut().unwrap();
                while !source.has_reserved_slot(){let required=source.next_allocation_bytes().map_err(|fault|semio_framework_value::ValueError::new(fault.kind,fault.reason))?;assert!(required<=4096);control.checkpoint()?;control.charge(required)?;assert!(source.reserve_one(4096).map_err(|fault|semio_framework_value::ValueError::new(fault.kind,fault.reason))?.progressed)}
                source.push_reserved(byte).unwrap();self.byte+=1;Ok(())
            }
            fn end_operation(&mut self,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(),semio_os_kernel::os_pack::PackRefusal>{
                while !self.complete.has_reserved_slot(){let required=self.complete.next_allocation_bytes().map_err(semio_framework_value::ValueError::from)?;assert!(required<=4096);control.charge(required)?;assert!(self.complete.reserve_one_cold(4096).unwrap().progressed)}
                assert!(self.complete.push_reserved(self.partial.take().unwrap()).is_ok());Ok(())
            }
            fn begin_labels(&mut self,count:usize,_:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(),semio_os_kernel::os_pack::PackRefusal>{assert_eq!(count,self.expected.0[self.group].labels.len());Ok(())}
            fn begin_label(&mut self,index:usize,_:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(),semio_os_kernel::os_pack::PackRefusal>{self.label=index;Ok(())}
            fn label_text(&mut self,terminology:semio_ui_locale::Terminology,locale:semio_ui_locale::Locale,source:semio_os_kernel::os_pack::codec::ByteSpan<'_>,_:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(),semio_os_kernel::os_pack::PackRefusal>{assert!(source.iter().eq(self.expected.0[self.group].labels[self.label].resolve(terminology,locale).bytes()));Ok(())}
            fn end_label(&mut self,_:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(),semio_os_kernel::os_pack::PackRefusal>{Ok(())}
            fn end_group(&mut self,_:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(),semio_os_kernel::os_pack::PackRefusal>{Ok(())}
            fn end_groups(&mut self,_:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(),semio_os_kernel::os_pack::PackRefusal>{assert!(self.partial.is_none());Ok(())}
        }
        fn close_groups(groups:&mut Groups){for group in &mut groups.0{while let Some(source)=group.ops.get_mut(group.ops.len().saturating_sub(1)){crate::granted::drain_bytes(source);group.ops.pop();}while !group.ops.terminal_is_empty(){assert!(group.ops.release_empty_page_leased(4096).unwrap().progressed)}while let Some(label)=group.labels.get_mut(group.labels.len().saturating_sub(1)){while label.close_owned_cell_one(4096).unwrap().is_some(){}group.labels.pop();}while !group.labels.terminal_is_empty(){assert!(group.labels.release_empty_page_leased(4096).unwrap().progressed)}}}
        #[test]
        fn child_complete_group_source_candidate_reader_retains_actual_partial8194_pages(){
            let fixture:serde_json::Value=crate::fixtures::fixture_json("🎒️child-group-wire");let mut groups=groups(&fixture);
            let mut output=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);candidate::encode_groups_into(&groups,&semio_os_kernel::os_pack::codec::PackEncodeOptions::default(),&mut output,&mut encoding).unwrap();
            let options=semio_os_kernel::os_pack::codec::PackDecodeOptions::default();
            let mut visitor=Visitor::new(&groups);let mut allow=|_|true;let mut decoding=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(65536,&mut allow);reader::visit_groups_span(semio_os_kernel::os_pack::codec::ByteSpan::from_source(&output),&options,&mut decoding,&mut visitor).unwrap();assert_eq!(visitor.complete.len(),2);assert!(visitor.complete[0].iter().eq(groups.0[0].ops[0].iter()));assert_eq!(visitor.complete[0].len(),8194);assert!(visitor.complete[1].iter().eq(groups.0[1].ops[0].iter()));let decoded_owned=decoding.owned_bytes();assert_eq!(decoded_owned,visitor.complete.allocated_bytes()+visitor.complete.iter().map(OwnedOperationBytes::allocated_bytes).sum::<usize>());visitor.close();
            let mut visitor=Visitor::new(&groups);let mut cancel=|progress:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|progress.owned_bytes<8192;let mut decoding=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(65536,&mut cancel);assert!(reader::visit_groups_span(semio_os_kernel::os_pack::codec::ByteSpan::from_source(&output),&options,&mut decoding,&mut visitor).is_err());let partial=visitor.partial.as_ref().unwrap();assert!(partial.len()>0&&partial.len()<8194);assert!(partial.allocated_bytes()>=8192);assert_eq!(partial.allocated_bytes(),decoding.owned_bytes());let retained=partial.len();visitor.close();
            let mut visitor=Visitor::new(&groups);let mut allow=|_|true;let mut decoding=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(4096,&mut allow);assert!(reader::visit_groups_span(semio_os_kernel::os_pack::codec::ByteSpan::from_source(&output),&options,&mut decoding,&mut visitor).is_err());assert!(visitor.partial.as_ref().unwrap().is_empty());assert!(visitor.partial.as_ref().unwrap().allocated_bytes()>0);assert!(decoding.owned_bytes()<=4096);visitor.close();
            let mut visitor=Visitor::new(&groups);let mut allow=|_|true;let mut decoding=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(65536,&mut allow);let short=semio_os_kernel::os_pack::codec::ByteSpan::from_source(&output).slice(0,output.len()-1).unwrap();assert!(reader::visit_groups_span(short,&options,&mut decoding,&mut visitor).is_err());assert_eq!(visitor.complete.len(),2);visitor.close();
            crate::granted::drain_bytes(&mut output);close_groups(&mut groups);
        }
        #[test]
        fn child_complete_group_source_candidate_matches_independent_wire(){
            let fixture:serde_json::Value=crate::fixtures::fixture_json("🎒️child-group-wire");
            let mut groups=groups(&fixture);assert_eq!(groups.0[0].ops[0].len(),8194);
            let options=semio_os_kernel::os_pack::codec::PackEncodeOptions::default();
            let mut output=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);
            let count=candidate::encode_groups_into(&groups,&options,&mut output,&mut control).unwrap();assert_eq!(count,output.len());
            let expected=fixture["wire"].as_array().unwrap();assert_eq!(output.len(),expected.len());for(index,byte)in expected.iter().enumerate(){assert_eq!(output.byte_at(index),Some(byte.as_u64().unwrap()as u8),"exact original group octet {index}")}
            let mut comparison=semio_os_kernel::os_spr::operation_bytes::OperationByteComparison::new(semio_os_kernel::os_pack::codec::ByteSpan::from_source(&output));let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);candidate::encode_groups_into(&groups,&options,&mut comparison,&mut control).unwrap();comparison.finish().unwrap();
            let mut empty=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);assert_eq!(candidate::encode_groups_into(&Groups(Vec::new()),&options,&mut empty,&mut control).unwrap(),0);assert_eq!(empty.len(),0);assert_eq!(empty.allocated_bytes(),0);crate::granted::drain_bytes(&mut empty);
            let mut partial=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut allow=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|progress.owned_bytes<8192;let mut control=NativeEncodeControl::new(65536,&mut allow);assert!(candidate::encode_groups_into(&groups,&options,&mut partial,&mut control).is_err());assert!(partial.allocated_bytes()>0);let retained=partial.allocated_bytes();crate::granted::drain_bytes(&mut partial);
            let mut refused=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut small=options.clone();small.limits.max_total_alloc=128;let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);assert!(candidate::encode_groups_into(&groups,&small,&mut refused,&mut control).is_err());assert!(control.owned_bytes()<=128);crate::granted::drain_bytes(&mut refused);
            crate::granted::drain_bytes(&mut output);
            close_groups(&mut groups);
        }
        #[test]
        fn child_complete_original_semantic_metadata8194_grant4096(){
            let corpus:serde_json::Value=crate::fixtures::fixture_json("🔤️semantic-text-wire");
            for case in corpus["cases"].as_array().unwrap(){
                let field=case["field"].as_str().unwrap();let large="m".repeat(8194);assert_eq!(large.len(),8194);let mut child=super::ChildEmit::open("","",0);
                match field{"owner"=>child.owner=large,"slot"=>child.slot=large,"child_id"=>child.child_id=large,"op_schema"=>child.op_schema.0=large,_=>{let keys:Vec<&str>=field.split('.').collect();assert_eq!(keys.len(),3);let label=semio_ui_locale::LocalizedLabel::from_fn(|terminology,locale|if terminology.as_str()==keys[1]&&locale.as_str()==keys[2]{large.clone()}else{String::new()});child.labels.push(label);}}
                use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};let page=RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:semio_framework_value_ordered::ordered::MAX_AVL_HEIGHT * 5 + 16 };
                assert_eq!(child.close_one(RetainedCloneGrant{maximum_items:0,..page}).unwrap(),RetainedCloneStep::Progress(RetainedCloneProgress::default()));let mut complete=false;let mut physical=0;
                if matches!(field,"owner"|"slot"|"child_id"|"op_schema"){for _ in 0..64{if matches!(child.close_one(page).unwrap(),RetainedCloneStep::Complete(_)){panic!("a String backing releases whole: a 4096-byte release grant can never retire the 8194-byte owner: {field}")}}}
                let whole=RetainedCloneGrant{maximum_release_bytes:8194+4096,..page};
                for _ in 0..8194+128{match child.close_one(whole).unwrap(){RetainedCloneStep::Progress(progress)=>{assert!(progress.fits(whole));physical+=progress.released_bytes;},RetainedCloneStep::Complete(progress)=>{assert!(progress.fits(whole));physical+=progress.released_bytes;complete=true;break}}}
                assert!(complete,"actual ChildEmit full8194 semantic owner remained held under the whole-backing release grant: {field}");assert!(physical>=8194);assert!(child.ops.is_empty());assert!(child.labels.is_empty());assert!(child.owner.is_empty());assert!(child.slot.is_empty());assert!(child.child_id.is_empty());assert!(child.op_schema.0.is_empty());
            }
        }
        fn drain_typed_groups(groups:&mut semio_framework_value_persistent::list::PagedList<paged_owner::PagedChildOwner,{isize::MAX as usize}>)->usize{
            let mut released=0;while let Some(child)=groups.get_mut(groups.len().saturating_sub(1)){released+=crate::granted::drain_retained(child);groups.pop();}while !groups.terminal_is_empty(){let step=groups.release_empty_page_leased(4096).unwrap();assert!(step.progressed);assert!(step.released_allocation_bytes<=4096);released+=step.released_allocation_bytes;}released
        }
        #[test]
        fn child_complete_group_source_candidate_paged_semantic8194_metadata_and_labels(){
            use semio_framework_value::{NativeEncodeControl, ErasedSnapshotRetirement};
            use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::NativeDecodeRetirementRecipient};
            use semio_os_kernel::{os_pack::codec::{ByteSpan,PackEncodeOptions,PackDecodeOptions},os_spr::operation_bytes::{OperationByteOutput,OperationByteComparison}};
            let corpus:serde_json::Value=crate::fixtures::fixture_json("🔤️semantic-text-wire");assert_eq!(corpus["maximumItems"],1);assert_eq!(corpus["maximumBytes"],4096);assert_eq!(corpus["maximumAllocationBytes"],65536);
            let mut cases=0;for case in corpus["cases"].as_array().unwrap(){
                let wire:Vec<u8>=case["wire"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();let mut source=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);source.write_bytes(&wire,&mut encoding).unwrap();let source_bytes=source.allocated_bytes();
                let mut recipient=NativeDecodeRetirementRecipient::new();let mut allow=|_|true;let mut decoding=NativeDecodeControl::new(65536,&mut allow);decoding.install_retirement_recipient(&mut recipient).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);let mut groups=paged_owner::decode_paged_groups_span(ByteSpan::from_source(&source),&PackDecodeOptions::default(),&PackEncodeOptions::default(),&mut decoding,&mut encoding).unwrap();let admitted=decoding.owned_bytes();drop(decoding);assert!(recipient.has_owner());assert_eq!(groups.len(),1);assert_eq!(groups[0].operations[0].len(),8194);assert!(groups[0].operations[0].iter().eq("17".bytes().chain(std::iter::repeat_n(b' ',8192))));
                let typed_bytes=groups.allocated_bytes()+groups.iter().map(paged_owner::PagedChildOwner::allocated_bytes).sum::<usize>();assert_eq!(admitted,std::mem::size_of::<paged_owner::PagedChildDecodeOwner>()+typed_bytes);let source_group=paged_encoder::PagedChildGroups::group(&groups,0).unwrap();let field=case["field"].as_str().unwrap();let text=match field{"owner"=>source_group.owner,"slot"=>source_group.slot,"child_id"=>source_group.child_id,"op_schema"=>source_group.schema,"label.native.en"=>source_group.labels.text(0,semio_ui_locale::Terminology::ALL[0],semio_ui_locale::Locale::ALL[0]).unwrap(),"label.native.de"=>source_group.labels.text(0,semio_ui_locale::Terminology::ALL[0],semio_ui_locale::Locale::ALL[1]).unwrap(),"label.reuse.en"=>source_group.labels.text(0,semio_ui_locale::Terminology::ALL[1],semio_ui_locale::Locale::ALL[0]).unwrap(),"label.reuse.de"=>source_group.labels.text(0,semio_ui_locale::Terminology::ALL[1],semio_ui_locale::Locale::ALL[1]).unwrap(),_=>panic!("closed semantic field changed")};assert_eq!(text.len(),8194);for index in 0..8194{assert_eq!(text.byte(index).unwrap(),b'm')}
                let scaffold_released=crate::granted::drain_retained(&mut recipient);assert!(scaffold_released>=std::mem::size_of::<paged_owner::PagedChildDecodeOwner>());assert_eq!(source.allocated_bytes(),source_bytes);assert_eq!(groups.allocated_bytes()+groups.iter().map(paged_owner::PagedChildOwner::allocated_bytes).sum::<usize>(),typed_bytes);
                let mut symbols=paged_encoder::ChildSymbolOwner::empty();let mut comparison=OperationByteComparison::new(ByteSpan::from_source(&source));let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);assert_eq!(paged_encoder::encode_paged_groups_into(&groups,&PackEncodeOptions::default(),&mut comparison,&mut symbols,&mut encoding).unwrap(),wire.len());comparison.finish().unwrap();crate::granted::drain_retained(&mut symbols);assert_eq!(drain_typed_groups(&mut groups),typed_bytes);assert_eq!(source.allocated_bytes(),source_bytes);
                let mut recipient=NativeDecodeRetirementRecipient::new();let mut stages=0;let mut cancel=|progress:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|{if progress.total==8194&&progress.completed==0{stages+=1;}!(progress.total==8194&&stages>=2&&progress.completed>=256)};let mut decoding=NativeDecodeControl::new(65536,&mut cancel);decoding.install_retirement_recipient(&mut recipient).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);assert!(paged_owner::decode_paged_groups_span(ByteSpan::from_source(&source),&PackDecodeOptions::default(),&PackEncodeOptions::default(),&mut decoding,&mut encoding).is_err());let partial_admitted=decoding.owned_bytes();drop(decoding);assert!(recipient.has_owner());assert!(partial_admitted>=std::mem::size_of::<paged_owner::PagedChildDecodeOwner>()+4096);assert_eq!(crate::granted::drain_retained(&mut recipient),partial_admitted);
                let mut recipient=NativeDecodeRetirementRecipient::new();let mut allow=|_|true;let mut decoding=NativeDecodeControl::new(4096,&mut allow);decoding.install_retirement_recipient(&mut recipient).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);assert!(paged_owner::decode_paged_groups_span(ByteSpan::from_source(&source),&PackDecodeOptions::default(),&PackEncodeOptions::default(),&mut decoding,&mut encoding).is_err());let small_admitted=decoding.owned_bytes();drop(decoding);assert!(small_admitted<=4096);assert_eq!(crate::granted::drain_retained(&mut recipient),small_admitted);
                let mut allow=|_|true;let mut decoding=NativeDecodeControl::new(65536,&mut allow);let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);assert!(paged_owner::decode_paged_groups_span(ByteSpan::from_source(&source),&PackDecodeOptions::default(),&PackEncodeOptions::default(),&mut decoding,&mut encoding).is_err());assert_eq!(decoding.owned_bytes(),0);crate::granted::drain_bytes(&mut source);cases+=1;
            }assert_eq!(cases,8);
        }
        #[test]
        fn child_complete_group_source_candidate_direct_producer_and_parent(){
            use semio_framework_value::{NativeEncodeControl,ErasedSnapshotRetirement,retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep}};
            use semio_os_kernel::{os_pack::codec::PackEncodeOptions,os_spr::operation_bytes::OperationByteOutput};
            use reader::ChildGroupText;
            let fixture:serde_json::Value=crate::fixtures::fixture_json("📤️child-group-producer");assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["textBytes"],8194);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);let large=fixture["text"].as_str().unwrap().repeat(8194);
            fn encode(output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),semio_os_kernel::os_spr::ProtocolError>{output.write_bytes(b"17",control)?;let padding=[b' ';256];for _ in 0..32{output.write_bytes(&padding,control)?;}Ok(())}
            fn initialize(owner:&mut paged_owner::PagedChildOwner,text:&str,options:&PackEncodeOptions,control:&mut NativeEncodeControl<'_>){for(field,value)in[(ChildGroupText::Owner,text),(ChildGroupText::Slot,"slot"),(ChildGroupText::ChildId,"child"),(ChildGroupText::Schema,"child.empty")]{owner.read_metadata_from_encoding_source(field,&value,options,control).unwrap();}}
            let options=PackEncodeOptions::default();let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);initialize(&mut owner,&large,&options,&mut control);let mut receipt=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|encode(output,control)).unwrap();receipt.read_first_schema_from_source(&"schema.actual",&mut control).unwrap();for terminology in semio_ui_locale::Terminology::ALL{for locale in semio_ui_locale::Locale::ALL{let text=if terminology==semio_ui_locale::Terminology::ALL[0]&&locale==semio_ui_locale::Locale::ALL[0]{large.as_str()}else{"small"};receipt.read_label_from_source(terminology,locale,&text,&mut control).unwrap();}}receipt.commit(&mut control).unwrap();assert_eq!(owner.operations.len(),1);assert_eq!(owner.operations[0].len(),8194);assert!(owner.operations[0].iter().eq(b"17".iter().copied().chain(std::iter::repeat_n(b' ',8192))));assert_eq!(owner.allocated_bytes(),control.owned_bytes());let allocated=owner.allocated_bytes();let mut parent=ParentAllocationReturn::<128>::try_new(4096,allocated).unwrap();assert!(!owner.return_one(&mut parent,0,4096).unwrap().progressed);let mut returned=0;
            for _ in 0..8194*3+256{if owner.terminal_is_empty(){break;}let before=owner.allocated_bytes();let step=owner.return_one(&mut parent,1,4096).unwrap();assert!(step.progressed);assert_eq!(before-owner.allocated_bytes(),step.returned_allocation_bytes);returned+=step.returned_allocation_bytes;}assert!(owner.terminal_is_empty());assert_eq!(returned,allocated);assert_eq!(parent.retained_bytes(),allocated);assert!(!parent.terminal_is_empty());let mut released=0;for _ in 0..256{match parent.close_step(1,4096){AllocationReturnStep::Complete=>break,AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);released+=released_bytes;}}}assert!(parent.terminal_is_empty());assert_eq!(released,allocated);
            for mode in["encoder","label","file","segment"]{let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);initialize(&mut owner,"root",&options,&mut control);let mut limits=options.clone();if mode=="file"{limits.limits.max_file_len=1;}if mode=="segment"{limits.limits.max_segment_len=128;}
                let result=owner.produce_owned_operation(8194,&limits,&mut control,|_,output,control|{encode(output,control)?;if mode=="encoder"{return Err(semio_os_kernel::os_pack::PackRefusal::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::Canceled,"neutral direct encoder refused after source")).into())}Ok(())});
                match result{Err(_)=>{assert!(mode=="encoder"||mode=="file")},Ok(mut receipt)=>{receipt.read_first_schema_from_source(&"schema.actual",&mut control).unwrap();assert!(mode=="label"||mode=="segment");if mode=="segment"{assert!(receipt.read_label_from_source(semio_ui_locale::Terminology::ALL[0],semio_ui_locale::Locale::ALL[0],&large.as_str(),&mut control).is_err());}else{receipt.read_label_from_source(semio_ui_locale::Terminology::ALL[0],semio_ui_locale::Locale::ALL[0],&large.as_str(),&mut control).unwrap();}assert!(receipt.commit(&mut control).is_err());}}
                assert_eq!(owner.operations.len(),0);let view=owner.view().unwrap();assert_eq!(view.labels.len(),0);assert_eq!(view.schema.len(),11);for(index,byte)in b"child.empty".iter().enumerate(){assert_eq!(view.schema.byte(index).unwrap(),*byte)}let allocated=owner.allocated_bytes();assert_eq!(allocated,control.owned_bytes());assert_eq!(crate::granted::drain_retained(&mut owner),allocated);
            }
        }
        #[test]
        fn child_complete_group_source_candidate_repeated_prefix_preserves_accepted_owners(){
            use semio_framework_value::{NativeEncodeControl,ErasedSnapshotRetirement,retirement::allocation_return::ParentAllocationReturn};
            use semio_os_kernel::{os_pack::codec::PackEncodeOptions,os_spr::operation_bytes::OperationByteOutput};
            let fixture:serde_json::Value=crate::fixtures::fixture_json("🔁️repeated-prefix");assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);assert_eq!(fixture["acceptedOperations"],2);
            fn encode(output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),semio_os_kernel::os_spr::ProtocolError>{output.write_bytes(b"17",control)?;for _ in 0..32{output.write_bytes(&[b' ';256],control)?;}Ok(())}
            let options=PackEncodeOptions::default();let original_large_schema=std::iter::repeat_n('s',8194).collect::<String>();for original_schema in["child.empty",original_large_schema.as_str()]{for refused in[false,true]{let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);for(field,text)in[(reader::ChildGroupText::Owner,"root"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,original_schema)]{owner.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}
                let mut receipt=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|encode(output,control)).unwrap();receipt.read_first_schema_from_source(&"schema.actual",&mut control).unwrap();for terminology in semio_ui_locale::Terminology::ALL{for locale in semio_ui_locale::Locale::ALL{receipt.read_label_from_source(terminology,locale,&"first",&mut control).unwrap();}}receipt.commit(&mut control).unwrap();let source_pointer=owner.operations[0].byte_ref(0).map(|byte|byte as *const u8).unwrap();let allocated=owner.allocated_bytes();let mut parent=ParentAllocationReturn::<16>::try_new(4096,65536).unwrap();assert!(!owner.return_committed_scaffold_one(&mut parent,0).unwrap().progressed);assert_eq!(owner.allocated_bytes(),allocated);
                let mut returned=0;for _ in 0..8194+128{let before=owner.allocated_bytes();let step=owner.return_committed_scaffold_one(&mut parent,1).unwrap();if !step.progressed{break;}assert_eq!(before-owner.allocated_bytes(),step.returned_allocation_bytes);returned+=step.returned_allocation_bytes;assert_eq!(owner.operations.len(),1);assert_eq!(owner.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(source_pointer));assert_eq!(owner.view().unwrap().labels.len(),1);}assert_eq!(parent.retained_bytes(),returned);if original_schema.len()==8194{assert!(returned>8194);assert!(!parent.terminal_is_empty());}else{assert_eq!(returned,0);assert!(parent.terminal_is_empty());}let mut schema_released=0;for _ in 0..128{match parent.close_step(1,4096){semio_framework_value::retirement::allocation_return::AllocationReturnStep::Complete=>break,semio_framework_value::retirement::allocation_return::AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);schema_released+=released_bytes;}}}assert!(parent.terminal_is_empty());assert_eq!(schema_released,returned);
                let result=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|{encode(output,control)?;if refused{return Err(semio_os_kernel::os_pack::PackRefusal::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::Canceled,"second direct encoder refused")).into())}Ok(())});match result{Err(_)=>assert!(refused),Ok(mut receipt)=>{assert!(!refused);for terminology in semio_ui_locale::Terminology::ALL{for locale in semio_ui_locale::Locale::ALL{receipt.read_label_from_source(terminology,locale,&"second",&mut control).unwrap();}}receipt.commit(&mut control).unwrap();}}
                assert_eq!(owner.operations.len(),if refused{1}else{2});assert_eq!(owner.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(source_pointer));assert!(owner.operations.iter().all(|source|source.len()==8194&&source.iter().eq(b"17".iter().copied().chain(std::iter::repeat_n(b' ',8192)))));let view=owner.view().unwrap();assert_eq!(view.labels.len(),if refused{1}else{2});for(index,byte)in b"schema.actual".iter().enumerate(){assert_eq!(view.schema.byte(index).unwrap(),*byte)}assert_eq!(owner.allocated_bytes()+schema_released,control.owned_bytes());let allocated=owner.allocated_bytes();assert_eq!(crate::granted::drain_retained(&mut owner),allocated);
            }}
        }
        #[test]
        fn child_complete_group_source_candidate_refuses_partial_last_locale_before_commit(){
            use semio_framework_value::{NativeEncodeControl,native_encoding::NativeEncodeProgress};
            use semio_os_kernel::os_pack::codec::PackEncodeOptions;
            let fixture:serde_json::Value=crate::fixtures::fixture_json("🌓️partial-label");
            assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["textBytes"],8194);assert_eq!(fixture["partialBytes"],256);assert_eq!(fixture["completeCells"],3);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
            let options=PackEncodeOptions::default();let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);
            for(field,text)in[(reader::ChildGroupText::Owner,"root"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,"child.empty")]{owner.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}
            let mut receipt=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|{output.write_bytes(b"17",control)?;for _ in 0..32{output.write_bytes(&[b' ';256],control)?;}Ok(())}).unwrap();receipt.read_first_schema_from_source(&"schema.actual",&mut control).unwrap();
            let mut axes=Vec::new();for terminology in semio_ui_locale::Terminology::ALL{for locale in semio_ui_locale::Locale::ALL{axes.push((terminology,locale));}}
            for(terminology,locale)in &axes[..3]{receipt.read_label_from_source(*terminology,*locale,&"first",&mut control).unwrap();}
            let baseline=control.owned_bytes();let continuation=control.pause().unwrap();let mut cancel=|progress:NativeEncodeProgress|progress.owned_bytes<=baseline||progress.completed<256;let mut control=NativeEncodeControl::resume(continuation,&mut cancel).unwrap();let large="x".repeat(8194);assert!(receipt.read_label_from_source(axes[3].0,axes[3].1,&large.as_str(),&mut control).is_err());assert!(control.owned_bytes()>baseline);
            let continuation=control.pause().unwrap();let mut allow=|_|true;let mut control=NativeEncodeControl::resume(continuation,&mut allow).unwrap();assert!(receipt.commit(&mut control).is_err(),"partial last locale was published despite lacking complete text authority");
            assert_eq!(owner.operations.len(),0);let view=owner.view().unwrap();assert_eq!(view.labels.len(),0);assert_eq!(view.schema.len(),11);for(index,byte)in b"child.empty".iter().enumerate(){assert_eq!(view.schema.byte(index).unwrap(),*byte)}let allocated=owner.allocated_bytes();assert_eq!(allocated,control.owned_bytes());assert_eq!(crate::granted::drain_retained(&mut owner),allocated);
        }
    }
