#!/usr/bin/env bun
/** 🖨️ `@semio-tech/print` router: `bun ./📜️script.ts generate|fonts|build|watch|test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { PrintFontProvisioningCommand } from "../../🎮️commands/🔤print-font-provisioning/🟦️.ts";
import { TemplatePdfBuildCommand } from "../../🎮️commands/🖨️template-pdf-build/🟦️.ts";
import { TemplatePdfWatchCommand } from "../../🎮️commands/👁️template-pdf-watch/🟦️.ts";
import { PrintPipelineVerificationCommand } from "../../🎮️commands/🧪️print-pipeline-verification/🟦️.ts";
import { renderPrintLatexTokenStylesheet } from "../../🔨️modules/🎨print-design-token-paints/🟦️.ts";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";

//#region 🖨️RouterAdapters
function latexTokenTarget(repoRoot: string): { readonly path: string; readonly content: string } {
  const tokenPath = join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json");
  const tokens = JSON.parse(readFileSync(tokenPath, "utf8")) as Parameters<typeof renderPrintLatexTokenStylesheet>[0];
  return { path: join(repoRoot, "🧰️framework/🛍️products/📓️print/🖋️latex/semio-tokens.sty"), content: renderPrintLatexTokenStylesheet(tokens) };
}

class GenerateScript extends BundleScript {
  run(): void {
    const target = latexTokenTarget(this.repoRoot);
    mkdirSync(dirname(target.path), { recursive: true });
    writeFileSync(target.path, target.content, "utf8");
    console.log("print: wrote latex/semio-tokens.sty");
  }
}

/** 🧾️ Emits the canonical read-only generator protocol from the same LaTeX renderer as generate. */
class PreviewGeneratedScript extends BundleScript {
  run(): void {
    const target = latexTokenTarget(this.repoRoot);
    const nodes = [{ bytesBase64: Buffer.from(target.content).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: relative(this.repoRoot, target.path).replaceAll("\\", "/").normalize("NFC") }];
    process.stdout.write(`${JSON.stringify({ contractId: "print-latex-tokens", nodes, schemaVersion: 1, staleRemovals: [] })}\n`);
  }
}

class FontsScript extends BundleScript {
  async run(): Promise<void> {
    await new PrintFontProvisioningCommand(this.root, this.repoRoot).run();
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await new TemplatePdfBuildCommand(this.root, this.repoRoot).run(segments);
  }
}

class WatchScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await new TemplatePdfWatchCommand(this.root, this.repoRoot).run(segments);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await new PrintPipelineVerificationCommand(this.root, this.repoRoot).run(segments);
  }
}
//#endregion 🖨️RouterAdapters

const router = new ScriptRouter(import.meta.dir)
  .register("generate", GenerateScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("fonts", FontsScript)
  .register("build", BuildScript)
  .register("watch", WatchScript)
  .register("test", TestScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
