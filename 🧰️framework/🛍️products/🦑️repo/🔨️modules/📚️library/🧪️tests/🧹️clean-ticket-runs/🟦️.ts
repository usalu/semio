import { expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { CleanScript } from "../../../../../../../📜️script.ts";

test("clean removes generated ticket run output without removing ticket material", () => {
  const root = mkdtempSync(join(tmpdir(), "semio-clean-ticket-runs-"));
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
