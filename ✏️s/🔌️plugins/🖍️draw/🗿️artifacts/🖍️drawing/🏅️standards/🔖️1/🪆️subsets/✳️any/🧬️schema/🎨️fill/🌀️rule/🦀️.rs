//! 🌀️ Authored compound-path winding shared by paint, picking and export.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslScalar)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all="camelCase")]
#[cfg_attr(test, serde(rename_all="camelCase"))]
pub enum FillRule { #[default] Evenodd, Nonzero }
impl FillRule {
    pub const fn as_str(self)->&'static str {match self {Self::Evenodd=>"evenodd",Self::Nonzero=>"nonzero"}}
    pub fn parse(value:&str)->Result<Self,&'static str> {match value {"evenodd"=>Ok(Self::Evenodd),"nonzero"=>Ok(Self::Nonzero),_=>Err("Invalid fill rule")}}
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
