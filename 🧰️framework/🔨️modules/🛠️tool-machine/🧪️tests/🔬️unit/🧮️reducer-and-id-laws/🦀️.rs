/// 🧮️ Reducer and id laws: the state, yield and refusal kinds, the reducer matrix and cases, the resume order and the LEB128 and blake3 oracle of the minted ids.

//#region 🔖️Reducer
#[test]
fn states_yield_kinds_refusals_and_reasons_mirror_the_fixture() {
    let law = law();
    assert_eq!(law["states"], json!(ToolTransactionState::ALL.map(ToolTransactionState::as_str)));
    assert_eq!(law["yieldKinds"], json!(ToolYieldKind::ALL.map(ToolYieldKind::as_str)));
    assert_eq!(law["refusals"], json!(ToolRefusal::ALL.map(ToolRefusal::code)));
    assert_eq!(law["abortReasons"], json!(ToolAbortReason::ALL.map(ToolAbortReason::as_str)));
    for refusal in ToolRefusal::ALL {
        assert_eq!(ToolRefusal::parse(&refusal.to_string()), Some(refusal));
    }
    for reason in ToolAbortReason::ALL {
        assert_eq!(ToolAbortReason::parse(reason.as_str()), Some(reason));
    }
    for kind in ToolYieldKind::ALL {
        assert_eq!(ToolYieldKind::parse(kind.as_str()), Some(kind));
        assert_eq!(sample_yield(kind).kind(), kind);
    }
    for state in ToolTransactionState::ALL {
        assert_eq!(ToolTransactionState::parse(state.as_str()), Some(state));
    }
}

#[test]
fn reducer_matches_every_matrix_row() {
    let law = law();
    let rows = law["matrix"].as_array().expect("matrix");
    assert_eq!(rows.len(), ToolTransactionState::ALL.len() * ToolYieldKind::ALL.len());
    for row in rows {
        let from = ToolTransactionState::parse(text(&row["from"])).expect("state");
        let kind = ToolYieldKind::parse(text(&row["yield"])).expect("yield");
        let label = format!("{} × {}", from.as_str(), kind.as_str());
        let mut transaction = transaction_in(from);
        let before = transaction.clone();
        let result = transaction.apply(sample_yield(kind));
        match row.get("refusal") {
            Some(code) => {
                assert_eq!(result.map_err(|refusal| refusal.to_string()), Err(text(code).to_string()), "{label}");
                assert_eq!(transaction, before, "{label}: a refused yield changes nothing");
            }
            None => {
                assert_eq!(result, Ok(()), "{label}");
                assert_eq!(transaction.state().as_str(), text(&row["to"]), "{label}");
            }
        }
    }
}

#[test]
fn resume_upserts_entries_in_order() {
    let resumed = ToolTransaction::resume(TransactionRef { id: "tx-0000000000000000".into(), tool: "law#resume".into() }, vec![("a".into(), 1), ("b".into(), 2), ("a".into(), 3)]);
    assert_eq!(resumed.state(), ToolTransactionState::Open);
    assert_eq!(resumed.entries(), &[("a".to_string(), 3), ("b".to_string(), 2)]);
}

#[test]
fn reducer_matches_every_case() {
    let law = law();
    for case in law["cases"].as_array().expect("cases") {
        let name = text(&case["name"]);
        let mut transaction = ToolTransaction::open(reference(&case["transaction"]));
        let mut refused = Vec::new();
        for (index, value) in case["yields"].as_array().expect("yields").iter().enumerate() {
            if transaction.apply(yielded(value)).is_err() {
                refused.push(index);
            }
        }
        assert_eq!(transaction.state().as_str(), text(&case["expect"]["state"]), "{name}");
        assert_eq!(entries_json(transaction.entries(), Value::clone), case["expect"]["entries"], "{name}");
        assert_eq!(json!(refused), case["expect"]["refused"], "{name}");
        assert_eq!(transaction.reference(), &reference(&case["transaction"]), "{name}");
        if transaction.state() == ToolTransactionState::Open {
            let entries = transaction.entries().to_vec();
            assert_eq!(ToolTransaction::resume(transaction.reference().clone(), entries), transaction, "{name}: resuming its entries rebuilds the open transaction");
        }
        let (_, mutations) = transaction.into_parts();
        assert_eq!(Value::Array(mutations), Value::Array(case["expect"]["entries"].as_array().expect("entries").iter().map(|entry| entry["mutation"].clone()).collect()), "{name}");
    }
}
//#endregion 🔖️Reducer

//#region 🔖️Ids
fn leb128(out: &mut Vec<u8>, mut value: u64) {
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        if value == 0 {
            out.push(byte);
            return;
        }
        out.push(byte | 0x80);
    }
}

fn length_prefixed(out: &mut Vec<u8>, text: &str) {
    leb128(out, text.len() as u64);
    out.extend_from_slice(text.as_bytes());
}

#[test]
fn replication_mint_and_the_blake3_crate_reproduce_every_fixture_id() {
    let law = law();
    for row in law["ids"].as_array().expect("ids") {
        let (actor, tool, expected) = (text(&row["actor"]), text(&row["tool"]), text(&row["id"]));
        let clock = clock(&row["clock"]);
        let minted = TransactionRef::mint(&ActorId(actor.into()), &clock, tool);
        assert_eq!(minted, TransactionRef { id: expected.to_string(), tool: tool.to_string() }, "{row}");
        let mut material = Vec::new();
        length_prefixed(&mut material, actor);
        leb128(&mut material, clock.actor);
        leb128(&mut material, clock.physical_ms);
        leb128(&mut material, clock.logical);
        length_prefixed(&mut material, tool);
        let oracle: String = blake3::hash(&material).as_bytes()[..8].iter().map(|byte| format!("{byte:02x}")).collect();
        assert_eq!(format!("tx-{oracle}"), expected, "{row}");
    }
}
//#endregion 🔖️Ids
