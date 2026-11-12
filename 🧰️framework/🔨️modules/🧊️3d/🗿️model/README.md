# 🗿️ Model kernel (`semio.model/v1`)

A domain-neutral, engine-neutral 3D model kernel: one JSON schema for procedural solids, rigs, poses, animations and materials, and the code that turns it into triangles, GLB and FBX files and a picture. It has no runtime dependency on any engine or library (three, Blender, FBX SDK, glTF libraries); third-party libraries are test oracles only (`🔮️oracles/🔣️.json`).

Staffage builds its documents from the asset volume (`🎓️teaching/🏛️architecture/🧍️staffage`, `modelOf`); nothing in this folder knows staffage.

## Contract

- Frame: right-handed, +Y up, +Z ahead, +X left, metres, radians, quaternions `[x, y, z, w]`, column-major 4×4 matrices. All text is `{ en, de }`; there is no default language.
- `🧬️schema/🔣️.json` is normative (JSON Schema draft-07, 31 `$defs`), `🧬️schema/🟦️.ts` its typed twin. A document is procedural (`solids`, no meshes) or resolved (`tessellate` turned every solid into primitives skinned to its node; `Primitive.solids` keeps which solid made which triangles). Solid kinds: `box`, `ellipsoid`, `capsule`, `revolve`, `extrude`, `loft`, each with an optional `repeat`.
- Every long function takes an `Operation { report(fraction), cancelled() }` and stops by `CancelledError`; polling cannot interrupt a synchronous function that never yields, so deadlines and the async variants (`tessellate`, `featureEdges`, `sceneOfAsync`) are the way to cancel.
- Identical input gives identical bytes.

## API (`./🟦️.ts`, package export `@semio-tech/framework-3d-js/model`)

| Call | Result |
|---|---|
| `validateModel(document)` | `Issue[]`, no throw for invalid data |
| `tessellate(document, quality, operation)` | resolved document; `tessellateSolid` for one solid |
| `poseModel(document, selector)` | node world matrices at a pose or an animation time; `sampleMaterials` for colour channels |
| `featureEdges(document, angleRad, options)` | the document with `lines` primitives |
| `encodeGlb(document, options, operation)` | glTF 2.0 binary: skins, animations (`KHR_animation_pointer` for material colours), `KHR_materials_variants`, `KHR_materials_unlit` |
| `encodeFbx` / `encodeFbxDetailed` | FBX 7.4 binary (zlib arrays from the owned deflate): `LimbNode` skeleton, skin, bind pose, animation stacks, joint limits; the notices list what FBX cannot hold (lines, loop, events) |

The viewer (`./🖥️viewer`, package export `…/model/viewer`) is an owned WebGL2 renderer with a software reference rasteriser, orbit and presets, picking by solid, follow camera and root motion, and the custom element `semio-model-viewer` (`defineModelViewerElement`; the client sets `strings`).

## Tests and oracles

Language-agnostic vectors are in `🧫️fixtures/` (one folder per solid kind, skinning, animation, polygon triangulation, GLB and FBX digests); the cases are in `🧪️tests/` and next to each encoder and the viewer. Oracles: Khronos `gltf-validator`, `@gltf-transform`, three `GLTFLoader`, `FBXLoader` and geometry classes, `assimpjs`, Ajv, fflate, fast-check. Direct run (bun): `bun node_modules/vitest/vitest.mjs run --configLoader native --config <vitest config>` per folder (`🗿️model`, `📤️export/🧊️glb`, `📤️export/🎬️fbx`, `🖥️viewer`); the repo command is `repo run @semio-tech/framework-3d-js:test`. The WebGL2 backend is checked in a real browser, not in headless runs: its one test is skipped with that reason.
