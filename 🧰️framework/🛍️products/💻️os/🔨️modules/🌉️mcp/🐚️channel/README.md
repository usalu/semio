# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🐚️ The LIVE-SHELL artifact route — the `ArtifactChannel` that executes the mutation protocol
**in the user's running shell** instead of in this process's own headless interpreter.

`📓️r2-reactor-retained-command-owner.md` §12 measured the gap this module closes: only
`ui_reveal`/`ui_focus` were ever shell-bound, so `artifact_open/create`, `action_prepare/invoke`,
`artifact_snapshot`, `history_undo/redo`, `transaction_*` and `artifact_export` all resolved
through `🏠️workspace::PluginArtifactChannel` → the headless `OwnedRuntime` interpreter, against
the gateway's OWN `--folder` workspace. That is a different document owner from the shell the
human is looking at: the agent's edit never appeared on screen, never entered the human's undo
history, and never replicated to a collaborator.

The route is one more implementation of the SAME narrow port (`crate::actions::ArtifactChannel`,
one `exchange(instance, commands) -> Vec<AppFrame>`), so every tool above it — the whole
`🗿️artifact`/`🔀️dispatch` stack, its handles, its typed results, its approval gate — is unchanged.
What changes is who executes: [`ShellArtifactChannel`] encodes each `AppCommand` with this
module's own versioned JSON codec, sends it as a `GatewayToShell::AppCommand` frame carrying a
correlation `seq`, and blocks (bounded, never forever) on the `ShellToGateway::AppFrames` whose
`in_reply_to` equals that `seq`. The shell answers by dispatching through the very action lane
its own UI uses, so the human watches the edit happen and can undo it.

**Selection is per session and is reported.** [`SessionChannelBinding`] decides ONCE — at
`context_resolve` time, or lazily on the first exchange if a client never resolves — between
[`ChannelKind::Shell`] (a live shell connection that declared `BridgeFlags::relay_app_commands`)
and [`ChannelKind::Headless`] (everything else), and `ContextSummary.channel` reports the answer
so a client can never be wrong about which document owner it is talking to. The headless channel
remains the path whenever no shell is attached — it is a fallback, not a deprecated lane.

<a id="source-notes-94"></a>

### 🦀️.rs:94

📤️ Encodes one [`AppCommand`] as this codec's JSON payload. Field names are camelCase and the
`kind` discriminator is the variant's own name — the shared fixtures
(`🐚️channel/🧫️fixtures/🗿️app-commands.json`) are the SSOT both banks are tested against.

💡️ An inference never travels the shell route: `ensure_inference_route` owns its own guest
and the shell has no seam to drive one. `ShellArtifactChannel::exchange` refuses it before
encoding, so this arm exists only to keep the match total.

<a id="source-notes-284"></a>

### 🦀️.rs:284

🐚️ The `ArtifactChannel` whose other end is the human's shell.

`instance` is the adapter's own opaque handle; the shell addresses its plugin instances by
string id (`BridgeInstanceRef.instance_id`). The mapping is resolved from the shell's own
`Instances` frame — by the owning plugin of the command's capability when the command carries
one, and otherwise by a sticky binding this channel remembers per `instance`, so every leg of a
prepare/commit/undo saga lands on the SAME live instance that ran the first leg.

<a id="source-notes-295"></a>

### 🦀️.rs:295

🎯️ The plugin this channel's artifact belongs to, when the caller already knows it — the
artifact's own kind names its owner, and `artifact_create`/`artifact_open`/`artifact_snapshot`
all resolve that owner before they open a channel. Without it a capability-less command
(`ReadArtifact` carries no capability at all) had nothing to resolve an instance from and
refused outright as soon as the shell had more than one program open — measured inside `s`
with a spawned editor: `the attached shell has 3 open instances` (ticket 26/09/18 S6 §5.4).
