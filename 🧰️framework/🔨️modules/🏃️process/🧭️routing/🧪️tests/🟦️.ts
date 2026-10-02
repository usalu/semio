import { spawnSync } from "node:child_process";
import { dirname, join, resolve } from "node:path";
import { existsSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import findUp from "find-up";
import { findWorkspaceRoot } from "../🟦️.ts";
import { runScriptMain } from "../🚪️entrypoint/🟦️.ts";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { Command } from "commander";
import { BundleScript, ScriptRouter } from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import corpus from "../🧫️fixtures/🔣️.json";

expect(new Ajv({ strict: true }).compile(schema)(corpus)).toBe(true);

for (const row of corpus.cases) test(row.id, async () => {
  const execute = async (oracle: boolean): Promise<typeof row.expected> => {
    const trace: string[] = [], router = new ScriptRouter("/future-owner", "/neutral-workspace");
    const commander = new Command().exitOverride().configureOutput({ writeErr() {}, writeOut() {} });
    for (const owner of row.commands) {
      class OwnedCommand extends BundleScript {
        run(args: string[]): void {
          expect(this.root).toBe("/future-owner");
          expect(this.repoRoot).toBe("/neutral-workspace");
          trace.push(`run:${owner.id}:${args.join(",")}`);
        }
      }
      const load = (): Promise<typeof OwnedCommand> => Promise.resolve().then(() => {
        trace.push(`load:${owner.id}`);
        if (owner.refuse) throw Error("absent owner");
        return OwnedCommand;
      });
      let oracleLoad: Promise<typeof OwnedCommand> | undefined;
      if (oracle) commander.command(owner.id).argument("[args...]").action(async (args: string[]) => {
        const Selected = owner.mode === "lazy" ? await (oracleLoad ??= load()) : OwnedCommand;
        await new Selected("/future-owner", "/neutral-workspace").run(args);
      });
      else if (owner.mode === "lazy") router.registerLazy(owner.id, load);
      else router.register(owner.id, OwnedCommand);
    }
    try {
      const run = (args: string[]): Promise<unknown> => oracle ? commander.parseAsync(args, { from: "user" }) : router.run(args);
      if (row.parallel) await Promise.all(row.requests.map(run));
      else for (const args of row.requests) await run(args);
      return { trace, rejected: false };
    } catch { return { trace, rejected: true }; }
  };
  expect(await execute(true)).toEqual(row.expected);
  expect(await execute(false)).toEqual(row.expected);
});

test("ambiguous registration and absent commands refuse without process termination", async () => {
  const router = new ScriptRouter("/future-owner", "/neutral-workspace");
  class OwnedCommand extends BundleScript { run(): void {} }
  router.register("future", OwnedCommand);
  expect(() => router.register("future", OwnedCommand)).toThrow("already registered");
  expect(() => router.registerLazy("future", async () => OwnedCommand)).toThrow("already registered");
  await expect(router.run(["absent"])).rejects.toThrow("unknown command");
  await expect(router.run([])).rejects.toThrow("usage:");
});


test("keeps routing loadable without any specific product", async () => {
  const { build } = await import("esbuild");
  const source = `import { BundleScript, ScriptRouter } from ${JSON.stringify(resolve(import.meta.dir, "../🟦️.ts"))};
import { runScriptMain } from ${JSON.stringify(resolve(import.meta.dir, "../🚪️entrypoint/🟦️.ts"))};
const corpus = ${JSON.stringify(corpus)};
const results = [];
for (const row of corpus.cases) {
  const trace = [], router = new ScriptRouter("/future-owner", "/neutral-workspace");
  for (const owner of row.commands) {
    class OwnedCommand extends BundleScript { run(args) { trace.push("run:" + owner.id + ":" + args.join(",")); } }
    if (owner.mode === "lazy") router.registerLazy(owner.id, async () => { trace.push("load:" + owner.id); if (owner.refuse) throw Error("absent owner"); return OwnedCommand; });
    else router.register(owner.id, OwnedCommand);
  }
  let rejected = false;
  try { if (row.parallel) await Promise.all(row.requests.map(args => router.run(args))); else for (const args of row.requests) await router.run(args); } catch { rejected = true; }
  results.push({ id: row.id, actual: { trace, rejected } });
}
for (const row of corpus.mainCases) {
  const trace = [], router = new ScriptRouter("/future-owner", "/neutral-workspace");
  for (const id of row.commands) { class OwnedCommand extends BundleScript { run(args) { trace.push("run:" + id + ":" + args.join(",")); } } router.register(id, OwnedCommand); }
  const beforeDispatch = row.interception === "none" ? undefined : async argv => { trace.push("intercept:" + argv.join(",")); if (row.interception === "refuse") throw Error("contribution refused"); return row.interception === "consume"; };
  let rejected = false;
  try { await runScriptMain(router, { argv: row.argv, defaultCommand: row.defaultCommand ?? undefined, beforeDispatch }); } catch { rejected = true; }
  results.push({ id: row.id, actual: { trace, rejected } });
}
console.log(JSON.stringify(results));`;
  const result = await build({ stdin: { contents: source, resolveDir: process.cwd(), sourcefile: "routing-product-refusal.ts" }, bundle: true, platform: "node", format: "esm", write: false, logLevel: "silent", plugins: [{ name: "refuse-specific-products", setup(builder) { builder.onLoad({ filter: /./ }, (args) => { if (args.path.replaceAll("\\", "/").includes("/🧰️framework/🛍️products/")) throw Error(`general routing loads specific product ${args.path}`); }); } }] });
  const child = spawnSync("node", ["--input-type=module", "-e", result.outputFiles[0]!.text], { encoding: "utf8" });
  expect(child.status, child.stderr).toBe(0);
  expect(JSON.parse(child.stdout)).toEqual([...corpus.cases, ...corpus.mainCases].map(row => ({ id: row.id, actual: row.expected })));
});

for (const row of corpus.mainCases) test(row.id, async () => {
  const execute = async (oracle: boolean): Promise<typeof row.expected> => {
    const trace: string[] = [], router = new ScriptRouter("/future-owner", "/neutral-workspace"), commander = new Command().exitOverride().configureOutput({ writeErr() {}, writeOut() {} });
    for (const id of row.commands) {
      class OwnedCommand extends BundleScript { run(args: string[]): void { trace.push(`run:${id}:${args.join(",")}`); } }
      router.register(id, OwnedCommand);
      commander.command(id).argument("[args...]").action((args: string[]) => { trace.push(`run:${id}:${args.join(",")}`); });
    }
    const beforeDispatch = row.interception === "none" ? undefined : async (argv: readonly string[]): Promise<boolean> => {
      trace.push(`intercept:${argv.join(",")}`);
      if (row.interception === "refuse") throw Error("contribution refused");
      return row.interception === "consume";
    };
    try {
      if (oracle) {
        if (await beforeDispatch?.(row.argv)) return { trace, rejected: false };
        const argv = row.argv.length ? row.argv : row.defaultCommand ? [row.defaultCommand] : [];
        if (!row.commands.length || !argv.length) throw Error("missing command");
        await commander.parseAsync(argv, { from: "user" });
      } else await runScriptMain(router, { argv: row.argv, defaultCommand: row.defaultCommand ?? undefined, beforeDispatch });
      return { trace, rejected: false };
    } catch { return { trace, rejected: true }; }
  };
  expect(await execute(true)).toEqual(row.expected);
  expect(await execute(false)).toEqual(row.expected);
});

for (const row of corpus.workspaceCases) test(row.id, () => {
  const root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "process-root-"));
  try {
    mkdirSync(resolve(root, row.start), { recursive: true });
    for (const marker of row.markers) {
      mkdirSync(resolve(root, marker.directory), { recursive: true });
      for (const filename of marker.files) writeFileSync(resolve(root, marker.directory, filename), "{}");
    }
    const oracle = findUp.sync((directory: string) => existsSync(join(directory, "nx.json")) && existsSync(join(directory, "package.json")) ? "nx.json" : directory === root ? findUp.stop : undefined, { cwd: resolve(root, row.start) });
    const expected = row.expected === null ? undefined : resolve(root, row.expected);
    expect(oracle === undefined ? undefined : dirname(oracle)).toBe(expected);
    if (expected === undefined) expect(() => findWorkspaceRoot(resolve(root, row.start), { stopAt: root })).toThrow("workspace root");
    else expect(findWorkspaceRoot(resolve(root, row.start), { stopAt: root })).toBe(expected);
  } finally { rmSync(root, { recursive: true, force: true }); }
});
