import { dirname, join, relative } from "node:path";
import { existsSync, readFileSync } from "node:fs";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { cargoRepositoryPackages } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { runtimeParseTomlV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";

interface CrateIndexRecord {
  dir: string;
  pkgName: string;
  libName: string;
}

const CARGO_PREFIX_WORDS = new Set(["semio", "s", "framework", "os", "kernel", "plugin", "module", "tech", "app"]);

function generateCargoVariants(name: string): string[] {
  const variants = new Set<string>([name]);
  const parts = name.split("-");

  let firstNonPrefix = parts.length - 1;
  for (const [i, part] of parts.entries()) {
    if (!CARGO_PREFIX_WORDS.has(part)) {
      firstNonPrefix = i;
      break;
    }
  }

  const coreSuffix = parts.slice(firstNonPrefix).join("-");
  variants.add(coreSuffix);

  const prefixParts = parts.slice(0, firstNonPrefix);
  for (const [i, prefix] of prefixParts.entries()) {
    variants.add(parts.slice(i).join("-"));
    variants.add([prefix, ...parts.slice(firstNonPrefix)].join("-"));
  }

  return Array.from(variants);
}

/** 🦀️ Indexes present package identities across explicitly authored Cargo workspace owners. */
function getCargoWorkspaceIndex(repoRoot = getWorkspaceRoot()) {
  const exactPkgNames = new Set<string>();
  const libNameToCrates = new Map<string, CrateIndexRecord[]>();
  const aliasToCrates = new Map<string, CrateIndexRecord[]>();

  const addAlias = (alias: string, record: CrateIndexRecord) => {
    if (!alias) return;
    const normalized = alias.replaceAll("-", "_").toLowerCase();
    const list = aliasToCrates.get(normalized) ?? [];
    if (!list.includes(record)) list.push(record);
    aliasToCrates.set(normalized, list);
  };

  for (const member of cargoRepositoryPackages(repoRoot)) {
    const manifest = join(repoRoot, member.manifest);
    const parsed = runtimeParseTomlV1(readFileSync(manifest, "utf8")) as { package?: { name?: string }; lib?: { name?: string } };
    const pkgName = member.name;
    const libName = parsed.lib?.name ?? pkgName.replaceAll("-", "_");
    const record: CrateIndexRecord = { dir: dirname(manifest), pkgName, libName };
    exactPkgNames.add(pkgName);
    libNameToCrates.set(libName, [...(libNameToCrates.get(libName) ?? []), record]);
    addAlias(libName, record);
    addAlias(pkgName, record);
    for (const variant of generateCargoVariants(pkgName)) addAlias(variant, record);
    for (const variant of generateCargoVariants(libName.replaceAll("_", "-"))) addAlias(variant, record);
  }
  return { exactPkgNames, libNameToCrates, aliasToCrates };
}

export function resolveCargoPackageName(pkg: string, cwd: string): string {
  const index = getCargoWorkspaceIndex();
  if (index.exactPkgNames.has(pkg)) return pkg;

  // 1. Check local Cargo.toml in cwd, cwd/rs, dirname(cwd)
  for (const candidateDir of [cwd, join(cwd, "rs"), dirname(cwd)]) {
    const cargoPath = join(candidateDir, "Cargo.toml");
    if (existsSync(cargoPath)) {
      try {
        const content = readFileSync(cargoPath, "utf8");
        const pkgMatch = content.match(/\[package\][\s\S]*?\bname\s*=\s*"([^"]+)"/);
        const libMatch = content.match(/\[lib\][\s\S]*?\bname\s*=\s*"([^"]+)"/);
        const localPkg = pkgMatch ? pkgMatch[1] : null;
        const localLib = libMatch ? libMatch[1] : localPkg ? localPkg.replaceAll("-", "_") : null;
        if (localPkg) {
          if (pkg === localLib || pkg === localPkg || pkg.replaceAll("-", "_") === localLib || pkg.replaceAll("_", "-") === localPkg) {
            return localPkg;
          }
        }
      } catch {
        /* ignore */
      }
    }
  }

  // 2. Direct libName lookup
  const byLib = index.libNameToCrates.get(pkg);
  const firstLib = byLib?.[0];
  if (byLib && firstLib) {
    if (byLib.length === 1) return firstLib.pkgName;
    let best = firstLib;
    let bestDist = Infinity;
    for (const item of byLib) {
      const dist = relative(cwd, item.dir).length;
      if (dist < bestDist) {
        bestDist = dist;
        best = item;
      }
    }
    return best.pkgName;
  }

  // 3. Alias / prefix-variant lookup
  const normPkg = pkg.replaceAll("-", "_").toLowerCase();
  const byAlias = index.aliasToCrates.get(normPkg);
  const firstAlias = byAlias?.[0];
  if (byAlias && firstAlias) {
    if (byAlias.length === 1) return firstAlias.pkgName;
    let best = firstAlias;
    let bestDist = Infinity;
    for (const item of byAlias) {
      const dist = relative(cwd, item.dir).length;
      if (dist < bestDist) {
        bestDist = dist;
        best = item;
      }
    }
    return best.pkgName;
  }

  // 4. Fallback if cwd has Cargo.toml
  for (const candidateDir of [cwd, join(cwd, "rs")]) {
    const cargoPath = join(candidateDir, "Cargo.toml");
    if (existsSync(cargoPath)) {
      try {
        const content = readFileSync(cargoPath, "utf8");
        const pkgMatch = content.match(/\[package\][\s\S]*?\bname\s*=\s*"([^"]+)"/);
        if (pkgMatch?.[1] !== undefined) return pkgMatch[1];
      } catch {
        /* ignore */
      }
    }
  }

  return pkg;
}

export function resolveCargoPackageNames(packages: string[], cwd: string): string[] {
  if (packages.length === 0) {
    for (const candidateDir of [cwd, join(cwd, "rs")]) {
      const cargoPath = join(candidateDir, "Cargo.toml");
      if (existsSync(cargoPath)) {
        try {
          const content = readFileSync(cargoPath, "utf8");
          const pkgMatch = content.match(/\[package\][\s\S]*?\bname\s*=\s*"([^"]+)"/);
          if (pkgMatch?.[1] !== undefined) return [pkgMatch[1]];
        } catch {
          /* ignore */
        }
      }
    }
    return [];
  }
  return packages.map((pkg) => resolveCargoPackageName(pkg, cwd));
}
