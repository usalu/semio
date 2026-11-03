/** 📏️ The display range control of the analytic views: automatic, fixed minimum and maximum, or a span in kelvin around the air temperature.
 *
 * A native radio group picks the mode; fixed shows two number fields, span one. Every field has a visible label and an invalid entry keeps the last valid range. The span is the one the cast exporter writes as `range: { mode: "span", spanK }`.
 *
 * @see ../../🪟️chrome/🟦️.tsx — `Segmented`, `NumberField`
 * @see ../../../../../🔨️modules/🔗️contracts/🟦️.ts — `DisplayRange`
 */
import type { ReactElement } from "react";
import type { DisplayRange } from "@teaching/staffage";
import { useArchive } from "../../🧭️context/🟦️.tsx";
import { NumberField, Segmented } from "../../🪟️chrome/🟦️.tsx";

/** 🌡️ The span of the course clips, in kelvin around the air temperature. */
export const DEFAULT_SPAN_K = 25;

/** 🔢️ The range a mode starts with: the legend of the scene for fixed, {@link DEFAULT_SPAN_K} for span. */
export function rangeOfMode(mode: DisplayRange["mode"], legend: { readonly min: number; readonly max: number } | undefined, previous: DisplayRange): DisplayRange {
  if (mode === "fixed") return previous.mode === "fixed" ? previous : { mode, min: legend?.min ?? 0, max: legend?.max ?? 1 };
  if (mode === "span") return previous.mode === "span" ? previous : { mode, spanK: DEFAULT_SPAN_K };
  return { mode: mode === "shared" ? "shared" : "auto" };
}

/** 🎚️ The display range control. */
export function RangeControl(props: { readonly range: DisplayRange; readonly legend?: { readonly min: number; readonly max: number }; readonly onChange: (range: DisplayRange) => void }): ReactElement {
  const { range, legend, onChange } = props;
  const { t } = useArchive();
  return (
    <>
      <Segmented<DisplayRange["mode"]>
        label={t("viewer.range")}
        value={range.mode}
        onChange={(mode) => onChange(rangeOfMode(mode, legend, range))}
        options={[
          { value: "auto", label: t("viewer.range.auto") },
          { value: "fixed", label: t("viewer.range.fixed") },
          { value: "span", label: t("viewer.range.span"), title: t("viewer.range.spanHelp") },
        ]}
      />
      {range.mode === "fixed" ? (
        <div className="flex gap-double">
          <NumberField label={t("viewer.range.min")} value={range.min} onChange={(min) => min !== null && min < range.max && onChange({ mode: "fixed", min, max: range.max })} />
          <NumberField label={t("viewer.range.max")} value={range.max} onChange={(max) => max !== null && max > range.min && onChange({ mode: "fixed", min: range.min, max })} />
        </div>
      ) : null}
      {range.mode === "span" ? <NumberField label={t("viewer.range.spanK")} unit="K" min={0.1} max={200} value={range.spanK} onChange={(spanK) => spanK !== null && spanK > 0 && onChange({ mode: "span", spanK })} /> : null}
    </>
  );
}
