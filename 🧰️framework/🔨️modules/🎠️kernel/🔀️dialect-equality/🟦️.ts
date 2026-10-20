/** 🔀️ Equality of artifact dialects and app references, shared by the app router, the IO entry graph and the opening resolver.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import { type ArtifactDialect } from "@semio-tech/schema-ts/artifact-reference";
import { type AppRef } from "@semio-tech/framework-manifest/schema";

// #region 🎠️Kernel
//#region 🔖️AppRouter
export function dialectEquals(a: ArtifactDialect, b: ArtifactDialect): boolean {
  return a.artifactKind === b.artifactKind && a.standard === b.standard && a.subset === b.subset;
}

export function appRefEquals(a: AppRef, b: AppRef): boolean {
  return a.pluginId === b.pluginId && a.appId === b.appId;
}
//#endregion 🔖️AppRouter
// #endregion 🎠️Kernel
