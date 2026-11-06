# Design Notes

## Preserved Source Rationale

<a id="source-notes-281"></a>

### 🦀️.rs:281

🧩️ The plugin COMPONENT the Hub authorized for `scope`, bit-for-bit the one its lease names — exact byte length and
exact SHA-256, compared before the bytes reach any runtime, because a component the Hub serves is executable code.
This is the leg of the browser shell's own attach chain a native client never had: `open-plan` →
`execution-target/{manifest, component, descriptor}`; it comes from the per-user store when that holds the bytes, else
from the hub as a static asset ([`HubComponentAssets`]), and the calling tool hears the transfer as progress.

<a id="source-notes-309"></a>

### 🦀️.rs:309

🪪️ Fetches the execution-target LEASE the Hub minted for one exact document — the small JSON
projection (≈3 KB, measured on hub 7621) that carries what the package-keyed catalog snapshot
deliberately does not: this document's own `surface` (`surfaceId`/`appId`/`windowKindId`),
its `artifact` kind and schema, and the grant behind them.

🧭️ `refresh_catalog` deduplicates its selections by PACKAGE identity, so its `scope` names
whichever document first resolved that package — never necessarily the one a caller is
opening. A per-document lease is therefore the only honest source for a per-document surface,
and it is fetched here rather than derived: a surface id assembled host-side from a manifest
would be this gateway's guess at what the hub authorized, which is exactly the class of
fabrication `PROBE_SURFACE_ID` already was.
