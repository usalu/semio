//! 🧾️ A completed guest failure can request replay of the exact retained lifecycle turn.
use super::*;
use semio_framework_value::{DslValue, FromValue};

fn fields<const N: usize>(value: DslValue, names: [&str; N]) -> Option<[DslValue; N]> {
    let DslValue::Object(entries) = value else { return None; };
    if entries.len() != N { return None; }
    let mut slots: [Option<DslValue>; N] = std::array::from_fn(|_| None);
    for (key, value) in entries {
        let index = names.iter().position(|name| *name == key.as_str())?;
        if slots[index].replace(value).is_some() { return None; }
    }
    Some(slots.map(|slot| slot.expect("all distinct required fields received")))
}

fn primary_fault(bytes: &[u8]) -> Option<semio_framework::Fault> {
    let [primary, effects] = fields(store::pack_rt::decode_wire_value(bytes).ok()?, ["primary", "effects"])?;
    semio_framework_value::RetainedCloneProgress::from_value(effects).ok()?;
    let [kind, original] = fields(primary, ["kind", "original"])?;
    if kind.as_str()? != "fault" { return None; }
    semio_framework::Fault::from_value(original).ok()
}

pub(super) fn retry_lifecycle(bytes: &[u8], events: Option<&[Event]>) -> Result<(), Fault> {
    let original = primary_fault(bytes).ok_or_else(|| PluginArtifactChannel::not_wired("guest failure", "guest returned a compound or non-fault Source failure"))?;
    use semio_plugin_guest_contract::TurnFault;
    let framework = original.origin == semio_framework::FaultOrigin::Framework;
    let fault = TurnFault::Guest(Box::new(original));
    if framework && events.is_some_and(|input| semio_plugin_host::retryable_lifecycle_turn(&fault, input)) { return Ok(()); }
    let TurnFault::Guest(fault) = fault else { unreachable!() };
    Err(Fault { code: fault.code.0, message: fault.message })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn genuine_guest_failure_retries_only_its_retained_lifecycle_input() {
        let law: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/lifecycle-retry.json")).unwrap();
        let bytes: Vec<_> = law["packHex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|p| u8::from_str_radix(std::str::from_utf8(p).unwrap(),16).unwrap()).collect();
        let fault = primary_fault(&bytes).unwrap();
        assert_eq!(fault.code.0, law["code"].as_str().unwrap());
        assert_eq!(fault.message, law["message"].as_str().unwrap());
        for case in law["cases"].as_array().unwrap() {
            let events = match case["event"].as_str().unwrap() {
                "empty" => vec![], "wake" => vec![Event::Wake], "two" => vec![Event::Wake,Event::Wake], _ => unreachable!(),
            };
            assert_eq!(retry_lifecycle(&bytes,Some(&events)).is_ok(),case["retry"].as_bool().unwrap());
        }
        assert!(retry_lifecycle(&bytes, None).is_err());
        let DslValue::Object(mut body) = store::pack_rt::decode_wire_value(&bytes).unwrap() else { panic!() };
        body.push(("settlement".into(), DslValue::Null));
        assert!(retry_lifecycle(&store::pack_rt::encode_wire_value(&DslValue::Object(body)), Some(&[])).is_err());
        assert!(retry_lifecycle(&bytes[..bytes.len()-1], Some(&[])).is_err());
    }
}
