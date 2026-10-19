//! 🎚️ Tuned encoder of the zlib codec: the tuned frame, the tuned deflate job with its interactive steps and the deterministic and high-search raw deflate entries. See [`super`].

use super::*;

//#region 🦑️DissolvedEngineCodec
//#region DeflateCodec
//#region 🧵️TunedEncode
#[derive(Clone, Copy, value_derive::ToValue, value_derive::FromValue)]
enum TunedFrame {
    Raw,
    Zlib { header: u16, adler: u32 },
}

/// 🎛️ Persistent bounded encoder for the selected Office, Illustrator, and level-nine policies.
#[derive(value_derive::ToValue, value_derive::FromValue)]
pub struct TunedDeflateEncodeJob {
    engine: dynamic::Job,
    pub(super) frame: TunedFrame,
    /// 📄️ The commit being published page by page — transient, never part of a checkpoint.
    #[value(skip)]
    pub(super) publication: Option<Box<Publication>>,
}

impl TunedDeflateEncodeJob {
    pub(super) fn classic(input: Vec<u8>, policy: dynamic::Policy, frame: TunedFrame) -> Self {
        Self { engine: dynamic::Job::classic(input, policy), frame, publication: None }
    }

    /// 🗜️ Creates the deterministic Office-compatible raw encoder.
    pub fn office(input: Vec<u8>) -> Self {
        Self::classic(input, dynamic::Policy { window_bits: 15, memory_level: 8, good: 1, lazy: 4, nice: 258, chain: 1024, finish: dynamic::Finish::Sync }, TunedFrame::Raw)
    }

    /// 🔎 Creates the deterministic high-search raw encoder.
    pub fn office_high_search(input: Vec<u8>) -> Self {
        Self::classic(input, dynamic::Policy { window_bits: 15, memory_level: 8, good: 4, lazy: 4, nice: 258, chain: 4096, finish: dynamic::Finish::Sync }, TunedFrame::Raw)
    }

    /// 🧳 Creates the deterministic compact high-search raw encoder.
    pub fn office_compact_high_search(input: Vec<u8>) -> Self {
        Self::classic(input, dynamic::Policy { window_bits: 15, memory_level: 7, good: 4, lazy: 4, nice: 258, chain: 4096, finish: dynamic::Finish::Sync }, TunedFrame::Raw)
    }

    /// 🎨 Creates the deterministic Illustrator partial-flush RFC 1950 encoder.
    pub fn illustrator(input: Vec<u8>) -> Self {
        let frame = TunedFrame::Zlib { header: 0x4889, adler: adler32(&input) };
        Self::classic(input, dynamic::Policy { window_bits: 12, memory_level: 5, good: 8, lazy: 16, nice: 128, chain: 128, finish: dynamic::Finish::Partial }, frame)
    }

    /// 🏁 Creates the deterministic miniz-compatible level-nine RFC 1950 encoder.
    pub fn level_nine(input: Vec<u8>) -> Self {
        let frame = TunedFrame::Zlib { header: 0x78da, adler: adler32(&input) };
        Self { engine: dynamic::Job::miniz(input), frame, publication: None }
    }

    /// 📈 Returns applied transition progress and its deterministic upper bound.
    pub fn progress(&self) -> (usize, usize) {
        self.engine.progress()
    }

    /// 💾 Captures all encoder cursors, indices, tokens, and partial output without replay.
    pub fn checkpoint_bytes(&self) -> Vec<u8> {
        semio_framework_pack_json::to_json_string(self).into_bytes()
    }

    /// ♻️ Restores a tuned encoder from [`Self::checkpoint_bytes`].
    pub fn from_checkpoint(bytes: &[u8]) -> Result<Self, String> {
        std::str::from_utf8(bytes).map_err(|error| format!("invalid tuned DEFLATE checkpoint: {error}")).and_then(|text| semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("invalid tuned DEFLATE checkpoint: {error}")))
    }

    pub(super) fn output(&self) -> Vec<u8> {
        match self.frame {
            TunedFrame::Raw => self.engine.output().to_vec(),
            TunedFrame::Zlib { header, adler } => {
                let mut output = Vec::with_capacity(self.engine.output().len() + 6);
                output.extend_from_slice(&header.to_be_bytes());
                output.extend_from_slice(self.engine.output());
                output.extend_from_slice(&adler.to_be_bytes());
                output
            }
        }
    }

    pub(super) fn finish(mut self) -> Vec<u8> {
        while !self.engine.step() {}
        self.output()
    }

    /// 📏️ Quotes the next close turn: the staged publication, then the engine's first backing.
    pub(super) fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let mut demand = publication_demands(&self.publication, self.engine.next_close_release_bytes())?;
        if self.publication.is_none() && demand.release_bytes == 0 && !semio_framework_job::InteractiveJob::terminal_is_empty(self) {
            demand.depth = 1;
        }
        Ok(demand)
    }

    /// 🏁️ Freezes the committed output and stages its publication.
    pub(super) fn stage_commit(&mut self) {
        self.publication = Some(Publication::new(PublicationKind::Commit, Vec::new(), self.output()));
    }
}

impl TunedDeflateEncodeJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        if self.publication.as_ref().is_some_and(|publication| publication.is_delivered()) {
            return retire_delivered_publication(&mut self.publication, context);
        }
        if context.is_cancelled() {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(context);
        }
        if self.publication.is_some() {
            return drive_publication(&mut self.publication, context);
        }
        context.set_stage("deflate:tuned-encode");
        loop {
            if self.engine.step() {
                self.stage_commit();
                return drive_publication(&mut self.publication, context);
            }
            context.consume_fuel(1);
            if context.is_cancelled() {
                return semio_framework_job::JobOutcomeBorrow::admit_cancelled(context);
            }
            if context.should_yield() {
                return Ok(None);
            }
        }
    }
}

impl semio_framework_job::InteractiveJob for TunedDeflateEncodeJob {
    fn step<'a>(&'a mut self,context:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(context).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        borrow_publication_outcome(&self.publication, descriptor)
    }

    fn begin_close(&mut self) {}

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            use semio_framework_job::InteractiveJobCloseStep;
            let demand = match self.close_demands() {
                Ok(demand) => demand,
                Err(error) => return Err(error),
            };
            if semio_framework_job::InteractiveJob::terminal_is_empty(self) {
                return Ok(deflate_close_complete());
            }
            if !deflate_close_admits_demand(grant, demand) {
                return Ok(deflate_close_idle());
            }
            if let Some(publication) = self.publication.as_mut() {
                let step = publication.close_step(grant)?;
                if publication.terminal_is_empty() {
                    self.publication = None;
                }
                return Ok(match step {
                    InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                    step => step,
                });
            }
            Ok(match self.engine.close_step() {
                Some(released_bytes) => deflate_close_receipt(1, released_bytes),
                None => deflate_close_complete(),
            })
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.publication.is_none() && self.engine.terminal_is_empty()
    }
}
//#endregion 🧵️TunedEncode

fn deflate_raw_tuned(data: &[u8], memory: i32, good: i32, lazy: i32, nice: i32, chain: i32, sync: bool) -> Result<Vec<u8>, String> {
    let value = |name: &str, value: i32| usize::try_from(value).map_err(|_| format!("raw DEFLATE {name} must be non-negative"));
    Ok(TunedDeflateEncodeJob::classic(
        data.to_vec(),
        dynamic::Policy {
            window_bits: 15,
            memory_level: value("memory level", memory)?,
            good: value("good length", good)?,
            lazy: value("lazy length", lazy)?,
            nice: value("nice length", nice)?,
            chain: value("chain length", chain)?,
            finish: if sync { dynamic::Finish::Sync } else { dynamic::Finish::Raw },
        },
        TunedFrame::Raw,
    )
    .finish())
}

/// 🎯 Deterministic Office-compatible raw DEFLATE materialization for container formats.
pub fn deflate_raw_deterministic(data: &[u8]) -> Result<Vec<u8>, String> {
    deflate_raw_tuned(data, 8, 1, 4, 258, 1024, true)
}

/// 🖼️ Deterministic high-search raw DEFLATE materialization for vector-media payloads.
pub fn deflate_raw_deterministic_high_search(data: &[u8]) -> Result<Vec<u8>, String> {
    deflate_raw_tuned(data, 8, 4, 4, 258, 4096, true)
}

/// 🧳 Deterministic compact-block high-search raw DEFLATE for embedded binary payloads.
pub fn deflate_raw_deterministic_compact_high_search(data: &[u8]) -> Result<Vec<u8>, String> {
    deflate_raw_tuned(data, 7, 4, 4, 258, 4096, true)
}
//#endregion DeflateCodec
//#endregion 🦑️DissolvedEngineCodec
