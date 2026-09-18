/** 🏃️ Ambient declarations for the Bun runtime this repository's tooling runs on.
 *
 * Every `📜️script.ts` router, every `bun nx run` target and every `bun tsc` invocation executes under Bun, so
 * `import.meta.dir`, the `Bun` global and the `bun:*` builtin modules are real, always-present runtime APIs here —
 * not optional extras. They are nevertheless invisible to `tsc` because `@types/bun` is **not installed**: a plain
 * `bun add -d @types/bun` currently fails repo-wide, because the root `package.json` `workspaces` array does not
 * list `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript` or its `🎯️targets/⚛️react` sibling, while
 * `♻️mit-bestand/🎤️präsentation/…` depends on both as `workspace:*` (`error: Workspace dependency
 * "@semio-tech/presentation" not found`). Until those two workspace members are registered and `@types/bun` can be
 * installed (after which this file should be deleted in favour of `"types": ["bun"]`), the surface actually used by
 * repo tooling is declared here, once, instead of being re-declared per file (see the same `declare global
 * { interface ImportMeta { readonly dir: string } }` workaround already inlined in
 * `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts` and
 * `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📜️script.ts`).
 *
 * Declared here only: members with a live call site under `🦑️repo/🔨️modules/📚️library/**`, the root `📜️script.ts`
 * or `🌎️hub/**`. Anything else stays undeclared on purpose, so this file cannot silently drift into a second,
 * hand-maintained copy of `@types/bun`.
 */

/** 📁️ Bun's absolute directory of the current module — the runtime counterpart of `dirname(fileURLToPath(import.meta.url))`. */
interface ImportMeta {
  readonly dir: string;
}

/** 🧵️ The completed result of `Bun.spawnSync`. */
interface BunSyncSubprocess {
  readonly exitCode: number;
  readonly success: boolean;
  readonly stdout: Buffer;
  readonly stderr: Buffer;
}

/** 🧵️ The live handle returned by `Bun.spawn`. */
interface BunSubprocess {
  readonly exited: Promise<number>;
  readonly pid: number;
  kill(code?: number): void;
}

/** 🗂️ A single compiled glob pattern (`new Bun.Glob(pattern)`). */
interface BunGlob {
  scanSync(options?: { readonly cwd?: string; readonly onlyFiles?: boolean; readonly dot?: boolean }): Iterable<string>;
  scan(options?: { readonly cwd?: string; readonly onlyFiles?: boolean; readonly dot?: boolean }): AsyncIterable<string>;
  match(path: string): boolean;
}

/** 📄️ A lazily-read file handle; it is a `Blob`, so `new Response(Bun.file(path))` streams it. */
interface BunFile extends Blob {
  exists(): Promise<boolean>;
}

/** 🌐️ The listening server returned by `Bun.serve`. */
interface BunServer {
  readonly port: number;
  readonly hostname: string;
  stop(closeActiveConnections?: boolean): void;
}

declare namespace Bun {
  /** 🗂️ Compiled glob pattern. */
  const Glob: new (pattern: string) => BunGlob;
  /** 🧾️ TOML document parser — a TOML document is always a table, hence the keyed result. */
  const TOML: { parse(input: string): Record<string, unknown> };
  /** 🧾️ JSON-with-comments parser (`.vscode/*.json`, `bun.lock`, `*.jsonc`). */
  const JSONC: { parse(input: string): unknown };
  /** 🔍️ Resolves an executable on `PATH`, or `null` when it is absent. */
  function which(command: string, options?: { readonly PATH?: string; readonly cwd?: string }): string | null;
  /** 🧵️ Runs a command to completion and buffers its output. */
  function spawnSync(command: readonly string[], options?: { readonly cwd?: string; readonly env?: Record<string, string | undefined>; readonly stdin?: unknown }): BunSyncSubprocess;
  /** 🧵️ Starts a command and returns immediately. */
  function spawn(command: readonly string[], options?: { readonly cwd?: string; readonly env?: Record<string, string | undefined>; readonly stdout?: unknown; readonly stderr?: unknown }): BunSubprocess;
  /** 📄️ Opens a file lazily. */
  function file(path: string | URL): BunFile;
  /** 🌐️ Binds an HTTP server. */
  function serve(options: { readonly hostname?: string; readonly port?: number; fetch(request: Request): Response | Promise<Response> }): BunServer;
}

/** 🗃️ Bun's embedded SQLite driver — the Bun-side twin of `node:sqlite`'s `DatabaseSync`. */
declare module "bun:sqlite" {
  /** 🔎️ A prepared statement. */
  export interface Statement {
    get(...values: unknown[]): unknown;
    all(...values: unknown[]): unknown[];
    run(...values: unknown[]): void;
  }
  /** 🗃️ An open database handle. */
  export class Database {
    constructor(path: string, options?: { readonly create?: boolean; readonly readonly?: boolean });
    exec(sql: string, ...values: unknown[]): void;
    query(sql: string): Statement;
    prepare(sql: string): Statement;
    close(): void;
  }
}
