# Source documentation

⛽️ Every cold relay turn stays inside the user-visible interactive slice. The guest job owns its
persistent state; the host grants one `step-job` call per shared-pool closure.

🧮️ The logical work units one relay request may spend across its crossings.

🔁️ One retained-waker future polled once per finite shared-pool turn.

🏃️ One relay request. Its first poll runs on the thread that owns the relay — the caller's own
poll of the mounted future — so a crossing that completes inside that poll (every `step-job`
that does not suspend) pays no pool queue and no hop back to the caller. A request that suspends
(the instance gate is held, the guest yields, the step waits on the caller's cancel) is resumed by
the pool exactly as before, so an abandoned or detached relay still finishes and releases the
instance on its own.

🎟️ Retains the request this slot answers; it owes its first poll to the owner.

🏃️ Gives an owed request its first poll on the calling thread. A panic reaches the request's
own recovery exactly as the pool-hosted future's did, so the instance is restored or
quarantined and the owner still receives one completion.

🧺️ An owner that goes away before the first poll hands the request to the pool, so nothing
it would have admitted is lost.

