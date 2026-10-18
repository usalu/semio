import { parseGeneratorPreviewArgumentsV1 } from "../../🏭️generator/👁️preview/🔤️arguments/🟦️.ts";
import type { GeneratorContract } from "../📦️package-projection-contracts/🟦️.ts";

export interface TaxonomyCliWritePreparationFacts {
  readonly directoryName: string;
  readonly leafNames: readonly string[];
}

export interface TaxonomyCliAttemptPreparationChildFacts {
  readonly name: string;
  readonly nodeKind: "directory" | "file";
}

export interface TaxonomyCliAttemptPreparationFacts {
  readonly parentKindId: string;
  readonly directoryName: string;
  readonly children: readonly TaxonomyCliAttemptPreparationChildFacts[];
}

/** ⚙️ Derives the only runnable generator command shape admitted by v7. */
export function generatorBuildCommand(contract: GeneratorContract): readonly ["bun", "repo", "run", string] {
  if (contract.ownership !== "owned" || !contract.target) throw new Error("Only owned generator contracts are runnable.");
  return ["bun", "repo", "run", contract.target];
}

/** 👁️ Derives an owned generator's mandatory read-only preview command. */
export function generatorBuildPreviewCommand(contract: GeneratorContract): readonly ["bun", "repo", "run", string] {
  if (contract.ownership !== "owned" || !contract.previewTarget) throw new Error("Only owned generator contracts have preview targets.");
  return ["bun", "repo", "run", contract.previewTarget];
}

/** 👓️ Admits an exact read-only script invocation without sharing another generator's preview. */
export function generatorPreviewScriptArguments(contract: Pick<GeneratorContract, "ownership" | "target" | "previewTarget" | "previewArguments">): readonly string[] {
  if (contract.ownership !== "owned" || !contract.target || !contract.previewTarget) throw new Error("Only owned generator contracts have preview invocations");
  const project = contract.target.slice(0, contract.target.lastIndexOf(":"));
  if (contract.previewArguments === undefined) {
    if (contract.previewTarget !== `${project}:preview-generated`) throw new Error("previewTarget must be the exact owner preview-generated target or an explicit same-project preview-* invocation");
    return ["preview-generated"];
  }
  const args = parseGeneratorPreviewArgumentsV1(contract.previewArguments), separator = contract.previewTarget.lastIndexOf(":");
  if (contract.previewTarget.slice(0, separator) !== project || !/^preview-[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(contract.previewTarget.slice(separator + 1)) || !/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(args[0]!) || args.at(-1) !== "preview") throw new Error("Invalid explicit same-project preview invocation");
  return args;
}

/** ⏱️ Bounds a declared compiler preview without changing established generator defaults. */
export function generatorPreviewResourceLimits(contract: Pick<GeneratorContract, "ownership" | "previewTarget" | "previewLimits">): { readonly maxOutputBytes: number; readonly timeoutMs: number } {
  const limits = contract.previewLimits;
  if (limits === undefined) return { maxOutputBytes: 134217728, timeoutMs: 60000 };
  if (contract.ownership !== "owned" || !contract.previewTarget || !limits || typeof limits !== "object" || Array.isArray(limits) || Object.keys(limits).sort().join("|") !== "maxOutputBytes|timeoutMs" || !Number.isSafeInteger(limits.maxOutputBytes) || limits.maxOutputBytes < 1024 || limits.maxOutputBytes > 536870912 || !Number.isSafeInteger(limits.timeoutMs) || limits.timeoutMs < 1000 || limits.timeoutMs > 600000) throw new Error("Invalid bounded generator preview limits");
  return limits;
}

/** ✅️ Derives an owned generator's exact optional freshness command. */
export function generatorBuildCheckCommand(contract: GeneratorContract): readonly ["bun", "repo", "run", string] | null {
  if (contract.ownership !== "owned" || !contract.checkTarget) return null;
  return ["bun", "repo", "run", contract.checkTarget];
}
