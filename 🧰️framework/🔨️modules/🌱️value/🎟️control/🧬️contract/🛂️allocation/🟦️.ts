import type { ValueError } from "../../../⚠️refusal/🟦️.ts";
export type { ValueError } from "../../../⚠️refusal/🟦️.ts";

/** 📏️ Separates cumulative admission from an optional performed allocation. */
export type NativeAllocationProgress = { readonly admittedBytes: number; readonly actualCapacityBytes: number | null; } & ({ readonly priorAdmittedBytes: number; readonly requestedBytes: number; readonly maximumBytes: number; } | { readonly priorAdmittedBytes: null; readonly requestedBytes: null; readonly maximumBytes: null; });

/** 🧳️ Preserves independently owned original allocation refusals. */
export interface NativeAllocationFailure {
  readonly primary: ValueError | null;
  readonly admission: ValueError | null;
  readonly forwardedAdmission: ValueError | null;
  readonly scopedAdmission: ValueError | null;
  readonly forwardedQuote: ValueError | null;
  readonly scopedQuote: ValueError | null;
  readonly forwardedPost: ValueError | null;
  readonly scopedPost: ValueError | null;
  readonly forwardedCancel: ValueError | null;
  readonly scopedCancel: ValueError | null;
  readonly receipt: ValueError | null;
  readonly extent: ValueError | null;
  readonly progress: NativeAllocationProgress;
}
