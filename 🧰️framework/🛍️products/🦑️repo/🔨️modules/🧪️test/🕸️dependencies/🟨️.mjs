import { observedFs } from "../../📚️library/📦️packages/🟦️typescript/🏗️build/🟨️.mjs";
import { join, resolve, relative, isAbsolute } from "node:path";
import { createRequire } from "node:module";

const { existsSync, readFileSync } = observedFs;

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

/** 🔖️ Whether an oracle pin names an installable release: a bare version or a PEP 440 / npm range, never a lock marker such as `locked`, `*` or `builtin`. */
export function isInstallablePin(version) {
  return typeof version === "string" && /^(?:[<>=!~]=?\s*)?\d/.test(version);
}

/** 🌱️ The external host distributions one oracle declares: its primary package and each linked one, in the adapter language its ecosystem runs in. Registering the oracle IS the declaration. */
export function oracleHostPackagesOf(oracle) {
  const implementation = { python: "python", javascript: "typescript" }[oracle.ecosystem];
  if (implementation === undefined) return [];
  return [oracle, ...(oracle.packages ?? [])]
    .filter((entry) => typeof entry.package === "string" && entry.package.length > 0 && !entry.package.startsWith("node:"))
    .map((entry) => ({ implementation, package: entry.package, ...(isInstallablePin(entry.version) ? { version: entry.version } : {}), ...(entry.module === undefined ? {} : { module: entry.module }) }));
}

/** 🔮️ Selects every applicable ancestor contribution for one adapter implementation: the explicit local sources plus the external distributions its oracles declare, one row per package. */
export function packagesForOwner(contributions, owner, implementation) {
  const ancestors = new Set(ownerAncestors(owner));
  const merged = new Map();
  for (const entry of contributions.filter((candidate) => ancestors.has(candidate.owner)))
    for (const row of [...(entry.oracleHostPackages ?? []), ...(entry.oracles ?? []).flatMap(oracleHostPackagesOf)]) {
      if (implementation !== undefined && row.implementation !== implementation) continue;
      const key = `${row.implementation}:${row.path ?? ""}:${row.package}`;
      const previous = merged.get(key);
      if (previous === undefined || (previous.version === undefined && row.version !== undefined)) merged.set(key, { ...previous, ...row });
    }
  return [...merged.values()];
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
