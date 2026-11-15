import { execFileSync } from "node:child_process";
import { mkdtempSync, realpathSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join } from "node:path";
import { createInterface } from "node:readline";
import { fileURLToPath } from "node:url";
import { libraryBootstrap } from "../../../../../🟨️.mjs";
import { projectGeneratorContractsV1 } from "../../../../🟨️.mjs";
import { closeProjectFarmsV1, openProjectFarmV1 } from "../../../🟨️.mjs";
import { farmGrantEnvironmentV1 } from "../🎟️grant/🟨️.mjs";

/** 🪦️ A read of its own: opens the farm of one session with two real worker processes (the pool of its own grant and one host grant), prints their process ids once both are ready, and then waits for the commands `abort` (cancels the signal of the read, as a stopped read does) and `exit` on its input. */
const root = (() => { let at = dirname(fileURLToPath(import.meta.url)); while (basename(at) !== "🧰️framework") at = dirname(at); return dirname(at); })();
await libraryBootstrap;
const granted = mkdtempSync(join(realpathSync(tmpdir()), "farm-orphan-")), controller = new AbortController(), lines = [];
const farm = openProjectFarmV1({ workspaceRoot: root, context: { registry: null }, contracts: projectGeneratorContractsV1(root), projects: 100, commandInputs: null, projectsByRoot: new Map(), componentRoots: [], environment: { ...process.env, ...farmGrantEnvironmentV1(granted, "farm_orphan_001"), SEMIO_GRAPH_WORKERS: "2" }, processed: () => 24, cores: 4, write: line => lines.push(line), launch: null, engage: true, signal: controller.signal, request: () => ({ granted: Promise.resolve({ jobs: 1, megabytes: 3584 }), release: () => undefined }) });
const loops = await new Promise(done => { farm.onReady(total => { if (total >= 6) done(total); }); setTimeout(() => done(0), 120000).unref?.(); });
const children = () => execFileSync("pgrep", ["-P", String(process.pid)], { encoding: "utf8" }).split("\n").filter(Boolean).map(Number);
console.log(JSON.stringify({ loops, children: children(), lines }));
for await (const line of createInterface({ input: process.stdin })) {
  if (line === "abort") { controller.abort(); console.log(JSON.stringify({ aborted: true })); }
  if (line === "exit") break;
}
await farm.close();
await closeProjectFarmsV1();
rmSync(granted, { recursive: true, force: true });
process.exit(0);
