
## 🧭️ `plugin_id` for every catalog entry `CapabilityOwner::Plugin{..}` names, deduplicated and

sorted — doubles as a stable, catalog-derived NUMBERING (`plugin_instance_slot`/
`plugin_for_instance_slot` below), not just a set: `ActionAdapter::prepare` issues a bare
`AppCommand::ReadHistory` (no capability id at all) BEFORE the `PureCommand` that names one
(`🔀️dispatch/🦀️.rs`'s own `prepare`), so `RoutingArtifactChannel::exchange` cannot always
read a capability id off the wire — it needs `instance` itself to already encode the plugin, which
only works if every caller derives `instance` from the SAME deterministic function of the catalog.

## 🎯️ The `instance` slot any caller must pass to [`ActionAdapter::prepare`]/[`ActionAdapter::invoke`]

for `capability_id` — [`plugin_instance_slot`] of the plugin that capability's own catalog entry
names. Public because the `action_prepare`/`action_invoke` MCP tools drive the ROOT-owned
adapter directly (never [`HeadlessWorkspace::prepare_action`]) and so need the identical
derivation: with the bare `0` they used to pass, `prepare`'s leading capability-less
`ReadHistory` was routed by [`plugin_for_instance_slot`] to whichever plugin happens to sort
first in the catalog, not to the capability's own.

## 🔎️ `capability_id` → its owning plugin id — [`HeadlessWorkspace::resolve_plugin_for_capability`]'s

actual body, factored out so [`RoutingArtifactChannel`] (which holds only the `Arc<Catalog>` it
needs, not a whole `&HeadlessWorkspace`) can run the identical lookup for a `PureCommand` it is
asked to route, without a second copy of this match. An unknown id or one owned by anything other
than a plugin (`Os`/`Framework`/`🐚️Shell`/`Gateway`/`Extension`) is a typed error naming exactly
which of the two it was — never a guess.

## 🧩️ Where a workspace's plugin components come from. A `--folder` workspace builds them itself and

reads them off the repo's own `wasm32-wasip2` deliverable tree; a `--hub` workspace has NO repo —
its components are the execution targets the Hub authorized, served by the Hub, verified against
the Hub's own manifest. One enum rather than an `Option<PathBuf>` that silently means "no plugins
at all" for every hub-bound session, which is what made every `action_prepare` from an agent
answer `repo root not found` (M8 §5.3, measured live 2026-09-21).

## 🌎️ Resolves a plugin id to the component bytes and package descriptor the authenticated Hub

selected for one of the space's own documents. The descriptor comes from the catalog snapshot the
binding already verified (manifest identity, canonical projection, digest); only the component
bytes are fetched here, lazily — one plugin at a time, when an open or an action needs it — by the
content address the current catalog's lease names, from the persistent execution-target store or
the hub ([`remote::HubComponentAssets`]); a republished catalog names other bytes, never a stale hit.

## 🚦️ `crate::actions::ArtifactChannel` implementor that picks the plugin from the CALL instead of

pinning one for the whole process — replaces the deleted `resolve_default_plugin_id`'s "exactly
one plugin or bust" (`📓️w8-capability-routing.md`). A `PureCommand{capability_id,..}` names its
plugin directly (`resolve_plugin_for_capability_in`); every other `AppCommand` variant carries no
capability id, so it decodes the SAME plugin from `instance` via `plugin_for_instance_slot` —
sound only because `HeadlessWorkspace::prepare_action` (the one caller that mints a FRESH
`instance`) derives it from `plugin_instance_slot(capability's owner)` up front, never a bare `0`.
Opens (and caches) at most one real `PluginArtifactChannel` per [`AppRoute`] — never all up
front, never twice for the same app; the cache lock is held only across one `HashMap`
lookup/insert, never across a channel's own `exchange` (no lock-ordering hazard against
`open_probes`/`action_adapter`'s mutexes, which this struct never touches).

## 🧵️ Hands `plugin_id`'s bound document actor every backbone message its guest just published.

`ArtifactActorMsg::DocumentBackbone` is the actor's own verified ingress — it decodes the
envelopes, refuses any that name another document, retains them and relays them to the hub as
`ClientFrame::Commands`. Nothing here interprets a byte of them.

🚧️ A binding with no document actor is a typed, named fault, never a silent drop: the guest
HAS committed by the time these bytes exist, so "the edit went nowhere" must be something the
agent is told. [`PluginArtifactBinding::backbone_blocked_by`] carries the reason.

🔒️ `session_artifact_for` takes the same lock, so it is called BEFORE this one is held —
`std::sync::Mutex` is not reentrant and nesting them deadlocked the very first committed
message (measured 2026-09-22, the test hung past its 60 s report threshold).

## 🗿️ Re-read on every exchange, never once at open: `artifact_create` binds the artifact

AFTER the channel that seeded it from the plugin's genesis was already opened, so a
stamp taken from an open-time snapshot would name nothing for the very first mutation.

📥️ …and the same re-read seeds the guest with the bound document itself when the binding
carries one (a hub document, whose bytes live on the hub and nowhere this guest can
reach). `load_session_document` is idempotent per instance, so a prepare/commit pair does
not reload between its two halves; an `Infer` skips it because the inference lane drives
its own guest on its own actor ordinal, which this document never belongs to.

🧵️ …then bind the guest's document backbone, in that order: the guest refuses a load
while bound. Until this runs the guest publishes no backbone effect at all, so a
committed transaction would answer `SUCCEEDED` and reach nobody.

🧵️ …and whatever the guest published on its document backbone during that exchange leaves
the process now, on the bound document's own actor mailbox. Drained on BOTH outcomes: a
command that faulted after its guest already committed still owes those envelopes to the
hub, and leaving them in the channel would hand them to the next, unrelated exchange.

## 🔀️ The channel a `--folder`/`--hub`-bound gateway that can ALSO reach a live shell runs on: one

value holding both routes, with [`crate::shell_channel::SessionChannelBinding`] deciding — once,
at `context_resolve` time — which of them a session executes through.

Both are built up front rather than lazily, because the decision must be observable before the
first mutation: `ContextSummary.channel` reports it, and a client that reads `"shell"` is
promised the shell for the whole session (a shell that then disconnects answers a typed,
retryable fault — never a silent fall back onto a different document owner mid-transaction).

## 💡️ An inference is not a shell command and never was: `ShellArtifactChannel` refuses

`AppCommand::Infer` outright ("the gateway owns its own inference guest"), and the
headless lane's `ensure_inference_route` drives its OWN guest instance on its own actor
ordinal, holding no document the shell owns. Sending it down the shell branch made
`inference_run` answer `plugin.unavailable` for the whole of a shell-resolved session —
i.e. exactly the session an MCP client driving a live shell has (measured 2026-09-22
22:1x, `🗑️generated/gj1-trap-probe.txt` first run). Routing it headless here is what the
refusal's own message asks a caller to do, done for them, and it splits no document:
the inference guest is not the shell's.

🎯️ Name the artifact's owner before dispatching, exactly as the headless lane
does: without it a capability-less leg of the mutation protocol (`ReadArtifact`,
`ReadHistory`) had nothing to resolve an instance from and refused as soon as the
shell had more than one program open.

## 🚧️ Why this binding opened no document actor, when it did not. A hub document whose artifact

schema has no registered `store::os_store::serialization::ArtifactCodec` in this process cannot open a document socket
at all: `connect_hub` reads `document_codec(schema).pack_schema_hash` to build its
`SocketHelloV1` and silently reschedules a reconnect forever when there is none. Recording
the reason here is what turns that silence into something a reader can see.


## Preserved Source Rationale

<a id="source-notes-674"></a>

### 🦀️.rs:674

🔀️ dedyn-fw-os-misc, O1/R11: closes `crate::actions::ArtifactChannel`'s implementor set
(`UnboundArtifactChannel` in `🔀️dispatch`, `PluginArtifactChannel`/`RoutingArtifactChannel` above)
— defined here (not in `🔀️dispatch`, where `ActionAdapter` actually stores it) because this is the
one module every implementor is jointly nameable from; `🔀️dispatch` imports `ArtifactChannels`
back in. Replaces `Box<dyn ArtifactChannel>`. The enum is spelled twice under complementary
`cfg`s because `dyn_enum_close!` carries no per-variant attributes: the scripted
`MockArtifactChannel` exists ONLY in a test build, so no production path can reach it
(`📓️g7-mcp-agent-and-collaboration-audit.md` §6 P0.3).
