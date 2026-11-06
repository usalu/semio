# Design Notes

## Preserved Source Rationale

<a id="source-notes-7"></a>

### 🦀️.rs:7

#region 🔗️ExtensionActivation
📌️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): [`Self::activate`],
widened for extension activation — pins to an EXACT `shard` via [`ShardTable::pin_to`] instead
of the least-loaded heuristic (so an extension always lands on its parent's shard — see
module design doc M6), and computes this actor's initial `capabilities` via
[`intersect_capabilities`] against `parent`'s own already-granted set when `parent` is
`Some` — the never-escalate-past-the-parent security property. `parent` is `None` for a
top-level activation with a caller-supplied grant set (e.g. a `PluginApp` whose capabilities
the host's own broker already resolved); passing `Some` is what makes this the extension path.
This method does **not** itself record the [`Self::link_extension`] parent→child edge — call
both from the host cascade so the two concerns (placement/capability vs. cascade topology)
stay independently testable.

<a id="source-notes-87"></a>

### 🦀️.rs:87

🌳️ Iterative (never self-recursive — R10's "self-recursive async fn" residue shape applies to
`async fn`s exactly as much as sync ones, and this crate's own rule bans the `Box::pin` escape
hatch where a loop will do) post-order walk of `root`'s cascade subtree: every descendant
appears strictly before its own parent, terminating with `root` itself last — i.e. leaves-first.
Shared by every cascading lifecycle op below; [`Self::resume_cascade`] reverses this order to
get the symmetric parent-first restore direction the design calls for.

<a id="source-notes-109"></a>

### 🦀️.rs:109

✂️ Shared removal primitive behind [`Self::deactivate`] and [`Self::kill`] — full teardown of
`root`'s ENTIRE cascade subtree, leaves-first: `Scheduler::unregister_actor` (drops the DRR
entry + mailbox), `ShardTable::unpin` (frees the shard slot/any exclusive lease), removes the
`ActorMeta` and this actor's own outgoing link-table row. Also scrubs every removed id out of
every OTHER actor's children list, so a subtree removal can never leave a dangling edge pointing
at an id [`Self::actor_record`] can no longer resolve. Returns the removed ids leaves-first —
the caller's evidence that the cascade actually walked the whole subtree, not just `root`.

<a id="source-notes-141"></a>

### 🦀️.rs:141

🔪️ The failure ladder's cascade teardown — same leaves-first, zero-orphan removal as
[`Self::deactivate`] (this crate's pure kernel state has no separate "abrupt vs. graceful"
axis: both fully retire the subtree; a host distinguishes them only by WHEN it calls each —
`deactivate` for user/uninstall-driven teardown, `kill` for the failure ladder giving up on an
unrecoverable actor). Kept as its own named entry point rather than an alias so call sites read
as intent, and so a future host-side distinction (e.g. a forced `ShardTransport::kill` versus a
graceful drain) has an unambiguous kernel-level hook to attach to. "Parent kill takes its
extensions down" (design doc M6's acceptance wording) is exactly this method called on the
parent.

<a id="source-notes-157"></a>

### 🦀️.rs:157

💤️ Cascading [`Self::suspend`], leaves-first: every descendant is suspended before `root`
itself. This is ALSO this crate's "checkpoint cascade" — [`ActorStatus::Suspended`] already
carries an optional checkpoint payload, so "checkpoint" and "suspend-with-bytes" are the same
kernel operation; only `root` receives the caller's `checkpoint` bytes (a per-descendant
checkpoint is a host/guest-runtime concern — the pure kernel only ever records the bytes it is
handed, never produces them). Descendants suspend with `checkpoint: None`.
