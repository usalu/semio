# Design Notes

## Preserved Source Rationale

<a id="source-notes-31"></a>

### 🦀️.rs:31

🌎️ What `artifact_open` binding a HUB document reports back: which plugin's guest now holds this
document as its session document, on which authorized surface, and whether a mutation committed
against it can actually leave the process. `null` for a folder artifact and for a hub document
the hub authorizes no execution target for — neither of which has a hub surface at all.

🧭️ `writePath` is a FACT the agent is owed before it spends a mutation: `open` means the
document's own actor and socket are up, and any other value is the reason they are not, in the
words of whatever refused. An agent that reads a closed write path knows its edit would commit in
a guest and reach nobody.

<a id="source-notes-121"></a>

### 🦀️.rs:121

🧊️ Both halves of the document, because one of them can never move. A `.spk` pack is
`envelope.vcs.initial_snapshot.encode_pack()` — the GENESIS snapshot, fixed for the life of
the document — while every committed mutation lands in the `.spr` event log. Publishing
`packBase64` alone therefore handed an agent bytes that are byte-identical before and after
its own commit (measured 2026-09-22, slice CE3: `note.addBlock` moved spr 223 → 612 bytes
with the pack at 299 on both sides), so the shipped `mutate-safely` prompt's "re-read the
artifact and confirm the change landed" could not be carried out from this tool's reply.
Both halves are document-authored, so they travel only inside `untrusted`; the sizes beside it
are the gateway's own measurement.
