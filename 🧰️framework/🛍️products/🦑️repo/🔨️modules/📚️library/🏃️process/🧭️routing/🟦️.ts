import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";

/** 🧭️Bundle command; `run` receives argv segments after the subcommand (e.g. `dev mcp` → `["mcp"]`). */
export abstract class Script {
  protected readonly root: string;
  protected readonly repoRoot: string;

  constructor(root: string, repoRoot: string) {
    this.root = root;
    this.repoRoot = repoRoot;
  }
  abstract run(segments: string[]): void | Promise<void>;
}

/** 📦️Bundle-scoped command with `root` at the package directory. */
export abstract class BundleScript extends Script {
  constructor(bundleRoot: string, repoRoot?: string) {
    super(bundleRoot, repoRoot ?? findRepoRoot(bundleRoot));
  }
}
export type ScriptCommand = new (root: string, repoRoot: string) => Script;

/** 🧭️Declarative subcommand registry for a single `script.ts`. */
export class ScriptRouter {
  private readonly commands = new Map<string, ScriptCommand>();
  readonly bundleRoot: string;
  readonly repoRoot: string;

  constructor(bundleRoot: string, repoRoot: string = findRepoRoot(bundleRoot)) {
    this.bundleRoot = bundleRoot;
    this.repoRoot = repoRoot;
  }

  /** 📌️Registers a subcommand implemented by a `Script` subclass. */
  register(name: string, Command: ScriptCommand): this {
    this.commands.set(name, Command);
    return this;
  }

  /** 📋️Human-readable usage line for this router. */
  usage(): string {
    const names = [...this.commands.keys()];
    if (names.length === 0) return "bun ./📜️script.ts policy";
    return `bun ./📜️script.ts <${names.join("|")}> [args…]`;
  }

  /** 📊️Whether any subcommands are registered (policy-only bundles may have none). */
  hasCommands(): boolean {
    return this.commands.size > 0;
  }

  /** ▶️Dispatches `segments[0]` to a registered command class. */
  async run(segments: string[]): Promise<void> {
    const name = segments[0];
    if (!name) {
      console.error(`usage: ${this.usage()}`);
      process.exit(1);
    }
    const Command = this.commands.get(name);
    if (!Command) {
      console.error(`unknown command ${JSON.stringify(name)}`);
      console.error(`usage: ${this.usage()}`);
      process.exit(1);
    }
    await Promise.resolve(new Command(this.bundleRoot, this.repoRoot).run(segments.slice(1)));
  }
}

/** 📁️Walks parents until the monorepo root (`nx.json` + workspace `package.json`). */
export function findRepoRoot(start: string): string {
  let dir = start?.trim() ? start : getWorkspaceRoot();
  for (let i = 0; i < 32; i++) {
    if (existsSync(join(dir, "nx.json")) && existsSync(join(dir, "package.json"))) return dir;
    const parent = dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return getWorkspaceRoot();
}
