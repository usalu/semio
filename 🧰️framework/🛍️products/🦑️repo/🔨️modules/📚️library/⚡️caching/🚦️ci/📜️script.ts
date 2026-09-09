#!/usr/bin/env bun
import { appendFileSync, readFileSync, statSync } from "node:fs";
import { Script, ScriptRouter } from "../../🏃️process/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import { runTool } from "../🚀️bootstrap/📦️dependencies/📜️script.ts";
import { githubJsonTransport } from "./🐙️github/🟦️.ts";
import { resolveCiValidation } from "./🧭️baseline/🏃️resolve/🟦️.ts";

/** 🚦️ Publishes an uncached CI boundary derived from the actual checkout and successful workflow ancestry. */
export class BaselineScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length > 1 || args.some(arg => arg !== "--full")) throw new Error("CI baseline accepts only --full");
    const controller = new AbortController();
    let cancelled: NodeJS.Signals | undefined;
    const stop = (signal: NodeJS.Signals): void => { cancelled ??= signal; controller.abort(); };
    const interrupt = (): void => stop("SIGINT"), terminate = (): void => stop("SIGTERM");
    process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
    const progress = setInterval(() => console.error("Resolving successful CI ancestry…"), 10000);
    try {
      const head = (await runTool("git", ["rev-parse", "--verify", "HEAD^{commit}"], this.root, controller.signal, true)).trim();
      let event: unknown = null;
      if (process.env.GITHUB_ACTIONS === "true") {
        const path = process.env.GITHUB_EVENT_PATH;
        if (!path || statSync(path).size > 8 * 1024 * 1024) throw new Error("Invalid GitHub event file");
        event = JSON.parse(readFileSync(path, "utf8"));
      }
      const result = await resolveCiValidation({ environment: process.env, event, head, full: args.includes("--full") }, githubJsonTransport({ token: process.env.GITHUB_TOKEN }), async (base, head) => {
        try { return (await runTool("git", ["merge-base", base, head], this.root, controller.signal, true)).trim() === base; }
        catch { controller.signal.throwIfAborted(); return false; }
      }, controller.signal);
      controller.signal.throwIfAborted();
      if (process.env.GITHUB_ACTIONS === "true" && process.env.GITHUB_OUTPUT) appendFileSync(process.env.GITHUB_OUTPUT, `mode=${result.baseline.mode}\nbase=${result.baseline.base ?? ""}\nhead=${result.baseline.head}\n`);
      console.log(JSON.stringify(result));
    } catch (error) { if (!cancelled) throw error; process.exitCode = cancelled === "SIGINT" ? 130 : 143; }
    finally { clearInterval(progress); process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); }
  }
}

if (import.meta.main) await new ScriptRouter(getWorkspaceRoot()).register("baseline", BaselineScript).run(process.argv.slice(2));
