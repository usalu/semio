import "../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🟦️.ts";
import { test, expect } from "bun:test";
import { existsSync, mkdirSync, readFileSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🦀️exact-cargo-laws/🔣️.json", import.meta.url), "utf8"));

test("active exact Cargo lease protects ticket evidence from workspace cleanup", async () => {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
  const workspace = mkdtempSync(join(artifactRoot, "exact-cargo-clean-fixture-"));
  const ticket = join(workspace, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️09", "☀️05", "ACTIVE-CARGO");
  const generated = join(ticket, "🗑️generated");
  const lease = join(generated, "run", `${fixture.activeLease.directoryPrefix}fixture`);
  try {
    mkdirSync(lease, { recursive: true });
    writeFileSync(join(ticket, "🎫️ticket.json"), '{"status":"closed"}');
    writeFileSync(join(lease, fixture.activeLease.manifestName), JSON.stringify({ version: fixture.activeLease.version, pid: process.pid }));
    const { CleanScript } = await import("../../🧼️workspace-cleanup/🎮️command/🟦️.ts");
    await new CleanScript(workspace, workspace).run([]);
    expect(existsSync(generated)).toBe(true);
    rmSync(lease, { recursive: true });
    await new CleanScript(workspace, workspace).run([]);
    expect(existsSync(generated)).toBe(false);
  } finally { rmSync(workspace, { recursive: true, force: true }); }
}, 60_000);

