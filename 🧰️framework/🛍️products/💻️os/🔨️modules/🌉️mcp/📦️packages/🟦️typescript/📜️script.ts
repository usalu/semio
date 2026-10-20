#!/usr/bin/env bun
import { receiveScriptProcessInvocation, withIssuedResourceControlV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** 🌉️ `@semio-tech/framework-os-mcp` TS task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`.
 * The repository build tool restores Rust outputs before these process consumers run. An explicit binary override
 * remains a strict prebuilt-artifact seam; both paths require an executable before Vitest starts. */
import { join } from "node:path";
import { runVitest } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧪️vitest/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { requireMcpBinary } from "../../🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "quick");
    console.log(`[test] ${requireMcpBinary(this.repoRoot)}`);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

/** 🧱️ Runs the retained owned fixture law against its independent oracle. */
class ApprovalRequestFixtureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-approval-request-fixture accepts no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/✅️approval-request/🟦️.ts")], { cwd: this.repoRoot, env: process.env, budgetMs: 15_000, throwOnFailure: true });
  }
}

router.register("test-approval-request-fixture", ApprovalRequestFixtureScript);

/** 🏗️ Checks native staging ports without preparing a Rust binary. */
class NativeStagingSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-native-staging accepts no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🏗️staging/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, env: process.env, budgetMs: 15_000, throwOnFailure: true });
  }
}

router.register("test-native-staging", NativeStagingSourceScript);

/** 📡️ Exercises the installed native frontend and forwarded OS protocol through one owned test phase. */
class CanonicalStdioProbeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-canonical-stdio accepts no arguments");
    const { runCanonicalOsMcpProbeV1, canonicalOsMcpProbeMaximumMillisecondsV1 } = await import("../../🧪️tests/🚪️canonical/🟦️.ts");
    await withIssuedResourceControlV1(this.invocation, "mcp:test-canonical-stdio", canonicalOsMcpProbeMaximumMillisecondsV1, control => runCanonicalOsMcpProbeV1(this.repoRoot, this.invocation, control, process.env));
  }
}

router.register("test-canonical-stdio", CanonicalStdioProbeScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
