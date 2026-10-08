# 🐾️ Pets

The semio pets product: small animated companions for any user interface. A pet is a rigged species — bones,
parts attached to them, a face, keyframed clips — that lives on a stage: it idles, blinks, follows the pointer
with its eyes, walks along the top edges of the interface, hops between them, and meets the other pets it
likes or squabbles with. The model is render-independent and deterministic: a stage is a pure fold of events,
and the same seed with the same events yields the same frames bit for bit in TypeScript (`@semio-tech/pets`)
and Rust (`semio-framework-pets`). The product knows nothing about what the pets stand for; a domain ships its
own menagerie (the architecture quizzes: `🎓️teaching/🏛️architecture/🐾️pets`) and a render target draws the
frames (`🎯️targets/⚛️react`, `@semio-tech/pets-react`).

## Layout

| Path | What lives there |
|---|---|
| `🧬️schema/🔣️.json` | The normative contract (JSON Schema draft-07): menageries, species, ensembles, the stage, its events and its frames. |
| `🧬️schema/🟦️.ts`, `🧬️schema/🦀️.rs` | Hand-written type twins, one type per `$defs` entry under the same name. |
| `🔨️modules/📐️trigonometry/` | `sinTurns`, `cosTurns` from fixed polynomials, `clamp`, `lerp`, `smoothstep`: the only sine and cosine the cores use. |
| `🔨️modules/🎲️randomness/` | Counter-based draws: `randomWords(key, count)` (numpy's seed sequence), `unitOf`, `randomUnit`, `randomBetween`, `weightedIndex`, `randomPick`, and the four reserved streams (`STAGE_STREAM`, `CAST_STREAM`, `ROTATION_STREAM`, `CHEMISTRY_STREAM`). |
| `🔨️modules/🦴️rig/` | 2×3 affine matrices, `restPose`, `solveRig` (forward kinematics), `lookOffset` (where a pupil looks) and `pupilReach` (how far it travels). |
| `🔨️modules/🎞️animation/` | `easeBezier`, `sampleTrack`, `sampleClip`, `blendPose`, the gaze spring `springStep` and the blink `lidAt`. |
| `🔨️modules/🏞️terrain/` | `perchesOf` (surfaces minus keep-outs), strides, falls, landings and the ballistic hop between perches; `wallsOf` (the free stretches of the side walls, by the keep-outs beside them), `wallAt`, `nearestWall` and the line of sight `segmentHits`/`segmentClear` (Liang–Barsky). |
| `🔨️modules/🧗️climbing/` | Getting to a perch out of a hop's reach: holds on walls (`gripFor`), the climb, grip, slide and mantle; wall lines across the gaps between the cards of a column (`crossable`, `chainOf`, the lunge `lungePath`) and the whole way along one (`wallPath`, `wallCost`); ladders raised against a rim (`ladderFor`), climbed rung by rung and toppled by surveys (`ladderHolds`); the grappling rope (`shotFor`, `hookStep`, the straight `zipStep` and the swinging `swayStep`); and `routeOf`, the ways an actor's gear opens between two perches. |
| `🔨️modules/🧠️behavior/` | Mode limits, activity weights (a trick on a whim among them) and dwells, the activity graph, casts, encounters, affinity and rapport, needs. |
| `🔨️modules/🎪️stage/` | The façade of the stage: `openStage(seed)`, `advance(menagerie, stage, events)`, `frameOf(menagerie, stage)` and the normative order of a tick. The ten modules below are its parts; the packages export none of them. |
| `🔨️modules/📝️draft/` | The working copy a fold writes to (the actors beside their species and streams), lookups, draw keys and the smallest changes of an actor. |
| `🔨️modules/📏️spacing/` | The gaps between grounded actors of one surface: free places, the clear way of a walker, who is in the way, rooms for newcomers, hop arcs clear of keep-outs. |
| `🔨️modules/🗓️schedule/` | The whole seconds on which a pair may be drawn or somebody waiting off stage may arrive, the beats on which moods travel and the chemistry acts, the next tick on which mischief does something (`prankTick`, with the pets that could play, `prospectsOf`), and whether the gestures of the pointer must see the next tick. |
| `🔨️modules/👀️attention/` | Presence under the pointer, turning round, the gaze spring with the manners of the mood, blinks, perking up, the lean and the squeeze of a turn; the gestures of the pointer round every actor (stroke, circle, countercircle) and what they cue; the feelings of the hand (lifted, dangled, shaken, dropped, landed). |
| `🔨️modules/🚶️locomotion/` | Walking, hopping and gliding between perches, falling, landing (the side and top edges of the stage are walls a flier bounces off; at the bottom edge it is gone), being crowded out, waiting for a partner, leaving; trips with gear — planned whole and claimed (`outingsTo`, `setOut`, `embark`), travelled foothold by foothold (`travel`), given up where they are (`halt`) — and resting on a wall or a ladder (`cling`). |
| `🔨️modules/💞️sociability/` | Pairing, approach, encounters as moods tip them (and as the chemistry promised them), showing off a trick, sulks and mending, rapport, moods travelling between neighbours, the click and the deed of the learner. |
| `🔨️modules/🎯️choice/` | What an idle actor chooses next (moods weigh in, a trick on a whim; a pet with gear sets out for a friend's perch, for more room or for a rest on a wall instead of a hop or a walk), how every activity ends (a trick leaves its state behind), the beat on which moods travel and the chemistry of the menagerie acts, and mischief with the page (the prank picked, begun, put back, ended, and the throw-off on `reclaimed`). |
| `🔨️modules/👥️population/` | Surveys (perches and pitches, riding, seating; climbers and ladders follow their walls or throw their riders off), arrivals, spreading out, summons, freezing and thawing. |
| `🔨️modules/🕰️clock/` | One tick in the normative order, the lulls that are jumped over, time passing. |
| `🔨️modules/🎥️projection/` | Poses (idle loop, the look of a state, activity clips, the posture of a mood), faces, tilts, tools, bodies, particles, ladders, the lifted copy, dust, the rate all of it needs and `frameOf`. |
| `🔨️modules/✅️validation/` | `menagerieIssues`, `speciesIssues`, `ensembleIssues` without any schema library, and `assembleMenagerie`. |
| `🔨️modules/<module>/🧪️tests/🔬️unit/` | The unit suite of a module: `🟦️.ts` (vitest, found by a glob) and `🦀️.rs` (attached with `#[path]`). |
| `📦️packages/🟦️typescript/` | `@semio-tech/pets`: package glue only, a barrel over `🧬️schema` and `🔨️modules`. Zero runtime imports. |
| `📦️packages/🦀️rust/` | Crate `semio-framework-pets` (lib `pets`), nx `@semio-tech/pets-rs`: `#[path]` glue over the Rust twins. |
| `🎯️targets/⚛️react/` | `@semio-tech/pets-react`: the inline SVG depiction, the DOM survey, the pacer, the decorative pet layer and the stories gallery. |
| `🧪️tests/🎚️config/🟦️.ts` | Vitest configuration of the TypeScript core; its `include` is a glob over every module's unit suite. It also tells the suites how much to sample at the test level of the run (`sampled(few, full, more)`). |
| `🧪️tests/<case>/` | The language-agnostic Protocol v2 cases (`🥒️.feature` with one adapter per language) and the suites of the React target (`🟦️.tsx`). |
| `🧫️fixtures/<case>/🔣️.json` | Shared vectors — inputs with expected outputs — generated from the oracle adapters, never edited by hand. |
| `🔮️oracles/🔣️.json` | The owner oracle registry: the third-party references the cases compare against, and the one recorded no-oracle decision. |

## Commands

Everything runs through each package's `📜️script.ts`; `📋️project.json` registers the same entry points as nx
targets.

```bash
cd "🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript"
bun ./📜️script.ts test                        # the unit suites of the TypeScript core: every assertion, a few samples per sweep
bun ./📜️script.ts test quick                  # every sweep and statistical session in full (`long` runs the same amount)
bun ./📜️script.ts test exhaustive             # more seeds, finer grids, longer sessions, and coverage

bun nx run @semio-tech/pets:test              # the same through nx
bun nx run @semio-tech/pets:typecheck         # the core, its suites, adapters
bun nx run @semio-tech/pets-rs:test           # the Rust core
bun nx run @semio-tech/pets-react:test        # the React target
bun nx run @semio-tech/pets-react:typecheck   # its types
bun run dashboard run @semio-tech/pets-react:dev --detach --wait-ready   # the stories gallery (prints its URL)

cd "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test"
bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/🐾️pets"
bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/🐾️pets"                                 # every case
bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🧬️schema-conformance"    # one case, by its directory name
```

## Domain model

### Menagerie, ensemble, species

A **menagerie** (`semio.pets.menagerie/v1`) holds every species of a domain, their bonds, the casts of its
scenes and the chemistry between its species. It is authored as an **ensemble** (`semio.pets.ensemble/v1`): the
same head, bonds, casts and chemistry, with every species in a document of its own that the ensemble names by a
path relative to itself.
`assembleMenagerie(ensemble, species)` takes the species documents in the order of those paths and yields the
menagerie; the `$schema` hints authored documents may carry stay behind. A menagerie reaches the browser as
statically imported data; nothing is fetched at run time.

A **species** has an id, a `name` and the `thing` it stands for (both in every language; there is no default
language), the `grounds` that tie it to its domain (free strings the domain interprets), a `size`, a `palette`
of three colours (ink and paper come from the theme), its rig, its motions, a `locomotion` and a `temperament`;
then what it can become and do — `states`, `tricks`, a `purr` and `emitters` —, the `gear` it owns, its `grip`
and `reach`, an optional `canopy` and its resting `mood` (see "States, tricks, purr and emitters" and "Gear,
grip and reach" below).

### Rig

Units are CSS pixels at scale 1. The feet stand on the origin, the body rises towards negative y, and a
species faces right at rest; the stage mirrors it.

- **Bones** are listed parents first; exactly the first bone has no parent. A bone has a rest offset and a rest
  rotation in degrees (clockwise on screen) relative to its parent.
- **Parts** are drawn in list order, back to front, each rigidly attached to one bone: a `path`, an `ellipse`,
  a `rect` or a `line` in the bone's own coordinates, with a `fill` and a `stroke` paint (`body`, `accent`,
  `detail`, `ink`, `paper`, `none`) and an optional `strokeWidth` (2 by default).
- The **face** has eyes (a white of `radius` and a pupil of radius `pupil` that follows what the actor looks
  at) and an optional mouth that bends with the mood. It is drawn above the part named by `above`, above every
  part when absent.

`solveRig(species, pose)` yields six numbers per bone in rig order — the SVG `matrix(a b c d e f)` of the bone
relative to the feet — from `translate(rest + pose offset) × rotate(rest + pose rotation) × scale(pose scale)`
composed down the chain.

### Clips and repertoire

A **clip** lasts `seconds` and is played once or looped. Each **track** animates one channel of one bone —
`x`, `y` (offsets in pixels), `rotation` (an offset in degrees), `scaleX`, `scaleY` (factors, rest = 1) — with
at least two **keys** in strictly ascending phase `at`, the first at 0 and the last at 1. A key may carry an
`ease`, the control points `[x1, y1, x2, y2]` of a CSS `cubic-bezier()` towards the next key; without one the
way is linear. In a looping clip every track ends on the value it starts with; on the `rotation` channel it
ends on the same angle modulo 360°, so 0 → 360 is a seamless spin.

The **repertoire** maps activities to clip ids; one of several is drawn, none leaves the body at rest. There
are twenty-six activities, in an order that never changes because new ones are appended: the life on a perch
(`idle`, `fidget`, `walk`, `hop`, `fall`, `land`, `sleep`, `greet`, `cuddle`, `squabble`, `sulk`), then `hang`
(held by the learner), `tumble` (thrown), `glide` (under a parachute), `aim` and `reel` (a grappling line),
`climb`, `mantle` and `slide` (a wall or a ladder), `carry` (a ladder), `trick`, `purr`, `dizzy`, `shrug`,
`scoot` (making room for another pet) and `push` (a fixture of the page). The **locomotion** names the gait
(`walk`, `hop`, `float`), the speed in pixels per second and, exactly when the gait is `float`, the `hover`
height above the perch. A walker needs a `walk` clip, a hopper a `hop` clip, and every species a clip for
`hang`, `tumble`, `purr`, `dizzy`, `shrug` and `push`: whatever its nature, a learner can pick it up, throw it,
pet it and shake it.

### States, tricks, purr and emitters

- A **state** is a lasting condition of a species, such as shining strongly: an id, a name in every language
  and, optionally, a `tint` (colours painted over the palette; a colour it leaves out stays as authored), a
  `clip` looped on top of the idle loop and an `emitter` that runs while the state lasts. A species has at
  least one state and the first is its resting state. A state with `lasts` (seconds) gives way to the state
  `then` when that time is over, and must name it.
- A **trick** is a clip played once, set off by any of its `cues` — `click`, `circle` and `countercircle` (the
  pointer circling the pet), `stroke`, `shake` (while it is held), `whim` (its own idea) and `show` (showing
  off to another pet); a trick without cues is only ever asked for by chemistry. It may carry an `emitter`, is
  on offer in the states `from` (in every state when absent) and may leave the species in the state `to` and
  in a `mood`. A ladder of states is written as tricks: `circle` tricks that lead one state up, `countercircle`
  tricks that lead one down.
- The **purr** is every species' contentment: a looping `clip` and optionally an `emitter`.
- An **emitter** is a source of particles on a bone, `x` and `y` away from it: every particle is drawn as
  `shape` around its own origin and painted like a part (`fill`, `stroke`, `strokeWidth`); `motion` is `fall`,
  `rise`, `burst`, `orbit` or `drift`; at most `count` (1…32, whole) are alive at once, each lives `life`
  seconds and leaves at `speed` pixels per second in directions that scatter over `spread` of a full turn
  (0…1).
- The **mood** of a species is the one it rests in: `content`, `happy`, `playful`, `curious`, `proud`,
  `sleepy`, `grumpy`, `sad` or `scared`.

### Gear, grip and reach

`gear` lists what a species owns to get around, each at most once: `climb` (it climbs walls), `ladder`,
`grapple` and `parachute`. Gear brings activities and the species needs a clip for each: `climb` → `climb`,
`mantle`, `slide`; `ladder` → `carry`, `climb`; `grapple` → `aim`, `reel`; `parachute` → `glide`. A floater
changes lanes instead of climbing: it owns a parachute or nothing. `canopy` is the shape drawn as the
parachute (a plain one when absent). `grip` is how high above its feet a species is gripped at the scruff
(above 0, at most its height) and `reach` how far its encounter poses lean out (at least 0, at most its
width): two pets that meet keep the sum of their reaches apart.

### Bonds and casts

A **bond** joins two different species with an affinity from −1 (they squabble) to 1 (they adore each other);
unlisted pairs are neutral, and each unordered pair is listed at most once. A **cast** names the species of one
scene: the `core` (at least one) and the `rotation`. Scenes are unique within a menagerie.

`castOf(cast, capacity, epoch, seed)` is who is on stage. Whenever a rotation exists and the stage holds at
least two, one place belongs to a visitor from the rotation; the core has the others — as a whole when it fits,
in turn when it does not — and places the core leaves empty go to further visitors. Core and rotation are two
rings, each read from a place drawn from the seed (`CAST_STREAM`) and moved on by one place per epoch: a new
epoch swaps one member of a ring, everybody gets a turn, and the same seed and epoch always name the same
troupe. A species is listed once; the core wins.

### Chemistry

`chemistry` is a list of **reactions** with unique ids: when an actor that matches the trait `when` is within
`within` pixels of one that matches the trait `near` — seen from the second `where` says (`above`, `below`,
`beside`, or `any` and absent for anywhere) — the effects of `then` (at least one) happen, at most once in
`every` seconds and with the probability `chance` (always when absent). A reaction may hold back while a third
actor that matches the trait `unless` stands within `within` pixels of the second, and may ask the affinity of the
two (the authored bond moved by their rapport) to lie within `affinity` (`[low, high]`, −1…1). A **trait** names a
species of the menagerie — or none: then any actor matches — and may narrow it to a `state` (of that species, of
some species when it names none) held for at least `held` seconds, a `mood` (the one an actor shows), an
`activity` and a `trick` it is performing. An **effect** acts `on` one side (`when` or `near`): it puts it into a
`state` of its species, gives it a `mood` of intensity `amount` (0…1), shifts the rapport of the two by `rapport`
(−1…1), makes their next encounter a `greet`, `cuddle` or `squabble`, has it perform a `trick` of its species, or
sets it off on an `activity` it could start by itself (`fidget`, `walk`, `hop`, `sleep`).

### Stage, events, frames

`openStage(seed)` is an empty stage. `advance(menagerie, stage, events)` folds events into it —
`ticked`, `pointed`, `unpointed`, `glanced`, `surveyed`, `summoned`, `tuned`, `hushed`, `poked` — and
`frameOf(menagerie, stage)` projects what a render target draws: the actors back to front with a matrix per
bone, their eyes and mood, what they are doing (`activity`), the rate the motion needs (64, 32, 16 or 0 ticks
per second) and the tick of the next change when nothing moves. Time is whole ticks, 64 per second.

Per actor the frame also carries its footing, the state of its species (whose clip overrides the idle loop on
every channel it keys and blends in from the former state's), the mood it feels with its intensity and its
spirits (the bend of the mouth; the mood also sets the resting height of the lids and lets the head sink or
lift), the tilt of its drawing about its pivot (`x`, `y` place the rig so that the turn keeps the feet where the
stage has them), its tools (parachute, rope, hook, gun, carried ladder) and its solid body; beside the actors the
ladders that stand, the particles of every plume (the youngest 160), the lifted copy of a fixture, the dust where
a pet vanished and the pet in the learner's hand. All of it is a pure function of the stage at its tick.

What an actor does with the pointer, all of it inside the fold:

- **Eyes.** The gaze follows the pointer through a spring; a pupil travels `pupilReach(eye)` — its white's
  radius minus its own and a quarter pixel for the outline — so it reaches the rim of the white.
- **Turning.** A grounded actor that is not walking turns to face a pointer that is clearly on its other side
  (a dead zone around its middle, a short rest between turns). It faces the new way at once (`Actor.facing`)
  and the drawing sweeps round over the eight ticks before `Actor.faced`; a turn can be reversed midway.
- **Leaning.** The bone that carries the first eye leans a few degrees towards what the actor looks at, and
  nods up or down with it.
- **Perking up.** A pointer that lingers close beside an idle actor makes a curious one greet it; how soon it
  does so again depends on `temperament.curiosity`.

In mode `still` none of this happens. While the stage is quiet (`hushed`) the pupils still follow, and nothing
else does: no turn, no lean, no greeting, and a poke is ignored.

Where actors appear: a newcomer takes the least crowded perch, the ground (the lowest perches of the stage)
last, so a cast spreads over the surfaces instead of lining up on the floor. When a survey brings new ground,
whoever shares a perch moves to one that holds nobody, and an actor whose surface vanished in that same survey
is gone with it and arrives anew instead of falling in front of what is there now.

Mischief with the page (`🎯️choice` `mischief`, the horizon `prankTick` of `🗓️schedule`): a host marks elements
with a key among the grounds (`<quiz>/<task>/…`), the survey reports them as fixtures, and once the gates of
`🪄️mischief` open — mischief permitted (the render target permits it only where the pointer is fine), a stage at
least 1024 px wide that is not still, the learner still for 12 s (30 s while quiet; an input, a move of the pointer
and a scroll all count), the mode's cooldown since the last prank (a stage that just opened rests a cooldown too),
none under way — the stage picks one pet whose ground covers a key and that stands idle where it can reach a post
beside the element, by a draw of its own stream (`MISCHIEF_STREAM`) and nothing else. The pet goes there on a trip
(over the rim and down the card's side to the element's lower edge, or along its perch — never past another — to a
perch beside it) or, where its gear has no clear way (cards packed without room on top), slips over in a puff and
fades in at its post; it pushes for the whole lift (`liftAt`: the copy leaves its stack by up to its width and slides
home within 10.75 s), and is playful afterwards. When the learner takes the element back (`reclaimed`, which the
render target tells the stage before the survey of the same frame) the copy is gone at once and the pet is
thrown off — it tumbles, opens its parachute where it would land hard, is frightened and, once down and calm again,
sheepish. A prank ends quietly when its element goes or moves, mischief is withdrawn or the stage turns still, and
the copy slides home by itself when its pusher is taken away. Nothing but a fixture's key and box is ever read.

### Determinism

The cores use only `+ − × ÷`, square roots, `abs`, `floor`, `min`, `max`, comparisons and 32-bit integer
operations, in the same order in both languages; sine and cosine come from `📐️trigonometry`. A draw is a pure
function of the key `[seed, stream, counter]`, so the order in which actors are processed never changes what
they draw. The cores never read a clock, the environment or a platform random source.

### State classes

| Class | Held where | What |
|---|---|---|
| Persisted local-only | the host application | whether pets are shown and how lively (`off`, `still`, `calm`, `lively`) |
| Ephemeral local-only | the render target | the stage: actors, perches, rapport; every device simulates its own pets |
| Ephemeral shared (read only) | the host application | other people's cursors, fed to the stage as `glanced` points |
| Persisted shared | — | none: pets never travel over the wire |

### The sample menagerie

`🧫️fixtures/🧬️schema-conformance/🔣️.json` carries a small complete menagerie that the stories gallery and
several suites load: `menagerie` (three species — the walker `blobby`, the hopper `hoppy`, the floater
`floaty` — with three bonds, the casts of the scenes `home` and `meadow` and six reactions), `ensemble` (its
authoring form), `species` (`{ path, document }` per species in ensemble order, with their `$schema` hints)
and `bases` (the smallest valid species, menagerie and ensemble). `blobby` has a clip for every activity, a
ladder of three states (`resting`, `glowing`, `radiant`, the upper two with tints, an emitter and a time after
which they give way), six tricks that together use every cue but `stroke` (which `hoppy` uses) and lead up and
down that ladder, two emitters, a purr with an emitter, every gear and a canopy; `hoppy` has two states, two
tricks, a grapple and a parachute and rests `playful`; `floaty` has only its resting state, a parachute and
rests `sleepy`. The vectors beside them are `accepted`, `structural` and `rules`; the file is generated by the
ticket script named in the case's feature.

## React target

`@semio-tech/pets-react` draws a cast on a web page. `<PetLayer>` renders one static `<div class="pet-layer"
aria-hidden="true">` over the viewport; a single effect then opens a stage, feeds it what it sees of the page and
paints every frame into one inline `<svg class="pet" data-pet="<species>" data-pet-activity="<activity>"
data-pet-footing="<footing>" data-pet-state="<state>" data-pet-mood="<mood>">` per actor (each attribute written only
when it changes; tests and tools read them). React never renders per frame. A drawing holds one group per run of
parts on the same bone, and a frame rewrites a bone, a pupil, a lid or the mouth only once it moved by `PAINT_SHIFT`
(0.05 px) or `PAINT_LINEAR` (0.002 of a matrix entry) since its last write: every written attribute costs the host
page a style recalculation, a repaint and a raster, and most frames of a breathing pet move it by less. The survey
never looks into the layer itself.

| Prop | Default | What |
|---|---|---|
| `menagerie` | — | the species, bonds and casts; one with validation issues is never shown |
| `scene` | — | the scene whose cast is on stage; an unknown scene falls back to `home`, then to nobody |
| `mode` | — | `still`, `calm` or `lively` (`off` is not rendering the layer) |
| `quiet` | `false` | a time of concentration: no rotation, no turn, lean or greeting towards the pointer; clicks and picking up still work |
| `play` | `true` | the learner lets the pets be played with: they answer clicks and can be picked up (handed to the stage as `permitted`) |
| `mischief` | `true` | the learner lets the pets play with the page (handed to the stage as `permitted`, and only while the primary pointer is fine, `PET_FINE_POINTER`) |
| `capacity` | 2 below 768 px, 4 below 1024 px, else 6 | the most actors at once |
| `surfaces` | `[data-pet-surface]` | selector of the elements whose top edge carries pets; the bottom edge of the layer is always the floor |
| `keepouts` | `PET_KEEPOUTS` | selector of what pets never cover: controls (`PET_CONTROLS`), what a person reads (`PET_TEXTS`) and `[data-pet-keepout]` — the mark for text that lies bare in a `div` or a `span` |
| `controls` | none | selector of more controls a press on a pet never takes away from, besides `PET_PRESS_CONTROLS` (controls and labels) — a host's drag grips, for instance |
| `walls` | none | selector of more elements whose sides pets may climb, besides the sides of the surfaces' own elements |
| `props` | `[data-pet-prop]` | selector of the elements a pet may play with (their key is the value of `data-pet-prop`) |
| `ref` | none | receives `{ play(species, deed) }`: a deed (`hello`, `trick`, `pet`, `toss`) the learner asked a pet for without a pointer — the keyboard equivalent of the hand for a "Play with the pets" group |
| `glances` | none | other things worth a look, in viewport pixels, sampled with every survey |
| `scale` | 0.8 below 768 px, else 1 | the size pets are drawn at |
| `seed` | random per mount | the seed of the stage |
| `zIndex` | 35 | above cards, below dialogs |
| `onCast` | none | told which species are on stage, in menagerie order, whenever that changes (nobody when the show ends) |
| `tempo` | 1 | how fast the pets' time passes, held between ⅛ and 8; for tools and tests that must see in seconds what takes minutes |

The layer is decoration and keeps out of the way: it is hidden from assistive technology and holds nothing that
takes focus; it takes no pointer events (the stylesheet says so, and a running show repeats it on the element), but
for the touch pads below; it never changes layout or scroll and makes no request, no sound and no console output.

The learner's hand (`🤏️grasp`) reaches the pets through capture-phase listeners on the window, ahead of the page. A
primary press counts for the pets only while play is permitted and the stage is not still, nothing interactive lies
under the pointer (`PET_PRESS_CONTROLS` and the host's `controls`: a control always wins), no text is selected, no
modifier key is held, and the point lies in a visible pet's solid box (`ActorFrame.body`) of the last frame. That
press alone is kept from the page — cancelled (no selection, no focus, no `mousedown`) and stopped —, the pointer is
captured on the document element and the click it leads to is swallowed; the stage gets `pressed`, the latest
`dragged` of every frame, `released`, or `cancelled` (Escape, which is then kept from the page too, a cancelled or
lost pointer, a window that loses focus, a hidden document, an inert layer, play switched off, a still stage, the
layer's end). Whether a press was a click, a hold or a pick-up is the stage's to decide. The document element shows
`data-pet-cursor="grab"` while the pointer rests on a pet that can be picked up and `grabbing` while the frame holds
one (`Frame.held`); no rule names that attribute, since a browser restyles the whole page whenever an attribute a
descendant rule names changes: the grab cursor sits inline on the one element under the pointer (and is given back
exactly), and `data-pet-held` marks the root while a pet is held, for the stylesheet's grabbing hand. Every key,
press, wheel and input becomes `stirred` (at most once a second), every scroll `scrolled`; the pointer feed tells the stage whether the pointer is `over` a control or free space (also `control`
while a press the pets did not take, such as a text selection, is under way). On a device whose primary pointer is
coarse, while play is permitted, a `div.pet-pad` over every grounded pet (`pointer-events: auto; touch-action: none`)
lets a finger drag the pet instead of panning the page; taps work everywhere without them. The survey also reports
the sides of the surfaces' elements as `walls` (`wallId`: the surface id and the side), where a scrolling ancestor
does not cut them away. Frames are written by script
and show as written: nothing in the layer transitions, whatever the host's stylesheet says (`transition-property: none
!important` on the layer and everything in it) — a host that shortens every transition for a device that asks for
reduced motion would otherwise start one with every write of a frame. Besides the keep-outs of the
selector, every surface is solid (pets stand on a thing, never in front of another one) and the focused element
keeps `FOCUS_MARGIN` (8 px) free around itself. A pet stays whole under a pointer that rests on it — it perks up
instead (design-v2 §17); it turns see-through only while it is in the air, on a wall, a ladder or a rope in front of a
control or of text the page keeps free, and is whole again once it is past it. Nothing runs
while the document is hidden, while the layer is inert or hidden and under forced colours (the layer stays empty);
in print the layer is not shown. A fault inside the show ends the show silently.

Time comes from the pacer (`createPacer`): frames at the rate the stage asks for — 64, 32 or 16 ticks per second —
and a timer for the next scheduled change while nothing moves. The cast rotates every 60 to 120 seconds of stage
time, never while the stage is `still` or quiet. The stories gallery (`📖️stories`, dashboard command `@semio-tech/pets-react:dev`, port
6069; `PETS_MENAGERIE` names another menagerie) has a page per species — at rest and every clip, every state with its
tint, look and particles, every trick and the purr, every clip of getting around with its gear, wall, ladder, rope,
block or hand, on a light and a dark ground — and a sandbox of mock cards (walls, gutters, rows marked
`data-pet-prop` with grounds of the cast) where a cast lives in the real layer, with the host's switches (play,
mischief, tempo …), the hand, the deeds and an inspector. Only its dev server puts a director between the layer and
the core (`pets-stories:director`), through which the inspector reads every frame (poofs, overlapping bodies,
particles, rate) and folds forced states, tricks, moods and activities into the stage.

What pets get around with and what their tricks leave in the air is drawn by two modules beside the depiction, by
the same rules (elements built once, attribute and CSSOM writes only where a rounded value changed, no ids, nothing
a policy without inline styles blocks). `🧰️gear`: `equip(species)` builds the tools of one actor — two
`<g class="pet-gear">` for its `<svg class="pet">`, one behind its parts (the parachute: the species' `canopy` or a
plain striped dome, its cords running from the `grip` to the canopy's rim; the carried ladder) and one in front (rope,
hook, grappling gun) — and `paintTools(equipment, actor)` shows the tools of a frame and hides the others;
`rackLadders()` and `paintLadders(rack, ladders, scale)` draw the ladders that stand on the stage (two rails, rungs
perpendicular to them and evenly spaced); `tiltedPlacement(x, y, flip, scale, tilt, pivot)` is the transform of an
actor whose whole drawing is tilted about a pivot, the same way on screen whichever way it faces. `✨️effects`:
`stageEffects()`, `stockEffects(effects, species)` and `paintEffects(effects, kinds, particles, scale)` paint
particles from pools of elements built once per emitter (one `<svg class="pet-effects">`; a particle is a
`transform` and an `opacity`, an unused element is hidden by `visibility`, nothing is written while nothing lives);
`paintPuffs(effects, puffs, scale)` draws the dust where a pet vanished (`Frame.puffs`) behind the particles — pooled
clouds of blobs in ink and paper that spread from the body to its outline, rise and fade (`puffShape`);
`tintPalette(element, palette, tint)` paints the tint of a state over the palette and restores it. The stylesheet
also holds the grabbing hand of the learner: `data-pet-held` on the root element of the document.

## Validation issues

`menagerieIssues(document)`, `speciesIssues(document)` and `ensembleIssues(document)` take any JSON value and
return `{ path, code }[]`: a JSON pointer into the document (`~` and `/` escaped as `~0` and `~1`) and a
kebab-case code, deduplicated and sorted by path, then code, in code point order. An empty list means the
document is valid. A value that fails its structure is not judged any further, and an id that is not a slug
does not count as declared. In a menagerie the pointers of a species begin with `/species/<index>`. An ensemble
is judged without its species documents, so the species ids of its bonds, casts and reactions — and the states
and tricks its reactions name — resolve only in the assembled menagerie. Where two species of a menagerie carry
the same id, references resolve against the first.

The Rust twins (`menagerie_issues`, `species_issues`, `ensemble_issues`) take a document that is already decoded
into its typed twin and return `Issue { path, code: IssueCode }` with the same pointers and codes. Decoding refuses
what the type-level structure forbids — a wrong JSON type, a missing or undeclared property, a literal outside its
enumeration, an `ease` or a bond of the wrong length — so `required` and `property-unknown` never come out of the
Rust validator, `type-invalid` only for a number that is not finite and `value-invalid` only for the `schema`
identifier; every other code is reported exactly as in TypeScript. A host that decodes pets documents with
serde_json must enable its `float_roundtrip` feature: without it a long decimal is read one unit in the last place
off, and the Rust core would no longer compute the bits of the TypeScript core.

Structure, as `🧬️schema/🔣️.json` states it:

| Code | Reported at | Meaning |
|---|---|---|
| `type-invalid` | the value | wrong JSON type, a non-finite number, or a particle `count` that is not whole |
| `required` | the missing property | a required property is absent |
| `property-unknown` | the property | a property the schema does not declare (objects are closed; an activity the repertoire does not know) |
| `value-invalid` | the value | a `const` or enumeration mismatch: `schema`, a shape `kind`, a paint, a channel, a gait, a mood, a gear, a cue, an emitter's `motion`, an activity of a trait, the side `on` of an effect, its `encounter` and its `activity`, the `where` of a reaction |
| `slug-invalid` | the value | an id or reference that is not a slug (`^[a-z0-9]+(?:-[a-z0-9]+)*$`, 1…64 characters) |
| `length-invalid` | the value | a text, path data, ground or species path without a character; an `ease` that has not four numbers; a bond that does not join two species; a reaction's `affinity` that is not two bounds |
| `items-too-few` | `/bones`, `/states`, `/chemistry/i/then` | a rig without a bone, a species without a state, a reaction without an effect |

Rules beyond the structure:

| Code | Reported at | Rule |
|---|---|---|
| `duplicate-id` | the repeated `id` (`/bones/i/id`, `/parts/i/id`, `/face/eyes/i/id`, `/clips/i/id`, `/states/i/id`, `/tricks/i/id`, `/emitters/i/id`, `/species/i/id`, `/chemistry/i/id`); in an ensemble the repeated path `/species/i` | ids are unique within the bones, the parts, the eyes, the clips, the states, the tricks and the emitters of a species and within the species and the reactions of a menagerie; every later occurrence is reported |
| `unknown-reference` | the reference | `Bone.parent`, `Part.bone`, `Eye.bone`, `Mouth.bone`, `Track.bone` and `Emitter.bone` name a bone, `Face.above` a part, a repertoire entry, `SpeciesState.clip`, `Trick.clip` and `Purr.clip` a clip of the species, `SpeciesState.emitter`, `Trick.emitter` and `Purr.emitter` an emitter of it, `SpeciesState.then`, `Trick.from` and `Trick.to` a state of it; the two ends of a bond, the core and rotation of a cast and the `species` of a trait name species of the menagerie; the `state` and `trick` of a trait (`when`, `near`, `unless`) name a state and a trick of its species — of some species of the menagerie when it names none —, and the `state` and `trick` of an effect a state and a trick of the species of the side it acts on (not judged when that species is unknown) |
| `bone-order` | `/bones/0/parent`; `/bones/i` for a later bone without a parent; `/bones/i/parent` for a parent that is the bone itself or listed later | the first bone has no parent, every other bone has one, and it is listed earlier |
| `key-order` | `…/keys` for fewer than two keys; otherwise the `at` of the key | a track has at least two keys, every `at` lies in [0, 1], the first is 0, the last is 1, and each is greater than the one before |
| `loop-seam` | the `value` of the last key | in a looping clip the first and the last key of a track carry the same value; on the `rotation` channel the same angle modulo 360° (their difference is a whole number of turns) |
| `ease-range` | `…/ease/0`, `…/ease/2` | the abscissas `x1` and `x2` of an easing lie in [0, 1] |
| `out-of-range` | the value | sizes, ellipse radii, rectangle sides, stroke widths, eye and pupil radii, mouth widths, clip seconds, speeds and hover heights are above 0; a rectangle's corner radius is at least 0; a pupil is smaller than its eye (at `…/pupil`); temperament traits lie in [0, 1], affinities in [−1, 1]; colours (of a palette and of a tint) are `#rrggbb` in lowercase hexadecimal digits; a state `lasts` more than 0 seconds; an emitter has 1…32 particles, a `life` above 0, a `speed` of at least 0 and a `spread` in [0, 1]; `grip` is above 0 and at most the height, `reach` at least 0 and at most the width (the upper bounds only against a size that is itself above 0); a reaction's `within` and `every` are above 0, its `chance` lies in [0, 1] and its `affinity` bounds in [−1, 1], the low one first (an empty range is reported at `…/affinity`); a trait's `held` is above 0; an effect's `amount` lies in [0, 1] and its `rapport` in [−1, 1] |
| `self-bond` | `/bonds/i/between` | a bond joins two different species |
| `duplicate-bond` | `/bonds/i/between` | an unordered pair of species is bonded at most once; every later bond of the pair is reported |
| `missing-gait-clip` | `/locomotion/gait` | a `walk` gait has at least one clip under `repertoire.walk`, a `hop` gait under `repertoire.hop` |
| `float-hover` | `/locomotion/hover` | `hover` is present exactly when the gait is `float` |
| `empty-cast` | `/casts/i/core` | a cast has at least one core species |
| `duplicate-scene` | `/casts/i/scene` | scenes are unique; every later cast of a scene is reported |
| `duplicate-entry` | the repeated entry (`/gear/i`, `/tricks/i/cues/j`, `/tricks/i/from/j`) | a gear, a cue of a trick and a state a trick is on offer in are listed once; every later occurrence is reported |
| `lasts-then` | `/states/i/then` | a state with `lasts` names the state `then` it gives way to |
| `missing-gear-clip` | `/gear/i` | a species has at least one clip for every activity its gear brings: `climb` → `climb`, `mantle`, `slide`; `ladder` → `carry`, `climb`; `grapple` → `aim`, `reel`; `parachute` → `glide` |
| `missing-activity-clip` | `/repertoire/<activity>` | every species has at least one clip for `hang`, `tumble`, `purr`, `dizzy`, `shrug` and `push` |
| `floater-gear` | `/gear/i` | a species whose gait is `float` owns neither `climb` nor `ladder` nor `grapple` |

The schema itself expresses the structure and, of the rules, the ranges (but not `grip` and `reach` against the
size), the colour pattern, the ease abscissas, the two different ends of a bond, the core of a cast, the minimum
of two keys, of one state and of one effect, and the distinct entries of gear, cues and `from`; the other rules
are the validator's alone. Case `🧪️tests/🧬️schema-conformance` holds the validator to python-jsonschema on
the normative file and to one reading of the rules across the cores; the unit suite of `✅️validation` holds it
to ajv, including a sweep that breaks every property of the sample documents in turn.
