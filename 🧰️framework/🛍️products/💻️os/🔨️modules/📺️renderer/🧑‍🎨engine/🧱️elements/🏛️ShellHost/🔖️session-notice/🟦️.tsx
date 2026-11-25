import * as React from "../../⚛️react-port/🟦️.ts";
import { directorySessionAuthorityTextV1 } from "@semio-tech/framework/directory/session-refresh";

import { sharedBorderlessBoxProps } from "@semio-tech/ui-react";
import { NoticeCell, NoticeStack } from "../🧱️notice-bands/🟦️.tsx";
export type SessionAuthorityNoticeStateV1 = "pending" | "unavailable";

/** 🪪️ Presents the exact private-session handshake state without inventing progress counts. */
export function SessionAuthorityNotice(props: Readonly<{ state: SessionAuthorityNoticeStateV1; locale: string; onCancel(): void }>): React.ReactElement {
  const text = directorySessionAuthorityTextV1(props.locale);
  const pending = props.state === "pending";
  return (
    <NoticeStack
      role={pending ? "status" : "alert"}
      aria-live={pending ? "polite" : "assertive"}
      aria-busy={pending}
      className="top-workbench right-double max-w-[28rem]"
      data-semio-session-authority={props.state}
    >
      <NoticeCell className="flex-row items-baseline"><span>{pending ? text.pending : text.unavailable}</span>
      {pending ? <button {...sharedBorderlessBoxProps} type="button" className="ml-single underline" onClick={props.onCancel}>{text.cancel}</button> : null}</NoticeCell>
    </NoticeStack>
  );
}
