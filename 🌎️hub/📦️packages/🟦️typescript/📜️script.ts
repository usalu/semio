#!/usr/bin/env bun
/** 🌎️ `os-hub-ts` (nx `os-hub-ts`) router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`.
 * Bun integration-test harness that boots the REAL `os-hub` binary and drives it with two
 * independent clients to prove the hub's collaboration contract end-to-end (ticket
 * 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS, lane 3-E). Gated behind
 * `HUB_E2E=1` (see `🧪️index.test.ts`'s own doc) — the default `test` run never touches cargo and
 * reports the whole e2e suite as skipped in well under a second. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo, runVitest } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const HUB_RUST_DIR = "🌎️hub/📦️packages/🦀️rust";

/** 🏗️ Builds the real `os-hub` debug binary the test spawns directly (never `cargo run` — no
 * wrapper-process tree to chase on teardown). Default cargo features only (sqlite): contract-freeze
 * Amendment 2 says `--all-features` is red repo-wide and pre-existing (`🛢️db`'s postgres/neo4j
 * features have no wired driver deps), so this never passes it. Only runs when `HUB_E2E=1`, so the
 * default `test` target never pays a cargo build. */
function buildHubBinary(repoRoot: string): void {
  if (process.env.HUB_E2E !== "1") return;
  console.log("[os-hub-ts] HUB_E2E=1 — building os-hub (default features, no --all-features)…");
  runCargo(["build", "--manifest-path", "Cargo.toml"], join(repoRoot, HUB_RUST_DIR));
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    buildHubBinary(this.repoRoot);
    await runVitest(this.root, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
