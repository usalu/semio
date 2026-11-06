# Design Notes

## Preserved Source Rationale

<a id="source-notes-103"></a>

### 🦀️.rs:103

⌨️ The keyboard law of every numeric control (slider, dial, stepper, number field, vector axis). The step is `step`,
else (a non-positive or non-finite one) `10^-precision` display units converted back by the display factor, else 1.
Arrows walk one step of the ladder from `min` (from 0 without one; `large` walks [`SLIDER_PAGE_STEPS`]); from an
off-ladder value the first rung beyond it is one step, and they never stop on a detent off their path, so a detent
never traps them. Page keys jump to the next/previous detent, else walk [`SLIDER_PAGE_STEPS`] steps. Home and End
go to the bounds (and keep the value without one). A walked value is cleaned to the decimals of the ladder origin and
step (`0.2 + 0.1` lands on `0.3`), then — with a `precision` — rounded half away from zero at that precision in
display units and divided back by the factor, then clamped to the bounds; a value landing within ladder tolerance of
a detent lands on the detent exactly (a rung of a `π/180` ladder from `-π` becomes `π/2` itself).

<a id="source-notes-183"></a>

### 🦀️.rs:183

🖊️ The keyboard law of every text field that holds a draft (`commit: "blur"`) — the text counterpart of
[`ui_number_key_value`] (ticket 26/09/30 design §22.7), pinned by `🧫️text-controls`. Escape reverts the draft. A
single-line field commits on Enter. A multi-line field ([`InputKind::LongText`]) inserts a line on Enter, with or
without Shift, and commits on Ctrl/⌘+Enter. A single-line Shift+Enter and every Alt chord are not the field's
(`None`): Alt+Enter is a shell chord and never reaches a draft. Leaving the field commits in both — that is the commit
convention, not a key.
