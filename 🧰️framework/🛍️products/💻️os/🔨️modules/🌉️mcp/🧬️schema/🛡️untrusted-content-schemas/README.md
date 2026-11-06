# Design Notes

## Preserved Source Rationale

<a id="source-notes-5"></a>

### 🦀️.rs:5

#region 🔖️ToolSchemas
#region 🔖️UntrustedContent
🧷️ `schema` of the one envelope document-authored content reaches an agent in. A shared
document is written by every collaborator of its space (other people, other agents), so its
bytes are data an agent reads, never instructions it follows: every tool result and resource that
forwards them carries them ONLY under an `untrusted` field of this shape, next to where they came
from. Audit `📓️audit-s12-ai-mcp.md` §3.3 (G12-P1-2).
