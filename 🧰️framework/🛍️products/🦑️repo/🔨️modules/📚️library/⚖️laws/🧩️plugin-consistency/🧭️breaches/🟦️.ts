/** 🧭️ Wave 4 plugin consistency policy: the full breach run over every rule.
 * @see 📜️script.ts
 */
import { getWorkspaceRoot, type BreachRecord } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { policyArtifactSchemaBreaches } from "../../../🧬️schema/🗿️artifact/⚖️laws/📋️aggregate/🟦️.ts";
import { policyAppSchemaBreaches } from "../../../🧬️schema/🗺️surface/⚖️laws/📋️aggregate/🟦️.ts";
import { policyInferenceFamilyBreaches } from "../../../🧬️schema/💡️inference/⚖️laws/📋️aggregate/🟦️.ts";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { policyAppCouplingBreaches, policyCargoArtifactBreaches, policyJsonFixtureBreaches, policyOpsGrammarBreaches } from "../🧱️app-coupling/🟦️.ts";
import { policyStdioArtifactsBreaches } from "../🚪️artifact-io/🟦️.ts";
import { policyBudgetNullBreaches, policyDbServerOnlyBreaches, policyDocumentAppShapeBreaches, policyMcpConfigBreaches, policyNoPackFilesBreaches, policyOsStateAuthorityBreaches, policyRawSpawnBreaches } from "../🚧️authority-bans/🟦️.ts";
import { policyCleanArtifactStandardSubsetMechanismBreaches, policyDerivedConfigBreaches, policyPackageLanguagePurityBreaches } from "../🧼️clean-mechanism/🟦️.ts";
import { policyCommandEnvelopeCompletenessBreaches, policyDiffCompletenessBreaches, policyDslCompletenessBreaches, policyGrammarFileBreaches, policyPackCompletenessBreaches, policyProtocolFileBreaches, policyProtocolMigrationBreaches, policyTsFacadeBreaches } from "../🧮️completeness/🟦️.ts";
import { policyArtifactAppLawDelegateBreaches, policyLabelsStructBreaches, policyManifestRegionBreaches, policyModLayoutBreaches, policyRegionFormatBreaches, policySelectionIdsBreaches, policyStructNamingBreaches, policyTreeItemBreaches } from "../🏗️crate-shape/🟦️.ts";
import { policyCompositionBreaches, policyDissolvedKernelsBreaches } from "../🧊️dissolved-kernels/🟦️.ts";
import { policyDiscoverCrateDirs } from "../🏛️foundation/🟦️.ts";
import { policyContributionTargetBreaches, policyDeadExampleLeafBreaches, policyHandcraftedSpecP3Breaches, policyMutationArtifactEngineBreaches, policyPluginDependencyParityBreaches } from "../✍️handcrafted-mutation/🟦️.ts";
import { policyMutationOutcomeMergePolicyBreaches } from "../🎯️mutation-outcome/🟦️.ts";
import { policyApaBreaches } from "../🔌️plugin-architecture/🟦️.ts";
import { policySchemaOverhaulPCBreaches } from "../📐️schema-overhaul-pc/🟦️.ts";
import { policySchemaOverhaulS2Breaches, policySniffRealityBreaches } from "../🧬️schema-overhaul-s2/🟦️.ts";
import { policyContributedSurfaceTargetBreaches, policyOsConfigShapeBreaches, policySubsetConformanceBreaches, policySubsetSurfaceCompletenessBreaches, policyViewerPurityBreaches } from "../🪆️subset-conformance/🟦️.ts";
import { policyBannedNameStemBreaches, policyComponentFileBreaches, policyEmojiPrefixBreaches, policyModeCompletenessBreaches, policyPluginBuilderBreaches, policyPluginRootShapeBreaches, policySemioArtifactExamplesBreaches, policySprNamingBreaches, policyTaxonomyBarrelShapeBreaches, policyTaxonomyDirsBreaches, policyTaxonomyLibShapeBreaches, policyWindowCompletenessBreaches } from "../🗺️taxonomy-shape/🟦️.ts";

//#region 🔖️Policy
//#region 🔖️PolicyExport
/**
 * ⚖️Runs every Wave 4 app-plugin rule over every discovered crate that belongs to a plugin, plus the
 * taxonomy rules (`PolicyRuleTaxonomy` region) over EVERY discovered Shape V2 rust package repo-wide.
 * Discovery is the shared package catalog (`policyDiscoverCrateDirs` → `discoverPackages`), so the
 * plugins/framework/hub split is expressed by each package's declared `role`, not by a path literal:
 * the Wave 4 rules encode plugin-app conventions (`App::builder`, `Plugin::builder`, the SDK artifact app laws) and
 * stay `role = "plugin"`, while the structural taxonomy rules apply to every owner that has adopted the
 * shape. The framework SDK crate is excluded by role, exactly as the old plugins-only path scoping did.
 */
export function policyBreaches(): BreachRecord[] {
  const repoRoot = getWorkspaceRoot();
  const crateDirs = policyDiscoverCrateDirs(repoRoot);
  const breaches: BreachRecord[] = [];

  for (const crate of crateDirs) {
    if (crate.role !== "plugin") continue;
    const abs = join(repoRoot, crate.libRelPath);
    if (!existsSync(abs)) continue; // tolerant: an in-flight taxonomy crate whose lib.rs hasn't landed yet must not crash the lint
    const content = readFileSync(abs, "utf8");
    const lines = content.split(/\r?\n/);

    breaches.push(...policyRegionFormatBreaches(crate.dir, lines));
    breaches.push(...policyManifestRegionBreaches(crate.dir, lines));
    breaches.push(...policyStructNamingBreaches(crate.dir, content));
    breaches.push(...policyModLayoutBreaches(crate.dir, lines));
    breaches.push(...policySelectionIdsBreaches(crate.dir, content));
    breaches.push(...policyArtifactAppLawDelegateBreaches(crate.dir, content));
    breaches.push(...policyTreeItemBreaches(crate, content, lines));
    breaches.push(...policyLabelsStructBreaches(crate, content));
  }

  breaches.push(...policyCargoArtifactBreaches(repoRoot, crateDirs));
  breaches.push(...policyAppCouplingBreaches(repoRoot, crateDirs));
  breaches.push(...policyTaxonomyDirsBreaches(repoRoot, crateDirs));
  breaches.push(...policyWindowCompletenessBreaches(repoRoot, crateDirs));
  breaches.push(...policyModeCompletenessBreaches(repoRoot, crateDirs));
  breaches.push(...policySemioArtifactExamplesBreaches(repoRoot, crateDirs));
  breaches.push(...policyDeadExampleLeafBreaches(repoRoot, crateDirs));
  breaches.push(...policyComponentFileBreaches(repoRoot, crateDirs));
  breaches.push(...policyTaxonomyLibShapeBreaches(repoRoot, crateDirs));
  breaches.push(...policyTaxonomyBarrelShapeBreaches(repoRoot));
  breaches.push(...policySprNamingBreaches(repoRoot, crateDirs));
  breaches.push(...policyBannedNameStemBreaches(repoRoot));
  breaches.push(...policyEmojiPrefixBreaches(repoRoot));
  breaches.push(...policyPluginRootShapeBreaches(repoRoot));
  breaches.push(...policyPluginBuilderBreaches(repoRoot, crateDirs));
  breaches.push(...policyApaBreaches(repoRoot));
  breaches.push(...policyJsonFixtureBreaches(repoRoot));
  breaches.push(...policyOpsGrammarBreaches(repoRoot));
  breaches.push(...policyDslCompletenessBreaches(repoRoot));
  breaches.push(...policyPackCompletenessBreaches(repoRoot));
  breaches.push(...policyCommandEnvelopeCompletenessBreaches(repoRoot));
  breaches.push(...policyDiffCompletenessBreaches(repoRoot));
  breaches.push(...policyGrammarFileBreaches(repoRoot));
  breaches.push(...policyProtocolFileBreaches(repoRoot));
  breaches.push(...policyTsFacadeBreaches(repoRoot));
  breaches.push(...policyMutationArtifactEngineBreaches(repoRoot));
  breaches.push(...policyMutationOutcomeMergePolicyBreaches(repoRoot));
  breaches.push(...policyPluginDependencyParityBreaches(repoRoot));
  breaches.push(...policyContributionTargetBreaches(repoRoot));
  breaches.push(...policyInferenceFamilyBreaches(repoRoot));
  breaches.push(...policySchemaOverhaulS2Breaches(repoRoot));
  breaches.push(...policySchemaOverhaulPCBreaches(repoRoot));
  breaches.push(...policyDissolvedKernelsBreaches(repoRoot));
  breaches.push(...policyCompositionBreaches(repoRoot));
  breaches.push(...policyHandcraftedSpecP3Breaches(repoRoot));
  breaches.push(...policyArtifactSchemaBreaches(repoRoot));
  breaches.push(...policyAppSchemaBreaches(repoRoot));
  breaches.push(...policyStdioArtifactsBreaches(repoRoot));
  breaches.push(...policyProtocolMigrationBreaches(repoRoot));
  breaches.push(...policyDbServerOnlyBreaches(repoRoot));
  breaches.push(...policyOsStateAuthorityBreaches(repoRoot));
  breaches.push(...policyDocumentAppShapeBreaches(repoRoot));
  breaches.push(...policyNoPackFilesBreaches(repoRoot));
  breaches.push(...policyRawSpawnBreaches(repoRoot));
  breaches.push(...policyBudgetNullBreaches(repoRoot));
  breaches.push(...policyMcpConfigBreaches(repoRoot));
  breaches.push(...policyDerivedConfigBreaches(repoRoot));
  breaches.push(...policySniffRealityBreaches(repoRoot));
  breaches.push(...policySubsetConformanceBreaches(repoRoot));
  breaches.push(...policySubsetSurfaceCompletenessBreaches(repoRoot));
  breaches.push(...policyViewerPurityBreaches(repoRoot));
  breaches.push(...policyContributedSurfaceTargetBreaches(repoRoot));
  breaches.push(...policyOsConfigShapeBreaches(repoRoot));
  breaches.push(...policyCleanArtifactStandardSubsetMechanismBreaches(repoRoot));
  breaches.push(...policyPackageLanguagePurityBreaches(repoRoot));
  return breaches;
}
//#endregion 🔖️PolicyExport
//#endregion 🔖️Policy
