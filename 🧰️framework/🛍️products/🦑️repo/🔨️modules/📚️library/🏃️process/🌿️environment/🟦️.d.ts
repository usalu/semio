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
 * Declared here only: members with a live call site under `🦑️repo/🔨️modules/📚️library/**`, `🧰️framework/🔨️modules/**`,
 * the root `📜️script.ts` or `🌎️hub/**`. Anything else stays undeclared on purpose, so this file cannot silently
 * drift into a second, hand-maintained copy of `@types/bun`. Each program that needs these globals lists this file
 * in its own tsconfig `include` (`🧰️framework/📦️packages/🟦️typescript/tsconfig.json` does).
 */

/** 📁️ Bun's absolute directory and absolute file path of the current module — the runtime counterparts of
 * `dirname(fileURLToPath(import.meta.url))` and `fileURLToPath(import.meta.url)`. */
interface ImportMeta {
  readonly dir: string;
  readonly path: string;
}

/** 🚰️ How one standard stream of a spawned process is wired. `"pipe"` is the only mode whose handle is read here. */
type BunStdioMode = "pipe" | "inherit" | "ignore";

/** 🧵️ The options shared by `Bun.spawn` and `Bun.spawnSync`.
 *
 * 🩸️ `stdin` also accepts an in-memory body, which Bun writes to the child and closes — the form
 * `git check-ignore --stdin` is driven with. */
interface BunSpawnOptions {
  readonly cwd?: string;
  readonly env?: Record<string, string | undefined>;
  readonly stdin?: BunStdioMode | ArrayBufferView | ArrayBuffer | Blob;
  readonly stdout?: BunStdioMode;
  readonly stderr?: BunStdioMode;
  readonly timeout?: number;
}

/** 🧵️ The completed result of `Bun.spawnSync`. */
interface BunSyncSubprocess {
  readonly exitCode: number;
  readonly signal: NodeJS.Signals | null;
  readonly success: boolean;
  readonly stdout: Buffer;
  readonly stderr: Buffer;
}

/** 🧵️ The live handle returned by `Bun.spawn`.
 *
 * 🩸️ `stdout`/`stderr`/`stdin` are declared non-nullable because every call site that touches them passes
 * `"pipe"`; the `"ignore"`/`"inherit"` call sites never read the handle. Narrowing them to the union Bun
 * actually returns would demand a non-null assertion at ~120 live sites that cannot be wrong. */
interface BunSubprocess {
  readonly exited: Promise<number>;
  readonly exitCode: number | null;
  readonly pid: number;
  readonly stdin: BunFileSink;
  readonly stdout: BunReadableStream;
  readonly stderr: BunReadableStream;
  kill(signal?: number | NodeJS.Signals): void;
}

/** 🚿️ Every readable stream this runtime hands out — a spawned child's pipe and a `fetch` response body alike — is
 * async iterable under both Bun and Node, which the DOM `ReadableStream` declaration does not state. */
interface ReadableStream<R = any> {
  [Symbol.asyncIterator](): AsyncIterableIterator<R>;
}

/** 🚿️ A spawned child's `"pipe"` stream, whose chunks are always bytes. */
interface BunReadableStream extends ReadableStream<Uint8Array> {
  [Symbol.asyncIterator](): AsyncIterableIterator<Uint8Array>;
}

/** 🚰️ The incremental writer `Bun.spawn` hands back for a `"pipe"` stdin: a sink, not a `WritableStream`. */
interface BunFileSink {
  write(chunk: string | ArrayBufferView | ArrayBuffer): number;
  flush(): number | Promise<number>;
  end(error?: Error): number | Promise<number>;
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

/** 🔌️ One accepted WebSocket connection, carrying the per-connection record handed to `upgrade`. */
interface BunServerWebSocket<Data> {
  readonly data: Data;
  readonly readyState: number;
  send(message: string | ArrayBufferView | ArrayBuffer): number;
  close(code?: number, reason?: string): void;
}

/** 🔌️ The server-side WebSocket handlers `Bun.serve` dispatches to after a successful `upgrade`. */
interface BunWebSocketHandler<Data> {
  open?(socket: BunServerWebSocket<Data>): void;
  message(socket: BunServerWebSocket<Data>, message: string | Buffer): void;
  close?(socket: BunServerWebSocket<Data>, code: number, reason: string): void;
}

/** 🌐️ The listening server returned by `Bun.serve`; `Data` is the per-socket record `upgrade` attaches. */
interface BunServer<Data = undefined> {
  readonly port: number;
  readonly hostname: string;
  readonly url: URL;
  upgrade(request: Request, options?: { readonly data?: Data; readonly headers?: Readonly<Record<string, string>> }): boolean;
  requestIP(request: Request): { readonly address: string; readonly port: number; readonly family: string } | null;
  stop(closeActiveConnections?: boolean): void;
}

/** 🔐️ An incremental digest (`new Bun.CryptoHasher("sha256")`), chained `update(…).digest("hex")`. */
interface BunCryptoHasher {
  update(data: string | ArrayBufferView | ArrayBuffer): BunCryptoHasher;
  digest(encoding: "hex" | "base64"): string;
  digest(): Buffer;
}

/** 🔎️ The import/export census `Bun.Transpiler.prototype.scan` returns. */
interface BunTranspilerScan {
  readonly imports: readonly { readonly path: string; readonly kind: string }[];
  readonly exports: readonly string[];
}

/** 🔧️ A TypeScript-to-JavaScript transpiler instance (`new Bun.Transpiler({ loader: "ts" })`). */
interface BunTranspiler {
  transformSync(code: string): string;
  transform(code: string): Promise<string>;
  scan(code: string): BunTranspilerScan;
}

/** 📦️ One emitted artifact of `Bun.build`. */
interface BunBuildArtifact extends Blob {
  readonly path: string;
  readonly kind: string;
}

/** 🪵️ One bundler diagnostic `Bun.build` reports in `logs`. */
interface BunBuildMessage {
  readonly message: string;
  readonly name: string;
  readonly level?: string;
}

/** 🗺️ The `metafile: true` census of what `Bun.build` read and wrote, keyed by build-cwd-relative path. */
interface BunBuildMetafile {
  readonly inputs: Readonly<Record<string, { readonly bytes: number }>>;
  readonly outputs: Readonly<Record<string, { readonly bytes: number }>>;
}

/** 📦️ The result of `Bun.build`. */
interface BunBuildOutput {
  readonly success: boolean;
  readonly outputs: readonly BunBuildArtifact[];
  readonly logs: readonly BunBuildMessage[];
  readonly metafile?: BunBuildMetafile;
}

/** 📄️ What a `Bun.build` `onLoad` hook hands back: source text or the exact bytes it snapshotted. */
interface BunBuildLoadResult {
  readonly contents: string | Uint8Array;
  readonly loader?: string;
}

/** 🧩️ The bundler hook surface a `Bun.build` plugin registers against. */
interface BunBuildPluginBuilder {
  onLoad(constraints: { readonly filter: RegExp; readonly namespace?: string }, callback: (args: { readonly path: string; readonly namespace: string }) => undefined | Promise<undefined> | BunBuildLoadResult | Promise<BunBuildLoadResult>): void;
  onResolve(constraints: { readonly filter: RegExp; readonly namespace?: string }, callback: (args: { readonly path: string; readonly importer: string; readonly kind: "entry-point" | "import-statement" | "require-call" | "dynamic-import" | "require-resolve" | "import-rule" | "url-token" | "internal" }) => undefined | { readonly path: string; readonly namespace?: string }): void;
}

/** 🧩️ A `Bun.build` plugin. */
interface BunBuildPlugin {
  readonly name: string;
  setup(build: BunBuildPluginBuilder): void | Promise<void>;
}

/** 📦️ The `Bun.build` configuration surface used in this repository. */
interface BunBuildConfig {
  readonly entrypoints: readonly string[];
  readonly root?: string;
  readonly outdir?: string;
  readonly naming?: string;
  readonly target?: "browser" | "bun" | "node";
  readonly format?: "esm" | "cjs" | "iife";
  readonly minify?: boolean;
  readonly splitting?: boolean;
  readonly metafile?: boolean;
  readonly write?: boolean;
  readonly throw?: boolean;
  readonly sourcemap?: "none" | "linked" | "inline" | "external";
  readonly external?: readonly string[];
  readonly define?: Record<string, string>;
  readonly plugins?: readonly BunBuildPlugin[];
}

declare namespace Bun {
  /** 🗂️ Compiled glob pattern. */
  const Glob: new (pattern: string) => BunGlob;
  /** 🧾️ TOML document parser — a TOML document is always a table, hence the keyed result. */
  const TOML: { parse(input: string): Record<string, unknown> };
  /** 🧾️ JSON-with-comments parser (`.vscode/*.json`, `bun.lock`, `*.jsonc`). */
  const JSONC: { parse(input: string): unknown };
  /** 🔐️ Incremental digest constructor. */
  const CryptoHasher: new (algorithm: "sha1" | "sha256" | "sha512" | "blake2b256") => BunCryptoHasher;
  /** 🔧️ Transpiler constructor. */
  const Transpiler: new (options: { readonly loader: "ts" | "tsx" | "js" | "jsx"; readonly target?: "browser" | "bun" | "node" }) => BunTranspiler;
  /** 🔢️ The running Bun release. */
  const version: string;
  /** ⌨️ The process's standard input, as a lazily-read file handle. */
  const stdin: BunFile;
  /** 🔍️ Resolves an executable on `PATH`, or `null` when it is absent. */
  function which(command: string, options?: { readonly PATH?: string; readonly cwd?: string }): string | null;
  /** 🧵️ Runs a command to completion and buffers its output. */
  function spawnSync(command: readonly string[], options?: BunSpawnOptions): BunSyncSubprocess;
  /** 🧵️ Starts a command and returns immediately. */
  function spawn(command: readonly string[], options?: BunSpawnOptions): BunSubprocess;
  /** 📄️ Opens a file lazily. */
  function file(path: string | URL): BunFile;
  /** 📝️ Writes a whole file in one call. */
  function write(destination: string | URL | BunFile, input: string | ArrayBufferView | ArrayBuffer | Blob | Response): Promise<number>;
  /** 🌐️ Binds an HTTP server. */
  function serve<Data = undefined>(options: { readonly hostname?: string; readonly port?: number; fetch(request: Request, server: BunServer<Data>): Response | undefined | Promise<Response | undefined>; readonly websocket?: BunWebSocketHandler<Data> }): BunServer<Data>;
  /** ⏱️ Resolves after the given number of milliseconds. */
  function sleep(milliseconds: number): Promise<void>;
  /** 🧭️ Resolves a module specifier against a directory, the way Bun's own loader does. */
  function resolveSync(specifier: string, parent: string): string;
  /** 📦️ Bundles one or more entrypoints. */
  function build(config: BunBuildConfig): Promise<BunBuildOutput>;
  /** 🗑️ Runs the garbage collector; `force` performs a synchronous full collection. */
  function gc(force?: boolean): void;
}

/** 🗃️ Bun's embedded SQLite driver — the Bun-side twin of `node:sqlite`'s `DatabaseSync`. */
declare module "bun:sqlite" {
  /** 🔎️ A prepared statement. */
  export interface Statement {
    get(...values: unknown[]): unknown;
    all(...values: unknown[]): unknown[];
    values(...values: unknown[]): unknown[][];
    run(...values: unknown[]): { changes: number; lastInsertRowid: number | bigint };
  }
  /** 🗃️ An open database handle. */
  export class Database {
    constructor(path: string, options?: { readonly create?: boolean; readonly readonly?: boolean });
    exec(sql: string, ...values: unknown[]): { changes: number; lastInsertRowid: number | bigint };
    run(sql: string, ...values: unknown[]): { changes: number; lastInsertRowid: number | bigint };
    query(sql: string): Statement;
    prepare(sql: string): Statement;
    close(): void;
  }
}

/** ✅️ One asserted value, and the matchers this repository actually calls on it. */
interface BunExpectation {
  readonly not: BunExpectation;
  readonly rejects: BunExpectation;
  readonly resolves: BunExpectation;
  toBe(expected: unknown): void;
  toEqual(expected: unknown): void;
  toMatchObject(expected: object): void;
  toContain(expected: unknown): void;
  toContainEqual(expected: unknown): void;
  toHaveLength(expected: number): void;
  toHaveProperty(key: string, value?: unknown): void;
  toThrow(expected?: string | RegExp | Error | (new (...args: never[]) => Error)): void;
  toBeCloseTo(expected: number, precision?: number): void;
  toMatch(expected: string | RegExp): void;
  toStartWith(expected: string): void;
  toBeDefined(): void;
  toBeUndefined(): void;
  toBeNull(): void;
  toBeTruthy(): void;
  toBeFunction(): void;
  toBeInstanceOf(expected: Function): void;
  toBeGreaterThan(expected: number | bigint): void;
  toBeGreaterThanOrEqual(expected: number | bigint): void;
  toBeLessThan(expected: number | bigint): void;
  toBeLessThanOrEqual(expected: number | bigint): void;
}

/** 🧪️ Bun's built-in test runner — the runner every `📚️library` suite is launched under (`bun test <file>`). */
declare module "bun:test" {
  /** ⏱️ The per-case budget a suite may declare instead of passing a bare millisecond argument. */
  export interface TestOptions {
    readonly timeout?: number;
    readonly retry?: number;
    readonly repeats?: number;
  }
  /** 🧪️ One test case; the budget is either a trailing millisecond argument or a leading options record. */
  export interface TestFunction {
    (label: string, options: TestOptions, body: () => void | Promise<void>): void;
    (label: string, body: () => void | Promise<void>, timeoutMs?: number): void;
    readonly concurrent: TestFunction;
    readonly skip: TestFunction;
    readonly if: (condition: boolean) => TestFunction;
    readonly todo: (label: string, body?: () => void | Promise<void>) => void;
  }
  /** 🗂️ One suite; `describe.if(condition)` registers its cases only when the condition holds (test levels). */
  export interface DescribeFunction {
    (label: string, body: () => void | Promise<void>): void;
    readonly if: (condition: boolean) => DescribeFunction;
  }
  /** 🧪️ The asymmetric matchers and the entry point in one callable. */
  export interface ExpectFunction {
    (actual: unknown, label?: string): BunExpectation;
    readonly any: (constructor: unknown) => unknown;
    readonly arrayContaining: (expected: readonly unknown[]) => unknown;
    readonly objectContaining: (expected: object) => unknown;
    readonly stringContaining: (expected: string) => unknown;
    readonly stringMatching: (expected: string | RegExp) => unknown;
  }
  /** 🧪️ A replaced function that records the calls made through it. */
  export interface Mock<T extends (...args: never[]) => unknown> {
    (...args: Parameters<T>): ReturnType<T>;
    readonly mock: { readonly calls: readonly Parameters<T>[]; readonly results: readonly { readonly type: string; readonly value: unknown }[] };
    mockImplementation(implementation: T): Mock<T>;
    mockRestore(): void;
  }
  export const test: TestFunction;
  /** 🧪️ `it` is Bun's alias for `test`; both spellings are live in `🧰️framework/🔨️modules/**` suites. */
  export const it: TestFunction;
  export const expect: ExpectFunction;
  export const describe: DescribeFunction;
  export function afterAll(body: () => void | Promise<void>): void;
  export function mock<T extends (...args: never[]) => unknown>(implementation: T): Mock<T>;
  /** 🧪️ `mock.module` replaces a resolved module for the rest of the suite; the factory returns the substitute namespace. */
  export namespace mock {
    function module(specifier: string, factory: () => unknown): void;
  }
  export function spyOn<T extends object, K extends keyof T>(target: T, key: K): T[K] extends (...args: never[]) => unknown ? Mock<T[K]> : never;
}
