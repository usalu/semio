import { expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, rmSync, symlinkSync, truncateSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import { CLEAN_OVERSIZED_IGNORED_FILE_MAX_BYTES } from "../../🧼️workspace-cleanup/🔍️candidate-discovery/🟦️.ts";
import { CleanScript } from "../../🧼️workspace-cleanup/🎮️command/🟦️.ts";

test("clean removes generated ticket run output without removing ticket material", () => {
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
    try { symlinkSync(material, join(dirname(runs[0]!), "alias")); } catch {}
    new CleanScript(root, root).run([]);
    for (const run of runs) expect(existsSync(run)).toBe(false);
    for (const probe of probes) expect(existsSync(probe)).toBe(false);
    expect(existsSync(material)).toBe(true);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("clean removes root Generation3d transient mounts without removing ordinary directories", () => {
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
    new CleanScript(root, root).run([]);
    for (const directory of transients) expect(existsSync(directory)).toBe(false);
    expect(existsSync(retained)).toBe(true);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("clean removes oversized ignored files from open tickets", () => {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for cleanup fixture output.");
  mkdirSync(artifactRoot, { recursive: true });
  const root = mkdtempSync(join(artifactRoot, "semio-clean-oversized-ignored-"));
  const ticket = join(root, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️09", "☀️24", "CLEAN-OVERSIZED-IGNORED");
  const huge = join(ticket, "bin", "os-hub.big");
  const retained = join(ticket, "📝️notes.md");
  try {
    const init = spawnSync("git", ["init"], { cwd: root, stdio: "ignore" });
    if (init.status !== 0) throw new Error("git init failed for cleanup fixture.");
    writeFileSync(join(root, ".gitignore"), "*.big\n");
    mkdirSync(dirname(huge), { recursive: true });
    writeFileSync(huge, "x");
    truncateSync(huge, CLEAN_OVERSIZED_IGNORED_FILE_MAX_BYTES + 1);
    writeFileSync(join(ticket, "🎫️ticket.json"), JSON.stringify({ status: "open" }, null, 2));
    writeFileSync(retained, "ticket material\n");
    new CleanScript(root, root).run([]);
    expect(existsSync(huge)).toBe(false);
    expect(existsSync(retained)).toBe(true);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("clean removes oversized non-ignored files from open tickets", () => {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for cleanup fixture output.");
  mkdirSync(artifactRoot, { recursive: true });
  const root = mkdtempSync(join(artifactRoot, "semio-clean-oversized-open-ticket-"));
  const ticket = join(root, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️09", "☀️24", "CLEAN-OPEN-TICKET-OVERSIZED");
  const huge = join(ticket, "db-test");
  const retained = join(ticket, "📝️notes.md");
  try {
    mkdirSync(ticket, { recursive: true });
    writeFileSync(huge, "x");
    truncateSync(huge, CLEAN_OVERSIZED_IGNORED_FILE_MAX_BYTES + 1);
    writeFileSync(join(ticket, "🎫️ticket.json"), JSON.stringify({ status: "open" }, null, 2));
    writeFileSync(retained, "ticket material\n");
    new CleanScript(root, root).run([]);
    expect(existsSync(huge)).toBe(false);
    expect(existsSync(retained)).toBe(true);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
