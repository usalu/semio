//! 🧭️ The retained field frontier follows the authored AppFrame binary order.
use super::*;
#[derive(Clone,Copy)]
pub(super) enum Atom{Number(usize),Byte(usize),Optional(usize,usize),Bytes(usize),Text(usize),List(u8),Handle,Archive,Query,Identity,Finish}
pub(super) fn plan(tag:u8,index:usize)->Result<Atom,OriginalTurnFailure>{use Atom::*;Ok(match tag{
 0=>match index{0=>Number(0),_=>Finish},
 1=>match index{0=>Number(0),1..=7=>Bytes(index-1),_=>Finish},
 2|5=>match index{0=>List(0),1=>Text(0),_=>Finish},
 3|4|11=>match index{0=>Number(0),1|2=>Bytes(index-1),3=>Text(0),_=>Finish},
 6|14=>match index{0=>Number(0),1=>Bytes(0),_=>Finish},
 7=>match index{0=>Number(0),1=>Text(0),2|3=>Bytes(index-2),_=>Finish},
 8=>match index{0=>Number(0),1=>Text(0),2=>Bytes(0),_=>Finish},
 9=>match index{0=>Optional(0,0),1|2=>Bytes(index-1),_=>Finish},
 10=>match index{0=>Number(0),1..=6=>Bytes(index-1),_=>Finish},
 12=>match index{0=>Number(0),1=>List(1),_=>Finish},
 13=>match index{0=>Bytes(0),1|2=>Number(index-1),3..=5=>Bytes(index-2),_=>Finish},
 15=>match index{0=>Number(0),1=>Text(0),2=>List(2),3=>List(3),_=>Finish},
 16=>match index{0=>Text(0),1=>List(3),2=>Bytes(0),_=>Finish},
 17=>match index{0|1=>Text(index),_=>Finish},18=>match index{0=>Text(0),_=>Finish},
 19|20=>match index{0=>Optional(0,0),1=>Bytes(0),_=>Finish},
 21=>match index{0=>Optional(0,0),1|2=>Text(index-1),3|4=>Number(index-2),5=>Bytes(0),_=>Finish},
 22=>match index{0=>Number(0),_=>Finish},23=>match index{0=>Query,_=>Finish},
 24=>match index{0=>Number(0),1=>List(4),_=>Finish},
 25=>match index{0|1=>Number(index),2|3=>Bytes(index-2),_=>Finish},
 26=>match index{0=>Number(0),1=>Archive,_=>Finish},
 27=>match index{0|1=>Number(index),2=>Byte(0),3..=5=>Number(index-1),6=>Bytes(0),_=>Finish},
 28=>match index{0=>Number(0),1=>Handle,_=>Finish},
 29=>match index{0=>Number(0),1=>Handle,2=>Byte(0),3=>Number(5),4=>Byte(1),5=>Text(1),6=>Number(6),7=>Text(2),8=>Bytes(0),_=>Finish},
 30=>match index{0=>Number(0),1=>Handle,2=>Bytes(0),3=>Byte(0),_=>Finish},
 31=>match index{0|1=>Number(index),2=>Identity,3=>Number(2),_=>Finish},
 32=>match index{0=>Number(0),1=>List(5),_=>Finish},
 _=>return Err(invalid("AppFrame tag is outside its authored binary schema"))
})}
