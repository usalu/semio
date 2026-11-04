import * as React from "../../⚛️react-port/🟦️.ts";
import { cn } from "@semio-tech/ui-react";
import { TRANSIENT_NOTICE_TONE_CLASS } from "../🎚️shell-model/🟦️.tsx";
import type { Severity } from "@semio-tech/framework";

export type TransientNoticeViewV1 = Readonly<{ message: string; kind: Severity; code?: string | undefined }>;

/** 🍞️ One transient notice: errors and warnings interrupt as assertive alerts, everything else is polite status, and the close button is named with its notice. */
export function TransientNotice(props: Readonly<{ notice: TransientNoticeViewV1; closeLabel: string; onClose(): void }>): React.ReactElement {
  const urgent = props.notice.kind === "error" || props.notice.kind === "warning";
  return (
    <div
      role={urgent ? "alert" : "status"}
      aria-live={urgent ? "assertive" : "polite"}
      data-semio-transient-notice=""
      data-notice-code={props.notice.code}
      className={cn("pointer-events-auto absolute top-workbench left-1/2 z-50 -translate-x-1/2 rounded-sm border px-double py-single text-sm shadow-sm", TRANSIENT_NOTICE_TONE_CLASS[props.notice.kind])}
    >
      {props.notice.message}
      <button type="button" className="ml-single underline" aria-label={`${props.closeLabel}: ${props.notice.message}`} onClick={props.onClose}>
        {props.closeLabel}
      </button>
    </div>
  );
}
