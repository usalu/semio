/** 🧾️ Acceptance requires executed evidence on every renderer parity axis. */
import type { ParityPlaygroundReport } from "../🏗️structure/🟦️.ts";

export type ParityVerdict = "PASS" | "FAIL" | "INCOMPLETE" | "STALE-BRIDGE";

export function parityVerdict(report: ParityPlaygroundReport): ParityVerdict {
  if (report.boot.react === "STALE-BRIDGE" || report.boot.wgpu === "STALE-BRIDGE") return "STALE-BRIDGE";
  if (report.boot.react !== "PASS" || report.boot.wgpu !== "PASS" || report.structural?.status === "FAIL" || report.pixel?.status === "FAIL" || report.behavioral?.status === "FAIL" || report.behavioral?.steps.some(step => step.status === "FAIL")) return "FAIL";
  if (report.structural?.status !== "PASS" || !(report.structural.nodeCount > 0) || report.pixel?.status !== "PASS" || !(report.pixel.comparedRegions > 0) || report.behavioral?.status !== "PASS" || !report.behavioral.steps.length || report.behavioral.steps.some(step => step.status !== "PASS")) return "INCOMPLETE";
  return "PASS";
}
