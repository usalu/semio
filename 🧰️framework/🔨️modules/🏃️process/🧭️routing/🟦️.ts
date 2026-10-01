import { lstatSync } from "node:fs";
import { dirname, isAbsolute, join, parse, relative, resolve, sep } from "node:path";

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
    super(bundleRoot, repoRoot ?? findWorkspaceRoot(bundleRoot));
  }
}
export type ScriptCommand = new (root: string, repoRoot: string) => Script;
export type ScriptCommandLoader = () => Promise<ScriptCommand>;

/** 🧭️Declarative subcommand registry for a single `script.ts`. */
export class ScriptRouter {
  private readonly commands = new Map<string, () => ScriptCommand | Promise<ScriptCommand>>();
  readonly bundleRoot: string;
  readonly repoRoot: string;

  constructor(bundleRoot: string, repoRoot: string = findWorkspaceRoot(bundleRoot)) {
    this.bundleRoot = bundleRoot;
    this.repoRoot = repoRoot;
  }

  /** 📌️Registers a subcommand implemented by a `Script` subclass. */
  register(name: string, Command: ScriptCommand): this {
    this.add(name, () => Command);
    return this;
  }

  /** 💤️Loads only the selected command owner, sharing one load across concurrent dispatches. */
  registerLazy(name: string, load: ScriptCommandLoader): this {
    let command: Promise<ScriptCommand> | undefined;
    this.add(name, () => command ??= Promise.resolve().then(load));
    return this;
  }

  private add(name: string, load: () => ScriptCommand | Promise<ScriptCommand>): void {
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
  async run(segments: string[]): Promise<void> {
    const name = segments[0];
    if (!name) throw Error(`usage: ${this.usage()}`);
    const load = this.commands.get(name);
    if (!load) throw Error(`unknown command ${JSON.stringify(name)}; usage: ${this.usage()}`);
    const Command = await load();
    await Promise.resolve(new Command(this.bundleRoot, this.repoRoot).run(segments.slice(1)));
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
