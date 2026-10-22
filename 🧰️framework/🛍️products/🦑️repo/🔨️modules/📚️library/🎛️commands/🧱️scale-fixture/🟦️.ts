import { Script } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🧱️ The 50×50 scale fixture (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet F1): `check` regenerates the seeded registry and catalog and diffs, the freshness idiom `plugin-registry:check` uses. */
export class ScaleFixtureScript extends Script {
  async run(segments: string[]): Promise<void> {
    if (segments[0] !== "check") throw new Error(`unknown scale-fixture subcommand: ${segments[0]} (expected check)`);
    const { runCmd } = await import("../../🏃️process/🟦️.ts");
    runCmd("repo", ["run", "@semio-tech/framework-os-dev:scale-fixture-check"], { cwd: this.root });
  }
}
