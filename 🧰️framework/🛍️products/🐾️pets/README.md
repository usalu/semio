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
| `🔨️modules/🎲️randomness/` | Counter-based draws: `randomWords(key, count)` (numpy's seed sequence), `unitOf`, `randomUnit`, `randomBetween`, `weightedIndex`, `randomPick`, and the three reserved streams (`STAGE_STREAM`, `CAST_STREAM`, `ROTATION_STREAM`). |
| `🔨️modules/🦴️rig/` | 2×3 affine matrices, `restPose`, `solveRig` (forward kinematics), `lookOffset` (where a pupil looks) and `pupilReach` (how far it travels). |
| `🔨️modules/🎞️animation/` | `easeBezier`, `sampleTrack`, `sampleClip`, `blendPose`, the gaze spring `springStep` and the blink `lidAt`. |
| `🔨️modules/🏞️terrain/` | `perchesOf` (surfaces minus keep-outs), strides, falls, landings and the ballistic hop between perches. |
| `🔨️modules/🧠️behavior/` | Mode limits, activity weights and dwells, casts, encounters, affinity and rapport, needs. |
| `🔨️modules/🎪️stage/` | `openStage(seed)`, `advance(menagerie, stage, events)`, `frameOf(menagerie, stage)`. |
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

bun nx run @semio-tech/pets:test              # the same through nx            (bun run test:pets)
bun nx run @semio-tech/pets:typecheck         # the core, its suites, adapters  (bun run typecheck:pets)
bun nx run @semio-tech/pets-rs:test           # the Rust core                  (bun run test:pets:rs)
bun nx run @semio-tech/pets-react:test        # the React target               (bun run test:pets:react)
bun nx run @semio-tech/pets-react:typecheck   # its types                      (bun run typecheck:pets:react)
bun nx run @semio-tech/pets-react:dev         # the stories gallery            (bun run dev:pets:stories)

cd "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test"
bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/🐾️pets"
bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/🐾️pets"                                 # every case
bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🧬️schema-conformance"    # one case, by its directory name
```

## Domain model

### Menagerie, ensemble, species

A **menagerie** (`semio.pets.menagerie/v1`) holds every species of a domain, their bonds and the casts of its
scenes. It is authored as an **ensemble** (`semio.pets.ensemble/v1`): the same head, bonds and casts, with every
species in a document of its own that the ensemble names by a path relative to itself.
`assembleMenagerie(ensemble, species)` takes the species documents in the order of those paths and yields the
menagerie; the `$schema` hints authored documents may carry stay behind. A menagerie reaches the browser as
statically imported data; nothing is fetched at run time.

A **species** has an id, a `name` and the `thing` it stands for (both in every language; there is no default
language), the `grounds` that tie it to its domain (free strings the domain interprets), a `size`, a `palette`
of three colours (ink and paper come from the theme), its rig, its motions, a `locomotion` and a `temperament`.

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

The **repertoire** maps activities (`idle`, `fidget`, `walk`, `hop`, `fall`, `land`, `sleep`, `greet`,
`cuddle`, `squabble`, `sulk`) to clip ids; one of several is drawn, none leaves the body at rest. The
**locomotion** names the gait (`walk`, `hop`, `float`), the speed in pixels per second and, exactly when the
gait is `float`, the `hover` height above the perch. A walker needs a `walk` clip, a hopper a `hop` clip.

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

### Stage, events, frames

`openStage(seed)` is an empty stage. `advance(menagerie, stage, events)` folds events into it —
`ticked`, `pointed`, `unpointed`, `glanced`, `surveyed`, `summoned`, `tuned`, `hushed`, `poked` — and
`frameOf(menagerie, stage)` projects what a render target draws: the actors back to front with a matrix per
bone, their eyes and mood, what they are doing (`activity`), the rate the motion needs (64, 32, 16 or 0 ticks
per second) and the tick of the next change when nothing moves. Time is whole ticks, 64 per second.

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
`floaty` — with three bonds and the casts of the scenes `home` and `meadow`), `ensemble` (its authoring form),
`species` (`{ path, document }` per species in ensemble order, with their `$schema` hints) and `bases` (the
smallest valid species, menagerie and ensemble). The vectors beside them are `accepted`, `structural` and
`rules`; the file is generated by the ticket script named in the case's feature.

## React target

`@semio-tech/pets-react` draws a cast on a web page. `<PetLayer>` renders one static `<div class="pet-layer"
aria-hidden="true">` over the viewport; a single effect then opens a stage, feeds it what it sees of the page and
paints every frame into one inline `<svg class="pet" data-pet="<species>" data-pet-activity="<activity>">` per
actor. React never renders per frame.

| Prop | Default | What |
|---|---|---|
| `menagerie` | — | the species, bonds and casts; one with validation issues is never shown |
| `scene` | — | the scene whose cast is on stage; an unknown scene falls back to `home`, then to nobody |
| `mode` | — | `still`, `calm` or `lively` (`off` is not rendering the layer) |
| `quiet` | `false` | a time of concentration: no rotation, no turn, lean or greeting towards the pointer, no poke |
| `capacity` | 2 below 768 px, 4 below 1024 px, else 6 | the most actors at once |
| `surfaces` | `[data-pet-surface]` | selector of the elements whose top edge carries pets; the bottom edge of the layer is always the floor |
| `keepouts` | `PET_KEEPOUTS` | selector of what pets never cover: controls (`PET_CONTROLS`), what a person reads (`PET_TEXTS`) and `[data-pet-keepout]` — the mark for text that lies bare in a `div` or a `span` |
| `glances` | none | other things worth a look, in viewport pixels, sampled with every survey |
| `scale` | 0.8 below 768 px, else 1 | the size pets are drawn at |
| `seed` | random per mount | the seed of the stage |
| `zIndex` | 35 | above cards, below dialogs |
| `onCast` | none | told which species are on stage, in menagerie order, whenever that changes (nobody when the show ends) |
| `tempo` | 1 | how fast the pets' time passes, held between ⅛ and 8; for tools and tests that must see in seconds what takes minutes |

The layer is decoration and keeps out of the way: it is hidden from assistive technology and holds nothing that
takes focus; it never takes pointer events (the stylesheet says so, and a running show repeats it on the element);
it never changes layout or scroll and makes no request, no sound and no console output. Frames are written by script
and show as written: nothing in the layer transitions, whatever the host's stylesheet says (`transition-property: none
!important` on the layer and everything in it) — a host that shortens every transition for a device that asks for
reduced motion would otherwise start one with every write of a frame. Besides the keep-outs of the
selector, every surface is solid (pets stand on a thing, never in front of another one) and the focused element
keeps `FOCUS_MARGIN` (8 px) free around itself. A pet turns see-through while the pointer rests on it. Nothing runs
while the document is hidden, while the layer is inert or hidden and under forced colours (the layer stays empty);
in print the layer is not shown. A fault inside the show ends the show silently.

Time comes from the pacer (`createPacer`): frames at the rate the stage asks for — 64, 32 or 16 ticks per second —
and a timer for the next scheduled change while nothing moves. The cast rotates every 60 to 120 seconds of stage
time, never while the stage is `still` or quiet. The stories gallery (`📖️stories`, `bun run dev:pets:stories`, port
6069; `PETS_MENAGERIE` names another menagerie) shows every species at rest and with every clip, and a sandbox of
mock cards for a cast.

## Validation issues

`menagerieIssues(document)`, `speciesIssues(document)` and `ensembleIssues(document)` take any JSON value and
return `{ path, code }[]`: a JSON pointer into the document (`~` and `/` escaped as `~0` and `~1`) and a
kebab-case code, deduplicated and sorted by path, then code, in code point order. An empty list means the
document is valid. A value that fails its structure is not judged any further, and an id that is not a slug
does not count as declared. In a menagerie the pointers of a species begin with `/species/<index>`. An ensemble
is judged without its species documents, so the species ids of its bonds and casts resolve only in the
assembled menagerie.

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
| `type-invalid` | the value | wrong JSON type, or a non-finite number |
| `required` | the missing property | a required property is absent |
| `property-unknown` | the property | a property the schema does not declare (objects are closed; an activity the repertoire does not know) |
| `value-invalid` | the value | a `const` or enumeration mismatch: `schema`, a shape `kind`, a paint, a channel, a gait |
| `slug-invalid` | the value | an id or reference that is not a slug (`^[a-z0-9]+(?:-[a-z0-9]+)*$`, 1…64 characters) |
| `length-invalid` | the value | a text, path data, ground or species path without a character; an `ease` that has not four numbers; a bond that does not join two species |
| `items-too-few` | `/bones` | a rig without a bone |

Rules beyond the structure:

| Code | Reported at | Rule |
|---|---|---|
| `duplicate-id` | the repeated `id` (`/bones/i/id`, `/parts/i/id`, `/face/eyes/i/id`, `/clips/i/id`, `/species/i/id`); in an ensemble the repeated path `/species/i` | ids are unique within the bones, the parts, the eyes and the clips of a species and within the species of a menagerie; every later occurrence is reported |
| `unknown-reference` | the reference | `Bone.parent`, `Part.bone`, `Eye.bone`, `Mouth.bone` and `Track.bone` name a bone, `Face.above` a part, a repertoire entry a clip of the species; the two ends of a bond and the core and rotation of a cast name species of the menagerie |
| `bone-order` | `/bones/0/parent`; `/bones/i` for a later bone without a parent; `/bones/i/parent` for a parent that is the bone itself or listed later | the first bone has no parent, every other bone has one, and it is listed earlier |
| `key-order` | `…/keys` for fewer than two keys; otherwise the `at` of the key | a track has at least two keys, every `at` lies in [0, 1], the first is 0, the last is 1, and each is greater than the one before |
| `loop-seam` | the `value` of the last key | in a looping clip the first and the last key of a track carry the same value; on the `rotation` channel the same angle modulo 360° (their difference is a whole number of turns) |
| `ease-range` | `…/ease/0`, `…/ease/2` | the abscissas `x1` and `x2` of an easing lie in [0, 1] |
| `out-of-range` | the value | sizes, ellipse radii, rectangle sides, stroke widths, eye and pupil radii, mouth widths, clip seconds, speeds and hover heights are above 0; a rectangle's corner radius is at least 0; a pupil is smaller than its eye (at `…/pupil`); temperament traits lie in [0, 1], affinities in [−1, 1]; colours are `#rrggbb` in lowercase hexadecimal digits |
| `self-bond` | `/bonds/i/between` | a bond joins two different species |
| `duplicate-bond` | `/bonds/i/between` | an unordered pair of species is bonded at most once; every later bond of the pair is reported |
| `missing-gait-clip` | `/locomotion/gait` | a `walk` gait has at least one clip under `repertoire.walk`, a `hop` gait under `repertoire.hop` |
| `float-hover` | `/locomotion/hover` | `hover` is present exactly when the gait is `float` |
| `empty-cast` | `/casts/i/core` | a cast has at least one core species |
| `duplicate-scene` | `/casts/i/scene` | scenes are unique; every later cast of a scene is reported |

The schema itself expresses the structure and, of the rules, the ranges, the colour pattern, the ease
abscissas, the two different ends of a bond, the core of a cast and the minimum of two keys; the other rules
are the validator's alone. Case `🧪️tests/🧬️schema-conformance` holds the validator to python-jsonschema on
the normative file and to one reading of the rules across the cores; the unit suite of `✅️validation` holds it
to ajv, including a sweep that breaks every property of the sample documents in turn.
