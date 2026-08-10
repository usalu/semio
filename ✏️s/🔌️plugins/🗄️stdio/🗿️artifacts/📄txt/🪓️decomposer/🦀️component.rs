//! 📑️ TxtDecomposer — local ArtifactDecomposer until SDK Wave 3.

use crate::artifacts::txt::{TxtSnapshot};

//#region 🔖️LocalContracts
/// 🎚 Soft confidence for partial decomposition success.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Confidence { High, Medium, Low }

/// 📥 One decomposition source blob.
#[derive(Clone, Debug)]
pub enum DecomposeSource<'a> { Text(&'a str), Binary(&'a [u8]) }

/// 📦 Decomposition result carrying soft diagnostics.
#[derive(Clone, Debug)]
pub struct Decomposition<T> {
    pub parts: T,
    pub confidence: Confidence,
    pub diagnostics: Vec<dsl::Diagnostic>,
}

/// 📑️ Local decomposer contract (W3 swaps to SDK `ArtifactDecomposer`).
pub trait ArtifactDecomposer: Sized {
    type Snapshot;
    type Parts;
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts>;
}
//#endregion 🔖️LocalContracts

//#region 🔖️Parts
/// 🧩 Decomposed `stdio.txt` parts.
#[derive(Clone, Debug, Default)]
pub struct TxtParts { pub snapshot: Option<TxtSnapshot>, }
//#endregion 🔖️Parts

//#region 🔖️Decomposer
/// 📑️ Decomposes `stdio.txt` sources.
pub struct TxtDecomposer;

impl ArtifactDecomposer for TxtDecomposer {
    type Snapshot = TxtSnapshot;
    type Parts = TxtParts;
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts> {
        let mut parts = TxtParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = Confidence::High;
        for source in sources {
            match source {
                DecomposeSource::Text(text) => match <TxtSnapshot as store::DocumentDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.decompose.text",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
                    }
                },
                DecomposeSource::Binary(bytes) => match <TxtSnapshot as store::DocumentPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.decompose.binary",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
                    }
                },
            }
        }
        Decomposition { parts, confidence, diagnostics }
    }
}
//#endregion 🔖️Decomposer
