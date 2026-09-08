import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter } from "../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { buildRegisteredPrintTemplate, printTemplatePdfNames } from "./🟦️.ts";
import { printDocuments, printDocumentOutputDirectory } from "./📇️catalog/🟦️.ts";

class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length !== 1) throw new Error("Print build requires exactly one document id");
    const controller = new AbortController(), abort = (): void => controller.abort(new Error("Print compilation cancelled"));
    process.once("SIGINT", abort); process.once("SIGTERM", abort);
    try { await buildRegisteredPrintTemplate(args[0]!, controller.signal); }
    finally { process.removeListener("SIGINT", abort); process.removeListener("SIGTERM", abort); }
  }
}

class CompleteScript extends BundleScript {
  run(args: string[]): void {
    if (args.length !== 1 || !["templates", "visualizations"].includes(args[0]!)) throw new Error("Print completion requires a document collection");
    const documents = printDocuments().filter(document => document.collection === args[0]);
    for (const document of documents) for (const name of Object.values(printTemplatePdfNames(document.texPath))) {
      const path = join(printDocumentOutputDirectory(document.id, this.repoRoot), name);
      if (readFileSync(path).subarray(0, 5).toString() !== "%PDF-") throw new Error(`Invalid PDF: ${path}`);
    }
    console.log(`[print] ${documents.length * 2} PDFs ready`);
  }
}

class WatchScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Print watch accepts no compiler arguments");
    console.log("[print] Ready for Nx source-watch rebuilds");
    await new Promise<void>(accept => {
      const timer = setInterval(() => {}, 60000);
      const stop = (): void => { clearInterval(timer); process.removeListener("SIGINT", stop); process.removeListener("SIGTERM", stop); accept(); };
      process.once("SIGINT", stop); process.once("SIGTERM", stop);
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("complete", CompleteScript).register("watch", WatchScript);
if (import.meta.main) await router.run(process.argv.slice(2));
