//! 🔢️ Original float words advance through paid lexical, short-endpoint and exact-midpoint conversion events.
use crate::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
use std::mem::size_of;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original float word changed its source or retained frontier")}
fn fixed()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn unit()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn permits(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items!=0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
const DIGITS:usize=768;

const LIMBS:usize=DIGITS.div_ceil(9);
const BASE:u64=1_000_000_000;
const QUANTUM:usize=64;
const POW5:[u32;14]=[1,5,25,125,625,3125,15625,78125,390625,1953125,9765625,48828125,244140625,1220703125];
const POW10:[u32;9]=[1,10,100,1000,10000,100000,1000000,10000000,100000000];

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum RoundPhase{Prefix,Lower,Upper,Seed,Multiply,Compare,Done}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct RoundControl{
 phase:RoundPhase,prefix:u64,power:i16,low:u64,scientific:i16,remaining:u16,five:bool,factor:u32,chunk:u16,
 used:usize,index:usize,limit:usize,carry:u64,decimal_power:i16,decimal_digits:usize,compared:usize,
}
/// ⚖️ Short decimal endpoints prove ordinary rounding; ambiguous intervals use an exact dyadic midpoint.
/// Every event copies its fixed control record and touches at most 64 coefficient limbs or decimal positions.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct DecimalRounding{control:RoundControl,limbs:[u32;LIMBS]}
impl DecimalRounding{
 fn new()->Self{Self{control:RoundControl{phase:RoundPhase::Prefix,prefix:0,power:0,low:0,scientific:0,remaining:0,five:false,factor:0,chunk:0,used:0,index:0,limit:0,carry:0,decimal_power:0,decimal_digits:0,compared:0},limbs:[0;LIMBS]}}
 fn copy_demand(&self)->usize{
  2*size_of::<RoundControl>()+match self.control.phase{
   RoundPhase::Lower|RoundPhase::Upper=>58,
   RoundPhase::Seed=>2*size_of::<u32>(),
   RoundPhase::Multiply=>QUANTUM.min(self.control.limit.saturating_sub(self.control.index)+2)*size_of::<u32>(),
   _=>0,
  }
 }
 fn endpoint(mut prefix:u64,power:i16,copied:&mut usize)->Result<u64,ValueError>{
  let mut word=[0u8;32];*copied+=word.len();let mut start=20;
  loop{start-=1;word[start]=b'0'+(prefix%10)as u8;*copied+=1;prefix/=10;if prefix==0{break}}
  let mut end=20;word[end]=b'e';end+=1;*copied+=1;
  if power<0{word[end]=b'-';end+=1;*copied+=1}
  let magnitude=power.unsigned_abs();let mut divisor=100;
  while divisor>1&&magnitude<divisor{divisor/=10}
  loop{word[end]=b'0'+((magnitude/divisor)%10)as u8;end+=1;*copied+=1;if divisor==1{break}divisor/=10}
  let text=std::str::from_utf8(&word[start..end]).map_err(|_|invalid())?;
  text.parse::<f64>().map(f64::to_bits).map_err(|_|invalid())
 }
 fn start_pass(c:&mut RoundControl){
  c.index=0;c.limit=c.used;c.carry=0;
  c.chunk=c.remaining.min(if c.five{13}else{29});
  c.factor=if c.five{POW5[usize::from(c.chunk)]}else{1u32<<c.chunk};
 }
 fn start_compare(c:&mut RoundControl,limbs:&[u32;LIMBS]){
  let mut top=limbs[c.used-1];let mut width=1;
  while top>=10{top/=10;width+=1}
  c.decimal_digits=(c.used-1)*9+width;c.compared=0;c.phase=RoundPhase::Compare;
 }
 fn decimal_digit(c:&RoundControl,limbs:&[u32;LIMBS],index:usize)->u8{
  if index>=c.decimal_digits{return 0}
  let first=c.decimal_digits-(c.used-1)*9;
  let(limb,power)=if index<first{(c.used-1,first-1-index)}else{let rest=index-first;(c.used-2-rest/9,8-rest%9)};
  ((limbs[limb]/POW10[power])%10)as u8
 }
 fn decision(c:&mut RoundControl,order:std::cmp::Ordering)->u64{
  c.phase=RoundPhase::Done;
  c.low+u64::from(match order{std::cmp::Ordering::Less=>false,std::cmp::Ordering::Greater=>true,std::cmp::Ordering::Equal=>c.low&1!=0})
 }
 fn advance(&mut self,digits:&[u8],scientific:i16,sticky:bool)->(Result<Option<u64>,ValueError>,usize){
  let mut c=self.control;let mut copied=2*size_of::<RoundControl>();
  let result=(||{
   match c.phase{
    RoundPhase::Prefix=>{
     let count=digits.len().min(19);let mut prefix=0u64;
     for digit in &digits[..count]{prefix=prefix*10+u64::from(*digit-b'0')}
     c.prefix=prefix;c.scientific=scientific;c.power=scientific-i16::try_from(count).map_err(|_|invalid())?+1;c.phase=RoundPhase::Lower;
    },
    RoundPhase::Lower=>{
     c.low=Self::endpoint(c.prefix,c.power,&mut copied)?;
     if digits.len()<=19&&!sticky{c.phase=RoundPhase::Done;return Ok(Some(c.low))}
     c.phase=RoundPhase::Upper;
    },
    RoundPhase::Upper=>{
     let upper=Self::endpoint(c.prefix+1,c.power,&mut copied)?;
     if upper==c.low{c.phase=RoundPhase::Done;return Ok(Some(c.low))}
     if upper.checked_sub(c.low)!=Some(1){return Err(invalid())}
     c.phase=RoundPhase::Seed;
    },
    RoundPhase::Seed=>{
     if c.used!=0{return Err(invalid())}
     let exponent=((c.low>>52)&0x7ff)as i16;
     let significand=(c.low&0x000f_ffff_ffff_ffff)|if exponent==0{0}else{1u64<<52};
     let mut coefficient=(significand<<1)|1;
     while coefficient!=0{
      if c.used==LIMBS{return Err(invalid())}
      self.limbs[c.used]=(coefficient%BASE)as u32;copied+=size_of::<u32>();c.used+=1;coefficient/=BASE;
     }
     let power=if exponent==0{-1075}else{exponent-1076};
     c.remaining=power.unsigned_abs();c.five=power<0;c.decimal_power=power.min(0);
     if c.remaining==0{Self::start_compare(&mut c,&self.limbs)}else{Self::start_pass(&mut c);c.phase=RoundPhase::Multiply}
    },
    RoundPhase::Multiply=>{
     let mut touched=0;
     while c.index<c.limit&&touched<QUANTUM{
      let product=u64::from(self.limbs[c.index])*u64::from(c.factor)+c.carry;
      self.limbs[c.index]=(product%BASE)as u32;copied+=size_of::<u32>();c.carry=product/BASE;c.index+=1;touched+=1;
     }
     while c.index==c.limit&&c.carry!=0&&touched<QUANTUM{
      if c.used==LIMBS{return Err(invalid())}
      self.limbs[c.used]=(c.carry%BASE)as u32;copied+=size_of::<u32>();c.carry/=BASE;c.used+=1;touched+=1;
     }
     if c.index==c.limit&&c.carry==0{
      c.remaining-=c.chunk;
      if c.remaining==0{Self::start_compare(&mut c,&self.limbs)}else{Self::start_pass(&mut c)}
     }
    },
    RoundPhase::Compare=>{
     let boundary=i16::try_from(c.decimal_digits).map_err(|_|invalid())?+c.decimal_power-1;
     let order=c.scientific.cmp(&boundary);
     if order!=std::cmp::Ordering::Equal{return Ok(Some(Self::decision(&mut c,order)))}
     let count=digits.len().max(c.decimal_digits);let end=(c.compared+QUANTUM).min(count);
     for index in c.compared..end{
      let source=digits.get(index).copied().unwrap_or(b'0')-b'0';
      let order=source.cmp(&Self::decimal_digit(&c,&self.limbs,index));
      c.compared=index+1;
      if order!=std::cmp::Ordering::Equal{return Ok(Some(Self::decision(&mut c,order)))}
     }
     if c.compared==count{return Ok(Some(Self::decision(&mut c,if sticky{std::cmp::Ordering::Greater}else{std::cmp::Ordering::Equal})))}
    },
    RoundPhase::Done=>return Err(invalid()),
   }
   Ok(None)
  })();
  self.control=c;(result,copied)
 }
}

/// 🔢️ The original Rust float field yields a parsed scalar or its declared invalid-input default signal.
pub struct OriginalFloatWord{binding:Option<(usize,usize)>,digits:[u8;DIGITS],rounding:DecimalRounding,stored:usize,significant:usize,fraction:usize,exponent:u128,negative:bool,exponent_negative:bool,exponent_sign:bool,decimal:bool,any:bool,sticky:bool,lex:u8,offset:usize,phase:u8,output:Option<Option<f64>>,closing:bool,progress:RetainedCloneProgress,issued:Option<Option<u64>>}
impl OriginalFloatWord{
 pub fn new()->Self{Self{binding:None,digits:[0;DIGITS],rounding:DecimalRounding::new(),stored:0,significant:0,fraction:0,exponent:0,negative:false,exponent_negative:false,exponent_sign:false,decimal:false,any:false,sticky:false,lex:0,offset:0,phase:0,output:None,closing:false,progress:Default::default(),issued:None}}
 pub fn phase(&self)->u8{self.phase}
 pub fn offset(&self)->usize{self.offset}
 pub fn next_demands(&self,source:&str)->RetirementDemand{RetirementDemand{copy_bytes:match self.phase{0=>size_of::<Option<(usize,usize)>>(),1=>64.min(source.len().saturating_sub(self.offset))+128,2=>if self.needs_rounding(){self.rounding.copy_demand()+size_of::<Option<Option<f64>>>()+size_of::<u8>()}else{size_of::<Option<Option<f64>>>()+size_of::<u8>()},3=>size_of::<Option<Option<f64>>>(),_=>0},..fixed()}}
 fn digit(&mut self,byte:u8){self.any=true;if self.decimal{self.fraction+=1}if self.stored!=0||byte!=b'0'{self.significant+=1;if self.stored<DIGITS{self.digits[self.stored]=byte;self.stored+=1}else{self.sticky|=byte!=b'0'}}}
 fn exponent_digit(&mut self,byte:u8,maximum:u128){self.exponent=self.exponent.saturating_mul(10).saturating_add(u128::from(byte-b'0')).min(maximum)}
 fn read(&mut self,byte:u8,position:usize,length:usize){
  match self.lex{
   0=>match byte{b'+'|b'-' if position==0=>self.negative=byte==b'-',b'0'..=b'9'=>{self.lex=1;self.digit(byte)},b'.'=>{self.lex=1;self.decimal=true},_=>self.lex=if length<=9{4}else{5}},
   1=>match byte{b'0'..=b'9'=>self.digit(byte),b'.' if !self.decimal=>self.decimal=true,b'e'|b'E' if self.any=>self.lex=2,_=>self.lex=5},
   2=>match byte{b'+'|b'-' if !self.exponent_sign=>{self.exponent_sign=true;self.exponent_negative=byte==b'-'},b'0'..=b'9'=>{self.lex=3;self.exponent_digit(byte,length as u128+4096)},_=>self.lex=5},
   3=>if byte.is_ascii_digit(){self.exponent_digit(byte,length as u128+4096)}else{self.lex=5},
   _=>{},
  }
 }
 fn scientific(&self)->i128{
  let explicit=if self.exponent_negative{-(self.exponent as i128)}else{self.exponent as i128};
  explicit-self.fraction as i128+self.significant as i128-1
 }
 fn needs_rounding(&self)->bool{self.lex!=4&&self.any&&self.lex!=0&&self.lex!=2&&self.lex!=5&&self.stored!=0&&(-324..=308).contains(&self.scientific())}
 fn interpret_step(&mut self,source:&str)->(Result<Option<Option<f64>>,ValueError>,usize){
  if self.lex==4{return(Ok(Some(source.parse().ok())),0)}
  if !self.any||self.lex==0||self.lex==2||self.lex==5{return(Ok(Some(None)),0)}
  if self.stored==0{return(Ok(Some(Some(if self.negative{-0.0}else{0.0}))),0)}
  let scientific=self.scientific();
  if scientific>308{return(Ok(Some(Some(if self.negative{f64::NEG_INFINITY}else{f64::INFINITY}))),0)}
  if scientific< -324{return(Ok(Some(Some(if self.negative{-0.0}else{0.0}))),0)}
  let(result,copied)=self.rounding.advance(&self.digits[..self.stored],scientific as i16,self.sticky);
  (result.map(|output|output.map(|bits|Some(f64::from_bits(bits|if self.negative{1u64<<63}else{0})))),copied)
 }
 pub fn next_work_demand(&self)->u64{1}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn advance(&mut self,source:&str,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<Option<f64>>>{
  self.progress=Default::default();let mut work=0;
  let result=(||{let binding=(source.as_ptr()as usize,source.len());if self.closing||self.phase>=4||self.binding.is_some_and(|b|b!=binding){return Err(invalid())}
   let demand=self.next_demands(source);if !permits(g,demand){return Ok(None)}
   work=self.next_work_demand();self.progress=RetainedCloneProgress{copied_bytes:demand.copy_bytes,..unit()};
   match self.phase{
    0=>{self.binding=Some(binding);self.phase=1},
    1=>{let end=(self.offset+64).min(source.len());for position in self.offset..end{self.read(source.as_bytes()[position],position,source.len())}self.offset=end;if end==source.len()||self.lex==5{self.phase=2}},
    2=>{let(result,copied)=self.interpret_step(source);self.progress.copied_bytes=copied;if let Some(output)=result?{self.output=Some(output);self.phase=3;self.progress.copied_bytes+=size_of::<Option<Option<f64>>>()+size_of::<u8>()}},
    3=>{self.phase=4;let output=self.output.take();self.issued=output.map(|value|value.map(f64::to_bits));return Ok(output)},
    _=>return Err(invalid()),
   }Ok(None)
  })();RetainedWorkOutcome{result:result.map_err(|error|error.with_retained_progress(self.progress)),work}
 }
 pub fn restore_output(&mut self,output:Option<f64>){assert_eq!(self.phase,4);assert_eq!(self.issued,Some(output.map(f64::to_bits)));self.issued=None;self.output=Some(output);self.phase=3}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demands(&self,_copy:usize)->Result<RetirementDemand,ValueError>{Ok(if self.binding.is_some()||self.output.is_some()||self.issued.is_some(){RetirementDemand{copy_bytes:size_of::<Option<(usize,usize)>>()+size_of::<Option<Option<f64>>>()+size_of::<Option<Option<u64>>>(),..fixed()}}else{Default::default()})}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  self.progress=Default::default();if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !permits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.binding=None;self.output=None;self.issued=None;self.progress=RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()};RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(self.progress)),work:1}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.binding.is_none()&&self.output.is_none()&&self.issued.is_none()}
}
impl Drop for OriginalFloatWord{fn drop(&mut self){assert!(self.terminal_is_empty(),"original numeric field requires explicit output and source closure")}}

#[cfg(test)]
#[path="🧪️laws/🦀️.rs"]
mod laws;
