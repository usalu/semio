# Design Notes

## Preserved Source Rationale

<a id="source-notes-139"></a>

### 🦀️.rs:139

⏳️ Parks the calling (tool) thread until the refresh in flight settles or `wait_ms` passes — or, while the refresh
waits for its network byte budget to refill, until [`HUB_NETWORK_BUDGET_SETTLE_WAIT_MS`] passes, so a call spans the
refill instead of refusing inside it. A call that carries a progress token hears each second of the wait as
`notifications/progress` naming what it waits for. It decides nothing: the caller's own gate still fails closed if
the authority is not ready. Only request threads call it; the binding actor that performs the refresh never does.

<a id="source-notes-210"></a>

### 🦀️.rs:210

🔎 Fetches every selected document's manifest and verifies the protected descriptor body it names. No repository
path participates: manifest identity, bytes and descriptor contents all come from authenticated execution-target
routes and are fenced by the current authority generation. A descriptor body is fetched only the first time its
SHA-256 is named (see [`HubDescriptorCache`]): a space of 63 documents over 33 packages moved 16.7 MB of descriptors
per refresh — every directory event triggers one — and spent the gateway's whole byte budget within a minute.

<a id="source-notes-438"></a>

### 🦀️.rs:438

📚️ The space's administration projection across every bounded window. The hub bounds each window to
`DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_ROWS` rows and a cursor advances only the window it was issued for, so the
documents are followed to their last page, then the members until the authenticated principal's own row, every
page bound to the first page's authorization generation and at most [`HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS`]
documents / [`HUB_SPACE_MEMBERS_MAX`] members. Reading only the first page refused every space past 64 documents
(live hub coverage on 7800/p33, 66 documents: every `artifact_open` answered "descriptor index is refreshing").
