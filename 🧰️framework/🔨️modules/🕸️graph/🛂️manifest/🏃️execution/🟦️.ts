import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { buildBudgetMs, cmdBudgetMs } from "../../../🏃️process/⏱️budget/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🏃️ Graph generator command composition. */
import { existsSync, readFileSync } from "node:fs";
import { basename, join, relative, resolve } from "node:path";

import { BundleScript } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { readGraphOutputCatalog } from "../📇️catalog/🟦️.ts";
import { renderGraphArtifacts } from "../📽️projection/🟦️.ts";
import { graphOutputInventory, graphOutputNodes, writeGraphArtifacts } from "../📤️publication/🟦️.ts";

export class GenerateScript extends BundleScript {
  async run(): Promise<void> {
    const root = this.repoRoot;
    const outDir = join(this.root, "..", "..", "🤖️generated");
    const rendered = renderGraphArtifacts(root, outDir, readGraphOutputCatalog(join(this.root,"../../🛂️manifest/📇️outputs.json")));
    writeGraphArtifacts(outDir, rendered.artifacts);
    console.log(`[framework-graph] wrote ${rendered.manifestCount} manifests to ${relative(root, outDir)}`);
  }
}

/** 🧾️Emits exact graph bytes, nested owners, and stale removals without writing the output root. */
export class PreviewGeneratedScript extends BundleScript {
  async run(): Promise<void> {
    const root = this.repoRoot;
    const outDir = join(this.root, "..", "..", "🤖️generated");
    const catalog = readGraphOutputCatalog(join(this.root,"../../🛂️manifest/📇️outputs.json"));
    const rendered = renderGraphArtifacts(root, outDir, catalog, false);
    const rootPath = relative(root, outDir).replaceAll("\\", "/").normalize("NFC");
    const nodes = [
      { bytesBase64: "", mode: 0o755, nodeKind: "directory" as const, path: rootPath },
      ...graphOutputNodes(outDir, rendered.artifacts).filter((entry) => entry.nodeKind === "directory").map((entry) => ({ bytesBase64: "", mode: 0o755, nodeKind: "directory" as const, path: `${rootPath}/${entry.path}` })),
      ...rendered.artifacts.map((artifact) => ({ bytesBase64: Buffer.from(artifact.content).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: relative(root, artifact.path).replaceAll("\\", "/").normalize("NFC") })),
    ].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
    const expected = new Map(graphOutputNodes(outDir, rendered.artifacts).map((entry) => [entry.path, entry.nodeKind]));
    const staleRemovals = graphOutputInventory(outDir)
      .filter((entry) => expected.get(entry.path) !== entry.nodeKind)
      .map((entry) => `${rootPath}/${entry.path.normalize("NFC")}`)
      .sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
    process.stdout.write(`${JSON.stringify({ contractId: catalog.contractId, nodes, schemaVersion: 1, staleRemovals })}\n`);
  }
}

/** ✅️ Checks exact graph catalog bytes and output membership without rewriting artifacts. */
export class CheckGeneratedScript extends BundleScript {
  async run(): Promise<void> {
    const root = this.repoRoot;
    const outDir = join(this.root, "..", "..", "🤖️generated");
    const rendered = renderGraphArtifacts(root, outDir, readGraphOutputCatalog(join(this.root,"../../🛂️manifest/📇️outputs.json")));
    const expected = graphOutputNodes(outDir, rendered.artifacts);
    const actual = graphOutputInventory(outDir);
    const stale = rendered.artifacts.filter((artifact) => !existsSync(artifact.path) || readFileSync(artifact.path, "utf8") !== artifact.content).map((artifact) => basename(artifact.path));
    if (JSON.stringify(actual) !== JSON.stringify(expected) || stale.length > 0) throw new Error(`framework-graph generated catalog is stale: membership=${JSON.stringify(actual) !== JSON.stringify(expected)}, files=${JSON.stringify(stale)}`);
    await runOwnedCommand("bun", ["test", resolve(this.root, "../../🧪️tests/🧩️suite/🟦️.ts")], this.repoRoot, "tool:owner", 60_000, {env: process.env});
    console.log(`[framework-graph] ${rendered.manifestCount} generated manifests are fresh`);
  }
}

export class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { runCargoTestsV1, readCargoTestPolicyV1 } = await import("../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts");
    const { rest } = resolveTestLevel(segments);
    await runOwnedCommand("bun", ["test", resolve(this.root, "../../🧪️tests/🧩️suite/🟦️.ts")], this.repoRoot, "tool:owner", cmdBudgetMs(), {env: process.env});
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-graph"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}

/** 🧹️Zero-warning clippy gate: `cargo clippy -p semio-framework-graph --all-targets -- -D warnings`. */
export class LintScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { runCargoLintV1, readCargoTestPolicyV1 } = await import("../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts");
    await runCargoLintV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-graph"],cwd:this.root,extraArgs:segments},readCargoTestPolicyV1(process.env));
  }
}


/** 🧪️ Runs the owning package's generated exact-total enum wire laws. */
export class OwnerGraphWireCheckScript extends BundleScript {
  async run(segments:string[]):Promise<void> {
    if(segments.length) throw new Error("Owned graph wire law has no arguments");
    const manifest=Bun.TOML.parse(readFileSync(join(this.root,"Cargo.toml"),"utf8")) as {package?:{name?:string}};
    const name=manifest.package?.name;
    if(typeof name!=="string" || !/^[a-z][a-z0-9-]+$/u.test(name)) throw new Error("Owned graph wire law requires an actual Cargo package identity");
    await runOwnedCommand("cargo",["test","--manifest-path",join(this.root,"Cargo.toml"),"-p",name,"--lib","owner_wire_law","--","--nocapture"],this.root,"cargo:owner-wire",buildBudgetMs(),{env:process.env});
  }
}

/** 🧪️Runs portable graph ownership and independent schema laws without native compilation. */
export class ManifestContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("Graph manifest contract takes no arguments");
    await runOwnedCommand("bun", ["test", resolve(this.root, "../../🧪️tests/🧩️suite/🟦️.ts")], this.repoRoot, "graph:manifest-contract", cmdBudgetMs(), { env: process.env });
  }
}
