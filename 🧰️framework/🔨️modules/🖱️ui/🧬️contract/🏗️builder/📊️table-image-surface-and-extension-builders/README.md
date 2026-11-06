# Design Notes

## Preserved Source Rationale

<a id="source-notes-128"></a>

### 🦀️.rs:128

#region 🖼️Image
📍️ Typestate marker: [`ImageBuilder::alt`]/[`ImageBuilder::decorative`] not yet called. An
`ImageBuilder<NoAlt>` has no `build()` at all — there is no inherent `build` on this state, and the
blanket [`Buildable`] impl only applies to `T: Into<BuiltNode>`, which `ImageBuilder<NoAlt>` is not
(only `ImageBuilder<HasAlt>` implements [`From`] into [`BuiltNode`]). Calling `.build()` on this
state is therefore a COMPILE error (`E0599: no method named build`), never a runtime panic — see
this module's own accessible-UI mandate in [`crate`]'s docs and the compile-fail example below.

<a id="source-notes-152"></a>

### 🦀️.rs:152

💿️ An image — `Component::Image`. Build with [`image`], which returns `ImageBuilder<NoAlt>`; call
[`ImageBuilder::alt`] or [`ImageBuilder::decorative`] to obtain an `ImageBuilder<HasAlt>` before
[`Buildable::try_build`] is even callable — CLAUDE.md's accessible-UI mandate
makes an image's accessible name non-optional, and a **typestate**, not a runtime panic, is the
cheapest place to hold that line: a plugin that forgets `.alt(..)`/`.decorative()` fails to compile,
it never reaches a running actor to crash.


<a id="source-notes-232"></a>

### 🦀️.rs:232

👶️ A surface's children are NOT rendered chrome — a renderer draws `props` and nothing else. They
are the surface's own out-of-doc payload carriers: `SurfaceDoc.bytes` is [`crate::UiFixedBytes`], a
hard [`crate::UI_FIXED_BYTES`] ceiling that cannot page, so a scene whose payload scales with its
document (a world of 10 000 instances, say) can only publish by leaving that payload OUTSIDE the
doc, as retained text-leaf subtrees the reconciler pages and diffs per node like any other subtree.
The layout stays [`crate::LayoutSpec::Leaf`] precisely because these children occupy no space.
