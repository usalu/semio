#[derive(semio_framework_value::RetireOwned)]
struct MediaFaultReceiving {
    pub source: Option<MediaArtifactError>,
    pub fault: Option<Fault>,
    pub parameters: (String, String, String, String),
    pub entries: Vec<(String, String)>,
}
struct MediaFaultProjection {
    pub fault: MediaControlled<Fault>,
}
impl MediaFaultProjection {
    pub fn fault(&self) -> Option<&Fault> {
        self.fault.original()
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.fault.terminal_is_empty()
    }
    pub fn close_step(&mut self, effect: &mut MediaEffect<'_>) -> semio_framework_value::OriginalRetirementPoll {
        close_media_projection(&mut self.fault, effect)
    }
}
/// 🎟️ Retires an admitted media result one effect at a time; a pending poll keeps the owner for the next turn.
fn close_media_projection<T:MediaRetireOwned>(value:&mut MediaControlled<T>,effect:&mut MediaEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
    let body=effect.grant().maximum_copy_bytes;
    value.step(body,effect)
}
/// 📬️ Preserves the complete successful artifact or typed fault after original receiving.
#[derive(semio_framework_value::RetireOwned)]
enum MediaOutputValue{Artifact(MediaArtifact),Fault(Fault)}
/// 🧳️ Owns one admitted media output until its caller performs paid publication or retirement.
struct MediaOutputProjection{pub value:MediaControlled<MediaOutputValue>}
impl MediaOutputProjection{
    pub fn artifact(&self)->Option<&MediaArtifact>{match self.value.original(){Some(MediaOutputValue::Artifact(value))=>Some(value),_=>None}}
    pub fn fault(&self)->Option<&Fault>{match self.value.original(){Some(MediaOutputValue::Fault(value))=>Some(value),_=>None}}
    pub fn terminal_is_empty(&self)->bool{self.value.terminal_is_empty()}
    pub fn close_step(&mut self,effect:&mut MediaEffect<'_>)->semio_framework_value::OriginalRetirementPoll{close_media_projection(&mut self.value,effect)}
}
/// 🛬️ Admits original parent custody before invoking the exporter and projecting its full result.
#[doc(hidden)]
pub fn receive_media_output(owner:&mut MediaOwner<'_, '_>,exporter:impl FnOnce(&mut MediaOwner<'_, '_>)->Result<MediaArtifact,MediaArtifactError>)->Result<MediaOutputProjection,MediaValueError>{
    let grant=owner.remaining_grant();
    if grant.maximum_items==0||grant.maximum_copy_bytes==0||grant.maximum_capacity_bytes==0||grant.maximum_release_bytes==0||grant.maximum_depth<4{return Err(media_frontier_error())}
    let mut exporter=Some(exporter);
    let mut output=None;
    let turn=owner.receive_nested::<MediaArtifactError,MediaOutputValue,semio_os_kernel::io::control::NativeSnapshotFailure>(&mut output,&mut |slot,result,_marker,_unwind,original|semio_os_kernel::io::control::native_snapshot_turn(|work|{work.performed();(||->Result<(),MediaValueError>{
        let exporter=exporter.take().ok_or_else(media_projection_error)?;
        *result=Some(match exporter(original){
            Ok(value)=>MediaOutputValue::Artifact(value),
            Err(source)=>{
                *slot=Some(source);
                original.record_progress(MediaProgress{copied_items:1,..Default::default()})?;
                let mut projection=receive_media_artifact_fault(slot,original)?;
                projection.fault.take_original().map(MediaOutputValue::Fault).ok_or_else(media_projection_error)?
            }
        });
        Ok(())
    })().map_err(Into::into)}));
    turn.result.map_err(media_snapshot_error)?;
    let value=output.ok_or_else(media_projection_error)?;
    Ok(MediaOutputProjection{value:MediaControlled::new(value).ok().unwrap()})
}
struct MediaFaultTextExtent<'a, 'control> {
    pub bytes: usize,
    pub native: &'a mut MediaNative<'control>,
    pub wallet: &'a mut MediaWallet,
    pub error: Option<MediaValueError>,
}
impl std::fmt::Write for MediaFaultTextExtent<'_, '_> {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        let result = (|| {
            self.wallet.admit_frontier(MediaGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 })?;
            self.native.checkpoint()?;
            self.bytes = self.bytes.checked_add(text.len()).ok_or_else(media_frontier_error)?;
            self.wallet.record_progress(MediaProgress { copied_items: 1, ..Default::default() })?;
            let mut remaining = text.len();
            while remaining > 0 {
                let work = remaining.min(65536);
                self.native.advance(work)?;
                remaining -= work;
            }
            Ok::<_, MediaValueError>(())
        })();
        result.map_err(|error| {
            self.error = Some(error);
            std::fmt::Error
        })
    }
}
struct MediaFaultTextWriter<'a, 'control> {
    pub output: &'a mut String,
    pub native: &'a mut MediaNative<'control>,
    pub error: Option<MediaValueError>,
}
impl std::fmt::Write for MediaFaultTextWriter<'_, '_> {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        if self.error.is_some() {
            return Err(std::fmt::Error);
        }
        if self.output.len().checked_add(text.len()).is_none_or(|length| length > self.output.capacity()) {
            self.error = Some(media_frontier_error());
            return Err(std::fmt::Error);
        }
        let mut copied = 0;
        while copied < text.len() {
            let mut end = copied.saturating_add(65536).min(text.len());
            while !text.is_char_boundary(end) {
                end -= 1;
            }
            self.output.push_str(&text[copied..end]);
            if let Err(error) = self.native.advance(end - copied) {
                self.error = Some(error);
                return Err(std::fmt::Error);
            }
            copied = end;
        }
        Ok(())
    }
}
#[doc(hidden)]
pub fn receive_media_artifact_fault(source: &mut Option<MediaArtifactError>, owner: &mut MediaOwner<'_, '_>) -> Result<MediaFaultProjection, MediaValueError> {
    use semio_framework_diagnostic::{FaultParams, FaultScope, Severity};
    let grant = owner.remaining_grant();
    if grant.maximum_items == 0 || grant.maximum_copy_bytes == 0 || grant.maximum_capacity_bytes == 0 || grant.maximum_release_bytes == 0 || grant.maximum_depth < 4 {
        return Err(media_frontier_error());
    }
    let original = source.as_ref().ok_or_else(media_projection_error)?;
    let mismatch = matches!(original, MediaArtifactError::SchemaMismatch { .. });
    let code = if mismatch { MEDIA_SCHEMA_MISMATCH_CODE } else { "plugin.internal" };
    let mut counting = MediaWallet::new(grant);
    let extent = owner.native().scoped_stage(|native| {
        native.begin_stage(0)?;
        let mut extent = MediaFaultTextExtent { bytes: 0, native, wallet: &mut counting, error: None };
        let result = std::fmt::write(&mut extent, format_args!("{original}"));
        if let Some(error) = extent.error {
            return Err(error);
        }
        result.map_err(|_| media_projection_error())?;
        Ok(extent.bytes)
    });
    owner.record_progress(counting.progress())?;
    let extent = extent?;
    let mut output = None;
    let turn = owner.receive::<MediaFaultReceiving, Fault, semio_os_kernel::io::control::NativeSnapshotFailure>(&mut output, &mut |slot, result, _marker, _unwind, native, wallet| semio_os_kernel::io::control::native_snapshot_turn(|work| { work.performed(); (|| -> Result<(), MediaValueError> {
        let transfer = size_of::<Option<MediaArtifactError>>();
        wallet.admit_frontier(MediaGrant { maximum_items: 1, maximum_copy_bytes: transfer, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 2 })?;
        native.checkpoint()?;
        *slot = Some(MediaFaultReceiving { source: source.take(), fault: None, parameters: (String::new(), String::new(), String::new(), String::new()), entries: Vec::new() });
        wallet.record_progress(MediaProgress { copied_items: 1, copied_bytes: transfer, ..Default::default() })?;
        let state = slot.as_mut().unwrap();
        let scope_bytes = size_of::<FaultScope>();
        wallet.admit_frontier(MediaGrant { maximum_items: 1, maximum_copy_bytes: size_of::<Option<Fault>>(), maximum_capacity_bytes: scope_bytes, maximum_release_bytes: 0, maximum_depth: 2 })?;
        native.checkpoint()?;
        native.charge(scope_bytes)?;
        state.fault = Some(Fault {
            retained_progress: Default::default(),
            origin: if mismatch { FaultOrigin::Framework } else { FaultOrigin::Plugin },
            code: FaultCode(String::new()),
            severity: Severity::Error,
            message: String::new(),
            scope: Box::new(FaultScope::default()),
            span: None,
            source:None,
            domain:None,
            causes: Vec::new(),
            params: None,
            retryable: false,
        });
        wallet.record_progress(MediaProgress { copied_items: 1, copied_bytes: size_of::<Option<Fault>>(), retained_capacity_bytes: scope_bytes, released_bytes: 0 })?;
        let fault = state.fault.as_mut().unwrap();
        wallet.copy_encode_text_into(native, code, &mut fault.code.0, 2)?;
        wallet.admit_frontier(MediaGrant { maximum_items: 1, maximum_copy_bytes: extent, maximum_capacity_bytes: extent, maximum_release_bytes: 0, maximum_depth: 2 })?;
        native.begin_stage(extent)?;
        native.charge(extent)?;
        fault.message.try_reserve_exact(extent).map_err(|_| MediaValueError::literal(MediaRefusal::AllocationFailed, "media fault message receiving allocation failed"))?;
        wallet.record_progress(MediaProgress { copied_items: 1, retained_capacity_bytes: fault.message.capacity(), ..Default::default() })?;
        let mut writer = MediaFaultTextWriter { output: &mut fault.message, native, error: None };
        let formatted = std::fmt::write(&mut writer, format_args!("{}", state.source.as_ref().unwrap()));
        let error = writer.error.take();
        let copied = writer.output.len();
        drop(writer);
        wallet.record_progress(MediaProgress { copied_bytes: copied, ..Default::default() })?;
        if let Some(error) = error {
            return Err(error);
        }
        formatted.map_err(|_| media_projection_error())?;
        if let Some(MediaArtifactError::SchemaMismatch { expected, found }) = state.source.as_ref() {
            wallet.copy_encode_text_into(native, "expected", &mut state.parameters.0, 2)?;
            wallet.copy_encode_text_into(native, expected, &mut state.parameters.1, 2)?;
            wallet.copy_encode_text_into(native, "found", &mut state.parameters.2, 2)?;
            wallet.copy_encode_text_into(native, found, &mut state.parameters.3, 2)?;
            wallet.allocate_encode_vec_into(native, 2, &mut state.entries, 2)?;
            let transfer = size_of::<(String, String, String, String)>();
            let params_bytes = size_of::<FaultParams>();
            wallet.admit_frontier(MediaGrant { maximum_items: 1, maximum_copy_bytes: transfer + size_of::<Vec<(String, String)>>(), maximum_capacity_bytes: params_bytes, maximum_release_bytes: 0, maximum_depth: 2 })?;
            native.checkpoint()?;
            native.charge(params_bytes)?;
            let (expected_key, expected, found_key, found) = std::mem::take(&mut state.parameters);
            state.entries.push((expected_key, expected));
            state.entries.push((found_key, found));
            fault.params = Some(Box::new(FaultParams(std::mem::take(&mut state.entries))));
            wallet.record_progress(MediaProgress { copied_items: 1, copied_bytes: transfer + size_of::<Vec<(String, String)>>(), retained_capacity_bytes: params_bytes, released_bytes: 0 })?;
        }
        let transfer = size_of::<Option<Fault>>();
        wallet.admit_frontier(MediaGrant { maximum_items: 1, maximum_copy_bytes: transfer, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 2 })?;
        native.checkpoint()?;
        wallet.record_progress(MediaProgress { copied_items: 1, copied_bytes: transfer, ..Default::default() })?;
        *result = Some(state.fault.take().ok_or_else(media_projection_error)?);
        Ok(())
    })().map_err(Into::into) }));
    turn.result.map_err(media_snapshot_error)?;
    let fault = output.ok_or_else(media_projection_error)?;
    Ok(MediaFaultProjection { fault: MediaControlled::new(fault).ok().unwrap() })
}
