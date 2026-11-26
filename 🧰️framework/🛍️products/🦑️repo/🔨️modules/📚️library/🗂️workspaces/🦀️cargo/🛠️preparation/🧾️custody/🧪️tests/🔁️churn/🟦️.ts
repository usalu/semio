import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { resolve, dirname, join } from "node:path";
import cases from "./🔣️.json" with { type: "json" };
import discovery from "./📁️discovery.json" with { type: "json" };
import { readdirSync } from "node:fs";
import * as custody from "../../🟦️.ts";
import * as routing from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { concurrentEditOfV1, retryOnConcurrentEditV1 } from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

test("Cargo preparation retries changed reads and refuses conflicting output writes", async () => {
  const root = custody.cargoPreparationSourceRootV1(), path = resolve(import.meta.dirname, "🔣️.json");
  const current = custody.cargoPreparationInputV1(root, path, "file"), changed = { ...current, sha256: "0".repeat(64) };
  const sql = new Database(":memory:");
  try {
    for (const row of cases) {
      const envelope = routing.createScriptProcessEnvelope({ version: 1, owner: "test:preparation-churn", maximumElapsedMilliseconds: 10000 }, { workspaceRoot: root }, Date.now());
      await routing.withScriptProcessEnvelope(envelope, async original => {
        const stop = new AbortController(), control = routing.createIssuedResourceControlV1(original, "test:preparation-churn", 5000, stop.signal);
        const cancelled = new Error("fixture cancelled");
        const phase = {
          leaseControl: control, signal: control.signal, advance: async () => { if (row.effect === "cancel") stop.abort(cancelled); },
          childBudget: () => 5000, childOptions: () => ({ signal: control.signal, output: { maximumBytes: 8192, maximumLines: 32 } }),
        };
        const input = row.input === "directory" ? { ...changed, path: dirname(path), kind: "directory" as const } : changed;
        const observed = row.effect === "vanished" ? { ...input, path: path + ".absent" } : input;
        const observation = {
          version: 1, root, sourceRoot: root, script: path, command: ["fixture"],
          sources: [row.source ? observed : current], resolverInputs: [],
          inputs: row.input ? [observed] : [], outputs: row.source || row.input ? [] : [observed], resolutions: [],
        };
        const failure = await custody.assertCargoPreparationObservationCurrentV1(observation, phase).catch(error => error);
        expect(failure).toBeInstanceOf(Error);
        if (row.effect === "cancel") expect(failure).toBe(cancelled);
        const wire = custody.cargoPreparationSourceEditDiagnosticV1(failure) ?? "Preparation custody changed: " + path;
        const result = { status: row.status, signal: row.signal, stderr: row.effect === "invalid" ? wire.replace('"bytes"', '"output"') : wire, stdout: "" };
        const restored = custody.readCargoPreparationSourceEditV1(result, row.effect === "outside" ? join(dirname(root), "other") : root, row.effect === "outside" ? join(dirname(root), "source") : root);
        const oracle = sql.query("SELECT ?1 AND ?2 IN ('edit','vanished') AND ?3>0 AND ?4 IS NULL AS retry").get(Number(row.source || Boolean(row.input)), row.effect, row.status, row.signal) as { retry: number };
        expect(Boolean(restored)).toBe(Boolean(oracle.retry));
        expect(Boolean(restored)).toBe(row.retry);
        if (restored) expect(concurrentEditOfV1(restored)).toMatchObject({ reason: row.effect === "vanished" ? "vanished" : "bytes", path: observed.path });
        let attempts = 0;
        const terminal = row.effect === "cancel" ? failure : new Error("Native Cargo owner preparation failed");
        const operation = retryOnConcurrentEditV1("build", async () => {
          if (++attempts === 1) throw restored ?? terminal;
        }, { modifiedAgeMs: () => undefined, sleep: async () => {}, onRetry: () => {} });
        if (restored) await operation;
        else await expect(operation).rejects.toBe(terminal);
        expect(attempts).toBe(row.retry ? 2 : 1);
      });
    }
  } finally { sql.close(); }
});

test("registry directory custody depends on children the discovery actually examines", () => {
  const root = custody.cargoPreparationSourceRootV1();
  const path = import.meta.dirname;
  const sql = new Database(":memory:");
  try {
    sql.run("CREATE TABLE children (name TEXT, kind TEXT)");
    for (const entry of readdirSync(path, { withFileTypes: true })) {
      const kind = entry.isDirectory() ? "directory" : entry.isSymbolicLink() ? "symlink" : "file";
      sql.run("INSERT INTO children VALUES (?, ?)", [entry.name, kind]);
    }
    for (const row of discovery.cases) {
      const selected = sql.query("SELECT name, kind FROM children WHERE kind IN ('directory', 'symlink') OR substr(name, -length(?1)) = ?1 ORDER BY name").all(row.suffix) as { name: string; kind: string }[];
      expect(selected.map(entry => entry.name)).toEqual(row.expected);
      const hash = new Bun.CryptoHasher("sha256").update(JSON.stringify(selected.map(entry => [entry.name, entry.kind]))).digest("hex");
      expect(custody.cargoPreparationInputV1(root, path, "discovery", row.suffix)).toEqual({path, kind:"discovery", suffix:row.suffix, sha256:hash});
    }
    for (const suffix of discovery.invalidSuffixes) {
      expect(() => custody.cargoPreparationInputV1(root, path, "discovery", suffix)).toThrow();
    }
  } finally { sql.close(); }
});
