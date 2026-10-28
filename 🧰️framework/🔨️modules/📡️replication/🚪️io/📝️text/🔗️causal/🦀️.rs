//! ✉️ Causal envelope and frontier value representations.
use crate::{ArtifactDiff, FrontierComparison, FrontierSummary, InverseMutation, MutationEnvelope};

impl semio_framework_value::ToValue for MutationEnvelope {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = vec![
            ("mutationId".to_string(), semio_framework_value::ToValue::to_value(&self.mutation_id)),
            ("documentId".to_string(), semio_framework_value::ToValue::to_value(&self.document_id)),
            ("actor".to_string(), semio_framework_value::ToValue::to_value(&self.actor)),
            ("dependencies".to_string(), semio_framework_value::ToValue::to_value(&self.dependencies)),
            ("observed".to_string(), semio_framework_value::ToValue::to_value(&self.observed)),
            ("target".to_string(), semio_framework_value::ToValue::to_value(&self.target)),
            ("diff".to_string(), semio_framework_value::ToValue::to_value(&self.diff)),
            ("inverse".to_string(), semio_framework_value::ToValue::to_value(&self.inverse)),
            ("timestamp".to_string(), semio_framework_value::ToValue::to_value(&self.timestamp)),
        ];
        if self.transaction.is_some() {
            entries.push(("transaction".to_string(), semio_framework_value::ToValue::to_value(&self.transaction)));
        }
        if self.verb.is_some() {
            entries.push(("verb".to_string(), semio_framework_value::ToValue::to_value(&self.verb)));
        }
        if self.line.is_some() {
            entries.push(("line".to_string(), semio_framework_value::ToValue::to_value(&self.line)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}

impl semio_framework_value::FromValue for MutationEnvelope {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for MutationEnvelope, found {value:?}")));
        };
        let mut mutation_id = None;
        let mut document_id = None;
        let mut actor = None;
        let mut dependencies = None;
        let mut observed = None;
        let mut target = None;
        let mut diff = None;
        let mut inverse = None;
        let mut timestamp = None;
        let mut transaction = None;
        let mut verb = None;
        let mut line = None;
        for (key, entry) in fields {
            match key.as_str() {
                "mutationId" => mutation_id = Some(<semio_framework_replication_ids::MutationId as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("mutationId"))?),
                "documentId" => document_id = Some(<semio_framework_replication_ids::ArtifactId as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("documentId"))?),
                "actor" => actor = Some(<semio_framework_replication_ids::ActorId as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("actor"))?),
                "dependencies" => dependencies = Some(<Vec<semio_framework_replication_ids::MutationId> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("dependencies"))?),
                "observed" => observed = Some(<Option<semio_framework_replication_ids::MutationId> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("observed"))?),
                "target" => target = Some(<Vec<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("target"))?),
                "diff" => diff = Some(<ArtifactDiff as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("diff"))?),
                "inverse" => inverse = Some(<InverseMutation as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("inverse"))?),
                "timestamp" => timestamp = Some(<semio_framework_replication_ids::HybridLogicalTimestamp as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("timestamp"))?),
                "transaction" => transaction = <Option<semio_framework_replication_edit::TransactionRef> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("transaction"))?,
                "verb" => verb = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("verb"))?,
                "line" => line = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("line"))?,
                _ => {}
            }
        }
        Ok(MutationEnvelope {
            mutation_id: mutation_id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing mutationId"))?,
            document_id: document_id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing documentId"))?,
            actor: actor.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing actor"))?,
            dependencies: dependencies.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing dependencies"))?,
            observed: observed.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing observed"))?,
            target: target.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing target"))?,
            diff: diff.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing diff"))?,
            inverse: inverse.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing inverse"))?,
            timestamp: timestamp.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing timestamp"))?,
            transaction,
            verb,
            line,
        })
    }
}

impl semio_framework_value::ToValue for ArtifactDiff {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![("schema".to_string(), semio_framework_value::ToValue::to_value(&self.schema)), ("payload".to_string(), semio_framework_value::DslValue::Array(self.payload.iter().map(|byte|semio_framework_value::ToValue::to_value(&byte)).collect()))])
    }
}

impl semio_framework_value::FromValue for ArtifactDiff {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for ArtifactDiff, found {value:?}")));
        };
        let mut schema = None;
        let mut payload = None;
        for (key, entry) in fields {
            match key.as_str() {
                "schema" => schema = Some(<semio_framework_replication_ids::SchemaId as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("schema"))?),
                "payload" => payload = Some(<crate::CausalPayload as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("payload"))?),
                _ => {}
            }
        }
        Ok(ArtifactDiff { schema: schema.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "ArtifactDiff missing schema"))?, payload: payload.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "ArtifactDiff missing payload"))? })
    }
}

impl semio_framework_value::ToValue for InverseMutation {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![("schema".to_string(), semio_framework_value::ToValue::to_value(&self.schema)), ("payload".to_string(), semio_framework_value::DslValue::Array(self.payload.iter().map(|byte|semio_framework_value::ToValue::to_value(&byte)).collect()))])
    }
}

impl semio_framework_value::FromValue for InverseMutation {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for InverseMutation, found {value:?}")));
        };
        let mut schema = None;
        let mut payload = None;
        for (key, entry) in fields {
            match key.as_str() {
                "schema" => schema = Some(<semio_framework_replication_ids::SchemaId as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("schema"))?),
                "payload" => payload = Some(<crate::CausalPayload as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("payload"))?),
                _ => {}
            }
        }
        Ok(InverseMutation { schema: schema.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "InverseMutation missing schema"))?, payload: payload.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "InverseMutation missing payload"))? })
    }
}

impl semio_framework_value::ToValue for FrontierSummary {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![
            ("document_id".to_string(), semio_framework_value::ToValue::to_value(&self.document_id)),
            ("head_edit_ordinal".to_string(), semio_framework_value::ToValue::to_value(&self.head_edit_ordinal)),
            ("head_edit_id".to_string(), semio_framework_value::ToValue::to_value(&self.head_edit_id)),
            ("last_commit_seq".to_string(), semio_framework_value::ToValue::to_value(&self.last_commit_seq)),
            ("chain_hash".to_string(), semio_framework_value::DslValue::Array(self.chain_hash.iter().map(semio_framework_value::ToValue::to_value).collect())),
        ])
    }
}

impl semio_framework_value::FromValue for FrontierSummary {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for FrontierSummary, found {value:?}")));
        };
        let mut document_id = None;
        let mut head_edit_ordinal = None;
        let mut head_edit_id = None;
        let mut last_commit_seq = None;
        let mut chain_hash = None;
        for (key, entry) in fields {
            match key.as_str() {
                "document_id" => document_id = Some(<semio_framework_replication_ids::ArtifactId as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("document_id"))?),
                "head_edit_ordinal" => head_edit_ordinal = Some(<u64 as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("head_edit_ordinal"))?),
                "head_edit_id" => head_edit_id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("head_edit_id"))?),
                "last_commit_seq" => last_commit_seq = Some(<u64 as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("last_commit_seq"))?),
                "chain_hash" => {
                    let semio_framework_value::DslValue::Array(items) = entry else {
                        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "FrontierSummary.chain_hash must be an array").under("chain_hash"));
                    };
                    if items.len() != 32 {
                        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected exactly 32 bytes for chain_hash, found {}", items.len())).under("chain_hash"));
                    }
                    let mut bytes = [0u8; 32];
                    for (index, item) in items.into_iter().enumerate() {
                        bytes[index] = <u8 as semio_framework_value::FromValue>::from_value(item).map_err(|e| e.under(index).under("chain_hash"))?;
                    }
                    chain_hash = Some(bytes);
                }
                _ => {}
            }
        }
        Ok(FrontierSummary {
            document_id: document_id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "FrontierSummary missing document_id"))?,
            head_edit_ordinal: head_edit_ordinal.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "FrontierSummary missing head_edit_ordinal"))?,
            head_edit_id: head_edit_id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "FrontierSummary missing head_edit_id"))?,
            last_commit_seq: last_commit_seq.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "FrontierSummary missing last_commit_seq"))?,
            chain_hash: chain_hash.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "FrontierSummary missing chain_hash"))?,
        })
    }
}

impl semio_framework_value::ToValue for FrontierComparison {
    fn to_value(&self) -> semio_framework_value::DslValue {
        match self {
            FrontierComparison::Equal => semio_framework_value::DslValue::String("Equal".to_string()),
            FrontierComparison::Ahead => semio_framework_value::DslValue::String("Ahead".to_string()),
            FrontierComparison::Behind => semio_framework_value::DslValue::String("Behind".to_string()),
            FrontierComparison::Diverged { common_edit_count } => {
                semio_framework_value::DslValue::object(vec![("Diverged".to_string(), semio_framework_value::DslValue::object(vec![("common_edit_count".to_string(), semio_framework_value::ToValue::to_value(common_edit_count))]))])
            }
        }
    }
}

impl semio_framework_value::FromValue for FrontierComparison {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            semio_framework_value::DslValue::String(s) => match s.as_str() {
                "Equal" => Ok(FrontierComparison::Equal),
                "Ahead" => Ok(FrontierComparison::Ahead),
                "Behind" => Ok(FrontierComparison::Behind),
                other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown FrontierComparison variant `{other}`"))),
            },
            semio_framework_value::DslValue::Object(entries) => {
                if entries.len() != 1 {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an externally-tagged enum object with exactly one key"));
                }
                let (tag, payload) = entries.into_iter().next().unwrap();
                match tag.as_str() {
                    "Diverged" => {
                        let semio_framework_value::DslValue::Object(fields) = payload else {
                            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "FrontierComparison.Diverged payload must be an object").under("Diverged"));
                        };
                        let common_edit_count = fields
                            .into_iter()
                            .find(|(k, _)| k == "common_edit_count")
                            .map(|(_, v)| <u64 as semio_framework_value::FromValue>::from_value(v))
                            .transpose()
                            .map_err(|e: semio_framework_value::ValueError| e.under("common_edit_count"))?
                            .ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "FrontierComparison.Diverged missing common_edit_count"))?;
                        Ok(FrontierComparison::Diverged { common_edit_count })
                    }
                    other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown FrontierComparison variant `{other}`"))),
                }
            }
            other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string or object, found {other:?}"))),
        }
    }
}
