import * as React from "../../⚛️react-port/🟦️.ts";

/** 🌟️ The Open-with "Set as default" control: a fixed accessible name whose state lives only in `aria-pressed`; the star glyph is decorative. */
export function DefaultAppToggle(props: Readonly<{ isDefault: boolean; label: string; onToggle(): void }>): React.ReactElement {
  return (
    <button
      type="button"
      aria-pressed={props.isDefault}
      onClick={(event) => {
        event.stopPropagation();
        props.onToggle();
      }}
    >
      <span aria-hidden="true">{props.isDefault ? "★" : "☆"}</span> {props.label}
    </button>
  );
}
