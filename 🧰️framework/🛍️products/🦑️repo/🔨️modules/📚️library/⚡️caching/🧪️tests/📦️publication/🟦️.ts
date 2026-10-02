import { testCargoArtifactPublication } from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/📦️publication/🟦️.ts";
import { testArtifactPublication as testNeutralArtifactPublication } from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/📤️publication/🧪️tests/🟦️.ts";

/** 📦️ Executes neutral publication laws and the repository's actual selected Cargo composition oracle. */
export async function testArtifactPublication(output: string): Promise<void> {
  await testNeutralArtifactPublication(output);
  await testCargoArtifactPublication(output);
}

