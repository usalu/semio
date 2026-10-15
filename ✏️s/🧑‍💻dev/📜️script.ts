#!/usr/bin/env bun
import {resolve} from "node:path";
import {readFileSync} from "node:fs";
import { receiveScriptProcessInvocation } from "../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
class CompositionScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("composition-check accepts no arguments");
    const { sDevCompositionLawsV1 } = await import("./🧩️service-composition/🧪️tests/🔄️lifecycle/🟦️.ts");
    await sDevCompositionLawsV1();
  }
}
class SourceScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("source-check accepts no arguments");
    const { sDevCompositionOwnershipV1 } = await import("./🧩️service-composition/🧪️tests/🏷️ownership/🟦️.ts");
    await sDevCompositionOwnershipV1(this.repoRoot);
  }
}
class CatalogScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("catalog-check accepts no arguments");
    const { resolve } = await import("node:path");
    const { runOwnedCommand } = await import("../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts");
    await runOwnedCommand(process.execPath, [resolve(this.repoRoot, "node_modules/vitest/vitest.mjs"), "run", "--config", resolve(import.meta.dirname, "🧩️catalog/🧪️tests/🎚️config/🟦️.ts")], this.repoRoot, "specific-catalog", 300000, { env: { ...process.env, SEMIO_TEST_LEVEL: "long" } });
  }
}
class JackGraphScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("jack-graph-check accepts no arguments");
    const { resolve } = await import("node:path");
    const { runRepositoryCommand } = await import("../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts");
    const suite = resolve(import.meta.dir, "🔱️trinity/🪆️graph/🧪️tests/🟦️.ts");
    const budget = { version: 1 as const, maximumElapsedMilliseconds: 120000, output: { maximumBytes: 134217728, maximumLines: 65536 } };
    await runRepositoryCommand(this.invocation, process.execPath, ["x", "tsc", suite, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck"], this.repoRoot, "specific-jack-graph:source", budget);
    await runRepositoryCommand(this.invocation, process.execPath, ["test", suite], this.repoRoot, "specific-jack-graph:tests", budget);
    console.log("[s-dev] checked retained Semio/Jack graph composition");
  }
}
class PlaygroundSessionScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length > 1 || (args[0] !== undefined && args[0] !== "check" && args[0] !== "preview")) throw new Error("Specific session accepts check or preview");
    const { default: owning } = await import("./🧩️catalog/🎮️session/🔣️.json");
    const { parsePlaygroundSessionPublicationRequestV1 } = await import("../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🎮️playground-session/🧬️schema/🟦️.ts");
    const { PlaygroundSessionGenerateScript, PlaygroundSessionPreviewScript } = await import("../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🎮️playground-session/🏃️execution/🟦️.ts");
    const request = parsePlaygroundSessionPublicationRequestV1(owning);
    if (args[0] === "preview") await new PlaygroundSessionPreviewScript(this.root, this.repoRoot, this.invocation, request).run([]);
    else await new PlaygroundSessionGenerateScript(this.root, this.repoRoot, this.invocation, request).run(args);
  }
}
class CatalogPublicationScript extends BundleScript {
  async run(args:string[]):Promise<void>{
    if(args.length!==1||!["generate","check","preview","verify"].includes(args[0]!))throw Error("catalog-publication requires generate, check, preview or verify");
    const {specificCatalogPublicationV1}=await import("./🧩️catalog/📣️publication/🟦️.ts");
    const {GenerateScript,CheckGeneratedScript,PreviewGeneratedScript,CheckScript}=await import("../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts");
    const selected=args[0]==="generate"?GenerateScript:args[0]==="check"?CheckGeneratedScript:args[0]==="preview"?PreviewGeneratedScript:CheckScript;
    await new selected(this.root,this.repoRoot,this.invocation,specificCatalogPublicationV1(path=>readFileSync(resolve(this.repoRoot,path),"utf8"))).run([]);
  }
}
class CatalogPublicationTestScript extends BundleScript {
  async run(args:string[]):Promise<void>{
    if(args.length)throw Error("catalog-publication-test accepts no arguments");
    const {runOwnedCommand}=await import("../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts");
    await runOwnedCommand(process.execPath,["test",resolve(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🧪️tests/🟦️.ts"),resolve(this.root,"🧩️catalog/📣️publication/🧪️tests/🟦️.ts"),resolve(this.root,"🧩️catalog/🌐️browser-host/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🧪️tests/🟦️.ts"),resolve(this.root,"🔁️rebuild/🧪️tests/🟦️.ts"),resolve(this.root,"🧩️catalog/🧪️tests/🧩️native-prerequisites/🟦️.ts"),resolve(this.root,"🧩️catalog/🧪️tests/🏗️preparation/🟦️.ts"),resolve(this.root,"🧩️catalog/🧪️tests/🌐️playground-site-dist/🟦️.ts")],this.repoRoot,"catalog-publication",120000,{signal:this.invocation.control.signal});
  }
}
/** 🤝️ The real-client end-to-end gate: spawns BOTH `.mcp.json` servers with the literal `command`/`args` a Claude Code or Cursor host uses, then drives the whole agent journey over stdio JSON-RPC against the pins this specific layer owns. `--folder <path>` binds an existing space instead of a throwaway one; the capability catalog always comes from the repo's installed plugin registry. */
class McpClientEndToEndScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const flag = args.indexOf("--folder");
    if (flag >= 0 && !args[flag + 1]) throw new Error("usage: mcp-client-e2e [--folder <path>]");
    const { mkdtempSync } = await import("node:fs");
    const { tmpdir } = await import("node:os");
    const { join } = await import("node:path");
    const { requireMcpBinary, runMcpClientEndToEnd } = await import("../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts");
    const { SPECIFIC_MCP_CLIENT_E2E_PINS_V1 } = await import("./🤝️mcp-client-e2e/🟦️.ts");
    const folder = flag >= 0 ? join(this.repoRoot, args[flag + 1] as string) : mkdtempSync(join(tmpdir(), "semio-mcp-client-e2e-"));
    console.log(`[mcp-client-e2e] binary ${requireMcpBinary(this.repoRoot)}`);
    console.log(`[mcp-client-e2e] workspace folder ${folder}`);
    const steps = await runMcpClientEndToEnd(this.repoRoot, folder, SPECIFIC_MCP_CLIENT_E2E_PINS_V1);
    for (const step of steps) console.log(`${step.ok ? "PASS" : "FAIL"}  ${step.step} — ${step.detail}`);
    const failed = steps.filter((step) => !step.ok);
    console.log(`mcp-client-e2e: ${steps.length - failed.length}/${steps.length} steps green over the two real .mcp.json servers.`);
    if (failed.length > 0) throw new Error(`mcp-client-e2e: ${failed.length} step(s) red: ${failed.map((step) => step.step).join(", ")}`);
  }
}
const router = new ScriptRouter(import.meta.dir,resolve(import.meta.dir,"../..")).register("composition-check", CompositionScript).register("mcp-client-e2e", McpClientEndToEndScript).register("source-check", SourceScript).register("catalog-check", CatalogScript).register("catalog-publication",CatalogPublicationScript).register("catalog-publication-test",CatalogPublicationTestScript).register("jack-graph-check", JackGraphScript).register("playground-session", PlaygroundSessionScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
