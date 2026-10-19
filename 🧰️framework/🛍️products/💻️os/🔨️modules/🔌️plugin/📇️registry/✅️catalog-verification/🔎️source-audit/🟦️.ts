/** 🔎️ Source audit of the catalog verification: the audit of every plugin catalog source against its strict descriptor, its dependency order and its node cap. */
import { existsSync, lstatSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { declaredComponentKind } from "@semio-tech/repo-lib/inventory";
import { CATALOG_ID, DESCRIPTOR_JSON_REL_PATH, findPluginCargoFiles, parseComponentSourceOwnerV1, parseCompiledComponentOwnerV1, type ComponentSourceOwnerV1 } from "../../🔎️discovery/🟦️.ts";
import { CATALOG_DESCRIPTOR_PACK_FILENAME, CATALOG_NODE_MAX, type CatalogSourceAudit, type CatalogSourceIssue, type StrictCatalogDescriptor } from "../🟦️.ts";
import { boundedCatalogDiagnostic } from "../🛡️guards/🟦️.ts";
import { orderCatalogNodes } from "../🗺️verification-plan/🟦️.ts";
import { validateCatalogDescriptorPair } from "../🧬️descriptor-pair/🟦️.ts";

//#region 🔖️CatalogCompleteness
/** 🧮️ Independently enumerates discovered component manifests and audits source identities without reading generated catalog files. */
export function auditPluginCatalogSources(repoRoot = findWorkspaceRoot(), control?: { readonly cancelled?: () => boolean; readonly progress?: (completed: number, total: number, path: string) => void; readonly descriptorAdmission?: "compiled" | "source" }): CatalogSourceAudit {
  const manifestPaths = findPluginCargoFiles(repoRoot);
  if (manifestPaths.length > CATALOG_NODE_MAX) throw new Error(`catalog discovery exceeds ${CATALOG_NODE_MAX} manifests`);
  const entries: ComponentSourceOwnerV1[] = [];
  let sources: StrictCatalogDescriptor[] = [];
  const issues: CatalogSourceIssue[] = [];
  const byPlugin = new Map<string, string>();
  const byPackage = new Map<string, string>();
  for (let index = 0; index < manifestPaths.length; index++) {
    const manifestPath = manifestPaths[index]!;
    if (control?.cancelled?.()) throw new Error("catalog source audit cancelled");
    let entry: ComponentSourceOwnerV1 | undefined;
    try {
      const info = lstatSync(manifestPath);
      if (info.isSymbolicLink() || !info.isFile()) throw new Error("Cargo manifest is not a regular non-symlink file");
      const manifestText = readFileSync(manifestPath, "utf8");
      const rawRole = declaredComponentKind(manifestText);
      if (!rawRole) throw new Error("Component source must declare its exact component-kind");
      entry = parseComponentSourceOwnerV1(manifestPath, repoRoot);
      if (!CATALOG_ID.test(entry.pluginId) || entry.role !== rawRole) throw new Error("Cargo component/role identity is malformed");
      if (rawRole === "extension" && (!entry.extends || entry.dependsOn[0] !== entry.extends)) throw new Error("extension must declare extends as its first dependency");
      if (rawRole === "plugin" && entry.extends !== undefined) throw new Error("plugin must not declare extends");
      const previousPlugin = byPlugin.get(entry.pluginId);
      const previousPackage = byPackage.get(entry.packageName);
      if (previousPlugin || previousPackage) {
        issues.push({ code: "identity-conflict", path: relative(repoRoot, manifestPath), pluginId: entry.pluginId, diagnostic: boundedCatalogDiagnostic(`duplicate identity conflicts with ${previousPlugin ?? previousPackage}`) });
      } else {
        byPlugin.set(entry.pluginId, manifestPath);
        byPackage.set(entry.packageName, manifestPath);
        entries.push(entry);
      }
    } catch (error) {
      issues.push({ code: "manifest-invalid", path: relative(repoRoot, manifestPath), diagnostic: boundedCatalogDiagnostic(error) });
    }
    if (entry && control?.descriptorAdmission !== "source") {
      const jsonPath = resolve(repoRoot, entry.cratePath, ...DESCRIPTOR_JSON_REL_PATH);
      const packPath = join(dirname(jsonPath), CATALOG_DESCRIPTOR_PACK_FILENAME);
      const jsonExists = existsSync(jsonPath);
      const packExists = existsSync(packPath);
      if (!jsonExists && !packExists) {
        issues.push({ code: "descriptor-pair-missing", path: relative(repoRoot, dirname(jsonPath)), pluginId: entry.pluginId, diagnostic: "owner-root descriptor JSON and pack are both missing" });
      } else if (!jsonExists || !packExists) {
        issues.push({ code: "descriptor-pair-incomplete", path: relative(repoRoot, dirname(jsonPath)), pluginId: entry.pluginId, diagnostic: `owner-root descriptor ${jsonExists ? "pack" : "JSON"} is missing` });
      } else {
        try {
          sources.push(validateCatalogDescriptorPair(parseCompiledComponentOwnerV1(manifestPath, repoRoot), repoRoot));
        } catch (error) {
          issues.push({ code: "descriptor-invalid", path: relative(repoRoot, dirname(jsonPath)), pluginId: entry.pluginId, diagnostic: boundedCatalogDiagnostic(error) });
        }
      }
    }
    control?.progress?.(index + 1, manifestPaths.length, relative(repoRoot, manifestPath));
  }
  for (const entry of entries) {
    for (const dependencyId of entry.dependsOn) {
      if (byPlugin.has(dependencyId)) continue;
      issues.push({ code: "dependency-invalid", path: relative(repoRoot, byPlugin.get(entry.pluginId) ?? "Cargo.toml"), pluginId: entry.pluginId, diagnostic: boundedCatalogDiagnostic(`metadata.semio.depends-on names "${dependencyId}", which no discovered crate provides`) });
    }
  }
  let order: string[] = [];
  try {
    order = orderCatalogNodes(entries).map(({ pluginId }) => pluginId);
  } catch (error) {
    issues.push({ code: "dependency-invalid", path: "Cargo.toml", diagnostic: boundedCatalogDiagnostic(error) });
  }
  issues.sort((left, right) => `${left.pluginId ?? ""}:${left.code}:${left.path}`.localeCompare(`${right.pluginId ?? ""}:${right.code}:${right.path}`));
  return { manifestCount: manifestPaths.length, entries, sources, order, issues };
}
//#endregion 🔖️CatalogCompleteness
