import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { dirname, join, resolve } from "node:path";
import { mkdirSync, mkdtempSync, readFileSync, utimesSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { spawnSync } from "node:child_process";
import * as configuration from "../../🟦️.ts";

/** 📦️ Compares VSCE selection and repeated archive bytes with the fixture and independent ZIP metadata decoding. */
export async function testExtensionPackage(generated: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  const root = mkdtempSync(join(generated, "extension-package-")), packageRoot = resolve(import.meta.dir, "../../.."), vsce = require("@vscode/vsce");
  for (const [file, content] of Object.entries(fixture.files)) { const path = join(root, file); mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, content as string); }
  writeFileSync(join(root, ".vscodeignore"), readFileSync(join(packageRoot, ".vscodeignore")));
  const selected = await vsce.listFiles({ cwd: root, packageManager: vsce.PackageManager.None });
  assert.deepEqual(selected.sort(), fixture.expectedFiles.toSorted());
  const env = configuration.extensionPackageEnvironment({ ...process.env, SOURCE_DATE_EPOCH: "1", TZ: "Pacific/Honolulu" });
  assert.equal(env.SOURCE_DATE_EPOCH, fixture.epoch); assert.equal(env.TZ, fixture.timezone);
  const cli = join(dirname(require.resolve("@vscode/vsce/package.json")), "vsce"), archives: string[] = [];
  for (const [index, timestamp] of [1_500_000_000, 1_700_000_000].entries()) {
    for (const file of fixture.expectedFiles) utimesSync(join(root, file), timestamp, timestamp);
    const path = join(root, `${index}.vsix`), result = spawnSync("node", [cli, "package", "--no-dependencies", "--out", path], { cwd: root, env, encoding: "utf8", timeout: 30000 });
    writeFileSync(join(root, `${index}.log`), result.stdout + result.stderr);
    assert.equal(result.status, 0, (result.stdout + result.stderr).slice(-2000)); archives.push(path);
  }
  assert.equal(createHash("sha256").update(readFileSync(archives[0])).digest("hex"), createHash("sha256").update(readFileSync(archives[1])).digest("hex"));
  const zip = createRequire(require.resolve("@vscode/vsce/package.json"))("yauzl");
  const entries = await new Promise<{ name: string; time: number; date: number }[]>((accept, reject) => {
    zip.open(archives[1], { lazyEntries: true }, (error: Error | null, reader: any) => {
      if (error) return reject(error);
      const rows: { name: string; time: number; date: number }[] = [];
      reader.on("error", reject); reader.on("end", () => accept(rows));
      reader.on("entry", (entry: any) => { rows.push({ name: entry.fileName, time: entry.lastModFileTime, date: entry.lastModFileDate }); reader.readEntry(); }); reader.readEntry();
    });
  });
  assert.deepEqual(entries.map((entry) => entry.name).sort(), fixture.expectedZip.toSorted());
  for (const entry of entries) { assert.equal(entry.time, fixture.time, entry.name); assert.equal(entry.date, fixture.date, entry.name); }
  console.log("[DEBUG] Native VSCE ships only the runtime allowlist; changed source timestamps preserve exact archive bytes and ZIP metadata PASS");
}
