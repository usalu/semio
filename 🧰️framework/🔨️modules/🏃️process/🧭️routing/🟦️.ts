import { lstatSync } from "node:fs";
import { dirname, isAbsolute, join, parse, relative, resolve, sep } from "node:path";
import { advanceScriptInvocation, checkScriptInvocation, type ScriptInvocation } from "./📥️invocation/🟦️.ts";
export { readScriptPolicy, checkScriptInvocation, scriptInvocationBudget, type ScriptPolicy, type ScriptControl, type ScriptInvocation, type ScriptProgress } from "./📥️invocation/🟦️.ts";

/** 🧭️Bundle command; `run` receives argv segments after the subcommand (e.g. `dev mcp` → `["mcp"]`). */
export abstract class Script<Capabilities extends object = object> {
  protected readonly root: string;
  protected readonly repoRoot: string;

  constructor(root: string, repoRoot: string, protected readonly invocation: ScriptInvocation<Capabilities>) {
    checkScriptInvocation(invocation);
    this.root = root;
    this.repoRoot = repoRoot;
  }
  abstract run(segments: string[]): void | Promise<void>;
}

/** 📦️Bundle-scoped command with `root` at the package directory. */
export abstract class BundleScript<Capabilities extends object = object> extends Script<Capabilities> {
  constructor(bundleRoot: string, repoRoot: string, invocation: ScriptInvocation<Capabilities>) {
    super(bundleRoot, repoRoot, invocation);
  }
}
export type ScriptCommand<Capabilities extends object = object> = new (root: string, repoRoot: string, invocation: ScriptInvocation<Capabilities>) => Script<Capabilities>;
export type ScriptCommandLoader<Capabilities extends object = object> = (invocation: ScriptInvocation<Capabilities>) => Promise<ScriptCommand<Capabilities>>;

/** 🧭️Declarative subcommand registry for a single `script.ts`. */
export class ScriptRouter<Capabilities extends object = object> {
  private readonly commands = new Map<string, (invocation: ScriptInvocation<Capabilities>) => ScriptCommand<Capabilities> | Promise<ScriptCommand<Capabilities>>>();
  readonly bundleRoot: string;
  readonly repoRoot: string;

  constructor(bundleRoot: string, repoRoot: string = findWorkspaceRoot(bundleRoot)) {
    this.bundleRoot = bundleRoot;
    this.repoRoot = repoRoot;
  }

  /** 📌️Registers a subcommand implemented by a `Script` subclass. */
  register(name: string, Command: ScriptCommand<Capabilities>): this {
    this.add(name, () => Command);
    return this;
  }

  /** 💤️Loads only the selected command owner, sharing one load across concurrent dispatches. */
  registerLazy(name: string, load: ScriptCommandLoader<Capabilities>): this {
    let command: Promise<ScriptCommand<Capabilities>> | undefined;
    this.add(name, invocation => command ??= Promise.resolve().then(() => load(invocation)));
    return this;
  }

  private add(name: string, load: (invocation: ScriptInvocation<Capabilities>) => ScriptCommand<Capabilities> | Promise<ScriptCommand<Capabilities>>): void {
    if (!name || name.trim() !== name) throw Error("Command name must be nonempty and trimmed");
    if (this.commands.has(name)) throw Error(`Command ${JSON.stringify(name)} is already registered`);
    this.commands.set(name, load);
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
  async run(segments: string[], invocation: ScriptInvocation<Capabilities>): Promise<void> {
    checkScriptInvocation(invocation);
    const name = segments[0];
    if (!name) throw Error(`usage: ${this.usage()}`);
    const load = this.commands.get(name);
    if (!load) throw Error(`unknown command ${JSON.stringify(name)}; usage: ${this.usage()}`);
    await advanceScriptInvocation(invocation, name, "loading");
    const Command = await load(invocation);
    await advanceScriptInvocation(invocation, name, "running");
    await Promise.resolve(new Command(this.bundleRoot, this.repoRoot, invocation).run(segments.slice(1)));
    await advanceScriptInvocation(invocation, name, "complete");
  }
}

/** 🔎️ Limits paired workspace-marker discovery to one caller-owned ancestry. */
export interface WorkspaceRootSearch {
  readonly stopAt?: string;
}

/** 📁️ Resolves the nearest paired Nx/package owner and refuses missing or escaping roots. */
export function findWorkspaceRoot(start?: string, options: WorkspaceRootSearch = {}): string {
  let directory = resolve(start?.trim() || process.env.NX_WORKSPACE_ROOT?.trim() || process.env.REPO_ROOT?.trim() || process.cwd());
  const boundary = resolve(options.stopAt ?? parse(directory).root), distance = relative(boundary, directory);
  if (isAbsolute(distance) || distance === ".." || distance.startsWith(`..${sep}`)) throw Error("workspace root boundary must contain its start directory");
  for (let depth = 0; depth < 32; depth++) {
    if (["nx.json", "package.json"].every((filename) => lstatSync(join(directory, filename), { throwIfNoEntry: false })?.isFile())) return directory;
    if (directory === boundary) break;
    const parent = dirname(directory);
    if (parent === directory) break;
    directory = parent;
  }
  throw Error("workspace root with paired Nx and package markers was not found");
}
