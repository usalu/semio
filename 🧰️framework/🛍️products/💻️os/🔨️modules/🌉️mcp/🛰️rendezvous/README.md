# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🛰️ The local bridge rendezvous — how a `semio-os-mcp` process launched over **stdio** (the one
shape every client `.mcp.json`/`mcp.json`/`config.toml` in this repo actually launches) and a
**live os session** find each other, without either one being told the other's address out of
band.

Topology is unchanged from `run_http`: the gateway is the `/bridge` WebSocket SERVER and a shell
DIALS it (`🧵️bridge`, `🚚️transport`). What stdio mode lacked was (a) a listener at all and (b)
anywhere to publish its address. Both live here:

- `~/.semio/agent/bridge/sessions/<pid>.json` — one record per **live os session**, written by the
  session itself. Its presence is the only "a `dev s` is running" signal this crate trusts; a
  record whose pid is gone is stale and is swept on the next read, never believed.
- `~/.semio/agent/bridge/offers/<pid>.json` — one record per **live gateway**, written here,
  carrying the loopback `ws://…/bridge` url and the single-process admission proof a dialer must
  present as the second websocket subprotocol. Mode `0600` (owner-only) on unix, in the user's own
  home: the same trust boundary the agent audit lane (`📒️audit`) already stands on, and strictly
  stronger than an argv/environment carrier, which `🏗️bootstrap`'s process-entry seal forbids
  outright. The offer is removed when the gateway exits.

The admission proof is minted per process and never derived from the hub credential: a stdio
gateway has no fd-3 credential to delegate, and inventing a path that would copy one into a file
is exactly what the credential seal exists to prevent.

<a id="source-notes-32"></a>

### 🦀️.rs:32

🏷️ The carrier that points one gateway and one os session at a rendezvous of their own instead of
the per-user default. It is a **directory path, never a credential** (the admission proof stays in
the owner-only offer file), and it is spelled with the `S_` prefix the process-entry seal admits.

Why it exists: the default rendezvous is per-user, and `newestLiveAgentBridgeOffer` hands a shell
whichever gateway published last. With several gateways and several dev sessions alive at once —
an agent fleet, or one developer running two products — a shell dials a gateway that is not the
one its own agent launched. Pointing both halves of one pair at one directory is the only way to
make the attachment deterministic.
