//! ⏱️ Installs the browser's real monotonic source behind the framework clock authority.

use js_sys::{Function, Reflect};
use std::cell::OnceCell;
use wasm_bindgen::{JsCast, JsValue};

//#region ⏱️Authority
thread_local! {
    static SOURCE: OnceCell<(JsValue, Function)> = const { OnceCell::new() };
}

fn browser_now_us() -> Option<u64> {
    SOURCE.with(|source| {
        let (performance, now) = source.get()?;
        let milliseconds = now.call0(performance).ok()?.as_f64()?;
        semio_framework_trace::microseconds_from_milliseconds(milliseconds)
    })
}

/// 🌐️ Admits one cached browser Performance receiver and shares it with jobs and watchdogs.
pub fn install_browser_monotonic_clock() -> Result<(), &'static str> {
    SOURCE.with(|source| {
        if source.get().is_none() {
            let performance = Reflect::get(&js_sys::global(), &JsValue::from_str("performance")).map_err(|_| "browser performance receiver is unavailable")?;
            let now = Reflect::get(&performance, &JsValue::from_str("now")).map_err(|_| "browser monotonic clock is unavailable")?.dyn_into::<Function>().map_err(|_| "browser monotonic clock is not callable")?;
            source.set((performance, now)).map_err(|_| "browser monotonic source was already installed")?;
        }
        browser_now_us().ok_or("browser monotonic clock is outside the unsigned microsecond domain")?;
        semio_framework_trace::install_clock(browser_now_us).map_err(|_| "a different monotonic clock authority is already installed")
    })
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
fn initialize_browser_clock() -> Result<(), JsValue> {
    install_browser_monotonic_clock().map_err(JsValue::from_str)
}
//#endregion ⏱️Authority
