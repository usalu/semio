/** 🏃️ Entity-kind generator command composition. */
import { existsSync, readFileSync } from "node:fs";
import { relative } from "node:path";
import { BundleScript, getWorkspaceRoot } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { writeGeneratedFileIfChanged } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🗂️files/🟦️.ts";
import { entityKindIndexByEmoji } from "../../🟦️.ts";
import { GENERATOR_ID, REFRESH_COMMAND, readEntityCatalog } from "../📥️source/🟦️.ts";
import { generatedTargets } from "../📋️plan/🟦️.ts";

//#region 🔖️preview-generated
/** 🧾️ Emits the canonical read-only generator protocol from the same byte plan as generate/check. */
export class PreviewGeneratedScript extends BundleScript {
  run(_segments: string[]): void {
    const repoRoot = getWorkspaceRoot();
    const nodes = generatedTargets(repoRoot, readEntityCatalog(repoRoot))
      .map((target) => ({ bytesBase64: Buffer.from(target.content).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: relative(repoRoot, target.path).replaceAll("\\", "/").normalize("NFC") }))
      .sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
    process.stdout.write(`${JSON.stringify({ contractId: GENERATOR_ID, nodes, schemaVersion: 1, staleRemovals: [] })}\n`);
  }
}
//#endregion 🔖️preview-generated

//#region 🔖️generate
export class GenerateScript extends BundleScript {
  run(_segments: string[]): void {
    const repoRoot = getWorkspaceRoot();
    const source = readEntityCatalog(repoRoot);
    for (const target of generatedTargets(repoRoot, source)) {
      writeGeneratedFileIfChanged(target.path, target.content);
    }
    const shadowed = source.kinds.length - entityKindIndexByEmoji(source.kinds).size;
    console.log(`entity catalog refreshed (${source.kinds.length} entity kinds, ${shadowed} emoji-shadowed, sha256 ${source.sha256}) -> 🤖️generated/🏷️entity-kinds/🟦️.ts, ⌨️cli/🏷️entity-kinds/🐹️.go, 🤖️generated/🏷️entity-kinds/🦀️.rs`);
  }
}
//#endregion 🔖️generate

//#region 🔖️check
/** @emoji 🔎️ Renders the catalog in memory and byte-compares it against the committed generated files — never writes. */
export class CheckScript extends BundleScript {
  run(_segments: string[]): void {
    const repoRoot = getWorkspaceRoot();
    const source = readEntityCatalog(repoRoot);
    const stale = generatedTargets(repoRoot, source)
      .filter((target) => !existsSync(target.path) || readFileSync(target.path, "utf8") !== target.content)
      .map((target) => target.path);
    if (stale.length > 0) {
      console.error(`entity catalog is stale: ${stale.join(", ")}`);
      console.error(`run \`${REFRESH_COMMAND}\` to refresh.`);
      process.exit(1);
    }
    console.log(`entity catalog is fresh (${source.kinds.length} entity kinds, sha256 ${source.sha256}).`);
  }
}
//#endregion 🔖️check
