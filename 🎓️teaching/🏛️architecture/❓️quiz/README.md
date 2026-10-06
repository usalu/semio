# ❓️ Quizze · Architektur und Technologie

The quiz website of **quizze.architektur-und-technologie.de** and its proctor at **semio.iek.uni-hannover.de**: the
architecture catalog (`🔣️.json`, id `architecture`) with the quizzes it names, rendered by `@semio-tech/quiz-react` in a
static site on a CDN and proctored by `proctor` (`🎓️teaching/🛂️proctor`), an API-only server over one SQLite file that
runs as a zero-touch Docker stack. Both hosts, the image and the port live in one place, `🚀️deploy/🔣️.json`; the Vite
config, the operator verbs and the tests read them there.

| Path | What |
|---|---|
| `🔣️.json` | the catalog: introduction, quiz paths (relative to this file), badges |
| `🟦️.ts`, `🌐️.html` | the site entry: `mountQuiz(root, { proctor: <baked proctor origin or "">, tenant: "architecture", material, logo, legal: site.legal, pets: () => import("../🐾️pets/🟦️.ts")… })` — `logo` the emblem, `legal` the imprint and privacy links of `🚀️deploy/🔣️.json`, `pets` the menagerie loaded lazily (see [Pets](#pets)) |
| `📚️catalog/🟦️.ts` | the material the site hands the quiz: the catalog and the quizzes it names, imported statically in catalog order, so the page shows the catalog at once and goes on working [while the proctor is away](#while-the-proctor-is-away) |
| `🎨️.css` | the brand layer: the semio palette compiled to custom properties, its fonts shipped with the build |
| `🏗️builder/🌐️vite/🟦️.ts` | Vite configuration: host HTML, `CNAME`, aliases; `build` bakes `VITE_PROCTOR_URL`, seals the document with its Content-Security-Policy and emits `robots.txt` and `manifest.webmanifest`; `serve` proxies the gateway routes to the dev proctor |
| `📦️packages/🟦️typescript` | `@teaching/architecture-quiz`: `package.json`, `📋️project.json`, `📜️script.ts` |
| `🧪️tests/🧪️catalog/🟦️.ts` | the catalog and every quiz validate in the TS core and against the draft-07 contract (ajv); the material module ships exactly these documents; every badge for perfection asks for medium or harder |
| `🧪️tests/🧪️deploy/🟦️.ts` | `🚀️deploy/🔣️.json` holds to its schema (ajv); no deployment file drifts from it and seeded drifts are found; the Dockerfile, `compose.yaml` and the workflow (parsed with `yaml`), the Caddyfile, the CDN artifact check and the bundle for the proctor host (its `.env` and steps) hold what the runbook promises |
| `🧪️tests/🧱️local-stack/🟦️.ts` | waiting for a server, stopping an owned command, and the static origin of the rehearsal (against Vite's preview server) |
| `🧪️tests/🐾️pet-cast/🟦️.ts` | the pets fit the quizzes: every species and the ensemble of `../🐾️pets` validate (ajv and the pets product), grounds exist in the quiz files, every quiz and the home screen have a cast; every state is reachable and leads back, tricks and gear have their clips, the chemistry names what exists and looks and rests within sane bounds (see [Pets](#pets)) |
| `🧱️stack/🟦️.ts` | the local stack: `dev` (proctor and site in one terminal) and the parts the end-to-end gate boots its stacks from |
| `🎭️e2e/` | the end-to-end gate (`🟦️.ts`), its Playwright projects (`🎚️config`) and the learner the specs drive (`🚶️learner`) |
| `🚀️deploy/` | `🔣️.json` (hosts, image, port) and its contract (`🧬️schema/🔣️.json`), `Dockerfile` with its build context (`Dockerfile.dockerignore`), `compose.yaml`, `Caddyfile` and the operator verbs (`🟦️.ts`): `publish`, `docker-image-build`, `docker-image-check`, `docker-stack-check`, `docker-stack-bundle`, `docker-image-publish`, `deploy-check` |

## Challenges

A learner plays every run at one of four challenges, chosen on the quiz's page (the device remembers the choice; `medium`
at first). The catalog's introduction explains them to learners; the rules live in the quiz core (`CHALLENGE_RULES`), so
the proctor and the device decide alike, and the quiz files say nothing about them, so their revisions stay.

| Challenge | The keys | While playing | Points per quiz at most |
|---|---|---|---|
| easy | shown: the ladder of values a sorting is placed on, the cards of a matching, category descriptions and diagram numbers | beside a value placed farther off than its set's reach — on a logarithmic scale a factor of 1000, or half the orders of magnitude the values span where that is less; on a linear scale half their range — a question doubts the relation the learner's own keys claim between it and another item (preferably one placed right and familiar, the one whose relation is the most wrong), naming items by their short labels: a claim the wrong way round as "Are you sure “Burning tea light” is higher in power than “Kettle”?", amounts that add up as "Are you sure 10 × “Person sitting still” together only add up to the power of 1 × “Kettle”?" (the keys claim too little) or "Are you sure it takes 1,000 × “Burning tea light” to add up to the power of 1 × “Kettle”?" (too much), other quantities as "…only / really 1,000 times as high in U-value as…", linear ones as "…only / really 5 °C higher in … than…", always naming the quantity (German "in puncto U-Wert"), huge counts in words or as "1.1 × 10¹⁶"; beside a misplaced item of a classification: whether it is higher or lower than a standard placed right on one axis of the spider profile it was put into, or fits that profile with the axis at its value, else whether it and another item belong to the same or to different categories, else whether it belongs to the category it was put into, with that category's description. At most three hints per task, the most wrong first. A hint never states the truth and never says only "too high" or "how many" | 100 |
| medium | shown | nothing | 200 |
| hard | hidden: a typed estimate per value to sort or match; descriptions and diagram numbers gone | an estimate beyond the reach scores as a miss | 300 |
| expert | hidden, as hard | every task runs against its own clock from *Start the clock*; the run may be submitted with tasks unanswered, which score as misses | 400 |

A run earns its score times the points of its challenge; per quiz the run with the most points counts, and the
leaderboard sums those. Every badge of this catalog but `completionist` asks for a perfect result at medium or harder
(`"challenge": "medium"` on its rule), so the hints of easy earn none.

## Develop

Run everything from `.vscode/launch.json` (groups `3_dev` and `4_gate`):

| Launch row | Command | Port |
|---|---|---|
| `🛠️dev🎓️teaching🏛️architecture❓️quiz` | `bun nx run @teaching/architecture-quiz:dev` — backend and frontend together | `8791` and `6061` |
| `🛠️dev🎓️teaching🏛️architecture❓️quiz🌐️site` | `bun nx run @teaching/architecture-quiz:dev-site` — the site alone | `6061` (`TEACHING_ARCHITECTURE_QUIZ_PORT`) |
| `🛠️dev🎓️teaching🛂️proctor` | `bun nx run @teaching/proctor:dev` — the proctor alone | `8791` (`PROCTOR_PORT`) |
| `🧪️test🎓️teaching🏛️architecture❓️quiz` | `bun nx run @teaching/architecture-quiz:test` | |
| `🛠️dev🎓️teaching🏛️architecture❓️quiz🪁️typecheck` | `bun nx run @teaching/architecture-quiz:typecheck` — the entry, the Vite configuration, the stack, the deploy verbs, the end-to-end gate and every test and spec against the compiler; it first regenerates the git-ignored bindings its sources read | |
| `⚖️gate🎓️teaching🏛️architecture❓️quiz🎭️e2e` | `bun nx run @teaching/architecture-quiz:test-e2e` — the end-to-end gate | `6161`/`8891`, `6162`/`8892` |
| `✅️check🎓️teaching🏛️architecture❓️quiz📚️catalog` | `bun nx run @teaching/architecture-quiz:check` (`proctor check` on the catalog) | |
| `🔁️rebuild🎓️teaching🛂️proctor` | `bun nx run @teaching/proctor:rebuild` (refold the dev proctor's read models) | |

### One command for backend and frontend

`dev` (`🧱️stack/🟦️.ts`) runs the whole stack in one terminal:

1. A proctor that already answers `GET /instance` on `PROCTOR_PORT` is reused, and never stopped by this command; when
   it serves another quiz contract than the site (an older build, say), the command says so in one `[WARN]` line — the
   site then sends it nothing and keeps everything on the device — and stopping it and running `dev` again builds the
   current one. Otherwise the proctor is built (a first build takes minutes; a progress line follows every ten seconds) and launched
   with the defaults of `@teaching/proctor:dev`: the same data directory `.🧬semio/🎓️teaching/proctor-dev/` and this catalog.
   That directory is disposable development data. When it holds a database of a storage format this proctor does not read
   (the proctor refuses such a file, and there is no migration), `dev` and `@teaching/proctor:dev` move it aside to
   `.🧬semio/🎓️teaching/proctor-dev.v<format>-<time>/`, start with empty data and say so in one line; delete what was moved
   aside when you do not need it. A directory you named yourself with `PROCTOR_DATA` is never touched: the command ends
   at once with one line that names it and says how to go on. To reset development data at any time, stop the proctor and
   delete `.🧬semio/🎓️teaching/proctor-dev/`.
2. The command waits until the proctor answers ready. If the proctor exits first, the command fails and says so.
3. The site's dev server starts (`dev-site`). A site that already answers on its port is reused the same way.
4. The proctor this command launched is supervised, each step said in one `[stack]` line. Whenever a Rust source of a
   crate it is built from changes (cargo's own resolution of the teaching workspace names those crates), it is built
   anew while the running one serves and then launched in its place — a build that fails leaves the running one
   serving; whenever the catalog or one of its quizzes changes, it is launched anew; whenever it ends by itself, it is
   launched again after one second, then ever more patiently, at most every half minute. Every such swap is a short
   absence of the proctor, which the site tolerates like any other. `TEACHING_ARCHITECTURE_QUIZ_WATCH=off` stops the
   following (and the site's reloads).
5. One Ctrl+C, a termination or a closed terminal stops both, with every process they started. If the site exits, the
   proctor is stopped as well.

The dev site bakes no proctor origin and proxies `/instance`, `/commands`, `/queries`, `/actors` (WebSocket event streams
included) and `/scopes` to `http://127.0.0.1:${PROCTOR_PORT:-8791}`, so the browser talks to one origin in dev. A proctor
that goes away while a socket is proxied never takes the dev server along, also under Bun 1.3, whose sockets lack the
`destroySoon` the proxy ends them with (`semioServeUpgradeVitePlugin`).

### End-to-end gate

`test-e2e` (`🎭️e2e/🟦️.ts`) boots throw-away stacks and drives the real site in Chromium with Playwright. It never touches
the dev ports or the dev data.

| Topology | Site | Proctor |
|---|---|---|
| `dev` | the dev server on `6161`, one origin through the proxy | development mode on `8891` |
| `rehearsal` | the release build with `http://127.0.0.1:8892` baked in, served as static files on `6162` the way a CDN serves them | production mode on `8892`, granting exactly `http://127.0.0.1:6162` |

Both run by default, side by side. Name one to run only that one, and pass anything else on to `playwright test`:

```sh
bun nx run @teaching/architecture-quiz:test-e2e                                # both topologies
bun nx run @teaching/architecture-quiz:test-e2e -- rehearsal                   # the release rehearsal only
bun nx run @teaching/architecture-quiz:test-e2e -- dev --grep "leaderboard"    # one spec against the dev stack
bun nx run @teaching/architecture-quiz:test-e2e -- --serial --keep             # one after the other; keep the run directory
bun nx run @teaching/architecture-quiz:test-e2e -- --proctor <executable>      # run that proctor instead of building one
```

The release document's Content-Security-Policy is part of the rehearsal: its `connect-src` names the baked proctor origin,
and any violation fails the spec that caused it. The dev server of the `dev` topology has its own dependency cache and does
not watch the sources, so an edit during a run never reloads a page under test.

The specs live in `🧪️tests/` beside the node tests, one directory each, and share the learner of
`🎭️e2e/🚶️learner/🟦️.ts`. Every answer is computed from the quiz sources by item id, and a spec fails on any console
error, page error, failed request or policy violation it did not announce.

A page of the quiz keeps a processor core busy while a test drives it. One topology therefore runs with four Playwright
workers and two topologies at once with two each; `--workers <n>` overrides both. On a 16-thread machine both topologies
take seven minutes together, eleven beside other builds. A step that does not see what it expects fails after twenty
seconds; a whole test may take ten minutes.

| Spec | What a learner does |
|---|---|
| `🚀️site-boot` | the site loads and shows the introduction (runs first, so a cold dev server is warm for the others) |
| `🪪️first-visit` | introduction, then anonymous, pseudonym or name; a known pseudonym recalls its progress on a fresh device; switching identity asks first |
| `🥞️layered-home` | the nine cards around the leaderboard, the live screen-sized pages behind the glass that the mouse pans between the cards and that come to the screen, clear, on a card (on a device that reports reduced motion too), hash and Escape, and the navbar's ways — the overview, back and forward along the trail, up from a run to its quiz — around what the quizzes are about in its middle |
| `🎯️quiz-runs` | every quiz to a perfect score and every badge; one mistake per task for partial credit (the classification mistake is dragged with the mouse) |
| `⛰️challenge-levels` | physics at every challenge: easy with the keys on the ladder and, beside each value placed far off until it is fixed, a question asserted by its structure with items named by their short labels (extremes exchanged: the order question without a number, the larger first, against an item placed right; the two largest exchanged: "together only add up to" and "it takes", no long digit runs), exactly three on a sorting placed upside down, nothing general anywhere (100 points, no badge); easy on a matching (both exchanged cards asked for their order, on two quantities the one named, in German "in puncto Heizlast") and on spider profiles (above or below a standard placed right on the named axis, else the profile's value), in English and German; medium (200 points and the physics badges), hard with typed guesses (300 points; one guess far off costs score and shows as a miss), expert behind a clock per task driven past its deadline with Playwright's page clock (read-only, open tasks score nothing, points of 400) |
| `🏆️live-leaderboard` | rows ordered by total, arriving on a watching device without a reload; the same learners on today's, this week's and this month's leaderboard and on the one of the quiz they played; a column heading clicked to sort |
| `🗣️both-languages` | every screen, the document's language and title and the notice of what is stored in English and German, switched both ways; the language follows the browser and is asked for when the browser names neither |
| `🐕️pet-walk` | the pets: on the home screen within seconds, hidden from assistive technology and never a hit target of their own; a click says hello, then asks for a trick, then for a purr; picked up, a pet hangs and, let go high, opens its parachute and lands; circling a pet changes its state; no two pets overlap during a lively minute with drags; a control under a pet keeps its click; the settings' play group and the footer switch work by keyboard; the cast of a quiz on its page and in its run; `still` and `off`; reduced motion as the default only (still pets that answer no hand until the learner chooses, calm pets that walk afterwards) |
| `📱️phone` | 375 × 812: home as the same grid swiped page by page along both axes, one quiz played |
| `👥️shared-presence` | two devices: online count, cursors, what the other thinks, what everyone answered (runs alone) |
| `🔌️connection-shortage` | a reload mid-run; the proctor stopped and started again mid-run (runs alone) |
| `📴️proctor-away` | the proctor stopped before a learner ever arrives: a pseudonym taken, a quiz played to its score, feedback and badges, the result kept over a reload, all on the device; the proctor started again: the connection calms, a fresh device finds the same learner with the same result; a pseudonym the proctor already knew continues as its holder (runs alone) |
| `🛟️proctor-faults` | the proctor dies between every two tasks of an expert run and in the moment the run is submitted: full points, no notice, a fresh device finds the run; a proctor of an older contract (its `GET /instance` answered one version lower in the browser) is sent no command and no query while the device decides a whole run, one of a newer contract shows the page out of date with a reload, and once they agree everything arrives (runs alone) |

Each run keeps its stacks under the git-ignored `.🧬semio/🎓️teaching/architecture-quiz-e2e/<run>/` and deletes that
directory when it passes. A failed run keeps it — the proctor and site logs per topology, and `report/` with Playwright's
traces and screenshots — and prints its path. Chromium is taken from `PLAYWRIGHT_BROWSERS_PATH`, else from the repo's
tool cache, and is downloaded once if it is missing.

### Pets

The site hands the quiz its pets: `🟦️.ts` passes `pets: () => import("../🐾️pets/🟦️.ts")…` to `mountQuiz`, and the quiz
fetches that menagerie together with the render target (`@semio-tech/pets-react`) only for a learner whose preference is
not `off`. The menagerie is the architecture one ([`../🐾️pets`](../🐾️pets/README.md)): twenty species, each the
likeness of a thing the quiz items are about, their bonds, and one cast for the home screen and one per quiz, so the pets
on screen fit the topic. They stand on the top edges of the cards and on the floor of the window, keep clear of text and
controls, rest during a run and never take a click; the preferences offer `off`, `still`, `calm` (the default) and
`lively`, and a device that asks for reduced motion gets them motionless until its learner chooses — a choice made in
the preferences or with the switch on every screen holds whatever the device asks for.

| What | Where |
|---|---|
| species, ensemble (bonds, casts), roster and how to add a pet | `../🐾️pets/` and its README |
| the quiz's glue (preference, scene, lazy loading, mounting) | `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🐾️pets` |
| the model and the layer | `🧰️framework/🛍️products/🐾️pets` (`@semio-tech/pets`, `@semio-tech/pets-react`) |
| judging rigs and casts by eye | launch row `🛠️dev🎓️teaching🏛️architecture🐾️pets📖️stories` (Claude preview `architecture-pets-stories`), http://127.0.0.1:6074/ |

In a release build the pets are four lazy script chunks and one lazy stylesheet beside the entry (the menagerie, the
render target with the model, their shared constants, the half of the quiz's glue that comes with the pets — the layer
as the quiz sets it up and "Play with the pets" —, the paints), all content-hashed under `assets/` and loaded from
the site's own origin, so the document's Content-Security-Policy needs nothing new and the entry script budget is not
charged with them. `🧪️tests/🐾️pet-cast` holds the menagerie to the quiz files; `🧪️tests/🐕️pet-walk` drives the pets in
both topologies of the end-to-end gate.

## While the proctor is away

The site and the proctor are two deployments, and the site does not need the proctor to be there. It carries its material
(`📚️catalog/🟦️.ts`: the catalog and its quizzes, solutions included), so the quiz client has the proctor's own deciders
on the device (the deputy, see the quiz product's README, *While the proctor is away*):

| The proctor | What a learner gets |
|---|---|
| answers | everything is decided by the proctor, as before; the header says *All answers saved* |
| does not answer (not deployed yet, down, behind a firewall the learner is outside of, no network) | the catalog shows at once; a learner takes an identity, starts quizzes, answers, submits, sees the score, the feedback and the badges — decided and saved in the browser's storage. The header says *Quiz server not reachable – everything is saved on this device* (with the number of commands still to send), the leaderboard shows this device's own standing and says so, the others online and what they answered are not there |
| answers again | everything waiting is sent in the order it happened, each command once; the proctor decides it again and its views replace the device's (its clock, its score). A pseudonym or name the proctor already knew continues as its holder, and the learner is told |
| serves another quiz contract (an older deployment, a newer one the page predates) | it is sent nothing and treated as away, as above; a newer proctor shows *This page is older than the quiz server* with *Reload page*. Once the two agree — the proctor or the page was replaced — everything waiting arrives |
| dies again and again, also in the middle of a submission | nothing is lost and nothing is told: whatever it did not decide the device decides, and the proctor decides it again once it is back |

A learner waits at most three seconds for a proctor that says nothing before the device decides. What is saved on the
device stays there until the proctor has it: a learner who clears the browser's data before that loses it. The proctor
scores every run itself, so the leaderboard never trusts a device.

The gate proves it in a real browser in both topologies (`🧪️tests/📴️proctor-away`, `🧪️tests/🛟️proctor-faults`); the
unit proofs are in the quiz product (`🚶️learner-journey`, `🫡️deputy-decisions`, `📬️outbox-delivery`,
`🤝️wire-version`).

## Proof of concept by hand

The split deployment without GitHub and without a registry: two directories, each carried to its place by hand. Stage
them in this order — a build of the site empties `dist`, the bundle included:

```sh
bun nx run @teaching/architecture-quiz:publish                # 1. the site           → dist/pages/quizzes
bun nx run @teaching/architecture-quiz:docker-image-build     # 2. the proctor image (a first build takes minutes)
bun nx run @teaching/architecture-quiz:docker-stack-bundle    #    the stack with it  → dist/proctor
```

Launch rows: `🚚️publish🎓️teaching🏛️architecture❓️quiz`, `📦️build🎓️teaching🏛️architecture❓️quiz🐳️docker-image`,
`📦️bundle🎓️teaching🏛️architecture❓️quiz🐳️docker-stack`.

**1. The site.** Upload the content of `📦️packages/🟦️typescript/dist/pages/quizzes` to the static host or CDN that serves
`https://quizze.architektur-und-technologie.de` ⚠️. Any static host works: the app addresses its pages by hash, so no
rewrite rule is needed; `_headers`, `CNAME` and `.nojekyll` are honoured where the host knows them and ignored elsewhere.
The build talks to `https://semio.iek.uni-hannover.de` and to nothing else (its Content-Security-Policy says so), and
the proctor grants exactly the origin `https://quizze.architektur-und-technologie.de`: served under another name, the
site needs that origin added to `PROCTOR_ALLOWED_ORIGINS` in the `.env` of the stack.

**2. The proctor.** `docker-stack-bundle` stages everything `semio.iek.uni-hannover.de` needs in
`📦️packages/🟦️typescript/dist/proctor` (`--out <directory>` stages elsewhere, `--tag <tag>` bundles another image tag):

| File | What |
|---|---|
| `compose.yaml`, `Caddyfile` | the stack, copied out of the image (an image built from other stack files than this checkout's is refused) |
| `.env` | pins `PROCTOR_TAG` to the bundled image and lists the other overrides (`PROCTOR_HOST`, `PROCTOR_ALLOWED_ORIGINS`, `QUIZ_HTTP_PORT`, `QUIZ_HTTPS_PORT`) as comments |
| `certificates/` | empty; the place for a certificate of your own |
| `proctor-image.tar` | `docker save` of `ghcr.io/usalu/architecture-quiz-proctor:<tag>` |
| `README.txt` | the steps below |

Copy the directory to the machine ⚠️ (`scp -r dist/proctor <user>@semio.iek.uni-hannover.de:architecture-quiz`), then
there, in `~/architecture-quiz`, on `linux/amd64` with Docker Engine 25 or newer and its compose plugin 2.24 or newer:

1. `docker load --input proctor-image.tar`
2. The certificate, one of:
   - Let's Encrypt ⚠️, nothing to do: Caddy obtains and renews it once ports 80/tcp, 443/tcp and 443/udp of the machine are
     open from the internet.
   - Your own: one file `certificates/<any name>.pem` with the certificate and its full chain first and the private key
     after it, owned by root with mode `0600` (Caddy runs as root without any capability and does not start on a file
     it cannot read). Caddy serves it and orders none. After replacing it:
     `docker compose up --detach --force-recreate caddy`.
3. `docker compose up --detach --wait`
4. `curl --fail https://semio.iek.uni-hannover.de/instance` ⚠️ answers JSON that starts `{"id":"teaching-proctor"`.

`semio.iek.uni-hannover.de` resolves to 130.75.188.198, and on 2026-10-02 its ports 80 and 443 did not answer from
outside the university. Until the university firewall lets them in, Let's Encrypt cannot issue (supply a certificate
instead) and a learner outside the campus network does not reach the proctor at all, whatever the certificate.

The bundle was run on 2026-10-02 as the machine runs it (`docker load`, `docker compose up --detach --wait`, with
`PROCTOR_HOST=localhost` and Caddy's internal CA): `/instance` answered through Caddy, a preflight from
`https://quizze.architektur-und-technologie.de` was granted and one from a foreign origin was not. `docker-stack-check`
proves the supplied certificate: it mints one for `localhost` with another CA, puts it into `certificates/` and sees
Caddy serve exactly that one. Everything under [Deploy](#deploy) from *Update* on applies to this stack as it is; for an
update, stage a new bundle, take a backup, then repeat steps 1 and 3 in the same directory (your `.env` and
`certificates/` stay when you copy only the new `proctor-image.tar`, `compose.yaml` and `Caddyfile`).

## Deploy

Two artifacts, released independently by one manual workflow (`.github/workflows/architecture-quiz.yml`) — or carried
by hand, see [Proof of concept by hand](#proof-of-concept-by-hand):

| Artifact | Where it runs | Built and proven by |
|---|---|---|
| the site: static files with `index.html`, `404.html` (the same document, for deep links), hashed `assets/`, `robots.txt`, `manifest.webmanifest`, `CNAME`, `.nojekyll`, `_headers` | a static CDN at `https://quizze.architektur-und-technologie.de` (GitHub Pages) | `🚚️publish🎓️teaching🏛️architecture❓️quiz` (`bun nx run @teaching/architecture-quiz:publish`) |
| the proctor image `ghcr.io/usalu/architecture-quiz-proctor` as `sha-<commit>`, `<workspace version>` and `latest` | the university's Docker host at `https://semio.iek.uni-hannover.de`, behind Caddy | `📦️build…🐳️docker-image`, `⚖️gate…🐳️docker-image`, `⚖️gate…🐳️docker-stack`, `📦️bundle…🐳️docker-stack`, `🚚️publish…🐳️docker-image` |

`🚀️deploy/🔣️.json` is the one authored source of both hosts, the image repository and the port (contract:
`🚀️deploy/🧬️schema/🔣️.json`). The Dockerfile, `compose.yaml`, the `Caddyfile`, the workflow and this file repeat those
values because Docker, Caddy and GitHub read them there; `🧪️tests/🧪️deploy` and the readiness gate fail on any drift.

### Readiness gate

`⚖️gate🎓️teaching🏛️architecture❓️quiz🚀️deploy` (`bun nx run @teaching/architecture-quiz:deploy-check`) says whether the
checkout is ready to deploy. It needs a running Docker daemon, pushes and uploads nothing, announces every step with its
number, and stops at the first failure or at Ctrl+C (whatever a step created is removed):

1. nothing drifted from `🚀️deploy/🔣️.json` (hosts, image, port, pinned base images and actions, targets named in docs);
2. the sources pass the compiler (`typecheck`): the entry, the Vite configuration, the stack, the deploy verbs, the
   end-to-end gate and every test and spec;
3. the catalog and every quiz are valid in the Rust core (`proctor check`);
4. the site builds for the production proctor and is the CDN artifact: entry points, a document sealed by its
   Content-Security-Policy, hashed assets, no source map or development leftover, within the size budget
   (`QUIZ_SITE_BUDGET`: what the document links — the entry script 260 000 B and the stylesheet 60 000 B gzip — and
   4 MB on disk; a run with its results and the pets are lazy chunks the first screen does not wait for, so they are
   not charged: on 2026-10-03 the entry weighed 253 374 B, the run chunk 13 867 B, the results chunk 3 217 B and the
   quiz's half of the pets' glue 1 499 B);
5. the proctor image builds;
6. the image is what it claims (labels, unprivileged account, no shell, the stack files inside) and serves the API as the
   site meets it, read-only and without capabilities;
7. the stack works exactly as a host gets it: the two stack files copied out of the image, TLS through Caddy, the
   hardening Docker applied, a backup while serving, an update without a failed request, a restore, a supplied
   certificate served instead of Caddy's own;
8. the end-to-end gate (`test-e2e`): the release build in a real browser against a production-mode proctor.

It ends with `ready to deploy` and warns while `site.legal` in `🚀️deploy/🔣️.json` names no imprint or privacy notice.

### What only the owner can do, in order

Every command below was run on 2026-10-02 against Docker Desktop (Linux engine, amd64) with `PROCTOR_HOST=localhost`,
except the lines marked ⚠️, whose outward part (DNS, GitHub, the registry, a public certificate) cannot be rehearsed.

**0. The legal pages.** Put the two absolute `https://` URLs into `site.legal.imprint` and `site.legal.privacy` of
`🚀️deploy/🔣️.json` and commit; the footer links only what is set, the site must be built again afterwards (step 2.3),
and the readiness gate warns until both are there.

**1. DNS and the firewall.** Only the site name is in the owner's zone (at the provider of
`architektur-und-technologie.de`):

```
quizze.architektur-und-technologie.de.                                CNAME  usalu.github.io.
_github-pages-challenge-usalu.quizze.architektur-und-technologie.de.  TXT    <value shown by GitHub in step 2>
```

If that zone has `CAA` records, they must allow `letsencrypt.org` (GitHub Pages obtains its certificate there).

The proctor's name is the university's and exists: `semio.iek.uni-hannover.de` resolves to 130.75.188.198. What is
missing is the way in ⚠️: on 2026-10-02 ports 80 and 443 of that address did not answer from outside the university.
Ask the university's network operators to let 80/tcp, 443/tcp and 443/udp through to the machine. Without that, learners
outside the campus network do not reach the proctor, and Let's Encrypt cannot issue a certificate — then supply one
(step 3). A `CAA` record of `uni-hannover.de` that does not allow `letsencrypt.org` has the same effect.

**2. GitHub, once** ⚠️:

1. Account → Settings → Pages → *Add a domain* → `quizze.architektur-und-technologie.de`; create the `TXT` record it
   shows and press *Verify*. A verified domain cannot be claimed by another account's Pages site, and that host is the one
   origin the proctor grants.
2. Repository → Settings → Pages → *Source*: **GitHub Actions**; *Custom domain*: `quizze.architektur-und-technologie.de`;
   after the certificate is issued tick *Enforce HTTPS*. With this source the custom domain is this setting: the
   artifact's `CNAME` file is ignored, and dot files are not uploaded.
3. Repository → Actions → *architecture quiz* → *Run workflow* on branch `main` (both boxes ticked). The run is skipped on
   any other branch. The `site` job tests, builds, verifies and deploys the site; the `proctor` job builds the image,
   checks the image and the stack and only then pushes `sha-<commit>`, `<version>` and `latest`, printing the digest.
4. After the first push: profile → Packages → `architecture-quiz-proctor` → *Package settings* → *Change visibility* →
   **Public**, so the host pulls without credentials. (To keep it private instead, run `docker login ghcr.io` on the host
   once with a token that has `read:packages`.)

**3. The Docker host, once.** `semio.iek.uni-hannover.de`: a `linux/amd64` machine with Docker Engine 25 or newer and its
compose plugin (2.24 or newer), reachable on ports 80/tcp, 443/tcp and 443/udp. The whole footprint is the directory
`~/architecture-quiz` with two files, and both come out of the image:

```sh
mkdir -p ~/architecture-quiz && cd ~/architecture-quiz
docker pull ghcr.io/usalu/architecture-quiz-proctor:latest   # ⚠️ needs the published image
container=$(docker create ghcr.io/usalu/architecture-quiz-proctor:latest) && docker cp "$container:/srv/quiz/deploy/." . && docker rm --volumes "$container"
docker compose up --detach --wait
```

(Without the registry the same directory comes from `docker-stack-bundle`, image included: see
[Proof of concept by hand](#proof-of-concept-by-hand).)

`docker compose up` starts the proctor (unprivileged, read-only root, no capabilities, reachable by Caddy only) and, once
it is healthy, Caddy, which obtains and renews the certificate for the proctor host by itself ⚠️. Nothing is configured:
the image carries the production configuration. Overrides go into an `.env` file beside `compose.yaml`: `PROCTOR_HOST`,
`PROCTOR_TAG`, `PROCTOR_ALLOWED_ORIGINS`, `QUIZ_HTTP_PORT`, `QUIZ_HTTPS_PORT` (these five also from the environment), and
any other variable of the proctor — both services read that file as their environment.

**A certificate of your own.** Where Let's Encrypt cannot reach the machine, put the certificate into the directory
`certificates` beside `compose.yaml` (Docker creates it at the first `up`) before, or any time after, the first start:

```sh
cat fullchain.pem privkey.pem | sudo tee certificates/semio.pem > /dev/null   # the chain first, the private key after it
sudo chown root:root certificates/semio.pem && sudo chmod 600 certificates/semio.pem
docker compose up --detach --force-recreate caddy
```

The file name is free as long as it ends in `.pem`. Caddy loads every such file at its start; with a certificate that
names the proctor host it serves that one and orders none (its log says `skipping automatic certificate management
because one or more matching certificates are already loaded`); for a host without one it goes on obtaining its own.
Caddy runs as root without any capability, so it reads a file only as its owner or as everyone: a bundle it cannot read
(owned by another account with mode `0600`) stops Caddy at its start with `permission denied`, and `docker compose logs
caddy` names the file. Renewal is yours: replace the file before the certificate expires and recreate `caddy` again.
Rehearsed with the pinned Caddy image: a supplied bundle served for `localhost` (`docker-stack-check`) and for the
proctor host's own name, no order made; a file owned by root with `0600` read, one owned by another account refused with
`0600` and read with `0644` (in a Docker volume ⚠️ — Docker Desktop does not carry a host file's owner into a bind mount,
so the first two lines above are not rehearsed as written).

**4. Verify** ⚠️ (needs DNS, the open firewall and both deployments; run the `curl` lines from outside the university):

| Check | Expected |
|---|---|
| `curl --fail --silent https://semio.iek.uni-hannover.de/instance` | `200`, JSON starting `{"id":"teaching-proctor"` |
| `curl --silent --output /dev/null --write-out '%{http_code} %{redirect_url}\n' http://semio.iek.uni-hannover.de/instance` | `308 https://semio.iek.uni-hannover.de/instance` |
| `curl --silent --include --request OPTIONS --header 'Origin: https://quizze.architektur-und-technologie.de' --header 'Access-Control-Request-Method: POST' https://semio.iek.uni-hannover.de/queries` | `204` with `access-control-allow-origin: https://quizze.architektur-und-technologie.de` |
| `curl --silent --verbose --output /dev/null https://semio.iek.uni-hannover.de/instance 2>&1 \| grep -E 'issuer\|expire'` | the issuer you expect (Let's Encrypt, or the authority of the certificate you supplied) and an expiry date in the future |
| `docker compose ps` on the host | `proctor` and `caddy` both `running (healthy)` |
| `https://quizze.architektur-und-technologie.de` in a browser | the introduction, then a pseudonym, a quiz run and the leaderboard; the header shows *Connected*, the console is free of errors |
| `curl --head https://quizze.architektur-und-technologie.de/does-not-exist` | `404` with the site's document (GitHub Pages answers deep links with `404.html`) |

### Update

On the host, in `~/architecture-quiz`, after a workflow run pushed a new image:

```sh
docker compose exec -T proctor proctor backup - > "proctor-before-update-$(date +%Y-%m-%dT%H%M%S).sqlite"
docker compose images proctor
docker compose pull proctor   # ⚠️ needs the registry
container=$(docker create ghcr.io/usalu/architecture-quiz-proctor:latest) && docker cp "$container:/srv/quiz/deploy/." . && docker rm --volumes "$container"
docker compose up --detach --wait
```

The backup comes first: it is the only way back across a release that changed what the events mean. `docker compose images`
shows the image that was running, the third and fourth lines fetch the new image and its stack files, and `up` replaces
only what changed. The volumes stay. Without the registry, copy `proctor-image.tar`, `compose.yaml` and the `Caddyfile`
of a new bundle (`docker-stack-bundle`) into the directory and run `docker load --input proctor-image.tar` in place of
those two lines: `up` replaces the proctor because its tag now names another image. While the proctor container is replaced, Caddy holds requests for up to 20 seconds
instead of failing them, and the site keeps unsent answers in its outbox. At boot the proctor validates the catalog,
refolds its read models when the catalog or the projector changed (the health check waits up to five minutes for that)
and then listens. A quiz whose file changed gets a new revision: an open run on the old revision can no longer record
answers (`quiz-revised`) and is voided when it is submitted or started again; submitted results, badges and the
leaderboard stay. Run the workflow's `site` job for a new client; the two parts are independent.

`docker compose up --detach` alone never updates anything (`pull_policy: missing`); it only starts what is stopped — which
is what to run after a host reboot found the stack down.

### Rollback

Every release is also pushed as `sha-<first 12 digits of the commit>`, which is never moved:

```sh
PROCTOR_TAG=sha-0123456789ab docker compose up --detach --wait
```

(or `PROCTOR_TAG=sha-…` in `.env`, so that the choice survives the next `up`). If the newer release wrote events the older
one does not know, restore the backup taken before the update as well. A proctor that meets a database of another storage
format refuses to start and names both formats; the data is untouched.

### Backup and restore

The whole state is `proctor.sqlite` (with its write-ahead log beside it while the proctor runs) on the volume
`architecture-quiz_proctor-data`. Events are the truth; every read model in the file can be rebuilt from them.
`caddy-data` holds the ACME account and the certificates Caddy obtained and can be re-issued; a certificate you supplied
lives in the directory `certificates`, not in a volume — keep its source elsewhere too.

```sh
docker compose exec -T proctor proctor backup - > "proctor-$(date +%Y-%m-%dT%H%M%S).sqlite"
```

is safe while the proctor serves: one read transaction writes a whole, consistent database without blocking a learner.
Keep copies off the host. From another machine one line takes the backup and brings it home ⚠️ (needs the host):

```sh
ssh <user>@semio.iek.uni-hannover.de 'cd ~/architecture-quiz && docker compose exec -T proctor proctor backup -' > "proctor-$(date +%Y-%m-%dT%H%M%S).sqlite"
```

and a daily one on the host itself, kept for 30 days, is one `crontab -e` line:

```
17 3 * * * cd ~/architecture-quiz && mkdir -p backups && docker compose exec -T proctor proctor backup - > "backups/proctor-$(date +\%F).sqlite" && find backups -name 'proctor-*.sqlite' -mtime +30 -delete
```

A backup holds what the learners entered (pseudonyms, names); treat it like the database.

Restore replaces the database of a stopped proctor. The proctor refuses a file that is not a whole proctor database,
and refuses while a proctor serves the volume:

```sh
docker compose stop proctor
docker compose run --rm -T --no-deps proctor restore - < proctor-2026-10-02T093000.sqlite
docker compose up --detach --wait
```

The read models never need care: the proctor refolds them at boot whenever the catalog or its projector changed. To
force it (after editing the database by hand, say), stop the proctor and run its `rebuild` once:

```sh
docker compose stop proctor
docker compose run --rm --no-deps proctor rebuild
docker compose up --detach --wait
```

### Erasing a learner on request

The proctor removes one learner — the events, the handles they hold, their scores and every read model that names them
— from the database of a stopped proctor. `--handle "<pseudonym or name>"`, `--tag <the 8 digits on the leaderboard>`
or `--learner <id>` selects; the dry run prints what would go and changes nothing:

```sh
docker compose stop proctor
docker compose run --rm --no-deps proctor erase --handle "Ada Lovelace" --dry-run
docker compose run --rm --no-deps proctor erase --handle "Ada Lovelace"
docker compose up --detach --wait
```

Backups taken before still hold that learner: delete them, or take them again.

### Pruning registrations nobody played under

Signing up needs no password, so registrations collect that nobody ever plays under — visitors who looked and left, or
a script. They cost disk and count against the proctor's cap of registrations (`PROCTOR_MAX_LEARNERS`, 100000); once
that cap is reached, new learners are refused until it is raised or pruned. The proctor removes, from the database of
a stopped proctor, every learner that never submitted a run and registered longer ago than `--older-than` (a whole
number and `m`, `h`, `d` or `w`: `36h`, `7d`, `2w`) together with the handles it holds, and nobody who played. The dry
run prints the counts — never a name — and changes nothing; a real run reports its progress, and Ctrl+C stops it
between two batches (run it again for the rest):

```sh
docker compose stop proctor
docker compose run --rm --no-deps proctor prune --older-than 7d --dry-run
docker compose run --rm --no-deps proctor prune --older-than 7d
docker compose up --detach --wait
```

Once a month is plenty: one client address may register 1200 learners at once and 100 more per hour
(`PROCTOR_LIMIT_SIGNUPS_BURST`, `PROCTOR_LIMIT_SIGNUPS_PER_HOUR`), so a single address needs 41 days to reach the cap,
and 100000 registrations weigh 170 to 650 MiB on the volume.

### Logs and limits

`docker compose logs --since 1h proctor caddy` (add `--follow` to watch). Docker keeps at most 5 × 10 MB per service. The
proctor logs its start, its stop and errors; Caddy logs certificate management and proxy errors. There is no access log:
neither service records client addresses, so the stack stores no personal data besides what learners enter. To trace an
abuse, add `log` to the site block of the `Caddyfile` and `docker compose up --detach` — and decide first how long those
addresses may be kept. The proctor is limited to 2 CPUs, 1 GiB and 512 processes, Caddy to 1 CPU, 256 MiB and 256, so a
runaway process cannot take the host down. Its own request limits (per client address and in total), the sign-ups it
accepts per client address, its cap of registrations and the size of a request body are variables of the proctor
(`🎓️teaching/🛂️proctor/README.md`); set one in `.env` and both services follow.

Nothing watches the stack. Point an uptime monitor at `https://semio.iek.uni-hannover.de/instance` (expects `200`) —
one outside the university, so that it sees what a learner at home sees — and at the site, and look at the host's disk
now and then: the database only grows, except by an erasure or a pruning. With a certificate of your own, let the
monitor warn before it expires: nothing renews it.

### When the certificate cannot be issued

Caddy retries by itself; `docker compose logs caddy | grep -i -E 'obtain|acme|challenge|error'` says why it fails.

1. `dig +short semio.iek.uni-hannover.de` must print the host's address, 130.75.188.198 (and `AAAA` only an address the
   host really answers on: a wrong `AAAA` record fails the challenge for IPv6-first validators).
2. Ports 80 and 443 must reach Caddy from the internet: `curl --head http://semio.iek.uni-hannover.de/` from a network
   outside the university must answer `308` from Caddy, not time out (the university firewall, the host firewall). This
   is the expected obstacle for this host.
3. A `CAA` record that does not allow `letsencrypt.org` forbids the issuance.
4. After many failed attempts Let's Encrypt rate-limits the host name for up to an hour; fix the cause first, then wait.
   Never delete the `caddy-data` volume to "retry": it holds the account and every issued certificate.

When the cause cannot be removed, supply a certificate (step 3 of [What only the owner can do](#what-only-the-owner-can-do-in-order)):
with it Caddy stops ordering.

`PROCTOR_HOST=localhost QUIZ_HTTP_PORT=18080 QUIZ_HTTPS_PORT=18443 docker compose up --detach --wait` on any machine runs
the same stack with Caddy's internal certificate authority, which is what `docker-stack-check` does.

### Never

`docker compose down --volumes` deletes the database and the certificates Caddy obtained. `docker compose down` (without
the flag) only removes the containers and keeps both.

### Staging, other CDNs, other architectures

- The site bakes its proctor origin at build time: `PROCTOR_URL`, else the production proctor. A staging site on another
  origin needs both: `PROCTOR_URL` when building it (`publish` then flags the artifact as a rehearsal), and its origin
  added, comma-separated, to `PROCTOR_ALLOWED_ORIGINS` on the proctor host.
- GitHub Pages cannot send response headers and caches every file for ten minutes. The document therefore carries its
  Content-Security-Policy as a `meta` tag (scripts only from the site and by hash, connections only to the proctor), and
  a visitor holding the old document for those minutes may ask for an asset the new deployment no longer has; a reload
  fixes it. The staged `dist/pages/quizzes` also runs on CDNs that honour `_headers` (Cloudflare Pages, Netlify): there
  hashed assets are immutable, documents are revalidated, and every response forbids framing.
- The workflow publishes `linux/amd64`. On `linux/arm64` build the same Dockerfile on the host in a clone
  (`docker build -f 🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/Dockerfile -t ghcr.io/usalu/architecture-quiz-proctor .`):
  `compose.yaml` uses a local image as it is.
- With an `AAAA` record, check once that the proctor sees real client addresses over IPv6 (Docker's userland proxy can
  replace them with the bridge gateway, which would put every IPv6 learner into one rate-limit bucket):
  `docker compose logs proctor` after a request made with `curl -6`.
