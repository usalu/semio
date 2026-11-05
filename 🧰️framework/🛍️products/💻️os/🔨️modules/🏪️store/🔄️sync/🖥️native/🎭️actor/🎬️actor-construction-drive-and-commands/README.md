
## 🪢️ Starts this hub document at the canonical checkpoint pair its shell verified and already loaded into the

guest: the pair's history is known (a tail never re-applies it), a folder binding persists it, and the first
`SocketHelloV1` names its baseline, so the hub sends only what followed it. A pair whose SPR cannot be read
is reported as a conflict and the document starts unseeded.

## 🛟️ Restarts a rebootstrapping hub document at the pair the host verified and loaded into the guest

([`ArtifactActorMsg::Reseed`]): the pair becomes the baseline, the unacked local operations the pair does not already
hold go back to the guest, and the actor dials at once — no outage backoff, Hello at the baseline.

## 🏃️ Advances exactly one command, readiness source, timer, backbone owner, or status turn.

It idles only after observing an empty mailbox and an empty store outbound in the same turn:
a command that relayed one backbone owner consumes that owner's wake, so a second queued
owner has no wake left (ticket 26/09/23 `📓️wp-h6.md`).
