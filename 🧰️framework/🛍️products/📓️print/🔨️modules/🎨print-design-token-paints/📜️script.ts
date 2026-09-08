import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { BundleScript, ScriptRouter } from "../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { renderPrintLatexTokenStylesheet } from "./🟦️.ts";

class GenerateScript extends BundleScript {
  run(args: string[]): void {
    if (args.length) throw new Error("Print token generation accepts no arguments");
    const path = join(this.repoRoot, "🧰️framework/🛍️products/📓️print/🖋️latex/semio-tokens.sty");
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, renderPrintLatexTokenStylesheet());
    console.log("[print] Wrote LaTeX design tokens");
  }
}

class PreviewScript extends BundleScript {
  run(args: string[]): void {
    if (args.length) throw new Error("Print token preview accepts no arguments");
    const path = join(this.repoRoot, "🧰️framework/🛍️products/📓️print/🖋️latex/semio-tokens.sty");
    const nodes = [{ bytesBase64: Buffer.from(renderPrintLatexTokenStylesheet()).toString("base64"), mode: 0o644, nodeKind: "file", path: relative(this.repoRoot, path).replaceAll("\\", "/").normalize("NFC") }];
    process.stdout.write(`${JSON.stringify({ contractId: "print-latex-tokens", nodes, schemaVersion: 1, staleRemovals: [] })}\n`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("preview-generated", PreviewScript);
if (import.meta.main) await router.run(process.argv.slice(2));
