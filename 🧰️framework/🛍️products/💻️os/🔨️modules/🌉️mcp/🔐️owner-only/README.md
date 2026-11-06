# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🔐️ Owner-only files for the semio MCP (bridge offers, delegated agent credentials) on every platform. POSIX gets an
exact `0600`; Windows ignores modes, so the file's DACL is replaced with one entry for the current account (`icacls`)
and read back as SDDL (`Get-Acl`) to prove no other principal kept access. The TypeScript twin is
`📚️library/🏃️process/🔐️owner-only/🟦️.ts`; both are pinned by `📚️library/🧫️fixtures/🔐️owner-only/🔣️.json`.
<https://learn.microsoft.com/windows-server/administration/windows-commands/icacls>
<https://learn.microsoft.com/windows/win32/secauthz/security-descriptor-string-format>
