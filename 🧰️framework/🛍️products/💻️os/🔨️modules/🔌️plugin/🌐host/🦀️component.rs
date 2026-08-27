//! 🌐️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2/sdk-async, design-abi.md §4): the async host-
//! capability API, replacing `host_port`/`component::host_*`. `log`/`now_ms`/`trace_span` stay
//! synchronous (they wrap the `pure` WIT import, the world's only import for BOTH worlds); every
//! other `Host` method is a two-arm match over `HostBackend`:
//!
//! - `HostBackend::Poll` (`world actor`, today's only LIVE backend): builds a
//!   `semio_framework::kernel::Effect`, hands it to a `⚛️reactor/📮️requests::RequestRegistry`, and
//!   awaits the matching `Event::Completed`/`Event::HttpChunk`/`Event::JobCompleted` on a LATER
//!   `reactor::poll` call — this arm is BYTE-FOR-BYTE what every method did before this packet.
//! - `HostBackend::Direct` (only constructible with `component-guest-async` on): calls the matching
//!   `host-async` import directly and `.await`s wasmtime's own `component-model-async` correlation
//!   — no `RequestId`, no registry slot, no `poll` round-trip. Since B1 world-collapse the IMPORT
//!   it needs is on `world actor` itself (there is no second world any more), but the arm is still
//!   a landing pad rather than a live path: the mounted host (`🖥️host/🦀️component.rs`'s
//!   `WasmtimeRuntime`) answers every `host-async` import with a typed `host-async.poll-backed`
//!   fault, because a poll-shaped turn has no point at which such a future could resolve. The
//!   runtime that serves them for real is built on `🖥️host/⏳️imports.rs`'s `AsyncActorHostState`.
//!
//! Both arms build/consume the SAME `*-params` records `effects.wit`/`host-async` share via
//! `use effects.{...}` — the host side's `🖥️host/🧪️schema-parity/🦀️component.rs` (4/4) verifies this
//! mechanically for the HOST's own bindings; there is no guest-side equivalent yet (see report).
//!
//! `Host` is constructed per app instance (never a process-global singleton — see
//! `important.md`'s "Replace, never wrap" list: `set_host_backbone_channel` is explicitly one of
//! the things a pooled multi-instance actor cannot keep). Cloning is cheap either way (`Poll`
//! wraps an `Rc`; `Direct` carries no state at all).

use crate::reactor::requests::RequestRegistry;
use dsl::DslValue;
use semio_framework::kernel::{CapabilityId, ClipboardFragment, Effect, IconRenderExportItem, JobPlacement, RequestId, RequestOutcome, WindowHandle, WindowKindId};
use semio_framework::{Fault, FaultCode, FaultOrigin, MediaType};

#[path = "📖️body/🦀️component.rs"]
pub mod body;
pub use body::BodyReader;

/// 🩹️ Decodes a `RequestOutcome` into the `Result<Vec<u8>, Fault>` every `host::*` async call
/// resolves to — `Err` bytes are `dsl::encode_fault_bytes` output, the SAME convention every
/// synchronous `host_*` wrapper already used. Called from `⚛️reactor/🦀️component.rs`'s
/// `Event::Completed` routing step before it hands the result to `RequestRegistry::resolve` —
/// that routing step lives in `wit_bridge`, so this is gated identically (native never reaches it).
// 🚫️async: E1 pure decode consumed by `⚛️reactor/🦀️component.rs`'s sync `world actor` boundary —
// `dsl::{decode_fault_bytes,encode_fault_bytes}` are both plain `fn`, zero suspension here — R9.
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
pub(crate) fn outcome_to_result(outcome: RequestOutcome) -> Result<Vec<u8>, Fault> {
    match outcome {
        RequestOutcome::Ok(bytes) => Ok(bytes),
        RequestOutcome::Err(bytes) => Err(dsl::decode_fault_bytes(&bytes)),
    }
}

//#region 🔖️Direct

/// ⚡️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async → B1 world-collapse): the guest-side
/// `host-async` import bindings. `world actor-async` is DELETED — `world actor` imports
/// `host-async` itself now — so this targets `world: "actor"`, the one and only world.
///
/// 🧹️ This is a SECOND `wit_bindgen::generate!` for the same world (the root `🦀️component.rs`'s
/// `pub mod component` runs the first, and owns the `export!`). Two calls in one crate is normal
/// wit-bindgen usage — each gets its own module tree, and only the exporting one emits symbols —
/// but since the collapse it is also REDUNDANT: `mod component`'s own generated tree now carries
/// `semio::framework::host_async` too. Folding `HostBackend::Direct`'s arms onto that single
/// generated module (and deleting this one) is a genuine simplification the collapse unlocks; it is
/// deliberately NOT done here, because it rewrites this file's ~40 `Direct` arms rather than its
/// bindgen mount, and belongs to whichever packet next owns this file. Never call
/// `crate::host::direct::*` outside this file's own `HostBackend::Direct` arms.
#[cfg(all(feature = "component-guest-async", target_arch = "wasm32", target_env = "p2"))]
pub mod direct {
    #![allow(unsafe_op_in_unsafe_fn, dead_code)]

    wit_bindgen::generate!({
        world: "actor",
        path: "../../🧬️schema",
    });

    pub use semio::framework::{effects, host_async, types};
}

/// 🚧️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): the typed fault every `HostBackend::
/// Direct` arm returns on a build that has `component-guest-async` on but is NOT a real
/// wasm32-wasip2 target — `⚛️reactor/💼️jobs`'s `JobCtx::host()` is gated on the feature ALONE (no
/// arch check, see that module's own doc), so `Host`'s Direct arms must still type-check natively;
/// this is what they do at runtime instead of reaching a real `wit_bindgen` import that does not
/// exist off-target. Mirrors this crate's `log`/`now_ms`/`trace_span` double-`#[cfg]`-with-fallback
/// idiom below, just returning a fault instead of an `eprintln!`.
#[cfg(feature = "component-guest-async")]
async fn direct_unavailable_fault(op: &str) -> Fault {
    Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.host.direct-unavailable"), format!("host::{op}: the Direct (component-guest-async) backend requires a real wasm32-wasip2 build"))
}

/// 📦️ The SAME `store::pack_rt::encode_wire_value(dsl::to_dsl_value(..))` idiom
/// `⚛️reactor/🦀️component.rs`'s `kernel_effect_to_wit` uses for every Rust-only field type crossing
/// the WIT boundary — duplicated here (not imported) because it targets a DIFFERENT nominal
/// `pack`/`Vec<u8>` wire shape per call site, not a different encoding: both are the literal same
/// bytes, only the surrounding record type differs between `world actor`'s generated module and
/// this file's own `direct` module.
// 🚫️async: E5 executor bridge — R9, and byte-for-byte the same decision the sibling
// `⚛️reactor/🦀️component.rs` already made for its own identical `pack` helper. `pack` is consumed
// from sync `Option::map` closures in half a dozen `*-params` constructors below (R10 residue shape
// 1, where `.await` is illegal), and `store::pack_rt::encode_wire_value` is pure in-memory wire
// encoding with zero suspension points of its own — so it is Ready on its first poll and the bridge
// is sound. NOTE the justification is the CALLEE's purity, not "world actor imports no host-async"
// (which B1 world-collapse made false); the sibling module's wording is now stale, the bridges
// themselves are not.
#[cfg(feature = "component-guest-async")]
fn pack<T: serde::Serialize>(value: &T) -> Vec<u8> {
    semio_framework::io::resolve_ready(store::pack_rt::encode_wire_value(&dsl::to_dsl_value(value).unwrap_or(DslValue::Null)))
}

/// 🔀️ `kernel::JobPlacement` → the Direct world's `job-placement` enum — only ever called from
/// inside `Host::spawn_job`'s OWN `#[cfg(all(target_arch = "wasm32", target_env = "p2"))]`-gated
/// arm, so (unlike `kernel_effect_to_direct_wit`/`direct_unavailable_fault`, which must compile on
/// EVERY `component-guest-async` build, arch or no arch — see `direct_unavailable_fault`'s doc) this
/// one can just gate on the same full arch check `direct` itself does; no native-fallback shape
/// needed since nothing off-target ever calls it.
#[cfg(all(feature = "component-guest-async", target_arch = "wasm32", target_env = "p2"))]
fn kernel_placement_to_direct_wit(placement: JobPlacement) -> direct::effects::JobPlacement {
    use direct::effects::JobPlacement as W;
    match placement {
        JobPlacement::Inline => W::Inline,
        JobPlacement::Isolated => W::Isolated,
        JobPlacement::Exclusive => W::Exclusive,
    }
}

//#endregion 🔖️Direct

/// 🔀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): which world this `Host` handle talks
/// to — see module doc.
#[derive(Clone)]
enum HostBackend {
    Poll(RequestRegistry),
    #[cfg(feature = "component-guest-async")]
    Direct,
}

impl Default for HostBackend {
    fn default() -> Self {
        HostBackend::Poll(RequestRegistry::default())
    }
}

/// 🌐️ Per-instance async host-capability handle — see module doc.
#[derive(Clone, Default)]
pub struct Host {
    backend: HostBackend,
}

/// 🌊️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): the envelope `http_fetch` resolves to —
/// `body` streams so a Direct-world caller can start consuming before the response finishes; a
/// Poll-world caller gets the SAME type back, already fully reassembled by `⚛️reactor/📮️requests::
/// RequestRegistry::append_chunk` (see `Host::http_fetch`'s own doc).
pub struct HttpFetchResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: BodyReader,
}

/// 🩹️ The poll world's `Event::Completed` payload for `Effect::HttpRequest` — JSON-encoded by
/// `🖥️host/⚡️effects/🦀️component.rs`'s `encode_http_response`/`HttpResponseWire` (that struct is
/// private to the host crate; this is the guest-side decode counterpart, field-for-field the same
/// shape, never imported directly since they live in different crates on either side of the wasm
/// boundary).
#[derive(serde::Serialize, serde::Deserialize)]
struct HttpResponseWire {
    status: u16,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

impl Host {
    pub async fn new(registry: RequestRegistry) -> Self {
        Self { backend: HostBackend::Poll(registry) }
    }

    /// ⚡️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): the Direct-world constructor.
    /// Unused by any live caller yet — no actor task drives `world actor-async`'s `runner::run` in
    /// this wave (see this packet's report's honest gaps), landing pad only, matching
    /// `AsyncActorHostState`'s own "built, not yet wired to a live `Store`" shape on the host side.
    #[cfg(feature = "component-guest-async")]
    pub async fn new_direct() -> Self {
        Self { backend: HostBackend::Direct }
    }

    /// 📮️ `None` once this handle is `Direct` (there is no `RequestRegistry` in that world at
    /// all — the returned future itself IS the correlation). Was infallible before this packet;
    /// nothing in this crate calls it today (checked — see this packet's report), so narrowing the
    /// signature breaks no live caller.
    pub async fn registry(&self) -> Option<&RequestRegistry> {
        match &self.backend {
            HostBackend::Poll(registry) => Some(registry),
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => None,
        }
    }

    /// 🔥️ Fire-and-forget dispatch shared by every non-completable method below — Poll queues
    /// `effect` exactly as `RequestRegistry::emit` always did; Direct hands the WHOLE effect
    /// variant to `host-async`'s one fire-and-forget door, `emit`, per that WIT func's own doc
    /// ("takes the whole existing `effect` variant rather than growing a hand-written signature
    /// per case").
    async fn emit(&self, effect: Effect) {
        match &self.backend {
            HostBackend::Poll(registry) => registry.emit(effect),
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::emit(&kernel_effect_to_direct_wit(effect));
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = effect;
                }
            }
        }
    }

    //#region 🔖️Blobs
    pub async fn blob_load(&self, hash: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let hash = hash.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::BlobLoad { req, hash }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::blob_load(direct::effects::BlobLoadParams { hash }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = hash;
                    Err(direct_unavailable_fault("blob-load").await)
                }
            }
        }
    }

    pub async fn blob_write(&self, media_type: MediaType, bytes: Vec<u8>) -> Result<Vec<u8>, Fault> {
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::BlobWrite { req, media_type, bytes }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::blob_write(direct::effects::BlobWriteParams { media_type: pack(&media_type), bytes }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (media_type, bytes);
                    Err(direct_unavailable_fault("blob-write").await)
                }
            }
        }
    }

    /// 🌊️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): the STREAMING counterpart to
    /// `blob_load` — no chunked blob backend exists anywhere in this codebase yet (host report,
    /// `terra-async-imports`: "`blob-read` calls the same buffered path and hands back an already-
    /// `done`, one-item `ChunkStreamProducer`"), so the Poll arm below reuses the identical
    /// `Effect::BlobLoad` round-trip `blob_load` uses and wraps the WHOLE result as a single-chunk
    /// `BodyReader` — genuinely honest, not a stand-in (real chunked blob delivery needs a new host
    /// service neither this packet nor the host's own owns). The Direct arm calls the real
    /// streaming import.
    pub async fn blob_read(&self, hash: impl Into<String>) -> Result<BodyReader, Fault> {
        let hash = hash.into();
        match &self.backend {
            HostBackend::Poll(registry) => match registry.request(move |req| Effect::BlobLoad { req, hash }).await {
                Ok(bytes) => Ok(BodyReader::poll_buffered(bytes).await),
                Err(fault) => Err(fault),
            },
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    match direct::host_async::blob_read(hash).await {
                        Ok(stream) => Ok(BodyReader::direct(stream).await),
                        Err(bytes) => Err(dsl::decode_fault_bytes(&bytes)),
                    }
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = hash;
                    Err(direct_unavailable_fault("blob-read").await)
                }
            }
        }
    }
    //#endregion 🔖️Blobs

    //#region 🔖️Http
    #[allow(clippy::too_many_arguments)]
    pub async fn http_request(&self, method: impl Into<String>, url: impl Into<String>, headers: Vec<(String, String)>, body: Option<Vec<u8>>, stream: bool) -> Result<Vec<u8>, Fault> {
        let method = method.into();
        let url = url.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::HttpRequest { req, method, url, headers, body, stream }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    let response = direct::host_async::http_fetch(direct::effects::HttpParams { method, url, headers, body, streaming: stream }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))?;
                    let body = collect_direct_body(response.body).await?;
                    Ok(serde_json::to_vec(&HttpResponseWire { status: response.status, headers: response.headers, body }).unwrap_or_default())
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (method, url, headers, body, stream);
                    Err(direct_unavailable_fault("http-fetch").await)
                }
            }
        }
    }

    /// 🌊️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): the STREAMING counterpart to
    /// `http_request` — `host-async`'s `http-fetch` renames the poll world's `http-request` and
    /// resolves to a `stream<u8>` body (design-abi.md §4's fix for `Event::HttpChunk`'s discarded-
    /// chunk bug, see `⚛️reactor/📮️requests::RequestRegistry::append_chunk`). Poll arm reuses the
    /// SAME `Effect::HttpRequest` round-trip `http_request` uses (`append_chunk` has already fully
    /// reassembled the body by the time this resolves — a poll-world completion is one shot, there
    /// is nothing left to stream), decodes the JSON `HttpResponseWire` envelope
    /// `🖥️host/⚡️effects/🦀️component.rs::encode_http_response` writes, and hands back the WHOLE body
    /// as one chunk. Direct arm gets the real thing: the response head arrives as soon as it's
    /// available and `body` streams independently.
    pub async fn http_fetch(&self, method: impl Into<String>, url: impl Into<String>, headers: Vec<(String, String)>, body: Option<Vec<u8>>) -> Result<HttpFetchResponse, Fault> {
        let method = method.into();
        let url = url.into();
        match &self.backend {
            HostBackend::Poll(registry) => {
                let bytes = registry.request(move |req| Effect::HttpRequest { req, method, url, headers, body, stream: true }).await?;
                let wire: HttpResponseWire =
                    serde_json::from_slice(&bytes).map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.host.http-decode-error"), format!("could not decode the http-request completion envelope: {error}")))?;
                Ok(HttpFetchResponse { status: wire.status, headers: wire.headers, body: BodyReader::poll_buffered(wire.body).await })
            }
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    let response = direct::host_async::http_fetch(direct::effects::HttpParams { method, url, headers, body, streaming: true }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))?;
                    Ok(HttpFetchResponse { status: response.status, headers: response.headers, body: BodyReader::direct(response.body).await })
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (method, url, headers, body);
                    Err(direct_unavailable_fault("http-fetch").await)
                }
            }
        }
    }
    //#endregion 🔖️Http

    //#region 🔖️Documents
    pub async fn document_read(&self, doc: semio_framework::kernel::ArtifactHandle, lane: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let lane = lane.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::DocumentRead { req, doc, lane }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::document_read(direct::effects::DocumentReadParams { doc: doc.0 as u64, lane }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (doc, lane);
                    Err(direct_unavailable_fault("document-read").await)
                }
            }
        }
    }

    pub async fn document_write(&self, doc: semio_framework::kernel::ArtifactHandle, lane: impl Into<String>, ops: Vec<u8>) -> Result<Vec<u8>, Fault> {
        let lane = lane.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::DocumentWrite { req, doc, lane, ops }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::document_write(direct::effects::DocumentWriteParams { doc: doc.0 as u64, lane, ops }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (doc, lane, ops);
                    Err(direct_unavailable_fault("document-write").await)
                }
            }
        }
    }
    //#endregion 🔖️Documents

    //#region 🔖️Links
    pub async fn resolve_link(&self, link: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let link = link.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::LinkResolve { req, link }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::link_resolve(link.into_bytes()).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = link;
                    Err(direct_unavailable_fault("link-resolve").await)
                }
            }
        }
    }
    //#endregion 🔖️Links

    //#region 🔖️Io — CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM absorption
    /// 🌉️ Absorbs the old host import `io-routes`: `kind = "io-routes"`, `filter` is the
    /// `{source, target}` query — see `effects.wit`'s `registry-query`.
    pub async fn io_routes(&self, source: &str, target: &str) -> Result<Vec<u8>, Fault> {
        #[derive(serde::Serialize)]
        struct IoRoutesFilter<'a> {
            source: &'a str,
            target: &'a str,
        }
        let filter = dsl::to_dsl_value(&IoRoutesFilter { source, target }).ok();
        self.registry_query("io-routes", filter).await
    }

    /// 🌉️ Absorbs the old host import `io-identify`: `kind = "io-identify"`.
    pub async fn io_identify(&self, payload: &[u8]) -> Result<Vec<u8>, Fault> {
        #[derive(serde::Serialize)]
        struct IoIdentifyFilter {
            payload: Vec<u8>,
        }
        let filter = dsl::to_dsl_value(&IoIdentifyFilter { payload: payload.to_vec() }).ok();
        self.registry_query("io-identify", filter).await
    }

    /// 🌉️ Absorbs the old host import `io-run` (multi-hop, cross-plugin) — distinct from the
    /// `semio.io-run` COLD JOB kind, which is the single-hop, THIS-plugin-only registry lookup
    /// (see `⚛️reactor/💼️jobs`).
    ///
    /// 🚧️ The Poll arm below is UNCHANGED (byte-for-byte, per this packet's own mandate) and stays
    /// a documented APPROXIMATION: `semio_framework::kernel::Effect` has no `IoRun` variant yet
    /// (the same A3 gap `🦀️component.rs`'s own `wit_effect_to_kernel` already documents, and the
    /// host report's own honest gaps confirm), so it synthesizes a one-hop `IoCompose` instead of
    /// a real multi-hop route. The Direct arm calls the REAL `host-async` `io-run` import, which
    /// has no such gap — this is a genuine capability improvement Direct gets over Poll, not a
    /// bug to reconcile between the two arms.
    pub async fn io_run(&self, source: impl Into<String>, target: impl Into<String>, payload: Vec<u8>) -> Result<Vec<u8>, Fault> {
        let source = source.into();
        let target = target.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::IoCompose { req, key: format!("{source}->{target}"), sources: vec![String::from_utf8_lossy(&payload).into_owned()] }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::io_run(direct::effects::IoRunParams { source, target, payload }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (source, target, payload);
                    Err(direct_unavailable_fault("io-run").await)
                }
            }
        }
    }

    /// 🌉️ One-hop compose, unchanged semantics from the old `io-compose` host import.
    pub async fn io_compose(&self, key: impl Into<String>, sources: Vec<String>) -> Result<Vec<u8>, Fault> {
        let key = key.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::IoCompose { req, key, sources }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::io_compose(direct::effects::IoComposeParams { key: key.into_bytes(), sources: pack(&sources) }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (key, sources);
                    Err(direct_unavailable_fault("io-compose").await)
                }
            }
        }
    }
    //#endregion 🔖️Io

    //#region 🔖️Registry
    pub async fn registry_query(&self, kind: impl Into<String>, filter: Option<DslValue>) -> Result<Vec<u8>, Fault> {
        let kind = kind.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::RegistryQuery { req, kind, filter }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::registry_query(direct::effects::RegistryQueryParams { kind, filter: filter.map(|value| pack(&value)).unwrap_or_default() }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (kind, filter);
                    Err(direct_unavailable_fault("registry-query").await)
                }
            }
        }
    }
    //#endregion 🔖️Registry

    //#region 🔖️Cache
    pub async fn cache_derive(&self, engine_id: impl Into<String>, input: Vec<u8>) -> Result<Vec<u8>, Fault> {
        let engine_id = engine_id.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::CacheDerive { req, engine_id, input }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::cache_derive(direct::effects::CacheDeriveParams { engine_id, input }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (engine_id, input);
                    Err(direct_unavailable_fault("cache-derive").await)
                }
            }
        }
    }

    pub async fn cache_read(&self, engine_id: impl Into<String>, key: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let engine_id = engine_id.into();
        let key = key.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::CacheRead { req, engine_id, key }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::cache_read(direct::effects::CacheReadParams { engine_id, key: key.into_bytes() }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (engine_id, key);
                    Err(direct_unavailable_fault("cache-read").await)
                }
            }
        }
    }
    //#endregion 🔖️Cache

    //#region 🔖️Extensions / Messaging / Respond
    pub async fn invoke_extension(&self, extension_id: impl Into<String>, capability: impl Into<String>, request_json: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let extension_id = extension_id.into();
        let capability = capability.into();
        let request_json = request_json.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::InvokeExtension { req, extension_id, capability, request_json }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::invoke_extension(direct::effects::InvokeExtensionParams { extension_id, capability, payload: request_json.into_bytes() }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (extension_id, capability, request_json);
                    Err(direct_unavailable_fault("invoke-extension").await)
                }
            }
        }
    }

    pub async fn send_message(&self, target: semio_framework::kernel::MessageEndpoint, payload: Vec<u8>) {
        self.emit(Effect::SendMessage { target, payload }).await;
    }

    pub async fn publish_event(&self, topic: impl Into<String>, payload: Vec<u8>) {
        self.emit(Effect::PublishEvent { topic: topic.into(), payload }).await;
    }

    pub async fn subscribe(&self, topic: impl Into<String>) {
        self.emit(Effect::Subscribe { topic: topic.into() }).await;
    }

    pub async fn unsubscribe(&self, topic: impl Into<String>) {
        self.emit(Effect::Unsubscribe { topic: topic.into() }).await;
    }

    /// ↩️ Answers an inbound `Event::Request{req, ..}` — must be called within the bounded number
    /// of turns the host allows, or the caller sees a timeout fault.
    pub async fn respond(&self, req: RequestId, result: Result<Vec<u8>, Vec<u8>>) {
        let result = match result {
            Ok(bytes) => RequestOutcome::Ok(bytes),
            Err(bytes) => RequestOutcome::Err(bytes),
        };
        self.emit(Effect::Respond { req, result }).await;
    }
    //#endregion 🔖️Extensions

    //#region 🔖️Timers / Jobs
    pub async fn set_timer(&self, id: u64, after_ms: u64, repeat: bool) {
        self.emit(Effect::SetTimer { id, after_ms, repeat }).await;
    }

    /// 💼️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1, design-abi.md §4): moves a genuinely long-
    /// running computation off this turn's own budget. Poll arm allocates its `job` id from the
    /// SAME `RequestRegistry` counter every other `host::*` call uses (`self.call`'s `RequestId`) —
    /// `job == req.0` is the correlation the host's completion, `Event::JobCompleted{job, ..}`,
    /// resolves against (`⚛️reactor/🦀️component.rs`'s `Event::JobCompleted` routing step), so no
    /// separate job/request mapping table is needed on either side of the component boundary. The
    /// host drives `start-job`/`step-job` under a `JobBudget` across as many turns as the job
    /// needs — see `🖥️host/🧵️shard/🦀️component.rs`'s `ShardLoop::pump`. The Direct arm has no
    /// `RequestRegistry` counter to reuse (there is no registry in that world at all), so it mints
    /// `job` from its own dedicated counter instead — see `DIRECT_JOB_IDS` below.
    pub async fn spawn_job(&self, kind: impl Into<String>, input: Vec<u8>, placement: JobPlacement) -> Result<Vec<u8>, Fault> {
        let kind = kind.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::SpawnJob { job: req.0, kind, input, placement }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    let job = next_direct_job_id().await;
                    direct::host_async::spawn_job(job, kind, input, kernel_placement_to_direct_wit(placement)).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (kind, input, placement);
                    Err(direct_unavailable_fault("spawn-job").await)
                }
            }
        }
    }

    /// 🛑️ `job` must be one this SAME instance's own `host::jobs::spawn` call returned — cancelling
    /// an id minted by the `RequestRegistry`'s `req` counter (Poll world) or `DIRECT_JOB_IDS`
    /// (Direct world), exactly as `respond`/every other `req`-carrying effect already assumes about
    /// ids it did not itself allocate.
    pub async fn cancel_job(&self, job: u64) {
        self.emit(Effect::CancelJob { job }).await;
    }
    //#endregion 🔖️Timers

    //#region 🔖️Ui / Shell
    pub async fn open_window(&self, kind: impl Into<String>, params: DslValue) -> Result<Vec<u8>, Fault> {
        let kind = WindowKindId(kind.into());
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::OpenWindow { req, kind, params }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::open_window(direct::effects::OpenWindowParams { kind: kind.0, params: pack(&params) }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (kind, params);
                    Err(direct_unavailable_fault("open-window").await)
                }
            }
        }
    }

    pub async fn close_window(&self, window: WindowHandle) {
        self.emit(Effect::CloseWindow { window }).await;
    }

    pub async fn notify(&self, message: impl Into<String>) {
        self.emit(Effect::Notify { message: message.into() }).await;
    }

    pub async fn clipboard_write(&self, fragment: ClipboardFragment) {
        self.emit(Effect::ClipboardWrite { fragment }).await;
    }

    pub async fn navigate(&self, uri: impl Into<String>) {
        self.emit(Effect::Navigate { uri: uri.into() }).await;
    }

    pub async fn open_external_url(&self, url: impl Into<String>) {
        self.emit(Effect::OpenExternalUrl { url: url.into() }).await;
    }

    pub async fn set_panel(&self, panel_json: impl Into<String>) {
        self.emit(Effect::SetPanel { panel_json: panel_json.into() }).await;
    }

    pub async fn set_active_utility(&self, window_id: impl Into<String>, utility_id: impl Into<String>) {
        self.emit(Effect::SetActiveUtility { window_id: window_id.into(), utility_id: utility_id.into() }).await;
    }

    pub async fn set_active_tool(&self, tool_id: impl Into<String>) {
        self.emit(Effect::SetActiveTool { tool_id: tool_id.into() }).await;
    }

    pub async fn replay_shell_command(&self, action_id: impl Into<String>, args: Option<DslValue>) {
        self.emit(Effect::ReplayShellCommand { action_id: action_id.into(), args }).await;
    }

    pub async fn download_media_export(&self, filename: impl Into<String>, mime_type: impl Into<String>, data: impl Into<String>, encoding: Option<String>) {
        self.emit(Effect::DownloadMediaExport { filename: filename.into(), mime_type: mime_type.into(), data: data.into(), encoding }).await;
    }

    pub async fn icon_render_export(&self, items: Vec<IconRenderExportItem>) {
        self.emit(Effect::IconRenderExport { items }).await;
    }

    pub async fn request_file_open(&self, accept: impl Into<String>, read_as: Option<String>, import_action: impl Into<String>, multiple: bool) -> Result<Vec<u8>, Fault> {
        let accept = accept.into();
        let import_action = import_action.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::RequestFileOpen { req, accept, read_as, import_action, multiple }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::request_file_open(direct::effects::RequestFileOpenParams { accept, read_as, import_action, multiple }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (accept, read_as, import_action, multiple);
                    Err(direct_unavailable_fault("request-file-open").await)
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn request_media_frames(
        &self,
        accept: impl Into<String>,
        frame_action: impl Into<String>,
        done_action: impl Into<String>,
        fallback_action: impl Into<String>,
        sample_stride: u32,
        max_frames: u32,
        max_long_edge_px: u32,
        fps_hint: f64,
        payload: Option<String>,
        args: Option<DslValue>,
    ) -> Result<Vec<u8>, Fault> {
        let accept = accept.into();
        let frame_action = frame_action.into();
        let done_action = done_action.into();
        let fallback_action = fallback_action.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::RequestMediaFrames { req, accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::request_media_frames(direct::effects::RequestMediaFramesParams {
                        accept,
                        frame_action,
                        done_action,
                        fallback_action,
                        sample_stride,
                        max_frames,
                        max_long_edge_px,
                        fps_hint,
                        payload,
                        args: args.map(|value| pack(&value)),
                    })
                    .await
                    .map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args);
                    Err(direct_unavailable_fault("request-media-frames").await)
                }
            }
        }
    }

    pub async fn load_document(&self, pack: Vec<u8>, spr: Vec<u8>) {
        self.emit(Effect::LoadDocument { pack, spr }).await;
    }

    pub async fn spawn_plugin_instance(&self, plugin_id: impl Into<String>, app_id: impl Into<String>, os_instance_id: Option<String>, label: Option<String>, document_json: Option<String>) -> Result<Vec<u8>, Fault> {
        let plugin_id = plugin_id.into();
        let app_id = app_id.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::SpawnPluginInstance { req, plugin_id, app_id, os_instance_id, label, document_json }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::spawn_plugin_instance(direct::effects::SpawnPluginInstanceParams { plugin_id, app_id, os_instance_id, label, document_json }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (plugin_id, app_id, os_instance_id, label, document_json);
                    Err(direct_unavailable_fault("spawn-plugin-instance").await)
                }
            }
        }
    }

    pub async fn open_plugin_instance(&self, plugin_id: impl Into<String>, app_id: impl Into<String>, os_instance_id: Option<String>) {
        self.emit(Effect::OpenPluginInstance { plugin_id: plugin_id.into(), app_id: app_id.into(), os_instance_id }).await;
    }

    /// 🕹️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): NEW — `effects.wit`'s `dispatch-
    /// action-effect`/`host-async`'s `dispatch-action` import both existed before this packet, but
    /// no `Host` method ever built the effect (every existing caller of `Effect::DispatchAction`
    /// is host-side, replaying a shell action — see `⚛️reactor/🦀️component.rs`'s
    /// `kernel_effect_to_wit`). Added so the guest SDK can dispatch one too, matching the 24-import
    /// table this packet's report carries.
    pub async fn dispatch_action(&self, action: impl Into<String>, args: Option<DslValue>, delay_ms: u64) -> Result<Vec<u8>, Fault> {
        let action = action.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::DispatchAction { req, action, args, delay_ms }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::dispatch_action(direct::effects::DispatchActionParams { action, args: args.map(|value| pack(&value)), delay_ms }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (action, args, delay_ms);
                    Err(direct_unavailable_fault("dispatch-action").await)
                }
            }
        }
    }

    pub async fn open_dialog(&self, dialog_id: impl Into<String>, args: Option<DslValue>) -> Result<Vec<u8>, Fault> {
        let dialog_id = dialog_id.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::OpenDialog { req, dialog_id, args }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::open_dialog(direct::effects::OpenDialogParams { dialog_id, args: args.map(|value| pack(&value)) }).await.map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (dialog_id, args);
                    Err(direct_unavailable_fault("open-dialog").await)
                }
            }
        }
    }

    pub async fn request_sync(&self) {
        self.emit(Effect::RequestSync).await;
    }
    //#endregion 🔖️Ui

    //#region 🔖️Storage / Capabilities
    pub async fn storage_read(&self, key: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let key = key.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::StorageRead { req, key }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    match direct::host_async::storage_read(direct::effects::StorageReadParams { key }).await {
                        Ok(Some(bytes)) => Ok(bytes),
                        Ok(None) => Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("plugin.storage.not-found"), "storage-read: no value at this key")),
                        Err(bytes) => Err(dsl::decode_fault_bytes(&bytes)),
                    }
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = key;
                    Err(direct_unavailable_fault("storage-read").await)
                }
            }
        }
    }

    pub async fn storage_write(&self, key: impl Into<String>, bytes: Vec<u8>) -> Result<Vec<u8>, Fault> {
        let key = key.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::StorageWrite { req, key, bytes }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::storage_write(direct::effects::StorageWriteParams { key, value: bytes }).await.map(|_| Vec::new()).map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = (key, bytes);
                    Err(direct_unavailable_fault("storage-write").await)
                }
            }
        }
    }

    pub async fn storage_delete(&self, key: impl Into<String>) -> Result<Vec<u8>, Fault> {
        let key = key.into();
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::StorageDelete { req, key }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::storage_delete(direct::effects::StorageDeleteParams { key }).await.map(|_| Vec::new()).map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = key;
                    Err(direct_unavailable_fault("storage-delete").await)
                }
            }
        }
    }

    pub async fn request_capability(&self, capability: semio_framework::kernel::CapabilityRequest) -> Result<Vec<u8>, Fault> {
        match &self.backend {
            HostBackend::Poll(registry) => registry.request(move |req| Effect::RequestCapability { req, capability }).await,
            #[cfg(feature = "component-guest-async")]
            HostBackend::Direct => {
                #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
                {
                    direct::host_async::request_capability(direct::effects::RequestCapabilityParams { id: capability.id.0, scope: capability.scope, reason: capability.reason, optional: capability.optional })
                        .await
                        .map_err(|bytes| dsl::decode_fault_bytes(&bytes))
                }
                #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
                {
                    let _ = capability;
                    Err(direct_unavailable_fault("request-capability").await)
                }
            }
        }
    }

    pub async fn release_capability(&self, id: CapabilityId) {
        self.emit(Effect::ReleaseCapability { id }).await;
    }
    //#endregion 🔖️Storage
}

/// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): `spawn_job`'s Direct-world job-id
/// counter. The Poll world reuses `RequestRegistry`'s own `req` counter (`job == req.0`, see
/// `Host::spawn_job`'s doc) because it already needs one anyway; the Direct world has no registry
/// at all, so `spawn-job`'s `job: u64` (the one host-async param NOT wrapped in a `*-params`
/// record and NOT correlated by the returned future — `⚛️reactor/💼️jobs`'s own `JOBS` table still
/// keys progress/cancel by this same id) gets its own dedicated monotonic source instead. Global,
/// not per-instance: `world actor-async` is one actor talking to one host `Store`, same "today: one
/// actor per app instance" granularity `RequestRegistry`'s own doc already assumes.
#[cfg(all(feature = "component-guest-async", target_arch = "wasm32", target_env = "p2"))]
static DIRECT_JOB_IDS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

#[cfg(all(feature = "component-guest-async", target_arch = "wasm32", target_env = "p2"))]
async fn next_direct_job_id() -> u64 {
    DIRECT_JOB_IDS.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// 🌊️ Drains a Direct-world `stream<u8>` body into one buffer for `Host::http_request`'s (non-
/// streaming) return shape — `http_request` predates `BodyReader` and its signature stays
/// `Result<Vec<u8>, Fault>` (a live cross-file contract this packet must not break), so its Direct
/// arm collects `http_fetch`'s real stream the same way its Poll arm has always gotten one
/// complete buffer back. `usize::MAX` cap: this call site has no instance quota in scope (unlike
/// `RequestRegistry::append_chunk`, which reads `QuotaSchema.message_bytes` from the reactor's own
/// per-instance table) — an actual cap belongs to whichever future packet gives `Host` an instance
/// handle to read a quota from; recorded as an honest gap, not silently unlimited.
#[cfg(all(feature = "component-guest-async", target_arch = "wasm32", target_env = "p2"))]
async fn collect_direct_body(body: wit_bindgen::StreamReader<u8>) -> Result<Vec<u8>, Fault> {
    BodyReader::direct(body).await.collect(usize::MAX).await
}

/// 🔀️ kernel `Effect` → the Direct world's WIT `effect`, for `Host::emit`'s Direct arm. Mirrors
/// `⚛️reactor/🦀️component.rs`'s `kernel_effect_to_wit` for the ~22 variants `Host` itself ever
/// constructs (every `self.emit(Effect::X { .. })` call site above) — NOT the full ~40-variant
/// `Effect` enum reactor's own conversion covers, since `Host` never builds the completable
/// variants (those go through their own dedicated `HostBackend::Direct` arm and a specific
/// `host-async` import instead of `emit`, see e.g. `blob_load`). A variant Host never constructs
/// hitting the fallback arm is a genuine bug in THIS file (a new `self.emit(..)` call site added
/// without a matching conversion arm here), not a WIT/schema gap — hence `unreachable!`, not a
/// silent default.
#[cfg(all(feature = "component-guest-async", target_arch = "wasm32", target_env = "p2"))]
fn kernel_effect_to_direct_wit(effect: Effect) -> direct::effects::Effect {
    use direct::effects as wit_effects;
    match effect {
        Effect::SendMessage { target, payload } => wit_effects::Effect::SendMessage(wit_effects::SendMessageEffect { target: kernel_endpoint_to_direct_wit(target), payload }),
        Effect::PublishEvent { topic, payload } => wit_effects::Effect::PublishEvent(wit_effects::PublishEventEffect { topic, payload }),
        Effect::Subscribe { topic } => wit_effects::Effect::Subscribe(wit_effects::SubscribeEffect { topic }),
        Effect::Unsubscribe { topic } => wit_effects::Effect::Unsubscribe(wit_effects::SubscribeEffect { topic }),
        Effect::Respond { req, result } => wit_effects::Effect::Respond(wit_effects::RespondEffect { req: req.0, outcome: kernel_outcome_to_direct_wit_respond(result) }),
        Effect::SetTimer { id, after_ms, repeat } => wit_effects::Effect::SetTimer(wit_effects::SetTimerEffect { id, after_ms: after_ms as u32, repeat }),
        Effect::CancelJob { job } => wit_effects::Effect::CancelJob(wit_effects::CancelJobEffect { job }),
        Effect::CloseWindow { window } => wit_effects::Effect::CloseWindow(wit_effects::CloseWindowEffect { window: window.0 as u64 }),
        Effect::Notify { message } => wit_effects::Effect::Notify(wit_effects::NotifyEffect { message }),
        Effect::ClipboardWrite { fragment } => wit_effects::Effect::ClipboardWrite(wit_effects::ClipboardWriteEffect { fragment: pack(&fragment) }),
        Effect::Navigate { uri } => wit_effects::Effect::Navigate(wit_effects::NavigateEffect { uri }),
        Effect::OpenExternalUrl { url } => wit_effects::Effect::OpenExternalUrl(wit_effects::OpenExternalUrlEffect { url }),
        Effect::SetPanel { panel_json } => wit_effects::Effect::SetPanel(wit_effects::SetPanelEffect { panel_json }),
        Effect::SetActiveUtility { window_id, utility_id } => wit_effects::Effect::SetActiveUtility(wit_effects::SetActiveUtilityEffect { window_id, utility_id }),
        Effect::SetActiveTool { tool_id } => wit_effects::Effect::SetActiveTool(wit_effects::SetActiveToolEffect { tool_id }),
        Effect::ReplayShellCommand { action_id, args } => wit_effects::Effect::ReplayShellCommand(wit_effects::ReplayShellCommandEffect { action_id, args: args.map(|value| pack(&value)) }),
        Effect::DownloadMediaExport { filename, mime_type, data, encoding } => wit_effects::Effect::DownloadMediaExport(wit_effects::DownloadMediaExportEffect { filename, mime_type, data, encoding }),
        Effect::IconRenderExport { items } => wit_effects::Effect::IconRenderExport(wit_effects::IconRenderExportEffect { items: pack(&items) }),
        Effect::LoadDocument { pack: doc_pack, spr } => wit_effects::Effect::LoadDocument(wit_effects::LoadDocumentEffect { doc_pack, spr }),
        Effect::OpenPluginInstance { plugin_id, app_id, os_instance_id } => wit_effects::Effect::OpenPluginInstance(wit_effects::OpenPluginInstanceEffect { plugin_id, app_id, os_instance_id }),
        Effect::RequestSync => wit_effects::Effect::RequestSync,
        Effect::ReleaseCapability { id } => wit_effects::Effect::ReleaseCapability(wit_effects::ReleaseCapabilityEffect { id: id.0 }),
        other => unreachable!(
            "Host::emit was called with an effect variant it never constructs itself: {other:?} — every completable/streaming variant has its own dedicated HostBackend::Direct arm and host-async import instead of going through emit; this fallback firing means a new self.emit(..) call site was added above without a matching arm here"
        ),
    }
}

#[cfg(all(feature = "component-guest-async", target_arch = "wasm32", target_env = "p2"))]
fn kernel_endpoint_to_direct_wit(endpoint: semio_framework::kernel::MessageEndpoint) -> direct::types::MessageEndpoint {
    use direct::types::MessageEndpoint as W;
    use semio_framework::kernel::MessageEndpoint as K;
    match endpoint {
        K::Shell { instance } => W::Shell(instance.0.parse().unwrap_or(0)),
        K::Backbone { uri } => W::Backbone(uri),
        K::PluginInstance { id } => W::PluginInstance(id.0.parse().unwrap_or(0)),
        K::Extension { id } => W::Extension(id),
        K::Topic { name } => W::Topic(name),
    }
}

#[cfg(all(feature = "component-guest-async", target_arch = "wasm32", target_env = "p2"))]
fn kernel_outcome_to_direct_wit_respond(result: RequestOutcome) -> direct::effects::RespondResult {
    match result {
        RequestOutcome::Ok(bytes) => direct::effects::RespondResult::Ok(bytes),
        RequestOutcome::Err(bytes) => direct::effects::RespondResult::Fault(bytes),
    }
}

/// 📝️ Synchronous — wraps the `pure` WIT import `log`. Native/test builds (no `component-guest`
/// wasm32-wasip2 target) fall back to `eprintln!`, mirroring `host_port::host_now_ms`'s own
/// fallback shape.
pub async fn log(level: &str, message: &str) {
    #[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
    {
        crate::component::component::semio::framework::pure::log(level, message);
        return;
    }
    #[cfg(not(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2")))]
    eprintln!("[{level}] {message}");
}

/// ⏱️ Synchronous — wraps the `pure` WIT import `now-ms`.
pub async fn now_ms() -> i64 {
    #[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
    {
        return crate::component::component::semio::framework::pure::now_ms();
    }
    #[cfg(not(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2")))]
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis() as i64).unwrap_or(0)
}

/// 📏️ Synchronous — wraps the `pure` WIT import `trace-span`.
pub async fn trace_span(name: &str) {
    #[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
    {
        crate::component::component::semio::framework::pure::trace_span(name);
        return;
    }
    #[cfg(not(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2")))]
    let _ = name;
}
