//! 🧪️ Laws of the range-text operation: the Rust twin replays the language-neutral `semio.ui.scene.text-splice.v1` fixture
//! exactly as the TS twin does (edit → splice, application + inverse, concurrent folds in hub order, host rebase, UTF-8 ↔ scalar).
use super::{rebase_text_edits, TextSplice, TextSpliceComposition, TEXT_EDITOR_TYPING_BUFFER_ARG, TEXT_EDITOR_TYPING_COMMIT_ARG, TEXT_EDITOR_TYPING_IDLE_MS, TEXT_SPLICE_CONTEXT_SCALARS, TEXT_SPLICE_MIN_TWO_SIDED_SCALARS};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/✂️text-splice/🔣️.json")).expect("text splice fixture")
}

fn splice(value: &Value) -> TextSplice {
    serde_json::from_value(value.clone()).expect("fixture splice")
}

#[test]
fn the_fixture_pins_the_context_constants() {
    let fixture = fixture();
    assert_eq!(fixture["schema"], "semio.ui.scene.text-splice.v1");
    assert_eq!(fixture["contextScalars"], TEXT_SPLICE_CONTEXT_SCALARS);
    assert_eq!(fixture["minTwoSidedScalars"], TEXT_SPLICE_MIN_TWO_SIDED_SCALARS);
}

#[test]
fn every_edit_yields_the_fixture_splice_and_round_trips() {
    for row in fixture()["edits"].as_array().unwrap() {
        let (previous, next) = (row["previous"].as_str().unwrap(), row["next"].as_str().unwrap());
        let derived = TextSplice::from_edit(previous, next, TEXT_SPLICE_CONTEXT_SCALARS);
        let expected = if row["splice"].is_null() { None } else { Some(splice(&row["splice"])) };
        assert_eq!(derived, expected, "edit {}", row["id"]);
        if let Some(derived) = derived {
            assert_eq!(derived.splice_into(previous, TEXT_SPLICE_CONTEXT_SCALARS).text, next, "edit {} round trip", row["id"]);
        }
    }
}

#[test]
fn every_application_lands_results_and_inverts_like_the_fixture() {
    for row in fixture()["applications"].as_array().unwrap() {
        let (text, id) = (row["text"].as_str().unwrap(), &row["id"]);
        let applied = splice(&row["splice"]).splice_into(text, TEXT_SPLICE_CONTEXT_SCALARS);
        assert_eq!(serde_json::to_value(applied.located).unwrap(), row["located"], "apply {id} located");
        assert_eq!(applied.text, row["result"].as_str().unwrap(), "apply {id} result");
        assert_eq!(applied.inverse, splice(&row["inverse"]), "apply {id} inverse");
        if !applied.located.clamped {
            assert_eq!(applied.inverse.splice_into(&applied.text, TEXT_SPLICE_CONTEXT_SCALARS).text, text, "apply {id} undo");
        }
    }
}

#[test]
fn every_concurrent_fold_in_hub_order_matches_the_fixture() {
    for row in fixture()["concurrent"].as_array().unwrap() {
        let mut text = row["base"].as_str().unwrap().to_string();
        let mut clamped = Vec::new();
        for (index, entry) in row["order"].as_array().unwrap().iter().enumerate() {
            let applied = splice(&entry["splice"]).splice_into(&text, TEXT_SPLICE_CONTEXT_SCALARS);
            if applied.located.clamped {
                clamped.push(index);
            } else {
                assert_eq!(applied.inverse.splice_into(&applied.text, TEXT_SPLICE_CONTEXT_SCALARS).text, text, "concurrent {} #{index} undo", row["id"]);
            }
            text = applied.text;
        }
        assert_eq!(text, row["expected"]["text"].as_str().unwrap(), "concurrent {}", row["id"]);
        assert_eq!(serde_json::to_value(&clamped).unwrap(), row["expected"]["clamped"], "concurrent {} clamped", row["id"]);
    }
}

fn composition_json(composition: &TextSpliceComposition) -> Value {
    match composition {
        TextSpliceComposition::Composed(splice) => serde_json::json!({ "kind": "composed", "splice": splice }),
        TextSpliceComposition::Cancelled => serde_json::json!({ "kind": "cancelled" }),
        TextSpliceComposition::Disjoint => serde_json::json!({ "kind": "disjoint" }),
    }
}

/// ⚖️ LAW: a typed splice joins its run exactly like the TS twin composes it — the union of both contiguous changes, a run
/// that changes nothing cancelled, a caret that jumped away disjoint — and the composed splice takes `T0` straight to `T2`.
#[test]
fn every_composition_joins_its_run_like_the_fixture() {
    for row in fixture()["compositions"].as_array().unwrap() {
        let (net, next) = (splice(&row["net"]), splice(&row["next"]));
        let composition = net.then(&next, TEXT_SPLICE_CONTEXT_SCALARS);
        assert_eq!(composition_json(&composition), row["composition"], "compose {}", row["id"]);
        let text = row["text"].as_str().unwrap();
        let typed = next.splice_into(&net.splice_into(text, TEXT_SPLICE_CONTEXT_SCALARS).text, TEXT_SPLICE_CONTEXT_SCALARS).text;
        match composition {
            TextSpliceComposition::Composed(splice) => assert_eq!(splice.splice_into(text, TEXT_SPLICE_CONTEXT_SCALARS).text, typed, "compose {} lands", row["id"]),
            TextSpliceComposition::Cancelled => assert_eq!(typed, text, "compose {} cancels", row["id"]),
            TextSpliceComposition::Disjoint => {}
        }
    }
}

/// ⚖️ LAW: one author typing through `texts` folds the `from_edit` splice of every step into exactly the fixture's runs — a
/// disjoint step starts a new run, a cancelled run leaves none — and the runs take the first text to the last.
#[test]
fn every_typing_session_folds_into_the_fixture_runs() {
    for row in fixture()["typingRuns"].as_array().unwrap() {
        let texts: Vec<&str> = row["texts"].as_array().unwrap().iter().map(|text| text.as_str().unwrap()).collect();
        let (mut runs, mut net): (Vec<TextSplice>, Option<TextSplice>) = (Vec::new(), None);
        for step in texts.windows(2) {
            let Some(next) = TextSplice::from_edit(step[0], step[1], TEXT_SPLICE_CONTEXT_SCALARS) else { continue };
            net = match net.take().map(|net| (net.then(&next, TEXT_SPLICE_CONTEXT_SCALARS), net)) {
                None => Some(next),
                Some((TextSpliceComposition::Composed(splice), _)) => Some(splice),
                Some((TextSpliceComposition::Cancelled, _)) => None,
                Some((TextSpliceComposition::Disjoint, previous)) => {
                    runs.push(previous);
                    Some(next)
                }
            };
        }
        runs.extend(net);
        let expected: Vec<TextSplice> = row["runs"].as_array().unwrap().iter().map(splice).collect();
        assert_eq!(runs, expected, "typing run {}", row["id"]);
        assert_eq!(runs.iter().fold(texts[0].to_string(), |text, run| run.splice_into(&text, TEXT_SPLICE_CONTEXT_SCALARS).text), *texts.last().unwrap(), "typing run {} lands", row["id"]);
    }
}

/// ⚖️ LAW: the host's typing-run protocol constants are the tool-machine owner's (typing-law fixture).
#[test]
fn the_typing_protocol_constants_are_the_tool_machine_owners() {
    let law: Value = serde_json::from_str(include_str!("../../../../🛠️tool-machine/🧫️fixtures/🧫️typing-law/🔣️.json")).expect("typing law");
    assert_eq!((law["args"]["buffer"].as_str(), law["args"]["commit"].as_str(), law["idleMs"].as_u64()), (Some(TEXT_EDITOR_TYPING_BUFFER_ARG), Some(TEXT_EDITOR_TYPING_COMMIT_ARG), Some(TEXT_EDITOR_TYPING_IDLE_MS)));
}

/// ⚖️ LAW: every host signal of the corpus appears once and ends the run with a reason the tool machine knows (typing-law
/// `reasons`), so a host commit signal is never refused.
#[test]
fn every_host_signal_commits_with_a_tool_machine_reason() {
    let law: Value = serde_json::from_str(include_str!("../../../../🛠️tool-machine/🧫️fixtures/🧫️typing-law/🔣️.json")).expect("typing law");
    let reasons: Vec<&str> = law["reasons"].as_array().unwrap().iter().filter_map(Value::as_str).collect();
    let rows = fixture()["hostSignals"].as_array().unwrap().clone();
    let mut signals: Vec<&str> = rows.iter().map(|row| row["signal"].as_str().unwrap()).collect();
    signals.sort_unstable();
    signals.dedup();
    assert_eq!(signals.len(), rows.len(), "every host signal appears once");
    for row in &rows {
        assert!(reasons.contains(&row["commit"].as_str().unwrap()), "host signal {} commits with an unknown reason", row["signal"]);
    }
}

#[test]
fn every_host_rebase_matches_the_fixture() {
    for row in fixture()["rebases"].as_array().unwrap() {
        let unapplied: Vec<TextSplice> = row["unapplied"].as_array().unwrap().iter().map(splice).collect();
        let selection = &row["selection"];
        let (text, anchor, caret) = rebase_text_edits(row["remote"].as_str().unwrap(), &unapplied, row["local"].as_str().unwrap(), selection["anchor"].as_u64().unwrap() as usize, selection["caret"].as_u64().unwrap() as usize);
        assert_eq!((text.as_str(), anchor as u64, caret as u64), (row["expected"]["text"].as_str().unwrap(), row["expected"]["anchor"].as_u64().unwrap(), row["expected"]["caret"].as_u64().unwrap()), "rebase {}", row["id"]);
    }
}

#[test]
fn utf8_offsets_and_scalar_indices_agree_with_the_fixture() {
    for row in fixture()["offsets"].as_array().unwrap() {
        let text = row["text"].as_str().unwrap();
        let bytes = row["utf8"].as_u64().unwrap() as usize;
        let scalar = text.char_indices().take_while(|(offset, _)| *offset < bytes).count();
        assert_eq!(scalar as u64, row["scalar"].as_u64().unwrap(), "offset {text:?}@{bytes}");
    }
}

#[test]
fn draft_changes_json_preserves_neutral_ranges_and_matches_independent_serde_oracle() {
 for row in fixture()["draftJson"].as_array().unwrap() {
  let changes: Vec<super::DraftChange> = serde_json::from_value(row["changes"].clone()).unwrap();
  let mut observe=|_|true; let mut control=protocol::value::NativeEncodeControl::new(0,&mut observe); let mut bytes=Vec::new(); super::write_draft_changes_json_into(&changes,1024,3,64,&mut |part:&[u8],_:&mut protocol::value::NativeEncodeControl<'_>|->Result<(),protocol::value::ValueError>{bytes.extend_from_slice(part);Ok(())},&mut control).unwrap(); let actual=String::from_utf8(bytes).unwrap();
  assert_eq!(actual, row["json"].as_str().unwrap()); assert_eq!(actual, serde_json::to_string(&changes).unwrap());
  println!("[DEBUG] draft JSON native id={} bytes={} independentSerde=true",row["id"].as_str().unwrap(),actual.len());
 }
}

#[test]
fn draft_wire_borrows_original_changes_and_retains_refusal_prefixes() {
    use protocol::value::{NativeEncodeControl,ValueError,ValueRefusalKind};
    let fixture=fixture();
    for row in fixture["draftJson"].as_array().unwrap() {
        let changes:Vec<super::DraftChange>=serde_json::from_value(row["changes"].clone()).unwrap();
        let expected=serde_json::to_string(&changes).unwrap();
        assert_eq!(expected,row["json"].as_str().unwrap());
        for maximum in [0,1,expected.len().saturating_sub(1),expected.len(),1024] {
            let mut callbacks=0;
            let mut observe=|_|{callbacks+=1;true};
            let mut control=NativeEncodeControl::new(0,&mut observe);
            let mut output=Vec::new();
            let result=super::write_draft_changes_json_into(&changes,maximum as u64,3,64,&mut |bytes:&[u8],_:&mut NativeEncodeControl<'_>|->Result<(),ValueError>{output.extend_from_slice(bytes);Ok(())},&mut control);
            assert!(expected.as_bytes().starts_with(&output));
            if maximum>=expected.len() {assert_eq!(result.unwrap(),expected.len() as u64);assert_eq!(output,expected.as_bytes());}
            else {assert_eq!(result.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);}
            assert_eq!(control.owned_bytes(),0);
            assert!(callbacks>0);
        }
        for cancel_at in [0,1,3,9] {
            let mut callbacks=0;
            let mut observe=|_|{let admitted=callbacks<cancel_at;callbacks+=1;admitted};
            let mut control=NativeEncodeControl::new(0,&mut observe);
            let mut output=Vec::new();
            let result=super::write_draft_changes_json_into(&changes,1024,3,64,&mut |bytes:&[u8],_:&mut NativeEncodeControl<'_>|->Result<(),ValueError>{output.extend_from_slice(bytes);Ok(())},&mut control);
            assert!(expected.as_bytes().starts_with(&output));
            if let Err(fault)=result {assert_eq!(fault.kind,ValueRefusalKind::Canceled);}
            assert_eq!(control.owned_bytes(),0);
        }
        for (depth,items,kind) in [(0,64,ValueRefusalKind::DepthLimit),(3,0,ValueRefusalKind::WorkLimit)] {
            let mut observe=|_|true;
            let mut control=NativeEncodeControl::new(0,&mut observe);
            let result=super::write_draft_changes_json_into(&changes,1024,depth,items,&mut |_:&[u8],_:&mut NativeEncodeControl<'_>|->Result<(),ValueError>{Ok(())},&mut control);
            if depth==0||!changes.is_empty(){assert_eq!(result.unwrap_err().kind,kind);}
        }
        println!("[DEBUG] draft-wire borrowed original={} byte/refusal/cancel/depth/item laws admitted",row["id"]);
    }
}

#[test]
fn draft_wire_incremental_cursor_keeps_original_source_and_cumulative_admission(){
    let policy:Value=serde_json::from_str(include_str!("../../✂️text-splice/📡️draft-wire/🧫️fixtures/🔣️.json")).unwrap();
    let caller=&policy["callerGrant"];
    let grant=protocol::value::RetainedCloneGrant{maximum_items:caller["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:caller["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:caller["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:caller["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:caller["maximumDepth"].as_u64().unwrap()as usize};
    use protocol::value::{NativeEncodeControl,ValueError,ValueRefusalKind};
    for row in fixture()["draftJson"].as_array().unwrap(){
        for units in [0,1,3,64]{
            let changes:Vec<super::DraftChange>=serde_json::from_value(row["changes"].clone()).unwrap();
            let original=(changes.as_ptr(),changes.len(),changes.capacity());
            let mut cursor=super::DraftChangesJsonCursor::new(changes,64,3).map_err(|(fault,_)|fault).unwrap();
            let mut callbacks=0;
            let mut observe=|_|{callbacks+=1;true};
            let mut control=NativeEncodeControl::new(32768,&mut observe);
            assert!(cursor.step(0,&mut control,grant).unwrap().is_none());
            let receipt=control.pause().unwrap();
            let mut cancel=|_|false;
            let mut refused=NativeEncodeControl::resume(receipt,&mut cancel).unwrap();
            assert_eq!(cursor.step(1,&mut refused,grant).unwrap_err().kind,ValueRefusalKind::Canceled);
            let receipt=refused.pause().unwrap();
            let mut checkpoints=0;
            let mut observe=|_|{checkpoints+=1;true};
            let mut control=NativeEncodeControl::resume(receipt,&mut observe).unwrap();
            let mut wire=None;
            for _ in 0..5000 {if let Some(output)=cursor.step(units.max(1),&mut control,grant).unwrap(){assert!(cursor.normal_step_progress().fits(grant));wire=Some(output);break;}assert!(cursor.normal_step_progress().fits(grant));}
            let wire=wire.expect("original finite draft wire completes");
            assert_eq!(wire,row["json"].as_str().unwrap());
            let changes=cursor.take_changes().expect("original ranges transfer");
            assert_eq!((changes.as_ptr(),changes.len(),changes.capacity()),original);
            assert_eq!(wire,serde_json::to_string(&changes).unwrap());
            assert!(control.owned_bytes()>=wire.capacity());
            assert!(matches!(cursor.step(1,&mut control,grant),Ok(None)));
            println!("[DEBUG] draft-wire cursor original={} units={} admitted={} exactPointer=true",row["id"],units,control.owned_bytes());
        }
    }
}
