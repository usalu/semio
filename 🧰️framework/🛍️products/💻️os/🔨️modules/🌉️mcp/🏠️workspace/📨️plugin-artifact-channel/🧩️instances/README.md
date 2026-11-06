# Design Notes

## Preserved Source Rationale

<a id="source-notes-109"></a>

### 🦀️.rs:109

🔁️ Drives one `AppCommand` to its terminal frame. The command becomes a retained host batch
whose exact current page is handed to one guest turn; the next page stays untouched until the
guest acknowledges the one before it, so a command of N pages is N+1 turns. An interactive
host takes those turns off its own frame loop; a headless gateway has no next frame, so it
takes them here, bounded by [`COMMAND_RESUME_WALL_BUDGET`] and never unbounded.

🔢️ ONE sequence for the whole command, stamped into the command itself before it is
encoded. The kernel envelope and the `AppCommand` must carry the same number: the guest
answers with an `AppFrame` whose `in_reply_to` is the COMMAND's `seq`. Every call site in
this file used to pass a literal `seq: 0` while the envelope counted from 1, so even a host
reading the right lane would have correlated nothing.
