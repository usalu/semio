# Design Notes

## Preserved Source Rationale

<a id="source-notes-12"></a>

### 🦀️.rs:12

🗂️ Builds the live gateway's `CatalogSource` — `gateway` is always `crate::core_tool_capabilities()`
(Requirement 2: a production server with zero plugins installed still advertises its own gateway
tools, honestly, never the note/cad fixture); `descriptors` is whatever real discovery found under
`root`, or under `find_repo_root()` when `root` is `None` (the `--folder`-bound caller passes its
own root; the bare/no-argument caller lets this resolve the repo/space root itself). Root
resolution failing, or the resolved root having no registry, both degrade to an empty descriptor
list — never a fallback to fixture data, never a panic.
