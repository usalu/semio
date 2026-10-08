//! 🧪️ Native paged diagnostics share the neutral UTF8/JSON authority and measured physical grants.
use super::*;
use semio_framework_value::paged::PagedUtf8;
use semio_framework_trace::observe_heap_allocations_on_this_thread;

enum Fragment { Text(PagedUtf8<{usize::MAX}>), Static(&'static str), Quoted(PagedUtf8<{usize::MAX}>), Unsigned(u64) }
struct Source { level: Severity, code: &'static str, fragments: Vec<Fragment>, targets: Vec<PagedUtf8<{usize::MAX}>> }
impl ArtifactMessageSource for Source {
    fn level(&self) -> Severity { self.level }
    fn code(&self) -> &'static str { self.code }
    fn operation_index(&self) -> Option<u32> { Some(7) }
    fn fragment_count(&self) -> usize { self.fragments.len() }
    fn fragment(&self, index: usize) -> Option<ArtifactMessageFragment<'_>> {
        self.fragments.get(index).map(|fragment| match fragment {
            Fragment::Text(text) => ArtifactMessageFragment::Text(text),
            Fragment::Static(text) => ArtifactMessageFragment::Static(text),
            Fragment::Quoted(text) => ArtifactMessageFragment::JsonQuoted(text),
            Fragment::Unsigned(value) => ArtifactMessageFragment::Unsigned(*value),
        })
    }
    fn target_count(&self) -> usize { self.targets.len() }
    fn target(&self, index: usize) -> Option<&dyn Utf8Text> { self.targets.get(index).map(|target| target as &dyn Utf8Text) }
}
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn source(row: &serde_json::Value) -> Source {
    Source {
        level: match row["level"].as_str().unwrap() { "fatal"=>Severity::Fatal, "error"=>Severity::Error, "warning"=>Severity::Warning, _=>Severity::Info },
        code: match row["code"].as_str().unwrap() { "mutation.invariant"=>"mutation.invariant", "mutation.target-missing"=>"mutation.target-missing", "mutation.no-op"=>"mutation.no-op", "mutation.partial-selection"=>"mutation.partial-selection", _=>"mutation.cascade" },
        fragments: row["fragments"].as_array().unwrap().iter().map(|part| {
            let text=part["text"].as_str().unwrap().repeat(part["repeat"].as_u64().unwrap() as usize);
            match part["kind"].as_str().unwrap() {
                "quoted"=>Fragment::Quoted(text.into()), "unsigned"=>Fragment::Unsigned(text.parse().unwrap()), "static"=>Fragment::Static(match text.as_str(){"node \""=>"node \"","\" not found"=>"\" not found","invalid shape "=>"invalid shape ","no changes to apply"=>"no changes to apply","not found"=>"not found"," of "=>" of "," selected"=>" selected",_=>panic!("neutral static fragment missing literal authority")}), _=>Fragment::Text(text.into()),
            }
        }).collect(),
        targets: row["targets"].as_array().unwrap().iter().map(|target| target["text"].as_str().unwrap().repeat(target["repeat"].as_u64().unwrap() as usize).into()).collect(),
    }
}
fn close(cursor: &mut ArtifactMessageComposeCursor) {
    cursor.begin_close();
    for _ in 0..1000 {
        let demand=cursor.next_release_byte_demand().unwrap();
        if demand>0 {
            let (blocked,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:demand-1,maximum_depth:cursor.next_depth_demand()}).unwrap());
            assert_eq!(blocked.progress(),RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            assert_eq!(cursor.next_release_byte_demand().unwrap(),demand);
        }
        let (step,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(RetainedCloneGrant::one_release_turn(demand,cursor.next_depth_demand())).unwrap());
        assert_eq!(heap.requested_bytes,0);
        assert_eq!(heap.released_bytes,demand);
        if matches!(step,RetainedCloneStep::Complete(_)) { assert!(cursor.terminal_is_empty()); return; }
    }
    panic!("bounded diagnostic owner did not close");
}

#[test]
fn borrowed_message_composition_matches_neutral_and_serde_json() {
    let law=fixture();
    for row in law["cases"].as_array().unwrap() {
        let source=source(row);
        for grant in law["copyGrants"].as_array().unwrap() {
            let (mut cursor,heap)=observe_heap_allocations_on_this_thread(||ArtifactMessageComposeCursor::new(row["limit"].as_u64().unwrap() as usize));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            for turn in 0..20000 {
                let capacity=cursor.next_capacity_byte_demand(&source).unwrap();
                let copied=(grant.as_u64().unwrap() as usize).max(cursor.next_copy_byte_demand(&source).unwrap());
                if capacity>0 {
                    let (blocked,heap)=observe_heap_allocations_on_this_thread(||cursor.advance(&source,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:capacity-1,maximum_release_bytes:4096,maximum_depth:1}).unwrap());
                    assert_eq!(blocked.progress(),RetainedCloneProgress::default());
                    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                    assert_eq!(cursor.next_capacity_byte_demand(&source).unwrap(),capacity);
                }
                let (step,heap)=observe_heap_allocations_on_this_thread(||cursor.advance(&source,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copied,maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:1}).unwrap());
                let progress=match step { RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress };
                assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes,"{} turn{turn}",row["name"]);
                assert_eq!(heap.released_bytes,0);
                assert!(progress.copied_items<=1 && progress.copied_bytes<=copied && progress.retained_capacity_bytes<=capacity);
                if matches!(step,RetainedCloneStep::Complete(_)) { break; }
                assert!(turn<19999,"{} incomplete",row["name"]);
            }
            assert_eq!(cursor.truncated(),row["expected"]["truncated"].as_bool().unwrap());
            let message=cursor.take().unwrap();
            assert_eq!(message.level,source.level);
            assert_eq!(message.code.0,source.code);
            assert_eq!(message.op_index,Some(7));
            assert_eq!(message.message,row["expected"]["text"].as_str().unwrap(),"{}",row["name"]);
            let expected:Vec<_>=row["expected"]["targetIndices"].as_array().unwrap().iter().map(|index|source.targets[index.as_u64().unwrap() as usize].to_string_owner()).collect();
            assert_eq!(message.target,expected,"{}",row["name"]);
            let entry=MUTATION_MESSAGE_OWNER_BYTES+message.code.0.len()+message.message.len()+message.target.iter().map(|target|MUTATION_MESSAGE_TARGET_BYTES+target.len()).sum::<usize>();
            assert_eq!(entry,row["expected"]["entryBytes"].as_u64().unwrap() as usize);
            assert!(entry<=row["limit"].as_u64().unwrap() as usize);
            close(&mut cursor);
        }
        for fragment in &source.fragments {
            if let Fragment::Quoted(text)=fragment {
                let projected=text.to_string_owner();
                let quoted=serde_json::to_string(&projected).unwrap();
                assert!(row["expected"]["text"].as_str().unwrap().contains(&quoted) || row["expected"]["truncated"].as_bool().unwrap());
            }
        }
    }
    println!("[DEBUG] borrowed native diagnostic rows={} match neutral/Serde JSON and observed physical grant allocation",law["cases"].as_array().unwrap().len());
}

#[test]
fn borrowed_message_composition_cancels_every_partial_owner_and_rejects_source_swap() {
    let law=fixture();
    let source=source(&law["cases"][2]);
    let other=Source{level:source.level,code:source.code,fragments:Vec::new(),targets:Vec::new()};
    for stop in law["cancelTurns"].as_array().unwrap() {
        let mut cursor=ArtifactMessageComposeCursor::new(256);
        for _ in 0..stop.as_u64().unwrap() {
            let capacity=cursor.next_capacity_byte_demand(&source).unwrap();
            let copied=7.max(cursor.next_copy_byte_demand(&source).unwrap());
            if matches!(cursor.advance(&source,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copied,maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:1}).unwrap(),RetainedCloneStep::Complete(_)){break;}
        }
        if stop.as_u64().unwrap()>0 {
            let (refusal,heap)=observe_heap_allocations_on_this_thread(||cursor.advance(&other,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:0,maximum_depth:1}));
            assert_eq!(refusal,Err(ArtifactMessageComposeRefusal::SourceChanged));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        }
        cursor.cancel();
        assert!(cursor.take().is_none());
        close(&mut cursor);
        let (_,heap)=observe_heap_allocations_on_this_thread(||drop(cursor));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    }
    println!("[DEBUG] borrowed diagnostics cancellation/source-swap preserve baseline and retire all exact partial output owners");
}
