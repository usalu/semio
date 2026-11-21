/** 🦀️ The native transpile writes exactly the files `jco transpile` writes, over the language-agnostic cases of `🧫️cases.json`; jco is the third-party oracle. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { componentEmbed, componentNew } from "@bytecodealliance/jco";
import cases from "./🧫️cases.json" with { type: "json" };
import { nativeTranspileArgsV1, nativeTranspileBinaryV1 } from "../../🦀️native-transpile/🟦️.ts";

import { rewritePreview2ShimImports } from "../../🟦️.ts";

const repoRoot = fileURLToPath(new URL("../../../../../../../../../", import.meta.url));

function tree(root: string, dir = root): Map<string, string> {
  const files = new Map<string, string>();
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);
    if (statSync(path).isDirectory()) for (const [name, hash] of tree(root, path)) files.set(name, hash);
    else files.set(relative(root, path), new Bun.CryptoHasher("sha256").update(readFileSync(path)).digest("hex"));
  }
  return files;
}

const binary = existsSync(join(repoRoot, ".cache")) ? (() => { try { return nativeTranspileBinaryV1(repoRoot); } catch { return undefined; } })() : undefined;

for (const row of cases.cases) {
  test.skipIf(binary === undefined)(`native transpile equals jco: ${row.name}`, async () => {
    const work = mkdtempSync(join(tmpdir(), "native-transpile-"));
    try {
      mkdirSync(join(work, "wit"));
      writeFileSync(join(work, "wit", "probe.wit"), row.wit);
      const component = join(work, `${row.name_option}.wasm`);
      writeFileSync(component, await componentNew(await componentEmbed({ dummy: true, witPath: join(work, "wit") }), []));
      const reference = join(work, "jco"), native = join(work, "native");
      const jcoBin = join(repoRoot, "node_modules", ".bin", "jco");
      const jco = spawnSync(process.execPath, [jcoBin, "transpile", component, "-o", reference, "-q", "--name", row.name_option, ...row.maps.flatMap((map) => ["--map", map])], { encoding: "utf8" });
      expect(jco.status).toBe(0);
      const ours = spawnSync(binary!, [...nativeTranspileArgsV1(component, native, row.name_option, row.maps)], { encoding: "utf8" });
      expect(ours.stderr).toBe("");
      expect(ours.status).toBe(0);
      const expected = tree(reference);
      expect(expected.size).toBeGreaterThan(1);
      expect([...tree(native)]).toEqual([...expected]);
    } finally {
      rmSync(work, { recursive: true, force: true });
    }
  });
}

for (const row of cases.shim_imports) {
  test(`preview2 imports load from a ${row.name} directory`, async () => {
    const work = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "shim-import-"));
    try {
      const component = join(work, ...row.component.split("/"));
      const shim = join(work, ...row.shim.split("/"));
      mkdirSync(dirname(component), { recursive: true });
      mkdirSync(shim, { recursive: true });
      writeFileSync(component, 'export { marker } from "@bytecodealliance/preview2-shim/cli";\n');
      writeFileSync(join(shim, "cli.js"), `export const marker = ${JSON.stringify(row.expect)};\n`);
      rewritePreview2ShimImports(component, shim);
      const { marker } = await import(pathToFileURL(component).href);
      expect(marker).toBe(row.expect);
    } finally {
      rmSync(work, { recursive: true, force: true });
    }
  });
}
