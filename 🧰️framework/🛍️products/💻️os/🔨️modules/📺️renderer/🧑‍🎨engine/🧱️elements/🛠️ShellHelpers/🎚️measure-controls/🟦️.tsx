/** 🎚️ The two measures-rail controls that carry their own in-flight state.
 *
 * 🪟️ Split out of `🛠️ShellHelpers/🟦️.tsx` so a law can mount them without importing that module's
 * whole shell surface (which cycles through `🏛️ShellHost`/`🐚️Shell`).
 */
import { useEffect, useState } from "react";
import type { ActionDescriptor, WindowMeasure } from "@semio-tech/framework";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue, TreeCheckbox, uiDataLabel } from "@semio-tech/ui-react";

/**
 * 🕰️ The value a measures-rail control shows while its own dispatch is still in flight.
 *
 * Every rail control is CONTROLLED by the value the program published — `measure.pressed`,
 * `measure.value` — and that value only moves once the dispatch has crossed the guest, published on
 * the per-window `WindowConfig` lane, and come back through `refreshUi`'s `measures` section. Measured
 * on the live `:6013` shell (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12): 0.7 s on an idle app,
 * several seconds on a busy one. For that whole window a bare controlled control renders the value the
 * user just changed away from, so the click reads as ignored and a second click is dispatched against a
 * value the program has already left behind.
 *
 * `WindowMeasureSlider` has always had the answer — {@link resolveSliderDraftClear} keeps the user's
 * own value until the controlled prop catches up. This is the same discipline for the other two
 * control kinds: hold the dispatched value, render it, and drop the draft the moment the published
 * value equals it (or the program answers with a different value, which then wins — the program is the
 * authority, the draft is only ever the gap-filler).
 */
function useWindowMeasureDraft<T>(published: T): readonly [T, (next: T) => void] {
  const [draft, setDraft] = useState<{ readonly value: T; readonly published: T } | null>(null);
  // 🏛️ Retire the draft the moment the program publishes ANYTHING other than the value the draft was
  // made against — whether that is the draft's own value (the round trip landed) or a third one (the
  // program refused, or someone else moved this option). Retiring on inequality rather than on a match
  // is what stops a draft from resurrecting when the published value later happens to return to what it
  // was when the draft was taken.
  useEffect(() => setDraft((current) => (current !== null && current.published !== published ? null : current)), [published]);
  const live = draft !== null && draft.published === published ? draft : null;
  return [live === null ? published : live.value, (next: T) => setDraft(next === published ? null : { value: next, published })];
}

/** @emoji 🕰️ The value the PROGRAM last published for a rail control, exposed on the DOM beside the possibly
 * optimistic value the control RENDERS. Nothing outside React could tell the two apart: {@link
 * useWindowMeasureDraft} moves the rendered state first and holds it for the whole round trip (0.7 s idle,
 * seconds on a busy app), a combobox trigger carries no value of its own at all, and so
 * `🔍️browser-probe.ts`'s `projection-control-flips` read the draft as if it were the program's answer
 * (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B41). This is the authority's own reading, so a reader —
 * assistive technology, a tutorial, an end-to-end probe — can see whether a gesture has actually landed. */
const PUBLISHED_VALUE_ATTRIBUTE = "data-published-value";

/** @emoji 🔽️ A measures-rail select. A measure with no `label` renders no visible tree-row label either, so the combobox carries its own accessible name rather than reaching assistive technology as an unnamed control. */
export function WindowMeasureSelect({ measure, onAction }: { readonly measure: Extract<WindowMeasure, { kind: "select" }>; readonly onAction: (action: ActionDescriptor) => unknown }) {
  const [value, setValue] = useWindowMeasureDraft(measure.value);
  return (
    <Select
      id={measure.id}
      value={value}
      onValueChange={(next) => {
        setValue(next);
        onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value: next } });
      }}
    >
      <SelectTrigger id={measure.id} aria-label={uiDataLabel(measure.label ?? measure.id)} {...{ [PUBLISHED_VALUE_ATTRIBUTE]: measure.value }} className="h-small w-full min-w-0" size="sm">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        {measure.items.map((item) => (
          <SelectItem key={item.id} value={item.value}>
            {item.label}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}

/** @emoji ☑️ A measures-rail toggle, showing the state the user asked for until the program publishes it. */
export function WindowMeasureToggle({ measure, onAction }: { readonly measure: Extract<WindowMeasure, { kind: "toggle" }>; readonly onAction: (action: ActionDescriptor) => unknown }) {
  const label = uiDataLabel(measure.label ?? measure.text ?? measure.id);
  const [pressed, setPressed] = useWindowMeasureDraft(measure.pressed);
  return (
    <TreeCheckbox
      id={measure.id}
      publishedValue={String(measure.pressed)}
      checked={pressed}
      title={label}
      ariaLabel={label}
      onCheckedChange={(next) => {
        setPressed(next);
        onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), pressed: next } });
      }}
    />
  );
}

