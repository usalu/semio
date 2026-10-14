//! 👥️ Peer presence roots and the peer roster publication.
use crate::app_views::PeerPresence;
use crate::app_views::PEER_PRESENCE_SLOTS;
use crate::app_views::PeerPresenceEntry;
use crate::app_views::PeerPresenceRoot;
use semio_framework::kernel::FixedCommandPage;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin};
#[cfg(any(test, feature = "artifact-app-testing"))]
use store::os_store::store_schema::ArtifactCommand;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_value::retained_clone::RetainedCloneStep;
use std::mem::ManuallyDrop;
use crate::app_contract::ArtifactApp;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_ownership::bounded_ownership::plugin_retirement_fault;
use crate::child_view::child_content_arc_bytes;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
#[cfg(test)]
use crate::app_views::NoConfigMutation;
#[cfg(test)]
use crate::app_views::NoPresence;
#[cfg(test)]
use crate::reserved_jobs::ConfigurationDecodeRetirement;

pub(crate) struct PeerPresenceRootRetirement {
    root: ManuallyDrop<Option<std::sync::Arc<PeerPresenceRoot>>>,
    owned: ManuallyDrop<Option<PeerPresenceRoot>>,
    entry: ManuallyDrop<Option<semio_framework_value::retirement::shared::SharedControlledRetirement<PeerPresenceEntry>>>,
    cursor: usize,
}

impl PeerPresenceRootRetirement {
    pub(crate) fn new(root: std::sync::Arc<PeerPresenceRoot>) -> Self {
        Self { root: ManuallyDrop::new(Some(root)), owned: ManuallyDrop::new(None), entry: ManuallyDrop::new(None), cursor: 0 }
    }

    fn owned(root: PeerPresenceRoot) -> Self {
        Self { root: ManuallyDrop::new(None), owned: ManuallyDrop::new(Some(root)), entry: ManuallyDrop::new(None), cursor: 0 }
    }

    pub(crate) fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(entry) = self.entry.as_ref() {
            return Ok(RetirementDemand { copy_bytes: entry.next_copy_byte_demand()?, capacity_bytes: entry.next_capacity_byte_demand(body)?, release_bytes: entry.next_release_byte_demand()?, depth: entry.next_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "peer entry retirement depth overflow"))? });
        }
        if self.root.is_some() {
            return Ok(RetirementDemand { copy_bytes: size_of::<PeerPresenceRoot>(), release_bytes: semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<PeerPresenceRoot>(), depth: 1, ..Default::default() });
        }
        if let Some(root) = self.owned.as_ref() {
            return Ok(RetirementDemand { copy_bytes: if self.cursor < root.len { size_of::<std::sync::Arc<PeerPresenceEntry>>() } else { 0 }, depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }

    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(PluginLifecycleStep::Complete(empty)); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(PluginLifecycleStep::Progress(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes || demand.depth > grant.maximum_depth { return Ok(PluginLifecycleStep::Progress(empty)); }
        if let Some(entry) = self.entry.as_mut() {
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = entry.step(child)?;
            let progress = step.progress();
            if entry.terminal_is_empty() { drop(self.entry.take()); }
            return Ok(PluginLifecycleStep::Progress(progress));
        }
        if self.owned.is_none() {
            let root = self.root.as_mut().expect("peer root retains its original backing");
            if std::sync::Arc::get_mut(root).is_none() { return Ok(PluginLifecycleStep::Blocked { reason: "peer-presence root remains captured by a live operation" }); }
            let release = semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<PeerPresenceRoot>();
            if grant.maximum_release_bytes < release { return Ok(PluginLifecycleStep::Progress(empty)); }
            let root = self.root.take().expect("peer root original ownership");
            match std::sync::Arc::try_unwrap(root) {
                Ok(root) => *self.owned = Some(root),
                Err(root) => { *self.root = Some(root); return Ok(PluginLifecycleStep::Progress(empty)); }
            }
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<PeerPresenceRoot>(), released_bytes: release, ..empty }));
        }
        let root = self.owned.as_mut().expect("unique peer root remains inline");
        if self.cursor < root.len {
            let copy = size_of::<std::sync::Arc<PeerPresenceEntry>>();
            if grant.maximum_copy_bytes < copy { return Ok(PluginLifecycleStep::Progress(empty)); }
            let index = self.cursor;
            self.cursor += 1;
            if let Some(entry) = root.entries[index].take() {
                *self.entry = Some(semio_framework_value::retirement::shared::SharedControlledRetirement::new(entry));
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..empty }));
            }
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        root.len = 0;
        drop(self.owned.take());
        Ok(PluginLifecycleStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }))
    }

    pub(crate) fn terminal_is_empty(&self) -> bool { self.root.is_none() && self.owned.is_none() && self.entry.is_none() }
}

impl Drop for PeerPresenceRootRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "incomplete peer-presence root reached Drop without bounded disposal");
    }
}

#[cfg(test)]
mod peer_root_retirement_laws {
    use super::*;
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;

    #[test]
    fn configuration_completion_preserves_original_physical_owner_and_grants() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️retirement.json")).unwrap();
        let law = &fixture["configurationCompletion"];
        let axes = &law["grant"];
        let grant = RetainedCloneGrant { maximum_items: axes["items"].as_u64().unwrap() as usize, maximum_copy_bytes: axes["copyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: axes["capacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: axes["releaseBytes"].as_u64().unwrap() as usize, maximum_depth: axes["depth"].as_u64().unwrap() as usize };
        let (original, birth) = observe(|| std::sync::Arc::new(std::sync::Mutex::new(Some(Err::<ArtifactCommand<NoConfigMutation>, _>(Fault::new(FaultOrigin::Framework, FaultCode::new("original.configuration-cancellation"), law["message"].as_str().unwrap()).into())))));
        let identity = std::sync::Arc::as_ptr(&original);
        let (mut owner, handoff) = observe(|| ConfigurationDecodeRetirement { original: std::mem::ManuallyDrop::new(Some(original)), payload: None });
        assert_eq!((handoff.requested_bytes, handoff.released_bytes), (0, 0));
        for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }, RetainedCloneGrant { maximum_release_bytes: 0, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }] {
            let (step, heap) = observe(|| owner.step(denied).unwrap());
            assert_eq!(step.progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(std::sync::Arc::as_ptr(owner.original.as_ref().unwrap()), identity);
        }
        let mut allocated = birth.requested_bytes;
        let mut released = birth.released_bytes;
        for _ in 0..10000 {
            if owner.terminal_is_empty() { break; }
            let (step, heap) = observe(|| owner.step(grant).unwrap());
            let progress = step.progress();
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            allocated += heap.requested_bytes; released += heap.released_bytes;
        }
        assert!(owner.terminal_is_empty());
        assert_eq!(allocated, released);
        let (_, terminal) = observe(|| drop(owner));
        assert_eq!((terminal.requested_bytes, terminal.released_bytes), (0, 0));
        assert_eq!(serde_json::json!({ "message": law["message"] }), serde_json::from_str::<serde_json::Value>("{\"message\":\"Original configuration cancellation\"}").unwrap());
        eprintln!("[DEBUG] configuration original completion exact pointers, denied/constructor/final-drop zero effects, physical conservation {allocated}/{released}");
    }

    #[test]
    fn original_peer_fault_transfer_is_admitted_and_preserves_typed_custody() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️retirement.json")).unwrap();
        let law = &fixture["faultTransfer"];
        let fault = Fault::new(FaultOrigin::Framework, FaultCode::new(law["code"].as_str().unwrap()), law["message"].as_str().unwrap());
        let pointers = (fault.code.0.as_ptr(), fault.message.as_ptr(), fault.scope.as_ref() as *const _);
        let (mut retained, heap) = observe(|| PeerRosterFault::new(Some(fault)));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let axes = &law["grant"];
        let grant = RetainedCloneGrant { maximum_items: axes["items"].as_u64().unwrap() as usize, maximum_copy_bytes: axes["copyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: axes["depth"].as_u64().unwrap() as usize };
        for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }] {
            let (result, heap) = observe(|| retained.take_original(denied));
            assert!(result.is_err());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(!retained.terminal_is_empty());
        }
        let ((fault, progress), heap) = observe(|| retained.take_original(grant).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(progress, RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<Option<Fault>>(), ..Default::default() });
        assert!(progress.fits(grant));
        let fault = fault.unwrap();
        assert_eq!((fault.code.0.as_ptr(), fault.message.as_ptr(), fault.scope.as_ref() as *const _), pointers);
        assert_eq!(serde_json::json!({ "code": &fault.code.0, "message": &fault.message }), serde_json::json!({ "code": law["code"], "message": law["message"] }));
        assert!(retained.terminal_is_empty());
        let (_, heap) = observe(|| drop(retained));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        drop(fault);
        eprintln!("[DEBUG] original peer fault preserves all original pointers and admits typed outcome transfer without physical effects");
    }

    #[test]
    fn empty_presence_native_factory_uses_original_controls_and_preserves_refusal() {
        use store::ArtifactPresenceSnapshot;
        let law: serde_json::Value = serde_json::from_str(include_str!("🧫️retirement.json")).unwrap();
        let axes = &law["emptyPresence"]["grant"];
        let grant = RetainedCloneGrant { maximum_items: axes["items"].as_u64().unwrap() as usize, maximum_copy_bytes: axes["copyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: axes["capacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: axes["releaseBytes"].as_u64().unwrap() as usize, maximum_depth: axes["depth"].as_u64().unwrap() as usize };
        let mut decoder_events = 0;
        let mut admit_decode = |_| { decoder_events += 1; true };
        let mut decode = semio_framework_value::NativeDecodeControl::new(0, &mut admit_decode);
        let mut original = crate::io::control::NativeSnapshotDecodeOwner::new(&mut decode, grant);
        let (value, heap) = observe(|| NoPresence::decode_presence_native(&[], &mut original).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(original.progress(), RetainedCloneProgress { copied_items: 1, ..Default::default() });
        drop(original);
        drop(decode);
        assert!(decoder_events > 0);
        let expected = law["emptyPresence"]["output"].clone();
        assert_eq!(serde_json::to_value(semio_framework_value::ToValue::to_value(&value)).unwrap(), expected);
        let mut encode_events = 0;
        let mut admit_encode = |_| { encode_events += 1; true };
        let mut encode = semio_framework_value::NativeEncodeControl::new(0, &mut admit_encode);
        let mut original = crate::io::control::NativeSnapshotEncodeOwner::new(&mut encode, grant);
        let (bytes, heap) = observe(|| value.encode_presence_native(&mut original).unwrap());
        assert_eq!(bytes, Vec::<u8>::new());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(original.progress(), RetainedCloneProgress { copied_items: 1, ..Default::default() });
        drop(original);
        drop(encode);
        assert!(encode_events > 0);
        let mut reject = |_| false;
        let mut native = semio_framework_value::NativeDecodeControl::new(0, &mut reject);
        let mut original = crate::io::control::NativeSnapshotDecodeOwner::new(&mut native, grant);
        let (result, heap) = observe(|| NoPresence::decode_presence_native(&[], &mut original));
        assert!(result.is_err());
        assert_eq!(original.progress(), RetainedCloneProgress::default());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let mut reject = |_| false;
        let mut native = semio_framework_value::NativeEncodeControl::new(0, &mut reject);
        let mut original = crate::io::control::NativeSnapshotEncodeOwner::new(&mut native, grant);
        let (result, heap) = observe(|| value.encode_presence_native(&mut original));
        assert!(result.is_err());
        assert_eq!(original.progress(), RetainedCloneProgress::default());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }] {
            let mut accept = |_| true;
            let mut native = semio_framework_value::NativeDecodeControl::new(0, &mut accept);
            let mut original = crate::io::control::NativeSnapshotDecodeOwner::new(&mut native, denied);
            let (result, heap) = observe(|| NoPresence::decode_presence_native(&[], &mut original));
            assert!(result.is_err());
            assert_eq!(original.progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        eprintln!("[DEBUG] original empty presence native owners retain callbacks and exact zero physical effects");
    }

    #[test]
    fn peer_metadata_projection_matches_the_complete_neutral_wire_corpus() {
        let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../🔨️modules/📡️replication/🧪️tests/🧫️fixtures/👥️presence-peer-codec-v1/🔣️.json")).unwrap();
        let decode_hex = |text: &str| text.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect::<Vec<_>>();
        for row in corpus["cases"].as_array().unwrap() {
            let mut bytes = decode_hex(row["prefixHex"].as_str().unwrap());
            bytes.extend(std::iter::repeat_n(decode_hex(row["repeatHex"].as_str().unwrap())[0], row["repeatCount"].as_u64().unwrap() as usize));
            bytes.extend(decode_hex(row["suffixHex"].as_str().unwrap()));
            let (result, heap) = observe(|| protocol::borrow_presence_metadata(&bytes));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0), "{}", row["id"]);
            assert_eq!(result.is_ok(), row["accepted"].as_bool().unwrap(), "{}", row["id"]);
            if let Ok(view) = result {
                let expected = &row["expected"];
                assert_eq!(serde_json::json!({ "actor": view.actor, "connectedAtMs": view.connected_at_ms, "surface": view.surface, "color": view.color }), serde_json::json!({ "actor": expected["actor"], "connectedAtMs": expected["connectedAtMs"], "surface": expected["surface"], "color": expected["color"] }), "{}", row["id"]);
                let interaction = view.interaction.map(|value| serde_json::json!({ "appId": value.app_id, "domains": value.domains().map(|domain| serde_json::json!({ "domain": domain.domain, "granularity": domain.granularity, "selected": domain.selected.iter().collect::<Vec<_>>(), "hovered": domain.hovered.iter().collect::<Vec<_>>() })).collect::<Vec<_>>() }));
                assert_eq!(serde_json::to_value(interaction).unwrap(), expected["interaction"], "{}", row["id"]);
            }
        }
        eprintln!("[DEBUG] original borrowed peer projection matches the complete neutral codec corpus with zero physical effects");
    }

    #[test]
    fn peer_metadata_projection_borrows_the_complete_validated_original_wire() {
        let peer = protocol::PresencePeer { actor: "alice".into(), connected_at_ms: 1, label: Some("ignored label".into()), presence_pack: Some(vec![1, 2, 3]), user_id: Some("ignored user".into()), role: Some("ignored role".into()), drag_ghost_json: Some("{}".into()), interaction: Some(protocol::PresenceInteraction { app_id: "app".into(), domains: vec![protocol::PresenceDomain { domain: "items".into(), granularity: "item".into(), selected: vec!["selected".into()], hovered: vec!["hovered".into()] }] }), color: Some(3), surface: Some("original surface".into()), views: Vec::new(), ui: None, tool_run: None, principal_kind: None, active_tool: None, history_edit: None, typing: Vec::new() };
        let encoded = ::semio_framework_async::poll::resolve_ready(protocol::encode_presence_peer(&peer));
        let (view, heap) = observe(|| protocol::borrow_presence_metadata(&encoded).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let expected: serde_json::Value = serde_json::from_str(r#"{"actor":"alice","surface":"original surface","color":3}"#).unwrap();
        assert_eq!(serde_json::json!({ "actor": view.actor, "surface": view.surface, "color": view.color }), expected);
        for bytes in [view.actor.as_bytes(), view.surface.unwrap().as_bytes(), view.presence_pack.unwrap()] {
            assert!(bytes.as_ptr() as usize >= encoded.as_ptr() as usize && bytes.as_ptr() as usize + bytes.len() <= encoded.as_ptr() as usize + encoded.len());
        }
        let interaction = view.interaction.unwrap();
        assert_eq!(interaction.app_id, "app");
        let domain = interaction.domains().next().unwrap();
        assert_eq!((domain.domain, domain.granularity), ("items", "item"));
        assert_eq!(domain.selected.iter().collect::<Vec<_>>(), vec!["selected"]);
        assert_eq!(domain.hovered.iter().collect::<Vec<_>>(), vec!["hovered"]);
        let invalid = [0xff];
        let (error, heap) = observe(|| protocol::borrow_presence_metadata(&invalid));
        assert!(error.is_err());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        eprintln!("[DEBUG] original peer metadata projection borrowed validated wire without physical effects");
    }

    #[test]
    fn original_presence_page_close_charges_complete_fixed_backing() {
        let (page, birth) = observe(|| FixedCommandPage::try_copy_from(b"abc").unwrap());
        assert_eq!((birth.requested_bytes, birth.released_bytes), (4096, 0));
        let mut original = protocol::PresenceCommandCursor::retain_rejected(page, 1);
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: std::mem::size_of::<FixedCommandPage>(), maximum_capacity_bytes: 0, maximum_release_bytes: 4096, maximum_depth: 1 };
        for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - 1, ..grant }, RetainedCloneGrant { maximum_release_bytes: 4095, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }] {
            let (step, heap) = observe(|| original.close_step(denied));
            assert_eq!(step.progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(!original.terminal_is_empty());
        }
        let (step, heap) = observe(|| original.close_step(grant));
        assert!(matches!(step, RetainedCloneStep::Complete(_)));
        assert!(step.progress().fits(grant));
        assert_eq!(step.progress().released_bytes, birth.requested_bytes);
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, step.progress().released_bytes));
        let (_, heap) = observe(|| drop(original));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        eprintln!("[DEBUG] original presence page released its exact4096 backing under independent axes");
    }

    #[test]
    fn original_peer_root_retirement_preserves_pointers_and_conserves_physical_backing() {
        let law: serde_json::Value = serde_json::from_str(include_str!("🧫️retirement.json")).unwrap();
        let axes = &law["grant"];
        let grant = RetainedCloneGrant { maximum_items: axes["items"].as_u64().unwrap() as usize, maximum_copy_bytes: axes["copyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: axes["capacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: axes["releaseBytes"].as_u64().unwrap() as usize, maximum_depth: axes["depth"].as_u64().unwrap() as usize };
        let (root, birth) = observe(|| {
            let mut root = PeerPresenceRoot::empty();
            for actor in law["actors"].as_array().unwrap() {
                let mut text = String::with_capacity(128);
                text.push_str(actor.as_str().unwrap());
                let mut surface = String::with_capacity(256);
                surface.push_str("original surface");
                root.insert(text, PeerPresence { color: Some(3), surface: Some(surface), interaction: Some(protocol::PresenceInteraction { app_id: "original-app".into(), domains: vec![protocol::PresenceDomain { domain: "original-domain".into(), granularity: "item".into(), selected: vec!["original-selected".into()], hovered: vec!["original-hovered".into()] }] }) }).unwrap();
            }
            std::sync::Arc::new(root)
        });
        let actual: Vec<_> = root.iter().map(|(actor, _)| actor).collect();
        let reference: std::collections::BTreeMap<String, ()> = serde_json::from_value::<Vec<String>>(law["actors"].clone()).unwrap().into_iter().map(|actor| (actor, ())).collect();
        assert_eq!(actual, reference.keys().map(String::as_str).collect::<Vec<_>>());
        let pointers: Vec<_> = root.iter().map(|(actor, presence)| (actor.as_ptr(), presence.surface.as_ref().unwrap().as_ptr())).collect();
        let borrowed = root.clone();
        let (mut owner, handoff) = observe(|| PeerPresenceRootRetirement::new(root));
        assert_eq!((handoff.requested_bytes, handoff.released_bytes), (0, 0));
        for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_release_bytes: 0, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }] {
            let (_, heap) = observe(|| owner.close_step(denied));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        assert_eq!(borrowed.iter().map(|(actor, presence)| (actor.as_ptr(), presence.surface.as_ref().unwrap().as_ptr())).collect::<Vec<_>>(), pointers);
        let (_, heap) = observe(|| owner.close_step(grant).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        drop(borrowed);
        let (mut born, mut released) = (birth.requested_bytes, birth.released_bytes);
        for turn in 0..4096 {
            if owner.terminal_is_empty() { break; }
            let (step, heap) = observe(|| owner.close_step(grant).unwrap());
            let receipt = step.progress().unwrap_or_default();
            assert!(receipt.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
            born += heap.requested_bytes;
            released += heap.released_bytes;
            assert!(turn < 4095, "original peer root failed to reach its terminal witness");
        }
        assert!(owner.terminal_is_empty());
        assert_eq!(born, released);
        let (_, terminal) = observe(|| drop(owner));
        assert_eq!((terminal.requested_bytes, terminal.released_bytes), (0, 0));
        eprintln!("[DEBUG] original peer root pointer custody conserved {born} physical bytes");
    }
}

const PEER_ROSTER_WIRE_BYTES: usize = 4_096;

pub struct PresenceRosterAdmission {
    pub(crate) seq: u64,
    pub(crate) generation: u64,
    pub(crate) cancel: semio_framework_job::CancelToken,
}

impl PresenceRosterAdmission {
    pub(crate) fn sequence(&self) -> u64 {
        self.seq
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }
}

pub struct PresenceRosterOutcome {
    pub seq: u64,
    pub fault: Option<Fault>,
}

pub(crate) struct PeerRosterFault {
    original: ManuallyDrop<Option<Fault>>,
}

impl PeerRosterFault {
    fn new(original: Option<Fault>) -> Self { Self { original: ManuallyDrop::new(original) } }
    pub(crate) fn terminal_is_empty(&self) -> bool { self.original.is_none() }
    pub(crate) fn take(&mut self) -> Option<Fault> { self.original.take() }
    fn take_original(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Fault>, RetainedCloneProgress), ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok((None, empty)); }
        if grant.maximum_items == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "peer fault outcome needs original work admission")); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "peer fault outcome needs original depth admission")); }
        let copy = size_of::<Option<Fault>>();
        if grant.maximum_copy_bytes < copy { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "peer fault outcome needs original typed copy admission")); }
        Ok((self.original.take(), RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..empty }))
    }
}

impl Drop for PeerRosterFault {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "original peer fault remains retained until admitted outcome transfer"); }
}

pub(crate) struct PeerRosterCandidate<P> {
    pub(crate) seq: u64,
    pub(crate) generation: u64,
    pub(crate) cancel: semio_framework_job::CancelToken,
    pub(crate) own_color: Option<u8>,
    pub(crate) now_ms: i64,
    pub(crate) root: std::sync::Arc<PeerPresenceRoot>,
    pub(crate) typed: store::PresencePeersCommit<P>,
}

pub(crate) struct ValidatedPeerRosterCommit {
    pub(crate) generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PeerRosterPublicationStage {
    DecodeMetadata,
    PruneTyped,
    DecodeTyped,
    ReleaseCreated,
    CommitTyped,
    Ready,
}

/// 🧭️ What one granted roster turn did; the original decode owner's receipt is the turn's physical progress.
enum PeerRosterTurn {
    Spent,
    Failed(PluginLifecycleStep),
    Awaiting(&'static str),
    Complete,
}

/// 📦️ Fixed slots retaining each peer identity and its encoded presence.
type PeerPresencePacks = [Option<(String, Vec<u8>)>; PEER_PRESENCE_SLOTS];

pub(crate) struct PeerRosterPublication<A: ArtifactApp> {
    pub(crate) seq: u64,
    pub(crate) generation: u64,
    own_color: Option<u8>,
    now_ms: i64,
    pub(crate) cancel: semio_framework_job::CancelToken,
    raw: protocol::PresenceCommandCursor,
    candidate: ManuallyDrop<Option<PeerPresenceRoot>>,
    retirement: ManuallyDrop<Option<PeerPresenceRootRetirement>>,
    packs: ManuallyDrop<PeerPresencePacks>,
    packs_len: usize,
    packs_shell_released: bool,
    pack_close: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pending_commit: ManuallyDrop<Option<store::PresencePeersCommit<A::Presence>>>,
    pending_commit_retirement: ManuallyDrop<Option<store::PresenceStoreRetirement<A::Presence>>>,
    pending_actor: ManuallyDrop<Option<String>>,
    pending_presence: ManuallyDrop<Option<A::Presence>>,
    pending_pack: ManuallyDrop<Option<(String, Vec<u8>)>>,
    typed: ManuallyDrop<Option<store::PresencePeersPublication<A::Presence>>>,
    rejected_original: ManuallyDrop<Option<store::PresencePeerAdmissionRejected<A::Presence>>>,
    rejected: ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub(crate) stage: PeerRosterPublicationStage,
    faulted: bool,
    pub(crate) fault: PeerRosterFault,
}

impl<A: ArtifactApp> PeerRosterPublication<A> {
    pub(crate) fn new(admission: PresenceRosterAdmission, now_ms: i64, raw: protocol::PresenceCommandCursor) -> Self {
        Self {
            seq: admission.seq,
            generation: admission.generation,
            own_color: raw.own_color(),
            now_ms,
            cancel: admission.cancel,
            raw,
            candidate: ManuallyDrop::new(Some(PeerPresenceRoot::empty())),
            retirement: ManuallyDrop::new(None),
            packs: ManuallyDrop::new(std::array::from_fn(|_| None)),
            packs_len: 0,
            packs_shell_released: false,
            pack_close: None,
            pending_commit: ManuallyDrop::new(None),
            pending_commit_retirement: ManuallyDrop::new(None),
            pending_actor: ManuallyDrop::new(None),
            pending_presence: ManuallyDrop::new(None),
            pending_pack: ManuallyDrop::new(None),
            typed: ManuallyDrop::new(None),
            rejected_original: ManuallyDrop::new(None),
            rejected: ManuallyDrop::new(None),
            stage: PeerRosterPublicationStage::DecodeMetadata,
            faulted: false,
            fault: PeerRosterFault::new(None),
        }
    }

    /// 🧯️ Retains the first fault and routes every later turn through bounded cleanup.
    pub(crate) fn fail(&mut self, fault: Fault) -> PluginLifecycleStep {
        self.faulted = true;
        assert!(self.fault.terminal_is_empty(), "peer failure preserves its original outcome owner");
        *self.fault.original = Some(fault);
        PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })
    }

    pub(crate) fn is_faulted(&self) -> bool {
        self.faulted
    }
    pub(crate) fn rejected(admission: PresenceRosterAdmission, raw: FixedCommandPage, fault: Fault) -> Self {
        let seq = admission.seq;
        let mut publication = Self::new(admission, 0, protocol::PresenceCommandCursor::retain_rejected(raw, seq));
        publication.fail(fault);
        publication
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    pub(crate) fn push_page(&mut self, page_index: u32, page: FixedCommandPage) -> Result<(), (Fault, FixedCommandPage)> {
        self.raw.push_page(page_index, page).map_err(|(error, page)| (error.into_fault(), page))
    }

    /// 🥡️ SAFETY: every owned pack was transferred or released in a bounded step and this
    /// exact flag makes the fixed empty shell release once.
    fn release_packs_shell(&mut self) {
        if !self.packs_shell_released {
            debug_assert_eq!(self.packs_len, 0);
            unsafe { ManuallyDrop::drop(&mut self.packs) };
            self.packs_shell_released = true;
        }
    }

    /// 🎟️ Advances exactly one stage of the mounted roster under the caller's unchanged grant; the original decode owner
    /// accumulates every actual receipt of the turn, so the answer reports what the store and the decoder really spent.
    pub(crate) fn step(&mut self, presence_store: &store::PresenceStore<A::Presence, A::PresenceMutation>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let empty = RetainedCloneProgress::default();
        if self.faulted { return Ok(PluginLifecycleStep::Blocked { reason: "original peer fault awaits admitted outcome transfer" }); }
        if grant.maximum_items == 0 {
            return Ok(PluginLifecycleStep::Progress(empty));
        }
        if ::semio_framework_async::poll::resolve_ready(self.cancel.is_cancelled()) {
            return Ok(self.fail(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-cancelled"), "peer roster publication was cancelled before commit")));
        }
        let demand = self.step_demands().map_err(|error| plugin_sdk_fault(error.to_string()))?;
        if grant.maximum_depth < demand.depth { return Err(plugin_sdk_fault("peer roster step exceeds its admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(PluginLifecycleStep::Progress(empty)); }
        let mut admit = |_: semio_framework_value::native_decoding::NativeDecodeProgress| true;
        let mut native = semio_framework_value::NativeDecodeControl::new_retained(&mut admit);
        let mut original = crate::io::control::NativeSnapshotDecodeOwner::new(&mut native, grant);
        let turn = self.step_stage(presence_store, &mut original);
        let progress = original.progress();
        match turn {
            Ok(PeerRosterTurn::Spent) => Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress })),
            Ok(PeerRosterTurn::Failed(step)) => Ok(step),
            Ok(PeerRosterTurn::Awaiting(reason)) => Ok(PluginLifecycleStep::AwaitingInput { reason }),
            Ok(PeerRosterTurn::Complete) => Ok(PluginLifecycleStep::Complete(progress)),
            Err(fault) => Err(fault.with_retained_progress(progress)),
        }
    }

    fn step_stage(&mut self, presence_store: &store::PresenceStore<A::Presence, A::PresenceMutation>, original: &mut crate::io::control::NativeSnapshotDecodeOwner<'_, '_>) -> Result<PeerRosterTurn, Fault> {
        let refused = |error: ValueError| plugin_sdk_fault(error.to_string());
        match self.stage {
            PeerRosterPublicationStage::DecodeMetadata => {
                if self.raw.terminal_is_empty() {
                    let (typed, receipt) = presence_store.begin_peer_publication(original.remaining_grant()).map_err(refused)?;
                    original.record_progress(receipt).map_err(refused)?;
                    *self.typed = Some(typed);
                    self.stage = PeerRosterPublicationStage::PruneTyped;
                    return Ok(PeerRosterTurn::Spent);
                }
                if self.raw.waiting_for_page() {
                    return Ok(PeerRosterTurn::Awaiting("peer roster publication is waiting for its next exact page owner"));
                }
                let next_len = match self.raw.next_len() {
                    Ok(next_len) => next_len,
                    Err(error) => return Ok(PeerRosterTurn::Failed(self.fail(plugin_sdk_fault(error.to_string())))),
                };
                let Some(_) = next_len else {
                    let progress = self.raw.close_step(original.remaining_grant()).progress();
                    original.record_progress(progress).map_err(refused)?;
                    return Ok(PeerRosterTurn::Spent);
                };
                let raw = match self.raw.take_next() {
                    Ok(Some(raw)) => raw,
                    Ok(None) => unreachable!("presence cursor advertised one retained entry"),
                    Err(error) => return Ok(PeerRosterTurn::Failed(self.fail(plugin_sdk_fault(error.to_string())))),
                };
                let bytes = raw.len();
                let decoded = ::semio_framework_async::poll::resolve_ready(protocol::decode_presence_peer(raw.as_slice()));
                match decoded {
                    Ok(peer) => {
                        let protocol::PresencePeer { actor, presence_pack, color, surface, interaction, .. } = peer;
                        let pack_actor = presence_pack.as_ref().map(|_| actor.clone());
                        if let Err(fault) = self
                            .candidate
                            .as_mut()
                            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-candidate"), "peer roster candidate was already transferred"))?
                            .insert(actor, PeerPresence { color, surface, interaction })
                        {
                            return Ok(PeerRosterTurn::Failed(self.fail(fault)));
                        }
                        if let (Some(actor), Some(pack)) = (pack_actor, presence_pack) {
                            if pack.len() > PEER_ROSTER_WIRE_BYTES || self.packs_len == PEER_PRESENCE_SLOTS {
                                return Ok(PeerRosterTurn::Failed(self.fail(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-presence-pack-cap"), "one app-typed presence pack exceeds its fixed decode authority"))));
                            }
                            self.packs[self.packs_len] = Some((actor, pack));
                            self.packs_len += 1;
                        }
                    }
                    Err(error) => return Ok(PeerRosterTurn::Failed(self.fail(plugin_sdk_fault(error.to_string())))),
                }
                original.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, released_bytes: bytes, ..Default::default() }).map_err(refused)?;
                Ok(PeerRosterTurn::Spent)
            }
            PeerRosterPublicationStage::PruneTyped => {
                let root = self.candidate.as_ref().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-candidate"), "peer roster metadata candidate was already transferred"))?;
                let (pruned, receipt) = self
                    .typed
                    .as_mut()
                    .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-typed-candidate"), "app-typed peer candidate was not initialized"))?
                    .prune_one(|actor| root.contains_key(actor), original.remaining_grant())
                    .map_err(refused)?;
                original.record_progress(receipt).map_err(refused)?;
                if !pruned {
                    self.stage = PeerRosterPublicationStage::DecodeTyped;
                }
                Ok(PeerRosterTurn::Spent)
            }
            PeerRosterPublicationStage::DecodeTyped => {
                if self.pending_pack.is_some() || self.rejected.is_some() {
                    let step = if self.rejected.is_some() { store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.rejected, original.remaining_grant()) } else { store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending_pack, &mut self.rejected, original.remaining_grant()) }.map_err(refused)?;
                    original.record_progress(step.progress()).map_err(refused)?;
                    return Ok(PeerRosterTurn::Spent);
                }
                if self.packs_len == 0 {
                    self.release_packs_shell();
                    self.stage = PeerRosterPublicationStage::ReleaseCreated;
                    original.record_progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }).map_err(refused)?;
                    return Ok(PeerRosterTurn::Spent);
                }
                if self.pending_presence.is_none() {
                    let (_, pack) = self.packs[self.packs_len - 1].as_ref().expect("original typed presence pack");
                    let presence = <A::Presence as store::ArtifactPresenceSnapshot>::decode_presence_native(pack, original).map_err(refused)?;
                    *self.pending_presence = Some(presence);
                    let (actor, _) = self.packs[self.packs_len - 1].as_mut().expect("original typed presence pack");
                    *self.pending_actor = Some(std::mem::take(actor));
                    return Ok(PeerRosterTurn::Spent);
                }
                let typed = self.typed.as_mut().ok_or_else(|| plugin_sdk_fault("original typed presence candidate is absent"))?;
                typed.adopt(&mut self.pending_actor, &mut self.pending_presence, self.now_ms, original).map_err(refused)?;
                *self.pending_pack = self.packs[self.packs_len - 1].take();
                self.packs_len -= 1;
                Ok(PeerRosterTurn::Spent)
            }
            PeerRosterPublicationStage::ReleaseCreated => {
                let (released, receipt) = self.typed.as_mut().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-typed-candidate"), "app-typed peer candidate was already transferred"))?.release_created_one(original.remaining_grant()).map_err(refused)?;
                original.record_progress(receipt).map_err(refused)?;
                if !released {
                    self.stage = PeerRosterPublicationStage::CommitTyped;
                }
                Ok(PeerRosterTurn::Spent)
            }
            PeerRosterPublicationStage::CommitTyped => {
                let (commit, _) = self.typed.as_mut().ok_or_else(|| plugin_sdk_fault("original typed publication is absent"))?.take_commit(original).map_err(refused)?;
                *self.pending_commit = Some(commit);
                self.stage = PeerRosterPublicationStage::Ready;
                Ok(PeerRosterTurn::Spent)
            }
            PeerRosterPublicationStage::Ready => Ok(PeerRosterTurn::Complete),
        }
    }

    /// 🎁️ Hands the finished roster to its atomic publication: the metadata Arc is born here from the already-detached candidate,
    /// the caller books its birth on the capacity axis, and the typed commit was taken by the last granted turn.
    pub(crate) fn take_candidate(&mut self) -> Result<PeerRosterCandidate<A::Presence>, Fault> {
        if self.faulted || self.stage != PeerRosterPublicationStage::Ready || !self.raw.terminal_is_empty() || !self.packs_shell_released || self.pending_commit.is_none() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-not-ready"), "peer roster candidate was requested before every retained stage completed"));
        }
        let metadata = self.candidate.take().ok_or_else(|| plugin_sdk_fault("original peer metadata candidate is absent"))?;
        let root = std::sync::Arc::new(metadata);
        let typed = self.pending_commit.take().expect("original retained typed commit");
        let shell = self.typed.take().expect("original terminal typed publication");
        assert!(shell.terminal_is_empty()); drop(shell);
        Ok(PeerRosterCandidate { seq: self.seq, generation: self.generation, cancel: self.cancel.clone(), own_color: self.own_color, now_ms: self.now_ms, root, typed })
    }

    pub(crate) fn retain_rejected_candidate(&mut self, candidate: PeerRosterCandidate<A::Presence>) {
        assert!(self.terminal_is_empty(), "returned peer candidate must rejoin its exact empty publication owner");
        *self.retirement = Some(PeerPresenceRootRetirement::new(candidate.root));
        *self.pending_commit = Some(candidate.typed);
        self.fail(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-publication-rejected"), "peer roster atomic publication was refused after every retained stage completed"));
    }

    /// 📏️ Quotes the exact next [`Self::step`] turn of the current stage.
    pub(crate) fn step_demands(&self) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        Ok(match self.stage {
            PeerRosterPublicationStage::DecodeMetadata => {
                if self.raw.terminal_is_empty() { item }
                else if self.raw.waiting_for_page() { Default::default() }
                else {
                    match self.raw.next_len().map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))? {
                        Some(bytes) => RetirementDemand { copy_bytes: bytes, release_bytes: bytes, ..item },
                        None => RetirementDemand { copy_bytes: size_of::<FixedCommandPage>(), release_bytes: semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES, ..item },
                    }
                }
            }
            PeerRosterPublicationStage::PruneTyped | PeerRosterPublicationStage::ReleaseCreated => item,
            PeerRosterPublicationStage::CommitTyped => RetirementDemand { capacity_bytes: child_content_arc_bytes::<store::os_store::presence::PresencePeersRoot<A::Presence>>(), depth: 1, ..item },
            PeerRosterPublicationStage::DecodeTyped => {
                if self.rejected.is_some() { store::os_store::snapshot_read::artifact_retirement_box_demands(self.rejected.as_ref().expect("original peer rejection owner"), 0)? }
                else if self.pending_pack.is_some() { store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_pack)? }
                else if self.packs_len == 0 || self.pending_presence.is_none() { RetirementDemand { depth: 2, ..item } }
                else { RetirementDemand { capacity_bytes: child_content_arc_bytes::<A::Presence>() + child_content_arc_bytes::<(String, std::sync::Arc<A::Presence>, i64)>(), depth: 2, ..item } }
            }
            PeerRosterPublicationStage::Ready => Default::default(),
        })
    }

    /// 📏️ Quotes the exact next [`Self::close_step`] turn, one retained owner at a time in the same order.
    pub(crate) fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "peer roster close depth overflow"))?; Ok(demand) };
        if self.terminal_is_empty() { return Ok(Default::default()); }
        if let Some(retirement) = self.retirement.as_ref() { return nested(retirement.retirement_demands(body)?); }
        if let Some(rejected) = self.rejected.as_ref() { return nested(store::os_store::snapshot_read::artifact_retirement_box_demands(rejected, body)?); }
        if self.pending_commit.is_some() { return Ok(item); }
        if let Some(owner) = self.pending_commit_retirement.as_ref() { return nested(owner.retirement_demands(body)?); }
        if self.pending_actor.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_actor); }
        if self.pending_presence.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_presence); }
        if self.pending_pack.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_pack); }
        if !self.raw.terminal_is_empty() {
            return Ok(if self.raw.waiting_for_page() { item } else { RetirementDemand { copy_bytes: size_of::<FixedCommandPage>(), release_bytes: semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES, ..item } });
        }
        if let Some(owner) = self.pack_close.as_ref() { return nested(store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body)?); }
        if self.packs_len != 0 || self.candidate.is_some() { return Ok(item); }
        if let Some(typed) = self.typed.as_ref() { return typed.retirement_demands(body); }
        Ok(Default::default())
    }

    /// 🎟️ Releases one retained roster owner under the caller's unchanged grant; a refusal keeps every owner in place.
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        self.close_value_step(grant).map_err(plugin_retirement_fault)
    }

    fn close_value_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(PluginLifecycleStep::Complete(empty)); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(PluginLifecycleStep::Progress(empty)); }
        if let Some(retirement) = self.retirement.as_mut() {
            let step = retirement.close_step(grant)?;
            if retirement.terminal_is_empty() { drop(self.retirement.take()); }
            return Ok(match step { PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) => PluginLifecycleStep::Progress(progress), other => other });
        }
        if self.rejected.is_some() {
            let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.rejected, grant)?;
            return Ok(PluginLifecycleStep::Progress(step.progress()));
        }
        if let Some(commit) = self.pending_commit.take() {
            match commit.into_retirement(grant) {
                Ok((owner, progress)) => {
                    *self.pending_commit_retirement = Some(owner);
                    return Ok(PluginLifecycleStep::Progress(progress));
                }
                Err((error, commit)) => { *self.pending_commit = Some(commit); return Err(error); }
            }
        }
        if self.pending_commit_retirement.is_some() {
            return semio_framework_value_retirement_owner::retirement_owner_close(&mut self.pending_commit_retirement, grant).map(|step| PluginLifecycleStep::Progress(step.progress()));
        }
        if self.pending_actor.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending_actor, &mut self.rejected, grant).map(|step| PluginLifecycleStep::Progress(step.progress())); }
        if self.pending_presence.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending_presence, &mut self.rejected, grant).map(|step| PluginLifecycleStep::Progress(step.progress())); }
        if self.pending_pack.is_some() { return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending_pack, &mut self.rejected, grant).map(|step| PluginLifecycleStep::Progress(step.progress())); }
        if !self.raw.terminal_is_empty() {
            return Ok(PluginLifecycleStep::Progress(self.raw.close_step(grant).progress()));
        }
        if self.pack_close.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.pack_close, grant).map(|step| PluginLifecycleStep::Progress(step.progress()));
        }
        if self.packs_len != 0 {
            *self.pending_pack = self.packs[self.packs_len - 1].take(); self.packs_len -= 1;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        self.release_packs_shell();
        if self.candidate.is_some() {
            let root = self.candidate.take().expect("original peer metadata candidate");
            *self.retirement = Some(PeerPresenceRootRetirement::owned(root));
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if let Some(typed) = self.typed.as_mut() {
            let step = typed.close_step(grant)?;
            if typed.terminal_is_empty() { drop(self.typed.take()); }
            return Ok(PluginLifecycleStep::Progress(step.progress()));
        }
        Ok(PluginLifecycleStep::Complete(empty))
    }

    pub(crate) fn take_original_fault(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Fault>, RetainedCloneProgress), ValueError> { self.fault.take_original(grant) }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.pending_commit.is_none() && self.pending_commit_retirement.is_none() && self.pending_actor.is_none() && self.pending_presence.is_none() && self.pending_pack.is_none() && self.raw.terminal_is_empty() && self.packs_shell_released && self.pack_close.is_none() && self.candidate.is_none() && self.retirement.is_none() && self.typed.is_none() && self.rejected.is_none()
    }
}

impl<A: ArtifactApp> Drop for PeerRosterPublication<A> {
    fn drop(&mut self) {
        debug_assert!(std::thread::panicking() || self.terminal_is_empty(), "incomplete peer roster publication reached Drop without bounded disposal");
    }
}
