/** 🎚️ The measures-rail controls that carry their own in-flight state, plus the read-only process view.
 *
 * 🪟️ Split out of `🛠️ShellHelpers/🟦️.tsx` so a law can mount them without importing that module's
 * whole shell surface (which cycles through `🏛️ShellHost`/`🐚️Shell`).
 */
import { useEffect, useState } from "react";
import type { ActionDescriptor, MeasureProgressStep, MeasureProgressStepKind, WindowMeasure } from "@semio-tech/framework";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Stepper, TreeCheckbox, uiDataLabel, useLabel } from "@semio-tech/ui-react";

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


/** @emoji 🚦️ Kind → semantic palette token. Never a literal color: the four tokens are the same
 * `info`/`success`/`warning`/`danger` the wgpu target paints `MeasureProgressStepKind` with. */
const MEASURE_PROGRESS_STEP_CLASS: Readonly<Record<MeasureProgressStepKind, string>> = {
  info: "bg-info",
  success: "bg-success",
  warning: "bg-warning",
  danger: "bg-danger",
};

/** @emoji 🪜️ How many trailing step lines the rail shows — mirrors the wgpu `MEASURE_PROGRESS_STEPS_SHOWN`. */
export const MEASURE_PROGRESS_STEPS_SHOWN = 4;

/** @emoji ♾️ Share of the track an indeterminate (`total` absent) bar fills — mirrors the wgpu constant. */
const MEASURE_PROGRESS_INDETERMINATE_SHARE = 0.35;

/**
 * @emoji 🔢️ A measures-rail number entry — unbounded unless the measure declares `min`/`max`.
 *
 * Typing and stepper clicks move a DRAFT only; the action dispatches once the entry commits (blur,
 * Enter, or the release of a `+`/`−` press — `Stepper` funnels all three through `onPointerUp`), so a
 * count of 250 is one action, not the 1/2/25/250 prefix storm a per-keystroke dispatch would send.
 * The draft retires exactly as {@link WindowMeasureSelect}'s does, the moment the program publishes.
 */
export function WindowMeasureNumber({ measure, onAction }: { readonly measure: Extract<WindowMeasure, { kind: "number" }>; readonly onAction: (action: ActionDescriptor) => unknown }) {
  const [value, setValue] = useWindowMeasureDraft(measure.value);
  const disabled = measure.disabled === true;
  const floor = measure.min ?? 0;
  const span = measure.max === undefined ? 0 : measure.max - floor;
  const readyPercent = measure.ready === undefined || span <= 0 ? null : Math.min(100, Math.max(0, ((measure.ready - floor) / span) * 100));
  return (
    <div className="flex w-full min-w-0 flex-col gap-tiny" data-slot="window-measure-number" {...{ [PUBLISHED_VALUE_ATTRIBUTE]: String(measure.value) }}>
      <Stepper
        id={measure.id}
        value={value}
        min={measure.min}
        max={measure.max}
        step={measure.step ?? 1}
        onChange={(next) => {
          if (!disabled) setValue(next);
        }}
        onPointerUp={() => {
          if (disabled || value === measure.value) return;
          onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value } });
        }}
      />
      {readyPercent === null ? null : (
        <div data-slot="window-measure-number-ready" className="h-hairline w-full overflow-hidden rounded-full bg-muted">
          <div className="h-full bg-accent" style={{ width: `${readyPercent}%` }} />
        </div>
      )}
    </div>
  );
}

/**
 * @emoji ⏳️ A measures-rail progress view — read-only except for its cancel button.
 *
 * Determinate while `total` is known (`role="progressbar"` with `aria-valuenow`/`aria-valuemax`),
 * indeterminate otherwise (`aria-busy`, no value to announce, a fixed sweep instead of a fraction).
 * The stage caption and the last {@link MEASURE_PROGRESS_STEPS_SHOWN} step lines come from the
 * program already localized; only the cancel button's own copy is framework-owned, and it comes from
 * the `ui.common.cancel` bundle (EN + DE, no default language).
 */
export function WindowMeasureProgress({ measure, onAction }: { readonly measure: Extract<WindowMeasure, { kind: "progress" }>; readonly onAction: (action: ActionDescriptor) => unknown }) {
  const cancelLabel = useLabel("ui.common.cancel");
  const determinate = measure.total !== undefined && measure.total > 0;
  const percent = determinate ? Math.min(100, Math.max(0, (measure.completed / (measure.total as number)) * 100)) : MEASURE_PROGRESS_INDETERMINATE_SHARE * 100;
  const steps = measure.steps.slice(-MEASURE_PROGRESS_STEPS_SHOWN);
  const accessibleName = uiDataLabel(measure.label ?? measure.stage ?? measure.id);
  return (
    <div className="flex w-full min-w-0 flex-col gap-tiny" data-slot="window-measure-progress" data-measure-id={measure.id}>
      {measure.stage === undefined ? null : (
        <span data-slot="window-measure-progress-stage" className="text-muted-foreground truncate text-xs">
          {measure.stage}
        </span>
      )}
      <div
        data-slot="window-measure-progress-bar"
        role={determinate ? "progressbar" : undefined}
        aria-label={accessibleName}
        aria-valuenow={determinate ? measure.completed : undefined}
        aria-valuemin={determinate ? 0 : undefined}
        aria-valuemax={determinate ? measure.total : undefined}
        aria-busy={determinate ? undefined : true}
        data-completed={String(measure.completed)}
        data-total={measure.total === undefined ? undefined : String(measure.total)}
        className="h-tiny w-full overflow-hidden rounded-full bg-muted"
      >
        <div className={measure.loading === true && !determinate ? "h-full animate-pulse bg-accent" : "h-full bg-accent"} style={{ width: `${percent}%` }} />
      </div>
      {steps.length === 0 ? null : (
        <ul data-slot="window-measure-progress-steps" className="flex flex-col gap-tiny">
          {steps.map((step: MeasureProgressStep, index: number) => (
            <li key={`${measure.id}.step.${index}`} data-step-kind={step.kind} className="flex min-w-0 items-center gap-tiny">
              <span aria-hidden="true" className={`size-tiny shrink-0 rounded-full ${MEASURE_PROGRESS_STEP_CLASS[step.kind]}`} />
              <span className="text-muted-foreground truncate text-xs">{step.text}</span>
            </li>
          ))}
        </ul>
      )}
      {measure.cancel === undefined ? null : (
        <button
          type="button"
          id={`${measure.id}.cancel`}
          data-slot="window-measure-progress-cancel"
          className="border-border text-element hover:bg-muted rounded-sm border px-half py-0 text-xs"
          onClick={() => {
            if (measure.cancel !== undefined) onAction(measure.cancel);
          }}
        >
          {cancelLabel}
        </button>
      )}
    </div>
  );
}
