//! 🏗️ GifBuilder (89a) — local ArtifactBuilder until SDK Wave 3, plus typed constructors
//! (`new`/`add_frame`/`set_loop_count`) — THE way to build an 89a document from scratch, matching
//! the svg builder's "typed constructors, not raw snapshot literals" precedent (ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION D2). This is what the
//! analyzer→builder round-trip acceptance test drives: it reconstructs a document using ONLY
//! these typed methods, never `from_snapshot`/`SetSnapshot` directly.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::diff::GifDiff;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::mutations::GifMutation;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{GifFrame, GifSnapshot};

//#region 🔖️Builder
#[derive(Clone, Debug, Default)]
pub struct GifBuilder {
    snapshot: GifSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

//#region 🔖️TypedConstructors
impl GifBuilder {
    /// 🏗️ Starts a fresh 89a document at the given logical screen size.
    pub fn new(width: u32, height: u32) -> Self {
        Self { snapshot: GifSnapshot { width, height, ..GifSnapshot::default() }, diagnostics: Vec::new() }
    }
    /// 🏗️ Appends one animation frame, in order.
    pub fn add_frame(mut self, frame: GifFrame) -> Self {
        self.snapshot.frames.push(frame);
        self
    }
    /// 🏗️ Sets the NETSCAPE2.0 loop count (`None` = no loop extension, plays once).
    pub fn set_loop_count(mut self, loop_count: Option<u16>) -> Self {
        self.snapshot.loop_count = loop_count;
        self
    }
}
//#endregion 🔖️TypedConstructors

impl ArtifactBuilder for GifBuilder {
    type Snapshot = GifSnapshot;
    type Mutation = GifMutation;
    type Diff = GifDiff;
    fn empty() -> Self {
        Self { snapshot: GifSnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<GifSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<GifSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::gif::standards::v89a::subsets::any::schema::mutations::apply_gif_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <GifDiff as protocol::MutationDiff<GifSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder
