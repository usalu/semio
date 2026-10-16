/** 📋️ The per-playground parity report that the verdict, the report writer and the sweep share. */

import type { BootStatus, PixelRegionResult, StructuralResult } from "../🏗️structure/🟦️.ts";
import type { ProbeRunResult } from "../🔬️probe/🟦️.ts";

type ParityPlaygroundReport = {
  readonly variant: string;
  readonly boot: { readonly react: BootStatus; readonly wgpu: BootStatus; readonly detail?: string };
  readonly structural?: StructuralResult;
  readonly pixel?: { readonly status: "PASS" | "FAIL"; readonly comparedRegions: number; readonly regions: readonly PixelRegionResult[] };
  /** 🎬️See `🔖️ProbeCatalog` — behavioral (interaction-driven) parity, distinct from the static
   * `structural`/`pixel` end-state checks above. Optional: only populated once boot passed (a probe
   * can't drive a page that never finished booting). */
  readonly behavioral?: ProbeRunResult;
  readonly durationMs: number;
};

export type { ParityPlaygroundReport };
