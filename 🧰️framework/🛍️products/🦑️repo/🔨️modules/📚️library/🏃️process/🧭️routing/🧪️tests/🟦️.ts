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
