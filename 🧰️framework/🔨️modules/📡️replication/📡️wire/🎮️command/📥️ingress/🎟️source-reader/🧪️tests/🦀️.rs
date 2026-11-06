//! 🧪️ Physical page receipts and suspended bounded field copies use the actual original reader.
use super::*;
fn grant()->Grant{Grant{maximum_items:32,maximum_copy_bytes:16384,maximum_capacity_bytes:16384,maximum_release_bytes:16384,maximum_depth:32}}
fn observed<T>(f:impl FnOnce()->T)->(T,(usize,usize)){let(r,h)=semio_framework_trace::observe_heap_allocations_on_this_thread(f);(r,(h.requested_bytes,h.released_bytes))}
#[test]
fn source_reader_retains_denied_pages_and_copies_only_funded_chunks(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();assert_eq!(law["pageReleaseBytes"],COMMAND_PAGE_MAXIMUM_BYTES);
 let(data,heap)=observed(||{let mut data=CommandPageSet::try_new(2).unwrap();data.try_push(FixedCommandPage::try_copy_from(&[9;COMMAND_PAGE_MAXIMUM_BYTES]).unwrap()).unwrap();data.try_push(FixedCommandPage::try_copy_from(&[7;12]).unwrap()).unwrap();PagedCommandReader::new(PagedCommand::try_from_pages(data).unwrap())});
 let mut reader=data;let mut born=heap.0;let mut released=heap.1;let mut field=None;let stock=grant();let(pointer,initial_offset)=(reader.command.pages.front().unwrap().bytes.as_ptr(),reader.offset);
 for denied in [Grant{maximum_items:0,..stock},Grant{maximum_depth:0,..stock},Grant{maximum_copy_bytes:0,..stock},Grant{maximum_capacity_bytes:0,..stock}]{
  let(turn,heap)=observed(||PagedCommandReader::admit_field_source_step(&mut field,COMMAND_PAGE_MAXIMUM_BYTES+12,denied,100000));assert_eq!(turn.result.unwrap().progress(),Default::default());assert!(field.is_none());assert_eq!(heap,(0,0));
 }
 let(turn,heap)=observed(||PagedCommandReader::admit_field_source_step(&mut field,COMMAND_PAGE_MAXIMUM_BYTES+12,stock,1));let progress=turn.result.unwrap().progress();assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;released+=heap.1;
 for denied in [Grant{maximum_items:0,..stock},Grant{maximum_depth:0,..stock},Grant{maximum_copy_bytes:size_of::<Vec<u8>>()+size_of::<usize>(),..stock}]{
  let(turn,heap)=observed(||reader.copy_field_source_step(field.as_mut().unwrap(),COMMAND_PAGE_MAXIMUM_BYTES+12,denied,100000));assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(reader.offset,initial_offset);assert!(field.as_ref().unwrap().is_empty());
 }
 let(turn,heap)=observed(||reader.copy_field_source_step(field.as_mut().unwrap(),COMMAND_PAGE_MAXIMUM_BYTES+12,stock,4));assert_eq!(turn.work,4);assert_eq!(turn.result.unwrap().progress().copied_bytes,size_of::<Vec<u8>>()+size_of::<usize>()+3);assert_eq!(field.as_ref().unwrap().len(),3);assert_eq!(heap,(0,0));assert_eq!(reader.command.pages.front().unwrap().bytes.as_ptr(),pointer);
 while !reader.source_page_exhausted(){let turn=reader.copy_field_source_step(field.as_mut().unwrap(),COMMAND_PAGE_MAXIMUM_BYTES+12,stock,100000);assert!(turn.result.unwrap().progress().copied_items>0)}
 for denied in [Grant{maximum_items:0,..stock},Grant{maximum_depth:0,..stock},Grant{maximum_copy_bytes:0,..stock},Grant{maximum_release_bytes:COMMAND_PAGE_MAXIMUM_BYTES-1,..stock}]{
  let(turn,heap)=observed(||reader.release_consumed_source_step(denied,1));assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(reader.command.pages.front().unwrap().bytes.as_ptr(),pointer);
 }
 let(turn,heap)=observed(||reader.release_consumed_source_step(stock,1));let progress=turn.result.unwrap().progress();assert_eq!(heap,(0,COMMAND_PAGE_MAXIMUM_BYTES));assert_eq!(progress.released_bytes,heap.1);released+=heap.1;
 let turn=reader.copy_field_source_step(field.as_mut().unwrap(),COMMAND_PAGE_MAXIMUM_BYTES+12,stock,100);assert!(matches!(turn.result.unwrap(),Step::Complete(_)));assert!(reader.source_input_consumed());assert!(reader.source_page_exhausted());assert!(!reader.terminal_is_empty());assert_eq!(field.as_ref().unwrap(),&[vec![9;COMMAND_PAGE_MAXIMUM_BYTES],vec![7;12]].concat());
 let(turn,heap)=observed(||reader.release_consumed_source_step(stock,1));assert_eq!(turn.result.unwrap().progress().released_bytes,COMMAND_PAGE_MAXIMUM_BYTES);assert_eq!(heap,(0,COMMAND_PAGE_MAXIMUM_BYTES));released+=heap.1;
 let(mut owner,heap)=observed(||semio_framework_value::retirement::controlled::ControlledRetirement::new((reader,field.take().unwrap())).ok().unwrap());assert_eq!(heap,(0,0));
 for _ in 0..10000{if owner.terminal_is_empty(){break}let(turn,heap)=observed(||owner.step(stock));let progress=turn.unwrap().progress();assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;released+=heap.1}
 assert!(owner.terminal_is_empty());assert_eq!(born,released);
}
/// 🧫️ Independent fixtures exercise actual retained varints and incremental UTF8 acceptance.
#[test]
fn source_atoms_preserve_actual_partial_reads_and_original_field_backing(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔤️atoms/🔣️.json")).unwrap();let stock=grant();
 fn reader(bytes:&[u8])->PagedCommandReader{let mut pages=CommandPageSet::try_new(1).unwrap();pages.try_push(FixedCommandPage::try_copy_from(bytes).unwrap()).unwrap();PagedCommandReader::new(PagedCommand::try_from_pages(pages).unwrap())}
 fn close<T:semio_framework_value::retirement::RetireOwned>(value:T,mut born:usize,mut freed:usize){let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();for _ in 0..10000{if owner.terminal_is_empty(){assert_eq!(born,freed);return}let(result,heap)=observed(||owner.step(grant()));let progress=result.unwrap().progress();assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;freed+=heap.1}panic!("original atom close did not finish")}
 for row in law["varints"].as_array().unwrap(){let bytes:Vec<u8>=row["bytes"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect();let(mut reader,heap)=observed(||reader(&bytes));let(mut born,mut freed)=heap;let mut atom=OriginalCommandVarint::default();let mut failed=false;
  for _ in 0..100{if atom.value().is_some(){break}for denied in [Grant{maximum_items:0,..stock},Grant{maximum_depth:0,..stock},Grant{maximum_copy_bytes:0,..stock}]{let before=format!("{atom:?}");let(result,heap)=observed(||atom.step(&mut reader,denied,1));assert_eq!(heap,(0,0));if let Ok(step)=result.result{assert_eq!(step.progress(),Default::default())}assert_eq!(format!("{atom:?}"),before)}
   let(turn,heap)=observed(||atom.step(&mut reader,stock,1));match turn.result{Ok(step)=>{let progress=step.progress();assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;freed+=heap.1},Err(_)=>{assert_eq!(heap,(0,0));failed=true;break}}
  }assert_eq!(failed,row["value"].is_null());if !failed{assert_eq!(atom.value().unwrap().to_string(),row["value"].as_str().unwrap())}close((reader,atom),born,freed);
 }
 for row in law["texts"].as_array().unwrap(){let bytes:Vec<u8>=row["bytes"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect();assert_eq!(std::str::from_utf8(&bytes).is_ok(),row["valid"].as_bool().unwrap());let mut input=vec![bytes.len()as u8];input.extend_from_slice(&bytes);let(mut reader,heap)=observed(||reader(&input));let(mut born,mut freed)=heap;let mut atom=OriginalCommandField::default();let mut failed=false;
  for _ in 0..200{if atom.bytes().is_some(){break}let before=format!("{atom:?}");let(turn,heap)=observed(||atom.step(&mut reader,64,true,stock,0));assert_eq!(heap,(0,0));if let Ok(step)=turn.result{assert_eq!(step.progress(),Default::default())}assert_eq!(format!("{atom:?}"),before);
   let(turn,heap)=observed(||atom.step(&mut reader,64,true,stock,2));match turn.result{Ok(step)=>{let progress=step.progress();assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;freed+=heap.1},Err(_)=>{assert_eq!(heap,(0,0));failed=true;break}}
  }assert_eq!(!failed,row["valid"].as_bool().unwrap());if !failed{assert_eq!(atom.bytes().unwrap(),bytes)}close((reader,atom),born,freed);
 }
}
/// 🪦️ The existing boxed page cursor must pay its actual Option clearing before physical release.
#[test]
fn source_page_retirement_requires_its_original_owner_metadata(){
 use semio_framework_value::retirement::RetireOwned;
 let mut cursor=FixedCommandPage::try_copy_from(&[1,2,3]).unwrap().retirement();let stock=grant();let demand=cursor.next_demand(stock.maximum_copy_bytes).unwrap();assert_eq!(demand.copy_bytes,size_of::<Option<Box<[u8;COMMAND_PAGE_MAXIMUM_BYTES]>>>());assert_eq!(demand.release_bytes,COMMAND_PAGE_MAXIMUM_BYTES);
 let(result,heap)=observed(||cursor.close_step(Grant{maximum_copy_bytes:0,..stock}));assert!(matches!(result,semio_framework_value::retirement::RetirementStep::BudgetExhausted));assert_eq!(heap,(0,0));assert!(!cursor.terminal_is_empty());
 let(result,heap)=observed(||cursor.close_step(stock));let semio_framework_value::retirement::RetirementStep::Progress(progress,_) = result else{panic!("funded original page release")};assert_eq!(progress.copied_bytes,demand.copy_bytes);assert_eq!(progress.released_bytes,COMMAND_PAGE_MAXIMUM_BYTES);assert_eq!(heap,(0,COMMAND_PAGE_MAXIMUM_BYTES));assert!(cursor.terminal_is_empty());
}

#[test]
fn borrowed_binary_source_keeps_original_input_and_debits_actual_work(){
 let law:serde_json::Value=serde_json::from_str(include_str!("borrowed.json")).unwrap();let input=law["source"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect::<Vec<_>>();let pointer=input.as_ptr();let stock=grant();
 for row in law["cases"].as_array().unwrap(){
  let(mut offset,length,work,copy)=(row["offset"].as_u64().unwrap()as usize,row["length"].as_u64().unwrap()as usize,row["work"].as_u64().unwrap(),row["copyBody"].as_u64().unwrap()as usize);let initial=offset;let mut field=None;
  let(turn,heap)=observed(||PagedCommandReader::admit_field_source_step(&mut field,length,stock,1));let p=turn.result.unwrap().progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));
  let g=Grant{maximum_copy_bytes:size_of::<Vec<u8>>()+size_of::<usize>()+copy,..stock};
  let(turn,heap)=observed(||OriginalBinarySlice::new(&input,&mut offset).unwrap().copy_field_source_step(field.as_mut().unwrap(),length,g,work));let p=turn.result.unwrap().progress();let count=row["chunk"].as_u64().unwrap()as usize;assert_eq!(offset,initial+count);assert_eq!(field.as_ref().unwrap().as_slice(),&input[initial..initial+count]);assert_eq!(heap,(0,0));assert_eq!(turn.work,if count==0{0}else{count as u64+1});assert!(turn.work<=work);assert_eq!(input.as_ptr(),pointer);assert_eq!(p.copied_bytes,if count==0{0}else{size_of::<Vec<u8>>()+size_of::<usize>()+count});
  let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::retain(field);for _ in 0..32{if owner.terminal_is_empty(){break}let(turn,heap)=observed(||owner.step(stock));let p=turn.unwrap().progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));}assert!(owner.terminal_is_empty());
 }
 let mut wire=vec![8];wire.extend_from_slice(&input[1..]);let mut offset=0;let mut field=OriginalCommandField::default();let mut text=None;
 for _ in 0..256{if field.bytes().is_some(){break}let(turn,heap)=observed(||field.step(&mut OriginalBinarySlice::new(&wire,&mut offset).unwrap(),8,true,stock,32));assert!(turn.work<=32);let p=turn.result.unwrap().progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));}assert_eq!(field.characters(),Some(4));assert_eq!(offset,wire.len());let(turn,heap)=observed(||field.transfer_text_source_step(&mut text,stock,1));assert_eq!(heap,(0,0));assert!(turn.result.is_ok());assert_eq!(text.as_deref(),Some("Aé🦀Z"));
 let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::retain((field,text));for _ in 0..128{if owner.terminal_is_empty(){break}let(turn,heap)=observed(||owner.step(stock));let p=turn.unwrap().progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));}assert!(owner.terminal_is_empty());
}

#[test]
fn strict_nested_codec_observes_canonical_varints_without_changing_generic_acceptance(){
 let law:serde_json::Value=serde_json::from_str(include_str!("borrowed.json")).unwrap();for row in law["canonicalVarints"].as_array().unwrap(){let input=row["wire"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect::<Vec<_>>();let mut offset=0;let mut atom=OriginalCommandVarint::default();for _ in 0..32{if atom.value().is_some(){break}let turn=atom.step(&mut OriginalBinarySlice::new(&input,&mut offset).unwrap(),grant(),1);assert!(turn.work<=1);assert!(turn.result.is_ok());}assert!(atom.value().is_some());assert_eq!(atom.canonical(),row["canonical"].as_bool());assert_eq!(offset,input.len());}
}
