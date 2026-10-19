/** 🎞️ Tool run tick: the value one tick of a run carries (progress, steps, trace pages, appended entities and an opaque payload). */

import type { ToolRunIdentity } from "../🟦️.ts";
import type { ToolRunProgress, ToolRunStep } from "../📊️progress/🟦️.ts";
import type { ToolRunTracePage } from "../🗄️trace-store/🟦️.ts";
import type { PayloadUpdate } from "../📨️payload/🌊️stream/🟦️.ts";

//#region 🔖️Tick
export type ToolRunTick = {
  readonly identity: ToolRunIdentity;
  readonly sequence: bigint;
  readonly progress?: ToolRunProgress;
  readonly steps: readonly ToolRunStep[];
  readonly trace: readonly ToolRunTracePage[];
  readonly appendOps: readonly Uint8Array[];
  readonly appendEntities: readonly bigint[];
  readonly retractTo?: number;
  /** 📨️ Opaque plugin bytes the run's windows read back — the newest tick carrying one wins. */
  readonly payload?: Uint8Array;
  readonly payloadUpdate?:PayloadUpdate;
};
//#endregion 🔖️Tick
