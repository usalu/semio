import { readBuildMode, type BuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { join } from "node:path";

/** 📦️ Authored fields that decide where a browser release bundle is published. */
export type PlaygroundReactDistributionRow = { readonly variant: string; readonly distDir?: string };

function outputDirectory(playground: PlaygroundReactDistributionRow): string | undefined {
  if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(playground.variant)) throw Error("Invalid distribution variant");
  const path = playground.distDir;
  if (path !== undefined && (typeof path !== "string" || !path || /[\\:\u0000-\u001f\u007f]/u.test(path) || path.split("/").some(part => !part || part === "." || part === ".."))) throw Error("Distribution output must be a canonical workspace-relative directory");
  return path;
}

/** 🧭️ Nx output bound to the caller's actual staging project or an explicitly authored directory. */
export function playgroundReactNxOutput(stagingProjectRoot: string, playground: PlaygroundReactDistributionRow, mode: BuildMode): string {
  const declared = outputDirectory(playground);
  return `{workspaceRoot}/${declared ?? `${stagingProjectRoot}/dist/build-${playground.variant}-react`}/${readBuildMode(mode)}`;
}

/** 📂 Absolute destination bound to the producer's caller-supplied staging directory. */
export function playgroundReactOutputPath(workspaceRoot: string, stagingPackageRoot: string, playground: PlaygroundReactDistributionRow, mode: BuildMode): string {
  const declared = outputDirectory(playground);
  return declared === undefined ? join(stagingPackageRoot, "dist", `build-${playground.variant}-react`, readBuildMode(mode)) : join(workspaceRoot, declared, readBuildMode(mode));
}
