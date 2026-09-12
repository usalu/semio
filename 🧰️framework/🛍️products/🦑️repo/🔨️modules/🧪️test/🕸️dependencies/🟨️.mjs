import { existsSync, readFileSync } from "node:fs";
import { join, resolve, relative, isAbsolute } from "node:path";
import { createRequire } from "node:module";

/** 🦀️ Reads Cargo package identity through the tooling parser boundary. */
export function cargoPackage(workspaceRoot, path) {
  const manifest = join(workspaceRoot, path, "Cargo.toml");
  return existsSync(manifest) ? createRequire(import.meta.url)("@iarna/toml").parse(readFileSync(manifest, "utf8")).package : undefined;
}

/** 📍️ Resolves a repository-owned dependency without accepting traversal outside the workspace. */
export function localPackagePath(workspaceRoot, path) {
  const local = relative(workspaceRoot, resolve(workspaceRoot, path)).replaceAll("\\", "/");
  if (isAbsolute(local) || local === ".." || local.startsWith("../")) throw new Error(`Test dependency escapes the workspace: ${path}`);
  return local || ".";
}

/** 🌳️ Enumerates an owner's ancestry through the workspace root. */
export function ownerAncestors(owner) {
  const segments = owner === "." ? [] : owner.split("/");
  return Array.from({ length: segments.length + 1 }, (_, index) => segments.slice(0, segments.length - index).join("/") || ".");
}

/** 🧪️ Selects the same subject package for graph inference and generated host execution. */
export function rustSubjectPackage(workspaceRoot, owner) {
  for (const ancestor of ownerAncestors(owner)) {
    const path = localPackagePath(workspaceRoot, `${ancestor}/📦️packages/🦀️rust`), pkg = cargoPackage(workspaceRoot, path);
    if (pkg?.name === "semio-repo-test-host") return null;
    if (pkg?.name) return { name: pkg.name, path };
  }
  return null;
}

/** 🔮️ Selects every applicable ancestor contribution for one adapter implementation. */
export function packagesForOwner(contributions, owner, implementation) {
  const ancestors = new Set(ownerAncestors(owner));
  return contributions.filter((entry) => ancestors.has(entry.owner)).flatMap((entry) => entry.oracleHostPackages ?? []).filter((entry) => implementation === undefined || entry.implementation === implementation);
}

/** 🧾️ Reads only the contribution manifests that can supply packages to this owner. */
export function ownerContributions(workspaceRoot, vocabulary, owner) {
  const kind = vocabulary.fileKinds[vocabulary.testContributionFileKindId];
  return ownerAncestors(owner).reverse().flatMap((ancestor) => {
    const path = join(workspaceRoot, ancestor, vocabulary.testOraclesDirName, `${kind.emoji}${kind.extensionChains[0]}`);
    if (!existsSync(path)) return [];
    try { return [{ ...JSON.parse(readFileSync(path, "utf8")), owner: ancestor }]; }
    catch { return []; }
  });
}
