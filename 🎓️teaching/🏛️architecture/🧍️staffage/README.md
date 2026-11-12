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
                                                   ├─ modelScene ─▶ ModelScene (model metres) ─▶ DXF · DWG
                                                   └─ modelOf(asset, options) ─▶ ModelDocument (semio.model/v1, neutral frame) ─▶ GLB · FBX · JSON · ZIP
```

A query is a pose, a clip at an arbitrary time, or a transition between two poses. The same state drives all seven representations — dark line,
dark solid, light line, light solid, thermodynamic, structural forces, structural moments — so scrubbing, comparing or exporting can never
desynchronise geometry and analysis.

## Layout

| Path | Content |
|---|---|
| `🧬️schema/🔣️.json` | `semio.staffage/v2`, the normative contract (`🟦️.ts` is its typed twin) |
| `🗂️catalog/🔣️.json` | the manifest: categories, collections, one row per asset with its completion status |
| `🗂️catalog/<category>/<slug>/🔣️.json` | the handcrafted asset `<slug>` of one category |
| `🔨️modules/` | `🔗️contracts` · `📐️units` · `🦴️rig` · `🎞️animation` · `🧮️evaluation` · `🗂️catalog` · `🌡️thermal` · `💨️transport` · `🏗️structure` · `🖨️representation` · `🔺️polygon` · `🏷️revision` · `🎨️depiction` · `🔭️multiview` · `🧊️volume` (asset → `semio.model/v1`) · `📤️export` (svg, pdf, dxf, dwg, frames, video, cast, sheet, cli, `🧊️glb`, `🎬️fbx`, `🗿️model`, `🗜️zip`, `📦️batch`, `🔌️tools`) |
| `📦️packages/🟦️typescript` | library `@teaching/staffage` |
| `🎯️targets/⚛️react/📦️packages/🟦️typescript` | `@teaching/staffage-react`: `StaffageFigure`, the `<staffage-figure>` element and the archive app |
| `🌐️site/` | `@teaching/staffage-site`: the website shell and its Vite build |
| `🔮️oracles/` | the registry of the third-party test oracles |
| `🧪️tests/`, `🧫️fixtures/` | test cases (Vitest, Gherkin, Python oracles) and language-agnostic vectors |

The neutral 3D model kernel is not part of this folder: `🧰️framework/🔨️modules/🧊️3d/🗿️model` (README there).

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

The multiview sheet puts the three views of one evaluated state on one sheet: `--format sheet-svg|sheet-pdf|sheet-dxf|sheet-dwg --projection first|third`. Third angle (`third`): plan above the side elevation, front elevation to its right. First angle (`first`): plan below, front to the left. The projection is required with no default, the projection symbol is drawn on the sheet, and `--viewpoint` is refused with a sheet format. All cells come from one query, time, representation and scale; otherwise the sheet throws. Each output file name carries the viewpoint or the sheet projection (D14). DWG sheets are genuine AC1024 files from the crate's own writer, read back by ezdwg for all four reference assets in both projections (`🧪️tests/📑️multiview-sheet`). Source: `📝️sonnet-exports.md` in the ticket folder.
A view drawn as a section declares it in `sections` of the representation (`elevation`, `front` or `plan`, each `{ plane, label }`). The plane is the lateral position z of a vertical cut along the forward axis seen from the right (the part at smaller z is removed) for the side elevation, the forward position x of a vertical cut across the forward axis seen from ahead (the part at larger x is removed) for the front elevation, and the height y of a horizontal cut seen from above (the part above is removed) for the plan. A section view shows the cut faces of the volume and may show everything that remains visible beyond the plane; what lies in front of the plane appears at most as a thin outline (overhead and hidden lines). The side elevation carries the cut-line marker of a front or plan section, the plan the marker of an elevation section; captions read "Section A-A" / "Schnitt A-A" and "Plan section B-B" / "Grundriss-Schnitt B-B". Guard: `🧪️tests/🧊️catalog-silhouette` compares the drawing of every view with the volume cut by that plane.

Front and plan shapes are projections of one three-dimensional chain: `evaluate` carries every vertex of a front (lateral, up) or plan (ahead, lateral) shape from the rest frame of its bone to the current frame by the same transform that places the bone (`carry`), so swing, spread and twist all read right. A drawing has no depth of its own, so a vertex takes the depth it lacks from the elevation silhouette of the same bone at rest, at the height (front) or forward position (plan) of the vertex (the elevation twin of the same `id` is carried at the middle of its silhouette); a bone without elevation shapes pins its shapes at the rest depth of its origin. `followTip` therefore stretches nothing for a bone that has extent in the view; it only matters where the drawing really has no depth information: a `followTip` rect or poly on a bone that lies along the viewing direction (a wing or jib seen end-on) grows outwards from the origin to the tip by the lateral position across the shape, and a plan capsule with `followTip` follows the origin with its first end and the tip with its second. Every other part that moves out of the bone-parallel directions is a `solid`.

The evaluated state carries the chain itself (`bones[id].frame`, `tip`, `anchors[id].z`, `root`, `kinematics` as 3D vectors), `timeS` is the absolute query time and `clipTimeS` the wrapped clip phase. Painting order follows the nearest point of each shape (`shapeDepth`), with the cameras of `🎨️depiction/🔭️views` as the one source of the projection.

### Solid placements

Parts that rotate away from the bone-parallel directions (blind slats, flaps, booms, umbrella canopies, lids, drums) do not stretch: give the front or plan shape a `solid`. The profile is authored in the **bone's local frame like a side-elevation shape** and placed with the full bone transform, so the three views are right at every pose and clip instant.

| `solid` | Front | Plan |
|---|---|---|
| `{ "kind": "revolved", "axisBone", "axis": "u", "v" or "w", "axisX", "axisZ"?, "mirror"? }` an axisymmetric part about the direction `axis` of the bone-local frame of `axisBone` (u along the bone, v its normal, w lateral) through the rest origin of `axisBone`, offset by `axisX` (side-elevation x) and `axisZ` (lateral, default 0) from that origin: the axis is local to its bone and travels with it (D24) | paper x is the lateral distance from the axis, y the world height; `mirror` adds the reflection so a half profile reads as the whole body | the circle of the widest lateral reach about the axis |
| `{ "kind": "extruded", "sweep": "w" or "v", "z0", "z1", "zTip"? }` a prismatic part: `sweep` is required and never defaulted (D47); `w` sweeps the profile of the bone (u, v) plane laterally (left is +), `v` sweeps a profile of the (u, w) plane along the bone normal | `w`: `z0`..`z1` by the height range of the transformed profile; `v`: the lateral range of the profile by the height of the sweep | `w`: forward range of the transformed profile by `z0`..`z1`; `v`: `z0`..`z1` ahead by the lateral range of the profile |

`solid` is allowed in every viewpoint and excludes `followTip`; `axisBone` must be the shape's own bone or one of its ancestors (D21) and `z0 < z1`. A missing or unknown `sweep` is refused: `evaluate` and the multiview resolver throw `SolidSweepError`, the validator reports `solid-sweep`, the schema requires the member. Vectors: `🧫️fixtures/🧊️solid-placements`, test `🧪️tests/🧊️solid-placements` (shapely oracle); `🧫️fixtures/🧊️solid-sweep`, test `🧪️tests/🧊️solid-sweep` (gl-matrix and ajv).

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

## Volume

`volume` (`Asset.volume`, `Volume`) is the declarative 3D body of an asset: `{ status: "authored", solids, decisions }`, from which the engine-neutral model, the GLB and FBX files and the axonometric view are built. It is required: every catalog asset and every asset fixture has one. A solid is bound to a `bone` and authored in that bone's rest frame in metres: `u` along the bone, `v` its normal in the sagittal plane (the `y` of an elevation shape), `w` lateral (left is +), plus the bone's own `z`; the pose of the bone carries it through every pose and clip.

| `kind` | Fields (bone-local, metres) |
|---|---|
| `box` | `size` [u, v, w] centred on the origin, `cornerRadius` |
| `ellipsoid` | `radii` [u, v, w] centred on the origin |
| `capsule` | `p0`, `p1`, `r0`, `r1` (one may be 0), `lateralScale` stretches the cross-section along `w` |
| `revolve` | `profile` [axial, radial] (simple, radial ≥ 0; open, or closed when the first point equals the last, a torus-like loop), `axis`, `origin`, `sweepRad` (default a full turn), `lateralScale` |
| `extrude` | `profile` (simple polygon), `holes` (strictly inside, apart), `plane` `xy` (profile u,v; along w) / `yz` (v,w; along u) / `zx` (w,u; along v), `from` < `to`, `taperTo` [scaleFrom, scaleTo] |
| `loft` | `sections` ≥ 2 of `at`, `rx` (along w), `ry` (along v for a path along u), `exponent` (2 ellipse), `rotation`; `capStart`, `capEnd` |

Every solid also states `id` (unique), `material` (an asset material), `role`, `tone` (0..1), `region` or `regions` (thermal regions), optionally `maxDenominator` (the largest preset denominator that still shows it), `weights` (up to four `{ bone, weight }` summing to 1; absent = rigid to `bone`), `transform` (`scale`, `rotation` unit quaternion, `translation`, in that order) and `repeat` (`count` 1..256 copies, copy k applies the step `translation`, `rotationRad` about `rotationAxis` through `origin` k times). Validator rules (`🔨️modules/🗂️catalog/🧊️volume`): `volume-empty`, `volume-duplicate-id`, `volume-bone`, `volume-material`, `volume-region`, `volume-weights-bone|count|sum`, `volume-size`, `volume-non-finite`, `volume-extrude-range`, `volume-profile-degenerate`, `volume-profile-self-intersecting`, `volume-hole-outside`, `volume-holes-overlap`, `volume-loft-sections`, `volume-repeat`, `volume-transform`, `volume-max-denominator`. Vectors: `🧫️fixtures/🧊️volume-schema`, test `🧪️tests/🧊️volume-schema` (ajv, shapely and robust-predicates oracles); demonstration: the sample figure.

`ExportFormat` gains `glb` (glTF 2.0 binary of a pose), `fbx` (FBX 7.4 binary of one clip), `model` (the engine-neutral JSON model) and `model-zip` (all of them of one asset); `Viewpoint` gains `axonometric`, the derived 3D view of the volume with no authored shapes (`PLANAR_VIEWPOINTS` are the three authored ones). `glb`, `fbx`, `model` and `model-zip` are built and every catalog asset lists them in `exports` (see 3D models).

## Axonometric view

`viewpoint: "axonometric"` draws the 3D volume of the asset through `depict`, `modelScene` (DXF, DWG), `resolveSheet` (optional fourth cell), the CLI (`--viewpoint axonometric [--camera isometric|dimetric|<azimuth>,<elevation>]`, sheets `--views elevation,front,plan,axonometric`) and casts, in the same seven representations, pens, tones, patterns and legends as the planar views; the colours come from the same looks as the 3D model (`🔨️modules/🎨️depiction/🧊️axonometric`). Pipeline: evaluated bone frames pose the kernel-tessellated solids (chord 0.04 paper mm, `maxDenominator` and the role pens of the representation decide the detail), exact vector hidden-surface removal (nearer-triangle clipping, depth-plane splitting, no raster) leaves convex fragments that are merged per solid and shading band, silhouette, crease and interpenetration lines get exact hidden-line removal. Solid views shade four bands by a fixed light (`shading: "flat"` switches it off); analytic views never shade.

```ts
depict(state, { ...defaultDepictionOptions("light-solid", "en", 50, "axonometric"), axonometric: { kind: "dimetric" } /* | { kind: "isometric" } | { kind: "free", azimuthDeg, elevationDeg } */, shading: "banded" }, analysis, control?)  // DrawingScene
depictAsync(state, options, analysis, control?)  // yields between steps; control = { signal?, progress? }; loads the axonometric module first
```

The axonometric depiction (volume solids, kernel tessellation, hidden lines) is a chunk of its own: the synchronous `depict` and `modelScene` throw `AxonometricNotLoadedError` (named, English and German) for the `axonometric` viewpoint until `await loadAxonometric()` ran; `depictAsync` and `modelSceneAsync` load it themselves, the CLI, the cast exporter and the preview load it when asked for the viewpoint, the web target through its viewpoint gate. A program that draws the planar viewpoints never loads it (the site test checks the entry chunk for the volume module, the kernel and the axonometric depiction).

To scale: the camera is orthographic and the vertical axis is always true (`mm = m · 1000 / denominator`). Isometric: all three axes true; dimetric (ISO 5456-3): ahead and up true, lateral axis ½; free: up true, the horizontal axes by `metadata.projection.axisScale`. Only lengths along the world axes are to scale; diagonals are not. Tests: `🧪️tests/🧊️axonometric-{projection,visibility,depiction,exports}` (three, polygon-clipping, the kernel viewer's rasteriser, sharp, pdfjs, ezdxf as oracles).

## 3D models

`modelOf(asset, options, operation)` (`🔨️modules/🧊️volume`) turns the volume into a procedural `semio.model/v1` document of the neutral kernel (`🧰️framework/🔨️modules/🧊️3d/🗿️model`): right-handed, +Y up, +Z ahead, +X the asset's left, metres, radians, quaternions. The staffage world (x ahead, y up, z left) maps by `(x, y, z) → (z, y, x)`; the node frame of a bone is `(u, v, -w)`, so a solid is mirrored in `w` through its own transform. Nodes are the rig bones at rest, the rig root is `free`, a revolute joint is revolute about the node's `+Z` with its limits in radians, a bone that a pose or clip spreads or twists is `spherical`, a prismatic joint slides along its axis, a fixed one is fixed. Poses are every pose, animations every clip sampled with `evaluate` at the clip fps (root motion, looping, events); no kinematics is repeated. Solids pass through unchanged apart from the frame mirror; a loft section lies in the plane of its own `rotation` (kernel semantics). `maxDenominator` filters the solids by the requested scale.

The seven views are variants of one geometry: `dark-solid`, `light-solid` tone ramp; `dark-line`, `light-line` unlit paper plus feature edges; `thermal` per region colours from the very `thermalPaint` of the 2D view (shared range or `thermalSpanK`, the environment of the caller, else the reference environment, else the asset's own); `forces`, `moments` per bone colours from the very `memberPaint` of the 2D views. Glyph solids (tag `glyph`: force arrows, moment arcs, centre of mass, emitter cones) have the paper sizes of the 2D views at the requested scale and show only in their view. Analytic colours and glyphs animate with the clip.

```
repo tools exec bun -- 🔨️modules/📤️export/🎬️cli/🟦️.ts --asset <slug> --scale 50 --lang en --out <dir> --format glb --pose <slug>
… --format fbx --clip <slug> [--view <view>]     … --format model (--pose <slug> | --clip <slug>)     … --format model-zip
```

A GLB is one static pose (all seven views inside), an FBX one clip in one view (default `light-solid`), `model` the neutral JSON, `model-zip` every pose and clip; the web target adds an owned WebGL2 orbit viewer (`viewpoint="model"`, kernel `🖥️viewer`) and offers the same files as downloads; `--environment <preset>` and `--span-k <K>` steer the thermal view; nonsensical combinations are refused by name in both languages (`ModelRequestError`). Files are named `<asset>-<pose|clip>-<view|all>-model-1to<scale>` and are not checked in. Tests: `🧪️tests/🧊️model-frame`, `-skeleton`, `-looks`, `-bounds`, `-exports`, `-catalog` (gltf-validator, three `GLTFLoader` and `FBXLoader`, the 2D depiction and `evaluate` as oracles).

The files of the catalog as a build artifact (not checked in):

```
repo run @teaching/staffage:export-models [-- --lang en|de --out <dir> --scale 50 --asset a,b --view light-solid --force]
repo tools exec bun -- 🔨️modules/📤️export/📦️batch/🟦️.ts --lang en [--out <dir>] …
```

Default output `📦️packages/🟦️typescript/📤️dist/🧊️models`: `manifest.json` (settings, assets with the SHA-256 of their catalog document, every file with asset, pose or clip, view, kind, relative path, bytes, SHA-256, triangle count) and per asset `<asset>/<asset>-<pose>-all-model-1to50.glb`, `…-<clip>-light-solid-model-1to50.fbx`, `…-<pose>-all-model-1to50.pose.json`, `…-<clip>-all-model-1to50.clip.json`, `<asset>-all-model-1to50.zip` (members `glb/`, `fbx/`, `model/`, byte-identical to `writeModelBundle`). A pose and a clip may share an id, so the model JSON ends `.pose.json` or `.clip.json`. Assets are streamed one at a time, bytes are deterministic, a run resumes (files whose bytes hash to the manifest and whose asset document and settings are unchanged are kept; `--force` after a code change), paths stay under 256 characters, SIGINT cancels with exit 130. The site serves its exports from the browser (lazy exporters, blobs), not from this directory. Tests: `🧪️tests/🧊️model-batch`, `-catalog` (all 152 assets, oracles in `🧪️tests/🧊️model-catalog/🔍️oracles`), `-digests` (sample-figure vectors), `-silhouette`, `-glyph-edges`.

External programs (`ffmpeg` for MP4 and WebM, `rsvg-convert` for frames and the preview, `staffage-dwg`) are reached only through `ToolPort` (`🔨️modules/📤️export/🔌️tools`); an absent one is a `ToolMissingError` that names the program and says what to install in English and German, a cancelled export kills the running child. The DWG writer is a Rust crate with no runtime dependencies (`🔨️modules/📤️export/🖊️dwg/📦️packages/🦀️rust`, project `@teaching/staffage-dwg-rs`) that writes AC1024 (AutoCAD 2010) itself: bit codes, CRCs, the section container, the object map, LWPOLYLINE and TEXT on one layer per ModelScene layer, with insertion units set to metres. AC1024 is chosen because its container (R2004 pages with LZ77 compression, no Reed-Solomon pages as in R2007) and objects (R2010 compact object types, string and handle streams) are fully described by the Open Design Specification, current readers still accept the version, and it keeps the insertion units and the lineweights the exports need; it has not been opened in AutoCAD, the ODA tools or LibreDWG on a machine of this repo, only in the three readers below. `staffage_dwg.wasm` is git-ignored build output next to `🖊️dwg/🟦️.ts`: `repo ./📜️script.ts wasm` in the crate folder (target `@teaching/staffage-dwg-rs:wasm`) compiles it for `wasm32-unknown-unknown` through `repo cargo` with the pinned nightly of `rust-toolchain.toml`, and the tests that need it call that target themselves when the file is missing or older than a source. Without the file `writeDwg` throws `DwgUnavailableError` (bilingual, naming the build command). The files are verified by three independent readers: acadrust and ezdwg-core in `cargo test` (`repo ./📜️script.ts test` in the crate folder) and ezdwg from the locked `dwg` oracle set in the TypeScript tests (CPython 3.10 or newer).

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

A cast of an elevation, front or plan viewpoint that the asset draws as a section also writes `section` (`{ plane, label }`, as declared in `sections` of the representation) to its index.

The index holds the paper extents of the level it was drawn at in `boundsM` (everything drawn) and the level-independent physical truth in `logical`: `heightM` (the physical figure height for every viewpoint, never a paper measure, D13), `figureBoundsM` (the paper extent of the figure in world metres without any air layer: the rig bounds in the side elevation, the drawn shapes of the front and plan), `partsM` (the polygons of frame 0 by shape id) and `maxPaths`. Playback of a cast reads `loop` and `fps` from the index: a clip with `loop: false` (fan spin,
tealight flicker) is repeated by `fitted_staffage(..., loop=True)`.

### Textless casts and the frame cap

Casts are textless: no legend, title, label, dimension, scale bar or ground line, so a cast carries no language and its output is the same
for every language. A schedule of more than 600 frames (the toaster, the laptop, the vacuum run) is thinned to 600 evenly spaced samples
with a fractional `fps` in the index; play such a cast with `rate` (clip seconds divided by scene seconds), at `rate=1` only its first seconds are
seen in coarse steps.

### Levels of detail

Five concepts stay independent: the canonical reference (the cast id and the asset revision), viewport zoom (the Manim `unit`, applied only at placement), the authored print scale (`scale`: mm per metre and the authored geometry and pens), the render level of detail and the render profile. A profile names a level (`lod` of `🎬️clip/🎚️profile`); `🔨️modules/📤️export/🎭️cast/🧊️lod/🔣️.json` defines what each level samples and draws:

| Level | Frames | Shape roles | Patterns | Transport | Use |
|---|---|---|---|---|---|
| `full` | the canonical schedule (600 samples at most) | all | yes | as authored | final renders |
| `preview` | at most half the output rate, at most 90, both endpoints (loops wrap in whole periods) | `body`, `glazing`, `ground` and, per thermal region the roles leave unpainted, its largest shape | no | every 4th parcel and pathline | preview renders and scrubbing |
| `poster` | one, at `phase` of the duration | as `preview` | no | as `preview` | thumbnails and posters |

A level samples the same analytical state at canonical times (a thermal history is a pure function of asset, query and environment, so a sampled frame carries exactly the fills of the full frame at that time) and draws a subset of the same authored shapes; units, origin, anchors, scenario and the clock never change, `logical` is identical for every level, and the paper extent still reaches every shape. The exporter takes `--lod` and `--output-fps` explicitly; the bridge resolves the level from the render profile (`lod_scope` for explicit callers) and a call without either fails. Scope decision: there is no `detailScale` tier in this iteration (a coarser authored geometry source for far use); only frame sampling, roles, patterns and transport density vary. Language-agnostic vectors: `🧫️fixtures/🧊️cast-lod/🔣️.json`, run by `🧪️tests/🧊️cast-lod` and the Python bridge tests.

### Python API (`🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`)

| Call | Result |
|---|---|
| `staffage(cast, unit=, anchor=, color=, opacity=, window=)` | the first frame as a `VGroup`, world origin (the feet) on `anchor`, `unit` Manim units per metre |
| `staffage_motion(cast, unit=, anchor=, rate=1.0, color=, window=)` | a clip or transient swapped from scene time at `rate` times real time; it plays during `wait` and `play` |
| `fitted_staffage(cast, width=, height=, color=, rate=, loop=, window=)` (module `visuals`) | the largest uniform scale that fits `width` and `height`, feet on the origin; with `rate` it plays; `loop` repeats a non-looping clip |
| `staffage_height(cast)` | the physical figure height in metres for every viewpoint (D13), never the air layer of a transport cast |
| `resolve(cast, lod=None, output_fps=None, revision=None, view=, viewpoint=, scale=, pose=, scenario=)` | the cast at a level of detail before anything is decoded: cache folder, level-independent descriptor (height, figure bounds, parts, clock, `maxPaths`) and checked provenance; declared axes are checked against the manifest and a mismatch names the casts that do match |
| `bind_staffage(stage, entity_id, cast_ref, unit=, anchor=, rate=, ...)` | the morph stage binder `staffage`: bounds and anchors frozen from the descriptor, a factory per level |
| `staffage_parts(cast, prefix="")` | the filled polygons of the first frame as world-metre arrays, for attachment points, from `logical.partsM` |
| `ensure_casts(ids=None, progress=None, lod=None)` | renders every missing cast at the active level in one exporter call; `progress` receives each stderr line |

- Sizing is figure-bounded: every cast has its width, height, top, bottom and centre measured on `logical.figureBoundsM` alone, so a
  breathing plume never shrinks or shifts the person and no level of detail changes a layout; `window` (min x, min y, max x, max y in cast metres) keeps only the paths fully inside it and
  sizes by the window, for example a blind without its wall stubs or a head as an icon.
- Cache: casts live in `.🧬semio/🎓️teaching/⚡️cache/staffage/<key>/<cast>/<level>-<revision>` (`SEMIO_TEACHING_CACHE_ROOT` moves it). The key hashes the manifest
  row, the asset's catalog file, the environment presets the cast names, and the cast exporter closure (the exporter, everything it imports, its schema and levels file), the schema and the print profiles with LF line
  endings (editing a module the exporter never imports re-keys nothing), so it is the same on every platform. After a render, key folders no manifest cast maps to and abandoned `.tmp-*` folders older than
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

## Tests and oracles

```
bun node_modules/vitest/vitest.mjs run --configLoader native --config 🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎚️config/🟦️.ts [filter…]     # library, from the repo root
(cd 🎯️targets/⚛️react/📦️packages/🟦️typescript && bun <root>/node_modules/vitest/vitest.mjs run --configLoader native --config ../../🧪️tests/🎚️config/🟦️.ts)   # React target (its support reads the working directory)
```

The Python oracles (shapely, numpy, scipy, svgelements, PyMuPDF, pikepdf, pypdfium2, ezdxf, reportlab, PyAV, imageio-ffmpeg, ezdwg) run from wheels that `🧪️tests/🧰️support/🐍️oracles/uv.lock` pins with hashes for macOS, Linux and Windows, through the pinned `uv`; their environments live in the machine cache of the repo, never in the worktree. `🧪️tests/🧊️model-catalog` exports every asset and checks it with Khronos `gltf-validator`, three and assimp (about 8 minutes, 2.6 GB; `SEMIO_STAFFAGE_MODELS_DIR=<output of export-models>` makes it read a finished batch instead).

## Run

```
repo run @teaching/staffage:test --lang=en
repo run @teaching/staffage-site:dev --lang=en
repo run @teaching/staffage-site:build --lang=en
```
