/** 🛠️ `dev` command family: storybook, storybook-static and the MCP launchers.
 * @see 📜️script.ts
 */
import { readBuildMode } from "../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { ensureMcpBinary } from "../../../../../💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { daemonBudgetOpts, runCmd } from "../../🏃️process/🟦️.ts";
import { resolveMcpBin } from "../../🚨️policy/🖥️cli/🟦️.ts";
import { Script } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { existsSync, statSync } from "node:fs";
import { join } from "node:path";

const REPO_MCP_PROFILE_ENV = "SEMIO_REPO_MCP_CLIENT";

/** 🚪️Gates the repo MCP client on the cached `repo-mcp:build` output instead of
 * rebuilding it inline — a continuous `dev` session must stay thin and consume cached deliverables. */
function requireRepoMcpBinary(root: string): string {
  const bin = resolveMcpBin(root);
  if (!existsSync(bin) || !statSync(bin).isFile()) throw new Error(`repo MCP client binary is missing at ${bin}; run: bun repo run repo-mcp:build`);
  return bin;
}

//#region 🔖️DevScript
export class DevScript extends Script {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "storybook") {
      await this.runStorybook(segments.slice(1));
      return;
    }
    if (segments[0] === "storybook-static") {
      await this.runStorybookStatic();
      return;
    }
    if (segments[0] === "mcp") {
      this.runMcp(segments.slice(1));
      return;
    }
    console.error(`[dev] unknown route ${JSON.stringify(segments.join(" "))}: expected storybook, storybook-static or mcp. Playgrounds are dashboard commands: \`bun run dashboard run playground:<variant> --param renderer=react\` (list them with \`bun run dashboard commands playground\`).`);
    process.exit(1);
  }

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
    // A single scope arg may itself be a comma-list of top-level scopes ("ui,framework/os").
    // Multiple space-separated args keep slash-join behavior for hierarchical scopes ("framework" "os" -> "framework/os").
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

  private runMcp(segments: string[]): void {
    const a = segments[0];
    if (a === "stdio") {
      const rest = segments.slice(1);
      if ((rest[0] ?? "").trim().toLowerCase() === "os") {
        this.runMcpOs(rest.slice(1));
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
      runCmd(process.execPath, ["x", "@modelcontextprotocol/inspector", "--config", ".cursor/mcp.json", "--server", "repo"], {
        cwd: this.root,
        env: { ...process.env, HOST: host },
        ...daemonBudgetOpts(),
      });
      return;
    }
    runCmd(process.execPath, ["x", "@modelcontextprotocol/inspector"], { cwd: this.root, ...daemonBudgetOpts() });
  }

  /** 🌉️ Runs the `semio-os` MCP gateway (`semio-framework-os-mcp`) over stdio, which is what `.mcp.json` launches.
   * Streamable HTTP is the dashboard tool `tool:workspace/os-mcp-http` (build task `@semio-tech/framework-os-mcp-rs:dev`).
   * Extra argv passes straight through to the binary (`--folder`, `--hub`, `--principal`, `--scopes`, `--auto-approve`, …). */
  private runMcpOs(extra: string[]): void {
    runCmd(ensureMcpBinary(this.root), ["stdio", ...extra], { cwd: this.root, ...daemonBudgetOpts() });
  }

  private runMcpStdioRepo(slugs: string[]): void {
    if (slugs.length > 1) {
      console.error("[dev] `dev mcp stdio` accepts one repo client profile.");
      process.exit(1);
    }
    const profile = (slugs[0] ?? "client").trim().toLowerCase();
    const bin = requireRepoMcpBinary(this.root);
    runCmd(bin, [], { cwd: this.root, ...daemonBudgetOpts({ GOWORK: join(this.root, "go.work"), [REPO_MCP_PROFILE_ENV]: profile }) });
  }
}
//#endregion 🔖️DevScript
