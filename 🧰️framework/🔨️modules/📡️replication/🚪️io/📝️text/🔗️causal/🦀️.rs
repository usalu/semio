//! ✉️ Causal envelope and frontier value representations.
use crate::causal::{ArtifactDiff, FrontierComparison, FrontierSummary, InverseMutation, MutationEnvelope};

impl crate::value::ToValue for MutationEnvelope {
    fn to_value(&self) -> crate::value::DslValue {
        let mut entries = vec![
            ("mutationId".to_string(), crate::value::ToValue::to_value(&self.mutation_id)),
            ("documentId".to_string(), crate::value::ToValue::to_value(&self.document_id)),
            ("actor".to_string(), crate::value::ToValue::to_value(&self.actor)),
            ("dependencies".to_string(), crate::value::ToValue::to_value(&self.dependencies)),
            ("observed".to_string(), crate::value::ToValue::to_value(&self.observed)),
            ("target".to_string(), crate::value::ToValue::to_value(&self.target)),
            ("diff".to_string(), crate::value::ToValue::to_value(&self.diff)),
            ("inverse".to_string(), crate::value::ToValue::to_value(&self.inverse)),
            ("timestamp".to_string(), crate::value::ToValue::to_value(&self.timestamp)),
        ];
        if self.transaction.is_some() {
            entries.push(("transaction".to_string(), crate::value::ToValue::to_value(&self.transaction)));
        }
        if self.verb.is_some() {
            entries.push(("verb".to_string(), crate::value::ToValue::to_value(&self.verb)));
        }
        if self.line.is_some() {
            entries.push(("line".to_string(), crate::value::ToValue::to_value(&self.line)));
        }
        crate::value::DslValue::object(entries)
    }
}

impl crate::value::FromValue for MutationEnvelope {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected an object for MutationEnvelope, found {value:?}")));
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
                "mutationId" => mutation_id = Some(<crate::ids::MutationId as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("mutationId"))?),
                "documentId" => document_id = Some(<crate::ids::ArtifactId as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("documentId"))?),
                "actor" => actor = Some(<crate::ids::ActorId as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("actor"))?),
                "dependencies" => dependencies = Some(<Vec<crate::ids::MutationId> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("dependencies"))?),
                "observed" => observed = Some(<Option<crate::ids::MutationId> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("observed"))?),
                "target" => target = Some(<Vec<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("target"))?),
                "diff" => diff = Some(<ArtifactDiff as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("diff"))?),
                "inverse" => inverse = Some(<InverseMutation as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("inverse"))?),
                "timestamp" => timestamp = Some(<crate::ids::HybridLogicalTimestamp as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("timestamp"))?),
                "transaction" => transaction = <Option<crate::mutation::TransactionRef> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("transaction"))?,
                "verb" => verb = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("verb"))?,
                "line" => line = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("line"))?,
                _ => {}
            }
        }
        Ok(MutationEnvelope {
            mutation_id: mutation_id.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing mutationId"))?,
            document_id: document_id.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing documentId"))?,
            actor: actor.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing actor"))?,
            dependencies: dependencies.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing dependencies"))?,
            observed: observed.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing observed"))?,
            target: target.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing target"))?,
            diff: diff.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing diff"))?,
            inverse: inverse.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing inverse"))?,
            timestamp: timestamp.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MutationEnvelope missing timestamp"))?,
            transaction,
            verb,
            line,
        })
    }
}

impl crate::value::ToValue for ArtifactDiff {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![("schema".to_string(), crate::value::ToValue::to_value(&self.schema)), ("payload".to_string(), crate::value::ToValue::to_value(&self.payload))])
    }
}

impl crate::value::FromValue for ArtifactDiff {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected an object for ArtifactDiff, found {value:?}")));
        };
        let mut schema = None;
        let mut payload = None;
        for (key, entry) in fields {
            match key.as_str() {
                "schema" => schema = Some(<crate::ids::SchemaId as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("schema"))?),
                "payload" => payload = Some(<Vec<u8> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("payload"))?),
                _ => {}
            }
        }
        Ok(ArtifactDiff { schema: schema.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "ArtifactDiff missing schema"))?, payload: payload.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "ArtifactDiff missing payload"))? })
    }
}

impl crate::value::ToValue for InverseMutation {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![("schema".to_string(), crate::value::ToValue::to_value(&self.schema)), ("payload".to_string(), crate::value::ToValue::to_value(&self.payload))])
    }
}

impl crate::value::FromValue for InverseMutation {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected an object for InverseMutation, found {value:?}")));
        };
        let mut schema = None;
        let mut payload = None;
        for (key, entry) in fields {
            match key.as_str() {
                "schema" => schema = Some(<crate::ids::SchemaId as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("schema"))?),
                "payload" => payload = Some(<Vec<u8> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("payload"))?),
                _ => {}
            }
        }
        Ok(InverseMutation { schema: schema.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "InverseMutation missing schema"))?, payload: payload.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "InverseMutation missing payload"))? })
    }
}

impl crate::value::ToValue for FrontierSummary {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("document_id".to_string(), crate::value::ToValue::to_value(&self.document_id)),
            ("head_edit_ordinal".to_string(), crate::value::ToValue::to_value(&self.head_edit_ordinal)),
            ("head_edit_id".to_string(), crate::value::ToValue::to_value(&self.head_edit_id)),
            ("last_commit_seq".to_string(), crate::value::ToValue::to_value(&self.last_commit_seq)),
            ("chain_hash".to_string(), crate::value::DslValue::Array(self.chain_hash.iter().map(crate::value::ToValue::to_value).collect())),
        ])
    }
}

impl crate::value::FromValue for FrontierSummary {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected an object for FrontierSummary, found {value:?}")));
        };
        let mut document_id = None;
        let mut head_edit_ordinal = None;
        let mut head_edit_id = None;
        let mut last_commit_seq = None;
        let mut chain_hash = None;
        for (key, entry) in fields {
            match key.as_str() {
                "document_id" => document_id = Some(<crate::ids::ArtifactId as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("document_id"))?),
                "head_edit_ordinal" => head_edit_ordinal = Some(<u64 as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("head_edit_ordinal"))?),
                "head_edit_id" => head_edit_id = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("head_edit_id"))?),
                "last_commit_seq" => last_commit_seq = Some(<u64 as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("last_commit_seq"))?),
                "chain_hash" => {
                    let crate::value::DslValue::Array(items) = entry else {
                        return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "FrontierSummary.chain_hash must be an array").under("chain_hash"));
                    };
                    if items.len() != 32 {
                        return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected exactly 32 bytes for chain_hash, found {}", items.len())).under("chain_hash"));
                    }
                    let mut bytes = [0u8; 32];
                    for (index, item) in items.into_iter().enumerate() {
                        bytes[index] = <u8 as crate::value::FromValue>::from_value(item).map_err(|e| e.under(index).under("chain_hash"))?;
                    }
                    chain_hash = Some(bytes);
                }
                _ => {}
            }
        }
        Ok(FrontierSummary {
            document_id: document_id.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "FrontierSummary missing document_id"))?,
            head_edit_ordinal: head_edit_ordinal.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "FrontierSummary missing head_edit_ordinal"))?,
            head_edit_id: head_edit_id.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "FrontierSummary missing head_edit_id"))?,
            last_commit_seq: last_commit_seq.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "FrontierSummary missing last_commit_seq"))?,
            chain_hash: chain_hash.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "FrontierSummary missing chain_hash"))?,
        })
    }
}

impl crate::value::ToValue for FrontierComparison {
    fn to_value(&self) -> crate::value::DslValue {
        match self {
            FrontierComparison::Equal => crate::value::DslValue::String("Equal".to_string()),
            FrontierComparison::Ahead => crate::value::DslValue::String("Ahead".to_string()),
            FrontierComparison::Behind => crate::value::DslValue::String("Behind".to_string()),
            FrontierComparison::Diverged { common_edit_count } => {
                crate::value::DslValue::object(vec![("Diverged".to_string(), crate::value::DslValue::object(vec![("common_edit_count".to_string(), crate::value::ToValue::to_value(common_edit_count))]))])
            }
        }
    }
}

impl crate::value::FromValue for FrontierComparison {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        match value {
            crate::value::DslValue::String(s) => match s.as_str() {
                "Equal" => Ok(FrontierComparison::Equal),
                "Ahead" => Ok(FrontierComparison::Ahead),
                "Behind" => Ok(FrontierComparison::Behind),
                other => Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("unknown FrontierComparison variant `{other}`"))),
            },
            crate::value::DslValue::Object(entries) => {
                if entries.len() != 1 {
                    return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "expected an externally-tagged enum object with exactly one key"));
                }
                let (tag, payload) = entries.into_iter().next().unwrap();
                match tag.as_str() {
                    "Diverged" => {
                        let crate::value::DslValue::Object(fields) = payload else {
                            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "FrontierComparison.Diverged payload must be an object").under("Diverged"));
                        };
                        let common_edit_count = fields
                            .into_iter()
                            .find(|(k, _)| k == "common_edit_count")
                            .map(|(_, v)| <u64 as crate::value::FromValue>::from_value(v))
                            .transpose()
                            .map_err(|e: crate::value::ValueError| e.under("common_edit_count"))?
                            .ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "FrontierComparison.Diverged missing common_edit_count"))?;
                        Ok(FrontierComparison::Diverged { common_edit_count })
                    }
                    other => Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("unknown FrontierComparison variant `{other}`"))),
                }
            }
            other => Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected a string or object, found {other:?}"))),
        }
    }
}
