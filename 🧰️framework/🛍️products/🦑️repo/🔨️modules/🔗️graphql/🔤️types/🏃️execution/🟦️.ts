/** 🏃️ GraphQL type generator command composition: generate, check and preview. */
import { existsSync, readFileSync } from "node:fs";
import { relative } from "node:path";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { writeGeneratedFileIfChanged } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts";
import { generatedTargets } from "../📋️plan/🟦️.ts";
import { GENERATOR_ID, REFRESH_COMMAND } from "../📽️projection/🟦️.ts";
import { readGraphqlTypesSource } from "../📥️source/🟦️.ts";

//#region 🔖️preview-generated
/** 🧾️ Emits the canonical read-only generator protocol from the same byte plan as generate/check. */
export class PreviewGeneratedScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("GraphQL type generation accepts no arguments");
    const repoRoot = this.repoRoot;
    const nodes = generatedTargets(await readGraphqlTypesSource(repoRoot))
      .map((target) => ({ bytesBase64: Buffer.from(target.content).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: relative(repoRoot, target.path).replaceAll("\\", "/").normalize("NFC") }))
      .sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
    process.stdout.write(`${JSON.stringify({ contractId: GENERATOR_ID, nodes, schemaVersion: 1, staleRemovals: [] })}\n`);
  }
}
//#endregion 🔖️preview-generated

//#region 🔖️generate
export class GenerateScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("GraphQL type generation accepts no arguments");
    const source = await readGraphqlTypesSource(this.repoRoot);
    for (const target of generatedTargets(source)) writeGeneratedFileIfChanged(target.path, target.content);
    console.log(`graphql types refreshed (${source.operations.length} operations, schema sha256 ${source.schemaSha256}) -> 🤖️generated/🔗️graphql-types/🟦️.ts`);
  }
}
//#endregion 🔖️generate

//#region 🔖️check
/** 🔎️ Renders the types in memory and byte-compares them against the generated file — never writes. */
export class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("GraphQL type generation accepts no arguments");
    const source = await readGraphqlTypesSource(this.repoRoot);
    const stale = generatedTargets(source).filter((target) => !existsSync(target.path) || readFileSync(target.path, "utf8") !== target.content).map((target) => target.path);
    if (stale.length > 0) {
      console.error(`graphql types are stale: ${stale.join(", ")}`);
      console.error(`run \`${REFRESH_COMMAND}\` to refresh.`);
      process.exit(1);
    }
    console.log(`graphql types are fresh (${source.operations.length} operations, schema sha256 ${source.schemaSha256}).`);
  }
}
//#endregion 🔖️check
