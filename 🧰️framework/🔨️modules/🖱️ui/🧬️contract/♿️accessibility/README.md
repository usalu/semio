# Design Notes

## Preserved Source Rationale

<a id="source-notes-11"></a>

### 🦀️.rs:11

#region 🔖️AccessibilityProjection
🎭️ The ARIA role a component IMPLIES. [`AccessibilitySpec`] carries no `role` on purpose — a
`Component::Button` is a button on every renderer — so this is the one place that implication is
written down, shared by every target instead of re-derived per renderer. React reaches the same
roles implicitly, through the HTML element each component view renders (`<button>`, `<p>`,
`<input>`, …); a renderer with no DOM (the wgpu target) has to say them out loud, which is what
makes its accessibility projection readable at all.

`activatable` is the record's own `Trigger::Activate` binding: a container you can press IS a
button, exactly as React's Interpreter spells it (`role={activateBinding ? "button" : role}`).

<a id="source-notes-80"></a>

### 🦀️.rs:80

⌨️ Whether a component takes keyboard focus of its own — the same set the wgpu `EventRouter`'s
`is_focusable` walks for Tab traversal, said once here so the projection and the traversal can
never disagree about who is reachable.

[`crate::Component::Surface`] is in the set because it is an app's ONLY door to a scene it paints
itself: the node-graph and World3d canvases carry their whole interaction surface inside a
texture no assistive technology can walk, so a canvas nobody can Tab into is a canvas nobody can
drive without a mouse. Its `application` role is exactly the ARIA promise that the widget handles
its own arrow/Enter keys.

<a id="source-notes-117"></a>

### 🦀️.rs:117

📮️ ONE node of the accessibility projection a renderer publishes for an assistive technology to
read. Flat and ordered: `depth` carries the tree shape, so a consumer rebuilds nesting without
the projection having to nest (the same flat-table discipline [`crate::UiSnapshot`] uses).

A hidden node is KEPT, carrying `hidden: true`, rather than dropped — that is what React does
(`aria-hidden` on a rendered element), and a consumer that dropped it would silently disagree
with the DOM renderer about what exists.
