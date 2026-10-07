//! 🛡️ Conformance over decoded native values.
#[path="🧮️facts/🦀️.rs"]
pub mod facts;
pub use facts::{JpgBaselineFacts,JpgSamplingFact};
use crate::JpgSnapshot;
use crate::schema::snapshot::JpgHuffmanClass;
use semio_framework_value::ValueError;
use semio_framework_diagnostic::{Diagnostic,FaultCode,FaultScope,Severity,TextSpan};
    /// 🏷️ SOF0 (baseline sequential DCT) marker byte, T.81 Table B.1.
    pub const SOF0: u8 = 0xC0;

    //#region 🔖️Conformance
    pub const CODE_NO_FRAME: &str = "stdio.jpg.baseline.no-frame";
    pub const CODE_SOF_MARKER: &str = "stdio.jpg.baseline.sof-marker";
    pub const CODE_PRECISION: &str = "stdio.jpg.baseline.precision";
    pub const CODE_ARITHMETIC: &str = "stdio.jpg.baseline.arithmetic-conditioning-present";
    pub const CODE_HUFFMAN_TABLE_COUNT: &str = "stdio.jpg.baseline.huffman-table-count";
    pub const CODE_COMPONENT_SAMPLING: &str = "stdio.jpg.baseline.component-sampling";

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real ITU-T T.81 / ISO 10918-1 Annex F baseline sequential DCT conformance checks (JFIF
    /// 1.01 container) against one already-decoded `JpgSnapshot`. Shared single source of truth:
    /// `JpgBaselineComposer::compose` hard-gates on this (pre-serialization, authoritative),
    /// `JpgBaselineBuilder::build` hard-gates on this too, and the registered `SubsetValidator`
    /// re-runs it post-hoc against the wire payload for the D5 validate-on-build hook.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_baseline_conformance(snapshot: &JpgSnapshot) -> Vec<Diagnostic> {
        check_baseline_conformance_with(snapshot, &mut |_, _| Ok(())).expect("the unbounded conformance callback cannot fail")
    }

    /// ⏱️ Applies the baseline rules with bounded traversal checkpoints.
    pub fn check_baseline_conformance_with(snapshot: &JpgSnapshot, checkpoint: &mut dyn FnMut(usize, usize) -> Result<(), ValueError>) -> Result<Vec<Diagnostic>, ValueError> {
        checkpoint(0, snapshot.huffman_tables.len() + snapshot.frame.as_ref().map_or(0, |frame| frame.components.len()))?;
        let mut out = Vec::new();

        let Some(frame) = &snapshot.frame else {
            out.push(hard(CODE_NO_FRAME, "no SOF0 frame header retained on this snapshot -- baseline conformance cannot be certified without one (never decoded, or built without going through engine::decode_jpg)".into()));
            return Ok(out);
        };

        if snapshot.sof_marker != SOF0 {
            out.push(hard(CODE_SOF_MARKER, format!("frame marker 0x{:02X} is not SOF0 (0x{SOF0:02X}) -- T.81 Annex F baseline sequential DCT is SOF0 only (no progressive/extended/arithmetic SOFn variants)", snapshot.sof_marker)));
        }
        if frame.precision != 8 {
            out.push(hard(CODE_PRECISION, format!("sample precision {} is not 8 -- T.81 §4.2 baseline sequential DCT mandates 8-bit samples", frame.precision)));
        }
        if snapshot.arithmetic {
            out.push(hard(CODE_ARITHMETIC, "a DAC (arithmetic-coding conditioning) segment was present -- T.81 Annex F baseline sequential DCT is Huffman-entropy-coded only".into()));
        }

        let mut dc_count = 0;
        let mut ac_count = 0;
        for (position, table) in snapshot.huffman_tables.iter().enumerate() {
            if position % 256 == 0 { checkpoint(position, snapshot.huffman_tables.len())?; }
            match table.class { JpgHuffmanClass::Dc => dc_count += 1, JpgHuffmanClass::Ac => ac_count += 1 }
        }
        if dc_count > 2 || ac_count > 2 {
            out.push(soft(CODE_HUFFMAN_TABLE_COUNT, format!("{dc_count} DC / {ac_count} AC Huffman table(s) defined -- typical JFIF baseline practice never needs more than 2 of each (one luma, one chroma)")));
        }
        if frame.components.len() > 4 {
            out.push(soft(CODE_COMPONENT_SAMPLING, format!("{} frame components -- JFIF 1.01 conventionally encodes grayscale (1) or YCbCr (3) images; more than 4 is unusual", frame.components.len())));
        }
        for (position, c) in frame.components.iter().enumerate() {
            if position % 256 == 0 { checkpoint(position, frame.components.len())?; }
            if !(1..=4).contains(&c.h_sampling) || !(1..=4).contains(&c.v_sampling) {
                out.push(soft(CODE_COMPONENT_SAMPLING, format!("component {} has sampling factors {}x{} outside JFIF's conventional 1..=4 range", c.id, c.h_sampling, c.v_sampling)));
            }
        }
        Ok(out)
    }

/// 🧮️ Applies baseline rules to decoded facts without native tables or layouts.
pub fn check_baseline_facts(facts:&JpgBaselineFacts)->Vec<Diagnostic>{
    check_baseline_facts_with(facts,&mut |_,_|Ok(())).expect("unbounded baseline fact checkpoint")
}
/// ⏱️ Traverses first-party conformance facts under caller progress and cancellation.
pub fn check_baseline_facts_with(facts:&JpgBaselineFacts,checkpoint:&mut dyn FnMut(usize,usize)->Result<(),ValueError>)->Result<Vec<Diagnostic>,ValueError>{
    checkpoint(0,facts.components.len())?;
    let mut out=Vec::new();
    if !facts.has_frame {out.push(hard(CODE_NO_FRAME,"no decoded frame observation is available".into()));return Ok(out);}
    if !facts.baseline_sequential {out.push(hard(CODE_SOF_MARKER,"frame process is not baseline sequential DCT".into()));}
    if facts.sample_precision!=8 {out.push(hard(CODE_PRECISION,format!("sample precision {} is not 8",facts.sample_precision)));}
    if facts.arithmetic_conditioning {out.push(hard(CODE_ARITHMETIC,"arithmetic coding conditioning was present".into()));}
    if facts.dc_table_count>2||facts.ac_table_count>2 {out.push(soft(CODE_HUFFMAN_TABLE_COUNT,format!("{} DC / {} AC Huffman tables",facts.dc_table_count,facts.ac_table_count)));}
    if facts.components.len()>4 {out.push(soft(CODE_COMPONENT_SAMPLING,format!("{} frame components",facts.components.len())));}
    for(position,component)in facts.components.iter().enumerate(){
        checkpoint(position,facts.components.len())?;
        if !(1..=4).contains(&component.horizontal)||!(1..=4).contains(&component.vertical){out.push(soft(CODE_COMPONENT_SAMPLING,format!("component {} has sampling factors {}x{} outside 1..=4",component.id,component.horizontal,component.vertical)));}
    }
    Ok(out)
}
