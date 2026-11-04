import * as React from "../../../⚛️react-port/🟦️.ts";

/** 📢️ A visually hidden polite live region announcing a surface's loading or empty state, so the otherwise silent wait reaches assistive technology. */
export function TreeStatusAnnouncement(props: Readonly<{ text: string; busy: boolean }>): React.ReactElement {
  return (
    <span role="status" aria-live="polite" aria-busy={props.busy} className="sr-only">
      {props.text}
    </span>
  );
}
