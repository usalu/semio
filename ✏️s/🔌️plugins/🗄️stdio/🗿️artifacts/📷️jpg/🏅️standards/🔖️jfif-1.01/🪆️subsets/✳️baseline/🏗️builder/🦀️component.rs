//! 🏗️ JpgBaselineBuilder (jfif-1.01/✳️baseline) — wraps the ✳️any subset's `JpgBuilder`
//! (same `JpgSnapshot`, same mutation vocabulary); the only difference is `build()`: it re-runs
//! `check_baseline_conformance` as a hard gate, so a hard T.81 baseline violation can never leave
//! this builder as an `Ok(JpgSnapshot)`, regardless of which path (`from_binary`/`from_text`/
//! `mutate`) produced the in-flight snapshot.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::analyzer::check_baseline_conformance;
use crate::artifacts::jpg::standards::v_jfif_1_01::subsets::any::builder::JpgBuilder as JpgAnyBuilder;
use crate::artifacts::jpg::{JpgDiff, JpgMutation, JpgSnapshot};

//#region 🔖️Builder
#[derive(Clone, Debug, Default)]
pub struct JpgBaselineBuilder(JpgAnyBuilder);

impl ArtifactBuilder for JpgBaselineBuilder {
    type Snapshot = JpgSnapshot;
    type Mutation = JpgMutation;
    type Diff = JpgDiff;

    fn empty() -> Self { Self(JpgAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(JpgAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(JpgAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(JpgAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let (inner, diff) = self.0.mutate(mutation);
        (Self(inner), diff)
    }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }

    /// 🛡️ The real construction gate: however the wrapped snapshot got here, a hard baseline
    /// violation fails `build()` -- soft diagnostics are not surfaced here (`ArtifactBuilder`'s
    /// `build` has no diagnostics-on-success channel), matching `JpgAnyBuilder::build`'s existing
    /// contract of "diagnostics accumulated during mutation, not from validation".
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        let snapshot = self.0.build()?;
        let hard: Vec<dsl::Diagnostic> =
            check_baseline_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, dsl::Severity::Error | dsl::Severity::Fatal)).collect();
        if hard.is_empty() { Ok(snapshot) } else { Err(hard) }
    }
}
//#endregion 🔖️Builder

#[cfg(test)]
mod tests {
    use super::*;

    fn gradient_image(w: u32, h: u32) -> Vec<u8> {
        let mut out = vec![0u8; (w * h * 4) as usize];
        for y in 0..h {
            for x in 0..w {
                let idx = ((y * w + x) * 4) as usize;
                out[idx] = ((x * 255) / w.max(1)) as u8;
                out[idx + 1] = ((y * 255) / h.max(1)) as u8;
                out[idx + 2] = 128;
                out[idx + 3] = 255;
            }
        }
        out
    }

    #[test]
    fn real_encoded_jpeg_builds_clean_via_from_binary() {
        let (w, h) = (24u32, 24u32);
        let snap = JpgSnapshot { width: w, height: h, pixels: gradient_image(w, h), ..JpgSnapshot::default() };
        let bytes = crate::artifacts::jpg::standards::v_jfif_1_01::engine::encode_jpg(&snap).expect("encode");
        let decoded = crate::artifacts::jpg::standards::v_jfif_1_01::engine::decode_jpg(&bytes).expect("decode");
        let packed = <JpgSnapshot as store::ArtifactPack>::encode_pack(&decoded);
        let built = JpgBaselineBuilder::from_binary(&packed).expect("from_binary").build().expect("real baseline JPEG must build clean");
        assert!(built.frame.is_some());
    }

    #[test]
    fn empty_snapshot_fails_build_with_no_frame() {
        let err = JpgBaselineBuilder::empty().build().expect_err("an empty snapshot has no SOF0 frame -- must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::jpg::standards::v_jfif_1_01::subsets::baseline::analyzer::CODE_NO_FRAME));
    }
}
