/** 🧯️ App channel reply faults: the guest's `AppFrame::Error` as a thrown, labelled engineering error, and the label every client error message carries. */

import { decodePackValue } from "../../🎁️packs/🟦️.ts";
import type { AppFrameValue } from "../../🪢️wire-contracts/🟦️.ts";
import { faultDisplayMessage } from "../📋️report-decoders/🟦️.ts";

//#region 🔖️AppChannelClient
/** 🏷️ The prefix of every error one client method throws: `AppChannelClient.<method>(<appId>)`. */
export function channelLabel(method: string, appId: string): string {
  return `AppChannelClient.${method}(${appId})`;
}

/** 🚨️ Throws `<label>: <the guest's own fault text>` when the reply carries an `AppFrame::Error`; returns when it does not. */
export function throwOnFault(frames: readonly AppFrameValue[], label: string): void {
  const error = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
  if (error) throw new Error(`${label}: ${faultDisplayMessage(error.Error.fault, decodePackValue)}`);
}
//#endregion 🔖️AppChannelClient
