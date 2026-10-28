//! 🧵️ Original-source retained preparation for existing IFC entity arguments and type commands.
use super::*;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_plugin::{EditorApp, Emit};
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::sync::Arc;

type Source = wire::SnapshotSource;
fn missing_root_argument(event:&editing::SnapshotEditEvent,snapshot:&IfcSnapshot,address:&pointer::PathAdmission)->Option<pointer::NativeAddress>{
    let editing::SnapshotEditEvent::InsertValue{path,..}=event else{return None;};
    if address.field_count()!=4||!address.field_is(path,2,"args"){return None;}
    let position=address.position(event)?;let args=&snapshot.entities.get(position)?.args;
    let argument=address.field_index(path,3).or_else(||address.field_is(path,3,"-").then_some(args.len()))?;
    if argument!=args.len(){return None;}
    Some(pointer::NativeAddress{position,part:None,argument,prefix:address.field_end(3)?,field:4})
}
fn insertion_metadata_bytes()->usize{4*std::mem::size_of::<Option<pointer::NativeAddress>>()}
fn constituent_argument_collection(event:&editing::SnapshotEditEvent,snapshot:&IfcSnapshot,address:&pointer::PathAdmission)->Option<pointer::NativeAddress>{
    let (path,insert)=match event{editing::SnapshotEditEvent::InsertValue{path,..}=>(path,true),editing::SnapshotEditEvent::RemoveValue{path}=>(path,false),_=>return None};
    if address.field_count()!=6||!address.field_is(path,2,"complex")||!address.field_is(path,4,"args"){return None;}
    let position=address.position(event)?;let part=address.field_index(path,3)?;let args=&snapshot.entities.get(position)?.complex.get(part)?.args;
    let argument=address.field_index(path,5).or_else(||(insert&&address.field_is(path,5,"-")).then_some(args.len()))?;
    if argument>args.len()||!insert&&argument==args.len(){return None;}
    Some(pointer::NativeAddress{position,part:Some(part),argument,prefix:address.field_end(4)?,field:6})
}
fn retained_constituent_argument(event:&editing::SnapshotEditEvent,address:&pointer::PathAdmission)->bool {
    let Some(native)=address.native_address(event)else{return false;};
    native.part.is_some()&&match event {
        editing::SnapshotEditEvent::SetValue{..}=>true,
        editing::SnapshotEditEvent::InsertValue{..}|editing::SnapshotEditEvent::RemoveValue{..}|editing::SnapshotEditEvent::MoveValue{..}=>address.field_count()>native.field,
        _=>false,
    }
}

fn replacement_seed(event: &editing::SnapshotEditEvent, snapshot: &IfcSnapshot, position: Option<usize>, argument: Option<usize>, header: Option<usize>) -> Option<IfcValue> {
    let (path, set) = match event {
        editing::SnapshotEditEvent::SetValue { path, .. } => (path, true),
        editing::SnapshotEditEvent::InsertValue { path, .. } | editing::SnapshotEditEvent::RemoveValue { path } => (path, false),
        _ => return None,
    };
    if path.len() > 8192 {
        return None;
    }
    if let Some(column) = header {
        return (set && path.strip_prefix("/header/") == Some(["fileDescription", "fileName", "fileSchema"][column])).then(|| IfcValue::Aggregate(Vec::new()));
    }
    let (position, argument) = (position?, argument?);
    let mut fields = path.split('/').skip(1);
    if fields.next() != Some("entities") || fields.next()?.parse::<usize>().ok() != Some(position) || fields.next() != Some("args") || fields.next()?.parse::<usize>().ok() != Some(argument) {
        return None;
    }
    let Some(payload) = fields.next() else {
        return Some(IfcValue::Unset);
    };
    if !set || payload != "value" || fields.next().is_some() {
        return None;
    }
    Some(match snapshot.entities.get(position)?.args.get(argument)? {
        IfcValue::Integer(_) => IfcValue::Integer(0),
        IfcValue::Real(_) => IfcValue::Real(0.0),
        IfcValue::Reference(_) => IfcValue::Reference(0),
        IfcValue::String(_) => IfcValue::String(String::new()),
        IfcValue::Enum(_) => IfcValue::Enum(String::new()),
        IfcValue::Binary(_) => IfcValue::Binary(String::new()),
        IfcValue::Aggregate(_) => IfcValue::Aggregate(Vec::new()),
        IfcValue::TypedValue(_) => IfcValue::TypedValue(IfcTypedValue { name: String::new(), items: Vec::new() }),
        _ => return None,
    })
}

#[derive(Default)]
struct ReductionControl {
    #[cfg(test)]
    cancel_at: Option<usize>,
    #[cfg(test)]
    ticks: usize,
}

impl ReductionControl {
    fn tick(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), String> {
        #[cfg(test)]
        {
            self.ticks += 1;
            if self.cancel_at == Some(self.ticks) {
                cx.original_cancel_token().cancel_now();
            }
        }
        work_tick(cx)
    }
}

pub struct IfcDetailsWork<E: editing::SnapshotEditingEditor, C: IfcDetailsProfile> {
    tool: &'static str,
    original_event: Option<editing::SnapshotEditEvent>,
    value_edit: value_edit::IfcValueEdit,
    value_edit_done: bool,
    borrowed_preimage: bool,
    cursor: <IfcEntity as RetainedClone>::Cursor,
    value_cursor: <IfcValue as RetainedClone>::Cursor,
    header_cursor: <Vec<IfcValue> as RetainedClone>::Cursor,
    parts_cursor:<Vec<IfcComplexType> as RetainedClone>::Cursor,
    source: Option<Source>,
    entity: Option<IfcEntity>,
    value: Option<IfcValue>,
    header_values: Option<Vec<IfcValue>>,
    parts:Option<Vec<IfcComplexType>>,
    header_column: Option<usize>,
    garbage: Option<Vec<IfcValue>>,
    pending: Option<Vec<IfcMutation>>,
    semantic_emit: Option<Emit<IfcMutation, E::ConfigMutation, E::DraftMutation>>,
    unborn: Option<Arc<IfcSnapshot>>,
    retirement: RetainedCloneClose,
    control: ReductionControl,
    position: Option<usize>,
    argument: Option<usize>,
    part:Option<usize>,
    prefix:usize,
    prepared: bool,
    admitted: Option<bool>,
    admission: admission::EventAdmission,
    address: pointer::PathAdmission,
    reference:reference::ReferenceReduction,
    enumeration:contextual_edit::EnumerationEdit,
    entity_removal:entity_remove::EntityRemoval,
    constituent_roster:constituent_roster::ConstituentRoster,
    command_wire:command_wire::CommandWire,
    publication_validated: bool,
    domain_validated: bool,
    domain: domain_work::DomainWork,
    rules: C::CandidateRules,
    rules_validated: bool,
    publication: wire::IfcPublication,
    complete: bool,
    closing: bool,
    marker: std::marker::PhantomData<fn() -> (E, C)>,
}

impl<E: editing::SnapshotEditingEditor, C: IfcDetailsProfile> IfcDetailsWork<E, C> {
    pub fn new(tool: &'static str) -> Self {
        Self {
            tool,
            original_event: None,
            value_edit: Default::default(),
            value_edit_done: false,
            borrowed_preimage: false,
            cursor: IfcEntity::retained_clone_cursor(),
            value_cursor: IfcValue::retained_clone_cursor(),
            header_cursor: Vec::<IfcValue>::retained_clone_cursor(),
            parts_cursor:Vec::<IfcComplexType>::retained_clone_cursor(),
            source: None,
            entity: None,
            value: None,
            header_values: None,
            parts:None,
            header_column: None,
            garbage: None,
            pending: None,
            semantic_emit: None,
            unborn: None,
            retirement: Default::default(),
            control: Default::default(),
            position: None,
            argument: None,
            part:None,
            prefix:0,
            prepared: false,
            admitted: None,
            admission: Default::default(),
            address: Default::default(),
            reference:Default::default(),
            enumeration:Default::default(),
            entity_removal:Default::default(),
            constituent_roster:Default::default(),
            command_wire:Default::default(),
            publication_validated: false,
            domain_validated: false,
            domain: Default::default(),
            rules: Default::default(),
            rules_validated: false,
            publication: Default::default(),
            complete: false,
            closing: false,
            marker: std::marker::PhantomData,
        }
    }

    #[cfg(test)]
    pub fn cancel_at_reduction_tick(&mut self, tick: usize) {
        self.control.cancel_at = Some(tick);
    }

    #[cfg(test)]
    pub fn cancel_at_domain_turn(&mut self, turn: usize) {
        self.domain.cancel_at_turn(turn);
    }
    #[cfg(test)]
    pub fn domain_turns(&self) -> usize {
        self.domain.turns()
    }
    #[cfg(test)]
    pub fn reduction_ticks(&self) -> usize {
        self.control.ticks
    }
    #[cfg(test)]
    pub fn staged_native_value(&self) -> bool {
        self.value_edit.started() || self.borrowed_preimage
    }
    #[cfg(test)]
    pub fn borrowed_inverse_wire<'a>(&'a self, mutations: &'a [IfcMutation]) -> impl semio_framework_plugin::plugin_app_close_prelude::store::ArtifactCanonicalJson + 'a {
        self.publication.inverse_wire(mutations)
    }
    #[cfg(test)]
    pub fn cancel_at_publication_turn(&mut self, turn: usize) {
        self.publication.cancel_at_turn(turn);
    }
    #[cfg(test)]
    pub fn publication_turns(&self) -> usize {
        self.publication.turns()
    }

    #[cfg(test)]
    pub fn retained_text_bytes(&self) -> usize {
        fn values(items: &[IfcValue]) -> usize {
            items
                .iter()
                .map(|value| match value {
                    IfcValue::String(text) | IfcValue::Enum(text) | IfcValue::Binary(text) => text.len(),
                    IfcValue::Aggregate(items) => values(items),
                    IfcValue::TypedValue(value) => value.name.len() + values(&value.items),
                    _ => 0,
                })
                .sum()
        }
        fn parts(items: &[IfcComplexType]) -> usize {
            items.iter().map(|part| part.name.len() + values(&part.args)).sum()
        }
        self.entity.as_ref().map_or(0, |entity| entity.name.len() + values(&entity.args) + parts(&entity.complex))
            + self.value.as_ref().map_or(0, |value| values(std::slice::from_ref(value)))
            + self.garbage.as_deref().map_or(0, values)
            + self.header_values.as_deref().map_or(0, values)
            + self.parts.as_deref().map_or(0,parts)
            + self.pending.as_deref().map_or(0, |mutations| {
                mutations
                    .iter()
                    .map(|mutation| match mutation {
                        IfcMutation::SetEntityArg(value) => values(std::slice::from_ref(&value.value)),
                        IfcMutation::InsertEntityArg(value) => values(std::slice::from_ref(&value.value)),
                        IfcMutation::SetEntityComplex(value) => parts(&value.complex),
                        IfcMutation::SetEntityName(value) => value.name.len(),
                        _ => 0,
                    })
                    .sum()
            })
    }

    fn leaf_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<IfcMutation>()+std::mem::size_of::<IfcValue>()+if self.part.is_some(){std::mem::size_of::<IfcValue>()+std::mem::size_of::<Vec<IfcComplexType>>()+2*std::mem::size_of::<Vec<IfcValue>>()}else{0},capacity_bytes:std::mem::size_of::<IfcMutation>(),depth:1,..Default::default()}}
    fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.rules.terminal_is_empty(){return self.rules.close_demands(body);}
        if !self.command_wire.terminal_is_empty(){return self.command_wire.close_demands();}
        if !self.admission.terminal_is_empty() {
            return self.admission.close_demands();
        }
        if !self.enumeration.terminal_is_empty(){return self.enumeration.close_demands(body);}
        if !self.domain.terminal_is_empty(){return Ok(self.domain.close_demands());}
        if !self.entity_removal.terminal_is_empty(){return Ok(self.entity_removal.close_demands());}
        if !self.constituent_roster.terminal_is_empty(){return self.constituent_roster.close_demands(body);}
        if !self.reference.terminal_is_empty(){return Ok(self.reference.close_demands());}
        if !self.address.terminal_is_empty(){return Ok(self.address.close_demands());}
        if self.publication.codec_live() {
            return self.publication.close_demands();
        }
        if !self.cursor.terminal_is_empty() {
            return self.cursor.close_demands(body);
        }
        if !self.value_cursor.terminal_is_empty() {
            return self.value_cursor.close_demands(body);
        }
        if !self.header_cursor.terminal_is_empty() {
            return self.header_cursor.close_demands(body);
        }
        if !self.parts_cursor.terminal_is_empty(){return self.parts_cursor.close_demands(body);}
        if !self.value_edit.terminal_is_empty() {
            return self.value_edit.close_demands();
        }
        if self.value.is_some() {
            return self.retirement.next_owner_demands::<IfcValue>(true, &None, body);
        }
        if self.original_event.is_some() {
            return self.retirement.next_owner_demands::<editing::SnapshotEditEvent>(true, &None, body);
        }
        if self.header_values.is_some() {
            return self.retirement.next_owner_demands::<Vec<IfcValue>>(true, &None, body);
        }
        if self.parts.is_some(){return self.retirement.next_owner_demands::<Vec<IfcComplexType>>(true,&None,body);}
        if self.entity.is_some() || !self.retirement.is_empty() {
            return self.retirement.next_owner_demands::<IfcEntity>(self.entity.is_some(), &None, body);
        }
        if self.garbage.is_some() {
            return self.retirement.next_owner_demands::<Vec<IfcValue>>(true, &None, body);
        }
        if self.pending.is_some() {
            return self.retirement.next_owner_demands::<Vec<IfcMutation>>(true, &None, body);
        }
        if self.semantic_emit.is_some() {
            return self.retirement.next_owner_demands::<Emit<IfcMutation, E::ConfigMutation, E::DraftMutation>>(true, &None, body);
        }
        if self.unborn.is_some() {
            return self.retirement.next_owner_demands::<Arc<IfcSnapshot>>(true, &None, body);
        }
        if let Some(source) = self.source.as_ref().filter(|source| !source.terminal_is_empty()) {
            return source.next_close_demand(body);
        }
        if let Some(source) = self.publication.view.source.as_ref().filter(|source| !source.terminal_is_empty()) {
            return source.next_close_demand(body);
        }
        Ok(RetirementDemand { depth: 1, ..Default::default() })
    }
}

fn work_tick(cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), String> {
    if cx.is_cancelled() {
        return Err("details-cancelled".into());
    }
    if cx.should_yield() {
        return Err("details-work-limit".into());
    }
    cx.consume_fuel(1);
    Ok(())
}

#[cfg(test)]
fn declared_publication_rows(mutations: &[IfcMutation]) -> Result<usize, Fault> {
    mutations
        .iter()
        .try_fold(0usize, |rows, mutation| rows.checked_add(1).and_then(|rows| rows.checked_add(<IfcMutation as protocol::Mutation<IfcSnapshot>>::inverse_rows(mutation))).ok_or_else(|| Fault::from("IFC edit publication row count overflow")))
}

fn event_stub(event: &editing::SnapshotEditEvent) -> editing::SnapshotEditEvent {
    use editing::SnapshotEditEvent;
    match event {
        SnapshotEditEvent::SetValue { .. } => SnapshotEditEvent::SetValue { path: String::new(), value: DslValue::Null },
        SnapshotEditEvent::InsertValue { .. } => SnapshotEditEvent::InsertValue { path: String::new(), value: DslValue::Null },
        SnapshotEditEvent::RemoveValue { .. } => SnapshotEditEvent::RemoveValue { path: String::new() },
        SnapshotEditEvent::MoveValue { .. } => SnapshotEditEvent::MoveValue { from: String::new(), path: String::new() },
        SnapshotEditEvent::RenameKey { .. } => SnapshotEditEvent::RenameKey { path: String::new(), key: String::new() },
        SnapshotEditEvent::ReplaceSource { .. } => SnapshotEditEvent::ReplaceSource { source: String::new() },
    }
}

fn header_values(snapshot: &IfcSnapshot, column: usize) -> &Vec<IfcValue> {
    match column {
        0 => &snapshot.header.file_description,
        1 => &snapshot.header.file_name,
        _ => &snapshot.header.file_schema,
    }
}

impl<E, C> ArtifactCommandWork<EditorApp<E>> for IfcDetailsWork<E, C>
where
    E: editing::SnapshotEditingEditor<Snapshot = IfcSnapshot, Mutation = IfcMutation>,
    C: IfcDetailsProfile + 'static,
{
    fn tool_id(&self) -> &'static str {
        self.tool
    }
    fn original_wire_demands(&self,wire:&[u8],_:usize)->Result<Option<RetirementDemand>,ValueError>{
        Ok(Some(if self.command_wire.ready(){RetirementDemand{copy_bytes:2*std::mem::size_of::<editing::SnapshotEditEvent>()+std::mem::size_of::<E::Command>(),depth:1,..Default::default()}}else{self.command_wire.demands(wire)?}))
    }
    fn retains_original_wire(&self)->bool{self.command_wire.retains_source()}
    fn receive_original_wire(&mut self,wire:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<E::Command>,semio_framework_job::OriginalTurnFailure>{
        if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"IFC original command decoding cancelled").into());}
        let demand=self.original_wire_demands(wire,cx.retained_grant().maximum_copy_bytes)?.unwrap();if cx.should_yield()||!semio_framework_value::retained_clone::retained_clone_grant_admits(cx.retained_grant(),demand){return Ok(None);}
        if !self.command_wire.ready(){self.command_wire.advance(wire,cx)?;return Ok(None);}
        if self.original_event.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IFC wire receiving cannot displace an existing original event").into());}
        self.original_event=self.command_wire.output.take();let command=E::snapshot_edit_command_from_original(&mut self.original_event).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"IFC editor cannot receive its original event command"))?;
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(Some(command))
    }
    fn receive_original_command(&mut self, command: &mut E::Command, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if self.original_event.is_some() {
            return Ok(true);
        }
        let Some(event) = E::snapshot_edit_event_mut(command) else { return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "IFC original event owner is unavailable").into()) };
        let bytes = 2 * std::mem::size_of::<editing::SnapshotEditEvent>();
        let grant = cx.retained_grant();
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC original event receiving cancelled").into());
        }
        if cx.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < bytes {
            return Ok(false);
        }
        let stub = event_stub(event);
        self.original_event = Some(std::mem::replace(event, stub));
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        Ok(true)
    }
    fn extent(&self, command: &E::Command, snapshot: &IfcSnapshot, _: &protocol::InteractionState, _: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<E>>>) -> Option<usize> {
        let event = self.original_event.as_ref().or_else(|| E::snapshot_edit_event(command))?;
        match self.admitted { Some(false) => None, Some(true) => Some(E::snapshot_edit_work_capacity().work_items()), None => E::snapshot_edit_retained_extent(event, snapshot) }
    }
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
    fn work_demands(&self, input: &ArtifactCommandInputs<'_, EditorApp<E>>, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.original_event.is_none() {
            return Ok(RetirementDemand { copy_bytes: 2 * std::mem::size_of::<editing::SnapshotEditEvent>(), depth: 1, ..Default::default() });
        }
        if self.admitted.is_none() {
            return if self.admission.complete(){Ok(self.address.demands())}else{self.admission.demands()};
        }
        if !self.prepared && self.original_event.as_ref().or_else(|| E::snapshot_edit_event(input.command)).is_some_and(|event| !matches!(event, editing::SnapshotEditEvent::ReplaceSource { .. })) {
            let demand = Source::borrowed_constructor_demand::<Arc<IfcSnapshot>>();
            return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Arc<IfcSnapshot>>()+4*std::mem::size_of::<Option<usize>>()+std::mem::size_of::<usize>()+insertion_metadata_bytes(), capacity_bytes: demand.capacity_bytes, depth: demand.depth, ..Default::default() });
        }
        if !self.enumeration.complete(){return self.enumeration.demands(self.source.as_ref(),input.snapshot,body);}
        if !self.reference.complete(){return Ok(self.reference.demands());}
        if (self.pending.is_none()||self.constituent_roster.receiving_primitives())&&!self.constituent_roster.fallback()&&self.original_event.as_ref().is_some_and(|event|constituent_roster::ConstituentRoster::applies(event,&self.address)){return self.constituent_roster.demands(self.original_event.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"IFC original type event missing"))?,self.source.as_ref(),&self.parts_cursor,&self.value_cursor,input.snapshot,body);}
        if self.pending.is_none()&&self.original_event.as_ref().is_some_and(|event|entity_remove::EntityRemoval::address(event,&self.address).is_some()){return Ok(self.entity_removal.demands());}
        if let Some(source) = self.source.as_ref().filter(|_| self.entity.is_none() && self.value.is_none() && self.header_values.is_none() && !self.value_edit.started() && self.pending.is_none() && !self.complete) {
            if self.position.is_none() && self.header_column.is_none() {
                return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Vec<IfcMutation>>(), depth: 1, ..Default::default() });
            }
            if self.original_event.as_ref().is_some_and(|event|missing_root_argument(event,input.snapshot,&self.address).is_some())||self.reference.request.is_some_and(|request|self.part.is_none()&&request.kind!=reference::ReferenceKind::Append&&request.owner_end==self.prefix)||self.original_event.as_ref().or_else(|| E::snapshot_edit_event(input.command)).and_then(|event| replacement_seed(event, input.snapshot, self.position, self.argument, self.header_column)).is_some() {
                return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<IfcValue>()+if self.original_event.as_ref().is_some_and(|event|missing_root_argument(event,input.snapshot,&self.address).is_some()){insertion_metadata_bytes()}else{0}, depth: 1, ..Default::default() });
            }
            if let Some(column) = self.header_column {
                return self.header_cursor.advance_demands(source.borrow().project(column, |snapshot| header_values(snapshot, column)), body.min(4096));
            }
            let position = self.position.expect("retained entity position");
            let entity = source.borrow().project(position, |snapshot| &snapshot.entities[position]);
            if self.part.is_some(){return if self.parts.is_some(){Ok(RetirementDemand{copy_bytes:2*std::mem::size_of::<IfcValue>()+2*std::mem::size_of::<Vec<IfcValue>>()+insertion_metadata_bytes(),depth:1,..Default::default()})}else{self.parts_cursor.advance_demands(entity.project(0,|entity|&entity.complex),body.min(4096))};}
            if let Some(index) = self.argument {
                return self.value_cursor.advance_demands(entity.project(index, |entity| &entity.args[index]), body.min(4096));
            }
            return self.cursor.advance_demands(entity, body.min(4096));
        }
        if self.pending.is_none() && (self.argument.is_some() || self.header_column.is_some()) {
            if self.value_edit_done {
                return Ok(self.leaf_demand());
            }
            return self.value_edit.demands();
        }
        if self.pending.is_some() && !self.domain_validated && self.reference.request.is_none() { let mut demand=self.domain.demands();demand.copy_bytes+=std::mem::size_of::<bool>();return Ok(demand); }
        if self.pending.is_some() && self.domain_validated && !self.rules_validated {let mut demand=self.rules.demands(input.snapshot,self.pending.as_deref().unwrap_or(&[]))?;demand.copy_bytes+=std::mem::size_of::<bool>();return Ok(demand);}
        if self.pending.is_some() && !self.publication_validated {
            return self.publication.demands();
        }
        Ok(RetirementDemand { depth: 1, ..Default::default() })
    }
    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<E>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<E>>, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC edit preparation cancelled before publication").into());
        }
        if cx.should_yield() {
            return Ok(ArtifactCommandWorkStep::Pending { stage: "ifc-edit-prepare", preview: b"" });
        }
        let event = self.original_event.as_ref().or_else(|| E::snapshot_edit_event(input.command)).ok_or_else(|| Fault::from("IFC snapshot command missing"))?;
        if self.admitted.is_none() {
            if !self.admission.complete() {
                self.admission.advance(event,cx)?;
                return Ok(ArtifactCommandWorkStep::Progress { stage: "ifc-edit-original-admission", preview: b"" });
            }
            self.admitted=self.address.advance(event,input.snapshot,E::snapshot_edit_semantic_path_is_admitted,cx)?;
            if self.admitted == Some(false) {
                return Err(Fault::from("IFC original event path is not admitted").into());
            }
            return Ok(ArtifactCommandWorkStep::Progress { stage: "ifc-edit-original-address", preview: b"" });
        }
        if !self.prepared {
            let position=self.address.position(event);let header=self.address.header(event);let insertion=missing_root_argument(event,input.snapshot,&self.address).or_else(||constituent_argument_collection(event,input.snapshot,&self.address));let address=insertion.or_else(||self.address.native_address(event));
            if !matches!(event, editing::SnapshotEditEvent::ReplaceSource { .. }) {
                if position.is_some_and(|position| input.snapshot.entities.get(position).is_none()) {
                    return Err(Fault::from("IFC entity does not exist").into());
                }
                let owner = input.snapshot_owner.ok_or_else(|| Fault::from("IFC retained preparation requires its original captured snapshot"))?;
                let grant = cx.retained_grant();
                let alias_bytes = std::mem::size_of::<Arc<IfcSnapshot>>();
                let metadata=4*std::mem::size_of::<Option<usize>>()+std::mem::size_of::<usize>()+insertion_metadata_bytes();
                if grant.maximum_copy_bytes < alias_bytes+metadata {
                    return Ok(ArtifactCommandWorkStep::Pending { stage: "ifc-edit-source", preview: b"" });
                }
                let source_grant = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - alias_bytes-metadata, ..grant };
                let (source, mut progress) = match Source::admit_borrowed(Arc::clone(owner), |snapshot| snapshot.as_ref(), source_grant) {
                    Ok(source) => source,
                    Err((error, original)) => {
                        self.unborn = Some(original);
                        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: alias_bytes, ..Default::default() })?;
                        return Err(error.into());
                    }
                };
                progress.copied_bytes += alias_bytes+metadata;
                self.source = Some(source);
                self.position=position;self.header_column=header.map(|(column,_)|column);self.prefix=header.map_or(0,|(_,prefix)|prefix);
                if let Some(address)=address{let entity=&input.snapshot.entities[address.position];let args=address.part.and_then(|part|entity.complex.get(part)).map_or(entity.args.as_slice(),|part|part.args.as_slice());self.argument=(args.get(address.argument).is_some()||insertion.is_some()).then_some(address.argument);self.part=address.part;self.prefix=address.prefix;}
                cx.consume_retained(progress)?;
                cx.consume_fuel(1);
                self.prepared = true;
                return Ok(ArtifactCommandWorkStep::Progress { stage: "ifc-edit-source", preview: b"" });
            }
            self.prepared = true;
            self.semantic_emit = Some(E::snapshot_edit_emit_in_context(event, input.snapshot, cx)?);
            if self.semantic_emit.as_ref().is_some_and(|emit| !emit.artifact_mutations.is_empty()) {
                return Err(Fault::from("IFC source replacement cannot emit artifact mutations").into());
            }
            self.complete = true;
            return Ok(ArtifactCommandWorkStep::Complete(self.semantic_emit.take().unwrap()));
        }
        if !self.enumeration.complete(){self.enumeration.advance::<C>(event,input.snapshot,&self.address,self.source.as_ref(),&mut self.value_edit,cx)?;return Ok(ArtifactCommandWorkStep::Progress{stage:"ifc-edit-original-enumeration",preview:b""});}
        if !self.reference.complete(){
            if self.reference.advance::<C>(event,input.snapshot,&self.address,cx)?&&self.reference.request.is_none()&&self.part.is_some()&&!self.enumeration.receives_source_choice()&&constituent_argument_collection(event,input.snapshot,&self.address).is_none()&&!(self.argument.is_some()&&retained_constituent_argument(event,&self.address)){self.part=None;self.argument=None;}
            return Ok(ArtifactCommandWorkStep::Progress{stage:"ifc-edit-original-reference",preview:b""});
        }
        if self.pending.is_none()&&entity_remove::EntityRemoval::address(event,&self.address).is_some(){self.entity_removal.advance(event,input.snapshot,&self.address,&mut self.pending,cx)?;return Ok(ArtifactCommandWorkStep::Progress{stage:"ifc-edit-original-entity-removal",preview:b""});}
        if (self.pending.is_none()||self.constituent_roster.receiving_primitives())&&!self.constituent_roster.fallback()&&constituent_roster::ConstituentRoster::applies(event,&self.address){self.constituent_roster.advance::<C>(self.original_event.as_mut().ok_or_else(||Fault::from("IFC original constituent event owner missing"))?,input.snapshot,&self.address,self.source.as_ref(),&mut self.parts_cursor,&mut self.parts,&mut self.pending,&mut self.value_cursor,&mut self.value,cx)?;return Ok(ArtifactCommandWorkStep::Progress{stage:"ifc-edit-original-constituent-roster",preview:b""});}
        if self.position.is_none() && self.header_column.is_none() && self.pending.is_none() && !self.complete {
            if self.semantic_emit.is_none() {
                self.semantic_emit = Some(E::snapshot_edit_emit_in_context(event, input.snapshot, cx)?);
                return Ok(ArtifactCommandWorkStep::Progress { stage: "ifc-edit-domain", preview: b"" });
            }
            let bytes = std::mem::size_of::<Vec<IfcMutation>>();
            if cx.retained_grant().maximum_items == 0 || cx.retained_grant().maximum_copy_bytes < bytes {
                return Ok(ArtifactCommandWorkStep::Pending { stage: "ifc-edit-domain-owner", preview: b"" });
            }
            self.pending = Some(std::mem::take(&mut self.semantic_emit.as_mut().unwrap().artifact_mutations));
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(ArtifactCommandWorkStep::Progress { stage: "ifc-edit-domain-owner", preview: b"" });
        }
        if self.entity.is_none() && self.value.is_none() && self.header_values.is_none() && !self.value_edit.started() && self.pending.is_none() && !self.complete {
            if let Some(seed) = missing_root_argument(event,input.snapshot,&self.address).map(|_|IfcValue::Unset).or_else(||self.reference.request.filter(|request|self.part.is_none()&&request.kind!=reference::ReferenceKind::Append&&request.owner_end==self.prefix).map(|_|IfcValue::Unset)).or_else(||replacement_seed(event, input.snapshot, self.position, self.argument, self.header_column)) {
                let bytes = std::mem::size_of::<IfcValue>()+if missing_root_argument(event,input.snapshot,&self.address).is_some(){insertion_metadata_bytes()}else{0};
                let grant = cx.retained_grant();
                if grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth == 0 {
                    return Ok(ArtifactCommandWorkStep::Pending { stage: "ifc-edit-borrowed-preimage", preview: b"" });
                }
                self.value = Some(seed);
                self.borrowed_preimage = true;
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
                cx.consume_fuel(1);
                return Ok(ArtifactCommandWorkStep::Progress { stage: "ifc-edit-borrowed-preimage", preview: b"" });
            }
            let source = self.source.as_ref().ok_or_else(|| Fault::from("IFC retained source missing"))?;
            if self.part.is_some()&&self.parts.is_some(){
                let demand=RetirementDemand{copy_bytes:2*std::mem::size_of::<IfcValue>()+2*std::mem::size_of::<Vec<IfcValue>>()+insertion_metadata_bytes(),depth:1,..Default::default()};
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(cx.retained_grant(),demand){return Ok(ArtifactCommandWorkStep::Pending{stage:"ifc-edit-constituent-argument",preview:b""});}
                let tuple=constituent_argument_collection(event,input.snapshot,&self.address).is_some();let args=&mut self.parts.as_mut().unwrap()[self.part.unwrap()].args;
                self.value=Some(if tuple{IfcValue::Aggregate(std::mem::take(args))}else{std::mem::replace(&mut args[self.argument.unwrap()],IfcValue::Unset)});
                cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;cx.consume_fuel(1);
                return Ok(ArtifactCommandWorkStep::Progress{stage:"ifc-edit-constituent-argument",preview:b""});
            }
            let step = if let Some(column) = self.header_column {
                self.header_cursor.advance(source.borrow().project(column, |snapshot| header_values(snapshot, column)), cx.retained_grant().maximum_copy_bytes.min(4096), cx.retained_grant())?
            } else {
                let position = self.position.expect("retained entity position");
                let entity = source.borrow().project(position, |snapshot| &snapshot.entities[position]);
                if self.part.is_some(){self.parts_cursor.advance(entity.project(0,|entity|&entity.complex),cx.retained_grant().maximum_copy_bytes.min(4096),cx.retained_grant())?}
                else if let Some(index) = self.argument {
                    self.value_cursor.advance(entity.project(index, |entity| &entity.args[index]), cx.retained_grant().maximum_copy_bytes.min(4096), cx.retained_grant())?
                } else {
                    self.cursor.advance(entity, cx.retained_grant().maximum_copy_bytes.min(4096), cx.retained_grant())?
                }
            };
            cx.consume_retained(step.progress())?;
            cx.consume_fuel(step.progress().copied_items as u64);
            if matches!(step, RetainedCloneStep::Complete(_)) {
                if self.header_column.is_some() {
                    self.header_values = self.header_cursor.take();
                } else if self.part.is_some(){self.parts=self.parts_cursor.take();}
                else if self.argument.is_some() {
                    self.value = self.value_cursor.take();
                } else {
                    self.entity = self.cursor.take();
                }
            }
            return Ok(ArtifactCommandWorkStep::Progress { stage: "ifc-edit-copy", preview: b"" });
        }
        if self.complete {
            return Err(Fault::from("IFC retained edit already completed").into());
        }
        if self.pending.is_none() && (self.argument.is_some() || self.header_column.is_some()) {
            if self.header_values.is_some() {
                self.value = self.header_values.take().map(IfcValue::Aggregate);
                cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<IfcValue>(), ..Default::default() })?;
                cx.consume_fuel(1);
                return Ok(ArtifactCommandWorkStep::Progress { stage: "ifc-edit-header-owner", preview: b"" });
            }
            let demand=self.leaf_demand();
            let event = self.original_event.as_mut().ok_or_else(|| Fault::from("IFC retained edit requires its original owned command"))?;
            let prefix=self.prefix;
            let remove_argument = self.argument.is_some() && matches!(event, editing::SnapshotEditEvent::RemoveValue { path } if path.len() == prefix);
            if !self.value_edit_done {
                self.value_edit_done = remove_argument || self.value_edit.advance(&mut self.value, event, prefix, self.header_column.is_some()||constituent_argument_collection(event,input.snapshot,&self.address).is_some(),self.reference.request, cx)?;
                return Ok(ArtifactCommandWorkStep::Progress { stage: "ifc-edit-native-value", preview: b"" });
            }
            let grant = cx.retained_grant();
            if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_depth < demand.depth {
                return Ok(ArtifactCommandWorkStep::Pending { stage: "ifc-edit-leaf", preview: b"" });
            }
            let mutations = self.pending.get_or_insert_with(Vec::new);
            mutations.try_reserve_exact(1).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC primitive publication allocation failed"))?;
            use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{
                insert_entity_arg::InsertEntityArg, remove_entity_arg::RemoveEntityArg, set_entity_arg::SetEntityArg, set_entity_complex::SetEntityComplex, set_file_description::SetFileDescription, set_file_name::SetFileName, set_file_schema::SetFileSchema,
            };
            let mutation = if let Some(column) = self.header_column {
                if !matches!(self.value, Some(IfcValue::Aggregate(_))) {
                    return Err(Fault::from("IFC edited header tuple is missing").into());
                }
                let Some(IfcValue::Aggregate(values)) = self.value.take() else { unreachable!() };
                match column {
                    0 => IfcMutation::SetFileDescription(SetFileDescription { values }),
                    1 => IfcMutation::SetFileName(SetFileName { values }),
                    _ => IfcMutation::SetFileSchema(SetFileSchema { values }),
                }
            } else if let Some(part)=self.part{
                let tuple=constituent_argument_collection(event,input.snapshot,&self.address).is_some();if tuple&&!matches!(self.value,Some(IfcValue::Aggregate(_))){return Err(Fault::from("IFC constituent argument tuple is missing").into());}
                let value=self.value.take().ok_or_else(||Fault::from("IFC constituent argument is missing"))?;let parts=self.parts.as_mut().ok_or_else(||Fault::from("IFC constituent owner is missing"))?;
                if tuple{let IfcValue::Aggregate(args)=value else{unreachable!()};parts[part].args=args;}else{parts[part].args[self.argument.unwrap()]=value;}
                let complex=self.parts.take().unwrap();IfcMutation::SetEntityComplex(SetEntityComplex{id:input.snapshot.entities[self.position.unwrap()].id,complex})
            } else {
                let index = self.argument.unwrap();
                let id = input.snapshot.entities[self.position.unwrap()].id;
                if remove_argument {
                    IfcMutation::RemoveEntityArg(RemoveEntityArg { id, index })
                } else {
                    let value = self.value.take().ok_or_else(|| Fault::from("IFC edited argument is missing"))?;
                    if matches!(event, editing::SnapshotEditEvent::InsertValue { path, .. } if path.len() == prefix) {
                        IfcMutation::InsertEntityArg(InsertEntityArg { id, index, value })
                    } else {
                        IfcMutation::SetEntityArg(SetEntityArg { id, index, value })
                    }
                }
            };
            mutations.push(mutation);
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: mutations.capacity() * std::mem::size_of::<IfcMutation>(), ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(ArtifactCommandWorkStep::Progress { stage: "ifc-edit-leaf", preview: b"" });
        }
        if self.pending.is_none() {
            let position = self.position.expect("retained entity position");
            let control = &mut self.control;
            let mutations = self.pending.get_or_insert_with(Vec::new);
            owned_entity_edit::<C>(event, input.snapshot, position, self.entity.as_mut().expect("copied entity"), self.garbage.get_or_insert_with(Vec::new), mutations, &mut || control.tick(cx))?;
        }
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC edit preparation cancelled before publication").into());
        }
        if !self.domain_validated {
            if self.reference.request.is_none(){
                let mut demand=self.domain.demands();demand.copy_bytes+=std::mem::size_of::<bool>();
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(cx.retained_grant(),demand){return Ok(ArtifactCommandWorkStep::Pending{stage:"ifc-edit-domain",preview:b""});}
                if !self.domain.advance::<C>(event, input.snapshot, self.pending.as_deref().expect("prepared IFC mutations"), &self.address,self.constituent_roster.type_admission(), cx)? {return Ok(ArtifactCommandWorkStep::Progress { stage: "ifc-edit-domain", preview: b"" });}
                cx.consume_retained(RetainedCloneProgress{copied_bytes:std::mem::size_of::<bool>(),..Default::default()})?;
            }
            self.domain_validated = true;
        }
        if !self.rules_validated {
            let mut demand=self.rules.demands(input.snapshot,self.pending.as_deref().unwrap_or(&[]))?;demand.copy_bytes+=std::mem::size_of::<bool>();
            if !semio_framework_value::retained_clone::retained_clone_grant_admits(cx.retained_grant(),demand){return Ok(ArtifactCommandWorkStep::Pending{stage:"ifc-edit-rules",preview:b""});}
            if let Some(candidate)=self.constituent_roster.type_admission().and_then(|admission|candidate_rules::IfcTypeCandidate::new::<C>(admission,event,input.snapshot,&self.address,self.pending.as_deref().expect("prepared IFC mutations"))){
                match self.rules.advance(&candidate,cx)?{IfcCandidateRuleStep::Pending=>return Ok(ArtifactCommandWorkStep::Progress{stage:"ifc-edit-rules",preview:b""}),IfcCandidateRuleStep::Complete=>{},IfcCandidateRuleStep::Invalid=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"express.local-rule-false").into()),IfcCandidateRuleStep::Incomplete=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"express.local-rule-incomplete").into())}
            }
            self.rules_validated=true;
            cx.consume_retained(RetainedCloneProgress{copied_bytes:std::mem::size_of::<bool>(),..Default::default()})?;
        }
        if !self.publication_validated {
            if cx.should_yield() {
                return Ok(ArtifactCommandWorkStep::Pending { stage: "ifc-edit-publication", preview: b"" });
            }
            if self.publication.view.source.is_none() {
                self.publication.view.source = self.source.take();
            }
            if !self.publication.advance(self.pending.as_deref().expect("prepared IFC mutations"), E::snapshot_edit_work_capacity(), cx)? {
                return Ok(ArtifactCommandWorkStep::Progress { stage: "ifc-edit-publication", preview: b"" });
            }
            self.publication_validated = true;
        }
        if cx.is_cancelled() {
            return Err(ValueError::literal(ValueRefusalKind::Canceled, "IFC edit preparation cancelled before publication").into());
        }
        if cx.should_yield() {
            return Ok(ArtifactCommandWorkStep::Pending { stage: "ifc-edit-publication", preview: b"" });
        }
        self.complete = true;
        cx.consume_fuel(1);
        let mut emit = self.semantic_emit.take().unwrap_or_default();
        emit.artifact_mutations = self.pending.take().expect("prepared IFC mutation owner");
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }
    fn begin_close(&mut self) {
        self.closing = true;
        self.rules.begin_close();
        self.admission.begin_close();
        self.address.begin_close();
        self.reference.begin_close();
        self.enumeration.begin_close();
        self.command_wire.begin_close();
        self.domain.begin_close();
        self.entity_removal.begin_close();
        self.constituent_roster.begin_close();
        self.cursor.begin_close();
        self.value_cursor.begin_close();
        self.header_cursor.begin_close();
        self.parts_cursor.begin_close();
        self.value_edit.begin_close();
    }
    fn terminal_is_empty(&self) -> bool {
        self.rules.terminal_is_empty()
            && self.cursor.terminal_is_empty()
            && self.admission.terminal_is_empty()
            && self.command_wire.terminal_is_empty()
            && self.address.terminal_is_empty()
            && self.reference.terminal_is_empty()
            && self.enumeration.terminal_is_empty()
            && self.domain.terminal_is_empty()
            && self.entity_removal.terminal_is_empty()
            && self.constituent_roster.terminal_is_empty()
            && self.parts_cursor.terminal_is_empty()
            && self.parts.is_none()
            && self.value_cursor.terminal_is_empty()
            && self.header_cursor.terminal_is_empty()
            && self.value_edit.terminal_is_empty()
            && self.value.is_none()
            && self.original_event.is_none()
            && self.header_values.is_none()
            && self.entity.is_none()
            && self.garbage.is_none()
            && self.pending.is_none()
            && self.semantic_emit.is_none()
            && self.unborn.is_none()
            && self.retirement.is_empty()
            && !self.publication.codec_live()
            && self.source.as_ref().is_none_or(Source::terminal_is_empty)
            && self.publication.view.source.as_ref().is_none_or(Source::terminal_is_empty)
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(4096)?.copy_bytes)
    }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> {
        Ok(self.close_demand(body.min(4096))?.capacity_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(4096)?.release_bytes)
    }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demand(4096)?.depth)
    }
    fn close_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_job::InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        if !self.closing {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Blocked);
        }
        if self.terminal_is_empty() {
            return Ok(semio_framework_job::InteractiveJobCloseStep::Complete { progress: Default::default() });
        }
        let demand = self.close_demand(cx.retained_grant().maximum_copy_bytes.min(4096))?;
        semio_framework_job::close_original_retained_step(cx, demand, |grant| {
            let grant = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes.min(4096), ..grant };
            if !self.rules.terminal_is_empty(){return self.rules.close_step(grant);}
            if !self.command_wire.terminal_is_empty(){return self.command_wire.close_step(grant);}
            if !self.admission.terminal_is_empty() {
                return self.admission.close_step(grant);
            }
            if !self.enumeration.terminal_is_empty(){return self.enumeration.close_step(grant);}
            if !self.domain.terminal_is_empty(){return self.domain.close_step(grant);}
            if !self.entity_removal.terminal_is_empty(){return self.entity_removal.close_step(grant);}
            if !self.constituent_roster.terminal_is_empty(){return self.constituent_roster.close_step(grant);}
            if !self.reference.terminal_is_empty(){return self.reference.close_step(grant);}
            if !self.address.terminal_is_empty(){return self.address.close_step(grant);}
            if self.publication.codec_live() {
                return self.publication.close_codec(grant);
            }
            if !self.cursor.terminal_is_empty() {
                return self.cursor.close_step(grant);
            }
            if !self.value_cursor.terminal_is_empty() {
                return self.value_cursor.close_step(grant);
            }
            if !self.header_cursor.terminal_is_empty() {
                return self.header_cursor.close_step(grant);
            }
            if !self.parts_cursor.terminal_is_empty(){return self.parts_cursor.close_step(grant);}
            if !self.value_edit.terminal_is_empty() {
                return self.value_edit.close_step(grant);
            }
            if !self.retirement.is_empty() {
                return self.retirement.step_granted(grant);
            }
            if self.value.is_some() {
                return self.retirement.begin_granted(&mut self.value, grant)?.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "IFC argument retirement omitted its owner"));
            }
            if self.original_event.is_some() {
                return self.retirement.begin_granted(&mut self.original_event, grant)?.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "IFC original event retirement omitted its owner"));
            }
            if self.header_values.is_some() {
                return self.retirement.begin_granted(&mut self.header_values, grant)?.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "IFC header retirement omitted its owner"));
            }
            if self.parts.is_some(){return self.retirement.begin_granted(&mut self.parts,grant)?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"IFC constituent retirement omitted its owner"));}
            if self.entity.is_some() {
                return self.retirement.begin_granted(&mut self.entity, grant)?.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "IFC entity retirement omitted its owner"));
            }
            if self.garbage.is_some() {
                return self.retirement.begin_granted(&mut self.garbage, grant)?.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "IFC displaced values omitted their owner"));
            }
            if self.pending.is_some() {
                return self.retirement.begin_granted(&mut self.pending, grant)?.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "IFC pending mutations omitted their owner"));
            }
            if self.semantic_emit.is_some() {
                return self.retirement.begin_granted(&mut self.semantic_emit, grant)?.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "IFC semantic emission omitted its owner"));
            }
            if self.unborn.is_some() {
                return self.retirement.begin_granted(&mut self.unborn, grant)?.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "IFC refused source omitted its original owner"));
            }
            if let Some(source) = self.source.as_mut().filter(|source| !source.terminal_is_empty()) {
                return source.close_step(grant);
            }
            if let Some(source) = self.publication.view.source.as_mut().filter(|source| !source.terminal_is_empty()) {
                return source.close_step(grant);
            }
            Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn work_control_keeps_original_cancel_and_exact_final_fuel() {
        use semio_framework_job::{Generation, OperationId, StepBudget, StepContext, root_cancel_token};
        fn now() -> Option<u64> {
            Some(1)
        }
        for (fuel, cancelled, admitted) in [(0, false, false), (1, false, true), (8, true, false)] {
            let cancel = root_cancel_token();
            if cancelled {
                cancel.cancel_now();
            }
            let mut sequence = 0;
            let mut progress = Default::default();
            let mut cx = StepContext::new(OperationId(715), Generation(3), StepBudget::new(fuel, 1000, Default::default()), cancel.clone(), now, &mut sequence, &mut progress);
            assert!(cx.original_cancel_token().is_same_node(&cancel));
            assert_eq!(work_tick(&mut cx).is_ok(), admitted);
            if admitted {
                assert!(cx.should_yield());
                assert!(work_tick(&mut cx).is_err());
            }
        }
    }
    #[test]
    fn publication_rows_match_the_neutral_boundary() {
        let fixture = semio_framework_pack_json::parse(include_str!("🧪️tests/publication.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let fixture = semio_framework_pack_json::to_dsl_value(&fixture);
        let capacity = semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity::for_invertible_items(fixture.get("capacity").unwrap().get("invertibleItems").unwrap().as_u64().unwrap() as usize);
        for case in fixture.get("cases").unwrap().as_array().unwrap() {
            let count = case.get("mutations").unwrap().as_u64().unwrap() as usize;
            let mutation = IfcMutation::SetEntityName(semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::set_entity_name::SetEntityName { id: 1, name: "IFCWALL".into() });
            let mutations = vec![mutation; count];
            let rows = declared_publication_rows(&mutations).unwrap();
            assert_eq!(rows, case.get("rows").unwrap().as_u64().unwrap() as usize);
            assert_eq!(rows == 0 || capacity.admits(rows), case.get("admitted").unwrap().as_bool().unwrap());
        }
    }
}

#[cfg(test)]
#[path = "work_tests.rs"]
mod retained_work_tests;
