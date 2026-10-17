/** 📋️ App channel report decoders: faults, dispatch reports, merge reports and conflicts decoded from their pack encoded wire forms. */

import type { Conflict, DispatchReport, Fault, MergeReport, MutationMessage } from "@semio-tech/framework";

//#region 🔖️AppChannelClient
/**
 * 📡️ TS twin of `protocol_channel::CHANNEL_VERSION` (`🔨️modules/📡️protocol/🧵️channel/📦️packages/🦀️rust/📦️lib.rs`)
 * — bump both sides together on a wire-incompatible frame change.
 */
/** 📥️ Decodes a pack-encoded {@link Fault} from an app-channel error frame. */
export function decodeFaultFromWire(faultBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): Fault | null {
  try {
    const raw = decodePackValue(new Uint8Array(faultBytes));
    if (!raw || typeof raw !== "object" || !("message" in raw)) return null;
    return raw as Fault;
  } catch {
    return null;
  }
}

/** 🧯️ One display rule for every app-channel fault: a pack-encoded {@link Fault} reads as
 * `code: message`, and a guest that answered with plain UTF-8 fault text reads as that text. */
export function faultDisplayMessage(faultBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): string {
  const fault = decodeFaultFromWire(faultBytes, decodePackValue);
  if (!fault) {
    const text = new TextDecoder().decode(new Uint8Array(faultBytes)).trim();
    return text.length > 0 ? text : "unknown fault";
  }
  const code = typeof fault.code === "string" ? fault.code : String(fault.code);
  return `${code}: ${fault.message}`;
}

/** 📥️ Decodes a pack-encoded {@link DispatchReport} from an app-channel wire blob —
 * `AppFrame::Invocation.messages` (a successful dispatch's report) or `AppFrame::Error.report` (the
 * rejected dispatch's report, `Fault.code == "app.command.rejected"`). `null` for an empty blob (the
 * trailing field's zero value before every dispatch path was updated to populate it). */
export function decodeDispatchReportFromWire(reportBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): DispatchReport | null {
  if (reportBytes.length === 0) return null;
  try {
    return decodePackValue(new Uint8Array(reportBytes)) as DispatchReport;
  } catch {
    return null;
  }
}

/** 📨️ Decodes an `AppFrame::Error.report` blob into its typed `MutationMessage`s — so a
 * caller reacting to a rejected dispatch (contract-freeze §C8/§C9) gets structured messages instead
 * of parsing {@link faultDisplayMessage}'s prose string. Empty array for an empty/undecodable blob. */
export function faultMessages(reportBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): readonly MutationMessage[] {
  return decodeDispatchReportFromWire(reportBytes, decodePackValue)?.messages ?? [];
}

/** 📥️ Decodes a pack-encoded {@link MergeReport} from an `AppFrame::MergeReport.report`
 * blob — pushed unsolicited after every `ingest_remote`/`resolve_conflict`,
 * alongside `DocumentChanged`. */
export function decodeMergeReportFromWire(reportBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): MergeReport | null {
  if (reportBytes.length === 0) return null;
  try {
    return decodePackValue(new Uint8Array(reportBytes)) as MergeReport;
  } catch {
    return null;
  }
}

/** 📥️ Decodes a pack-encoded {@link Conflict}[] projection from an `AppFrame::
 * Conflicts.conflicts` blob — pushed unsolicited after every ingest (alongside `DocumentChanged`)
 * and in reply to `AppCommand::ReadConflicts`. */
export function decodeConflictsFromWire(conflictsBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): readonly Conflict[] {
  if (conflictsBytes.length === 0) return [];
  try {
    return decodePackValue(new Uint8Array(conflictsBytes)) as readonly Conflict[];
  } catch {
    return [];
  }
}
//#endregion 🔖️AppChannelClient
