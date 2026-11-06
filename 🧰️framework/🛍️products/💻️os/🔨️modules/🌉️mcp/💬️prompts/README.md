# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

💬️ `prompts` — ticket 26/08/29/AI-MCP-END-TO-END packet W7: the `prompts/list`+`prompts/get`
surface, which shipped as an empty `InMemoryPromptRegistry` until now. Every prompt here is
**plugin-, app- and artifact-agnostic**: it teaches the agent this gateway's *protocol* — how to
discover capabilities, how to mutate safely through prepare→preview→commit, how to undo, how to
read an artifact together with its inferences — never a specific plugin's vocabulary. A newly
installed plugin changes nothing here, which is exactly the point: the prompts route the agent to
`capabilities_search`, and the catalog answers with whatever is really installed.

Bilingual by construction (`📓️CLAUDE.md`: multiple languages, no default, English first then
German). Every prompt takes an optional `locale` argument; an unknown or absent locale resolves to
English rather than failing, because a prompt is guidance, not policy.
