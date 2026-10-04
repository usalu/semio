# 🛂️ Proctor

The proctor is the server of the teaching quizzes. It records who learns, deals every run its randomized sheet, takes
the answers, scores whole runs, awards badges and keeps the leaderboard. It is one native binary, `proctor` (crate
`teaching-proctor`, nx `@teaching/proctor`). Its only state is one SQLite file.

The proctor is **API only**. The quiz site is a static build on a CDN of its own origin
(`https://quizze.architektur-und-technologie.de`) and calls the proctor cross-origin at
`https://semio.iek.uni-hannover.de`. The proctor mounts the gateway's core routes and nothing
else — `GET /instance`, `POST /commands`, `POST /queries`, the presence socket and the learner event streams; no blobs,
no static apps, no ephemeral frames. Any other path answers a JSON `404` (`{"kind":"notFound",…}`).

Inside, it is an instance of the framework server product (`🧰️framework/🛍️products/🖥️server`), built with CQRS and
event sourcing:

- One **handle** actor per pseudonym or name registers it exactly once. There is no roster: a handle is its own small
  stream, so claiming one costs the same whether ten or ten thousand are taken.
- One **learner** actor per learner decides that learner's registration (directly when anonymous), runs, answers,
  submissions and badges, using the quiz core's pure `decide`/`evolve` (`🧰️framework/🛍️products/❓️quiz`).
- The **enrollment saga** relays the one fact of a handle stream to the learner it registered.
- The **admission** holds every command to its shapes and caps before anything is read or placed for it.
- A checkpointed **projector** folds every committed event into the read models: learner views, run views, who holds
  which handle, the crowd and the leaderboard.

Events are the only truth. Every read model can be rebuilt from them.

## Read this first: production

The proctor speaks plain HTTP and never terminates TLS. To bind a network interface, the environment must say three
things at once, exactly like the hub:

| Variable | Value | Why |
|---|---|---|
| `PROCTOR_MODE` | `production` | Development mode binds loopback only. |
| `PROCTOR_ALLOWED_ORIGINS` | `https://quizze.architektur-und-technologie.de` | The **site origin**. Only these origins get the CORS grant. Wildcards are refused. |
| `PROCTOR_TRUSTED_FORWARDING` | `proxy` | A TLS-terminating reverse proxy is the only client. Any request whose first `X-Forwarded-Proto` is not `https` gets `403` with `x-semio-refusal: insecure-transport`. |

If any one of the three is missing, the proctor refuses to boot and names the missing variable.

Nothing but the reverse proxy may reach the proctor's port. The deployment in
`🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/` puts Caddy in front: it serves `semio.iek.uni-hannover.de` over TLS (with a
certificate it obtains itself or one the operator supplies) and forwards `X-Forwarded-Proto`. For a health
check, use `GET /instance` with `X-Forwarded-Proto: https`.

### Cross-origin API

The site calls `POST /commands` and `POST /queries` with `content-type: application/json` and no credentials, so every
call is preceded by a CORS preflight. The request gate, the outermost layer, answers every `OPTIONS` itself with `204`:

| Preflight from | Answer |
|---|---|
| An allowlisted origin | `Access-Control-Allow-Origin: <that origin>`, `Access-Control-Expose-Headers: retry-after`, `Access-Control-Allow-Methods: GET, POST, HEAD, OPTIONS`, `Access-Control-Allow-Headers: content-type`, `Access-Control-Max-Age: 7200` (the browser reuses the grant for two hours instead of asking before every request), `Vary: Origin` |
| Any other origin | `204` with `Vary: Origin` and no `Access-Control-Allow-Origin` or max-age, so the browser blocks the call |

Actual responses carry the same grant (or none). The origin is always echoed, never `*`, and credentials are never
allowed: the client sends none (`fetch` with `credentials: "omit"`), so `Access-Control-Allow-Credentials` is not sent.
The exposed `Retry-After` lets a page read how long a refusal asks it to wait. The preflight also needs
`X-Forwarded-Proto: https`, which Caddy adds. In development (loopback bind, no allowlist), any loopback origin is
admitted, for example the Vite dev site on `http://localhost:6061`.

## Environment

The proctor is configured by environment variables only:

| Variable | Meaning | Default | Production (`semio.iek.uni-hannover.de`) |
|---|---|---|---|
| `PROCTOR_PORT` | TCP port | `8791` | `8791` |
| `PROCTOR_BIND` | IP address to bind | `127.0.0.1` | `0.0.0.0` (the container's own interface) |
| `PROCTOR_DATA` | Directory holding `proctor.sqlite` (created if missing) | required | the data volume |
| `PROCTOR_CATALOG` | The catalog `🔣️.json`. Its quizzes resolve relative to it. | required | the architecture catalog baked into the image |
| `PROCTOR_MODE` | `development` or `production` | loopback bind → development, otherwise production | `production` |
| `PROCTOR_ALLOWED_ORIGINS` | Comma-separated `scheme://host[:port]` origins (at most 32) | loopback bind → any loopback origin, otherwise none | `https://quizze.architektur-und-technologie.de` (the site origin) |
| `PROCTOR_TRUSTED_FORWARDING` | `none` or `proxy` | `none` | `proxy` |
| `PROCTOR_PRESENCE_TICK_MS` | How often, at most, a presence socket is sent what changed in its room, 10 to 1000 ms | `100` | `100` |
| `PROCTOR_LIMIT_PRESENCE_FRAME_BYTES` | The most one presence frame carries: per socket and tick, and per socket and watch interval over everything it watches; 4096 to 65536 bytes | `4096` | `4096` |
| `PROCTOR_LIMIT_PRESENCE_BYTES_PER_SECOND` | The most all presence rooms together send per second, 65536 to 4294967296 bytes | `16777216` | `16777216` |
| `PROCTOR_LIMIT_BODY_BYTES` | Largest request body, 1024 to 1048576 bytes | `16384` | `16384` |
| `PROCTOR_LIMIT_COMMANDS_PER_SECOND` | Per client address: sustained `POST /commands` | `300` | `300` |
| `PROCTOR_LIMIT_COMMANDS_BURST` | Per client address: `POST /commands` at once | `900` | `900` |
| `PROCTOR_LIMIT_QUERIES_PER_SECOND` | Per client address: sustained reads (`POST /queries` and every `GET`) | `600` | `600` |
| `PROCTOR_LIMIT_QUERIES_BURST` | Per client address: reads at once | `1800` | `1800` |
| `PROCTOR_LIMIT_UPGRADES_PER_SECOND` | Per client address: sustained WebSocket upgrades | `100` | `100` |
| `PROCTOR_LIMIT_UPGRADES_BURST` | Per client address: WebSocket upgrades at once | `1800` | `1800` |
| `PROCTOR_LIMIT_SIGNUPS_PER_HOUR` | Per client address: registrations (anonymous learners and claimed handles) accepted per hour | `100` | `100` |
| `PROCTOR_LIMIT_SIGNUPS_BURST` | Per client address: registrations accepted at once | `1200` | `1200` |
| `PROCTOR_LIMIT_SOCKETS_PER_ADDRESS` | WebSockets one client address may hold open | `2048` | `2048` |
| `PROCTOR_LIMIT_SOCKETS` | WebSockets open in total | `8192` | `8192` |
| `PROCTOR_LIMIT_IN_FLIGHT` | Requests served at once in total | `2048` | `2048` |
| `PROCTOR_MAX_LEARNERS` | Registrations in total: anonymous learners plus claimed handles | `100000` | `100000` |
| `PROCTOR_MAX_RUNS` | Runs one learner may start over all quizzes (open, submitted or voided) | `1000` | `1000` |

Every `PROCTOR_LIMIT_*` whose range the table does not name, and both `PROCTOR_MAX_*`, is a whole number from 1 to
1000000; a value outside its range stops the boot and names the variable.

## Identity: handles, ids and caps

Nobody has a password. A learner **is** its learner id, a secret only its browser holds, and may show a pseudonym or
name — a **handle** — to the others. This is what the proctor guarantees about them.

**Ids.** A command, learner or run id is exactly 32 lowercase hex characters; a quiz or task id is a slug
(`^[a-z0-9]+(?:-[a-z0-9]+)*$`, at most 64 characters). Anything else is refused as `id-invalid` — by the admission
before the command is placed, by the deciders again, and by every query before it looks anything up — so no malformed
id ever reaches an event, a key or a path.

**Handles** are normalized by the proctor, identically in the Rust and the TypeScript core, never trusted from the
client:

1. Every run of Unicode white space becomes one space, and leading and trailing white space is dropped.
2. The typographic apostrophe `’` becomes `'`.
3. What is left must be 1 to 64 code points of: digits; Latin letters including umlauts, `ß` and accented letters
   (`A–Z a–z À–Ö Ø–ö ø–ı Ĵ–ľ Ł–ň Ŋ–ž ƀ–ƺ Ƽ–ƿ Ǎ–ǰ Ǵ–ɏ Ḁ–ẙ ẜ–ỿ`, 681 letters); `'`, `.`, `_`, `-`; and single spaces
   between words. At least one letter or digit is required. At most 256 code points may be typed.
4. Everything else is refused as `handle-invalid`: control and format characters, zero-width and bidirectional
   characters, combining marks, other scripts (so no Cyrillic or Greek look-alike of a Latin name), emoji.

The alphabet holds no combining mark and only letters that are their own NFC form, so two handles that look the same
are the same code points; a handle typed with combining marks (`e` + `◌́`) is refused rather than silently composed —
keyboards and browsers send the composed letter. Two handles are the **same handle** when they are equal after
lowercasing: `Ada Lovelace`, `ada  lovelace` and `ADA LOVELACE` are one. The first registration keeps its spelling.

**Registering and recalling.** `identify-learner` with a pseudonym or name is addressed to the handle's own actor and
registers the learner under it, once: a second claim is refused as `handle-claimed`, whoever sends it. Returning to a
handle is a **read**, the `quiz.handle` query: it answers the normalized spelling and, when the handle is taken, the
learner that holds it and how it registered. Recalling writes nothing — no event, no receipt, no row — however often it
is asked. An anonymous learner registers in its own stream (`learner-exists` when it already did). A learner registers
once: one that claims a second handle holds both, and keeps the identity it registered first.

**Caps.** No stream and no table grows without bound:

| Cap | Default | Refusal | Set by |
|---|---|---|---|
| Registrations: anonymous learners plus claimed handles | 100000 | `roster-full` | `PROCTOR_MAX_LEARNERS` |
| Runs one learner started over all quizzes (open, submitted or voided) | 1000 | `runs-exhausted` | `PROCTOR_MAX_RUNS` |
| Runs one learner started in one quiz (open, submitted or voided), so switching the challenge back and forth stops | 200 | `runs-exhausted` | fixed |
| Open runs of one learner in one quiz | 1 | `run-open` at the open run's challenge (the client resumes it); a run left open at another challenge or on an earlier revision of the quiz is voided instead | fixed |
| Answers recorded in one run (every change of an answer counts) | 2000 | `answers-exhausted` | fixed |
| Openings of one task of an expert run | 1 | `already-opened` | fixed |

A receipt is stored for every accepted command and for no refusal, and no command is accepted without an event: a
repeated `open-task` is refused, so the receipts grow only with the facts the caps above bound.

A real class is far below every one of them (300 learners playing every quiz dozens of times). The registration cap is
held against a count the projector keeps, so it is a quota, not a ledger: a burst may pass it by the registrations
decided before the count caught up.

**What the registration cap is sized from.** It protects the disk, and it must not be a lever: once it is reached,
every new learner is refused `roster-full` until an operator acts. Two numbers set it, both measured on the release
build (`signup_measure_learner_cost.ts` in the ticket folder, 20000 registrations of each kind, 2026-10-02):

| One registration | Disk, live file | Disk, compact copy (`backup`) | Memory |
|---|---|---|---|
| Anonymous | 1.7 KiB | 1.3 KiB | none: the proctor keeps no learner in memory that does not play (37 MiB resident after 60000 registrations, 35 MiB after the first 20000) |
| Pseudonym of 16 letters | 2.9 KiB | 2.8 KiB | none |
| Name of 64 letters of three bytes each (the longest a handle can be) | 6.5 KiB | 6.3 KiB | none |

- **Disk at the cap of 100000:** 170 MiB if every registration is anonymous, 290 MiB with ordinary pseudonyms, 650 MiB
  in the worst case a client can construct. It is the whole cost: a registration that never plays costs no memory and
  no work per request.
- **Time to the cap from one address:** registrations are counted per client address by the
  [sign-up allowance](#limits-at-the-edge) — 1200 at once and 100 per hour —, so one address needs
  (100000 − 1200) / 100 = 988 hours, **41 days**, to fill the default cap (with the former cap of 10000 and no
  allowance it took 34 seconds; with that cap and this allowance it would take 88 hours). Ten addresses need four days,
  and what they leave is removed by [`proctor prune`](#prune-registrations-nobody-played-under) without touching anybody
  who played.

## Limits at the edge

The API has no passwords and is public, so the proctor bounds what one client address may ask and what it carries at
once. The defaults are sized for the real use: **a lecture hall of 300 learners behind one address** (a campus NAT)
must never be throttled, and **one script must not be able to take the proctor down**.

| Bound | Default | What a class of 300 needs (measured by the capacity gate) | Past it |
|---|---|---|---|
| Commands per address | 300 per second, 900 at once | 23 per second in a lecture (117 at the gate's five-fold pace); 300 at once when everybody signs up together | `429` |
| Reads per address | 600 per second, 1800 at once | 33 per second in a lecture (167 at the gate's pace: leaderboard every 10 s, run, learner and crowd views); 900 at once when a lecture starts | `429` |
| Socket upgrades per address | 100 per second, 1800 at once | 850 within two seconds when the network drops and every learner reconnects | `429` |
| Sign-ups per address (registrations that were accepted) | 100 per hour, 1200 at once | 300 within a minute when a lecture starts; 600 in the gate, two halls within two minutes | `429` naming the allowance `sign-up` |
| Sockets per address | 2048 | 900 (three per learner in a run) | `429` |
| Sockets in total | 8192 | — | `503` |
| Requests in flight in total | 2048 | a few dozen: a request is in flight for milliseconds, and its body is taken before it counts | `503` |
| Request body | 16384 bytes, delivered within 10 s | the largest real command is 4186 bytes (an answer guessing every item of the largest task, each guess the longest number JSON writes, `-1.2345678901234567e-300`; measured 2026-10-03), more than three times below, the headroom the end-to-end test holds every catalog it plays to | `413`; `408` when it does not arrive |
| Presence message (client to proctor) | 4096 bytes | a state is at most 2048 bytes | the socket closes |
| Presence frame (proctor to one socket) | 4096 bytes per tick; per watch interval over everything watched | 20 cursors per tick; a room that changes more is sent over the next ticks, every changed learner once before any twice | the rest waits for the next frame |
| Presence in total | 16 MiB per second | 11 MiB per second while 300 learners move their pointers | every socket waits longer for its next frame |
| Event-stream message | 1024 bytes | the client sends none | the socket closes |

- **Client address.** With `PROCTOR_TRUSTED_FORWARDING=proxy` it is the last entry of `X-Forwarded-For` — the one the
  terminating proxy wrote; anything a client put in front of it is ignored. Without the header (a health probe beside
  the proxy) and with `none`, it is the peer address. An IPv6 client is its /48 — the largest block a network commonly
  routes to one subscriber, who mints its 65536 networks at will and is still one client. At most 16384 addresses are
  tracked; one that sent nothing for two minutes, holds no socket and has every allowance full again is forgotten — so
  being forgotten never hands an address its sign-ups a second time —, and beyond the bound every further address
  shares one allowance.
- **A refusal** is answered before anything behind the edge runs: `429` (this address is over its allowance) or `503`
  (the proctor is at a total cap), both with `Retry-After: <seconds>` and
  `{"kind":"throttled"|"overloaded","message":…,"retryAfterMs":…}`, and with the CORS grant, so the site can read it.
  The client waits that long and retries; a command keeps its id, so the retry is applied once.
- **Sign-ups are an allowance of their own.** A registration is the one command that takes from a finite store, the
  registration cap, so the pace of requests is no bound for it. Every `identify-learner` — anonymous or claiming a
  handle — spends one token of its address's sign-up allowance and is handed it back when nothing was registered: a
  handle that is taken or refused, a malformed command and the replay of a sign-up cost nothing, so the allowance
  counts what an address made the proctor keep. 1200 at once is a lecture hall of 300 four times over (everybody
  signing up within a minute, the next hall right after, students who cleared their browser signing up again); 100
  per hour hands a hall's worth back every three hours and the whole allowance within twelve, so the same hall is never
  refused the week after. Past it the answer is
  `429 {"kind":"throttled","message":"the sign-up allowance of this address is spent","retryAfterMs":…,"allowance":"sign-up"}`,
  the wait being the time the next sign-up takes (at most 36 s). Nothing else is affected: who is registered plays on,
  and returning to a pseudonym is a read (`quiz.handle`), not a sign-up.
- **Bodies are taken at the edge.** A request's body is read whole — at most the body limit — before the request
  counts in flight, so a body that trickles in holds a connection for at most ten seconds (`408`,
  `{"kind":"stalled",…}`) and never one of the places requests are served in. A refused request's body is read too
  (for at most half a second): a response sent while a body is still on its way would close the connection, and a
  flood of refusals would then use up the ports the proxy reaches the proctor through. Only a body past the limit is
  left unread (`413`), and that connection closes — which is why the proxy must refuse larger bodies itself
  (Caddy `request_body max_size 16384`).
- **Presence queues nothing.** Every socket reads its rooms through a cursor of a few numbers: a socket that stops
  reading costs the proctor one frame and is closed after 60 s, a room of any size costs a reader only what it is sent,
  and all rooms together never send more than the total above for long. Presence is the first thing to slow down
  under a flood of sockets: a joining session still gets its `welcome` at once, but the rest of a large roster and the
  cursors of the others then arrive as the total allows — while commands and queries are served as before.
- **The presence total is dealt fairly among addresses.** However many sockets an address reads through, it is sent
  its fair share of the total and no more: an address is never held back while it takes less than an even split of
  the total among the addresses reading (8 MiB per second with one other address), what the modest ones leave is split
  evenly among those that ask for more, and the shares are dealt anew every quarter of a second. A script that pushes
  two thousand sockets into the hall's rooms therefore slows itself and leaves the hall half of the total, where the
  hall used to keep the share its 900 sockets were of all sockets, a third. A hall that asks for less than half is not
  slowed at all. The hall of the gate asks for more at its busiest — alone it is sent 11 MiB per second on average and
  up to the whole 16 —, so beside the flood it is sent 8 MiB per second then: 59 to 61 % of the presence bytes it is
  sent alone over the lecture (42 % before the total was dealt per address). Raising
  `PROCTOR_LIMIT_PRESENCE_BYTES_PER_SECOND` to 33554432 leaves that hall untouched by one flooding address, at twice
  the bytes the flood itself is sent. With k flooding addresses the hall keeps 1/(k+1) of the total: addresses are
  all the proctor can tell apart.
- **Errors say nothing about the inside.** A policy denial is `403 {"kind":"forbidden","message":"forbidden"}`, a
  storage fault `500 {"kind":"internal","message":"internal error"}` or a rejection `actorUnavailable` / `unavailable`;
  the cause is one `[ERROR]` line on stderr. `GET /instance` lists the command and query kinds a client addresses and
  no policy template.
- **Ids are checked first.** A command whose target, command id or idempotency key has a shape no quiz actor or
  command has is rejected (`invalid`, `id-invalid`) before the proctor reads or places anything for it, and an actor
  that never committed an event is not kept in memory. At most 2048 actors are kept placed; the least recently used
  one makes room and is rebuilt from its snapshot (written every 64 events) and the events after it.
- **An idempotency key is its sender's own.** A resubmitted command id answers the stored receipt only to the same
  principal for the same target; the enrollment saga's keys live in the proctor's own key space, so no caller can read
  or occupy them.

The gate `bun nx run @teaching/proctor:capacity` holds the release build to this, under its default cap of learners:
300 simulated learners from one address play two quizzes each, spread evenly over the four challenges (an easy run read
again after every answer for its hints, guesses on hard and expert, every expert task opened before its answers and
answered only while its clock runs), at five times the pace of a real lecture, alone and then
beside an abusive script from another address (it tries to fill the roster — registrations, anonymous and under
handles, as fast as it can post them — while the hall signs up, floods malformed commands, queries and sockets on
connections it keeps open, and sends oversized bodies and frames). It fails unless the hall saw no error and no
refusal inside its latency budget (p95 250 ms and p99 500 ms of every command and query alone, 500 ms and 1500 ms
beside the script) — every one of its twice three hundred sign-ups registered —, the script was served no more than
its allowances of commands, queries and sign-ups, capped and cut off, the hall was still sent at least half of the
presence bytes it is sent alone, the roster — counted by `proctor prune --dry-run` on the stopped proctor — held the
hall and what the script's sign-up allowance let through, less than 5 % of the cap, with exactly the script's
registrations being what a prune removes, and the proctor stayed below 512 MiB. Hall, script and proctor share one
machine and its pool of client ports, which the script empties: a request of the hall that gets no answer is sent
again, and a socket join that gets none (no connection, or one cut before the proctor answered) is made again, after
the quiz client's own backoff, the wait counted as latency — at most 1 % of each; a join the proctor answers with a
refusal and a socket that closes after its welcome stay errors. The hall is played on several threads and the gate
reports how late the load generator itself ran; on a machine too busy to measure it says so instead of blaming the
proctor.

Measured on an 8-core desktop (2026-10-02, release build, default cap of 100000 learners, SQLite on an NVMe disk, hall
and script on the same machine, two passing runs): the hall alone — every command and query p50 2 to 5 ms, p95 below
35 ms, p99 below 100 ms, no error; the hall beside the script, which was refused 430000 to 500000 times in a minute
while it was served its allowances and no more — p50 2 to 6 ms, p95 below 35 ms, p99 below 180 ms, no error, all 600
sign-ups of the two halls registered. The script registered 1201 learners (its sign-up allowance over the 53 to 57 s
it ran: 1202) and was refused 11300 to 12100 further sign-ups in the name of that allowance; the roster ended at 1801
of 100000 registrations (1.8 %), of which `prune` would remove exactly the script's 1201. The proctor used a fifth to
a third of a core alone and two fifths to two thirds of a core beside the script, and at most 109 MiB. Joining a presence room (until the
`welcome`) took 1 ms alone (p50; p99 9 to 13 ms) and 2 ms (p99 19 to 35 ms) while the script pushed two thousand
sockets into the hall's own roster room; what gives under such a flood is the rest of presence — the hall was sent
59 to 61 % of the presence bytes it was sent alone, its even split of the total. These figures, and the needs of a
class in the table of the edge limits, were measured while the hall played medium runs only. The gate now plays all
four challenges — an expert run adds one `open-task` per task and an easy run one read of the run per answer — and
has not yet been run on that mix, so every figure above is to be re-measured by its next run on a quiet machine.

## Run

| Command | What it does |
|---|---|
| `bun nx run @teaching/proctor:dev` | Serves `🎓️teaching/🏛️architecture/❓️quiz/🔣️.json` on `127.0.0.1:8791` over the git-ignored `.🧬semio/🎓️teaching/proctor-dev/`. The dev site (Vite, port 6061) proxies `/instance`, `/commands`, `/queries`, `/actors` and `/scopes` to it, so in development the client stays same-origin. `PROCTOR_*` variables set by the launcher win. |
| `bun nx run @teaching/architecture-quiz:dev` | The proctor and the site together in one terminal (launch row `🛠️dev🎓️teaching🏛️architecture❓️quiz`): it reuses a proctor that already answers on `PROCTOR_PORT` (saying so when it serves another quiz contract), else launches one exactly as `@teaching/proctor:dev` does, waits for `GET /instance`, then starts the site; the proctor it launched is built and launched anew when its sources or catalog change and launched again when it ends by itself. One Ctrl+C stops both. See the site's README. |
| `bun nx run @teaching/proctor:check [-- <catalog>]` | `proctor check <catalog>`: validates a catalog and every quiz it lists, and prints each quiz's revision. It exits non-zero and lists `[ERROR] <json-pointer> <code> (<quiz file>)` for every issue. |
| `bun nx run @teaching/proctor:rebuild` | `proctor rebuild`: drops every read model of the dev data directory and refolds the whole event log, with progress. |
| `bun nx run @teaching/proctor:health` | `proctor health` against the dev proctor (launch row `🩺️health🎓️teaching🛂️proctor`): exits 0 exactly when it serves. |
| `bun nx run @teaching/proctor:backup [-- <directory/\|file\|->]` | `proctor backup` of the dev database while the dev proctor serves it (launch row `💾️backup🎓️teaching🛂️proctor`). Without a target it writes `proctor-<UTC time>.sqlite` into the git-ignored `.🧬semio/🎓️teaching/proctor-backups/` and prints the path. |
| `bun nx run @teaching/proctor:restore -- <file\|->` | `proctor restore` into the dev data directory (launch row `♻️restore🎓️teaching🛂️proctor`). Stop the dev proctor first. |
| `bun nx run @teaching/proctor:erase -- --handle <handle> [--dry-run]` | `proctor erase` on the dev database (launch rows `🧨️erase🎓️teaching🛂️proctor🔍️dry-run` and `🧨️erase🎓️teaching🛂️proctor`); `--tag <tag>` and `--learner <id>` select as well. Stop the dev proctor first. |
| `bun nx run @teaching/proctor:prune -- --older-than <age> [--dry-run]` | `proctor prune` on the dev database (launch rows `🧹️prune🎓️teaching🛂️proctor🔍️dry-run` and `🧹️prune🎓️teaching🛂️proctor`, which ask for the age, `7d` unless stated). Stop the dev proctor first. |
| `bun nx run @teaching/proctor:test` | Unit tests, the storage conformance laws and the end-to-end API test. Levels are `test-quick`, `test-long` and `test-exhaustive`. |
| `bun nx run @teaching/proctor:build` | Builds the cargo artifacts. Pass `-- --release` for a deployment. |
| `bun nx run @teaching/proctor:capacity [-- --learners 300 --compression 5 --report <file> --executable <proctor> --hall-only]` | The capacity gate (launch row `⚖️gate🎓️teaching🛂️proctor🏋️capacity`): builds the release binary (or takes `--executable`), serves it in production mode on a throw-away port and data directory with the default cap of learners, plays the lecture hall alone and beside the abusive script (`--hall-only` skips the script and is not the gate), stops it and counts its roster (`prune --dry-run`), prints p50/p95/p99 per operation, how late the load generator itself ran, the script's answers by status, the roster and the proctor's memory, and exits non-zero on any violation. Run it on a quiet machine: it takes about five minutes with the build. |

The binary itself takes `proctor serve`, `proctor check <catalog>`, `proctor rebuild`, `proctor health`,
`proctor backup <directory/|file|->`, `proctor restore <file|->`,
`proctor erase (--handle <handle>|--tag <tag>|--learner <id>) [--dry-run]` and
`proctor prune --older-than <age> [--dry-run]`; the last five are the
[operator verbs](#operate-health-backup-restore-erase-prune). Anything else prints the usage and exits `2`.

Every nx target but `build`, `test` and `capacity` builds the binary and runs a private copy of it from the git-ignored
`.🧬semio/🎓️teaching/proctor-bin/` (one copy per run, deleted when it exits; a copy left behind by a killed launcher is
removed by the next run). A running dev proctor therefore never holds Cargo's own `proctor` executable, which Windows
would otherwise refuse to replace, so builds, tests, `check` and `rebuild` never contend with it.

The build reports a progress line every ten seconds and stops when the launcher is interrupted. A listening proctor
answers `GET /instance` with `200` and its instance id `teaching-proctor`; the launcher (`🏗️bootstrap/🟦️.ts`:
`buildProctor`, `launchProctor`, `proctorReady`) is what the site's `dev` and its end-to-end gate start their proctors
with.

At boot, the proctor:

1. Validates the catalog. An invalid catalog stops the boot and lists every issue.
2. Opens `proctor.sqlite` and refuses a file of another format (see [Data](#data)).
3. Rebuilds the read models if they were built against a different catalog fingerprint or by an earlier projector
   revision. The fingerprint changes whenever a quiz file changes; the revision whenever the proctor starts keeping a
   read model differently (revision 2 added the crowd, revision 3 the handles, the standings and the registration
   count, revision 4 keeps a transcript per ranked learner instead of its standing, for the leaderboards of a period
   and of one quiz, revision 5 the score and place distributions of the crowd, revision 6 the challenges: points and
   bests by points in the learner views and transcripts, opened tasks and hints in the run views, guesses in the crowd).
4. Relays any missing enrollment.
5. Catches the projections up with progress.
6. Listens.

**Nothing is skipped.** The fold reads every committed event strictly. An event of a stream kind the proctor does not
know, an event that does not decode, a registration in the wrong handle stream, or a stored state or transcript that does
not decode stops the catch-up — at boot and in `proctor rebuild` alike — with one line that names it:

```text
[ERROR] storage backend failure: event at position 4711 (architecture/quiz-learner/<id> seq 12, quiz.learner-recalled) cannot be folded: unknown variant `learner-recalled`, expected one of …; the log was written by another proctor version or is damaged
```

The proctor does not listen over a log it cannot read, and the checkpoint stays before the event, so the next start
fails the same way instead of serving views that silently miss facts. The same holds for one actor at run time: a
learner or handle whose stream or snapshot does not decode answers every command with `actorUnavailable`
(`actor-corrupt: …`), logs the cause, and no other learner is affected. The way out is the proctor version that wrote
the log, or a restore.

Ctrl+C, Ctrl+Break, `SIGTERM` (sent by `docker stop` through tini), closing the console or a system shutdown all stop
the proctor gracefully. In-flight requests finish, the sagas and projections settle once more, and SQLite checkpoints
the WAL into `proctor.sqlite`. A catch-up or rebuild interrupted this way stops between batches. The next start resumes
from the last folded batch.

## Wire (design §9a)

Clients use the framework server contract (`@semio-tech/framework-server`), and the gateway routes sit at the origin
root:

- `POST /commands` takes a `CommandEnvelope`:
  - Its `kind` is `quiz.<type>` and its `version` is the wire version of the quiz contract (`WIRE_VERSION`,
    `$defs/WireVersion` of the quiz schema; 4 since the hint verdicts and short labels). An envelope of any other version is
    `envelope-mismatch: …` (a query of another version is `400`). `GET /instance` declares every command and query kind
    at that version; the quiz client sends nothing to a proctor that does not declare its own (it keeps everything on
    the device instead), so a proctor and a site of different contracts never take each other's commands or views.
  - `commandId` and `idempotencyKey` both equal the quiz command's `id`.
  - `tenant` and `scope` are the catalog id.
  - `target` is `{kind:"quiz-learner",id:<learner>}` for every command — except an `identify-learner` under a
    pseudonym or name, whose target is the handle: `{kind:"quiz-handle",id:<lowercase hex of the UTF-8 bytes of the
    normalized, lowercased handle>}` (`Ada Lovelace` → `616461206c6f76656c616365`; in TypeScript
    `handleActorId(normalizeHandle(handle).key)` from the quiz core). A target of any other shape is `id-invalid`; a
    well-formed target that is not the command's own is `envelope-mismatch: …`.
  - `payload` holds the UTF-8 bytes of the quiz `Command` JSON.
- The commands a learner sends:

  | Kind | Payload beyond `id`, `learner` | Offline | What it decides |
  |---|---|---|---|
  | `quiz.identify-learner` | `identity` | authority required | a registration, anonymous or under a handle |
  | `quiz.start-run` | `run`, `quiz`, `challenge` (`easy`, `medium`, `hard` or `expert`), `at` | authority required | `run-started` with the challenge at the `at` the device claims; an open run of the quiz at another challenge is voided first (`run-voided`, at the proctor's clock), one at the same challenge answers `run-open` |
  | `quiz.open-task` | `run`, `task`, `at` | optimistic | starts the clock of a task of an expert run: `task-opened`, once per task (a repeat is refused with `already-opened`, so it stores no receipt) |
  | `quiz.record-answer` | `run`, `task`, `answer`, `at` | optimistic | `answer-recorded`; on an expert run only within the task's `seconds` from its opening |
  | `quiz.submit-run` | `run` | authority required | `run-submitted` with the result, its challenge and its points (`score × par`: 100, 200, 300, 400), then the badges; an expert run is submitted with whatever was answered |

  `at` is the instant the learner acted by the device's clock. The proctor first lowers every `at` to five minutes
  (`CLOCK_LEAD`, 300 000 ms) past its own clock: a device whose clock runs less than five minutes ahead is never
  lowered, while a start or an opening dated an hour ahead is and buys no time. A run starts at its `at`, so a run
  started offline keeps the device's start however late it arrives. The proctor raises the `at` of an opening or an
  answer to its floor — the run's start for an opening and an untimed answer, the task's opening for a timed
  answer — and applies the limit to those instants; apart from the lead its own clock never enters a time verdict,
  so an answer made in time and delivered after a connection shortage still counts. A `challenge` that is none of the four, or an `at` that is no
  integer from 0 to 2^64 − 1, is `command-malformed: …`; an `at` from 2^53 up to 2^64 − 1 (beyond the largest integer
  every client reads exactly) is `id-invalid`.
- A quiz rejection comes back as `{"status":"rejected","reason":{"kind":"invalid","detail":"<rejection>"}}`. Besides
  the lifecycle's own (`unknown-learner`, `run-open`, `answer-invalid`, …, and for the clock of an expert run
  `run-untimed` — an `open-task` on a run without a clock —, `task-unopened` — an answer to a task that was not
  opened —, `time-up` — an answer made after the task's `seconds` — and `already-opened` — an `open-task` for a task
  opened before) these are `id-invalid`, `handle-invalid`,
  `handle-claimed`, `learner-exists`, `roster-full`, `runs-exhausted` and `answers-exhausted`
  (see [Identity](#identity-handles-ids-and-caps)).
- Accepted events carry `kind: "quiz.<type>"` and the quiz `Event` JSON as their payload.
- A command id that is resubmitted answers the same receipt and produces no second event.
- An `identify-learner` from an address whose [sign-up allowance](#limits-at-the-edge) is spent is not decided at all:
  `429` with `{"kind":"throttled",…,"retryAfterMs":…,"allowance":"sign-up"}`. `allowance` is absent from the `429` of
  an address that merely sends too fast, which a client waits out silently; with it, the client tells the learner
  that too many people signed up from this network just now and when to try again. `roster-full` is the other refusal
  of a sign-up: a rejection, not worth retrying before an operator acted.
- `POST /queries` takes `quiz.catalog`, `quiz.learner`, `quiz.run`, `quiz.leaderboard`, `quiz.crowd` or `quiz.handle`.
  Its `arguments` are the bytes of the quiz `Query` JSON, and it answers a `snapshot` whose `value` is the bytes of the
  view JSON. An unknown learner, run or quiz answers `404`; a malformed id or a handle outside the policy answers `400`
  whose message starts with `id-invalid` or `handle-invalid`, before anything is looked up.
- `quiz.leaderboard` (`{"type":"leaderboard","period":"daily"|"weekly"|"monthly"|"all-time"}`, optionally with
  `"quiz":<one quiz of the catalog>` and `"learner":<caller's id>`) answers one of the four leaderboards, of every quiz
  or of one: exactly `{"period":…,"quiz":…,"window":{"from":…,"until":…},"rows":[…],"learners":n,"submissions":m,
  "own":{…}}` — `quiz` only when the query named one, `window` for every period but `all-time`. A board counts the runs
  submitted inside its window — the day, the ISO week (from Monday) or the month that contains the proctor's clock, in
  UTC — and of its quiz; its rows are made of those runs only (per quiz the best run — the one with the most points,
  whatever its challenge — as `{"challenge","score","points"}`, the `total` of those points, the badges they earned,
  their count and the last of them). It answers the top 100 rows, the number of ranked learners, the number of runs submitted in
  the whole catalog (whatever the board: it grows with every submission, which is how a client knows when to ask for the
  crowds again) and — when the query names a learner with a run in scope — that learner's own row with its true rank,
  wherever it stands. The caller is named in the query because the proctor resolves no principal. A row never carries a
  learner id, only the public tag. A quiz the catalog does not list answers `404`; a query without a period, or with
  another one, `400`. The ranking is the core's (`total` descending, then more badges, then who reached the total
  first); the proctor keeps the submitted runs and badges of every ranked learner (its transcript) in memory and, for
  every board that was asked for, a rank index of what orders the learners on it. It moves one entry per index when a
  run is submitted, a badge is awarded or an identity changes — never on an answer — and answers a query from the top
  of an index and one binary search, whatever the number of learners. When the clock enters the next day, week or
  month, the first query for that board builds its index anew from the transcripts. Every answer equals the core's
  `leaderboard` over every transcript at that instant.
- `quiz.handle` (`{"type":"handle","handle":<as typed>}`) answers `{"display":<normalized>,"holder":{"learner":…,
  "identity":…}}`, without `holder` when the handle is free. This is how a returning learner is recalled; it writes
  nothing.
- `quiz.crowd` (`{"type":"crowd","quiz":…}`) answers what the learners answered and scored in the submitted runs of
  one quiz, the `CrowdView` (design §17 and §20): the run scores in ten bins of whole percents (`scores`, `[0, 10)` …
  `[90, 100]`, summing to `runs`); per task (and matching dimension) the same ten bins of the task's (the dimension's)
  scores; and per item the counts per category or value, or for a sorting item the mean normalized position beside
  `places`, how often the learners put it at each place a sheet of the task presents. It exists for every quiz of the
  catalog, with `runs: 0`, ten zeros in every `scores` and no items before the first submission. The proctor keeps it
  incrementally, folding each `run-submitted` result into a stored per-quiz tally — score bins per quiz, task and
  dimension, and per sorting item the count per order length and position, so the view bins the positions against the
  places the current definition presents — and it equals the quiz core's `crowd_view` over every submitted result byte
  for byte. Runs of every challenge are mixed: the score bins count accuracy, a guessed matching value counts under the
  authored value nearest to it (the one thing a tally takes from the definition; a changed catalog refolds it), an
  item left unanswered on an expert run counts nowhere, and a sorting nobody guessed in adds its score only.
- `quiz.run` answers the run with its sheet at the run's challenge: `easy` and `medium` sheets show the keys (a
  sorting's ascending `keys`, a matching's `cards`, category descriptions and axis numbers), `hard` and `expert`
  sheets carry no keys (no sorting `keys`, no `cards`, no descriptions, no axis numbers; profiles are shares of the
  axis range), and every task of an `expert` sheet carries its `seconds`. An expert run view also
  holds `opened` (task → instant), and an open `easy` run view holds `hints` for the answers that earned any (task →
  questions, at most one per far-off item and three per task, those most wrong: `compare` a key against another
  item's, preferring a familiar one, with its `verdict` `under`, `over` or `reversed`; `profile` — beside an item
  rightly placed on the other side where there is one —, `group` or `category` for a misplaced classification item;
  each questions a relation the answer claims and never states the truth). Sheet items, axes and quantities carry their
  `short` labels for the hints to name; whether an item is `familiar` never leaves the proctor.
- `GET /actors/<catalog>/quiz-learner/<learner>/events[?since=n]` (and `/events/ws`) replays one learner's event
  stream. The handle streams are readable by nobody (`403`); who holds a handle is answered by `quiz.handle` only.
- Any other path answers the gateway's JSON `404`; a known route with another method answers `405` with its `Allow`.

## Presence (design §15–§17)

Every learner sees who else is online, where they are, their cursors and what they drag, and — in a run — what they
currently think. This state is **ephemeral and shared**: the proctor keeps it in memory only, never writes it to
SQLite, and forgets a learner the moment their socket closes.

Clients open `GET /scopes/<scope>/presence/ws?surface=<surface>` as a WebSocket with the subprotocol
`semio.presence.v1` (in TypeScript, `presenceSocketUrl(baseUrl, scope, surface)` from
`@semio-tech/framework-server`). The scope is percent-encoded, so `/` becomes `%2F`. The rooms of a catalog are:

| Scope | State shared there | Who joins |
|---|---|---|
| `<catalog>` | `PresenceState`: public tag, identity, place, whether the tab is visible | every learner past the identity screen |
| `<catalog>/introduction`, `<catalog>/home`, `<catalog>/leaderboard`, `<catalog>/badges` | `CursorState`: public tag, cursor relative to an anchor, keyboard focus; no drag | learners on that page |
| `<catalog>/quiz/<quiz>` | `CursorState`, anchors may be items (`item:<id>`) and categories (`category:<id>`), `drag` names an item of the quiz | learners on the quiz page, in a run or on the results of that quiz |
| `<catalog>/quiz/<quiz>/thinking` | `ThinkingState`: public tag and the draft answers per task — classification and sorting by item and category ids, matching as the values assigned per dimension | learners in a run of that quiz |

The protocol:

1. On join, the server sends `welcome`: the socket's session id (random, generated by the server), its colour slot and
   the room's roster, led by the new session. A roster larger than one frame (`PROCTOR_LIMIT_PRESENCE_FRAME_BYTES`,
   4096 bytes) begins in the `welcome` and continues in the `batch` frames that follow it at once.
2. The client sends `{"type":"state","state":…}` whenever its state changes. The server keeps only the latest state of
   each session. A state may be at most 2 KiB, and at most 30 per second are taken; the excess is dropped. A message
   of more than 4096 bytes is never read: the socket closes.
3. At most once per tick (`PROCTOR_PRESENCE_TICK_MS`, 100 ms by default), a socket is sent one `batch` with the
   sessions that changed since its last frame — their newest state only — and the sessions that left, so traffic grows
   with the number of learners who move, not with how often they move. A burst larger than one frame is spread over
   the next ticks, every changed session once before any twice. A client applies every frame as it comes: `welcome`
   replaces what it held, `batch` removes `left` and upserts `entries`.
4. A state the proctor does not admit is answered with `{"type":"refused","reason":…}` and not shared. The socket
   stays open.
5. The server pings every 20 s. A socket silent for 60 s is closed, and so is one that takes longer than that to be
   sent a frame; its session leaves the room.
6. A socket may also **watch** up to 16 other rooms read-only, for example the home page watching every page, quiz
   and thinking room behind its cards: `{"type":"watch","scopes":[…],"intervalMs":250}` replaces the watched set (an
   empty list stops watching). The server answers each newly watched scope with a `watched` frame flagged
   `"snapshot":true` holding the beginning of its roster, then at most one `watched` frame per scope and interval with
   the sessions that changed and those that left (and the rest of a roster that did not fit) — one frame's worth per
   interval over all watched scopes, the scope that had to wait going first the next time. The interval is clamped to
   at least the tick. Watching never joins: the watcher appears in no roster and publishes only to the room it joined.
   A socket that missed more departures of a room than the room remembers (256) is sent that room anew: a second
   `welcome`, or a `watched` snapshot.

What is admitted:

- **Joining and watching** need the `quiz-presence` policy template (`join`, `publish`, `watch`). Every caller holds
  it inside the catalog's rooms and nowhere else, so a socket to another catalog's room or to an unknown quiz is
  refused with `403`, and a watch naming such a scope is answered `refused` with `forbidden <scope>` and changes
  nothing; more than 16 scopes are refused with `watch-too-many`. A `surface` longer than 64 characters is refused
  with `400`.
- **Origin.** A socket whose `Origin` the cross-origin policy does not admit is refused with `403`: in production only
  the site origin, in development any loopback origin. Behind the proxy, the handshake also needs
  `X-Forwarded-Proto: https`, like every request. Caddy's `reverse_proxy` forwards WebSocket upgrades as they are.
- **States** must be the room's type and pass the quiz core's rules (`presence_problem`, `cursor_problem`,
  `thinking_problem`), and they may name only what the catalog renders: a place names a quiz of this catalog and a task
  of that quiz; a drag names an item of the room's quiz; a draft names tasks of the quiz with answers of their kind,
  their items, categories and dimensions, and only numbers an answer can carry: a matching value is a card value or,
  where the keys are hidden, the number guessed, and a sorting guess names an item of its task; on a logarithmic scale
  either is positive. The refusal reason is the issue code and its JSON pointer (`tag-invalid /tag`,
  `quiz-unknown /place/quiz`, `id-unknown /drag/item`, `kind-mismatch /answers/<task>`,
  `value-invalid /answers/<task>/values/<dimension>/<item>` or `/answers/<task>/guesses/<item>`),
  `state-invalid` for a state of the wrong shape, `state-too-large`, or `frame-invalid` for anything but a `state` or
  `watch` frame.

The quizzes are for fun: since design §17 the drafts and drags of every open run are shared while it is played, and
what everyone answered and scored is one read (`quiz.crowd`). Whether a learner sees either is the client's gate
(design §20): by default only once that learner has submitted the quiz, earlier on request — the proctor withholds
nothing, because the learner may ask at any time. Presence never carries a learner id, only the public tag.

## Data

All state is `<PROCTOR_DATA>/proctor.sqlite`. It runs in WAL mode with synchronous commits, and holds a format row that
refuses a file of another format. Events, receipts and outbox deliveries are append-only. The read models live in the
same file and can always be rebuilt (`proctor rebuild`, with the serving proctor stopped: the directory belongs to one
serving process).

The format is `semio.teaching.proctor.sqlite` **v3**: one stream per handle (since v2, which replaced the roster
stream and the `learner-recalled` event), every `run-started` with its `challenge`, `task-opened` facts, every
`answer-recorded` stamped with the instant the learner acted, and every result with its `challenge` and `points`. A file
of an earlier format — any development database written before the challenges — is refused at open
(`… holds format semio.teaching.proctor.sqlite v2; this proctor reads … v3`). There is no migration, because nothing
was in production: delete the directory and start again. In development nobody has to: `@teaching/proctor:dev` (and
the `dev` of a site that launches it) looks at the format row before it builds, moves the launcher's own disposable
`.🧬semio/🎓️teaching/proctor-dev/` aside to `.🧬semio/🎓️teaching/proctor-dev.v<format>-<time>/`, starts with empty data and
says so in one line. A directory named with `PROCTOR_DATA` is never touched — the command ends at once, status 1, with
one line that names it. `proctor serve` itself never does either, in any mode: it refuses the file.

## Operate: health, backup, restore, erase, prune

Five verbs of the same binary, made for an image without a shell (distroless, read-only root, uid 65532, only
`PROCTOR_DATA` writable). Each reads `PROCTOR_DATA` (and `health` `PROCTOR_BIND`/`PROCTOR_PORT`) from the environment
the container already has. Log lines go to stderr; what a verb answers goes to stdout.

| Verb | While serving? | What it does | Exit |
|---|---|---|---|
| `proctor health` | yes | Asks the listener of this environment for `GET /instance` exactly as the TLS-terminating proxy does (`X-Forwarded-Proto: https`, 3 s timeout). | `0` on `200`, else `1` |
| `proctor backup <directory/\|file\|->` | **yes** | Writes a whole, consistent copy of the database (one read transaction, `VACUUM INTO`) while the proctor keeps committing, verifies it and prints its path. | `0`, `1` on failure, `130` when interrupted |
| `proctor restore <file\|->` | no — refused | Replaces the database of a stopped proctor with a verified copy. | `0`, else `1` |
| `proctor erase (--handle <handle>\|--tag <tag>\|--learner <id>) [--dry-run]` | no — refused | Removes one learner for good, or with `--dry-run` reports what would go. | `0`, `1` when nobody or more than one learner matches, `2` on a wrong command line |
| `proctor prune --older-than <age> [--dry-run]` | no — refused | Removes every registration nobody played under that is older than the age, or with `--dry-run` counts them. | `0` (also when there is nothing to prune), `1` on failure, `130` when interrupted, `2` on a wrong command line |

### Health

`HEALTHCHECK CMD ["/usr/local/bin/proctor", "health"]` (exec form, no arguments, no shell). Healthy means: the catalog
was valid, the log was readable, the projections are caught up and the listener answers.

### Backup

- **Where.** A directory — an existing one, or any path ending in `/`, which is created — gets
  `proctor-<UTC time>.sqlite`, for example `proctor-20261002T093000Z.sqlite`; any other path is the file itself; `-`
  streams the copy to stdout (spooled through `PROCTOR_DATA`, the one place a read-only container may write, and
  removed afterwards).
- **Atomic.** The name is reserved first, the copy is written beside it as `<name>.partial-<pid>` and moved onto the
  name only once it is whole, so a backup file is never half a backup. An existing file is **never overwritten**.
- **Verified.** The copy must open as a proctor database of this format, pass SQLite's integrity check, and hold an
  event count between the source's before and after the copy. The last stderr line says what it holds:
  `[INFO] backup: 90112 bytes, 4 events up to position 4, verified`.
- **Progress and cancellation.** Progress is reported in ten-percent steps on stderr. Ctrl+C or `SIGTERM` interrupts
  the copy, removes the partial file and the reservation, and exits `130`.

```sh
# on a host: into a directory of the data volume, then copy it away at leisure
docker compose exec -T proctor proctor backup /srv/quiz/data/backups/
# without touching the volume: the copy on stdout (no docker cp, no shell in the image)
docker compose exec -T proctor proctor backup - > "proctor-$(date -u +%Y%m%dT%H%M%SZ).sqlite"
```

A backup taken with `-` is not verified on the receiving side by the proctor; `proctor restore` verifies it before it
replaces anything.

### Restore

`proctor restore <file|->` spools the copy into `PROCTOR_DATA`, verifies it (format, integrity) and only then puts it
in place of `proctor.sqlite`, removing any stale `-wal`/`-shm`. It refuses while any process holds the database open —
stop the proctor first — and a refused restore leaves the database and the directory as they were.

```sh
docker compose stop proctor
docker compose run --rm -T --no-deps proctor restore - < proctor-20261002T093000Z.sqlite
docker compose start proctor
```

The proctor started on the copy answers what the proctor it was taken from answered at that moment: same learners,
runs, leaderboard, crowd and handles (the end-to-end test boots one on a restored copy and compares every view). The
read models travel inside the copy; if the image's catalog differs from the one they were built against, the boot
rebuilds them.

### Erase (a learner asks to be forgotten)

A learner's name is personal data, and events are otherwise never removed. `erase` is the one sanctioned removal.

1. Find the learner: by the handle it registered (`--handle "Ada Lovelace"`, typed any way — it is normalized like any
   other), by the public tag the leaderboard shows next to a row (`--tag 0a1b2c3d`), or by its learner id
   (`--learner <32 hex>`). An anonymous learner has no handle; use the tag or the id. A tag is 32 bits, so two learners
   can share one: then `erase` lists them and changes nothing — name the one by handle or id.
2. Stop the proctor. `erase` refuses a database that is being served.
3. Run it with `--dry-run` and read what would go; then run it without.

```sh
docker compose stop proctor
docker compose run --rm -T --no-deps proctor erase --handle "Ada Lovelace" --dry-run
docker compose run --rm -T --no-deps proctor erase --handle "Ada Lovelace"
docker compose start proctor
```

```text
erased learner #0a1b2c3d name "Ada Lovelace" of catalog architecture
  learner stream: 9 events, 8 receipts, 9 outbox rows, 0 snapshots, 0 leases
  handle "ada lovelace": 1 events, 1 receipts, 1 outbox rows, 0 snapshots, 0 leases
  read models: 61 rows dropped; the next start rebuilds them from the events that are left
done: the handles are free again, the learner id is unknown to the proctor, the scores are gone
```

What is removed, in one transaction: the learner's whole event stream (registration, runs, answers, results, badges),
the stream of every handle it holds, their receipts, outbox rows, snapshots and leases, and **every** read model. The
file is then rewritten without its free pages (`secure_delete`, `VACUUM`, WAL truncated), so the name and the learner
id are no longer bytes of any file in `PROCTOR_DATA` — a test searches the files for them. The next start rebuilds the
read models from the events that are left.

- **The handle is free again**: somebody else — or the same person — can register it.
- **The scores go with the name.** The learner's runs leave the leaderboard and the crowd counts. Keeping anonymous
  scores would keep the answers of an identifiable person under an id the person still holds; a request to be forgotten
  is served in full. Every other learner's views are untouched (their rank may move up).
- **The learner id becomes a stranger**: commands under it answer `unknown-learner`, its learner and run views `404`.
- **Backups are not rewritten.** A backup taken before the erasure still holds the name. Delete those backups, or
  restore-erase-backup them; note the erasure so that a later restore of an old backup is followed by the same `erase`.
- **Presence** needs nothing: it was never stored.

### Prune (registrations nobody played under)

A public sign-up without a password collects registrations that nobody ever plays under: a visitor who looked and
left, a script that registered as many as its [sign-up allowance](#limits-at-the-edge) let through. They cost disk and
count against the registration cap. `prune` removes them and nobody else:

- **every learner that never submitted a run and registered longer ago than `--older-than`**, with every handle it
  holds — also a learner that started a run and never finished it;
- **every handle claimed that long ago beside the identity of a learner that stays.** A learner is the identity it
  registered first; a second handle claimed under the same learner id only occupies a name (the quiz site never does
  that, a script can).

Whoever submitted a run stays, whenever they registered, with every score, badge and handle they registered under.
What goes, goes by the rules of `erase`: whole streams with their receipts, outbox rows and snapshots, every read
model, and the file is rewritten without the removed bytes — the handles are free again, the learner ids are unknown
to the proctor, and the count the cap is held against goes down at the next start.

`--older-than <age>` is required and has no default: a whole number and a unit, `90m`, `36h`, `7d`, `2w` (`s`, `m`,
`h`, `d`, `w`). Choose it longer than a learner may take between signing up and finishing a first quiz — a week is a
good rhythm; someone who signed up yesterday and has not played yet stays.

```sh
docker compose stop proctor
docker compose run --rm -T --no-deps proctor prune --older-than 7d --dry-run
docker compose run --rm -T --no-deps proctor prune --older-than 7d
docker compose start proctor
```

```text
would prune 1203 registrations of catalog architecture older than 7d (registered before 20260925T070000Z)
  learners that never submitted a run: 1203 (603 anonymous, 600 under a pseudonym, 0 under a name), holding 600 handles
  handles claimed beside the identity of a learner that stays: 0
  staying: 600 learners (600 of them submitted a run), 400 handles, 600 registrations
dry run: nothing was changed
```

- **The report counts, it names nobody**: no handle and no learner id is printed, in a dry run or a real one.
- **Progress and cancellation.** Learners go in batches of 512, each batch one transaction and always a learner
  together with all of its handles; stderr reports every ten percent. Ctrl+C or `SIGTERM` stops between two batches:
  what was removed until then is removed for good (the file is still rewritten), the exit code is `130`, and running
  the same command again removes the rest.
- **Nothing to prune changes nothing**: the read models stay, and the next start does not rebuild them.
- **Backups are not rewritten**, as for `erase`.
- **When the cap was reached** (`roster-full`): stop, `prune` with an age that spares the real learners of the last
  days, start. If the junk is younger than every age you can spare, raise `PROCTOR_MAX_LEARNERS` for the time being —
  100000 registrations weigh 170 to 650 MiB — and prune a week later.

## Layout

| Path | Content |
|---|---|
| `🔨️modules/🗄️storage` | The four storage roles over one SQLite file: `AuthorityStore`, `ProjectionStore`, `BlobStore` and `SessionStore`; the online snapshot, its verification, the adoption of a copy, the removal of actors as one set and the rewrite that takes their bytes out of the file |
| `🔨️modules/📚️catalog` | Catalog loading and validation, SHA-256 quiz revisions and the catalog fingerprint |
| `🔨️modules/🎭️actors` | Handle and learner deciders, the command admission (shapes, targets, caps) and the enrollment saga |
| `🔨️modules/🔭️projections` | The checkpointed, strict projector, the leaderboards' transcripts and rank indexes (`Board`) and the registration count |
| `🔨️modules/👪️crowd` | The incremental per-quiz crowd tally and its `CrowdView` |
| `🔨️modules/❓️queries` | The six query handlers |
| `🔨️modules/🎚️config` | Environment configuration, production gating, the edge limits and the sign-up allowance |
| `🔨️modules/👥️presence` | The presence rooms of the catalog and the admission of the states shared in them |
| `🔨️modules/🧩️instance` | The `ServerInstance`, module, policy templates, the request gate (transport trust, CORS, preflight max-age), the wiring of the framework's throttle, route groups and command admission, presence wiring, the single-flight consistency middleware, and serving |
| `🔨️modules/⌨️cli` | `serve`, `check`, `rebuild` and the operator verbs `health`, `backup`, `restore`, `erase`, `prune` |
| `🏗️bootstrap` | Process entry (`🦀️.rs`) and the dev launcher that runs a private copy of the built binary (`🟦️.ts`) |
| `🧫️fixtures` | A two-quiz catalog the tests play (the heating systems carry a category without a profile for the hints without one; a familiar kettle and a few short labels for the hints to prefer and name) |
| `🧪️tests/🔬️conformance` | The framework storage laws, run against these stores |
| `🧪️tests/🌐️end-to-end` | The whole API over HTTP, including a restart, handles claimed and recalled, a run at every challenge (keys and hints on easy — far-off sorting keys and matching cards compared with another item by factor or difference, `under`, `over` or `reversed`, the familiar kettle preferred, three hints at most per task, short labels on the sheet and familiarity kept back, misplaced classification items asked about a profile axis beside an item rightly placed above or below, a pairing together or apart, or a category without a profile, none once the run is submitted or voided —, keys on medium, guesses and their misses in the result on hard, the clock on expert for every kind of task with `open-task`, `task-unopened`, `time-up` at explicit instants, `already-opened` without a further receipt and a partial submission, malformed and unsafe instants and malformed openings refused at admission, a voided run when the challenge changes and switching back and forth until `runs-exhausted`, points and bests on the learner view and the board), the leaderboard's shape, the cross-origin API behind the gate, presence, watching and thinking with real WebSocket clients, the crowd after submissions, the operator verbs run as the built binary (health, backup while serving, a proctor booted on the restored copy, an erasure and the freed handle, a pruning that leaves everybody who played), and the edge: idempotency keys, id admission, body limit, throttling per address, the sign-up allowance, route set and socket limits |
| `🧪️tests/🏋️capacity` | The capacity gate: a lecture hall of 300 from one address against the release binary under its default cap, alone and beside an abusive script that also tries to fill the roster |
