//! 🧪️ The shared portable contract and actual physical source-command laws.

use super::*;
use serde_json::Value;

fn fixture() -> Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn root(name:&str)->PathBuf {
    let output=std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("Explicit ticket artifact directory required");
    let root=PathBuf::from(output).join(format!("source-{name}-{}-{}",std::process::id(),SERIAL.fetch_add(1,Ordering::Relaxed)));
    fs::create_dir_all(&root).unwrap();root
}
fn code(error:ErrorKind)->&'static str { match error {ErrorKind::InvalidPath=>"invalid-path",ErrorKind::NotText=>"not-text",ErrorKind::TooLarge=>"too-large",_=>"other"} }

#[test]
fn source_workspace_shared_portable_contract() {
    let rows=fixture();
    assert_eq!(rows["policy"]["maximumFileBytes"].as_u64(),Some(MAXIMUM_FILE_BYTES as u64));
    assert_eq!(rows["policy"]["maximumDirectoryEntries"].as_u64(),Some(MAXIMUM_DIRECTORY_ENTRIES as u64));
    assert_eq!(rows["policy"]["maximumPathUnits"].as_u64(),Some(MAXIMUM_PATH_UNITS as u64));
    assert_eq!(rows["policy"]["maximumJournalBytes"].as_u64(),Some(MAXIMUM_JOURNAL_BYTES));
    for row in rows["cases"].as_array().unwrap(){
        let result=validate_relative(row["path"].as_str().unwrap()).and_then(|_|validate_text(row["text"].as_str().unwrap()));
        assert_eq!(result.is_ok(),row["valid"].as_bool().unwrap(),"{}",row["name"]);
        if let Err(error)=result{assert_eq!(code(error.kind),row["code"].as_str().unwrap(),"{}",row["name"]);}
    }
}

#[test]
fn source_workspace_unicode_save_reopen_and_retained_event_predecessor() {
    let root=root("save");let name="Grüße 🦑.txt";let before="first\r\n🦑\r\n";fs::write(root.join(name),before).unwrap();
    let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);let mut events=Vec::new();
    let loaded=workspace.load(name,&cancel,&mut |p|events.push(p.stage)).unwrap();
    let saved=workspace.save(name,&loaded.revision,"second\r\n🦑\r\n",&cancel,&mut |p|events.push(p.stage)).unwrap();
    assert_eq!(saved.text,"second\r\n🦑\r\n");assert_ne!(saved.revision,loaded.revision);
    assert_eq!(workspace.load(name,&cancel,&mut |_|{}).unwrap().text,saved.text);
    let history=fs::read_dir(root.join(STATE)).unwrap().map(|e|e.unwrap().path()).collect::<Vec<_>>();
    let outcome=history.iter().find(|p|p.extension().is_some_and(|e|e=="outcome")).unwrap();let receipt=fs::read_to_string(outcome).unwrap();let prior=PathBuf::from(receipt.lines().nth(2).unwrap());assert_eq!(fs::read_to_string(&prior).unwrap(),before);
    let recovered=workspace.load_recovery(prior.to_str().unwrap(),&cancel,&mut |_|{}).unwrap();assert_eq!(recovered.text,before);assert!(workspace.save(&recovered.path,&recovered.revision,"forbidden",&cancel,&mut |_|{}).is_err());
    assert!(history.iter().any(|p|p.extension().is_some_and(|e|e=="outcome")));
    assert!(events.contains(&Stage::Staging)&&events.contains(&Stage::Publishing)&&events.contains(&Stage::Complete));
}

#[test]
fn source_workspace_foreign_revision_and_final_checkpoint_edits_are_preserved() {
    for final_checkpoint in [false,true]{
        let root=root("conflict");fs::write(root.join("a.txt"),"before").unwrap();let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);
        let doc=workspace.load("a.txt",&cancel,&mut |_|{}).unwrap();
        if !final_checkpoint{fs::write(root.join("a.txt"),"foreign").unwrap();}
        let mut changed=false;
        let result=workspace.save("a.txt",&doc.revision,"local",&cancel,&mut |p|{if final_checkpoint&&p.stage==Stage::Publishing&&!changed{fs::write(root.join("a.txt"),"foreign").unwrap();changed=true;}});
        assert_eq!(result.unwrap_err().kind,ErrorKind::Conflict);assert_eq!(fs::read_to_string(root.join("a.txt")).unwrap(),"foreign");
    }
}

#[test]
fn source_workspace_cancellation_at_publication_never_publishes() {
    let root=root("cancel");fs::write(root.join("a.txt"),"before").unwrap();let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);let doc=workspace.load("a.txt",&cancel,&mut |_|{}).unwrap();
    let result=workspace.save("a.txt",&doc.revision,"local",&cancel,&mut |p|{if p.stage==Stage::Publishing{cancel.store(true,Ordering::Release);}});
    assert_eq!(result.unwrap_err().kind,ErrorKind::Cancelled);assert_eq!(fs::read_to_string(root.join("a.txt")).unwrap(),"before");
    assert!(!fs::read_dir(root.join(STATE)).unwrap().any(|p|p.unwrap().path().extension().is_some_and(|e|e=="new")));
}

#[test]
fn source_workspace_committed_save_survives_late_cancellation() {
    let root=root("late");fs::write(root.join("a.txt"),"before").unwrap();let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);let doc=workspace.load("a.txt",&cancel,&mut |_|{}).unwrap();
    let saved=workspace.save("a.txt",&doc.revision,"local",&cancel,&mut |p|{if p.stage==Stage::Complete{cancel.store(true,Ordering::Release);}}).unwrap();assert_eq!(saved.text,"local");assert_eq!(fs::read_to_string(root.join("a.txt")).unwrap(),"local");
}

#[test]
fn source_workspace_partial_publication_retains_readonly_recovery() {
    let root=root("partial");fs::write(root.join("a.txt"),"foreign predecessor").unwrap();let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);let doc=workspace.load("a.txt",&cancel,&mut |_|{}).unwrap();let mut blocked=false;
    let result=workspace.save("a.txt",&doc.revision,"local",&cancel,&mut |p|{if p.stage==Stage::Publishing&&!blocked{
        let intent=fs::read_dir(root.join(STATE)).unwrap().map(|e|e.unwrap().path()).find(|p|p.extension().is_some_and(|e|e=="intent")).unwrap();fs::create_dir(intent.with_extension("outcome")).unwrap();blocked=true;
    }}).unwrap_err();assert_eq!(result.kind,ErrorKind::Io);assert_eq!(fs::read_to_string(root.join("a.txt")).unwrap(),"local");let recovered=workspace.load_recovery(result.recovery_path.as_deref().unwrap(),&cancel,&mut |_|{}).unwrap();assert_eq!(recovered.text,"foreign predecessor");
}

#[test]
fn source_workspace_read_limits_binary_and_foreign_revision_refuse() {
    let root=root("limits");fs::write(root.join("a.txt"),"before").unwrap();fs::write(root.join("binary"),[0xff,0xfe]).unwrap();let large=File::create(root.join("large")).unwrap();large.set_len(MAXIMUM_FILE_BYTES as u64+1).unwrap();
    let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);assert_eq!(workspace.load("binary",&cancel,&mut |_|{}).unwrap_err().kind,ErrorKind::NotText);assert_eq!(workspace.load("large",&cancel,&mut |_|{}).unwrap_err().kind,ErrorKind::TooLarge);
    let doc=workspace.load("a.txt",&cancel,&mut |_|{}).unwrap();fs::write(root.join("b.txt"),"before").unwrap();assert_eq!(workspace.save("b.txt",&doc.revision,"local",&cancel,&mut |_|{}).unwrap_err().kind,ErrorKind::Conflict);
}

#[test]
fn source_workspace_browser_is_ordered_and_cancellable() {
    let root=root("browser");fs::create_dir(root.join("folder")).unwrap();fs::write(root.join("z.txt"),"z").unwrap();fs::write(root.join("a.txt"),"a").unwrap();let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);
    let rows=workspace.list("",&cancel,&mut |_|{}).unwrap();assert_eq!(rows.iter().map(|e|e.name.as_str()).collect::<Vec<_>>(),["folder","a.txt","z.txt"]);assert_eq!(workspace.list("",&cancel,&mut |_|{}).unwrap(),rows);
    let result=workspace.list("",&cancel,&mut |_|cancel.store(true,Ordering::Release));assert_eq!(result.unwrap_err().kind,ErrorKind::Cancelled);
}

#[cfg(unix)]
#[test]
fn source_workspace_never_follows_source_or_store_links() {
    use std::os::unix::fs::symlink;
    let outside=root("outside");let root=root("links");fs::write(outside.join("foreign"),"untouched").unwrap();symlink(outside.join("foreign"),root.join("link")).unwrap();symlink(&outside,root.join(STATE)).unwrap();fs::write(root.join("a.txt"),"before").unwrap();let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);
    assert_eq!(workspace.load("link",&cancel,&mut |_|{}).unwrap_err().kind,ErrorKind::Boundary);let doc=workspace.load("a.txt",&cancel,&mut |_|{}).unwrap();assert_eq!(workspace.save("a.txt",&doc.revision,"local",&cancel,&mut |_|{}).unwrap_err().kind,ErrorKind::Boundary);assert_eq!(fs::read_to_string(outside.join("foreign")).unwrap(),"untouched");
}

#[cfg(unix)]
#[test]
fn source_workspace_ancestry_replacement_cannot_redirect_a_save() {
    use std::os::unix::fs::symlink;
    let outside=root("foreign");fs::write(outside.join("a.txt"),"foreign").unwrap();let root=root("ancestry");fs::create_dir(root.join("folder")).unwrap();fs::write(root.join("folder/a.txt"),"before").unwrap();let workspace=Workspace::new(&root).unwrap();let cancel=AtomicBool::new(false);let doc=workspace.load("folder/a.txt",&cancel,&mut |_|{}).unwrap();let mut replaced=false;
    let result=workspace.save("folder/a.txt",&doc.revision,"local",&cancel,&mut |p|{if p.stage==Stage::Publishing&&!replaced{fs::rename(root.join("folder"),root.join("moved")).unwrap();symlink(&outside,root.join("folder")).unwrap();replaced=true;}});
    assert!(result.is_err());assert_eq!(fs::read_to_string(outside.join("a.txt")).unwrap(),"foreign");assert_eq!(fs::read_to_string(root.join("moved/a.txt")).unwrap(),"before");
}
