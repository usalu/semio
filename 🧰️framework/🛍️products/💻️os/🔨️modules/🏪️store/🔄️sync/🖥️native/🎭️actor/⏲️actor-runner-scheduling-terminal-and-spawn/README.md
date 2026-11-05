
## 📬️ A mailbox, readiness or close wake targets whichever turn is current: it carries no

generation, so a turn that completes between the sender's read and its request cannot
turn it into a stale wake (the actor then idled with a queued command, ticket 26/09/23
`📓️wp-h6.md`).

## 🏃️ TurnFault retains the future in `terminal_turn`. Do not `enqueue` a drain job:

against a shut-down pool that would mint a second `Pool(Shutdown)` terminal_job
grant and leak mailbox ownership (one-owner-per-grant). Host `close_one` drains.

## 🔌️ The one platform I/O reactor every native document actor polls its hub socket against: a

current-thread Tokio runtime whose only thread (`semio-document-io`) parks on the I/O and time
drivers, created on first use and never shut down, like `db_storage_driver_runtime`. Actors keep
running on the `WorkerPool` and only enter this handle while polled. It used to be whatever
runtime the spawning thread happened to be inside; a native wgpu shell spawns from a plain
thread, so the first hub dial panicked with `there is no reactor running` and the document
socket never left `Connecting` (ticket 26/09/23 slice WG8, `hub-live-collaboration-check` run 1).
`None` only when the operating system refuses the reactor, which the dial then reports as a fault.
