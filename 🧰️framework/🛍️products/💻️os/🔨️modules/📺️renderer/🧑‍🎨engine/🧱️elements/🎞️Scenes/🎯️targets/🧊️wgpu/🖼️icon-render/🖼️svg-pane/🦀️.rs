//! 🖼️ Owns the exact SVG request and preview raster of each IconRender pane.

use std::cell::RefCell;

use semio_framework_async::CancelToken;
use semio_framework_pixels::RasterImage;
use semio_renderer_wgpu_contracts::RasterUploadRefusal;
use semio_renderer_wgpu_scene_state::RASTER_UPLOAD_BYTE_CAPACITY;

use super::icon_export::{asset::{IconExportAssetFault, IconExportAssetRequest}, preview::IconSvgPreview, IconSvgRejected};

const PREVIEW_CAPACITY: usize = 16;
const REQUEST_BYTE_CAPACITY: usize = 16 * 1024;
const RETRY_DELAYS_SECONDS: [f64; 4] = [0.25, 0.5, 1.0, 2.0];

struct Request {
    json: String,
    url: String,
    raster_key: String,
}

enum Work {
    Loading(IconExportAssetRequest),
    Retry(f64),
    Drawing(IconSvgPreview, CancelToken),
    Rejected(IconSvgRejected),
    Publishing(RasterImage),
    Uploaded { painted: bool },
    Failed,
}

impl Work {
    fn cancel(&mut self) {
        match self {
            Self::Loading(load) => load.cancel(),
            Self::Drawing(_, cancel) => { cancel.cancel_now(); }
            _ => {}
        }
    }

    fn close_step(&mut self) -> bool {
        match self {
            Self::Loading(load) => load.close_step(),
            Self::Drawing(drawing, _) => drawing.close_step(),
            Self::Rejected(rejected) => rejected.close_step(),
            _ => true,
        }
    }
}

struct Pane {
    host: String,
    request: Option<Request>,
    work: Option<Work>,
    retiring: bool,
    retries: usize,
}

impl Pane {
    fn asset_failed(&mut self, now: f64, fault: IconExportAssetFault) -> Work {
        let Some(delay) = RETRY_DELAYS_SECONDS.get(self.retries).filter(|_| fault.is_retryable()) else { return Work::Failed };
        self.retries += 1;
        Work::Retry(now + delay)
    }
}

#[derive(Default)]
struct Previews {
    serial: u64,
    cursor: usize,
    panes: Vec<Pane>,
}

thread_local! {
    static PREVIEWS: RefCell<Previews> = RefCell::default();
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Paint {
    Rendering,
    Drawing(usize),
    Ready(String),
    Failed,
}

pub(super) fn observe(host: &str, json: &str, url: &str) -> Paint {
    if json.len() > REQUEST_BYTE_CAPACITY {
        cancel(host);
        return Paint::Failed;
    }
    PREVIEWS.with_borrow_mut(|previews| {
        let pane = match previews.panes.iter().position(|pane| pane.host == host) {
            Some(index) => &mut previews.panes[index],
            None if previews.panes.len() < PREVIEW_CAPACITY => {
                previews.panes.push(Pane { host: host.into(), request: None, work: None, retiring: false, retries: 0 });
                previews.panes.last_mut().unwrap()
            }
            None => return Paint::Failed,
        };
        if let Some(request) = &pane.request {
            if request.json == json && request.url == url {
                return match &mut pane.work {
                    Some(Work::Uploaded { painted }) if !pane.retiring => {
                        *painted = true;
                        Paint::Ready(request.raster_key.clone())
                    },
                    Some(Work::Drawing(drawing, _)) if !pane.retiring => {
                        drawing.vector_progress().filter(|&(_, total)| total > 0).map_or(Paint::Rendering, |(done, total)| Paint::Drawing(done.saturating_mul(100) / total))
                    }
                    Some(Work::Failed | Work::Rejected(_)) if !pane.retiring => Paint::Failed,
                    _ => Paint::Rendering,
                };
            }
        }
        let Some(serial) = previews.serial.checked_add(1) else { return Paint::Failed };
        previews.serial = serial;
        pane.request = Some(Request { json: json.into(), url: url.into(), raster_key: format!("icon-svg-preview:{serial}") });
        pane.retries = 0;
        pane.retiring = pane.work.is_some();
        if let Some(work) = &mut pane.work {
            work.cancel();
        }
        Paint::Rendering
    })
}

pub fn cancel(host: &str) {
    PREVIEWS.with_borrow_mut(|previews| {
        if let Some(pane) = previews.panes.iter_mut().find(|pane| pane.host == host) {
            pane.request = None;
            pane.retiring = true;
            if let Some(work) = &mut pane.work {
                work.cancel();
            }
        }
    });
}

pub fn close_step(host: Option<&str>) -> bool {
    PREVIEWS.with_borrow_mut(|previews| {
        let Some(index) = previews.panes.iter().position(|pane| host.is_none_or(|host| pane.host == host)) else { return true };
        let pane = &mut previews.panes[index];
        if let Some(work) = &mut pane.work {
            work.cancel();
            if !work.close_step() {
                return false;
            }
        }
        previews.panes.swap_remove(index);
        host.is_some()
    })
}

/// ⏳️ Whether a browser frame can advance the preview without an external completion.
#[cfg(any(test, target_arch = "wasm32"))]
pub fn has_pending() -> bool {
    PREVIEWS.with_borrow(|previews| previews.panes.iter().any(|pane| match &pane.work {
        Some(Work::Drawing(drawing, _)) => !drawing.waits_for_raster(),
        Some(Work::Loading(_) | Work::Retry(_) | Work::Uploaded { .. } | Work::Failed) => pane.retiring,
        _ => true,
    }))
}

/// ⏰️ Earliest recoverable asset retry on the host scheduler's monotonic clock.
pub fn retry_deadline() -> Option<f64> {
    PREVIEWS.with_borrow(|previews| previews.panes.iter().filter_map(|pane| match pane.work {
        Some(Work::Retry(due)) if !pane.retiring => Some(due),
        _ => None,
    }).min_by(f64::total_cmp))
}

pub fn advance(
    now: f64,
    wake: Option<semio_renderer_wgpu_host_base::RuntimeHostWaker>,
    mut publish: impl FnMut(&str, &str, u32, u32, &mut Vec<u8>) -> Result<(), RasterUploadRefusal>,
) -> bool {
    PREVIEWS.with_borrow_mut(|previews| {
        for _ in 0..previews.panes.len() {
            let index = previews.cursor % previews.panes.len();
            previews.cursor = (index + 1) % previews.panes.len();
            let pane = &mut previews.panes[index];
            if pane.retiring {
                if pane.work.as_mut().is_some_and(|work| !work.close_step()) {
                    if matches!(&pane.work, Some(Work::Drawing(drawing, _)) if drawing.waits_for_raster()) && wake.is_some() {
                        continue;
                    }
                    return true;
                }
                pane.work = None;
                pane.retiring = false;
                if pane.request.is_none() {
                    previews.panes.swap_remove(index);
                }
                return true;
            }
            if matches!(pane.work, Some(Work::Retry(due)) if now < due) { continue }
            let request = pane.request.as_ref().expect("active pane has a request");
            pane.work = Some(match pane.work.take() {
                None | Some(Work::Retry(_)) => match IconExportAssetRequest::new(&request.url) {
                    Ok(load) => Work::Loading(load),
                    Err(fault) => pane.asset_failed(now, fault),
                },
                Some(Work::Loading(mut load)) => match load.take_ready() {
                    Some(Ok(asset)) => {
                        pane.retries = 0;
                        let cancel = CancelToken::root_now();
                        match IconSvgPreview::new(&request.json, asset, cancel.clone(), RASTER_UPLOAD_BYTE_CAPACITY) {
                            Ok(drawing) => Work::Drawing(drawing.with_waker(wake.clone()), cancel),
                            Err(rejected) => Work::Rejected(rejected),
                        }
                    }
                    Some(Err(fault)) => pane.asset_failed(now, fault),
                    None => {
                        pane.work = Some(Work::Loading(load));
                        continue;
                    }
                },
                Some(Work::Drawing(mut drawing, cancel)) => match drawing.advance() {
                    Ok(Some(image)) => Work::Publishing(image),
                    Ok(None) if drawing.waits_for_raster() && wake.is_some() => {
                        pane.work = Some(Work::Drawing(drawing, cancel));
                        continue;
                    }
                    Ok(None) => Work::Drawing(drawing, cancel),
                    Err(_) => {
                        if drawing.close_step() { Work::Failed } else { Work::Drawing(drawing, cancel) }
                    }
                },
                Some(Work::Rejected(mut rejected)) => if rejected.close_step() { Work::Failed } else { Work::Rejected(rejected) },
                Some(Work::Publishing(mut image)) => match publish(&pane.host, &request.raster_key, image.width, image.height, &mut image.pixels) {
                    Ok(()) => Work::Uploaded { painted: false },
                    Err(RasterUploadRefusal::Busy) if !image.pixels.is_empty() => Work::Publishing(image),
                    Err(_) => Work::Failed,
                }
                Some(work) => {
                    pane.work = Some(work);
                    continue;
                }
            });
            return true;
        }
        false
    })
}

/// 🔄️ Retries a previously painted upload if its candidate never made the texture resident.
pub fn reconcile(mut resident: impl FnMut(&str) -> bool) -> bool {
    PREVIEWS.with_borrow_mut(|previews| {
        let mut retry = false;
        for pane in &mut previews.panes {
            retry |= pane.retiring || pane.work.is_none();
            let Some(Work::Uploaded { painted }) = &mut pane.work else { continue };
            if !std::mem::take(painted) || pane.retiring {
                continue;
            }
            if pane.request.as_ref().is_some_and(|request| !resident(&request.raster_key)) {
                pane.work = None;
                retry = true;
            }
        }
        retry
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn svg_pane_retry_lifecycle() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️retry.json")).unwrap();
        let publish = |_: &str, _: &str, _: u32, _: u32, _: &mut Vec<u8>| panic!("no raster before a valid GLB");
        let mut now = 0.0;
        observe("retry", "{}", "/retry.glb");
        for delay in fixture["retryDelayMs"].as_array().unwrap() {
            assert!(advance(now, None, publish));
            let mut owner = crate::asset_authority::take_next_renderer_asset().unwrap();
            let token = owner.token();
            owner.begin_close();
            crate::asset_authority::return_renderer_asset(owner).unwrap();
            assert!(crate::asset_authority::retire_cancelled_renderer_asset_step());
            super::super::icon_export::asset::reject(token, IconExportAssetFault::Transport { retryable: true });
            assert!(advance(now, None, publish));
            now += delay.as_f64().unwrap() / 1000.0;
            assert_eq!(retry_deadline(), Some(now));
            assert!(!has_pending() && !advance(now - 0.001, None, publish));
        }
        assert!(close_step(Some("retry")));
        assert!(retry_deadline().is_none() && !advance(now, None, publish));
    }

    #[test]
    fn native_svg_pane_never_publishes_a_replaced_request() {
        let first = r#"{"drawing":{"fill":true}}"#;
        let next = r#"{"drawing":{"fill":false,"hiddenLines":true}}"#;
        let host = "fixture-host";
        let image = || RasterImage { width: 1, height: 1, pixels: vec![13, 49, 81, 128] };
        let finish_raster = || PREVIEWS.with_borrow_mut(|previews| {
            let pane = &mut previews.panes[0];
            pane.work = Some(Work::Publishing(image()));
            pane.request.as_ref().unwrap().raster_key.clone()
        });

        assert_eq!(observe(host, first, "fixture.glb"), Paint::Rendering);
        assert!(has_pending());
        let first_key = finish_raster();
        assert!(advance(0.0, None, |_, key, _, _, pixels| {
            assert_eq!(key, first_key);
            assert_eq!(pixels.as_slice(), &[13, 49, 81, 128]);
            Err(RasterUploadRefusal::Busy)
        }));
        assert_eq!(observe(host, next, "fixture.glb"), Paint::Rendering);
        assert!(advance(0.0, None, |_, _, _, _, _| panic!("superseded raster must retire")));
        let next_key = finish_raster();
        assert_ne!(first_key, next_key);
        assert!(advance(0.0, None, |_, key, _, _, pixels| {
            assert_eq!(key, next_key);
            assert_eq!(std::mem::take(pixels), [13, 49, 81, 128]);
            Ok(())
        }));
        assert_eq!(observe(host, next, "fixture.glb"), Paint::Ready(next_key.clone()));
        assert!(!reconcile(|key| key == next_key));
        assert!(!has_pending());
        assert!(close_step(Some(host)));
        assert!(!advance(0.0, None, |_, _, _, _, _| unreachable!()));
    }
}
