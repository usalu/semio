//! 🌐️ The browser `performance.now()` source behind the monotonic port; no type of it is exported past the port.
use super::{unavailable,MonotonicPort};
use js_sys::{Function,Reflect};
use semio_framework_value_control_contract::ValueError;
use std::cell::OnceCell;
use wasm_bindgen::{JsCast,JsValue};

thread_local!{static SOURCE:OnceCell<(JsValue,Function)>=const{OnceCell::new()};}

fn admit()->Result<(JsValue,Function),ValueError>{
 let performance=Reflect::get(&js_sys::global(),&JsValue::from_str("performance")).map_err(|_|unavailable("browser performance receiver is unavailable"))?;
 let now=Reflect::get(&performance,&JsValue::from_str("now")).map_err(|_|unavailable("browser monotonic clock is unavailable"))?.dyn_into::<Function>().map_err(|_|unavailable("browser monotonic clock is not callable"))?;
 Ok((performance,now))
}

/// 🌐️ Whole microseconds of `performance.now()` since the browser time origin.
pub struct BrowserMonotonic;
impl MonotonicPort for BrowserMonotonic{
 type Mark=u64;
 fn mark()->Result<u64,ValueError>{
  SOURCE.with(|source|{
   if source.get().is_none(){let _=source.set(admit()?);}
   let(performance,now)=source.get().ok_or_else(||unavailable("browser monotonic clock is unavailable"))?;
   let milliseconds=now.call0(performance).map_err(|_|unavailable("browser monotonic clock call failed"))?.as_f64().ok_or_else(||unavailable("browser monotonic clock is not numeric"))?;
   if !milliseconds.is_finite()||milliseconds<0.0||milliseconds>=u64::MAX as f64/1000.0{return Err(unavailable("browser monotonic clock is outside the unsigned microsecond domain"))}
   Ok((milliseconds*1000.0) as u64)
  })
 }
 fn elapsed_micros(origin:u64,now:u64)->Option<u128>{now.checked_sub(origin).map(u128::from)}
}
