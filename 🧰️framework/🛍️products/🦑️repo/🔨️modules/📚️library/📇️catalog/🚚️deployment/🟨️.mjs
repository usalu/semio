import { parseInstallationDirectoryV1 } from "../../../../../../🔨️modules/🪪️identity/📁️installation/🟨️.mjs";

/** 🚚️ Admits optional owner-authored deployment without publishing compiled-only components. */
export function componentDeploymentDirectoryV1(metadata, admit = parseInstallationDirectoryV1) {
  const value = metadata?.["deployment-directory"];
  return value === undefined ? undefined : admit(value);
}
