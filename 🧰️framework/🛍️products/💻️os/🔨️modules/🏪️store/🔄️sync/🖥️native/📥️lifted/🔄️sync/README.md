
## 📜️ Owned append-only folder event log. Documents are written as indivisible

`(schema, pack, spr)` snapshot events; blobs are content-addressed put/delete events. Reads fold
the log deterministically, so persistence follows the repo's event-sourced model without a CRUD
database. Every record is length-delimited and checksummed; an incomplete final record from a
process interruption is ignored, while corruption in a complete record is reported.

## 🗃️ Textual persistence for one folder of documents: `<id>.<ext>` holds the DSL text (initial

snapshot), `<id>.<ext>.ops` holds the append-only op log (see `crate::os_store::text_format::print_document_text`/
`crate::os_store::text_format::parse_document_text`). No `Backbone` impl: like `FolderEventLogStorage` above, this actor
layer drives it from its own thread; this crate only owns the file format. Additive alongside the
sqlite storage today — a technology adopts it by implementing `ArtifactDsl`/`OpText` and having
its sync endpoint construct one of these instead; nothing currently reads or writes through it
automatically.

## 📖️ pack+spr-first read: reads the AUTHORITATIVE pair for `document_id`, or `None` if

the `.pack` file itself doesn't exist (unlike `read`, the DSL mirror's existence alone
doesn't count — pack+spr are authoritative per the disk-layout LAW, the DSL file is
import-only). A present `.pack` with a missing `.spr` is a hard error — no legacy: they are
always written together (see `write_pack`), so a missing `.spr` means corruption or a
manual edit, never a valid state to silently recover from.

## ✍️ Overwrites all four files: the AUTHORITATIVE `.pack` + `.spr` pair, the shared

`.ops` text mirror, and the always-written DSL mirror `dsl_mirror` (`print_dsl` on the
initial snapshot) — the pack-aware sibling of `write`.
