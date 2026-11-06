# Design Notes

## Preserved Source Rationale

<a id="source-notes-22"></a>

### 🦀️.rs:22

📻️ The one duplex line channel a stdio connection owns. Both the serve loop AND a server-initiated
request made from INSIDE a tool call (`elicitation/create`) read and write through this single
owner, so exactly one component ever touches the real descriptors. A line the elicitation wait
reads that is not its own response is pushed onto `deferred`, and the serve loop drains that queue
before touching the stream again — the client's own ordering is preserved and nothing is dropped.

⏱️ The client stream is drained by ONE owned reader thread into a channel rather than read
in place. That is what makes a wait on it deadline-bounded: `BufRead::read_line` blocks with no
timeout of any kind, so an elicitation waiting directly on the descriptor could never give up on
a client that goes silent (`📓️m4-mcp-bridge-approval-binding.md` §5.3). Every read still yields
lines in exact arrival order, and the thread ends on EOF, on an io error, or when the last
`StdioLines` is dropped. The one exception is `notifications/cancelled`, which this thread applies
immediately ([`crate::protocol::intercept_cancellation`]) so it interrupts the call still running.

<a id="source-notes-207"></a>

### 🦀️.rs:207

🙋 Issues one `elicitation/create` and waits on THIS connection for the client's answer.
Every other line that arrives meanwhile is deferred back onto the serve loop in arrival order,
so a client that keeps sending requests while a human decides loses nothing. The wait ends on
the matching response, on EOF, or on {@link ELICITATION_TIMEOUT_MS} of wall clock — a client
whose human walks away closes this lane exactly as a silent OS shell closes the bridge lane,
and the approval chain moves on instead of wedging the agent forever.
