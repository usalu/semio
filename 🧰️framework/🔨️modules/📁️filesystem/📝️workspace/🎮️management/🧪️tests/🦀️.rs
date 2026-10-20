//! 🧪️ Physical file-management commands retain truthful publication outcomes.

use super::*;
use crate::tests::{root,journal};

#[test]
fn source_management_creates_renames_retains_and_reopens_exact_events() {
    let root=root("management-files"); let workspace=Workspace::new(&root).unwrap(); let cancel=AtomicBool::new(false);
    let document=workspace.create_file("Grüße 🦑.txt","one\r\n🦑\r\n",&cancel,&mut |_|{}).unwrap();
    assert_eq!(document.text,"one\r\n🦑\r\n");
    assert_eq!(workspace.create_file(&document.path,"foreign",&cancel,&mut |_|{}).unwrap_err().kind,ErrorKind::Conflict);
    let revision=workspace.inspect_entry(&document.path,&cancel,&mut |_|{}).unwrap();
    let renamed=workspace.rename_entry(&document.path,&revision,"renamed.txt",&cancel,&mut |_|{}).unwrap();
    assert_eq!(renamed.path,"renamed.txt"); assert!(!root.join(&document.path).exists());
    assert_eq!(fs::read_to_string(root.join(&renamed.path)).unwrap(),document.text);
    let revision=workspace.inspect_entry(&renamed.path,&cancel,&mut |_|{}).unwrap();
    let retained=workspace.retain_entry(&renamed.path,&revision,&cancel,&mut |_|{}).unwrap();
    assert!(!retained.directory); assert!(!retained.command_id.is_empty()); assert!(!root.join(&renamed.path).exists());
    assert_eq!(fs::read_to_string(&retained.recovery_path).unwrap(),document.text);
    assert_eq!(workspace.load_recovery(&retained.recovery_path,&cancel,&mut |_|{}).unwrap().text,document.text);
    let oracle=serde_json::to_value(&document.text).unwrap(); assert_eq!(serde_json::to_value(fs::read_to_string(&retained.recovery_path).unwrap()).unwrap(),oracle);
    workspace.create_file("after.txt","after",&cancel,&mut |_|{}).unwrap();
}

#[test]
fn source_management_empty_directories_are_retained_without_blocking_future_commands() {
    let root=root("management-directories"); let workspace=Workspace::new(&root).unwrap(); let cancel=AtomicBool::new(false);
    assert!(workspace.create_directory("empty",&cancel,&mut |_|{}).unwrap().directory);
    let revision=workspace.inspect_entry("empty",&cancel,&mut |_|{}).unwrap();
    assert!(workspace.rename_entry("empty",&revision,"renamed",&cancel,&mut |_|{}).unwrap().directory);
    let revision=workspace.inspect_entry("renamed",&cancel,&mut |_|{}).unwrap();
    let retained=workspace.retain_entry("renamed",&revision,&cancel,&mut |_|{}).unwrap();
    assert!(retained.directory); assert!(Path::new(&retained.recovery_path).is_dir()); assert_eq!(fs::read_dir(&retained.recovery_path).unwrap().count(),0);
    workspace.create_file("after.txt","after",&cancel,&mut |_|{}).unwrap();
    assert_eq!(workspace.list("",&cancel,&mut |_|{}).unwrap().len(),1);
}

#[test]
fn source_management_nonempty_directory_moves_and_retains_the_complete_observed_tree(){
    let root=root("management-nonempty");let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);fs::create_dir(root.join("folder")).unwrap();fs::write(root.join("folder/child.txt"),"child").unwrap();let revision=workspace.inspect_entry("folder",&cancel,&mut |_|{}).unwrap();workspace.rename_entry("folder",&revision,"renamed",&cancel,&mut |_|{}).unwrap();assert!(!root.join("folder").exists());let revision=workspace.inspect_entry("renamed",&cancel,&mut |_|{}).unwrap();let retained=workspace.retain_entry("renamed",&revision,&cancel,&mut |_|{}).unwrap();assert!(!root.join("renamed").exists());assert_eq!(fs::read_to_string(Path::new(&retained.recovery_path).join("child.txt")).unwrap(),"child");
}

#[test]
fn source_management_stale_and_foreign_revisions_never_move_source() {
    let root=root("management-stale"); let workspace=Workspace::new(&root).unwrap(); let cancel=AtomicBool::new(false);
    fs::write(root.join("a.txt"),"before").unwrap(); let revision=workspace.inspect_entry("a.txt",&cancel,&mut |_|{}).unwrap();
    fs::write(root.join("a.txt"),"changed-extent").unwrap();
    assert_eq!(workspace.rename_entry("a.txt",&revision,"b.txt",&cancel,&mut |_|{}).unwrap_err().kind,ErrorKind::Conflict);
    assert_eq!(workspace.retain_entry("a.txt",&revision,&cancel,&mut |_|{}).unwrap_err().kind,ErrorKind::Conflict);
    fs::write(root.join("other.txt"),"other").unwrap();
    assert_eq!(workspace.rename_entry("other.txt",&revision,"b.txt",&cancel,&mut |_|{}).unwrap_err().kind,ErrorKind::Conflict);
    assert_eq!(fs::read_to_string(root.join("a.txt")).unwrap(),"changed-extent"); assert!(!root.join("b.txt").exists());
}

#[test]
fn source_management_destination_race_is_atomic_and_retains_both_foreign_and_source() {
    let root=root("management-destination-race"); let workspace=Workspace::new(&root).unwrap(); let cancel=AtomicBool::new(false);
    fs::write(root.join("a.txt"),"source").unwrap(); let revision=workspace.inspect_entry("a.txt",&cancel,&mut |_|{}).unwrap(); let mut raced=false;
    let result=workspace.rename_entry("a.txt",&revision,"b.txt",&cancel,&mut |p|{if p.stage==Stage::Publishing&&!raced{fs::write(root.join("b.txt"),"foreign").unwrap();raced=true;}});
    assert_eq!(result.unwrap_err().kind,ErrorKind::Conflict); assert!(raced);
    assert_eq!(fs::read_to_string(root.join("a.txt")).unwrap(),"source"); assert_eq!(fs::read_to_string(root.join("b.txt")).unwrap(),"foreign");
}

#[test]
fn source_management_postpublication_identity_race_has_truthful_recovery() {
    let root=root("management-source-race"); let workspace=Workspace::new(&root).unwrap(); let cancel=AtomicBool::new(false);
    fs::write(root.join("a.txt"),"expected").unwrap(); let revision=workspace.inspect_entry("a.txt",&cancel,&mut |_|{}).unwrap(); let mut raced=false;
    let result=workspace.rename_entry("a.txt",&revision,"b.txt",&cancel,&mut |p|{if p.stage==Stage::Publishing&&!raced{fs::rename(root.join("a.txt"),root.join("original.txt")).unwrap();fs::write(root.join("a.txt"),"foreign").unwrap();raced=true;}});
    let failure=result.unwrap_err(); assert_eq!(failure.kind,ErrorKind::Conflict); assert!(raced);
    assert_eq!(failure.recovery_path.as_deref(),Some(root.join("b.txt").to_str().unwrap()));
    assert_eq!(fs::read_to_string(root.join("b.txt")).unwrap(),"foreign"); assert_eq!(fs::read_to_string(root.join("original.txt")).unwrap(),"expected");
}

#[test]
fn source_management_postpublication_new_child_is_retained_and_reported() {
    let root=root("management-child-race"); let workspace=Workspace::new(&root).unwrap(); let cancel=AtomicBool::new(false);
    fs::create_dir(root.join("folder")).unwrap(); let revision=workspace.inspect_entry("folder",&cancel,&mut |_|{}).unwrap(); let mut raced=false;
    let result=workspace.retain_entry("folder",&revision,&cancel,&mut |p|{if p.stage==Stage::Publishing&&!raced{fs::write(root.join("folder/foreign.txt"),"foreign").unwrap();raced=true;}});
    let failure=result.unwrap_err(); assert_eq!(failure.kind,ErrorKind::Conflict); assert!(raced);
    assert_eq!(fs::read_to_string(Path::new(failure.recovery_path.as_ref().unwrap()).join("foreign.txt")).unwrap(),"foreign");
    workspace.create_file("after.txt","after",&cancel,&mut |_|{}).unwrap();
}

#[test]
fn source_management_cancel_before_publication_preserves_source_and_absence() {
    let root=root("management-cancel"); let workspace=Workspace::new(&root).unwrap(); let cancel=AtomicBool::new(false);
    fs::write(root.join("a.txt"),"source").unwrap(); let revision=workspace.inspect_entry("a.txt",&cancel,&mut |_|{}).unwrap();
    let failure=workspace.rename_entry("a.txt",&revision,"b.txt",&cancel,&mut |p|if p.stage==Stage::Publishing{cancel.store(true,Ordering::Release)}).unwrap_err();
    assert_eq!(failure.kind,ErrorKind::Cancelled); assert_eq!(fs::read_to_string(root.join("a.txt")).unwrap(),"source"); assert!(!root.join("b.txt").exists());
}

#[test]
fn source_management_late_cancellation_cannot_erase_committed_creation() {
    let root=root("management-late-cancel"); let workspace=Workspace::new(&root).unwrap(); let cancel=AtomicBool::new(false);
    let document=workspace.create_file("a.txt","created",&cancel,&mut |p|if p.stage==Stage::Complete{cancel.store(true,Ordering::Release)}).unwrap();
    assert!(cancel.load(Ordering::Acquire)); assert_eq!(document.text,"created"); assert_eq!(fs::read_to_string(root.join("a.txt")).unwrap(),"created");
}

#[test]
fn source_management_namespace_continuity_matches_closed_neutral_u64_coordinates(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🪪️namespace/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let parse=|value:&serde_json::Value|->Option<[(u64,u64);3]>{let object=value.as_object()?;if object.len()!=3{return None;}let mut result=[(0,0);3];for(index,key)in ["root","journal","entry"].iter().enumerate(){let item=object.get(*key)?.as_object()?;if item.len()!=2{return None;}let token=|key:&str|->Option<u64>{let text=item.get(key)?.as_str()?;let number=text.parse::<u64>().ok()?;if text!=number.to_string()||(key=="inode"&&number==0){return None;}Some(number)};result[index]=(token("device")?,token("inode")?);}Some(result)};
        let input=&row["input"];let parsed=input.as_object().filter(|v|v.len()==2).and_then(|v|Some((parse(v.get("published")?)?,parse(v.get("current")?)?)));
        assert_eq!(parsed.is_some(),row["schemaValid"].as_bool().unwrap(),"{}",row["id"]);let accepted=parsed.is_some_and(|(published,current)|crate::management::namespace_continuity(published,current));assert_eq!(accepted,row["accepted"].as_bool().unwrap(),"{}",row["id"]);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&serde_json::to_string(&accepted).unwrap()).unwrap(),row["accepted"]);
    }
}

#[test]
fn source_management_shared_command_cases_drive_actual_physical_outcomes(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let root=root("management-neutral");let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);let path=row["path"].as_str().unwrap();let initial=row["initial"].as_str().unwrap();
        match initial{"file"=>fs::write(root.join(path),"before").unwrap(),"empty-directory"=>fs::create_dir(root.join(path)).unwrap(),"nonempty-directory"=>{fs::create_dir(root.join(path)).unwrap();fs::write(root.join(path).join("child.txt"),"child").unwrap();},"missing"=>{},_=>panic!("Unknown neutral initial entry")}
        let destination=row["destination"].as_str();if row["destinationExists"]==true{fs::write(root.join(destination.unwrap()),"occupied").unwrap();}
        let expected=if matches!(row["command"].as_str().unwrap(),"rename-entry"|"retain-entry"){Some(workspace.inspect_entry(path,&cancel,&mut |_|{}).unwrap())}else{None};if row["stale"]==true{fs::write(root.join(path),"changed extent").unwrap();}
        let mut progress=|p:Progress|{if(row["cancel"]=="before-publication"&&p.stage==Stage::Publishing)||(row["cancel"]=="after-publication"&&p.stage==Stage::Complete){cancel.store(true,Ordering::Release);}};
        let result=match row["command"].as_str().unwrap(){"create-file"=>workspace.create_file(path,"",&cancel,&mut progress).map(|_|()),"create-directory"=>workspace.create_directory(path,&cancel,&mut progress).map(|_|()),"rename-entry"=>workspace.rename_entry(path,expected.as_ref().unwrap(),destination.unwrap(),&cancel,&mut progress).map(|_|()),"retain-entry"=>workspace.retain_entry(path,expected.as_ref().unwrap(),&cancel,&mut progress).map(|_|()),_=>panic!("Unknown command")};
        let outcome=match result{Ok(())=>"committed",Err(failure)=>match failure.kind{ErrorKind::Conflict=>"conflict",ErrorKind::Cancelled=>"cancelled",_=>panic!("{}: {failure}",row["name"])}};assert_eq!(outcome,row["expected"].as_str().unwrap(),"{}",row["name"]);
    }
}

#[test]
fn source_management_same_size_preserved_mtime_edit_is_reported_after_move(){
    let root=root("management-exact-bytes");let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);fs::write(root.join("a.txt"),"before").unwrap();let expected=workspace.inspect_entry("a.txt",&cancel,&mut |_|{}).unwrap();let before=fs::metadata(root.join("a.txt")).unwrap().modified().unwrap();let mut changed=false;
    let failure=workspace.rename_entry("a.txt",&expected,"b.txt",&cancel,&mut |p|if p.stage==Stage::Publishing&&!changed{fs::write(root.join("a.txt"),"FOREIG").unwrap();let file=OpenOptions::new().write(true).open(root.join("a.txt")).unwrap();file.set_times(std::fs::FileTimes::new().set_modified(before)).unwrap();changed=true;}).unwrap_err();assert_eq!(failure.kind,ErrorKind::Conflict);assert_eq!(fs::read_to_string(root.join("b.txt")).unwrap(),"FOREIG");assert!(failure.recovery_path.is_some());
}

#[test]
fn source_management_actual_receipt_coordinates_require_canonical_u64_text(){
    let root=root("management-canonical");let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);fs::write(root.join("a.txt"),"before").unwrap();let revision=workspace.inspect_entry("a.txt",&cancel,&mut |_|{}).unwrap();let retained=workspace.retain_entry("a.txt",&revision,&cancel,&mut |_|{}).unwrap();
    for phase in ["intent","outcome"]{let receipt=Path::new(&retained.recovery_path).parent().unwrap().join(format!("{}.{phase}",retained.command_id));let text=fs::read_to_string(&receipt).unwrap();for index in 5..=11{let mut rows=text.lines().map(str::to_owned).collect::<Vec<_>>();rows[index]=if index==11{"invalid-fingerprint".into()}else{format!("0{}",rows[index])};fs::write(&receipt,format!("{}\n",rows.join("\n"))).unwrap();assert_eq!(workspace.load_recovery(&retained.recovery_path,&cancel,&mut |_|{}).unwrap_err().kind,ErrorKind::Boundary,"{phase}:{index}");fs::write(&receipt,&text).unwrap();}}
}

#[test]
fn source_management_recovery_authentication_does_not_authorize_a_replacement_object(){
    let root=root("management-recovery-race");let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);fs::write(root.join("a.txt"),"before").unwrap();let revision=workspace.inspect_entry("a.txt",&cancel,&mut |_|{}).unwrap();let retained=workspace.retain_entry("a.txt",&revision,&cancel,&mut |_|{}).unwrap();let mut replaced=false;
    let result=workspace.load_recovery(&retained.recovery_path,&cancel,&mut |p|if p.stage==Stage::Checking&&p.completed==1&&p.total==Some(1)&&!replaced{fs::rename(&retained.recovery_path,root.join("original.txt")).unwrap();fs::write(&retained.recovery_path,"foreign").unwrap();replaced=true;});assert!(replaced);assert_eq!(result.unwrap_err().kind,ErrorKind::Conflict);assert_eq!(fs::read_to_string(root.join("original.txt")).unwrap(),"before");
}

#[test]
fn source_management_content_outcomes_match_shared_system_sha256_vectors(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🔏️fingerprint/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){let root=root("management-digest");let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);let value=row["value"].as_str().unwrap();let unit=if row["encoding"]=="hex"{value.as_bytes().chunks_exact(2).map(|v|u8::from_str_radix(std::str::from_utf8(v).unwrap(),16).unwrap()).collect::<Vec<_>>()}else{value.as_bytes().to_vec()};let bytes=unit.repeat(row["repeat"].as_u64().unwrap()as usize);assert_eq!(bytes.len()as u64,row["bytes"].as_u64().unwrap());fs::write(root.join("a.bin"),&bytes).unwrap();let revision=workspace.inspect_entry("a.bin",&cancel,&mut |_|{}).unwrap();workspace.rename_entry("a.bin",&revision,"b.bin",&cancel,&mut |_|{}).unwrap();let outcome=fs::read_dir(journal(&root)).unwrap().map(|v|v.unwrap().path()).find(|p|p.extension().is_some_and(|e|e=="outcome")).unwrap();let receipt=fs::read_to_string(outcome).unwrap();assert_eq!(receipt.lines().nth(11).unwrap(),row["sha256"].as_str().unwrap(),"{}",row["name"]);assert_eq!(fs::read(root.join("b.bin")).unwrap(),bytes);}
}

#[test]
fn source_management_retained_in_place_content_tampering_is_not_authenticated(){
    let root=root("management-content-proof");let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);fs::write(root.join("a.txt"),"before").unwrap();let revision=workspace.inspect_entry("a.txt",&cancel,&mut |_|{}).unwrap();let retained=workspace.retain_entry("a.txt",&revision,&cancel,&mut |_|{}).unwrap();fs::write(&retained.recovery_path,"FOREIG").unwrap();assert_eq!(workspace.load_recovery(&retained.recovery_path,&cancel,&mut |_|{}).unwrap_err().kind,ErrorKind::Conflict);
}

#[test]
fn source_management_streamed_content_cancellation_preserves_original_before_effect(){
    let root=root("management-stream-cancel");let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);fs::write(root.join("a.bin"),vec![7u8;MAXIMUM_FILE_BYTES+1]).unwrap();let revision=workspace.inspect_entry("a.bin",&cancel,&mut |_|{}).unwrap();let failure=workspace.rename_entry("a.bin",&revision,"b.bin",&cancel,&mut |p|if p.stage==Stage::Reading&&p.completed>=16384{cancel.store(true,Ordering::Release)}).unwrap_err();assert_eq!(failure.kind,ErrorKind::Cancelled);assert!(root.join("a.bin").is_file());assert!(!root.join("b.bin").exists());
}

#[test]
fn source_management_retained_tree_history_browses_and_reads_authenticated_descendants(){
    let root=root("management-history-tree");let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);assert!(workspace.list_recovery("",&cancel,&mut |_|{}).unwrap().is_empty());fs::create_dir_all(root.join("folder/nested")).unwrap();fs::write(root.join("folder/nested/Grüße.txt"),"retained 🦑").unwrap();let revision=workspace.inspect_entry("folder",&cancel,&mut |_|{}).unwrap();let renamed=workspace.rename_entry("folder",&revision,"renamed",&cancel,&mut |_|{}).unwrap();let revision=workspace.inspect_entry(&renamed.path,&cancel,&mut |_|{}).unwrap();let retained=workspace.retain_entry(&renamed.path,&revision,&cancel,&mut |_|{}).unwrap();let roots=workspace.list_recovery("",&cancel,&mut |_|{}).unwrap();assert_eq!(roots.len(),1);assert_eq!(roots[0].name,"renamed");assert_eq!(roots[0].path,retained.recovery_path);let child=workspace.list_recovery(&roots[0].path,&cancel,&mut |_|{}).unwrap();assert_eq!(child.len(),1);assert_eq!(child[0].name,"nested");let leaf=workspace.list_recovery(&child[0].path,&cancel,&mut |_|{}).unwrap();assert_eq!(leaf[0].name,"Grüße.txt");let document=workspace.load_recovery(&leaf[0].path,&cancel,&mut |_|{}).unwrap();assert_eq!(document.text,"retained 🦑");assert!(workspace.save(&document.path,&document.revision,"forbidden",&cancel,&mut |_|{}).is_err());assert_eq!(workspace.list_recovery("../foreign",&cancel,&mut |_|{}).unwrap_err().kind,ErrorKind::Boundary);workspace.create_file("after.txt","after",&cancel,&mut |_|{}).unwrap();
}

#[test]
fn source_management_postmove_changed_descendant_is_retained_as_actual_tree(){
    let root=root("management-tree-race");let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);fs::create_dir(root.join("folder")).unwrap();fs::write(root.join("folder/a.txt"),"before").unwrap();let revision=workspace.inspect_entry("folder",&cancel,&mut |_|{}).unwrap();let mut changed=false;let failure=workspace.retain_entry("folder",&revision,&cancel,&mut |p|if p.stage==Stage::Publishing&&!changed{fs::write(root.join("folder/a.txt"),"foreign").unwrap();changed=true;}).unwrap_err();assert_eq!(failure.kind,ErrorKind::Conflict);let recovery=failure.recovery_path.unwrap();let children=workspace.list_recovery(&recovery,&cancel,&mut |_|{}).unwrap();assert_eq!(workspace.load_recovery(&children[0].path,&cancel,&mut |_|{}).unwrap().text,"foreign");workspace.create_file("after.txt","after",&cancel,&mut |_|{}).unwrap();
}

#[test]
fn source_management_tree_node_limit_is_total_and_cancellable(){
    let root=root("management-tree-bound");let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);fs::create_dir(root.join("folder")).unwrap();for index in 0..MAXIMUM_DIRECTORY_ENTRIES-1{fs::write(root.join("folder").join(format!("n{index:04}")),[]).unwrap();}workspace.inspect_entry("folder",&cancel,&mut |_|{}).unwrap();fs::write(root.join("folder/overflow"),[]).unwrap();assert_eq!(workspace.inspect_entry("folder",&cancel,&mut |_|{}).unwrap_err().kind,ErrorKind::TooManyEntries);fs::remove_file(root.join("folder/overflow")).unwrap();assert_eq!(workspace.inspect_entry("folder",&cancel,&mut |p|if p.stage==Stage::Checking&&p.completed>=10{cancel.store(true,Ordering::Release)}).unwrap_err().kind,ErrorKind::Cancelled);assert!(root.join("folder/n0000").exists());
}

#[test]
fn source_management_projected_descendant_path_limit_refuses_before_move(){
    let root=root("management-tree-path");let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);fs::create_dir(root.join("a")).unwrap();let count=MAXIMUM_PATH_UNITS-root.to_string_lossy().encode_utf16().count()-3;let name="x".repeat(count);fs::write(root.join("a").join(&name),"before").unwrap();let revision=workspace.inspect_entry("a",&cancel,&mut |_|{}).unwrap();assert_eq!(workspace.rename_entry("a",&revision,"longer",&cancel,&mut |_|{}).unwrap_err().kind,ErrorKind::InvalidPath);assert_eq!(fs::read_to_string(root.join("a").join(&name)).unwrap(),"before");assert!(!root.join("longer").exists());
}

#[cfg(unix)]
#[test]
fn source_management_tree_and_history_never_follow_foreign_links(){
    use std::os::unix::fs::symlink;
    let root=root("management-tree-link");let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);fs::create_dir(root.join("folder")).unwrap();fs::write(root.join("outside.txt"),"outside").unwrap();symlink(root.join("outside.txt"),root.join("folder/link")).unwrap();assert_eq!(workspace.inspect_entry("folder",&cancel,&mut |_|{}).unwrap_err().kind,ErrorKind::Boundary);assert_eq!(fs::read_to_string(root.join("outside.txt")).unwrap(),"outside");
}

#[test]
fn source_management_projected_roster_matches_all_shared_tree_vectors(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();for row in fixture["trees"].as_array().unwrap(){let mut nodes=row["nodes"].as_array().unwrap().iter().map(|value|TreeNode{suffix:value["suffix"].as_str().unwrap().into(),directory:value["directory"].as_bool().unwrap(),stamp:Stamp{identity:(1,1),length:0,modified:None,change:(0,0)}}).collect::<Vec<_>>();while nodes.len()<row["nodeCount"].as_u64().unwrap()as usize{nodes.push(TreeNode{suffix:format!("n{:05}",nodes.len()),directory:false,stamp:Stamp{identity:(1,1),length:0,modified:None,change:(0,0)}});}let result=crate::management::validate_tree_projection(row["physicalRootUnits"].as_u64().unwrap()as usize,row["destination"].as_str().unwrap(),&nodes);let outcome=match result{Ok(())=>"accepted",Err(failure)=>match failure.kind{ErrorKind::InvalidPath=>"invalid-path",ErrorKind::TooManyEntries=>"too-many-entries",ErrorKind::Conflict=>"conflict",_=>panic!("Unexpected projection refusal: {failure}")}};assert_eq!(outcome,row["expected"].as_str().unwrap(),"{}",row["name"]);}
}

#[test]
fn source_management_managed_file_extent_refuses_before_effect(){
    let root=root("management-file-bound");let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);let file=File::create(root.join("large.bin")).unwrap();file.set_len(MAXIMUM_MANAGED_FILE_BYTES+1).unwrap();drop(file);let revision=workspace.inspect_entry("large.bin",&cancel,&mut |_|{}).unwrap();let failure=workspace.rename_entry("large.bin",&revision,"moved.bin",&cancel,&mut |_|{}).unwrap_err();assert_eq!(failure.kind,ErrorKind::TooLarge);assert!(root.join("large.bin").is_file());assert!(!root.join("moved.bin").exists());
}
