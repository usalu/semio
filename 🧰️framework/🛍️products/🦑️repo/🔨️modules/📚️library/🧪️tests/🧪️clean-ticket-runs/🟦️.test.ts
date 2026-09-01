import { expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { CleanScript } from "../../../../../../../📜️script.ts";

test("clean removes generated ticket run output without removing ticket material", () => {
  const root = mkdtempSync(join(tmpdir(), "semio-clean-ticket-runs-"));
  const ticket = join(root, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️09", "☀️01", "CLEAN-TICKET-RUNS");
  const run = join(ticket, "📓️energy-rust-reference-diagnostics", "🧭️finite-target-consumption", "🧾️runs", "🔖️05BsOk", "📝️.md");
  const material = join(ticket, "📝️summary.md");
  try {
    mkdirSync(dirname(run), { recursive: true });
    writeFileSync(run, "generated run output\n");
    writeFileSync(material, "ticket material\n");
    try { symlinkSync(material, join(dirname(run), "alias")); } catch {}
    new CleanScript(root, root).run([]);
    expect(existsSync(run)).toBe(false);
    expect(existsSync(material)).toBe(true);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
