//! 🌿️ Explicit native VCS repository text admission and emission.
use crate::io::binary::genesis::{AdmittedArtifactGenesis,ArtifactGenesisProducer};
use semio_framework_value::{DslValue,FromValue,ToValue,ValueError};

use crate::{ArtifactVcs,ArtifactHistoryLedger,ArtifactVcsRead,ArtifactGroupVisibility};
/// 🌿️ The original VCS fields admit stored genesis through the caller's selected native producer.
pub fn native_vcs_from_value<P,Mutation:FromValue>(value:DslValue,producer:&impl ArtifactGenesisProducer<P>)->Result<ArtifactVcs<AdmittedArtifactGenesis<P>,Mutation>,ValueError>{
        let DslValue::Object(fields) = value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "VCS requires an object")); };
        let mut admitted = std::collections::BTreeMap::new();
        for (key, value) in fields { if admitted.insert(key, value).is_some() { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "VCS repeats a field")); } }
        let mut fields = admitted;
        let mut take = |key: &str| fields.remove(key).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("VCS missing {key}")));
        let genesis = crate::io::text::genesis::native_genesis_from_value(take("initialPack")?,producer)?;
        let edits = ArtifactHistoryLedger::from_value(take("edits")?)?;
        let changes = ArtifactHistoryLedger::from_value(take("changes")?)?;
        let checkpoints = ArtifactHistoryLedger::from_value(take("checkpoints")?)?;
        let alternatives = ArtifactHistoryLedger::from_value(take("alternatives")?)?;
        if !fields.is_empty() { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "VCS has unknown fields")); }
        Ok(ArtifactVcs { genesis, edits, changes, checkpoints, alternatives })
}

pub fn native_vcs_read_value<P,M:ToValue>(read:&ArtifactVcsRead<'_,AdmittedArtifactGenesis<P>,M>)->DslValue{DslValue::object([("initialPack".into(),read.genesis.to_value()),("edits".into(),read.edits.to_value()),("changes".into(),read.changes.to_value()),("checkpoints".into(),read.checkpoints.to_value()),("alternatives".into(),read.alternatives.to_value())])}
pub fn native_vcs_value<P,M:ToValue>(vcs:&ArtifactVcs<AdmittedArtifactGenesis<P>,M>)->DslValue{let decision=vcs.group_visibility().expect("single VCS group authority").map(ArtifactGroupVisibility::capture);native_vcs_read_value(&vcs.read_group(decision.as_ref()).expect("captured VCS group authority"))}
