#!/usr/bin/env bun
import { createHash } from "node:crypto";
import { join } from "node:path";
import { readFileSync } from "node:fs";
import { BundleScript, ScriptRouter } from "../../🏃️process/🧭️routing/🟦️.ts";
import { publishGeneratorInputReceipt } from "./🟦️.ts";

/** 📇️ Scans catalog membership and exact source bytes once before Nx hashes the resulting receipt. */
class RegistryCatalogInputsScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("registry-catalog accepts no arguments");
    const kind = "registry-catalog", policy = JSON.parse(readFileSync(join(import.meta.dir, "../🔣️policy.json"), "utf8"));
    const { loadCatalogTaxonomy, registryCatalogInputPaths, registryCatalogInputView } = await import("../../🔍️discovery/🟦️.ts");
    const taxonomy = loadCatalogTaxonomy(), view = registryCatalogInputView(this.repoRoot, taxonomy), hash = createHash("sha256");
    for (const path of registryCatalogInputPaths(this.repoRoot, taxonomy, view)) {
      const nodeKind = view.kind(path);
      if (nodeKind === "symlink") throw new Error(`Generator input is a symlink: ${path}`);
      const content = nodeKind === "file" ? readFileSync(join(this.repoRoot, path)) : Buffer.alloc(0);
      hash.update(JSON.stringify([path, nodeKind, content.byteLength]) + "\n").update(content);
    }
    const digest = hash.digest("hex");
    publishGeneratorInputReceipt(this.repoRoot, policy.generatorInputs[kind].output, kind, digest);
    console.log(digest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("registry-catalog", RegistryCatalogInputsScript);
if (import.meta.main) await router.run(process.argv.slice(2));
