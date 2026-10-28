//! 📖️ Ref reads of the instance time-travel turns: the read-only base, query, retirement demands, snap, option and reference label resolution and the pending work, position and busy views. See [`super`].

use super::*;
use semio_framework_plugin_child::ChildContentRetirement;
use semio_framework::MutationId;

impl<'v, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TimeTravelRef<'v, A, M, X> {
    /// 🧭️ The store state a session observes — of the store it edits; `None` when that composed member is gone.
    pub(super) fn time_travel_base(&self) -> Option<TimeTravelBase> {
        match self.time_travel_query(TimeTravelStoreQuery::Base)? {
            TimeTravelQueryOutput::Base(base) => Some(base),
            TimeTravelQueryOutput::Positions(_) | TimeTravelQueryOutput::Value(_) => None,
        }
    }

    /// 🔎️ A store-only read of the store the session edits: the document's own, or the composed member's through its
    /// typed visitor (design §12); `None` when that member is gone.
    fn time_travel_query(&self, query: TimeTravelStoreQuery) -> Option<TimeTravelQueryOutput> {
        match self.time_travel.member.as_ref() {
            None => Some(TimeTravelStoreState::query(Some(&self.time_travel.document), &self.stores.store, query)),
            Some(member) => {
                let entry = self.child_composition.children.member(member.key.borrowed())?;
                Some(entry.member.visit_member(TimeTravelMemberQuery { owners: member.owners.as_deref(), query }))
            }
        }
    }

    /// 🧹️ One bounded close unit of the history-edit ledger: the session vanishes, every typed owner settles against its
    /// store, then one owner retires.
    pub(super) fn member_subject_retirement_demands(&self,body:usize)->Result<Option<RetirementDemand>,ValueError>{
        if !self.time_travel.closing&&self.time_travel.session.stage!=TimeTravelStage::Inactive{return Ok(None)}
        let Some(member)=self.time_travel.member.as_ref()else{return Ok(None)};
        if member.review_children.is_some()||member.children.is_some(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<TimeTravelReviewChildren>()+size_of::<ChildContentRetirement>()+size_of::<u64>(),depth:1,..Default::default()}))}
        if let Some(owners)=member.owners.as_ref(){if !owners.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"inactive original member retains live typed sources"))}return Ok(Some(RetirementDemand{copy_bytes:size_of_val(&member.owners),release_bytes:size_of_val(owners.as_ref()),depth:1,..Default::default()}))}
        if let Some(owner)=member.metadata_close.as_ref(){return if owner.terminal_is_empty(){Ok(Some(RetirementDemand{copy_bytes:size_of_val(&member.metadata_close),depth:1,..Default::default()}))}else{Ok(Some({let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original member metadata depth overflow"))?;demand}))}}
        Ok(Some(RetirementDemand{copy_bytes:if member.metadata_retired{size_of::<TimeTravelMemberSubject>()}else{size_of::<TimeTravelMemberMetadata>()},depth:1,..Default::default()}))
    }

    pub fn time_travel_retirement_demands(&self, body: usize) -> Result<RetirementDemand,ValueError> {
        if self.time_travel.owners().any(TimeTravelOwners::finalizing){if let Some(TimeTravelQueryOutput::CommitCloseDemand(demand))=self.time_travel_query(TimeTravelStoreQuery::CommitCloseDemand(body)){return demand}}
        if self.time_travel.document.discard_issuer_missing(){self.stores.store.owned_mutation_retirement_factory()?;return Ok(RetirementDemand{depth:1,..Default::default()});}
        let demand=self.time_travel.retirement_demands(body)?;if demand!=RetirementDemand::default(){return Ok(demand);}
        if let Some(demand)=self.member_subject_retirement_demands(body)?{return Ok(demand)}
        if self.time_travel.closing&&!self.time_travel.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"time travel close retains an undeclared original session editor preview or authoring frontier"));}
        Ok(Default::default())
    }

    /// 🧲️ Resolves every editor row's `snapSource` to concrete numbers before the panel is built
    /// ([`time_travel_resolve_snaps`]): a `Config` key reads the focused window's config, then the app config; a
    /// `Snapshot` pointer reads the document the session previews.
    pub fn resolve_time_travel_snaps(&self, panel: &mut TimeTravelPanel, view: &ViewModel) {
        let Some(editor) = panel.editor.as_mut() else { return };
        let config = std::cell::OnceCell::new();
        let preview = std::cell::OnceCell::new();
        let window = view.focused_window_id.as_deref().and_then(|id| view.window_instances.iter().find(|window| window.id == id));
        time_travel_resolve_snaps(&mut editor.rows, |source| match source {
            SnapSource::Step => None,
            SnapSource::Config { key } => {
                let pointer = if key.starts_with('/') { key.clone() } else { format!("/{key}") };
                let windowed = window
                    .and_then(|window| self.stores.window_config_store.pointer_values(&window.window_kind_id, std::slice::from_ref(&pointer)).into_iter().find(|(window_id, _)| *window_id == window.id))
                    .and_then(|(_, values)| values.into_iter().next().flatten())
                    .and_then(|value| value.as_f64());
                windowed.or_else(|| time_travel_pointer_get(config.get_or_init(|| semio_framework_value::ToValue::to_value(self.stores.config_store.snapshot_owner().as_ref())), &pointer).and_then(DslValue::as_f64))
            }
            SnapSource::Snapshot { pointer } => time_travel_pointer_get(preview.get_or_init(|| self.time_travel_previewed_value()), pointer).and_then(DslValue::as_f64),
        });
    }

    /// 🪟️ The document the session previews, as a value: the composed member's shown value, else the app's render snapshot
    /// (the state before the edited mutation with its draft while editing, the replayed head while reviewing).
    fn time_travel_previewed_value(&self) -> DslValue {
        match self.time_travel.member.is_some() {
            true => match self.time_travel_query(TimeTravelStoreQuery::ShownValue(self.time_travel.session.stage)) {
                Some(TimeTravelQueryOutput::Value(Some(value))) => value,
                _ => DslValue::Null,
            },
            false => {
                let committed = self.stores.store.snapshot_owner();
                semio_framework_value::ToValue::to_value(self.time_travel.render_snapshot_or(&*self.tool_runs, &committed).as_ref())
            }
        }
    }

    /// 🗝️ Resolves every editor row's `optionSource` to the keys the previewed document holds ([`time_travel_resolve_options`],
    /// design §20.11), its templates filled from the draft payload.
    pub fn resolve_time_travel_options(&self, panel: &mut TimeTravelPanel) {
        let Some(editor) = panel.editor.as_mut() else { return };
        if !editor.rows.iter().any(|row| matches!(row.input.schema, ArgSchema::String { option_source: Some(_), .. })) {
            return;
        }
        let payload = self.time_travel.editor.as_ref().map_or(DslValue::Null, |draft| draft.value.clone());
        time_travel_resolve_options(&mut editor.rows, &payload, &self.time_travel_previewed_value());
    }

    /// 🏷️ Labels every id the editor's reference rows name, in the document the session previews: the app's entity label
    /// (`ArtifactApp::entity_label` over the reference's kinds — the hook a plugin refines chips by), else the entity's own
    /// name in that document ([`time_travel_entity_names`]), else its kind word and a short id
    /// ([`time_travel_reference_fallback_label`], gap N3). A session on a composed member skips the app's hook, which reads
    /// only its own document, and names the member's entities from the member's preview.
    pub fn resolve_time_travel_reference_labels(&self, panel: &mut TimeTravelPanel) {
        let Some(editor) = panel.editor.as_mut() else { return };
        let member = self.time_travel.member.is_some();
        let committed = self.stores.store.snapshot_owner();
        let shown = self.time_travel.render_snapshot_or(&*self.tool_runs, &committed);
        let mut unnamed: Vec<(String, Vec<String>)> = Vec::new();
        for row in &editor.rows {
            let ArgSchema::Reference { kinds, .. } = &row.input.schema else { continue };
            let ids: Vec<String> = match &row.value {
                DslValue::Array(items) => items.iter().filter_map(reference_id_text).collect(),
                other => reference_id_text(other).into_iter().collect(),
            };
            for id in ids {
                match (!member).then(|| A::entity_label(shown.as_ref(), kinds, &id)).flatten() {
                    Some(label) => {
                        editor.reference_labels.insert(id, label);
                    }
                    None => unnamed.push((id, kinds.clone())),
                }
            }
        }
        if unnamed.is_empty() {
            return;
        }
        let document = match member {
            true => match self.time_travel_query(TimeTravelStoreQuery::ShownValue(self.time_travel.session.stage)) {
                Some(TimeTravelQueryOutput::Value(Some(value))) => value,
                _ => DslValue::Null,
            },
            false => semio_framework_value::ToValue::to_value(shown.as_ref()),
        };
        let names = time_travel_entity_names(&document, &unnamed.iter().map(|(id, _)| id.as_str()).collect());
        for (id, kinds) in unnamed {
            let label = names.get(&id).cloned().unwrap_or_else(|| time_travel_reference_fallback_label(&kinds, &id));
            editor.reference_labels.insert(id, label);
        }
    }

    /// 🏃️ Ledger work, a remote history change the document store still replays (unless paused), or a base change the
    /// watch has not delivered to an open session yet.
    pub fn time_travel_has_pending_work(&self) -> bool {
        self.time_travel.has_pending_work() || self.reprojection_drives() || (self.time_travel.is_active() && self.time_travel_base() != Some(self.time_travel.session.base))
    }

    /// 🚜️ Whether driver turns step the document store's waiting reprojection: a local history step always (it cannot be
    /// paused, only dropped), a remote change unless the user paused it.
    pub(super) fn reprojection_drives(&self) -> bool {
        self.stores.store.reprojection_progress().is_some() && (self.stores.store.local_step_pending() || !self.time_travel.reprojection_paused)
    }

    /// 🎯️ The applied position of every session target at the current base of the store it edits; `None` when one of
    /// them vanished or the edited member is gone.
    pub(super) fn time_travel_positions(&self) -> Option<Vec<TimeTravelTarget>> {
        let session = &self.time_travel.session;
        let targets: Vec<MutationId> = session.accepted.iter().map(|draft| draft.target.mutation.clone()).chain(session.pending.as_ref().map(|pending| pending.target.mutation.clone())).collect();
        if targets.is_empty() {
            return Some(Vec::new());
        }
        match self.time_travel_query(TimeTravelStoreQuery::Positions(targets))? {
            TimeTravelQueryOutput::Positions(positions) => positions,
            TimeTravelQueryOutput::Base(_) | TimeTravelQueryOutput::Value(_) => None,
        }
    }

    /// 🎞️ The identity of the document snapshot the session shows in its stage on the document's own store (`None`: the
    /// committed document) — what tells whether a session change swapped the document a window renders. A session on a
    /// composed member marks its children view itself ([`Self::refresh_time_travel_children`]).
    pub(super) fn time_travel_shown_identity(&self) -> Option<usize> {
        self.time_travel.document.shown(self.time_travel.session.stage).map(|shown| Arc::as_ptr(shown.snapshot_owner()) as usize)
    }

    /// ⏳️ Whether this instance refuses to open a history edit now (`timeTravel.busy`): a mutating tool run, an agent
    /// transaction or an open tool transaction holds it, the undo or redo of a finalize is being authored, or its own history
    /// step still replays — what disables every Edit row action with its reason (audit W2A-9).
    pub fn time_travel_busy(&self) -> bool {
        (!self.time_travel.is_active()&&self.time_travel.member.is_some()) || self.tool_runs.holds_mutating_run() || self.txn.pending_transaction.is_some() || self.stores.store.open_transaction().is_some() || self.time_travel.authoring.is_some() || self.stores.store.local_step_pending()
    }

    /// ⌛️ The wall deadline of a driver turn that begins now: [`TIME_TRAVEL_TURN_WALL_US`] on the ledger's turn clock (from
    /// zero when the clock cannot be read, so the turn ends after its first operation).
    pub(super) fn time_travel_turn_deadline(&self) -> u64 {
        (self.time_travel.turn_clock)().unwrap_or(0).saturating_add(TIME_TRAVEL_TURN_WALL_US)
    }
}
