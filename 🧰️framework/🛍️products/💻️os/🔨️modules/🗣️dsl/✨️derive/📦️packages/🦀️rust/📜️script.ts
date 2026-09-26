#!/usr/bin/env bun
/** ✨️ `@semio-tech/dsl-derive-rs` router: `bun ./📜️script.ts test`. */
import Ajv from "ajv";
import { readFileSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo, runCargoTestBudgeted } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** 🧭️ The committed projection the mutation derive macros read (`MUTATION_AUTHORITY_LOCATOR` in `✨️derive/🦀️.rs`). */
const MUTATION_AUTHORITY_PATH = join(import.meta.dir, "../../🔣️mutation-authority.json");

type TaxonomyFileKind = { emoji?: unknown; extensionChains?: unknown };

/** 🧬️ Projects the repository taxonomy onto the exact facts the mutation derive macros need, validated against
 * `MutationSourceAuthorityProjectionV1`, in the taxonomy's own order so the macros expand exactly as they did from the taxonomy. */
function mutationAuthorityProjection(repoRoot: string): string {
  const project = JSON.parse(readFileSync(join(repoRoot, "📋️project.json"), "utf8"));
  const taxonomy = JSON.parse(readFileSync(join(repoRoot, project.metadata.semio.taxonomy), "utf8"));
  const filename = (kindId: unknown): string => {
    const kind = taxonomy.fileKinds?.[String(kindId)] as TaxonomyFileKind | undefined;
    const extension = Array.isArray(kind?.extensionChains) ? kind.extensionChains[0] : undefined;
    if (typeof kind?.emoji !== "string" || typeof extension !== "string" || !extension.startsWith(".")) throw new Error(`taxonomy file kind ${String(kindId)} has no emoji and canonical extension`);
    return `${kind.emoji}${extension}`;
  };
  const collections = Object.entries(taxonomy.semanticCollections ?? {}).filter(([, definition]) => (definition as { kind?: unknown }).kind === "mutation").map(([name]) => name);
  if (collections.length !== 1) throw new Error("taxonomy mutation collection is ambiguous");
  const projection = {
    schema: "semio.dsl.mutation-source-authority/v1",
    sourceFilename: filename(taxonomy.mutationComponentFileKindId),
    descriptorFilename: filename(taxonomy.mutationDescriptorFileKindId),
    mutationCollection: collections[0],
    mutationPayloadFacet: Array.isArray(taxonomy.mutationBehaviorFacetDirs) ? taxonomy.mutationBehaviorFacetDirs[0] : undefined,
    mutationDomainOwners: taxonomy.mutationDomainOwners ?? {},
    mutationAggregateSources: taxonomy.mutationAggregateSources ?? {},
  };
  const document = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(document);
  const validate = ajv.getSchema(`${document.$id}#/$defs/MutationSourceAuthorityProjectionV1`)!;
  if (!validate(projection)) throw new Error(`mutation authority projection is outside its schema: ${JSON.stringify(validate.errors)}`);
  return `${JSON.stringify(projection, null, 2)}\n`;
}

/** 🔁️ Writes the projection only when its bytes change, so an unrelated taxonomy edit leaves every deriving crate fresh. */
class GenerateScript extends BundleScript {
  async run(): Promise<void> {
    const projected = mutationAuthorityProjection(this.repoRoot);
    let committed: string | undefined;
    try { committed = readFileSync(MUTATION_AUTHORITY_PATH, "utf8"); } catch { committed = undefined; }
    if (committed === projected) return console.log("mutation-authority: unchanged");
    writeFileSync(MUTATION_AUTHORITY_PATH, projected);
    console.log("mutation-authority: written");
  }
}

/** 👁️ Prints the generator-contract preview of the one projection file (`mutation-source-authority` in `generatorContracts`). */
class PreviewGeneratedScript extends BundleScript {
  async run(): Promise<void> {
    const path = relative(this.repoRoot, MUTATION_AUTHORITY_PATH).replaceAll("\\", "/").normalize("NFC");
    const nodes = [{ bytesBase64: Buffer.from(mutationAuthorityProjection(this.repoRoot)).toString("base64"), mode: 0o644, nodeKind: "file", path }];
    process.stdout.write(`${JSON.stringify({ contractId: "mutation-source-authority", nodes, schemaVersion: 1, staleRemovals: [] })}\n`);
  }
}

/** ✅️ Fails when the committed projection is not the projection of the current taxonomy. */
class CheckGeneratedScript extends BundleScript {
  async run(): Promise<void> {
    if (readFileSync(MUTATION_AUTHORITY_PATH, "utf8") !== mutationAuthorityProjection(this.repoRoot)) throw new Error("mutation-authority projection is stale; run bun nx run @semio-tech/dsl-derive-rs:generate");
    console.log("mutation-authority: fresh");
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-os-kernel-dsl-derive"], this.repoRoot, rest);
  }
}

class ExportTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "facade_exports_match_registered_macros", ...segments], this.root);
  }
}

class ExportSourceTestScript extends BundleScript {
  async run(): Promise<void> { await import("../../🧪️tests/📤️macro-exports/🟦️.ts"); }
}

class SourceAuthorityTestScript extends BundleScript {
  async run(): Promise<void> { await import("../../🧪️tests/🛂️mutation-source-authority/🟦️.ts"); }
}

const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("preview-generated", PreviewGeneratedScript).register("check-generated", CheckGeneratedScript).register("test", TestScript).register("test-exports", ExportTestScript).register("test-exports-source", ExportSourceTestScript).register("test-source-authority-source", SourceAuthorityTestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
