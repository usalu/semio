# Design Notes

## Preserved Source Rationale

<a id="source-notes-6"></a>

### 🦀️.rs:6

🌎️ Binds one HUB document to the plugin, app, artifact schema and SURFACE the hub's own
execution-target lease names for it, and holds the canonical pair the caller already read so
that plugin's guest can be seeded with THIS document instead of with its own genesis
(`RoutingArtifactChannel::exchange` → `PluginArtifactChannel::load_session_document`).

🐛️ Before this existed, `plugin_artifacts` was written by `create_plugin_artifact` alone — a
folder-only path — so a hub-bound agent that opened a real hub document had NO binding at
all: every stamp it answered named the pseudo-id `plugin:<id>`, and every mutation it prepared
ran against the plugin's genesis document rather than against the one it had open. That is
step 1 of the write path M8 §5.3 / M9 §8.1 name.

🪪️ Every field comes from an authenticated route, none is derived from the artifact id or
guessed from a single-plugin workspace: the lease is fetched for this exact scope and its
package identity is cross-checked against the descriptor snapshot the binding already
verified. `Ok(false)` means this workspace is not hub-bound or the hub authorizes no
execution target for the document — an unbound artifact stays readable, it simply has no
guest to seed.

🔁️ Re-opening a document this session already bound must not open a SECOND actor: the
first one holds the live socket, the presence lease and the outbox that still owes the hub
this agent's envelopes. Only the canonical pair is refreshed — unless its link EXPIRED (a shortage outlived its
bound), which a new open relinks with a fresh actor.

🔗️ Opening a new actor first closes the live links [`hub_links_to_close`] names — the app's older document
(the one opened now becomes the app's session document) and, past [`HUB_SESSION_LIVE_LINK_LIMIT`], the least
recently used — so one agent never holds more of the hub's document sockets than that (each is one of the hub's
bounded WAL writer slots; an agent that walked 63 documents took them all, measured on 7800/p33).

<a id="source-notes-84"></a>

### 🦀️.rs:84

🔗️ Closes the live hub links [`hub_links_to_close`] names before `opening` (a document of `route` = plugin, app)
links: each one's actor is closed (its socket, presence row and WAL writer slot released) and its binding keeps
the reason, which a later edit of it reports until `artifact_open` relinks it. Every edit the agent was told
succeeded is already acknowledged by the hub; an unacknowledged one was reverted and reported, so nothing an
agent believes the hub holds rides on a closed actor.

<a id="source-notes-121"></a>

### 🦀️.rs:121

🧵️ Opens the hub document's own `store::sync` actor — the one that holds the document socket
(`…/socket-grants` → `semio.socket.v1`), sends `ClientFrame::Commands` for every local
envelope, and beats presence for this session. Returns its mailbox, or the reason there is
none.

🪪️ The binding is `PersistenceBinding::Hub { surface: Some(<the LEASE's surface>) }`, not the
pinned `PROBE_SURFACE_ID` `WorkspaceOrigin::persistence_binding` mints for this crate's own
probe documents. A surface id is an authorization coordinate: `finish_connect_hub` compares
the socket authority's scope, schema and pack-schema hash against the client's own, and the
hub issues the authority for the surface the client asked for. Claiming the probe editor's
surface for a `gis` document was the pin M8 §5.3 and M9 §8.1 both name as the open unknown here
(`🏠️workspace/🦀️.rs:501`). The lease is handed to the host FIRST
(`set_document_execution_target_lease`), because `open` consumes it when it spawns the actor
and the actor sends it inside `DocumentSocketExpectationV1`.

🚧️ No registered `ArtifactCodec` for the document's artifact schema ⇒ no socket, and this
says so instead of opening an actor that would reconnect-loop in silence: `connect_hub`
needs `document_codec(schema).pack_schema_hash` to build `SocketHelloV1` at all. This
process links the three native codecs `📇️native-openable-provider` compiles in, so a
fourth package's kind — `gis.map` among them — has none. The root fix is to route the
registration through the component's own `codec` export
(`🔌️plugin/🧬️schema/📜️.wit`'s `interface codec`, TC3b); it is NOT done here and the
reason is reported rather than papered over.
🗂️ Registers the document kind's codec from the package the HUB authorized for it — the same
component bytes `action_prepare` executes, resolved through the same verified route by the same
content address the lease names, so the codec and the guest can never be two different builds
of one package.

🪪️ The hub's own declared `pack_schema_hash` is passed as a CROSS-CHECK and never as the
value — see [`register_guest_document_codec`] for why registering the hub's number would
delete the check that reads it back.

<a id="source-notes-205"></a>

### 🦀️.rs:205

🤖️ Makes this session a visible principal on the hub document it holds. The hub lists a
socket in the roster only once it has beaten, and keeps the row for as long as that socket
stays open (`expire_presence_for_live`), so the contract is one beat per hub connection: on
the connection's `Session` and on every `Live` status after it. The beat carries no identity
of its own — the hub stamps the delegation's label and the principal kind it authenticated
(`agent`), so the roster can never show this session as the human who delegated it. The task
ends when the document closes and drops its event channel.

🚦️ The same task records every sync status and coded fault the actor reports into `relay`, which
is how a commit learns whether the hub acknowledged it and `artifact_open` reports the link.

📥️ …and queues, in order, every baseline and mutation batch the actor delivers for the guest — the
bootstrap tail past the checkpoint and every other writer's edit — which the guest takes before its
next command ([`RoutingArtifactChannel::exchange`], [`plan_hub_inbound`]).
