//! 📑️ Jack snapshot clone: the retained snapshot clone authority and its step machine.

use super::*;

//#region 🔖️RetainedStoreInitialization
#[derive(semio_framework_value::RetireOwned)]
enum JackSnapshotCloneKind {
    Node { source: usize, property: usize, port: usize, value: crate::standards::v1::subsets::any::schema::manifest::NodeKindDef },
    Edge { source: usize, property: usize, value: crate::standards::v1::subsets::any::schema::manifest::EdgeKindDef },
    Port { source: usize, property: usize, value: crate::standards::v1::subsets::any::schema::manifest::PortKindDef },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JackSnapshotCloneStep {
    Pending { copied_bytes: usize },
    Complete,
}

pub struct JackSnapshotCloneAuthority {
    pub value: std::mem::ManuallyDrop<Option<JackSnapshot>>,
    pub active: std::mem::ManuallyDrop<Option<JackSnapshotCloneKind>>,
    pub retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub phase: u8,
    pub index: usize,
    retain_local_owner: bool,
    pub terminal: bool,
}

impl Default for JackSnapshotCloneAuthority {
    fn default() -> Self {
        Self::new()
    }
}

impl JackSnapshotCloneAuthority {
    pub fn new() -> Self {
        Self::with_local_owner(true)
    }

    pub fn metadata_only() -> Self {
        Self::with_local_owner(false)
    }

    fn with_local_owner(retain_local_owner: bool) -> Self {
        let content = store::os_store::artifact_child::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } });
        Self {
            value: std::mem::ManuallyDrop::new(Some(JackSnapshot { schema: String::new(), name: String::new(), manifest_id: None, manifest: Default::default(), camera: Default::default(), content, root_node_id: None, query: String::new() })),
            active: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            phase: 0,
            index: 0,
            retain_local_owner,
            terminal: false,
        }
    }

    fn clone_string(source: &str, maximum_bytes: usize) -> Result<String, ValueError> {
        if source.len() > maximum_bytes {
            return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"jack-store.initializer-field-too-large"));
        }
        let mut value = String::new();
        value.try_reserve_exact(source.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-string-admission"))?;
        value.push_str(source);
        Ok(value)
    }

    fn value_type_owned_bytes(source: &semio_framework_value::ValueType, maximum_bytes: usize) -> Result<usize, ValueError> {
        let mut bytes = 0usize;
        let mut value = source;
        loop {
            match value {
                semio_framework_value::ValueType::List(inner) => {
                    bytes = bytes.checked_add(size_of::<Box<semio_framework_value::ValueType>>()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"jack-store.initializer-property-size"))?;
                    if bytes > maximum_bytes {
                        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"jack-store.initializer-property-too-large"));
                    }
                    value = inner;
                }
                semio_framework_value::ValueType::Schema(schema) => {
                    bytes = bytes.checked_add(schema.len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"jack-store.initializer-property-size"))?;
                    return (bytes <= maximum_bytes).then_some(bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"jack-store.initializer-property-too-large"));
                }
                _ => return Ok(bytes),
            }
        }
    }

    fn property_owned_bytes(source: &PropertyDef, maximum_bytes: usize) -> Result<usize, ValueError> {
        let value_type = Self::value_type_owned_bytes(&source.value_type, maximum_bytes)?;
        source.name.len().checked_add(source.expr.as_ref().map_or(0, String::len)).and_then(|bytes| bytes.checked_add(value_type)).filter(|bytes| *bytes <= maximum_bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"jack-store.initializer-property-too-large"))
    }

    fn clone_property(source: &PropertyDef, maximum_bytes: usize) -> Result<PropertyDef, ValueError> {
        Self::property_owned_bytes(source, maximum_bytes)?;
        Ok(source.clone())
    }

    fn begin_kind(&mut self, source: &JackSnapshot, maximum_bytes: usize) -> Result<bool, ValueError> {
        let target = self.value.as_mut().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"jack-store.initializer-clone-target"))?;
        match self.phase {
            4 => {
                if self.index == 0 && target.manifest.node_kinds.capacity() == 0 {
                    target.manifest.node_kinds.try_reserve_exact(source.manifest.node_kinds.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-node-kind-admission"))?;
                }
                let Some(kind) = source.manifest.node_kinds.get(self.index) else {
                    self.phase = 5;
                    self.index = 0;
                    return Ok(true);
                };
                let mut properties = Vec::new();
                properties.try_reserve_exact(kind.properties.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-node-property-admission"))?;
                let mut port_kinds = Vec::new();
                port_kinds.try_reserve_exact(kind.port_kinds.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-node-port-admission"))?;
                *self.active =
                    Some(JackSnapshotCloneKind::Node { source: self.index, property: 0, port: 0, value: crate::standards::v1::subsets::any::schema::manifest::NodeKindDef { name: Self::clone_string(&kind.name, maximum_bytes)?, properties, port_kinds } });
                Ok(true)
            }
            5 => {
                if self.index == 0 && target.manifest.edge_kinds.capacity() == 0 {
                    target.manifest.edge_kinds.try_reserve_exact(source.manifest.edge_kinds.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-edge-kind-admission"))?;
                }
                let Some(kind) = source.manifest.edge_kinds.get(self.index) else {
                    self.phase = 6;
                    self.index = 0;
                    return Ok(true);
                };
                let mut properties = Vec::new();
                properties.try_reserve_exact(kind.properties.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-edge-property-admission"))?;
                *self.active = Some(JackSnapshotCloneKind::Edge { source: self.index, property: 0, value: crate::standards::v1::subsets::any::schema::manifest::EdgeKindDef { name: Self::clone_string(&kind.name, maximum_bytes)?, properties } });
                Ok(true)
            }
            6 => {
                if self.index == 0 && target.manifest.port_kinds.capacity() == 0 {
                    target.manifest.port_kinds.try_reserve_exact(source.manifest.port_kinds.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-port-kind-admission"))?;
                }
                let Some(kind) = source.manifest.port_kinds.get(self.index) else {
                    self.phase = 7;
                    self.index = 0;
                    return Ok(true);
                };
                let mut properties = Vec::new();
                properties.try_reserve_exact(kind.properties.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-port-property-admission"))?;
                *self.active =
                    Some(JackSnapshotCloneKind::Port { source: self.index, property: 0, value: crate::standards::v1::subsets::any::schema::manifest::PortKindDef { name: Self::clone_string(&kind.name, maximum_bytes)?, direction: kind.direction, properties } });
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    pub fn advance(&mut self, source: &JackSnapshot, maximum_bytes: usize) -> Result<JackSnapshotCloneStep, ValueError> {
        if maximum_bytes == 0 {
            return Ok(JackSnapshotCloneStep::Pending { copied_bytes: 0 });
        }
        if let Some(active) = self.active.as_mut() {
            let (completed, copied_bytes) = match active {
                JackSnapshotCloneKind::Node { source: source_index, property, port, value } => {
                    let source = source.manifest.node_kinds.get(*source_index).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"jack-store.initializer-node-kind-stale"))?;
                    if let Some(definition) = source.properties.get(*property) {
                        value.properties.push(Self::clone_property(definition, maximum_bytes)?);
                        *property += 1;
                        (false, Self::property_owned_bytes(definition, maximum_bytes)?)
                    } else if let Some(kind) = source.port_kinds.get(*port) {
                        value.port_kinds.push(Self::clone_string(kind, maximum_bytes)?);
                        *port += 1;
                        (false, kind.len())
                    } else {
                        (true, 0)
                    }
                }
                JackSnapshotCloneKind::Edge { source: source_index, property, value } => {
                    let source = source.manifest.edge_kinds.get(*source_index).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"jack-store.initializer-edge-kind-stale"))?;
                    if let Some(definition) = source.properties.get(*property) {
                        value.properties.push(Self::clone_property(definition, maximum_bytes)?);
                        *property += 1;
                        (false, Self::property_owned_bytes(definition, maximum_bytes)?)
                    } else {
                        (true, 0)
                    }
                }
                JackSnapshotCloneKind::Port { source: source_index, property, value } => {
                    let source = source.manifest.port_kinds.get(*source_index).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"jack-store.initializer-port-kind-stale"))?;
                    if let Some(definition) = source.properties.get(*property) {
                        value.properties.push(Self::clone_property(definition, maximum_bytes)?);
                        *property += 1;
                        (false, Self::property_owned_bytes(definition, maximum_bytes)?)
                    } else {
                        (true, 0)
                    }
                }
            };
            if completed {
                let active = self.active.take().expect("completed Jack kind clone remains exact");
                let target = self.value.as_mut().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"jack-store.initializer-clone-target"))?;
                match active {
                    JackSnapshotCloneKind::Node { value, .. } => target.manifest.node_kinds.push(value),
                    JackSnapshotCloneKind::Edge { value, .. } => target.manifest.edge_kinds.push(value),
                    JackSnapshotCloneKind::Port { value, .. } => target.manifest.port_kinds.push(value),
                }
                self.index += 1;
            }
            return Ok(JackSnapshotCloneStep::Pending { copied_bytes });
        }
        if self.begin_kind(source, maximum_bytes)? {
            return Ok(JackSnapshotCloneStep::Pending { copied_bytes: 0 });
        }
        let target = self.value.as_mut().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"jack-store.initializer-clone-target"))?;
        let copied_bytes = match self.phase {
            0 => {
                target.schema = Self::clone_string(&source.schema, maximum_bytes)?;
                source.schema.len()
            }
            1 => {
                target.name = Self::clone_string(&source.name, maximum_bytes)?;
                source.name.len()
            }
            2 => {
                target.manifest_id = source.manifest_id.as_deref().map(|value| Self::clone_string(value, maximum_bytes)).transpose()?;
                source.manifest_id.as_deref().map_or(0, str::len)
            }
            3 => {
                target.camera = source.camera.clone();
                0
            }
            7 => {
                target.content.child_id = Self::clone_string(&source.content.child_id, maximum_bytes)?;
                if self.retain_local_owner {
                    if let Some(owner) = source.content.local_owner::<crate::standards::v1::subsets::any::schema::content::JackContentOwner>() {
                        target.content.set_local_owner(owner);
                    }
                }
                source.content.child_id.len()
            }
            8 => {
                target.content.target.artifact_id = Self::clone_string(&source.content.target.artifact_id, maximum_bytes)?;
                source.content.target.artifact_id.len()
            }
            9 => {
                target.content.target.dialect.artifact_kind = Self::clone_string(&source.content.target.dialect.artifact_kind, maximum_bytes)?;
                source.content.target.dialect.artifact_kind.len()
            }
            10 => {
                target.content.target.dialect.standard = Self::clone_string(&source.content.target.dialect.standard, maximum_bytes)?;
                source.content.target.dialect.standard.len()
            }
            11 => {
                target.content.target.dialect.subset = Self::clone_string(&source.content.target.dialect.subset, maximum_bytes)?;
                source.content.target.dialect.subset.len()
            }
            12 => {
                target.root_node_id = source.root_node_id.as_deref().map(|value| Self::clone_string(value, maximum_bytes)).transpose()?;
                source.root_node_id.as_deref().map_or(0, str::len)
            }
            13 => {
                target.query = Self::clone_string(&source.query, maximum_bytes)?;
                source.query.len()
            }
            _ => {
                self.terminal = true;
                return Ok(JackSnapshotCloneStep::Complete);
            }
        };
        self.phase += 1;
        Ok(JackSnapshotCloneStep::Pending { copied_bytes })
    }

    pub fn step(&mut self, source: &JackSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, ValueError> {
        let step = self.advance(source, JACK_OWNED_FIELD_BYTES)?;
        match step {
            JackSnapshotCloneStep::Pending { copied_bytes } => {
                cx.consume_fuel(copied_bytes.max(1) as u64);
                Ok(false)
            }
            JackSnapshotCloneStep::Complete => Ok(true),
        }
    }

    pub fn take_value(&mut self) -> Option<JackSnapshot> {
        if !self.terminal || self.active.is_some() {
            return None;
        }
        self.value.take()
    }

    /// 📏️ Quotes the next close turn of the partial clone: the retained child cursor, else the partial kind, else the partial snapshot.
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(owner) = self.retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body);
        }
        if self.active.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.active);
        }
        if self.value.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.value);
        }
        Ok(Default::default())
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if !admit_demand(grant, demand, "jack snapshot clone close exceeds admitted depth")? {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if self.retirement.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.active.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.active, &mut self.retirement, grant);
        }
        if self.value.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant);
        }
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(empty))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.active.is_none() && self.retirement.is_none()
    }
}

impl Drop for JackSnapshotCloneAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "Jack snapshot clone reached Drop before exact handoff or cursor retirement");
    }
}
//#endregion 🔖️RetainedStoreInitialization
