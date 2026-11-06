# Design Notes

## Preserved Source Rationale

<a id="source-notes-22"></a>

### 🦀️.rs:22

🎬️ `action.prepare`/`action.invoke`'s real 2-phase mutation protocol (revision checks, undo
tokens) already lives at `🔀️dispatch`'s `ActionAdapter` — and, per that packet's own doc,
`MockArtifactChannel`/`PluginArtifactChannel` are its only two real implementors, ONE of which
(`PluginArtifactChannel`) this file owns. Rather than half-reimplement `ActionAdapter`'s
prepare/preview/commit/audit bookkeeping a second time in this struct, these two methods
genuinely DELEGATE to a real, workspace-owned `ActionAdapter` (`Self::action_adapter`) — see
that method's own doc for why it is a SEPARATE instance from `🦀️.rs`'s root-owned
one rather than a shared reference. Every real 6-verb exchange this now drives is implemented
for real at `PluginArtifactChannel::exchange` above; whether it SUCCEEDS today depends on
that arm's own honest gap note (`PureCommand`'s guest-side handler unconditionally rejects
for every currently-compiled plugin) — this is a real, non-fabricated round trip either way,
never a synthesized `PLUGIN_UNAVAILABLE` short-circuit before even trying.

🧭️ Resolved BEFORE the adapter ever runs — `ActionAdapter::prepare` issues a bare
`ReadHistory` (no capability id) before its `PureCommand`, so `RoutingArtifactChannel` must
already be able to decode the target plugin from `instance` alone; `plugin_instance_slot`
is the SAME deterministic function it decodes with, so passing its result here (never the
old bare `0`) is what makes that first `ReadHistory` land on the right plugin.

🔀️ `ActionAdapter::prepare` has no `expected_revision` parameter of its own — it always
captures a fresh baseline (`report.expected_revision`) via a real `ReadHistory`. This
trait's own `expected_revision` is the CALLER's assertion of what revision it expected to
still be current; checking it here (rather than silently ignoring it) surfaces a real
`REVISION_CONFLICT` the moment a caller's assumption is already stale, instead of only at
`invoke_action` — matches `ActionAdapter::invoke_uncached`'s own revision-conflict shape.

<a id="source-notes-62"></a>

### 🦀️.rs:62

🧭️ The trailing `0` here is `ActionAdapter::invoke`'s `instance` PARAMETER, which
`invoke_uncached` only reads on its "prepare inline" branch (`request.capability_id` set,
no `prepared_handle`) — never this one, since `prepared_handle` is always `Some` below.
Every real command this call issues instead reuses `record.instance`, captured on the
resolved handle back when `prepare_action` minted it from `plugin_instance_slot` — so the
correct plugin routing already travels with the handle, not through this parameter.
