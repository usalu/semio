//! 🖼️ Atomic asset replacement ordered for bounded capacity in both history directions.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(crate) enum Step {Detach,Remove,Add,Replace}
pub(crate) fn replacement_steps(count:usize,capacity:usize,removable:bool,next_exists:bool)->Result<&'static [Step],&'static str>{
    use Step::*;
    if capacity==0||count>capacity||(count==0&&(removable||next_exists)){return Err("raster.asset-envelope-invalid");}
    if !next_exists&&count==capacity {return if removable {Ok(&[Detach,Remove,Add,Replace])}else{Err("raster.asset-capacity")};}
    Ok(match (next_exists,removable) {(false,false)=>&[Add,Replace],(false,true)=>&[Add,Replace,Remove],(true,false)=>&[Replace],(true,true)=>&[Replace,Remove]})
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
