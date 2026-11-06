# Design Notes

## Preserved Source Rationale

<a id="source-notes-552"></a>

### 🦀️.rs:552

⏳️ A joined reader waits for the receipt's owner to publish or fail — woken by the completion itself or by its own cancel,
never by a timer: the gateway drives mounts on the framework executor (`TokioHostRuntime::block_on`), which runs no Tokio
reactor, and the 10 ms Tokio sleep this loop used to poll with panicked there, so every second concurrent reader of a
loading document died "without answering" (live quartet on 7800/p33, ticket 26/09/23 G12 session 14c). The owner's own
deadline — the same operation timeout, started earlier — bounds the wait; every wake re-checks this reader's deadline.
