import { generateStylingArtifacts, checkStylingArtifacts } from "../../../../../../../🔨️modules/🖱️ui/🎨️styling/📽️projection/🟦️.ts";
import { publishAssetArtifacts, checkAssetArtifacts } from "../../../../../../../🔨️modules/🖼️assets/🏗️builder/📦️publication/🟦️.ts";
import { writeGeneratedFileIfChanged } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts";

/** 🧰️ Prepares the authored browser catalogs before either Storybook compiler entry starts. */
export function prepareStorybookAssets(check: () => void): void {
  check();
  generateStylingArtifacts((...args) => {
    check();
    return writeGeneratedFileIfChanged(...args);
  });
  check();
  publishAssetArtifacts();
  check();
  checkStylingArtifacts();
  checkAssetArtifacts();
  check();
}
