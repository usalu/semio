# Design Notes

## Preserved Source Rationale

<a id="source-notes-170"></a>

### 🦀️.rs:170

🎯️ The world-3d `fit` lane: ask the render host to frame the scene's own bounds ONCE per
`revision`, preserving the current view direction.

`revision` is a producer-owned document identity, not a frame counter: `WorldAutoFit`
(`📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost`) keys its one-shot on
`${revision}:${meshes}` and never refits again while that key holds, so a producer that
derives the revision from what the DOCUMENT is (its catalogs, its domain) refits exactly on a
document swap, while a producer that derived it from the geometry would yank the camera on
every edit. Without a fit lane the camera after a document swap is whatever the previous
document left behind — a fixture centred elsewhere is simply off-screen (ticket 26/09/02 W-P5
§7, W-S2).
📦️ `bounds` is the producer's OWN delivered extent, published rather than rediscovered: a
render host that measures its scene graph instead can only frame what it has already
uploaded, and the wgpu surface has no scene graph to measure at all — so without this lane
the two renderers frame differently by construction
(ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`).
