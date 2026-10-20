import { testLevelAtLeast } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { describe } from "bun:test";
import { registerAuthoredDrawPreflightContextTests } from "./🔶️authored-draw-preflight-context/🟦️.ts";
import { registerAuthoredDrawResidueRecoveryTests } from "./🔸️authored-draw-residue-recovery/🟦️.ts";
import { registerAuthoredDrawSourceProducerTests } from "./🟠️authored-draw-source-producer/🟦️.ts";
import { registerArtifactExampleModelCatalogTests } from "./🟢️artifact-example-model-catalog/🟦️.ts";

describe.if(testLevelAtLeast("long"))("artifact path projection authority", () => {
  registerAuthoredDrawPreflightContextTests();
  registerAuthoredDrawResidueRecoveryTests();
  registerAuthoredDrawSourceProducerTests();
  registerArtifactExampleModelCatalogTests();
});
