# 🧍️ Staffage

An archive of handcrafted architectural staffage — people, vegetation, animals, objects, vehicles and machines, building components —
with real dimensions, rigs, poses and clips, and physical models (thermodynamics, structural forces and moments). One evaluator feeds
every view, the website, the reusable component and every export.

## Pipeline

```
🗂️catalog/*.json ──validate──▶ Asset ──evaluate(query)──▶ EvaluatedState (world metres, one instant)
                                                   ├─ solveThermal ─▶ ThermalField
                                                   ├─ solveStructure ─▶ StructuralResult
                                                   ├─ depict(view, scale) ─▶ DrawingScene (paper mm) ─▶ SVG · PDF · frames · MP4/WebM
                                                   └─ modelScene ─▶ ModelScene (model metres) ─▶ DXF · DWG
```

A query is a pose, a clip at an arbitrary time, or a transition between two poses. The same state drives all seven representations — dark line,
dark solid, light line, light solid, thermodynamic, structural forces, structural moments — so scrubbing, comparing or exporting can never
desynchronise geometry and analysis.

## Layout

| Path | Content |
|---|---|
| `🧬️schema/🔣️.json` | `semio.staffage/v1`, the normative contract (`🟦️.ts` is its typed twin) |
| `🗂️catalog/🔣️.json` | the manifest: categories, collections, one row per asset with its completion status |
| `🗂️catalog/<category>/<slug>/🔣️.json` | the handcrafted asset `<slug>` of one category |
| `🔨️modules/` | `🔗️contracts` · `📐️units` · `🦴️rig` · `🎞️animation` · `🧮️evaluation` · `🗂️catalog` · `🌡️thermal` · `🏗️structure` · `🎨️depiction` · `📤️export` |
| `📦️packages/🟦️typescript` | library `@teaching/staffage` |
| `🎯️targets/⚛️react/📦️packages/🟦️typescript` | `@teaching/staffage-react`: `StaffageFigure`, the `<staffage-figure>` element and the archive app |
| `🌐️site/` | `@teaching/staffage-site`: the website shell and its Vite build |
| `🧪️tests/`, `🧫️fixtures/` | test cases (Vitest, Gherkin, Python oracles) and language-agnostic vectors |

## Conventions

- SI internally: m, kg, s, N, Pa, W; temperatures authored in °C. World frame: x ahead, y up, z left of the asset (a left-handed triple), ground at y = 0; the side elevation is seen from the asset's right with ahead on the paper's right, the front elevation from ahead (the asset's left on the paper's right), the plan (top view) from above (the asset's left up the page); angles in degrees, CCW positive.
- Drawing scale only maps model metres to paper millimetres (`mm = m · 1000 / denominator`) and selects line weights, level of detail
  and annotation sizes. It never changes dimensions, masses, temperatures or loads. Viewport zoom is a separate screen transform.
  Presets 1:500, 1:200, 1:100, 1:50; any positive denominator is allowed.
- Axial force N > 0 is tension; bending moment M > 0 is sagging; joint moments are CCW positive. Compression is drawn warm, tension cool,
  with hatch patterns as a colour-independent reading.
- Every physical parameter is tagged `measured`, `literature`, `authored-estimate` or `illustrative`. No parameter claims measured accuracy;
  thermal fields are engineering estimates, not CFD; human joint loads are rigid-segment resultants, not clinical bone or muscle forces.
- All text is `{ en, de }`; there is no default language.

## Thermal options

- `thermal.referenceEnvironment` (optional): the ambient case standalone thermal previews and exports use: the id of a preset of `🗂️catalog/🌤️environments` or one inline environment (all required `Environment` fields). Precedence for `--view thermal` of the export CLI: `--environment <preset>` first, then the asset's `referenceEnvironment`, then the asset's own `thermal.environment` (the explicit default, solved by `solveThermal` whenever no environment is passed). The `solar-thermal-collector` declares `summer-heatwave-calm` (900 W/m², 0.3 m/s), so its absorber is the hot part in a plain export. `--environment` with another representation is refused.

- `phaseChange` on a material or a region: `{ "transition": "boiling" | "melting", "pointC", "latentHeatJKg" }`. While heated, the region
  holds at `pointC`; the surplus power becomes converted mass (`phaseChangeKgS`, `evaporationKgS`). A vent with a `region` and a
  `vapourKgPerKg` follows that evaporation. A region's own declaration overrides its material's; `body`, `fixed` and `leaf` regions cannot carry one.
- `thermalSpanK` on a cast (thermal representation only): a symmetric ±K surface-temperature range around the air temperature of the cast environment.
  Absent, the shared absolute range of −20 to 60 °C applies. One span per Scene: the thermal casts drawn by one Scene class share one span,
  so they stay comparable (the course uses 25 K; guard test `thermal_span` fails on a Scene that mixes spans). Choosing the smallest span that
  covers the hottest of them is authoring guidance, not checked by code.

## Front and plan views

Every authored representation draws `geometry.elevation` (side elevation), `geometry.front` (front elevation) and `geometry.plan` (plan, top view), each in rest-pose model metres bound to the same bones; the viewpoint is a depiction option (`--viewpoint elevation|front|plan`, `--views` for a sheet of the three views in the preview CLI; the viewer always offers the three views).

The multiview sheet puts the three views of one evaluated state on one sheet: `--format sheet-svg|sheet-pdf|sheet-dxf|sheet-dwg --projection first|third`. Third angle (`third`): plan above the side elevation, front elevation to its right. First angle (`first`): plan below, front to the left. The projection is required with no default, the projection symbol is drawn on the sheet, and `--viewpoint` is refused with a sheet format. All cells come from one query, time, representation and scale; otherwise the sheet throws. Each output file name carries the viewpoint or the sheet projection (D14). DWG sheets are unverified on a real file: `staffage_dwg.wasm` is not built here, so the DWG path fails with `DwgUnavailableError` (pending). Source: `📝️sonnet-exports.md` in the ticket folder.
Front and plan shapes are projections of one three-dimensional chain: `evaluate` carries every vertex of a front (lateral, up) or plan (ahead, lateral) shape from the rest frame of its bone to the current frame by the same transform that places the bone (`carry`), so swing, spread and twist all read right. A drawing has no depth of its own, so a vertex takes the depth it lacks from the elevation silhouette of the same bone at rest, at the height (front) or forward position (plan) of the vertex (the elevation twin of the same `id` is carried at the middle of its silhouette); a bone without elevation shapes pins its shapes at the rest depth of its origin. `followTip` therefore stretches nothing for a bone that has extent in the view; it only matters where the drawing really has no depth information: a `followTip` rect or poly on a bone that lies along the viewing direction (a wing or jib seen end-on) grows outwards from the origin to the tip by the lateral position across the shape, and a plan capsule with `followTip` follows the origin with its first end and the tip with its second. Every other part that moves out of the bone-parallel directions is a `solid`.

The evaluated state carries the chain itself (`bones[id].frame`, `tip`, `anchors[id].z`, `root`, `kinematics` as 3D vectors), `timeS` is the absolute query time and `clipTimeS` the wrapped clip phase. Painting order follows the nearest point of each shape (`shapeDepth`), with the cameras of `🎨️depiction/🔭️views` as the one source of the projection.

### Solid placements

Parts that rotate away from the bone-parallel directions (blind slats, flaps, booms, umbrella canopies, lids, drums) do not stretch: give the front or plan shape a `solid`. The profile is authored in the **bone's local frame like a side-elevation shape** and placed with the full bone transform, so the three views are right at every pose and clip instant.

| `solid` | Front | Plan |
|---|---|---|
| `{ "kind": "revolved", "axisBone", "axis": "u", "v" or "w", "axisX", "axisZ"?, "mirror"? }` an axisymmetric part about the direction `axis` of the bone-local frame of `axisBone` (u along the bone, v its normal, w lateral) through the rest origin of `axisBone`, offset by `axisX` (side-elevation x) and `axisZ` (lateral, default 0) from that origin: the axis is local to its bone and travels with it (D24) | paper x is the lateral distance from the axis, y the world height; `mirror` adds the reflection so a half profile reads as the whole body | the circle of the widest lateral reach about the axis |
| `{ "kind": "extruded", "z0", "z1" }` a laterally prismatic part (left is +) | `z0`..`z1` by the height range of the transformed profile | forward range of the transformed profile by `z0`..`z1` |

`solid` is allowed in every viewpoint and excludes `followTip`; `axisBone` must be the shape's own bone or one of its ancestors (D21) and `z0 < z1`. Vectors: `🧫️fixtures/🧊️solid-placements`, test `🧪️tests/🧊️solid-placements` (shapely oracle).

### Lateral spread

The rig is sagittal (forward and up), so wings spread sideways, arms raised sideways and straddled legs need a degree of freedom the front and plan views can read: `abductionDeg` in a pose or clip key (`BoneOffset`, revolute joints, -180 to 180 degrees).

```json
"bones": { "arm-n": { "angleDeg": -46, "abductionDeg": -75 }, "arm-f": { "angleDeg": -32, "abductionDeg": 75 } }
```

- After its own sagittal rotation the bone turns out of the sagittal plane about its normal. A point at distance `s` along the bone is `s·sin φ` to the left (+z, the left of the asset; the right-hand side takes negative values, so the near wing of a right-facing side view is negative) and keeps `s·cos φ` of its sagittal reach. The value is cumulative down the chain like `angleDeg`, so a hand follows the spread arm; interpolating poses and clips interpolate it.
- **All three views agree in every pose.** `solveRig` solves the chain in three dimensions whenever a bone is spread and the side elevation (`elevation`) is its orthographic projection: the bone matrix carries the projected axes, so elevation shapes foreshorten by `cos φ` exactly like the front and plan views, and placement, mass, thermal, structure, kinematics and grounding all read the same chain. Without a spread nothing changes (the old 2D solve is used). The guard checks the agreement (`spread-*` checks of the three-views test) for every pose and clip instant with a spread bone.
- Front and plan shapes add the lateral position to their projected bone travel: a rigid shape follows the lateral position of the bone origin (children ride on the spread parent), a `followTip` rect or poly stretches from the origin to the tip, in plan also forward; a plan capsule with `followTip` moves both ends. The fraction along the bone comes from the height (front) or the forward position (plan) when the bone has at least half its length of that extent at rest, otherwise from the lateral position across the shape from its inner to its outer end: a folded wing lying along the viewing direction therefore grows outwards from the body, and its front and plan shapes only need their folded rest outline. `solid` shapes ignore the spread.
- Validator: `joint-limit` for a spread on a fixed or prismatic joint and above 180 degrees (schema `BoneOffset.abductionDeg`).
- Vectors with hand-computed displacements: `🧫️fixtures/🧊️lateral-spread`; test `🧪️tests/🧊️lateral-spread` (numpy rotation matrices and shapely oracle); demonstration: the soaring and landing poses of `herring-gull`.

### Axial twist

`twistDeg` in a pose or clip key (`BoneOffset`, revolute joints, -180 to 180 degrees unless the joint declares `twistMinDeg`/`twistMaxDeg`, up to whole turns of 360 degrees each way) turns a bone and its whole subtree about the bone's own longitudinal axis, after its sagittal rotation and spread, accumulating down the chain. For a bone pointing up it is a yaw: crane slew, bar-stool or office-chair swivel, bicycle steering.

```json
"bones": { "turntable": { "twistDeg": 60 } }
```

- Positive twist turns the bone's local +y axis towards the right of the asset: a bone pointing up turns counter-clockwise seen from above (forward goes to the left), a bone pointing down clockwise.
- It lives in the same three-dimensional chain as the lateral spread, so the side elevation, the front elevation and the plan agree: a jib slewed by `τ` is `cos τ` as long in the side elevation and reaches `sin τ` of its length sideways in front and plan. Bones that lie along the viewing axis at rest (jib, boom, wing) need `followTip` front and plan shapes, or a plan capsule with `followTip`, to slew with it. `solid` extruded shapes widen their lateral range by the sweep of the bone.
- Validator: `joint-limit` for a twist on a fixed or prismatic joint and above 180 degrees. Guard: the `spread-*` checks of the three-views test cover every pose and clip instant with a spread or twisted bone.
- Vectors with hand-computed positions: `🧫️fixtures/🧊️axial-twist`; test `🧪️tests/🧊️axial-twist` (numpy rotation matrices and shapely); demonstration: `tower-crane` pose `parked-weathervane` (turntable twist of 60 degrees).

## Course casts

The course draws people, furniture, devices and machines only from this archive, through the cast manifest in
`🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json` (schema `🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json`).
Each cast names an `asset`, one `pose` or `clip`, a `view` and a required `theme` (`dark` or `light`). The exporter renders them:

```
repo tools exec bun -- 🔨️modules/📤️export/🎭️cast/🟦️.ts --manifest <file> [--ids a,b] --out <dir> [--parent <pid>]
```

Progress goes to stderr (`[pid …]`, `[cast i/n] id`, `[done/total] …` every 50 frames). SIGINT, SIGTERM, SIGHUP (POSIX) or SIGBREAK (Windows)
cancel the export between two frames with exit 130 and leave no partial cast (a cast is written to a temporary folder and renamed into place);
`--parent <pid>` cancels the same way when the calling process disappears. Unknown flags (there is no `--lang`) are refused.

### Cast row

| Field | Meaning |
|---|---|
| `asset`, `pose` xor `clip`, `view`, `theme` | required choice of what is drawn; `theme` (`dark` or `light`) colours the thermal, forces and moments representations and has no default |
| `viewpoint`, `scale`, `fps` | `viewpoint` is required, with no default: `elevation` (side elevation), `front` (front elevation) or `plan` (plan, top view); `mirror` is allowed only with `elevation`; denominator, default 50; frames per second, default 15, only with a clip or a `timeline` |
| `environment` xor `timeline` | thermal representation only: an environment preset id, or preset ids keyed by absolute time (`t`, `ease` `step`/`linear`) for a transient |
| `thermalSpanK` | thermal representation only; ±K around the cast environment's air temperature instead of −20…60 °C (one span per Scene, see above) |
| `transport` | any of `parcels`, `pathlines`, `streamlines`, `condensation`; figure-only, without emitter markers |
| `transportAgeS` | clips with `transport` only: longest parcel and pathline life; frames start at that time, so every frame already shows the developed field. A looping clip makes the field periodic with its duration (`periodS` is derived, never authored) |
| `transportStrokeMm`, `transportTrailS` | pathline paper width (default 0.08 mm) and visible trail length in seconds (default: whole path) |
| `mirror` | only with `viewpoint` `elevation`; flips the figure about the vertical axis of the world origin (D5); a mirror of a laterally asymmetric asset needs a reason in the guard's allowlist (D19) |

A cast of a front or plan viewpoint that the asset draws as a section also writes `section` (`{ plane, label }`, as declared in `sections` of the representation) to its index.

The index holds the paper extents in `boundsM` (everything drawn). A cast with `transport` also writes `figureBoundsM` (the person without the air layer). `heightM` is always the physical figure height for every viewpoint, never a paper measure (D13). Playback of a cast reads `loop` and `fps` from the index: a clip with `loop: false` (fan spin,
tealight flicker) is repeated by `fitted_staffage(..., loop=True)`.

### Textless casts and the frame cap

Casts are textless: no legend, title, label, dimension, scale bar or ground line, so a cast carries no language and its output is the same
for every language. A schedule of more than 600 frames (the toaster, the laptop, the vacuum run) is thinned to 600 evenly spaced samples
with a fractional `fps` in the index; play such a cast with `rate` (clip seconds divided by scene seconds), at `rate=1` only its first seconds are
seen in coarse steps.

### Python API (`🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`)

| Call | Result |
|---|---|
| `staffage(cast, unit=, anchor=, color=, opacity=, window=)` | the first frame as a `VGroup`, world origin (the feet) on `anchor`, `unit` Manim units per metre |
| `staffage_motion(cast, unit=, anchor=, rate=1.0, color=, window=)` | a clip or transient swapped from scene time at `rate` times real time; it plays during `wait` and `play` |
| `fitted_staffage(cast, width=, height=, color=, rate=, loop=, window=)` (module `visuals`) | the largest uniform scale that fits `width` and `height`, feet on the origin; with `rate` it plays; `loop` repeats a non-looping clip |
| `staffage_height(cast)` | the physical figure height in metres for every viewpoint (landing in the exporter, D13), never the air layer of a transport cast |
| `staffage_parts(cast, prefix="")` | the filled polygons of the first frame as world-metre arrays, for attachment points (straight `M`/`L`/`H`/`V`/`Z` outlines) |
| `ensure_casts(ids=None, progress=None)` | renders every missing cast in one exporter call; `progress` receives each stderr line |

- Sizing is figure-bounded: a cast with `figureBoundsM` has its width, height, top, bottom and centre measured on the figure alone, so a
  breathing plume never shrinks or shifts the person; `window` (min x, min y, max x, max y in cast metres) keeps only the paths fully inside it and
  sizes by the window, for example a blind without its wall stubs or a head as an icon.
- Cache: casts live in `.🧬semio/🎓️teaching/⚡️cache/staffage/<key>/<cast>` (`SEMIO_TEACHING_CACHE_ROOT` moves it). The key hashes the manifest
  row, the asset's catalog file, the environment presets the cast names, and the exporter and schema sources and print profiles with LF line
  endings, so it is the same on every platform. After a render, key folders no manifest cast maps to and abandoned `.tmp-*` folders older than
  24 h are pruned; a cast another process is rendering is waited for, not rendered twice.
- Cancellation: the exporter runs in its own process tree (job object on Windows, process group on POSIX). An exception, KeyboardInterrupt,
  SIGTERM or SIGHUP in the caller stops the tree (grace period, then kill), and the exporter's `--parent` watchdog covers a hard-killed caller.

### Guard

`🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🐍️staffage_only.py` (rules and allowlists in the sibling `🔣️staffage_only.json`, readable by any language):
- Rule A: no function in a clip builds a physical object from primitives without a staffage call.
- Rule B: every cast an energy clip names is thermal, or an allowlisted furniture cast (with a reason).
- Rule C: no function composes three or more distinct primitives into one group without staffage, outside charts, tables and axes.
- Thermal span: all thermal casts of one Scene class share one `thermalSpanK`.

### Adding a device

1. Catalog: add the asset to the fitting `🗂️catalog/<category>/<slug>/🔣️.json` and its row in `🗂️catalog/🔣️.json`; run the catalog check.
2. Manifest: add a cast row (`asset`, `pose` or `clip`, `view`, `theme`, plus `environment` and `thermalSpanK` for a thermal cast) to
   `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json`. Ids are kebab-case slugs.
3. Clip: call `fitted_staffage("<cast-id>", width=, height=)` (add `rate=` to play it); never draw the device from primitives, the guard fails.

Meters and building stores are device casts: `gas-meter`, `electricity-meter` and `hot-water-cylinder` (`🔌️appliances`), and the
`solar-thermal-collector` (`🏗️machinery`). Thermometers, clocks and abstract energy gauges (`energy_tank`) stay diagram glyphs.

## Run

```
repo run @teaching/staffage:test --lang=en
repo run @teaching/staffage-site:dev --lang=en
repo run @teaching/staffage-site:build --lang=en
```
