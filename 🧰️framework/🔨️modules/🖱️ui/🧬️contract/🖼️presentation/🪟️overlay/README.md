# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🪟️ The ONE anchored-overlay positioner every renderer resolves a floating surface with —
the renderer-neutral port of React's `resolvePopoverPlacement`
(`🧱️elements/🗨️Popover/🟦️.tsx:216-259`).

Lives in the contract rather than in a target because THREE renderers need the same answer: the
DOM one computes it in `Popover`, the retained wgpu one in `🎯️targets/🧊️wgpu/⚡️events`, and the
backend-neutral `🖌️render/🖱️dispatch` one in its own overlay region. Before this module the last
two each carried a private copy and the two copies had already drifted apart (ticket
`26/09/17/WGPU-RENDERER-REACT-PARITY`, packet W1n gap 6: one had been re-ported to React's
model, the other still carried a three-variant `BelowAnchorWithFlip`/`AtPointer`/`Centered`
guess).

Dependency-free by construction: plain `f32` rectangles, no `Rect` type from any target — a
target converts at its own boundary.

<a id="source-notes-146"></a>

### 🦀️.rs:146

🧷️ React's `resolvePopoverPlacement`, field for field: align the cross axis, place on the
requested side, flip to the opposite side when the main axis overflows *and* the flip does not,
then clamp both axes inside `collision_padding`.

`flow` is React's own `rtl` argument (`🗨️Popover/🟦️.tsx:326`, from `useFlow().inline`). It
touches exactly ONE line, the inline alignment: React's `align === (rtl ? "end" : "start")`
picks the anchor's LEFT edge, everything else the anchor's RIGHT edge minus the content width.
Side placement, the vertical alignment, the flip and the clamp are direction-invariant there and
here.
