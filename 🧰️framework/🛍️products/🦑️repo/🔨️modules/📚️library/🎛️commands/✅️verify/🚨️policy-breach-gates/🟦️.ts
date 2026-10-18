/** 🚨️ Single-rule stdio-artifact policy gates behind `verify policy-breach <rule>`.
 * @see 📜️script.ts
 */
import { type BreachRecord } from "../../../🚨️policy/🔖️breach/🟦️.ts";
import { policyArtifactAnalyzerBreaches, policyArtifactBuilderBreaches, policyArtifactBuilderMigratedBreaches, policyArtifactComposerBreaches, policyArtifactDecomposerBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🚪️artifact-io/🗂️stdio-catalog/🟦️.ts";
import { policyCodecFidelityBreaches, policyIoSerializerMatrixBreaches, policyIoTerminalityBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🚪️artifact-io/🔌️io-matrix/🟦️.ts";
import { policySchemaRepresentationBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🚪️artifact-io/🧬️schema-representation/🟦️.ts";
import { policyStandardsCoverageBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🚪️artifact-io/🧾️standards/🟦️.ts";
import { policyContributionTargetBreaches, policyPluginDependencyParityBreaches } from "../../../⚖️laws/🧩️plugin-consistency/✍️handcrafted-mutation/🟦️.ts";

/**
 * ⚖️The 12 single-rule stdio-artifact policy gates, previously only reachable via an inline `bun -e`
 * eval that bypassed the build framework entirely. `verify policy-breach
 * <rule>` gives each one a real CLI surface routable through a cached `repo:script` task.
 */
export const POLICY_BREACH_GATES: Record<string, (repoRoot: string) => BreachRecord[]> = {
  "artifact-builder": policyArtifactBuilderBreaches,
  "artifact-decomposer": policyArtifactDecomposerBreaches,
  "schema-representation": policySchemaRepresentationBreaches,
  "io-serializer-matrix": policyIoSerializerMatrixBreaches,
  "io-terminality": policyIoTerminalityBreaches,
  "codec-fidelity": policyCodecFidelityBreaches,
  "standards-coverage": policyStandardsCoverageBreaches,
  "artifact-analyzer": policyArtifactAnalyzerBreaches,
  "artifact-composer": policyArtifactComposerBreaches,
  "artifact-builder-migrated": policyArtifactBuilderMigratedBreaches,
  "plugin-dependency-parity": policyPluginDependencyParityBreaches,
  "contribution-target": policyContributionTargetBreaches,
};
