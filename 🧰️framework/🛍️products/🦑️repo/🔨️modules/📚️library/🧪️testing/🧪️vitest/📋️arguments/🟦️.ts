import { coverageEnabled } from "../../📊️coverage/🟦️.ts";
import { vitestArgumentsV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧪️vitest/🟦️.ts";
import { repositoryVitestPolicyV1 } from "../../../🏃️process/📋️context/🔣️policy/🟦️.ts";

export function vitestRunArguments(bundleRoot: string, segments: string[], config: string, collectingCoverage = coverageEnabled()): string[] {
  return vitestArgumentsV1(repositoryVitestPolicyV1(bundleRoot),segments,config,collectingCoverage);
}
