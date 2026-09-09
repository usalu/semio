import React from "react";
import { directorySessionAuthorityTextV1 } from "../🟦️.ts";

export type SessionAuthorityNoticeStateV1 = "pending" | "unavailable";

/** 🪪️ Presents the exact private-session handshake state without inventing progress counts. */
export function SessionAuthorityNotice(props: Readonly<{ state: SessionAuthorityNoticeStateV1; locale: string; onCancel(): void }>): React.ReactElement {
  const text = directorySessionAuthorityTextV1(props.locale);
  const pending = props.state === "pending";
  return (
    <section
      role={pending ? "status" : "alert"}
      aria-live={pending ? "polite" : "assertive"}
      aria-busy={pending}
      className="pointer-events-auto absolute top-workbench right-double z-50 max-w-[28rem] rounded-sm border bg-base px-double py-single text-sm shadow-sm"
      data-semio-session-authority={props.state}
    >
      <span>{pending ? text.pending : text.unavailable}</span>
      {pending ? <button type="button" className="ml-single underline" onClick={props.onCancel}>{text.cancel}</button> : null}
    </section>
  );
}
