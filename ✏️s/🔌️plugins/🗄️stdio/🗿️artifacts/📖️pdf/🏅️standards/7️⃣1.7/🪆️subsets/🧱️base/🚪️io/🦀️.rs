//! 🚪️ IO stdio.pdf (1.7/🧱️base): the native text, binary and SQLite representations, the mutation bridge, the foreign
//! artifact custody port, and the vocabulary every PDF codec layer shares — the typed engine error and the PDF text
//! string codec (§7.9.2.2). The byte reader is the `import` child (sniff → cross-reference → decrypt → retained graph
//! → typed lanes); the byte writer is the `export` child (typed lanes reconciled onto the retained graph → bytes),
//! together with the typed builders every consumer starts from.
//!
//! Laws (proven in `📤️export/🧪️tests`): `lift(lower(t)) == t` on the typed lanes; `decode(encode(s)) == s`;
//! a write re-states only what a moved lane owns, so every other retained object, dictionary entry
//! and trailer entry — direct COS edits included — survives it untouched; a decoded document is
//! its own fixed point.

use std::collections::HashMap;
use std::sync::OnceLock;

//#region 🔖️Error
/// 🚨 Typed engine error — never silent fabrication. `Unsupported` names a feature this codec
/// refuses to guess at (e.g. an encryption revision it does not implement).
#[derive(Clone, Debug, PartialEq)]
pub enum PdfEngineError {
    NotPdf,
    Unsupported(String),
    Malformed(String),
}

impl std::fmt::Display for PdfEngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdfEngineError::NotPdf => write!(f, "pdf: not a PDF file (missing %PDF- magic)"),
            PdfEngineError::Unsupported(s) => write!(f, "pdf: unsupported: {s}"),
            PdfEngineError::Malformed(s) => write!(f, "pdf: malformed: {s}"),
        }
    }
}
impl std::error::Error for PdfEngineError {}
/// 🧭️ Carries the engine's own unsupported/malformed/not-PDF authority into the canonical Value refusal kinds.
impl From<PdfEngineError> for semio_framework_value::ValueError {
    fn from(error: PdfEngineError) -> Self {
        let kind = match &error { PdfEngineError::Unsupported(_) => semio_framework_value::ValueRefusalKind::UnsupportedOwner, PdfEngineError::NotPdf | PdfEngineError::Malformed(_) => semio_framework_value::ValueRefusalKind::InvalidValue };
        Self::new(kind, error.to_string())
    }
}

pub type PResult<T> = Result<T, PdfEngineError>;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn malformed<T>(msg: impl Into<String>) -> PResult<T> {
    Err(PdfEngineError::Malformed(msg.into()))
}
//#endregion 🔖️Error

//#region 🔤️TextString
/// 🔡 PDFDocEncoding (Annex D.2): code → Unicode scalar (text strings, not fonts).
pub const PDF_DOC_ENCODING: &[(u8, u32)] = &[(24, 728), (25, 711), (26, 710), (27, 729), (28, 733), (29, 731), (30, 730), (31, 732), (32, 32), (33, 33), (34, 34), (35, 35), (36, 36), (37, 37), (38, 38), (39, 39), (40, 40), (41, 41), (42, 42), (43, 43), (44, 44), (45, 45), (46, 46), (47, 47), (48, 48), (49, 49), (50, 50), (51, 51), (52, 52), (53, 53), (54, 54), (55, 55), (56, 56), (57, 57), (58, 58), (59, 59), (60, 60), (61, 61), (62, 62), (63, 63), (64, 64), (65, 65), (66, 66), (67, 67), (68, 68), (69, 69), (70, 70), (71, 71), (72, 72), (73, 73), (74, 74), (75, 75), (76, 76), (77, 77), (78, 78), (79, 79), (80, 80), (81, 81), (82, 82), (83, 83), (84, 84), (85, 85), (86, 86), (87, 87), (88, 88), (89, 89), (90, 90), (91, 91), (92, 92), (93, 93), (94, 94), (95, 95), (96, 96), (97, 97), (98, 98), (99, 99), (100, 100), (101, 101), (102, 102), (103, 103), (104, 104), (105, 105), (106, 106), (107, 107), (108, 108), (109, 109), (110, 110), (111, 111), (112, 112), (113, 113), (114, 114), (115, 115), (116, 116), (117, 117), (118, 118), (119, 119), (120, 120), (121, 121), (122, 122), (123, 123), (124, 124), (125, 125), (126, 126), (128, 8226), (129, 8224), (130, 8225), (131, 8230), (132, 8212), (133, 8211), (134, 402), (135, 8260), (136, 8249), (137, 8250), (138, 8722), (139, 8240), (140, 8222), (141, 8220), (142, 8221), (143, 8216), (144, 8217), (145, 8218), (146, 8482), (147, 64257), (148, 64258), (149, 321), (150, 338), (151, 352), (152, 376), (153, 381), (154, 305), (155, 322), (156, 339), (157, 353), (158, 382), (160, 8364), (161, 161), (162, 162), (163, 163), (164, 164), (165, 165), (166, 166), (167, 167), (168, 168), (169, 169), (170, 170), (171, 171), (172, 172), (174, 174), (175, 175), (176, 176), (177, 177), (178, 178), (179, 179), (180, 180), (181, 181), (182, 182), (183, 183), (184, 184), (185, 185), (186, 186), (187, 187), (188, 188), (189, 189), (190, 190), (191, 191), (192, 192), (193, 193), (194, 194), (195, 195), (196, 196), (197, 197), (198, 198), (199, 199), (200, 200), (201, 201), (202, 202), (203, 203), (204, 204), (205, 205), (206, 206), (207, 207), (208, 208), (209, 209), (210, 210), (211, 211), (212, 212), (213, 213), (214, 214), (215, 215), (216, 216), (217, 217), (218, 218), (219, 219), (220, 220), (221, 221), (222, 222), (223, 223), (224, 224), (225, 225), (226, 226), (227, 227), (228, 228), (229, 229), (230, 230), (231, 231), (232, 232), (233, 233), (234, 234), (235, 235), (236, 236), (237, 237), (238, 238), (239, 239), (240, 240), (241, 241), (242, 242), (243, 243), (244, 244), (245, 245), (246, 246), (247, 247), (248, 248), (249, 249), (250, 250), (251, 251), (252, 252), (253, 253), (254, 254), (255, 255)];

/// 🔤 Decodes a PDF text string (§7.9.2.2): UTF-16BE with BOM, UTF-8 with BOM, else PDFDoc.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_text_string(bytes: &[u8]) -> String {
    if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF {
        let units: Vec<u16> = bytes[2..].chunks(2).filter(|c| c.len() == 2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect();
        return String::from_utf16_lossy(&units);
    }
    if bytes.len() >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
        return String::from_utf8_lossy(&bytes[3..]).into_owned();
    }
    bytes.iter().map(|byte| PDF_DOC_ENCODING.binary_search_by(|(code, _)| code.cmp(byte)).ok().and_then(|index| char::from_u32(PDF_DOC_ENCODING[index].1)).unwrap_or('\u{FFFD}')).collect()
}

/// 🔤 Encodes a text string: PDFDoc when every character is representable, else UTF-16BE + BOM.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_text_string(text: &str) -> Vec<u8> {
    static REVERSE: OnceLock<HashMap<u32, u8>> = OnceLock::new();
    let reverse = REVERSE.get_or_init(|| PDF_DOC_ENCODING.iter().map(|(code, value)| (*value, *code)).collect());
    let mut out = Vec::with_capacity(text.len());
    for character in text.chars() {
        match reverse.get(&(character as u32)) {
            Some(code) if character != '\u{FFFD}' => out.push(*code),
            _ => {
                let mut utf16 = vec![0xFE, 0xFF];
                for unit in text.encode_utf16() {
                    utf16.extend_from_slice(&unit.to_be_bytes());
                }
                return utf16;
            }
        }
    }
    out
}
//#endregion 🔤️TextString

#[path="📦️foreign-artifacts/🦀️.rs"]
pub mod foreign_artifacts;
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;
/// 🌉️ Where a builder or a test meets the central applier: a mutation's diff is applied atomically to the working snapshot, and a
/// rejection becomes a fatal outcome. The schema layer only builds diffs; applying them is an io/store concern.
pub mod mutation_bridge {
    use crate::standards::v1_7::subsets::base::schema::{diff::PdfDiff, mutations::PdfMutation, snapshot::PdfSnapshot};

    /// 🛡️ Applies `outcome` to `snapshot` atomically through the central applier and converts an apply rejection into a fatal outcome.
    pub fn apply_outcome(outcome: protocol::MutationOutcome<PdfDiff>, snapshot: &mut PdfSnapshot) -> protocol::MutationOutcome<PdfDiff> {
        let (diff, messages) = outcome.into_parts();
        match protocol::apply_diff(&diff, snapshot) {
            Ok(next) => {
                *snapshot = next;
                protocol::MutationOutcome::new(diff).absorb_messages(messages)
            }
            Err(error) => protocol::MutationOutcome::new(PdfDiff::default()).absorb_messages(messages).absorb_messages([protocol::MutationMessage::fatal(error.code, error.message).at(error.target)]),
        }
    }

    /// ▶️ Applies one mutation through its leaf-owned diff.
    pub fn apply_pdf_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfMutation) -> protocol::MutationOutcome<PdfDiff> {
        use protocol::Mutation;
        let outcome = mutation.diff(snapshot);
        apply_outcome(outcome, snapshot)
    }

    /// 🧪️ `base` after the `rows` of one edit, applied through the central applier as a graph edit.
    #[cfg(test)]
    pub fn after_rows(base: &PdfSnapshot, rows: PdfDiff) -> PdfSnapshot {
        protocol::apply_diff(&crate::standards::v1_7::subsets::base::schema::diff::graph_edit(rows), base).expect("the fixture rows apply")
    }

    /// 🧪️ `base` after `mutation`, applied through the central applier.
    #[cfg(test)]
    pub fn applied<M: protocol::Mutation<PdfSnapshot, Diff = PdfDiff>>(base: &PdfSnapshot, mutation: &M) -> PdfSnapshot {
        protocol::apply_diff(mutation.diff(base).diff(), base).expect("the fixture mutation applies")
    }
}

pub mod derived_construction {
    use crate::standards::v1_7::subsets::base::schema::diff::PdfDiff;
    use crate::standards::v1_7::subsets::base::io::mutation_bridge::apply_pdf_mutation;
    use crate::standards::v1_7::subsets::base::schema::mutations::{InsertPage, PdfMutation, SetInfo};
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfInfo, PdfPage, PdfSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.pdf.1.7` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct PdfBuilderConstruction {
        snapshot: PdfSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl PdfBuilderConstruction {
        /// ➕ Typed construction: appends a page (the analyzer→builder round-trip acceptance test's
        /// primary entry point -- requirement #8's `InsertPage`, exposed ergonomically).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_page(self, page: PdfPage) -> Self {
            let index = self.snapshot.pages.len();
            let (next, _diff) = self.mutate(PdfMutation::InsertPage(InsertPage { index, page }));
            next
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_info(self, info: PdfInfo) -> Self {
            let (next, _diff) = self.mutate(PdfMutation::SetInfo(SetInfo { info }));
            next
        }
    }

    impl ArtifactBuilder for PdfBuilderConstruction {
        type Snapshot = PdfSnapshot;
        type Mutation = PdfMutation;
        type Diff = PdfDiff;
        fn empty() -> Self {
            Self { snapshot: PdfSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<PdfSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PdfSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_pdf_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
