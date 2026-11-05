
## 📁️ A folder/file binding's storage driver, keyed for multi-document sqlite or single

pack-backed blob. Both variants move real `pack`+`spr` bytes end to end — this actor never
touches JSON. `Sqlite` is fully codec-free (its row IS the pack+spr pair, no bridging
needed). `Pack` requires the schema codec for both authoritative DSL fallback validation
and synchronized text mirrors.

## 🎨️ This connection's hub-assigned session color (`ServerFrame::Session.color`) —

`None` until the hub sends it (or for a folder-only document, which never connects to a
hub). Stamped onto every outbound `PresenceHeartbeat` via {@link stamp_session}.
