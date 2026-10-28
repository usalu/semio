import { expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, rmSync, symlinkSync, truncateSync, utimesSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import { CLEAN_GENERATED_OVERSIZED_FILE_BYTES, CLEAN_OVERSIZED_IGNORED_FILE_MAX_BYTES } from "../../🧼️workspace-cleanup/🔍️candidate-discovery/🟦️.ts";
import { CleanScript } from "../../🧼️workspace-cleanup/🎮️command/🟦️.ts";
import { createScriptProcessEnvelope, withScriptProcessEnvelope } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type { ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const clean = (root: string): Promise<void> => withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "clean-ticket-runs-test", maximumElapsedMilliseconds: 0 }, {}, Date.now()), async (invocation: ScriptInvocation) => { await new CleanScript(root, root, invocation).run([]); });

const old = (...paths: string[]): void => { const time = new Date(Date.now() - 7_200_000); for (const path of paths) utimesSync(path, time, time); };
const repository = (root: string): void => { if (spawnSync("git", ["init", "-q"], { cwd: root, stdio: "ignore" }).status !== 0) throw new Error("git init failed for cleanup fixture."); };

test("clean removes generated ticket run output without removing ticket material", async () => {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for cleanup fixture output.");
  mkdirSync(artifactRoot, { recursive: true });
  const root = mkdtempSync(join(artifactRoot, "semio-clean-ticket-runs-"));
  const ticket = join(root, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️09", "☀️01", "CLEAN-TICKET-RUNS");
  const runs = ["🧾️runs", "🧪️runs"].map((name) => join(ticket, "📓️energy-rust-reference-diagnostics", "🧭️finite-target-consumption", name, "🔖️05BsOk", "📝️.md"));
  const probes = ["🧪️cli-plan-cancellation-05BsOk", "🧪️inventory-producer-order-05BsOk"].map((name) => join(ticket, name, "📝️.md"));
  const material = join(ticket, "📝️summary.md");
  try {
    for (const run of runs) {
      mkdirSync(dirname(run), { recursive: true });
      writeFileSync(run, "generated run output\n");
    }
    for (const probe of probes) {
      mkdirSync(dirname(probe), { recursive: true });
      writeFileSync(probe, "generated probe output\n");
    }
    writeFileSync(material, "ticket material\n");
    writeFileSync(join(ticket, "🎫️ticket.json"), JSON.stringify({ status: "closed" }, null, 2));
    repository(root);
    old(...runs, ...probes, material);
    const openTicket = join(root, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️09", "☀️01", "OPEN-TICKET-RUNS");
    const openRun = join(openTicket, "🧾️runs", "🔖️05BsOk", "📝️.md");
    mkdirSync(dirname(openRun), { recursive: true });
    writeFileSync(openRun, "live run output\n");
    writeFileSync(join(openTicket, "🎫️ticket.json"), JSON.stringify({ status: "open" }, null, 2));
    try { symlinkSync(material, join(dirname(runs[0]!), "alias")); } catch {}
    await clean(root);
    for (const run of runs) expect(existsSync(run)).toBe(false);
    for (const probe of probes) expect(existsSync(probe)).toBe(false);
    expect(existsSync(openRun)).toBe(true);
    expect(existsSync(material)).toBe(true);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("clean removes root Generation3d transient mounts without removing ordinary directories", async () => {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for cleanup fixture output.");
  mkdirSync(artifactRoot, { recursive: true });
  const root = mkdtempSync(join(artifactRoot, "semio-clean-root-transient-"));
  const transients = [".generation3d-crate-link", ".generation3d-edit-link", ".w-g3-ticket"].map((name) => join(root, name));
  const retained = join(root, "source");
  try {
    for (const directory of transients) {
      mkdirSync(directory, { recursive: true });
      writeFileSync(join(directory, "state"), "transient\n");
    }
    mkdirSync(retained, { recursive: true });
    await clean(root);
    for (const directory of transients) expect(existsSync(directory)).toBe(false);
    expect(existsSync(retained)).toBe(true);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("clean keeps oversized ignored files of open tickets and removes those of closed tickets", async () => {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for cleanup fixture output.");
  mkdirSync(artifactRoot, { recursive: true });
  const root = mkdtempSync(join(artifactRoot, "semio-clean-oversized-ignored-"));
  const tickets = join(root, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️09", "☀️24");
  const open = join(tickets, "CLEAN-OVERSIZED-IGNORED-OPEN"), closed = join(tickets, "CLEAN-OVERSIZED-IGNORED-CLOSED");
  const hugeOpen = join(open, "bin", "os-hub.big"), hugeClosed = join(closed, "bin", "os-hub.big");
  const retained = join(open, "📝️notes.md");
  try {
    repository(root);
    writeFileSync(join(root, ".gitignore"), "*.big\n");
    for (const [ticket, huge, status] of [[open, hugeOpen, "open"], [closed, hugeClosed, "closed"]] as const) {
      mkdirSync(dirname(huge), { recursive: true });
      writeFileSync(huge, "x");
      truncateSync(huge, CLEAN_OVERSIZED_IGNORED_FILE_MAX_BYTES + 1);
      writeFileSync(join(ticket, "🎫️ticket.json"), JSON.stringify({ status }, null, 2));
    }
    writeFileSync(retained, "ticket material\n");
    await clean(root);
    expect(existsSync(hugeOpen)).toBe(true);
    expect(existsSync(hugeClosed)).toBe(false);
    expect(existsSync(retained)).toBe(true);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("clean never removes anything from an open ticket and never a tracked file or a fresh writer of a closed one", async () => {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for cleanup fixture output.");
  mkdirSync(artifactRoot, { recursive: true });
  const root = mkdtempSync(join(artifactRoot, "semio-clean-open-ticket-"));
  const tickets = join(root, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️09", "☀️24");
  const open = join(tickets, "CLEAN-OPEN-TICKET-OVERSIZED"), closed = join(tickets, "CLEAN-CLOSED-TICKET-TRACKED"), busy = join(tickets, "CLEAN-CLOSED-TICKET-BUSY");
  const big = CLEAN_GENERATED_OVERSIZED_FILE_BYTES + 1;
  const put = (path: string, size: number, text = "x"): string => { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, text); truncateSync(path, size); return path; };
  try {
    repository(root);
    const monolith = [put(join(open, "🪓️monolith-split", "a.bin"), big), put(join(open, "🪓️monolith-split", "b.bin"), big)];
    const openGenerated = put(join(open, "🗑️generated", "lane", "log.bin"), big);
    const openBlob = put(join(open, "db-test"), big);
    writeFileSync(join(open, "🎫️ticket.json"), JSON.stringify({ status: "open" }, null, 2));
    const closedAuthored = put(join(closed, "db-test"), big);
    const closedTracked = put(join(closed, "🗑️generated", "tracked.bin"), big);
    const closedLoose = put(join(closed, "🗑️generated", "loose.bin"), big);
    writeFileSync(join(closed, "🎫️ticket.json"), JSON.stringify({ status: "closed" }, null, 2));
    const busyOld = put(join(busy, "🗑️generated", "old.bin"), big);
    const busyFresh = put(join(busy, "🗑️generated", "lane.log"), 1);
    writeFileSync(join(busy, "🎫️ticket.json"), JSON.stringify({ status: "closed" }, null, 2));
    old(...monolith, openGenerated, openBlob, closedAuthored, closedTracked, closedLoose, busyOld);
    expect(spawnSync("git", ["add", "-f", "--", closedTracked], { cwd: root, stdio: "ignore" }).status).toBe(0);
    await clean(root);
    for (const kept of [...monolith, openGenerated, openBlob, closedAuthored, closedTracked, busyOld, busyFresh]) expect(existsSync(kept), kept).toBe(true);
    expect(existsSync(closedLoose)).toBe(false);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
