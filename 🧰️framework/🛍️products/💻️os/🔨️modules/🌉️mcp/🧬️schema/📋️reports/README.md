# Design Notes

## Preserved Source Rationale

<a id="source-notes-224"></a>

### 🦀️.rs:224

🔀️ Which document owner this session's artifact verbs execute against — `"shell"` when a
live shell is attached over `/bridge` and declared `relayAppCommands` (the agent's edits
happen in the window the human is looking at, land in that human's undo history and
replicate to their collaborators), `"headless"` when this process's own `--folder`/`--hub`
workspace owns them. The decision is taken HERE, once per session, and is sticky: a client
that reads `"shell"` is never silently re-routed onto a different document.
