/** 🧾️ Development owns activation and staged component convergence. */
import { join } from "node:path";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { developmentRuntimeRoot, pluginModulesRoot, readActivationReceipt } from "../../♻️activation/🟦️.ts";
import { COMPONENT_RECEIPT_FILE, SHIPPED_COMPONENT_PROFILE, componentDeliverableV1, readComponentReceiptV1 } from "../../../🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";
import { moduleDirectoryName } from "../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { REGISTRY_DIAGNOSTICS_FILE, parseRegistryChannelDiagnosticsV1 } from "../../../🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";
import { readGeneratedCatalogProjection, runtimeCatalogProjectionRootV1, registryModuleDirectories } from "../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";

/** 🧾️ One component's convergence row: its committed semantic descriptor, the receipt of its shipped `dist/components`
 * deliverable, the module page staged from it and the activation receipt must name ONE build. */
export type StagedConvergenceRowV1 = Readonly<{ pluginId: string; committed: string; dist: string; staged: string; activated: boolean; missing: readonly string[]; ok: boolean }>;

/** 🔍️ Proves `receipt(committed) == dist == staged` and full staging for every registry component of one dev variant: the shipped
 * receipt names the committed semantic descriptor and the bytes the module page binds; every plugin the dev catalog withholds
 * (§21.4 stale channel) is a refused row. */
export function stagedConvergence(repoRoot: string, variant: string): Readonly<{ rows: readonly StagedConvergenceRowV1[]; receiptPlugins: number }> {
  const sha = (path: string): string => (existsSync(path) ? createHash("sha256").update(readFileSync(path)).digest("hex") : "-");
  const registry = readGeneratedCatalogProjection().entries;
  const inventory = registryModuleDirectories(registry);
  const receipt = readActivationReceipt(join(developmentRuntimeRoot(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"), variant, "prototype", "react"), "activation"));
  const activated = new Set(receipt.plugins.map((row) => row.pluginId));
  const modules = pluginModulesRoot();
  const rows = registry.map((entry) => {
    const deliverable = componentDeliverableV1(join(repoRoot, entry.cratePath), entry.packageName, SHIPPED_COMPONENT_PROFILE);
    const receipt = existsSync(deliverable.receipt) ? readComponentReceiptV1(deliverable.receipt) : undefined;
    const semantic = sha(join(repoRoot, entry.cratePath, "..", "..", "🛂️.descriptor.semio"));
    const committed = receipt && receipt.semanticDescriptorSha256 === semantic ? receipt.wasmSha256 : "-";
    const dist = sha(deliverable.wasm);
    const moduleRoot = join(modules, moduleDirectoryName(entry.pluginId, inventory));
    const stagedReceipt = join(moduleRoot, COMPONENT_RECEIPT_FILE);
    const staged = existsSync(stagedReceipt) ? readComponentReceiptV1(stagedReceipt).wasmSha256 : "-";
    const manifest = join(moduleRoot, ".semio-artifact.json");
    const files = existsSync(manifest) ? (JSON.parse(readFileSync(manifest, "utf8")) as { files: string[] }).files : [];
    const missing = [...files.filter((file) => !existsSync(join(moduleRoot, file))), ...(files.some((file) => file.endsWith("_component.core.wasm")) ? [] : ["*_component.core.wasm"])];
    const ok = committed !== "-" && committed === dist && dist === staged && activated.has(entry.pluginId) && missing.length === 0;
    return Object.freeze({ pluginId: entry.pluginId, committed, dist, staged, activated: activated.has(entry.pluginId), missing: Object.freeze(missing), ok });
  });
  const diagnostics = join(runtimeCatalogProjectionRootV1() ?? (()=>{throw Error("Convergence requires a supplied catalog root");})(), REGISTRY_DIAGNOSTICS_FILE);
  const withheld = existsSync(diagnostics) ? parseRegistryChannelDiagnosticsV1(readFileSync(diagnostics, "utf8")).map((row) => Object.freeze({ pluginId: row.pluginId, committed: row.code, dist: "-", staged: "-", activated: activated.has(row.pluginId), missing: Object.freeze([]), ok: false })) : [];
  return Object.freeze({ rows: Object.freeze([...rows, ...withheld]), receiptPlugins: receipt.plugins.length });
}

/** ✅️ `verify-staged --variant <variant>`: one row per component, the summary line, exit 1 when any component diverged. */
export class VerifyStagedScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 2 || segments[0] !== "--variant") throw new Error("usage: verify-staged --variant <variant>");
    const { rows, receiptPlugins } = stagedConvergence(this.repoRoot, segments[1]!);
    for (const row of rows) console.log(`${row.ok ? "OK  " : "DIFF"} ${row.pluginId.padEnd(34)} committed=${row.committed.slice(0, 12)} dist=${row.dist.slice(0, 12)} staged=${row.staged.slice(0, 12)} activated=${row.activated}${row.missing.length ? ` missing=${row.missing.slice(0, 3).join(",")}` : ""}`);
    const diverged = rows.filter((row) => !row.ok).length;
    console.log(`components=${rows.length} consistent=${rows.length - diverged} diverged=${diverged} receiptPlugins=${receiptPlugins}`);
    if (diverged > 0) process.exitCode = 1;
  }
}
