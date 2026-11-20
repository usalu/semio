//! 🌐️ Bounded SVG preview transfer to the browser image decoder.

use js_sys::{Array, Function, Promise, Reflect, Uint8Array};
use semio_framework_async::{browser::{future_to_promise, JsFuture}, CancelToken};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

const PAGE_BYTES: usize = 256 * 1024;

pub async fn rasterize_browser_svg(contents: Vec<u8>, width: u32, height: u32, cancel: CancelToken) -> Result<Vec<u8>, String> {
    if cancel.is_cancelled_now() {
        return Err("icon SVG preview was cancelled".into());
    }
    let pixel_bytes = width.checked_mul(height).and_then(|area| area.checked_mul(4))
        .ok_or("icon SVG preview dimensions exceeded pixel credits")?;
    let global = js_sys::global();
    let door = Reflect::get(&global, &JsValue::from_str("semioWgpuRasterizeIconSvg"))
        .ok().and_then(|value| value.dyn_into::<Function>().ok())
        .ok_or("this isolate installs no icon SVG preview decoder")?;
    let size = contents.len();
    let mut contents = Some(contents);
    let read = Closure::<dyn FnMut(u32) -> Uint8Array>::new(move |offset| {
        let offset = offset as usize;
        let end = offset.saturating_add(PAGE_BYTES).min(size);
        let Some(chunk) = contents.as_ref().and_then(|bytes| bytes.get(offset..end)) else {
            return Uint8Array::new_with_length(0);
        };
        let page = Uint8Array::from(chunk);
        if end == size { contents = None; }
        page
    });
    let args = Array::new();
    for number in [size as f64, f64::from(width), f64::from(height)] {
        args.push(&JsValue::from_f64(number));
    }
    let scope = cancel.child_now();
    let wait = scope.clone();
    let cancelled = future_to_promise(async move {
        wait.cancelled().await;
        Ok(JsValue::UNDEFINED)
    });
    args.push(read.as_ref());
    args.push(&cancelled);
    let result = async {
        let pending = door.apply(&JsValue::NULL, &args).map_err(js_error)?
            .dyn_into::<Promise>().map_err(|_| "icon SVG preview decoder returned no promise".to_string())?;
        let bytes = JsFuture::from(pending).await.map_err(js_error)?
            .dyn_into::<Uint8Array>().map_err(|_| "icon SVG preview decoder returned no pixels".to_string())?;
        if bytes.length() != pixel_bytes {
            return Err("icon SVG preview decoder returned incorrect pixel credits".into());
        }
        Ok(bytes.to_vec())
    }.await;
    scope.cancel_now();
    result
}

fn js_error(error: JsValue) -> String {
    error.as_string().unwrap_or_else(|| format!("{error:?}"))
}
