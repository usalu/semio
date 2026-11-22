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

## Power

Next to size and mass every asset states one **total power**: the sum over all its elements of their **nameplate power** plus the metabolic heat of its living bodies (`🔨️modules/🌡️thermal/⚡️power`: `powerOf(asset)` with the breakdown per element, `totalPowerW(asset)`). It is an installed power, not a measured load and not released heat:

- every element states its nameplate once, on one of its thermal regions (any region, not only heated ones): `ratedPowerW` with `ratedPowerBasis`, tagged by the `source` and `reference` of that region (`authored-estimate` where the ratings are catalogue-typical). The basis says what the plate states: `electrical-input` (appliances, devices, lamps, kettles, hobs, ventilation and heat pumps), `fuel-input` (gas boiler, patio heater, grill, tealight), `rated-output` (combustion engines, traction and drive motors, hoists, generating sets: a 20 t wheel loader counts its 150 kW engine) or `passive` (0 W, stated explicitly);
- **a heat pump counts its electrical input (compressor and fan), not its heating or cooling capacity**; likewise a split air conditioner counts 0.7 kW, not 2.5 kW cooling. Elements fed from elsewhere draw nothing and are `passive` with 0 W: radiators, coils, collectors, cylinders, valves, brake resistors and heat exchangers. `operating.heatW` is heat released to the surroundings and is never summed as power;
- a living body (the primary body and every passenger) contributes its metabolic heat in the default pose, `max(basal, met · metWM2 · skinAreaM2)` at the pose's `activityMet` (else `thermal.baseMet`, else 1 met; passengers 1 met): a person cooking is 175 W of body plus the 1500 W of the hob;
- the default pose does not matter (a switched-off generator keeps its 320 kW) and elements that never run together are summed. An asset without a living body or a rating (trees, a bench) has 0 W. Validator: `power-rated-invalid` (a rating and its basis go together, `passive` is exactly 0 W); the test `🧪️tests/⚡️asset-power` requires a nameplate on every asset with operating heat or a heating circuit.

The manifest carries the value as `powerW` (0.1 W, required) so cards, sorting and filtering need no loaded asset; `validateManifest` reports `manifest-power-mismatch` beyond 0.5 W, the asset file is the single source. The archive app writes it in watts below 1 kW and in kilowatts of three significant digits above, in the reader's number format ("184 W", "1,68 kW"), as a chip next to height and mass on every card and in the compare panes, as "Power (total of all elements)" in "Dimensions and metadata", sorts by "Power" / "Leistung" and filters by a range in watts (address query `w=<min>-<max>`). Tests: `🧪️tests/⚡️asset-power` (hand-derived vectors `🧫️fixtures/⚡️asset-power`, d3-array `fsum` and mathjs as third-party cross-checks), `🎯️targets/⚛️react/🧪️tests/🔎️search-filter-sort`.

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
A view drawn as a section declares it in `sections` of the representation (`elevation`, `front` or `plan`, each `{ plane, label, retained? }`). `retained` names the half of the body the view shows, seen from the viewpoint: `far` (the default) the half behind the plane, the conventional section, `near` the half in front of it; the validator cuts the volume on that side. A hollow part (a sleeve, a pipe, a housing) of the shown half is drawn in a section view as a filled closed shape with its bore as a hole, never as an open outline, because the far inner wall of a hollow part would otherwise fill its bore. The plane is the lateral position z of a vertical cut along the forward axis seen from the right (the part at smaller z is removed) for the side elevation, the forward position x of a vertical cut across the forward axis seen from ahead (the part at larger x is removed) for the front elevation, and the height y of a horizontal cut seen from above (the part above is removed) for the plan. A section view shows the cut faces of the volume and may show everything that remains visible beyond the plane; what lies in front of the plane appears at most as a thin outline (overhead and hidden lines). The side elevation carries the cut-line marker of a front or plan section, the plan the marker of an elevation section; captions read "Section A-A" / "Schnitt A-A" and "Plan section B-B" / "Grundriss-Schnitt B-B". Guard: `🧪️tests/🧊️catalog-silhouette` compares the drawing of every view with the volume cut by that plane.

Front and plan shapes are projections of one three-dimensional chain: `evaluate` carries every vertex of a front (lateral, up) or plan (ahead, lateral) shape from the rest frame of its bone to the current frame by the same transform that places the bone (`carry`), so swing, spread and twist all read right. A drawing has no depth of its own, so a vertex takes the depth it lacks from the elevation silhouette of the same bone at rest, at the height (front) or forward position (plan) of the vertex (the elevation twin of the same `id` is carried at the middle of its silhouette); a bone without elevation shapes pins its shapes at the rest depth of its origin. `followTip` therefore stretches nothing for a bone that has extent in the view; it only matters where the drawing really has no depth information: a `followTip` rect or poly on a bone that lies along the viewing direction (a wing or jib seen end-on) grows outwards from the origin to the tip by the lateral position across the shape, and a plan capsule with `followTip` follows the origin with its first end and the tip with its second. Every other part that moves out of the bone-parallel directions is a `solid`.

The evaluated state carries the chain itself (`bones[id].frame`, `tip`, `anchors[id].z`, `root`, `kinematics` as 3D vectors), `timeS` is the absolute query time and `clipTimeS` the wrapped clip phase. Painting order follows the nearest point of each shape (`shapeDepth`), with the cameras of `🎨️depiction/🔭️views` as the one source of the projection.

### Solid placements

Parts that rotate away from the bone-parallel directions (blind slats, flaps, booms, umbrella canopies, lids, drums) do not stretch: give the front or plan shape a `solid`. The profile is authored in the **bone's local frame like a side-elevation shape** and placed with the full bone transform, so the three views are right at every pose and clip instant.

| `solid` | Front | Plan |
|---|---|---|
| `{ "kind": "revolved", "axisBone", "axis": "u", "v" or "w", "axisX", "axisZ"?, "mirror"? }` an axisymmetric part about the direction `axis` of the bone-local frame of `axisBone` (u along the bone, v its normal, w lateral) through the rest origin of `axisBone`, offset by `axisX` (side-elevation x) and `axisZ` (lateral, default 0) from that origin: the axis is local to its bone and travels with it (D24) | paper x is the lateral distance from the axis, y the world height; `mirror` adds the reflection so a half profile reads as the whole body | the circle of the widest lateral reach about the axis |
| `{ "kind": "extruded", "sweep": "w" or "v", "z0", "z1", "zTip"? }` a prismatic part: `sweep` is required and never defaulted (D47); `w` sweeps the profile of the bone (u, v) plane laterally (left is +), `v` sweeps a profile of the (u, w) plane along the bone normal | `w`: `z0`..`z1` by the height range of the transformed profile; `v`: the lateral range of the profile by the height of the sweep | `w`: forward range of the transformed profile by `z0`..`z1`; `v`: `z0`..`z1` ahead by the lateral range of the profile |

`z0`, `z1` and `zTip` are positions, not offsets from the bone. With `sweep: "w"` they are lateral positions of the REST pose in asset coordinates (world z of the rest pose, left is +, 0 is the centre plane of the asset): `evaluate` subtracts the rest z of the bone origin and carries the remainder with the bone along its w. A part on a bone that sits off the centre plane (a bone with `z`, a left or right limb) therefore states where it lies in the asset: a member 0.08 m thick on a left arm whose rest origin is at z 0.20 is `z0: 0.16, z1: 0.24`, not `-0.04, 0.04`; bone-relative values put the part off by the lateral rest offset of the bone, and it swings about the stem by that offset in a twisted pose. `zTip: [z0, z1]` is the same kind of span at the tip of the bone (the span runs linearly from `z0`..`z1` at the joint to `zTip` at the bone length). With `sweep: "v"` the span is measured from the bone origin along its normal v. Vectors: `lateralRest` in `🧫️fixtures/🧊️solid-sweep`.

`solid` is allowed in every viewpoint and excludes `followTip`; `axisBone` must be the shape's own bone or one of its ancestors (D21) and `z0 < z1`. A missing or unknown `sweep` is refused: `evaluate` and the multiview resolver throw `SolidSweepError`, the validator reports `solid-sweep`, the schema requires the member. Vectors: `🧫️fixtures/🧊️solid-placements`, test `🧪️tests/🧊️solid-placements` (shapely oracle); `🧫️fixtures/🧊️solid-sweep`, test `🧪️tests/🧊️solid-sweep` (gl-matrix and ajv).

### Rest placement of a bone

A bone frame has the axes `u` (along the bone, local x), `v` (its normal, local y) and `w` (its joint axis). The rig is one three-dimensional chain: `R(bone) = R(parent) · R(rest) · R(offset)` and `o(bone) = o(parent) + R(parent) · (x + dx, y + dy, z)`, with `R(angle, abduction, twist) = Rw(angle) · Rv(abduction) · Ru(twist)` for the rest placement and for the offset alike.

| Field of `Bone` | Turns about | Positive sense | Typical use |
| --- | --- | --- | --- |
| `angleDeg` | the joint axis `w` of the parent | `u` towards `v` (counter-clockwise in the side elevation for a sagittal parent) | every bone |
| `abductionDeg` (default 0) | the bone's own normal `v`, after `angleDeg` | `u` towards `w` (to the left of the asset under a sagittal parent) | 90 lays a bone laterally: clavicle, pelvis half, scapula, axle, handlebar |
| `twistDeg` (default 0) | the bone's own axis `u`, after the other two | `v` to `-w`, `w` to `v` | sets the joint axis of the bone, about which `angleDeg` offsets turn it, and the frame its children are placed in |

```json
{ "id": "clavicle-l", "parent": "torso", "x": 0.42, "y": 0, "z": 0.02, "angleDeg": 0, "abductionDeg": 90, "twistDeg": 90, "lengthM": 0.17, "joint": { "type": "revolute", "minDeg": -15, "maxDeg": 40 } },
{ "id": "arm-l", "parent": "clavicle-l", "x": 0.17, "y": 0, "angleDeg": -90, "twistDeg": 90, "lengthM": 0.6, "joint": { "type": "revolute", "minDeg": -60, "maxDeg": 170 } }
```

- Under an upright torso (`u` up, `v` back, `w` left) this clavicle runs to the left with its normal up and its joint axis back, so an `angleDeg` offset lifts the shoulder; the arm hangs from its tip with the frame of a sagittal arm (`u` down, `v` ahead, `w` left), so its poses, clips, 2D shapes and solids are those of a sagittal arm. The right side mirrors `z`, `abductionDeg` and `twistDeg`.
- A pose or clip offset turns a bone about its **own rest axes**, after the rest placement; children ride on their parent (a lifted clavicle carries the arm: let it hang with the opposite `abductionDeg` on the arm).
- A rig whose bones all rest in the sagittal plane (no `abductionDeg`, no `twistDeg`) is solved by the plain planar composition, exactly as before; `Bone.z` alone never leaves the plane.
- Elevation shapes are drawn in the bone's own (`u`, `v`) plane and projected, so a flat elevation shape on a bone that rests inclined is the exact foreshortened picture of a plate in that plane: the rest placement says where the plane stands, and the drawing is checked against the body like every other (`volumes-silhouette`). `lateral-ambiguous` reports only what neither the rig nor a solid places: a pose or clip that turns the plane against its rest placement carries the drawing by `A = [u₁ v₁]·[u₀ v₀]⁻¹` (paper projections of the bone axes at rest and now) while a point `c` beside the plane goes `c·(w₁ − A·w₀)` further, so the paper error is `min(extent of the shape, largest |w| of the body solids of the bone at that scale) · |w₁ − A·w₀| · 1000 / denominator` against the 0.2 mm print minimum (`|w₁ − A·w₀|` is `sin θ` for a bone resting in the paper and 0 for a bone that only moves with its parents in the sagittal plane; without a 3D model at that scale the extent of the shape alone bounds the part). A plane that rests edge-on (a clavicle laid sideways) has no flat drawing at all and is always reported. Remedies: `solid` shapes (`extruded` with `sweep` `u` for a bar along the bone), or the body solids of the bone close to its plane (a part far beside the plane belongs to a bone placed there with `z`). Example: a scapula resting 12° out and swung 25° about its own axis leans 0.0919, so a blade whose solids reach 44 mm beside the plane errs 0.08 mm at 1:50 and is accepted; vectors `lateralAmbiguous` of `🧫️fixtures/✅️asset-validation`, test `🧪️tests/✅️asset-validation` (numpy). Front and plan shapes are drawn in the rest pose of the world and carried by the bone, whatever its rest placement.
- 3D solids (`volumes`) are written in the (`u`, `v`, `w`) frame of their bone: when a bone's rest frame changes, its solids are re-expressed in the new frame.
- Vectors: `🧫️fixtures/🦴️lateral-rest` (hand-computed frames of clavicles with hanging arms, a shrug, a retraction, a scapula with a general rest placement); test `🧪️tests/🦴️lateral-rest` (scipy `Rotation` for the chain and the node placements, numpy for the anchors, shapely for the solids); demonstration: the shoulder girdle of `standing-adult`.

### Lateral spread

A sagittal bone moves forward and up, so wings spread sideways, arms raised sideways and straddled legs need a degree of freedom the front and plan views can read: `abductionDeg` in a pose or clip key (`BoneOffset`, revolute joints, -180 to 180 degrees).

```json
"bones": { "arm-n": { "angleDeg": -46, "abductionDeg": -75 }, "arm-f": { "angleDeg": -32, "abductionDeg": 75 } }
```

- After its own rotation about its joint axis the bone turns out of its rest plane about its own normal (out of the sagittal plane for a sagittal bone). A point at distance `s` along the bone is `s·sin φ` to the left (+z, the left of the asset; the right-hand side takes negative values, so the near wing of a right-facing side view is negative) and keeps `s·cos φ` of its sagittal reach. Children ride on the spread bone, so a hand follows the spread arm; interpolating poses and clips interpolate it.
- **All three views agree in every pose.** `solveRig` solves the chain in three dimensions whenever a bone is spread and the side elevation (`elevation`) is its orthographic projection: the bone matrix carries the projected axes, so elevation shapes foreshorten by `cos φ` exactly like the front and plan views, and placement, mass, thermal, structure, kinematics and grounding all read the same chain. Without a spread, at rest or by an offset, the plain planar solve is used. The guard checks the agreement (`spread-*` checks of the three-views test) for every pose and clip instant with a spread bone.
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

`volumes` (`Asset.volumes`, `Volumes`) holds the declarative 3D models of an asset, one individually handcrafted model per preset scale keyed `"50"`, `"100"`, `"200"`, `"500"` like `representations`: `{ status: "authored", denominator, body, thermal, structure, decisions }` or `{ status: "unavailable", denominator }` when neither a digital model nor a drawing exists. An authored digital scale can coexist with an unavailable print drawing. The viewer and 3D exports use the volume status. `body` carries the tone views, `thermal` the thermal model made spatial (every solid names its `region`, and a `layer` for nested shells) and `structure` the structural idealisation made spatial (every solid names its `member` and `part`); a thermal or structural solid may be an `{ of }` reference to a body solid. From these the engine-neutral model, the GLB and FBX files and the axonometric view are built. It is required: every catalog asset and every asset fixture has one. A solid is bound to a `bone` and authored in that bone's rest frame in metres: `u` along the bone, `v` its normal in the sagittal plane (the `y` of an elevation shape), `w` lateral (left is +), plus the bone's own `z`; the pose of the bone carries it through every pose and clip.

| `kind` | Fields (bone-local, metres) |
|---|---|
| `box` | `size` [u, v, w] centred on the origin, `cornerRadius` |
| `ellipsoid` | `radii` [u, v, w] centred on the origin |
| `capsule` | `p0`, `p1`, `r0`, `r1` (one may be 0), `lateralScale` stretches the cross-section along `w` |
| `revolve` | `profile` [axial, radial] (simple, radial ≥ 0; a first point equal to the last is a closed loop, the enclosed body without the axis: a shell, wall, bead or torus; every other profile is open and closed through the axis, so one that stays off the axis at both ends is a filled body, never a shell: `volume-revolve-implied-closure`), `axis`, `origin`, `sweepRad` (default a full turn), `lateralScale` |
| `extrude` | `profile` (simple polygon), `holes` (strictly inside, apart), `plane` `xy` (profile u,v; along w) / `yz` (v,w; along u) / `zx` (w,u; along v), `from` < `to`, `taperTo` [scale at from, scale at to] about the centroid of the outer ring (not a scale along u and v: `[1, 0]` is a pyramid, equal factors are no taper: `volume-taper-uniform`, `volume-taper-degenerate`) |
| `loft` | `sections` ≥ 2 of `at`, `rx`, `ry`, `exponent` (2 ellipse), `rotation` (quaternion `[x, y, z, w]`); a section is `\|x/rx\|^n + \|y/ry\|^n = 1` in its own x, y plane. `rx` is the half extent along the section's x and `ry` along its y, and which bone axes these are is decided by `rotation` alone: with the limb rotation `[0.5, 0.5, 0.5, 0.5]` (x = v, y = w, path along u) `rx` lies along v (half depth, in the sagittal plane) and `ry` along w (half width, lateral); with the rotation about v `[0, 0.7071, 0, 0.7071]` `rx` lies along w and `ry` along v; without `rotation` x is u projected perpendicular to the path (w when the path runs exactly along u) and y = path × x, so a path exactly along u has `rx` along w and `ry` along v, and the slightest lean of that path towards v swaps them. Recommendation: a loft that runs along its bone states `rotation: [0.5, 0.5, 0.5, 0.5]` on every section and reads `rx` = half depth (v), `ry` = half width (w); never rely on the section frame without `rotation` there (vectors `loftAxes` in `🧫️fixtures/🧊️model-frame`); `capStart`, `capEnd` |

Every solid also states `id` (unique), `material` (an asset material), `role`, `tone` (0..1), optionally `slender`, `sheet`, `envelope`, `texture`, `sectionFaces`, `weights` (up to four `{ bone, weight }` summing to 1; absent = rigid to `bone`), `transform` (`scale`, `rotation` unit quaternion, `translation`, in that order) and `repeat` (`count` 1..256 copies, copy k applies the step `translation`, `rotationRad` about `rotationAxis` through `origin` k times). Validator rules (`🔨️modules/🗂️catalog/🧊️volume`): `volume-empty`, `volume-duplicate-id`, `volume-bone`, `volume-material`, `volume-region`, `volume-weights-bone|count|sum`, `volume-size`, `volume-non-finite`, `volume-extrude-range`, `volume-profile-degenerate`, `volume-profile-self-intersecting`, `volume-hole-outside`, `volume-holes-overlap`, `volume-loft-sections`, `volume-repeat`, `volume-transform`, `volume-of`, `volume-layer`, `volume-member`, `volume-anchor`, `volume-envelope`; the per-scale rules are `volumes-availability`, `-bones`, `-bounds`, `-silhouette`, `-feature`, `-monotonic`, `-cross-scale`, `-thermal-coverage`, `-structure-coverage`, `-solids`, `-copied`, `-slender`, `-views-disagree`, `-connected`. Vectors: `🧫️fixtures/🧊️volume-schema`, test `🧪️tests/🧊️volume-schema` (ajv, shapely and robust-predicates oracles); demonstration: the sample figure.

`volumes-thermal-coverage` (measured part, `🔨️modules/🗂️catalog/🧊️volumes/🧮️measures`, `🌊️flood`): the area compared with `areaM2` is the outer surface of a region, the faces that the air flooded from outside through the thermal solids of the scale reaches. The flood cell is a quarter of the smallest feature: a gap of at most a quarter feature (1.9, 3.75, 7.5, 19 mm) is closed, an opening of half a feature or more is open, so the inside of a closed shell (the bore of a capped tube, a hollow flange, a sensor cap, a casing of panels) never counts and a hollow part is modelled hollow. A face that a solid of another region covers (inside it or within half a feature in front of it) still counts only when that solid stands between the face and the air (walking on along the normal, the first free space beyond it is reached by the air): a neck, a spigot or a sensor that passes through a closed shell covers nothing inside it. A region that is contents of a closed cavity counts its whole surface (E3) and is `hidden` unless a glazing region lets one see it. A real opening narrower than half a feature of a coarse scale (a lid slot, a running gap) is declared once as `thermal.minimumAirPassageM` (the narrowest real air passage of the asset in metres): the flood cell is then at most a third of it at every scale, so the opening stays open without redrawing the gap wider; a flood whose budget cannot keep that cell is a finding. Vectors: `🧫️fixtures/🧊️volumes-flood`, test `🧪️tests/🧊️volumes-flood` (hand values and an independent voxel flood on the kernel's triangles with trimesh, numpy and scipy).

`volumes-connected` (measured, `🔨️modules/🗂️catalog/🧊️volumes/🔗️connected`): in `presentation.defaultPose` the body of every scale is one object and every load path of the structure ends in a support. The parts are the copies of the solids (a `repeat` gives one part per copy); two parts touch when the triangles of the kernel come within a fifth of the smallest feature of the scale of each other (1.5, 3, 6, 15 mm at 1:50, 1:100, 1:200, 1:500; exact triangle distances, no bounding boxes) or one lies inside the other. Body: a group of touching parts apart from the largest must rest on the ground plane (y = 0) or at a `contact` or `support` anchor, else it floats (a mirror beside the door, a table top over its column); a group that rests on its own is a second object (a lamp stand beside the table, a basket beside the gardener) only when no movable joint of the rig ties its bones to bones of another group (a wheel belongs on its axle, a shoe on its shank; `fixed` joints only place a bone) and when it stands more than one smallest feature clear of everything else. Structure: every group that is not only `mass` solids (markers at centres of mass) contains a `support` solid or rests on the ground or at a `contact` or `support` anchor, directly or through one body solid that stands there and that it touches or lies in (the sole under a foot bone, the tyre around a rim); in a list without any `support` solid the largest group is the structure and every other group joins it; so members reach their joints, each other and what carries them; such groups keep the same clearance. No flag, role or tag switches a solid off; emitters live in the thermal list, which the rule does not read. A finding names the worst groups: first solid, number of further solids, gap, nearest solid, joint; in the structure list the gap is never measured to a `mass` marker and the nearest solid is never one, because a mass is no load path. `volumes-feature` judges every scale for its own smallest feature (7.5, 15, 30, 75 mm): a solid of a coarser scale gains nothing from being identical to a 1:50 solid. A long part keeps its true section at every scale by declaring `slender: true` (`volumes-slender` checks the declaration: at least 20 times as long as its largest cross extent along its centre line; a `member` solid of the `structure` list, a bone or a strut that is drawn by its axis, from 5 times and a length of two smallest features, so the long bones that `🧪️tests/🧊️model-skeleton` pins at one shaft section for every scale are declared slender and never thickened); a compact solid below the feature (a knob, a bolt head, a finger bone at 1:500) is merged into its neighbour or left out at that scale. A coarse skeleton is merged the same way: one `member` solid of the `structure` list may stand for several bones by naming the further ones in `members` beside its `member` (a spine of five segments as one rod, a tail as one bar, a skull that takes the mandible in). The named bones are one connected piece of the rig (each the parent or a child of another; `volume-member`) and, in the default pose, each lies from joint to tip in the solid or within one smallest feature of its surface (`volumes-structure-coverage`, measurement `spanGapM:<solid>:<bone>`): what the scale cannot show apart is one solid. That solid is the member solid of every bone it names, is judged for `slender` by its own length (so a run of short bones is one long bar at its true section) and stays coloured and loaded as its `member` bone in the structural views. A thin sheet of any kind (feather vane, leaf blade, fabric, membrane, foil, sheet metal, thin board or panel) keeps its physical thickness below the feature only when it is declared, the way a `slender` solid is: with `sheet: true`, or by a thermal layer that declares a gauge (`thicknessM`) for its bone and material, in which case the thickness is that gauge. `volumes-feature` checks the declaration (`sheetRatio:<id>`): the middle extent of the sheet (for a planar extrusion the smallest width of its profile, for a ring revolved from a closed loop off the axis the radial width of its band, so a hoop is a bent rod that declares `slender` and never a sheet) is at least 20 times its thickness and, where the sheet is below the feature, at least the feature; `sheet` together with `slender` fails, an undeclared sheet-shaped solid fails like any thin solid and is told to declare `sheet: true`, and a lump or a strip is never a sheet. A declared sheet that is at least as thick as the feature keeps the flag, so the same part carries it at every scale; a hollow vessel built as a closed-loop `revolve` or as an `extrude` with `holes` is judged by its overall extents, so its wall may be true and needs no flag. Vectors: `🧫️fixtures/🧊️volumes-connected`, test `🧪️tests/🧊️volumes-connected` (trimesh closest points and containment, scipy connected components).

2D and 3D agreement (`volumes-silhouette`, `volumes-cross-scale`, `volumes-views-disagree`; `🔨️modules/🗂️catalog/🧊️volumes`, `🌓️silhouette`):

- **Outline strokes.** A filled shape is compared by its rings; its pen is drawing convention. An unfilled closed outline is the edge of a body when the solids as they are (a slender solid at its true section) fill at least half of what it encloses (`bodyOutline`): it then counts as filled up to its centre line, and a slender solid that runs along it (a rim wire, an edge band) is compared at its true width, not at the pen. So the outline of a part is drawn with its centre line on the body edge; it need not be moved inside by half a pen. An outline around a void (a hoop, a frame) and every open stroke are geometry at pen width on both sides: the slender solid they stand for is drawn at the pen of the stroke it follows (C24), and a stroke that stands for nothing is a difference. Measured on `shopping-trolley` 1:500 plan (rim outline of 0.13 mm = 65 mm centred on the basket edge): IoU 0.898 before (the rim wires were widened to the pen and stood 32 mm outside the drawing all round), 0.955 now.
- **Typification.** A coarser scale may draw a row of many thin elements as fewer, thicker ones (balusters, fence pales, louvres). Give the row a bone of its own (a fixed joint), model the typified elements at the coarse scale as solids with `envelope: true` (a `repeat` is the natural form) and say so in `decisions`; draw the same elements in the 2D of that scale. `volumes-cross-scale` then compares the convex hull of the `envelope` solids of that bone with the convex hull of the finer solids of that bone (the common envelope of the row) instead of post against rod, and leaves both out of the volume band; `volumes-silhouette` compares the coarse 3D with its own 2D as always. A convex envelope prism (truss, mast) is its own hull, so the older use is unchanged. Without the declaration a typified row fails against the rods it replaces.
- **Views.** `volumes-views-disagree` asks two views of a scale for the same extent and the same place of the whole asset along the axis they share (both ends of the interval, `positionDifferenceM:<axis>`), within max(one smallest feature, 2 % of the larger extent). Per bone the differences are information only (`info:extentDifferenceM:<bone>:<axis>`, `info:positionDifferenceM:<bone>:<axis>`): a bone is legitimately drawn together with other parts in another view.
- Vectors `typified`, `follow.bodyEdge`, `viewsPosition` of `🧫️fixtures/🧊️volumes-rules`; test `🧪️tests/🧊️volumes-rules` (shapely convex hulls, unions and bounds).

`ExportFormat` gains `glb` (glTF 2.0 binary of a pose), `fbx` (FBX 7.4 binary of one clip), `model` (the engine-neutral JSON model) and `model-zip` (all of them of one asset); `Viewpoint` gains `axonometric`, the derived 3D view of the volume with no authored shapes (`PLANAR_VIEWPOINTS` are the three authored ones). `glb`, `fbx`, `model` and `model-zip` are built and every catalog asset lists them in `exports` (see 3D models).

## Axonometric view

`viewpoint: "axonometric"` draws the 3D volume of the asset through `depict`, `modelScene` (DXF, DWG), `resolveSheet` (optional fourth cell), the CLI (`--viewpoint axonometric [--camera isometric|dimetric|<azimuth>,<elevation>]`, sheets `--views elevation,front,plan,axonometric`) and casts, in the same seven representations, pens, tones, patterns and legends as the planar views; the colours come from the same looks as the 3D model (`🔨️modules/🎨️depiction/🧊️axonometric`). Pipeline: evaluated bone frames pose the kernel-tessellated solids (chord 0.04 paper mm; the list of the drawn scale's own authored volume and the role pens of the representation decide the detail), exact vector hidden-surface removal (nearer-triangle clipping, depth-plane splitting, no raster) leaves convex fragments that are merged per solid and shading band, silhouette, crease and interpenetration lines get exact hidden-line removal. Solid views shade four bands by a fixed light (`shading: "flat"` switches it off); analytic views never shade.

```ts
depict(state, { ...defaultDepictionOptions("light-solid", "en", 50, "axonometric"), axonometric: { kind: "dimetric" } /* | { kind: "isometric" } | { kind: "free", azimuthDeg, elevationDeg } */, shading: "banded" }, analysis, control?)  // DrawingScene
depictAsync(state, options, analysis, control?)  // yields between steps; control = { signal?, progress? }; loads the axonometric module first
```

The axonometric depiction (volume solids, kernel tessellation, hidden lines) is a chunk of its own: the synchronous `depict` and `modelScene` throw `AxonometricNotLoadedError` (named, English and German) for the `axonometric` viewpoint until `await loadAxonometric()` ran; `depictAsync` and `modelSceneAsync` load it themselves, the CLI, the cast exporter and the preview load it when asked for the viewpoint, the web target through its viewpoint gate. A program that draws the planar viewpoints never loads it (the site test checks the entry chunk for the volume module, the kernel and the axonometric depiction).

To scale: the camera is orthographic and the vertical axis is always true (`mm = m · 1000 / denominator`). Isometric: all three axes true; dimetric (ISO 5456-3): ahead and up true, lateral axis ½; free: up true, the horizontal axes by `metadata.projection.axisScale`. Only lengths along the world axes are to scale; diagonals are not. Tests: `🧪️tests/🧊️axonometric-{projection,visibility,depiction,exports}` (three, polygon-clipping, the kernel viewer's rasteriser, sharp, pdfjs, ezdxf as oracles).

## 3D models

`modelOf(asset, { scale, view, … }, operation)` and `modelOfAsset(asset, { scales?, … }, operation)` (`🔨️modules/🧊️volume`) turn the individually authored volumes (`asset.volumes[scale]`, a `body`, a `thermal` and a `structure` list per scale) into procedural `semio.model/v1` documents of the neutral kernel (`🧰️framework/🔨️modules/🧊️3d/🗿️model`): `modelOf` holds one scale and the mode of one view, `modelOfAsset` one kernel scene per authored scale and mode (`s50-body`, `s50-thermal`, `s50-structure`, `s100-body`, …) over one skeleton, every scale and mode with its own solids on scene-specific identity child nodes of the bones. A scale that is not authored is refused by name (`model-scale-unavailable`, `model-scale-unknown`), never answered by another scale. Frames: right-handed, +Y up, +Z ahead, +X the asset's left, metres, radians, quaternions. The staffage world (x ahead, y up, z left) maps by `(x, y, z) → (z, y, x)`; the node frame of a bone is `(u, v, -w)`, so a solid is mirrored in `w` through its own transform. Nodes are the rig bones at rest, the rig root is `free`, a revolute joint is revolute about the node's `+Z` with its limits in radians, a bone that a pose or clip spreads or twists is `spherical`, a prismatic joint slides along its axis, a fixed one is fixed. Poses are every pose, animations every clip sampled with `evaluate` at the clip fps (root motion, looping, events); no kinematics is repeated and the analyses are solved once per instant for every scale and mode. Solids pass through unchanged apart from the frame mirror; a loft section lies in the plane of its own `rotation` (kernel semantics); `{ of }` references of the thermal and structure lists share the geometry of their body solid.

The seven views are variants over the geometry of their mode: `dark-solid`, `light-solid` tone ramp and `dark-line`, `light-line` unlit paper plus feature edges over the `body` list; `thermal` over the `thermal` list, one material per region and layer (the inner shells of a multi-layer region show the skin value) with colours from the very `thermalPaint` of the 2D view (shared range or `thermalSpanK`, the environment of the caller, else the reference environment, else the asset's own); `forces`, `moments` over the `structure` list, member and joint solids with the colour of their member bone from the very `memberPaint` of the 2D views, supports and masses neutral. Glyph solids (tag `glyph`: force arrows, moment arcs, centre of mass, emitter cones) have the paper sizes of the 2D views at the requested scale, capped in model space (amendment D1) at 0.35 (arrows, plumes; moment arcs half of it as radius) and 0.06 (centre-of-mass radius) of the largest extent of the asset so that at 1:200 and 1:500 the body still fills the frame, and show only in their view; the 2D sheets and the axonometric drawing keep pure paper sizes. Analytic colours and glyphs animate with the clip.

```
repo tools exec bun -- 🔨️modules/📤️export/🎬️cli/🟦️.ts --asset <slug> --lang en --out <dir> --format glb --pose <slug> [--scale 100]
… --format fbx --clip <slug> --scale 50 [--view <view>]     … --format model (--pose <slug> | --clip <slug>) [--scale 100]     … --format model-zip [--scale 100]
```

Interactive 3D (`🎯️targets/⚛️react`, `🔨️modules/🧊️spatial`): the stage toolbar has a scale group (`1:50`, `1:100`, `1:200`, `1:500`; a scale without a model is `aria-disabled` with the reason of its representation, else the scales that exist) and a mode group (the seven views, which select the `body`, `thermal` or `structure` scene and its material variant), both native buttons with arrow-key, Home and End navigation, announced through the viewer's polite status ("Scale 1:50, thermal model, shown as thermal.", German likewise). The stage reports a choice through `onScale` and `onView`; the viewer page patches its one `ViewerState`, the `<staffage-figure>` element sets the attributes `scale` and `view` and fires `staffage-scale` and `staffage-view`, so 2D drawings and 3D model always show the same scale and view. Tests: `🎯️targets/⚛️react/🧪️tests/🎚️model-controls`.

A GLB is one static pose with every authored scale and mode as glTF scenes (`scene` 0 is the finest body; all seven views inside as variants), an FBX one clip at one scale in one view (default `light-solid`, one scene), `model` the multi-scene neutral JSON of a pose or clip, `model-zip` every pose and clip (without `--scale` they hold every authored scale, with it only that scale); the web target adds an owned WebGL2 orbit viewer (`viewpoint="model"`, kernel `🖥️viewer`) and offers the same files as downloads; `--environment <preset>` and `--span-k <K>` steer the thermal view; nonsensical combinations are refused by name in both languages (`ModelRequestError`). Files are named `<asset>.<pose>.glb`, `<asset>.<pose>.pose.json`, `<asset>.<clip>.clip.json`, `<asset>.<clip>.s<scale>.<view>.fbx` and `<asset>.zip` (`🔨️modules/📤️export/🗿️model/🏷️names`, the same names in the library, the batch, the command line and the React downloads) and are not checked in. One tessellation rule serves the viewer, every download and the budget oracle (`qualityOf(asset, scale)`: 0.1 mm of paper at the scale between 1/800 and 1/60 of the largest dimension, fewer segments at coarser scales), applied scene by scene (`🗿️model/🧩️scenes`); the clip keeps its root motion in the files and in the viewer, which follows the figure. Tests: `🧪️tests/🧊️model-frame`, `-skeleton`, `-looks`, `-bounds`, `-exports`, `-catalog` (gltf-validator, three `GLTFLoader` and `FBXLoader`, the 2D depiction and `evaluate` as oracles).

The files of the catalog as a build artifact (not checked in):

```
repo run @teaching/staffage:export-models [-- --lang en|de --out <dir> --scale 50,100 --asset a,b --view light-solid,thermal --force]
repo tools exec bun -- 🔨️modules/📤️export/📦️batch/🟦️.ts --lang en [--out <dir>] …
```

Default output `📦️packages/🟦️typescript/📤️dist/🧊️models`: `manifest.json` (settings of the last run, assets with the SHA-256 of their catalog document, every file with asset, pose or clip, scale, view, kind, relative path, bytes, SHA-256, triangle count and the content digest `key` it was written with) and per asset the files of the names above (ZIP members `glb/`, `fbx/`, `model/`, byte-identical to `writeModelBundle` for the files that exist). One run exports every authored scale of an asset (`--scale` and `--view` restrict only the FBX files of that run) and never deletes a file of another scale: the only files removed from an asset folder are those its complete plan no longer names; an asset without an authored scale is listed in the summary and not exported. Assets are streamed one at a time, bytes are deterministic, a run resumes where a file's `key` (digest of the asset document, the producing code, the schema and kernel, the environment presets and the thermal options) equals the current one and its bytes still hash to the manifest, and SIGINT, SIGTERM, SIGHUP (POSIX) or SIGBREAK (Windows) cancel between and inside files; every path stays under 256 characters. Tests: `🧪️tests/🧊️model-batch` (a temporary catalog of the four-scale sample figure; scale 50 then scale 100 into one directory keep both), `-scales`, `-scale-files`, `-catalog` (every catalog asset at all four digital scales).

External programs (`ffmpeg` for MP4 and WebM, `rsvg-convert` for frames and the preview, `staffage-dwg`) are reached only through `ToolPort` (`🔨️modules/📤️export/🔌️tools`); an absent one is a `ToolMissingError` that names the program and says what to install in English and German, a cancelled export kills the running child. The DWG writer is a Rust crate with no runtime dependencies (`🔨️modules/📤️export/🖊️dwg/📦️packages/🦀️rust`, project `@teaching/staffage-dwg-rs`) that writes AC1024 (AutoCAD 2010) itself: bit codes, CRCs, the section container, the object map, LWPOLYLINE and TEXT on one layer per ModelScene layer, with insertion units set to metres. AC1024 is chosen because its container (R2004 pages with LZ77 compression, no Reed-Solomon pages as in R2007) and objects (R2010 compact object types, string and handle streams) are fully described by the Open Design Specification, current readers still accept the version, and it keeps the insertion units and the lineweights the exports need; it has not been opened in AutoCAD, the ODA tools or LibreDWG on a machine of this repo, only in the three readers below. `staffage_dwg.wasm` is git-ignored build output next to `🖊️dwg/🟦️.ts`: `repo ./📜️script.ts wasm` in the crate folder (target `@teaching/staffage-dwg-rs:wasm`) compiles it for `wasm32-unknown-unknown` through `repo cargo` with the pinned nightly of `rust-toolchain.toml`, and the tests that need it call that target themselves when the file is missing or older than a source. Without the file `writeDwg` throws `DwgUnavailableError` (bilingual, naming the build command). The files are verified by three independent readers: acadrust and ezdwg-core in `cargo test` (`repo ./📜️script.ts test` in the crate folder) and ezdwg from the locked `dwg` oracle set in the TypeScript tests (CPython 3.10 or newer).

## Playgrounds

The archive's playground is one shared scene of placed assets with three ways to look at it, each at its own address: the **section** (`#/playground`, side or front elevation by `vp=elevation|front`), the **floor plan** (`#/playground/plan`, the plan viewpoint) and the **3D playground** (`#/playground/3d`, the neutral model of every placed asset in one scene, loaded on demand like the 3D viewer). A placement is one position in the world frame of the Conventions (`xM` ahead, `zM` to the asset's left, `yawDeg` turning counter-clockwise seen from above), a pose or clip, and a window on one timeline (`startS`, `durationS` or to the end, `speed`, `loop`, `offsetS`, and `holdS` that pauses it at one clip time). The section puts x along the paper and z in depth (front: z along the paper, the ground depth `depthM` long), the plan puts x to the right and z up the page, the 3D scene puts a placement at `(z, 0, x)` of the neutral frame (`🧰️framework/🔨️modules/🧊️3d/🗿️model`), so one position is at one place everywhere. A walking figure is planted on its ground point in all three. The plan also draws the yaw (a pure 2D turn about the ground point, positive yaw turning the forward direction up the page, to the left) and a mirrored placement (flipped across its forward axis); the side elevation mirrors about the vertical axis, the front elevation and the 3D scene draw a mirrored placement unmirrored and say so.

One shared clock plays the timeline (play, pause, step, scrub, rate, loop); every figure can also be paused, scrubbed, looped, sped up and moved on the timeline on its own, by pointer or keyboard sliders. The 3D scene bakes each placement's clip into one master animation with the same window rules (`🎯️targets/⚛️react/🔨️modules/🧪️playground/🧊️model/🎞️retime`), prepares the models with progress and cancellation, and follows the catalog: a new revision of an asset is a new object and so a new model, and a pose or clip it no longer has is named while the default plays. Guards: `🎯️targets/⚛️react/🧪️tests/🎞️playground-timeline` (three and the library's `evaluate` as oracles) and `🧪️playground-tabs`.

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
