import {scriptCompilerAdmissionControlV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
/** 🛠️ `dev` command family: playground, Storybook, time-travel, storybook-static and the MCP launchers.
 * @see 📜️script.ts
 */
import { readBuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { daemonBudgetOpts, runCmd } from "../../🏃️process/🟦️.ts";
import { resolveMcpBin } from "../../🚨️policy/🖥️cli/🟦️.ts";
import { Script, createIssuedResourceControlV1, createRequestedIssuedResourceControlV1, scriptInvocationEnvironment } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { existsSync, statSync } from "node:fs";
import { join } from "node:path";

const REPO_MCP_PROFILE_ENV = "SEMIO_REPO_MCP_CLIENT";

/** 🚪️Gates the repo MCP client on the cached `repo-mcp:build` output instead of
 * rebuilding it inline — a continuous `dev` session must stay thin and consume cached deliverables. */
function requireRepoMcpBinary(root: string): string {
  const bin = resolveMcpBin(root);
  if (!existsSync(bin) || !statSync(bin).isFile()) throw new Error(`repo MCP client binary is missing at ${bin}; run: repo run repo-mcp:build`);
  return bin;
}

//#region 🔖️DevScript
export class DevScript extends Script {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "playground") {
      const argv = ["dev", ...segments];
      const { rootDevPlaygroundRequestV1 } = await import("./🌱️birth/🟦️.ts");
      const request = rootDevPlaygroundRequestV1(argv);
      if (!request) throw Error("usage: dev playground <variant> <react|wgpu|native> <prototype|release>");
      const { readOwnedBuildInvocationV1 } = await import("../../🏗️build/📥️caller/🟦️.ts");
      const original = readOwnedBuildInvocationV1(this.repoRoot, this.invocation, argv);
      if (original.buildMode !== request.profile) throw Error("Playground profile must match the original native request");
      const { issueRepositoryScriptProvidersV1 } = await import("../../🏃️process/🔌️providers/🟦️.ts");
      issueRepositoryScriptProvidersV1(process.env);
      const [{ withRepositoryCacheStoreV1 }, { withRepositoryCompilerRuntimeEntriesV1 }] = await Promise.all([
        import("../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts"),
        import("../../🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🟦️.ts"),
      ]);
      const control = createIssuedResourceControlV1(this.invocation, "dev:playground:bootstrap", 0);
      return withRepositoryCompilerRuntimeEntriesV1(original.codeRoot, this.invocation, control, async () => {
        const { serveRootPlaygroundV1 } = await withRepositoryCacheStoreV1(this.repoRoot, "dependencies", this.invocation, control, (_dependencies, phase) =>
          withRepositoryCacheStoreV1(this.repoRoot, "products", this.invocation, phase, () => import("./🎮️playground/🟦️.ts")),
        );
        return serveRootPlaygroundV1(this.repoRoot, this.invocation, argv);
      });
    }
    if (segments[0] === "time-travel") {
      const { issueRepositoryScriptProvidersV1 } = await import("../../🏃️process/🔌️providers/🟦️.ts");
      issueRepositoryScriptProvidersV1(process.env);
      const { runTimeTravelCli } = await import("../../../../../💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts");
      await runTimeTravelCli(this.invocation, this.repoRoot, join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🤖️generated/🧪️time-travel"), segments.slice(1));
      return;
    }
    if (segments[0] === "storybook") {
      await this.runStorybook(segments.slice(1));
      return;
    }
    if (segments[0] === "storybook-static") {
      await this.runStorybookStatic();
      return;
    }
    if (segments[0] === "mcp") {
      await this.runMcp(segments.slice(1));
      return;
    }
    console.error(`[dev] unknown route ${JSON.stringify(segments.join(" "))}: expected playground, storybook, storybook-static, time-travel or mcp.`);
    process.exit(1);
  }

  /** 🔖️ Splits storybook arguments into the scope list and the pass-through arguments. A single scope argument may itself be a comma-list of top-level scopes (`"ui,framework/os"`); several space-separated arguments keep the slash-join of hierarchical scopes (`"framework" "os"` gives `"framework/os"`). */
  private async parseStorybookSegments(segments: string[]): Promise<{ scope: string; args: string[] }> {
    const scopeSegments: string[] = [];
    const args: string[] = [];
    let parsingScope = true;
    for (const segment of segments) {
      if (parsingScope && segment !== "--" && !segment.startsWith("-")) {
        scopeSegments.push(segment);
        continue;
      }
      parsingScope = false;
      if (segment !== "--") args.push(segment);
    }
    const scope = scopeSegments.length === 1 && scopeSegments[0]!.includes(",") ? scopeSegments[0]! : scopeSegments.join("/");
    const scopeIds = scope
      ? scope
          .split(",")
          .map((s) => s.trim())
          .filter(Boolean)
      : [];
    for (const id of scopeIds) {
      if (!/^[a-z0-9][a-z0-9/-]*$/i.test(id)) {
        throw Error(`[dev.storybook] invalid scope ${JSON.stringify(id)}`);
      }
    }
    if (scopeIds.length > 0) {
      try {
        const { resolveActiveScopes } = await import("../../../../../../../.storybook/📖️stories/🧭️coordination/🟦️.ts");
        resolveActiveScopes(scopeIds.join(","));
      } catch (error) {
        throw Error(error instanceof Error ? `[dev.storybook] ${error.message}` : String(error));
      }
    }
    return { scope: scopeIds.join(","), args };
  }

  private async runStorybook(extra: string[]): Promise<void> {
    const storybook = await this.parseStorybookSegments(extra);
    const {serveRepositoryStorybookV1}=await import("../../🏗️build/📖️storybook/🏃️execution/🟦️.ts");
    await serveRepositoryStorybookV1(this.root,storybook.scope,storybook.args,this.invocation);
  }

  private async runStorybookStatic():Promise<void>{
    const {serveRepositoryStorybookArtifactV1}=await import("../../🏗️build/📖️storybook/🏃️execution/🟦️.ts");await serveRepositoryStorybookArtifactV1(this.root,readBuildMode(process.env.SEMIO_BUILD_MODE),process.env.STORYBOOK_SCOPE??"",this.invocation);
  }

  private async runMcp(segments: string[]): Promise<void> {
    const a = segments[0];
    if (a === "stdio") {
      const rest = segments.slice(1);
      if ((rest[0] ?? "").trim().toLowerCase() === "os") {
        await this.runMcpOs(rest.slice(1));
        return;
      }
      this.runMcpStdioRepo(rest);
      return;
    }
    if (a !== undefined && a !== "repo") {
      console.error(`[dev] unknown mcp route ${JSON.stringify(segments.join(" "))}: expected stdio, repo or nothing (the inspector).`);
      process.exit(1);
    }
    const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
    if (a === "repo") {
      runCmd(process.execPath, ["x", "@modelcontextprotocol/inspector", "--config", ".mcp.json", "--server", "repo"], {
        cwd: this.root,
        env: { ...process.env, HOST: host },
        ...daemonBudgetOpts(),
      });
      return;
    }
    runCmd(process.execPath, ["x", "@modelcontextprotocol/inspector"], { cwd: this.root, ...daemonBudgetOpts() });
  }

  /** 🌉️ Runs the `semio-os` MCP gateway (`semio-os-mcp`) over stdio, which is what `.mcp.json` launches.
   * Streamable HTTP is the dashboard tool `tool:workspace/os-mcp-http` (build task `@semio-tech/framework-os-mcp-rs:dev`).
   * Extra argv passes straight through to the binary (`--folder`, `--hub`, `--principal`, `--scopes`, `--auto-approve`, …). */
  private async runMcpOs(extra: string[]): Promise<void> {
    const { servePublishedMcpStdioV1, createMcpPublicationProbeV1, takeMcpPluginCatalog, stageMcpBinaryAsync, buildMcpArtifactV1, MCP_BINARY_TARGET } = await import("../../../../../💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🟦️.ts");
    const {withHandledScriptInvocationSignalV1} = await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
    const catalog = takeMcpPluginCatalog(extra, this.root), stopped = new AbortController();
    const result = await withHandledScriptInvocationSignalV1(this.invocation,stopped.signal,async invocation=>{
      const control = createIssuedResourceControlV1(invocation, "dev:mcp:stdio", 0);
      const {projectOwnedBuildChildEnvironmentV1} = await import("../../🏗️build/📥️caller/📤️child/🟦️.ts");
      const [{ ensureRootDependenciesV1 }, { rootCommandRegistryV1 }] = await Promise.all([import("../🗂️registry/🥾️provision/🟦️.ts"), import("../🗂️registry/🟦️.ts")]);
      const stage = async (target: string, progress: (line: string) => void, signal: AbortSignal): Promise<number> => {
        await ensureRootDependenciesV1(this.root, rootCommandRegistryV1(), "dev", signal);
        if (target === MCP_BINARY_TARGET) {
          progress("[semio-os-mcp] building the owned native publication");
          await buildMcpArtifactV1(this.root,invocation,scriptCompilerAdmissionControlV1(invocation,"dev:mcp:build",0,signal),progress);
          return 0;
        }
        const phase = createRequestedIssuedResourceControlV1(control,0,signal);
        const environment = await projectOwnedBuildChildEnvironmentV1(this.root, ["run", target, "--output=stream"], invocation, phase);
        return stageMcpBinaryAsync(this.root, progress, signal, target, environment);
      };
      const launchEnvironment = scriptInvocationEnvironment(invocation, { ...process.env, ...catalog.env });
      return servePublishedMcpStdioV1(this.root, catalog.args, invocation, control, launchEnvironment, catalog.target, stage,reason=>stopped.abort(reason), createMcpPublicationProbeV1(launchEnvironment));
    });
    process.exit(result.status==="completed"?result.value:0);
  }

  private runMcpStdioRepo(slugs: string[]): void {
    if (slugs.length > 1) {
      console.error("[dev] `dev mcp stdio` accepts one repo client profile.");
      process.exit(1);
    }
    const profile = (slugs[0] ?? "client").trim().toLowerCase();
    const bin = requireRepoMcpBinary(this.root);
    runCmd(bin, ["mcp"], { cwd: this.root, ...daemonBudgetOpts({ GOWORK: join(this.root, "go.work"), [REPO_MCP_PROFILE_ENV]: profile }) });
  }
}
//#endregion 🔖️DevScript
