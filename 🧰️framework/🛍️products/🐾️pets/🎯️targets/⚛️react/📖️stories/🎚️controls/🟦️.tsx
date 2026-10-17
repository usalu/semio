/** 🎚️ The small controls of the stories page: slider, toggle, choice and action button. */
import { type ReactElement } from "react";
import "../🎨️.css";

//#region 🔖️Controls
/** 🎛️ A labelled range input that shows its value (or what `shown` calls it). */
export function Slider({ label, value, min, max, step, shown, onChange }: { readonly label: string; readonly value: number; readonly min: number; readonly max: number; readonly step: number; readonly shown?: string; readonly onChange: (value: number) => void }): ReactElement {
  return (
    <label className="control">
      <span>{label}</span>
      <input type="range" value={value} min={min} max={max} step={step} onChange={(event) => onChange(Number(event.target.value))} />
      <output>{shown ?? value}</output>
    </label>
  );
}

/** ☑️ A labelled checkbox. */
export function Toggle({ label, checked, onChange }: { readonly label: string; readonly checked: boolean; readonly onChange: (checked: boolean) => void }): ReactElement {
  return (
    <label className="control">
      <input type="checkbox" checked={checked} onChange={(event) => onChange(event.target.checked)} />
      <span>{label}</span>
    </label>
  );
}

/** 🔽️ A labelled select over `options` (value and what it is called). */
export function Choice<Value extends string>({ label, value, options, onChange }: { readonly label: string; readonly value: Value; readonly options: readonly (readonly [Value, string])[]; readonly onChange: (value: Value) => void }): ReactElement {
  return (
    <label className="control">
      <span>{label}</span>
      <select value={value} onChange={(event) => onChange(event.target.value as Value)}>
        {options.map(([option, name]) => (
          <option key={option} value={option}>
            {name}
          </option>
        ))}
      </select>
    </label>
  );
}

/** 🔘️ A plain button. */
export function Action({ label, onPress, pressed, disabled }: { readonly label: string; readonly onPress: () => void; readonly pressed?: boolean; readonly disabled?: boolean }): ReactElement {
  return (
    <button type="button" className="action" aria-pressed={pressed} disabled={disabled} onClick={onPress}>
      {label}
    </button>
  );
}
//#endregion 🔖️Controls
