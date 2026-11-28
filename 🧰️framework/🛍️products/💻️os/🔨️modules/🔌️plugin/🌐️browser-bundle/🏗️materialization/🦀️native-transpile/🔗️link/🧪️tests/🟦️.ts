/** 🔗️ The split link runs every corpus case of `🔣️.json`: `jco parse` (wasm-tools) is the third-party WAT encoder, V8 instantiates and runs the merged module,
 * and the refusals carry their exit code and message. The end-to-end oracle is the descriptor equality of a split and a monolithic plugin component.
 * @see https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { parse } from "@bytecodealliance/jco";
import corpus from "./🔣️.json" with { type: "json" };
import { nativeTranspileBinaryV1 } from "../../🟦️.ts";

type Exports = Record<string, unknown>;
type Call = { export: string; expect: number };

const repoRoot = fileURLToPath(new URL("../../../../../../../../../../", import.meta.url));
const binary = (() => { try { return process.env.SEMIO_NATIVE_TRANSPILE_BIN ?? nativeTranspileBinaryV1(repoRoot, "prototype"); } catch { return undefined; } })();

function results(exports: Exports, calls: readonly Call[]): number[] {
  return calls.map(call => (exports[call.export] as () => number)());
}

for (const row of corpus.cases) {
  test.skipIf(binary === undefined)(`split link: ${row.name}`, async () => {
    const work = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "split-link-"));
    try {
      const framework = await parse(row.framework), plugin = await parse(row.plugin);
      writeFileSync(join(work, "image.wasm"), framework);
      writeFileSync(join(work, "plugin.wasm"), plugin);
      const merged = join(work, "merged.wasm");
      const run = spawnSync(binary!, ["merge", "--framework", join(work, "image.wasm"), "--plugin", join(work, "plugin.wasm"), "-o", merged], { encoding: "utf8" });
      if ("error" in row) {
        expect(run.status).toBe("missing" in row ? 3 : 1);
        expect(run.stderr).toContain(row.error);
        for (const symbol of ("missing" in row ? row.missing : [])) expect(run.stderr).toContain(`\n${symbol}\n`);
        expect(existsSync(merged)).toBe(false);
        return;
      }
      expect(run.stderr).toBe("");
      expect(run.status).toBe(0);
      const module = await WebAssembly.compile(readFileSync(merged));
      expect(WebAssembly.Module.exports(module).map(e => e.name)).toEqual(row.exports!);
      const stubs: Record<string, Record<string, () => number>> = {};
      for (const entry of WebAssembly.Module.imports(module)) (stubs[entry.module] ??= {})[entry.name] = () => 0;
      const instance = (await WebAssembly.instantiate(module, stubs)).exports as Exports;
      (instance._initialize as () => void)();
      const expected = row.calls!.map(call => call.expect);
      expect(results(instance, row.calls!)).toEqual(expected);
    } finally {
      rmSync(work, { recursive: true, force: true });
    }
  });
}
