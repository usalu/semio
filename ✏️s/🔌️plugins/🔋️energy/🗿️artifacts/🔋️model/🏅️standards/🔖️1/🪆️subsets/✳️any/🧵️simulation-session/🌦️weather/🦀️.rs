//! 🌦️ Captured linked EPW bytes remain under original request, decoding and numerical source custody.
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_plugin_actor_host::requests::SharedRequestReceiver;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJobPort;
use semio_framework_os_kernel::os_store::codec_registry::native_snapshot::OriginalSnapshotRecordReceiving;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value_native_owned::native_decoding::{NativeConstructionStepFailure, NativeDecodeControl, NativeDecodeCloseControl};
use semio_s_artifact_stdio_epw::EpwSnapshot;
use semio_s_artifact_stdio_epw::standards::energyplus::subsets::any::schema::snapshot::EpwRecord;
use semio_framework_dsl_record::{BorrowedDslRecord,FieldValue,RecordValue,receiving::{DslOwnedReceivingField,OwnedFieldReceiving}};
type HeaderBinding=<EpwSnapshot as DslOwnedReceivingField>::Cursor;
type RowBinding=<EpwRecord as DslOwnedReceivingField>::Cursor;
type RowClose=ControlledRetirement<(Option<FieldValue>,EpwRecord)>;
use semio_s_plugin_energy_engine_site::{WeatherData, WeatherRecord};
use std::mem::{ManuallyDrop, size_of};

struct Owned {
    wire: Option<Vec<u8>>,
    fault: Option<semio_framework::Fault>,
    epw: Option<EpwSnapshot>,
    record:Option<RecordValue>,
    header:Option<FieldValue>,
    rows:Vec<FieldValue>,
    row_source:Option<FieldValue>,
    row:Option<EpwRecord>,
    weather: Option<WeatherData>,
    port: Option<ToolRunJobPort>,
}
semio_framework_value::artifact_retire_struct!(Owned { wire, fault, epw, record, header, rows, row_source, row, weather, port });

pub(super) struct LinkedWeather {
    receiver: Option<SharedRequestReceiver>,
    decoder: Option<OriginalSnapshotRecordReceiving<EpwSnapshot>>,
    header_binding:Option<HeaderBinding>,
    row_binding:Option<RowBinding>,
    row_close:Option<RowClose>,
    owned: ManuallyDrop<Owned>,
    closing: Option<ControlledRetirement<Owned>>,
    phase: u8,
    pub(super) units:u64,
    position: usize,
    hasher: semio_framework_hash::Sha256,
    pub(super) digest: Option<[u8; 32]>,
    closed: bool,
}

fn invalid(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, message) }
fn metadata<T>() -> RetirementDemand { RetirementDemand { copy_bytes: size_of::<T>(), depth: 1, ..Default::default() } }
fn funds(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool { semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) }
fn paid(cx: &mut StepContext<'_>, bytes: usize, born: usize) -> Result<(), OriginalTurnFailure> { cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: born, ..Default::default() })?; cx.consume_fuel(1); Ok(()) }

impl LinkedWeather {
    pub(super) fn new(receiver: SharedRequestReceiver, port: ToolRunJobPort) -> Self {
        Self { receiver: Some(receiver), decoder: None, header_binding:None,row_binding:None,row_close:None, owned: ManuallyDrop::new(Owned { wire: None, fault: None, epw: None, record:None,header:None,rows:Vec::new(),row_source:None,row:None, weather: None, port: Some(port) }), closing: None, phase: 0, units:0, position: 0, hasher: semio_framework_hash::Sha256::new(), digest: None, closed: false }
    }

    pub(super) fn progress(&self)->(u64,u64){(if self.phase>=2{self.owned.wire.as_ref().map_or(0,|bytes|bytes.len())as u64}else if self.phase==1{self.position as u64}else{0},if self.phase>=6{self.owned.weather.as_ref().map_or(self.position,|weather|weather.records.len())as u64}else{0})}

    pub(super) fn step(&mut self, cx: &mut StepContext<'_>, output: &mut Option<WeatherData>) -> Result<bool, OriginalTurnFailure> {
        let before=cx.retained_progress().copied_items;
        let result=self.step_one(cx,output);
        self.units=self.units.saturating_add((cx.retained_progress().copied_items-before)as u64);
        result
    }

    fn step_one(&mut self, cx: &mut StepContext<'_>, output: &mut Option<WeatherData>) -> Result<bool, OriginalTurnFailure> {
        if cx.should_yield() || cx.retained_grant().maximum_items == 0 || cx.retained_grant().maximum_depth == 0 { return Ok(false); }
        match self.phase {
            0 => {
                let port = self.owned.port.as_ref().unwrap();
                port.wait();
                if let Some(reply) = self.receiver.as_mut().unwrap().take_original(cx)? {
                    port.wake();
                    match reply { Ok(wire) => self.owned.wire = Some(wire), Err(fault) => { self.owned.fault = Some(fault); return Err(invalid("the bound weather artifact could not be resolved").into()); } }
                    self.phase = 1;
                } else if !self.receiver.as_ref().unwrap().is_pending() {
                    port.wake();
                    if self.receiver.as_ref().unwrap().is_disconnected() { return Err(invalid("the bound weather request ended without its original artifact").into()); }
                }
            }
            1 => {
                let wire = self.owned.wire.as_ref().unwrap();
                let count = (wire.len()-self.position).min(cx.retained_grant().maximum_copy_bytes).min(4096);
                if count == 0 && self.position < wire.len() { return Ok(false); }
                self.hasher.update(&wire[self.position..self.position+count]);
                self.position += count;
                paid(cx, count, 0)?;
                if self.position == wire.len() { self.digest = Some(std::mem::replace(&mut self.hasher, semio_framework_hash::Sha256::new()).finalize()); self.phase = 2; }
            }
            2 => {
                let wire=self.owned.wire.as_ref().unwrap();
                let decoder=&mut self.decoder;
                receive(cx,metadata::<OriginalSnapshotRecordReceiving<EpwSnapshot>>(),|native,grant| {
                    let Some((candidate,progress))=OriginalSnapshotRecordReceiving::admit(wire,"stdio.epw",Default::default(),native,grant)? else { return Ok(RetainedCloneStep::Progress(Default::default())); };
                    *decoder=Some(candidate);
                    Ok(RetainedCloneStep::Progress(progress))
                })?;
                if self.decoder.is_some() { self.phase=3; }
            }
            3 => {
                let wire=self.owned.wire.as_ref().unwrap();
                let decoder=self.decoder.as_mut().unwrap();
                let demand=decoder.demands(wire,4096)?;
                receive(cx,demand,|native,grant|decoder.step(wire,native,4096,grant).map(RetainedCloneStep::Progress))?;
                if decoder.record_is_ready() { self.phase=4; }
            }
            4 => {
                let decoder=self.decoder.as_mut().unwrap();
                let output=&mut self.owned.record;
                receive(cx,metadata::<RecordValue>(),|_,grant|decoder.take_record_into(output,grant).map(RetainedCloneStep::Progress))?;
                if self.owned.record.is_some() { self.phase=10; }
            }
            10 => {
                let demand=metadata::<(HeaderBinding,RecordValue,FieldValue,Vec<FieldValue>)>();
                if !funds(cx.retained_grant(),demand){return Ok(false);}
                let record=self.owned.record.as_mut().unwrap();
                let id=EpwSnapshot::RECORD.fields.iter().find(|field|field.key=="records").ok_or_else(||invalid("EPW record declaration has no weather rows"))?.id;
                let rows=record.fields.get_mut(&id).ok_or_else(||invalid("linked EPW has no declared weather rows"))?;
                if !matches!(rows,FieldValue::List(_)){return Err(invalid("linked EPW weather rows have the wrong shape").into());}
                let FieldValue::List(rows)=std::mem::replace(rows,FieldValue::List(Vec::new()))else{unreachable!()};
                if rows.is_empty(){self.owned.rows=rows;return Err(invalid("linked EPW has no weather records").into());}
                self.owned.rows=rows;
                self.owned.header=Some(FieldValue::Record(self.owned.record.take().unwrap()));
                self.header_binding=Some(Default::default());
                self.phase=11;
                paid(cx,demand.copy_bytes,0)?;
            }
            11 => {
                let binding=self.header_binding.as_mut().unwrap();
                if !binding.is_ready(){let demand=binding.demands(&self.owned.header,4096)?;let source=&mut self.owned.header;receive(cx,demand,|native,grant|binding.step(source,native,4096,grant).map(RetainedCloneStep::Progress))?;}
                else{let output=&mut self.owned.epw;receive(cx,metadata::<EpwSnapshot>(),|_,grant|binding.take_into(output,grant).map(RetainedCloneStep::Progress))?;if self.owned.epw.is_some(){self.phase=5;}}
            }
            5 => {
                let source=self.owned.epw.as_ref().unwrap();
                if source.data_periods.periods.len()>366 { return Err(invalid("linked EPW declares too many calendar periods").into()); }
                let metadata=semio_s_plugin_energy_epw::weather_metadata(source).map_err(|_|invalid("linked EPW location or calendar is invalid"))?;
                let bytes=metadata.location.len();
                let demand=RetirementDemand { copy_bytes:bytes+size_of::<WeatherData>(), capacity_bytes:bytes, depth:1, ..Default::default() };
                if !funds(cx.retained_grant(),demand) { return Ok(false); }
                let mut location=String::new();
                location.try_reserve_exact(bytes).map_err(|_|invalid("linked weather location storage is unavailable"))?;
                location.push_str(metadata.location);
                self.owned.weather=Some(WeatherData { calendar:metadata.calendar, location, latitude_deg:metadata.latitude_deg, longitude_deg:metadata.longitude_deg, elevation_m:metadata.elevation_m, time_zone_hours:metadata.time_zone_hours, records:Vec::new() });
                paid(cx,demand.copy_bytes,bytes)?;
                self.phase=6;
            }
            6 => {
                let records=self.owned.rows.len();
                let bytes=records.checked_mul(size_of::<WeatherRecord>()).ok_or_else(||invalid("linked weather records exceed their address space"))?;
                if !funds(cx.retained_grant(),RetirementDemand { capacity_bytes:bytes, depth:1, ..Default::default() }) { return Ok(false); }
                self.owned.weather.as_mut().unwrap().records.try_reserve_exact(records).map_err(|_|invalid("linked weather record storage is unavailable"))?;
                paid(cx,0,bytes)?;
                self.position=0;
                self.phase=7;
            }
            7 => {
                if self.position==self.owned.rows.len(){self.phase=8;return Ok(false);}
                let demand=metadata::<(RowBinding,FieldValue)>();
                if !funds(cx.retained_grant(),demand){return Ok(false);}
                self.owned.row_source=Some(std::mem::replace(&mut self.owned.rows[self.position],FieldValue::Absent));
                self.row_binding=Some(Default::default());
                self.phase=12;
                paid(cx,demand.copy_bytes,0)?;
            }
            12 => {
                let binding=self.row_binding.as_mut().unwrap();
                if !binding.is_ready(){let demand=binding.demands(&self.owned.row_source,4096)?;let source=&mut self.owned.row_source;receive(cx,demand,|native,grant|binding.step(source,native,4096,grant).map(RetainedCloneStep::Progress))?;}
                else{let output=&mut self.owned.row;receive(cx,metadata::<EpwRecord>(),|_,grant|binding.take_into(output,grant).map(RetainedCloneStep::Progress))?;if self.owned.row.is_some(){self.phase=13;}}
            }
            13 => {
                let demand=metadata::<(WeatherRecord,RowClose,Option<FieldValue>,EpwRecord)>();
                if !funds(cx.retained_grant(),demand){return Ok(false);}
                let record=semio_s_plugin_energy_epw::weather_record(self.owned.row.as_ref().unwrap()).map_err(|_|invalid("linked EPW contains an invalid hourly weather record"))?;
                self.owned.weather.as_mut().unwrap().records.push(record);
                let original=self.owned.row.take().unwrap();
                match ControlledRetirement::new((self.owned.row_source.take(),original)){Ok(close)=>self.row_close=Some(close),Err((error,(source,row)))=>{self.owned.row_source=source;self.owned.row=Some(row);return Err(error.into());}}
                self.phase=14;
                paid(cx,demand.copy_bytes,0)?;
            }
            14 => {
                let binding=self.row_binding.as_mut().unwrap();
                if !binding.terminal_is_empty(){let demand=binding.close_demands(4096)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|binding.close_step(grant).map(RetainedCloneStep::Progress))?;}
                else{let demand=metadata::<Option<RowBinding>>();if funds(cx.retained_grant(),demand){self.row_binding=None;self.phase=15;paid(cx,demand.copy_bytes,0)?;}}
            }
            15 => {
                let close=self.row_close.as_mut().unwrap();
                if !close.terminal_is_empty(){let demand=close.selected_demand(4096)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|close.step(grant))?;}
                else{let demand=metadata::<Option<RowClose>>();if funds(cx.retained_grant(),demand){self.row_close=None;self.position+=1;self.phase=7;paid(cx,demand.copy_bytes,0)?;}}
            }
            8 => {
                let demand=metadata::<Option<WeatherData>>();
                if output.is_some() { return Err(invalid("linked weather recipient is already occupied").into()); }
                if !funds(cx.retained_grant(),demand) { return Ok(false); }
                *output=self.owned.weather.take();
                self.phase=9;
                paid(cx,demand.copy_bytes,0)?;
                return Ok(true);
            }
            _ => return Ok(true),
        }
        Ok(false)
    }

    pub(super) fn close_demands(&self) -> Result<RetirementDemand,ValueError> {
        if let Some(receiver)=self.receiver.as_ref() { return if receiver.terminal_is_empty(){Ok(metadata::<Option<SharedRequestReceiver>>())}else{receiver.close_demands(4096)}; }
        if let Some(binding)=self.row_binding.as_ref(){return if binding.terminal_is_empty(){Ok(metadata::<Option<RowBinding>>())}else{binding.close_demands(4096)};}
        if let Some(binding)=self.header_binding.as_ref(){return if binding.terminal_is_empty(){Ok(metadata::<Option<HeaderBinding>>())}else{binding.close_demands(4096)};}
        if let Some(close)=self.row_close.as_ref(){return if close.terminal_is_empty(){Ok(metadata::<Option<RowClose>>())}else{close.selected_demand(4096)};}
        if let Some(decoder)=self.decoder.as_ref() { return if decoder.terminal_is_empty(){Ok(metadata::<Option<OriginalSnapshotRecordReceiving<EpwSnapshot>>>())}else{decoder.close_demands(4096)}; }
        if let Some(close)=self.closing.as_ref() { return if close.terminal_is_empty(){Ok(metadata::<Option<ControlledRetirement<Owned>>>())}else{close.selected_demand(4096)}; }
        if !self.closed { return Ok(RetirementDemand { copy_bytes:2*size_of::<ControlledRetirement<Owned>>()+size_of::<Owned>(),depth:1,..Default::default() }); }
        Ok(Default::default())
    }

    pub(super) fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<RetainedCloneProgress,OriginalTurnFailure> {
        let demand=self.close_demands()?;
        if let Some(decoder)=self.decoder.as_mut().filter(|_|self.receiver.is_none()&&self.header_binding.is_none()&&self.row_binding.is_none()&&self.row_close.is_none()) {
            if !decoder.terminal_is_empty() { return retire(cx,demand,|native,grant|decoder.close_step(native,grant)); }
        }
        let before=cx.retained_progress();
        semio_framework_job::close_original_retained_step(cx,demand,|grant|{
            if let Some(receiver)=self.receiver.as_mut() { if !receiver.terminal_is_empty(){return receiver.close_step(grant);}self.receiver=None; }
            else if let Some(binding)=self.row_binding.as_mut(){if !binding.terminal_is_empty(){return binding.close_step(grant).map(RetainedCloneStep::Progress);}self.row_binding=None;}
            else if let Some(binding)=self.header_binding.as_mut(){if !binding.terminal_is_empty(){return binding.close_step(grant).map(RetainedCloneStep::Progress);}self.header_binding=None;}
            else if let Some(close)=self.row_close.as_mut(){if !close.terminal_is_empty(){return close.step(grant);}self.row_close=None;}
            else if self.decoder.is_some() { self.decoder=None; }
            else if let Some(close)=self.closing.as_mut() { if !close.terminal_is_empty(){return close.step(grant);}self.closing=None;self.closed=true; }
            else if !self.closed { let original=std::mem::replace(&mut *self.owned,Owned { wire:None,fault:None,epw:None,record:None,header:None,rows:Vec::new(),row_source:None,row:None,weather:None,port:None });match ControlledRetirement::new(original){Ok(close)=>self.closing=Some(close),Err((error,original))=>{*self.owned=original;return Err(error);}} }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1,copied_bytes:demand.copy_bytes,..Default::default() }));
        })?;
        let after=cx.retained_progress();
        Ok(RetainedCloneProgress { copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes })
    }

    pub(super) fn register_close_wake(&self,wake:&std::task::Waker)->bool { self.receiver.as_ref().is_some_and(|receiver|receiver.register_wake(wake)) }
    pub(super) fn terminal_is_empty(&self)->bool { self.closed&&self.receiver.is_none()&&self.decoder.is_none()&&self.header_binding.is_none()&&self.row_binding.is_none()&&self.row_close.is_none()&&self.closing.is_none() }
}
impl Drop for LinkedWeather { fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"linked weather requires original source closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owned)}}} }

fn receive(cx:&mut StepContext<'_>,demand:RetirementDemand,run:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>)->Result<RetainedCloneProgress,OriginalTurnFailure>{
 let result=cx.with_native_receiving::<_,NativeConstructionStepFailure<ValueError>>(|native|native.receive_construction_step(demand,run));
 let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
 let reconciled=if let Some(grant)=cx.receiving_issued_grant(){cx.consume_receiving_retained(grant,progress)}else{Ok(())};
 cx.consume_fuel(progress.copied_items as u64);
 match(result,reconciled){(Ok(_),Ok(()))=>Ok(progress),(Err(error),Ok(()))=>Err(error.into()),(Ok(_),Err(error))=>Err(error.into()),(Err(error),Err(receipt))=>{let mut failure=OriginalTurnFailure::from(error);failure.context_refusal=Some(receipt);Err(failure)}}
}
fn retire(cx:&mut StepContext<'_>,demand:RetirementDemand,run:impl FnOnce(&mut dyn NativeDecodeCloseControl,RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>)->Result<RetainedCloneProgress,OriginalTurnFailure>{
 let grant=cx.retained_grant();if cx.retirement_should_yield()||!funds(grant,demand){return Ok(Default::default())}
 let result=cx.with_native_retirement(|native|run(native,grant));let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};let reconciled=cx.consume_retained(progress);cx.consume_fuel(progress.copied_items as u64);
 match(result,reconciled){(Ok(_),Ok(()))=>Ok(progress),(Err(error),Ok(()))=>Err(error.into()),(Ok(_),Err(error))=>Err(error.into()),(Err(error),Err(receipt))=>{let mut failure=OriginalTurnFailure::from(error);failure.receipt_refusal=Some(receipt);Err(failure)}}
}

#[cfg(test)]
#[path="🧪️.rs"]
mod tests;
