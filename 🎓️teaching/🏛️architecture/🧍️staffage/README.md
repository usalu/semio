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

A query is a pose, a clip at an arbitrary time, or a transition between two poses. The same state drives all seven views — dark line,
dark solid, light line, light solid, thermodynamic, structural forces, structural moments — so scrubbing, comparing or exporting can never
desynchronise geometry and analysis.

## Layout

| Path | Content |
|---|---|
| `🧬️schema/🔣️.json` | `semio.staffage/v1`, the normative contract (`🟦️.ts` is its typed twin) |
| `🗂️catalog/🔣️.json` | the manifest: categories, collections, one row per asset with its completion status |
| `🗂️catalog/<category>/🔣️.json` | the handcrafted assets of one category |
| `🔨️modules/` | `🔗️contracts` · `📐️units` · `🦴️rig` · `🎞️animation` · `🧮️evaluation` · `🗂️catalog` · `🌡️thermal` · `🏗️structure` · `🎨️depiction` · `📤️export` |
| `📦️packages/🟦️typescript` | library `@teaching/staffage` |
| `🎯️targets/⚛️react/📦️packages/🟦️typescript` | `@teaching/staffage-react`: `StaffageFigure`, the `<staffage-figure>` element and the archive app |
| `🌐️site/` | `@teaching/staffage-site`: the website shell and its Vite build |
| `🧪️tests/`, `🧫️fixtures/` | test cases (Vitest, Gherkin, Python oracles) and language-agnostic vectors |

## Conventions

- SI internally: m, kg, s, N, Pa, W; temperatures authored in °C. World x right, y up, ground at y = 0; angles in degrees, CCW positive.
- Drawing scale only maps model metres to paper millimetres (`mm = m · 1000 / denominator`) and selects line weights, level of detail
  and annotation sizes. It never changes dimensions, masses, temperatures or loads. Viewport zoom is a separate screen transform.
  Presets 1:500, 1:200, 1:100, 1:50; any positive denominator is allowed.
- Axial force N > 0 is tension; bending moment M > 0 is sagging; joint moments are CCW positive. Compression is drawn warm, tension cool,
  with hatch patterns as a colour-independent reading.
- Every physical parameter is tagged `measured`, `literature`, `authored-estimate` or `illustrative`. No parameter claims measured accuracy;
  thermal fields are engineering estimates, not CFD; human joint loads are rigid-segment resultants, not clinical bone or muscle forces.
- All text is `{ en, de }`; there is no default language.

## Run

```
repo run @teaching/staffage:test --lang=en
repo run @teaching/staffage-site:dev --lang=en
repo run @teaching/staffage-site:build --lang=en
```
