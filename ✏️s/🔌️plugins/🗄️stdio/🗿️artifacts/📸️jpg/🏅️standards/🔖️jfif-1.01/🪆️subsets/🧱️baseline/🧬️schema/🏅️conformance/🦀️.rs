//! 🛡️ Conformance over decoded native values.
#[path="🧮️facts/🦀️.rs"]
pub mod facts;
pub use facts::{JpgBaselineFacts,JpgSamplingFact};
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

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
