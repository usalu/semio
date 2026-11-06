# Design Notes

## Preserved Source Rationale

<a id="source-notes-41"></a>

### 🦀️.rs:41

🎬️ The budget a COLD OPEN turn runs under. A ceiling, not a slice: under the compiled runtime a
turn is atomic — an epoch deadline cut leaves the component instance mid-call and every later
call answers `cannot enter component instance` — so this number is the point at which the guest
is declared wedged and its instance is thrown away, never a quantum the host resumes from. 60 s
is far above what the os dev host needs to boot the same components (~20 plugins in seconds) and
far below the MCP client's own 240 s per-call budget, so a wedged open still answers the agent.

⛽️ `fuel` is deliberately disarmed. Under the JIT a fuel yield is exactly as unresumable as an
epoch cut, so arming both would only add a second, harder-to-read way to say "wedged"; the time
ceiling is the one an agent and an operator can both reason about. `🔬️owned-instance-open`'s
`wasmtime_runtime_instance_open_settles_against_a_real_plugin_component` opens a real component
under exactly this shape.
