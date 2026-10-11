mod private_child_metadata {
    use super::*;
    use semio_framework_value::{SharedUtf8, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
    use std::mem::ManuallyDrop;

    #[derive(Clone, Copy)]
    pub(crate) struct PrivateChildMemberMetadataSource<'a> {
        pub(crate) expected: &'a ArtifactRef,
        pub(crate) parent_id: &'a str,
        pub(crate) parent_dialect: &'a ArtifactDialect,
        pub(crate) key: MemberKeyRef<'a>,
        pub(crate) actor: &'a str,
        pub(crate) transaction: Option<&'a protocol::TransactionRef>,
        pub(crate) group_id: Option<&'a str>,
    }

    impl<'a> PrivateChildMemberMetadataSource<'a> {
        fn fields(self) -> [&'a str; 31] {
            let (transaction_id, tool) = self.transaction.map(|transaction| (transaction.id.as_str(), transaction.tool.as_str())).unwrap_or(("", ""));
            [&self.expected.artifact_id, &self.expected.dialect.artifact_kind, &self.expected.dialect.standard, &self.expected.dialect.subset, self.parent_id, &self.parent_dialect.artifact_kind, &self.parent_dialect.standard, &self.parent_dialect.subset, self.key.slot, self.key.child_id, self.actor, self.key.owner, self.key.slot, self.key.child_id, &self.expected.artifact_id, &self.expected.dialect.artifact_kind, &self.expected.dialect.standard, &self.expected.dialect.subset, self.key.owner, self.key.slot, self.key.child_id, self.actor, transaction_id, tool, self.group_id.unwrap_or(""), self.parent_id, &self.parent_dialect.artifact_kind, &self.parent_dialect.standard, &self.parent_dialect.subset, self.key.slot, self.key.child_id]
        }
    }

    pub(crate) struct PrivateChildMemberMetadataParts {
        pub(crate) expected: ArtifactRef,
        pub(crate) owner: store::OwnerRef,
        pub(crate) actor: protocol::ActorId,
        pub(crate) key: MemberKey,
        pub(crate) prepared_identity: PreparedChildContentIdentity,
        pub(crate) publication_actor: SharedUtf8,
        pub(crate) transaction: Option<protocol::TransactionRef>,
        pub(crate) group_id: Option<String>,
        pub(crate) registry_owner: store::OwnerRef,
    }

    impl PrivateChildMemberMetadataParts {
        fn fields_mut(&mut self) -> [Option<&mut String>; 31] {
            let (transaction_id, tool) = self.transaction.as_mut().map(|transaction| (Some(&mut transaction.id), Some(&mut transaction.tool))).unwrap_or((None, None));
            [Some(&mut self.expected.artifact_id), Some(&mut self.expected.dialect.artifact_kind), Some(&mut self.expected.dialect.standard), Some(&mut self.expected.dialect.subset), Some(&mut self.owner.parent.artifact_id), Some(&mut self.owner.parent.dialect.artifact_kind), Some(&mut self.owner.parent.dialect.standard), Some(&mut self.owner.parent.dialect.subset), Some(&mut self.owner.slot), Some(&mut self.owner.child_id), None, Some(&mut self.key.owner), Some(&mut self.key.slot), Some(&mut self.key.child_id), Some(&mut self.prepared_identity.reference.artifact_id), Some(&mut self.prepared_identity.reference.dialect.artifact_kind), Some(&mut self.prepared_identity.reference.dialect.standard), Some(&mut self.prepared_identity.reference.dialect.subset), Some(&mut self.prepared_identity.key.owner), Some(&mut self.prepared_identity.key.slot), Some(&mut self.prepared_identity.key.child_id), None, transaction_id, tool, self.group_id.as_mut(), Some(&mut self.registry_owner.parent.artifact_id), Some(&mut self.registry_owner.parent.dialect.artifact_kind), Some(&mut self.registry_owner.parent.dialect.standard), Some(&mut self.registry_owner.parent.dialect.subset), Some(&mut self.registry_owner.slot), Some(&mut self.registry_owner.child_id)]
        }
        fn shared_fields(&self) -> [&SharedUtf8; 2] { [&self.actor.0, &self.publication_actor] }
        fn shared_fields_mut(&mut self) -> [&mut SharedUtf8; 2] { [&mut self.actor.0, &mut self.publication_actor] }
        fn fields(&self) -> [Option<&String>; 31] {
            [Some(&self.expected.artifact_id), Some(&self.expected.dialect.artifact_kind), Some(&self.expected.dialect.standard), Some(&self.expected.dialect.subset), Some(&self.owner.parent.artifact_id), Some(&self.owner.parent.dialect.artifact_kind), Some(&self.owner.parent.dialect.standard), Some(&self.owner.parent.dialect.subset), Some(&self.owner.slot), Some(&self.owner.child_id), None, Some(&self.key.owner), Some(&self.key.slot), Some(&self.key.child_id), Some(&self.prepared_identity.reference.artifact_id), Some(&self.prepared_identity.reference.dialect.artifact_kind), Some(&self.prepared_identity.reference.dialect.standard), Some(&self.prepared_identity.reference.dialect.subset), Some(&self.prepared_identity.key.owner), Some(&self.prepared_identity.key.slot), Some(&self.prepared_identity.key.child_id), None, self.transaction.as_ref().map(|transaction| &transaction.id), self.transaction.as_ref().map(|transaction| &transaction.tool), self.group_id.as_ref(), Some(&self.registry_owner.parent.artifact_id), Some(&self.registry_owner.parent.dialect.artifact_kind), Some(&self.registry_owner.parent.dialect.standard), Some(&self.registry_owner.parent.dialect.subset), Some(&self.registry_owner.slot), Some(&self.registry_owner.child_id)]
        }
    }

    pub(crate) struct PrivateChildMemberMetadata { parts: ManuallyDrop<Option<PrivateChildMemberMetadataParts>> }

    impl PrivateChildMemberMetadata {
        pub(crate) fn from_parts(parts: PrivateChildMemberMetadataParts) -> Self { Self { parts: ManuallyDrop::new(Some(parts)) } }
        pub(crate) fn parts(&self) -> Option<&PrivateChildMemberMetadataParts> { self.parts.as_ref() }
        pub(crate) fn parts_mut(&mut self) -> Option<&mut PrivateChildMemberMetadataParts> { self.parts.as_mut() }
        /// 🧳️ Moves the ready exact identities once into the retained typed request and content owners.
        pub(crate) fn take_ready(&mut self, grant: RetainedCloneGrant) -> Result<Option<PrivateChildMemberMetadataParts>, ValueError> { if grant.maximum_items == 0 { return Ok(None); } Ok(self.parts.take()) }
        pub(crate) fn next_close_byte_demand(&self) -> usize { self.parts.as_ref().and_then(|parts| parts.fields().into_iter().flatten().find_map(|field| (field.capacity() != 0).then_some(field.capacity())).or_else(|| parts.shared_fields().into_iter().find_map(|field| field.has_owner().then(|| field.original_allocation_bytes())))).unwrap_or(0) }
        pub(crate) fn terminal_is_empty(&self) -> bool { self.parts.is_none() }
        /// 🪵️ Releases a ready bundle's original string allocation under one whole release grant.
        pub(crate) fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
            if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            if let Some(field) = self.parts.as_mut().unwrap().fields_mut().into_iter().flatten().find(|field| field.capacity() != 0) {
                let bytes = field.capacity();
                if grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                drop(std::mem::take(field));
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }));
            }
            if let Some(field) = self.parts.as_mut().unwrap().shared_fields_mut().into_iter().find(|field| field.has_owner()) {
                let bytes = field.original_allocation_bytes();
                if grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                drop(std::mem::take(field));
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }));
            }
            drop(self.parts.take());
            Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
        }
    }

    impl Drop for PrivateChildMemberMetadata { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "private child metadata retains every string until typed transfer or bounded close"); } }

    fn fault(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }

    struct PrivateMetadataStringIssuer<const N: usize> {
        fields: ManuallyDrop<[Option<String>; N]>,
        active: ManuallyDrop<Option<Vec<u8>>>,
        source: Option<[(usize, usize); N]>,
        field: usize,
        closing: bool,
    }

    impl<const N: usize> PrivateMetadataStringIssuer<N> {
        fn new() -> Self { Self { fields: ManuallyDrop::new(std::array::from_fn(|_| None)), active: ManuallyDrop::new(None), source: None, field: 0, closing: false } }
        fn ready(&self) -> bool { !self.closing && self.field == N && self.fields.iter().all(Option::is_some) }
        fn next_capacity_byte_demand(&self, source: [&str; N]) -> usize { if self.closing || self.field == N || self.active.is_some() { 0 } else { source[self.field].len() } }
        /// 🧶️ Copies one borrowed metadata field with complete capacity admission and a bounded byte prefix.
        fn advance(&mut self, fields: [&str; N], grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            if self.closing { return Err(fault("private metadata issuer is closing")); }
            if self.ready() { return Ok(RetainedCloneStep::Complete(Default::default())); }
            if fields.iter().any(|field| field.len() > store::MEMBER_OPEN_IDENTITY_BYTES) { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "private metadata exceeds member-open identity admission")); }
            let seal = fields.map(|field| (field.as_ptr() as usize, field.len()));
            if self.source.is_some_and(|original| original != seal) { return Err(fault("private metadata requires its exact borrowed originals")); }
            let source = fields[self.field].as_bytes();
            if self.active.is_none() {
                if grant.maximum_capacity_bytes < source.len() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                *self.active = Some(Vec::with_capacity(source.len()));
                self.source = Some(seal);
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: source.len(), ..Default::default() }));
            }
            let active = self.active.as_mut().unwrap();
            let copied = (source.len() - active.len()).min(grant.maximum_copy_bytes).min(64);
            if copied == 0 && active.len() != source.len() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            active.extend_from_slice(&source[active.len()..active.len() + copied]);
            if active.len() == source.len() { self.complete_field(); }
            Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: copied, ..Default::default() }))
        }
        /// 🧬️ Converts only a complete byte-for-byte copy of one original valid UTF-8 string.
        fn complete_field(&mut self) { self.fields[self.field] = Some(unsafe { String::from_utf8_unchecked(self.active.take().unwrap()) }); self.field += 1; }
        fn take_ready(&mut self, grant: RetainedCloneGrant) -> Option<[String; N]> {
            if !self.ready() || grant.maximum_items == 0 { return None; }
            let fields = std::array::from_fn(|index| self.fields[index].take().unwrap());
            self.closing = true;
            Some(fields)
        }
        fn next_close_byte_demand(&self) -> usize { self.active.as_ref().map(Vec::capacity).or_else(|| self.fields.iter().flatten().next().map(String::capacity)).unwrap_or(0) }
        fn terminal_is_empty(&self) -> bool { self.active.is_none() && self.fields.iter().all(Option::is_none) }
        /// 🪵️ Retains an original backing until one work item and its whole physical release are funded.
        fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
            if self.terminal_is_empty() { self.closing = true; return Ok(RetainedCloneStep::Complete(Default::default())); }
            if grant.maximum_items == 0 || grant.maximum_release_bytes < self.next_close_byte_demand() { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.closing = true;
            let bytes = self.next_close_byte_demand();
            if self.active.is_some() { drop(self.active.take()); }
            else { drop(self.fields.iter_mut().find(|field| field.is_some()).unwrap().take()); }
            Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }))
        }
    }

    impl<const N: usize> Drop for PrivateMetadataStringIssuer<N> { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "private metadata strings require guarded transfer or funded physical close"); } }

    fn advance_metadata<const N: usize>(strings: &mut PrivateMetadataStringIssuer<N>, transaction: &mut Option<bool>, group: &mut Option<bool>, fields: [&str; N], present: bool, group_present: bool, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if transaction.is_some_and(|original| original != present) { return Err(fault("private metadata retains exact transaction presence")); }
        if group.is_some_and(|original| original != group_present) { return Err(fault("private metadata retains exact group identity presence")); }
        let step = strings.advance(fields, grant)?;
        if step.progress().copied_items != 0 { *transaction = Some(present); *group = Some(group_present); }
        Ok(step)
    }

    pub(crate) struct PrivateChildMemberMetadataIssuer { strings: PrivateMetadataStringIssuer<31>, transaction_present: Option<bool>, group_present: Option<bool> }

    impl PrivateChildMemberMetadataIssuer {
        pub(crate) fn new() -> Self { Self { strings: PrivateMetadataStringIssuer::new(), transaction_present: None, group_present: None } }
        pub(crate) fn ready(&self) -> bool { self.strings.ready() }
        pub(crate) fn next_capacity_byte_demand(&self, source: PrivateChildMemberMetadataSource<'_>) -> Option<usize> { Some(self.strings.next_capacity_byte_demand(source.fields())) }
        /// 🫴️ Shares the exact bounded string issuer with the parent publication metadata path.
        pub(crate) fn advance(&mut self, source: PrivateChildMemberMetadataSource<'_>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { advance_metadata(&mut self.strings, &mut self.transaction_present, &mut self.group_present, source.fields(), source.transaction.is_some(), source.group_id.is_some(), grant) }
        /// 🧳️ Transfers every prebuilt child allocation into one guarded typed metadata bundle.
        pub(crate) fn take_ready(&mut self, grant: RetainedCloneGrant) -> Result<Option<PrivateChildMemberMetadata>, ValueError> {
            let Some([id, kind, standard, subset, parent, parent_kind, parent_standard, parent_subset, slot, child, actor, key_owner, key_slot, key_child, prepared_id, prepared_kind, prepared_standard, prepared_subset, prepared_owner, prepared_slot, prepared_child, publication_actor, transaction_id, tool, group_id, registry_parent, registry_kind, registry_standard, registry_subset, registry_slot, registry_child]) = self.strings.take_ready(grant) else { return Ok(None); };
            let transaction = (self.transaction_present == Some(true)).then_some(protocol::TransactionRef { id: transaction_id, tool });
            let group_id = (self.group_present == Some(true)).then_some(group_id);
            Ok(Some(PrivateChildMemberMetadata { parts: ManuallyDrop::new(Some(PrivateChildMemberMetadataParts { expected: ArtifactRef { artifact_id: id, dialect: ArtifactDialect { artifact_kind: kind, standard, subset } }, owner: store::OwnerRef { parent: ArtifactRef { artifact_id: parent, dialect: ArtifactDialect { artifact_kind: parent_kind, standard: parent_standard, subset: parent_subset } }, slot, child_id: child }, actor: protocol::ActorId(actor.into()), key: MemberKey { owner: key_owner, slot: key_slot, child_id: key_child }, prepared_identity: PreparedChildContentIdentity { key: MemberKey { owner: prepared_owner, slot: prepared_slot, child_id: prepared_child }, reference: ArtifactRef { artifact_id: prepared_id, dialect: ArtifactDialect { artifact_kind: prepared_kind, standard: prepared_standard, subset: prepared_subset } } }, publication_actor: publication_actor.into(), transaction, group_id, registry_owner: store::OwnerRef { parent: ArtifactRef { artifact_id: registry_parent, dialect: ArtifactDialect { artifact_kind: registry_kind, standard: registry_standard, subset: registry_subset } }, slot: registry_slot, child_id: registry_child } })) }))
        }
        pub(crate) fn next_close_byte_demand(&self) -> usize { self.strings.next_close_byte_demand() }
        pub(crate) fn terminal_is_empty(&self) -> bool { self.strings.terminal_is_empty() }
        pub(crate) fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.strings.close_granted(grant) }
    }

    #[derive(Clone, Copy)]
    pub(crate) struct PrivatePublicationMetadataSource<'a> { pub(crate) actor: &'a str, pub(crate) transaction: Option<&'a protocol::TransactionRef>, pub(crate) group_id: Option<&'a str> }

    impl<'a> PrivatePublicationMetadataSource<'a> {
        fn fields(self) -> [&'a str; 4] { let (id, tool) = self.transaction.map(|transaction| (transaction.id.as_str(), transaction.tool.as_str())).unwrap_or(("", "")); [self.actor, id, tool, self.group_id.unwrap_or("")] }
    }

    pub(crate) struct PrivatePublicationMetadataParts { pub(crate) actor: SharedUtf8, pub(crate) transaction: Option<protocol::TransactionRef>, pub(crate) group_id: Option<String> }

    impl PrivatePublicationMetadataParts {
        fn actor_if_owned(&mut self) -> Option<&mut SharedUtf8> { self.actor.has_owner().then_some(&mut self.actor) }
        fn fields(&self) -> [Option<&String>; 4] { [None, self.transaction.as_ref().map(|transaction| &transaction.id), self.transaction.as_ref().map(|transaction| &transaction.tool), self.group_id.as_ref()] }
        fn fields_mut(&mut self) -> [Option<&mut String>; 4] { let (id, tool) = self.transaction.as_mut().map(|transaction| (Some(&mut transaction.id), Some(&mut transaction.tool))).unwrap_or((None, None)); [None, id, tool, self.group_id.as_mut()] }
    }

    pub(crate) struct PrivatePublicationMetadata { parts: ManuallyDrop<Option<PrivatePublicationMetadataParts>> }

    impl PrivatePublicationMetadata {
        pub(crate) fn from_parts(parts: PrivatePublicationMetadataParts) -> Self { Self { parts: ManuallyDrop::new(Some(parts)) } }
        pub(crate) fn parts(&self) -> Option<&PrivatePublicationMetadataParts> { self.parts.as_ref() }
        pub(crate) fn parts_mut(&mut self) -> Option<&mut PrivatePublicationMetadataParts> { self.parts.as_mut() }
        pub(crate) fn take_ready(&mut self, grant: RetainedCloneGrant) -> Result<Option<PrivatePublicationMetadataParts>, ValueError> { if grant.maximum_items == 0 { return Ok(None); } Ok(self.parts.take()) }
        pub(crate) fn next_close_byte_demand(&self) -> usize { self.parts.as_ref().and_then(|parts| parts.fields().into_iter().flatten().find_map(|field| (field.capacity() != 0).then_some(field.capacity())).or_else(|| parts.actor.has_owner().then(|| parts.actor.original_allocation_bytes()))).unwrap_or(0) }
        pub(crate) fn terminal_is_empty(&self) -> bool { self.parts.is_none() }
        /// 🧺️ Closes one whole original parent metadata allocation or its empty inline bundle.
        pub(crate) fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
            if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            if let Some(field) = self.parts.as_mut().unwrap().fields_mut().into_iter().flatten().find(|field| field.capacity() != 0) {
                let bytes = field.capacity();
                if grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                drop(std::mem::take(field));
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }));
            }
            if let Some(actor) = self.parts.as_mut().unwrap().actor_if_owned() {
                let bytes = actor.original_allocation_bytes();
                if grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                drop(std::mem::take(actor));
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }));
            }
            drop(self.parts.take());
            Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
        }
    }

    impl Drop for PrivatePublicationMetadata { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "parent publication metadata retains originals until typed transfer or bounded close"); } }

    pub(crate) struct PrivatePublicationMetadataIssuer { strings: PrivateMetadataStringIssuer<4>, transaction_present: Option<bool>, group_present: Option<bool> }

    impl PrivatePublicationMetadataIssuer {
        pub(crate) fn new() -> Self { Self { strings: PrivateMetadataStringIssuer::new(), transaction_present: None, group_present: None } }
        pub(crate) fn ready(&self) -> bool { self.strings.ready() }
        pub(crate) fn next_capacity_byte_demand(&self, source: PrivatePublicationMetadataSource<'_>) -> Option<usize> { Some(self.strings.next_capacity_byte_demand(source.fields())) }
        pub(crate) fn advance(&mut self, source: PrivatePublicationMetadataSource<'_>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { advance_metadata(&mut self.strings, &mut self.transaction_present, &mut self.group_present, source.fields(), source.transaction.is_some(), source.group_id.is_some(), grant) }
        /// 👜️ Moves the four admitted original parent fields without metadata clones or heap birth.
        pub(crate) fn take_ready(&mut self, grant: RetainedCloneGrant) -> Result<Option<PrivatePublicationMetadata>, ValueError> {
            let Some([actor, id, tool, group_id]) = self.strings.take_ready(grant) else { return Ok(None); };
            let transaction = (self.transaction_present == Some(true)).then_some(protocol::TransactionRef { id, tool });
            let group_id = (self.group_present == Some(true)).then_some(group_id);
            Ok(Some(PrivatePublicationMetadata { parts: ManuallyDrop::new(Some(PrivatePublicationMetadataParts { actor: actor.into(), transaction, group_id })) }))
        }
        pub(crate) fn next_close_byte_demand(&self) -> usize { self.strings.next_close_byte_demand() }
        pub(crate) fn terminal_is_empty(&self) -> bool { self.strings.terminal_is_empty() }
        pub(crate) fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.strings.close_granted(grant) }
    }

    #[cfg(test)]
    include!("🧪️tests/🔬️unit/🦀️.rs");
}

pub(crate) use private_child_metadata::{PrivateChildMemberMetadata, PrivateChildMemberMetadataIssuer, PrivateChildMemberMetadataParts, PrivateChildMemberMetadataSource, PrivatePublicationMetadata, PrivatePublicationMetadataIssuer, PrivatePublicationMetadataParts, PrivatePublicationMetadataSource};
