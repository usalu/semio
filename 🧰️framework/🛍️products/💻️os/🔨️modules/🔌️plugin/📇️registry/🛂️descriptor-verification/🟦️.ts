import { createHash } from "node:crypto";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { CATALOG_DESCRIPTOR_PACK_FILENAME, StrictCatalogDescriptor, boundedCatalogDiagnostic, rejectPlaceholderCatalogIdentity, validateCatalogDescriptorPair } from "../✅️catalog-verification/🟦️.ts";
import { DESCRIPTOR_JSON_REL_PATH, DeployedRegistryEntryV1 } from "../🔎️discovery/🟦️.ts";



/** 🔎️ Renders the catalog in memory and byte-compares it against `generated/*` plus
 * `.vscode/launch.json` — never writes (a lint/verify step must never let the auto-commit daemon land
 * regenerated files). Launch freshness is folded in here rather than living in a second, unenforced
 * entry point, so one `check` covers every artifact `generate` produces. */
//#region 🔖️DescriptorGate


/** #️⃣ Lowercase hex SHA-256 — same algorithm `semio-framework-plugin-describe` uses for
 * `hashes.wasmSha256` (see that crate's own `sha256_hex` doc for why not this repo's usual
 * `blake3`-based `semio-framework-hash`). */
export function sha256HexOfFile(path: string): string {
  return createHash("sha256").update(readFileSync(path)).digest("hex");
}




export const INTERACTIVE_JOB_RUST_NAMES: Readonly<Record<string, string>> = { Unclassified: "unclassified", Migrated: "migrated", BatchOnlyPendingRewrite: "batchOnlyPendingRewrite", ForbiddenFromUi: "forbiddenFromUi", Deleted: "deleted" };

export const INTERACTIVE_JOB_RUST_CALL = /\.action_interactive_job\(\s*"([^"]+)"\s*,\s*(?:\w+::)*InteractiveJobClassification::(\w+)\s*\)/gu;

export const INTERACTIVE_JOB_SKIP_DIRS = new Set(["node_modules", "dist", "pkg", ".git", "🤖️generated", "🧩️extensions"]);


/** 🧵️ Collects every `action_interactive_job` disposition an owner tree declares in Rust, keyed by
 * action id — an id may legitimately carry a different disposition per surface, so the value is the
 * declared set rather than a single classification. */
export function rustInteractiveJobClassifications(ownerRoot: string): Map<string, Set<string>> {
  const declared = new Map<string, Set<string>>();
  const walk = (directory: string): void => {
    for (const child of readdirSync(directory, { withFileTypes: true })) {
      if (child.isDirectory()) {
        if (!INTERACTIVE_JOB_SKIP_DIRS.has(child.name) && !child.name.startsWith("target")) walk(join(directory, child.name));
      } else if (child.isFile() && child.name.endsWith(".rs")) {
        for (const [, actionId, variant] of readFileSync(join(directory, child.name), "utf8").matchAll(INTERACTIVE_JOB_RUST_CALL)) {
          const classification = INTERACTIVE_JOB_RUST_NAMES[variant!];
          if (classification) (declared.get(actionId!) ?? declared.set(actionId!, new Set()).get(actionId!)!).add(classification);
        }
      }
    }
  };
  if (existsSync(ownerRoot)) walk(ownerRoot);
  return declared;
}


/** 🧵️ Reports every committed action whose explicit `interactiveJob` contradicts what the owner's own
 * Rust declares — the exact shape a source migration that never re-ran `describe` leaves behind, which
 * the runtime then rejects at dispatch. An action the descriptor leaves unclassified, or an id the owner
 * inherits from a shared framework builder rather than declaring itself, is silent here by construction. */
export function auditInteractiveJobClassificationDrift(pluginId: string, ownerRoot: string, descriptor: Record<string, unknown>): string[] {
  const declared = rustInteractiveJobClassifications(ownerRoot);
  if (declared.size === 0) return [];
  const drift: string[] = [];
  const manifest = descriptor.manifest as { apps?: readonly Record<string, any>[] } | undefined;
  for (const app of manifest?.apps ?? []) {
    const rows = [...((app.actions ?? []) as readonly Record<string, any>[]), ...((app.windowKinds ?? []) as readonly Record<string, any>[]).flatMap((windowKind) => (windowKind.actions ?? []) as readonly Record<string, any>[])];
    for (const action of rows) {
      const committed = action.semantics?.execution?.interactiveJob as string | undefined;
      if (!committed || committed === "unclassified") continue;
      const source = declared.get(String(action.id));
      if (source && !source.has(committed)) drift.push(`${pluginId}: ${app.id}#${action.id} is committed as ${JSON.stringify(committed)} but its Rust declares ${[...source].sort().map((value) => JSON.stringify(value)).join(", ")} — re-run \`describe\` for this owner`);
    }
  }
  return drift;
}


/** 🧬️ The component a committed descriptor describes: `describe` hashes the `dist/component-dev` deliverable it staged
 * (`hashes.wasmSha256`, the `rebuild-all` invariant the hub's trusted-catalog preflight enforces as well), so that file is
 * what the descriptor is checked against — never the canonical `wasm-release` artifact of the ambient target dir, whose
 * bytes no descriptor names (an unrelated release build there failed every `check`, ticket 26/09/23 W4). */
export function describedComponentPath(repoRoot: string, entry: Pick<DeployedRegistryEntryV1, "cratePath" | "wasmOut">): string {
  return join(repoRoot, entry.cratePath, "dist", "component-dev", entry.wasmOut);
}


/** 🪪️ One descriptor's component finding: a missing deliverable is a warning ("not built"), bytes that differ from
 * `wasmSha256` are an error (the descriptor is stale against its own build). */
export function describedComponentFindings(repoRoot: string, entry: Pick<DeployedRegistryEntryV1, "pluginId" | "cratePath" | "wasmOut">, wasmSha256: string): { warnings: string[]; errors: string[] } {
  const described = describedComponentPath(repoRoot, entry);
  if (!existsSync(described)) return { warnings: [`${entry.pluginId}: has hashes.wasmSha256 but no component-dev deliverable at ${relative(repoRoot, described)} — the described component is not built`], errors: [] };
  const actual = sha256HexOfFile(described);
  return actual === wasmSha256 ? { warnings: [], errors: [] } : { warnings: [], errors: [`${entry.pluginId}: hashes.wasmSha256 is ${wasmSha256} but ${relative(repoRoot, described)} actually hashes to ${actual} — re-run \`describe\` after the latest build`] };
}


/**
 * 🛂️ `📓️design-abi.md` §3's registry `check` extension, fail-closed: every discovered crate owns a
 * complete `🔣️.json` + `🛂️.descriptor.semio` pair, the pair strict-decodes to one semantically
 * identical value in both forms, it carries no placeholder identity, `pluginId`/`packageId`/`extends`
 * match the Cargo component, every `on-extension-request:<point>` names a real host extension point,
 * the described component's sha256 matches `hashes.wasmSha256` ({@link describedComponentFindings}), and no
 * committed `interactiveJob` contradicts the owner's own Rust classification.
 *
 * A missing, half-published, divergent or placeholder pair is an **error**, not a warning: the strict
 * catalog gate treats exactly these as unpublishable, so a green `check` over them was reporting a
 * catalog state that `catalog-complete` could never accept. The one remaining warning is "wasm not
 * built" — publication identity is verified by `catalog-complete` against a dedicated fresh build
 * root, never against whatever the ambient `target/` happens to hold. `generate` stays permissive so
 * `dev s` keeps booting against a partially described catalog.
 */
export function validateDescriptors(entries: readonly DeployedRegistryEntryV1[], repoRoot: string): { warnings: string[]; errors: string[] } {
  const warnings: string[] = [];
  const errors: string[] = [];
  const byId = new Map(entries.map((entry) => [entry.pluginId, entry]));
  let described = 0;
  for (const entry of entries) {
    const descriptorPath = join(entry.cratePath, ...DESCRIPTOR_JSON_REL_PATH);
    const ownerRoot = resolve(repoRoot, dirname(descriptorPath));
    const jsonExists = existsSync(join(repoRoot, descriptorPath));
    const packExists = existsSync(join(ownerRoot, CATALOG_DESCRIPTOR_PACK_FILENAME));
    if (!jsonExists || !packExists) {
      errors.push(`${entry.pluginId}: owner-root descriptor ${jsonExists ? "pack 🛂️.descriptor.semio" : packExists ? "JSON 🔣️.json" : "pair"} is missing under ${relative(repoRoot, ownerRoot)} — run \`bun ./📜️script.ts describe\` in ${entry.cratePath} after building its wasm32-wasip2 component`);
      continue;
    }
    let pair: StrictCatalogDescriptor | undefined;
    try {
      pair = validateCatalogDescriptorPair(entry, repoRoot);
      rejectPlaceholderCatalogIdentity(entry.pluginId, String((pair.descriptor.manifest as Record<string, unknown>).version));
      errors.push(...auditInteractiveJobClassificationDrift(entry.pluginId, ownerRoot, pair.descriptor as Record<string, unknown>));
    } catch (error) {
      errors.push(`${entry.pluginId}: ${boundedCatalogDiagnostic(error)}`);
      continue;
    }
    described++;
    const firstDependencyId = ((pair.descriptor.manifest as Record<string, unknown>).dependencies as readonly { pluginId?: unknown }[] | undefined)?.[0]?.pluginId;
    if (entry.extends !== undefined && firstDependencyId !== entry.extends) {
      errors.push(`${entry.pluginId}: extends ${JSON.stringify(entry.extends)} but manifest.dependencies[0].pluginId is ${JSON.stringify(firstDependencyId)} — contract freeze §4 rule 1 requires these to match`);
    }
    const hostPluginId = entry.extends;
    const hostExtensionPoints = hostPluginId ? new Set(byId.get(hostPluginId)?.extensionPoints ?? []) : undefined;
    for (const activation of entry.activationEvents) {
      const point = activation.startsWith("on-extension-request:") ? activation.slice("on-extension-request:".length) : undefined;
      if (point === undefined) continue;
      if (hostPluginId === undefined) {
        errors.push(`${entry.pluginId}: declares on-extension-request:${point} but has no "extends" host plugin`);
      } else if (!hostExtensionPoints?.has(point)) {
        errors.push(`${entry.pluginId}: declares on-extension-request:${point}, but host plugin ${JSON.stringify(hostPluginId)} declares no extension point ${JSON.stringify(point)} (has: ${[...(hostExtensionPoints ?? [])].join(", ") || "none"})`);
      }
    }
    if (entry.hashes) {
      const findings = describedComponentFindings(repoRoot, entry, entry.hashes.wasmSha256);
      warnings.push(...findings.warnings);
      errors.push(...findings.errors);
    }
  }
  console.log(`descriptor gate: ${described}/${entries.length} crates own a verified 🔣️.json + 🛂️.descriptor.semio pair.`);
  return { warnings, errors };
}
