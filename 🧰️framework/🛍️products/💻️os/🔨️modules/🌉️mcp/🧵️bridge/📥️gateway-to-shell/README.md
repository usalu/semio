# Design Notes

## Preserved Source Rationale

<a id="source-notes-34"></a>

### 🦀️.rs:34

💬️ One chunk of the connected agent's OWN free-text turn — the channel that lets an agent
explain, ask a clarifying question, or answer at all, rather than only act. `reply_id`
identifies the turn (every chunk of one turn repeats it, so a shell appends instead of
stacking rows); `in_reply_to` is the `message_id` of the `ShellToGateway::AgentMessage` this
answers, or `None` for a turn the agent opened itself; `complete` marks the last chunk, so a
shell can show a turn as still arriving without inventing a heuristic. The text carries no
locale: it is the agent's own words, and the only translated strings on the surface are the
role nouns around them.
