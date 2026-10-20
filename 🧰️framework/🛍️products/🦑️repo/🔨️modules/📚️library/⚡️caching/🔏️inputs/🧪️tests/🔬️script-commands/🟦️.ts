import { createHash } from "node:crypto";
import { join } from "node:path";
import { readFileSync } from "node:fs";
import { BundleScript } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { publishGeneratorInputReceipt } from "../../🟦️.ts";
import { dirname as scriptModuleDirname, resolve as scriptModuleResolve } from "node:path";

/** 📍️ Location of the former `📜️script.ts` of this module, kept for code that resolves siblings of its entry. */
const scriptModulePath = scriptModuleResolve(import.meta.dir, "../../📜️script.ts");
const scriptModuleDirectory = scriptModuleDirname(scriptModulePath);



/** 📇️ Scans catalog membership and exact source bytes once before Nx hashes the resulting receipt. */
export class RegistryCatalogInputsScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("registry-catalog accepts no arguments");
    const kind = "registry-catalog", policy = JSON.parse(readFileSync(join(scriptModuleDirectory, "../🔣️policy.json"), "utf8"));
    const { loadCatalogTaxonomy, registryCatalogInputView } = await import("../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts"), { registryCatalogInputPaths } = await import("../../../../🔍️discovery/🧲️registry-catalog-projection/🟦️.ts");
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
