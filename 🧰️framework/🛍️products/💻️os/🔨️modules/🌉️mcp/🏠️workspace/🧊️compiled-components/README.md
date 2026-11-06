# Design Notes

## Preserved Source Rationale

<a id="source-notes-72"></a>

### 🦀️.rs:72

🧩️ ONE compiled component per staged build, for the life of the process, keyed by the
component's own content hash so a rebuilt plugin is a different entry rather than a stale one.

🪦️ This is not only about not compiling twice. A `wasmtime::Component` owns a mapped code image
— 136 MB for an unoptimized `🖍️draw` build — and releasing the LAST `Arc` to one runs
`CodeMemory`'s `UnwindRegistration::drop`, i.e. a `__deregister_frame` walk over every frame in
that image. Measured 2026-09-20 on a calm machine: the open itself is 34 ms and the drop of the
component behind it pins a thread at 100 % for minutes, which is what made the gateway's first
plugin tool call answer nothing at all inside the MCP client's 240 s budget — the work was in
tearing the component DOWN after a successful call, not in bringing it up. Holding the handle
here keeps the refcount above zero, so a channel's own drop releases its `Store` and stops.

<a id="source-notes-99"></a>

### 🦀️.rs:99

🔏️ The content hash of every staged component file already read, by file identity — held in this
process and persisted beside the compiled-code cache, so a staged build is read and hashed once
per build rather than once per process.

⏱️ Every channel open used to read the whole component and hash it before it could ask the
compiled cache anything: measured 2026-09-24 (ticket 26/09/23 slice G6) the 125 MB `🀄️wfc`
build took 8.5–11.5 s to hash in a dev host, once for `artifact_create` and again for the first
`inference_run` binding of the same session, while the compiled-code lookup behind it took 0.3 s.

<a id="source-notes-161"></a>

### 🦀️.rs:161

🔏️ `bytes`' own 32-byte content digest, one chunk per progress report and cancel check.

🔏️ The component's OWN digest, not `hash_bytes`' 64-char HEX STRING: that string's `into_bytes()`
is 64 long, so `try_into::<[u8; 32]>()` always failed and every plugin in this process took a
`[0u8; 32]` fallback. `WasmtimeRuntime::compile` keys its on-disk `.cwasm` cache on it, so the
placeholder made every plugin collide on ONE cache entry and replay whichever component compiled
first. Measured 2026-09-20: a single 130 MB `0000…0000.cwasm`.

<a id="source-notes-186"></a>

### 🦀️.rs:186

🧩️ The process's compiled handle for one component, reporting each phase it actually runs to
`scope`. A staged file already hashed in this process is neither read nor hashed again; compiled
code already in this process or on disk is loaded, never compiled again, and the component is
read only when it must be compiled. A cold compile runs in an isolated worker process shared by
every requester of the same content hash ([`CompileFlight`]), so a cancel detaches this requester
at once and the last requester's cancel kills the compile.
