/** 📚️ Navbar example audits of the catalog verification: the playground owners that ship without an example picker, the picker coverage audit of a published manifest and the example artifact payload audit. */
import { dialectCoordinate } from "../../../../../../../🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import { exampleBodyAssetPrefix, examplesForApp, normalizeManifestExamples, type PluginManifest } from "../../../../../../../🔨️modules/🛂️manifest/🗣️contributions-and-examples/🟦️.ts";

/** 🎛️ Playground owners that legitimately ship without a navbar example picker (library shells, aggregators, flow hosts). */
export const NAVBAR_EXAMPLE_PICKER_EXEMPT_PLUGIN_IDS = new Set(["demonstrator", "stdio"]);

/** 📚️ Every editor/viewer dialect in a published manifest must offer at least one example row for `NavbarExampleSelect`. */
export function auditNavbarExamplePickerCoverage(manifest: PluginManifest): readonly string[] {
  if (NAVBAR_EXAMPLE_PICKER_EXEMPT_PLUGIN_IDS.has(manifest.pluginId)) return [];
  const admitted = normalizeManifestExamples(manifest);
  const problems: string[] = [];
  for (const app of admitted.apps ?? []) {
    const row = app as { readonly id?: string; readonly dialect?: { readonly artifactKind: string; readonly standard: string; readonly subset: string } };
    if (!row.dialect) continue;
    if (examplesForApp(admitted.examples ?? [], { dialect: row.dialect }).length === 0) {
      problems.push(`${manifest.pluginId}: app ${row.id ?? "?"} (${dialectCoordinate(row.dialect)}) has no manifest examples`);
    }
  }
  return problems;
}

/** 📦️ Every manifest example row the navbar picker can load must ship a body: inline `artifactJson`,
 * or — once the body is deferred at the leaf (`ExampleSource::deferred`) or over
 * `DESCRIPTOR_INLINE_EXAMPLE_MAX_BYTES` (256 KiB) and therefore externalized by `describe` — an
 * `AssetDeclaration` under the row's own body prefix in the descriptor's `assets`
 * ({@link exampleBodyAssetPrefix}). A row with neither is a dead picker entry. */
export function auditNavbarExampleArtifactPayload(manifest: PluginManifest, assets: readonly { readonly name?: string }[] = []): readonly string[] {
  if (NAVBAR_EXAMPLE_PICKER_EXEMPT_PLUGIN_IDS.has(manifest.pluginId)) return [];
  const admitted = normalizeManifestExamples(manifest);
  const declared = assets.map((asset) => asset.name).filter((name): name is string => typeof name === "string");
  const problems: string[] = [];
  for (const example of admitted.examples ?? []) {
    const row = example as { readonly id?: string; readonly artifactJson?: string; readonly dialect?: { readonly artifactKind: string; readonly standard: string; readonly subset: string } };
    const payload = row.artifactJson;
    if (typeof payload === "string" && payload.trim().length > 0) continue;
    if (row.id && row.dialect) {
      const prefix = exampleBodyAssetPrefix({ id: row.id, dialect: row.dialect });
      if (declared.some((name) => name.startsWith(`${prefix}.`))) continue;
    }
    problems.push(`${manifest.pluginId}: example ${String(row.id ?? "?")} has neither inline artifactJson nor a declared example-body asset`);
  }
  return problems;
}
