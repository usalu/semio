import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { existsSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { CargoProvenanceSessionV1, cargoProvenancePhysicalControlV1, sharedProvenanceCustodyV1 } from "../../../🟦️.ts";
import { failedPassConcurrentEditV1 } from "../🟦️.ts";
import fixture from "./reclaim.json" with { type: "json" };

for (const row of fixture.cases) test(row.name, async () => {
  const root = mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!), "reclaim-"));
  const db = new Database(":memory:");
  try {
    const control = () => cargoProvenancePhysicalControlV1({ cancelled: () => false, onProgress: () => {} });
    const custody = sharedProvenanceCustodyV1(root, control);
    const session = new CargoProvenanceSessionV1(custody);
    const source = join(root, "lib.rs"), manifest = join(root, "Cargo.toml");
    const output = join(root, "generated.rs"), directory = join(root, "roster");
    mkdirSync(directory);
    const directoryOutput = join(directory, "build_script_build");
    writeFileSync(source, "source");
    writeFileSync(manifest, "[workspace]\n");
    writeFileSync(output, "generated");
    writeFileSync(directoryOutput, "roster entry");
    writeFileSync(join(root, "semio-runtime-resource-inputs.jsonl"), JSON.stringify({ kind: "directory", path: directory }) + "\n");
    const paths: [string, boolean][] = [[directoryOutput, true]];
    for (let pass = 0; pass < row.passes; pass++) {
      const artifact = join(root, `artifact-${pass}.rlib`);
      writeFileSync(artifact, "output");
      writeFileSync(artifact.replace(".rlib", ".d"), `${artifact}: ${source} ${output}\n`);
      await session.prefetch({ filenames: [artifact, output, directoryOutput] });
      await session.write(join(root, `receipt-${pass}.json`), {
        manifest, cwd: root, command: "cargo", args: ["build"], buildDirectory: root,
        builtAtMs: Date.now(), status: row.change !== "seal" && pass + 1 === row.passes ? 101 : 0, cancelled: false, units: [],
        buildScripts: [{ package_id: "fixture", out_dir: root }],
      }, new Map(), root);
      paths.push([artifact, true]);
      expect(session.reclaimOutput(artifact, () => undefined)).toBeUndefined();
    }
    paths.push([output, true]);
    db.exec("create table claim(path text, retained integer)");
    for (const [path, retained] of paths) db.query("insert into claim values (?, ?)").run(path, Number(retained));
    const expected = db.query<{ path: string }, []>("select path from claim where retained = 1 order by path").all().map(result => result.path);
    expect(paths.filter(([path]) => existsSync(path)).map(([path]) => path).sort()).toEqual(expected);
    expect(session.reclaimOutput(output, () => undefined)).toBeUndefined();
    expect(session.reclaimOutput(directoryOutput, () => undefined)).toBeUndefined();
    const unretired = join(root, "unretired.rlib");
    const changes: Record<string, () => void> = {
      none: () => undefined,
      overlap: () => writeFileSync(output, "changed"),
      source: () => rmSync(source),
      absent: () => writeFileSync(join(root, "rust-toolchain.toml"), "[toolchain]\n"),
      directory: () => writeFileSync(join(directory, "new"), "new"),
      refused: () => rmSync(unretired),
      seal: () => undefined,
    };
    if (row.change === "refused") {
      writeFileSync(unretired, "unretired");
      await session.prefetch({ filenames: [unretired] });
      expect(session.reclaimOutput(unretired, () => { throw Error("outside reservation"); })).toBeUndefined();
    }
    changes[row.change]!();
    const edit = await failedPassConcurrentEditV1({ startedMs: Date.now(), files: [], recheck: () => session.recheckFailedPass() });
    expect(edit?.reason ?? null).toBe(row.reason);
    if (row.change === "seal") {
      const unclaimed = join(root, "unclaimed.rmeta");
      writeFileSync(unclaimed, "unclaimed");
      expect(() => session.reclaimOutput(unclaimed, () => { throw Error("outside reservation"); })).toThrow("outside reservation");
      expect(existsSync(unclaimed)).toBeTrue();
      expect(session.reclaimOutput(unclaimed, () => undefined)).toBe(9);
      expect(existsSync(unclaimed)).toBeFalse();
      await session.seal();
      await session.close();
      expect(existsSync(join(root, "receipt-0.json"))).toBeTrue();
    }
  } finally {
    db.close();
    rmSync(root, { recursive: true, force: true });
  }
});
