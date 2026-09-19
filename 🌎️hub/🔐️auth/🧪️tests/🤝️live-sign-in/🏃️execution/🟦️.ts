import { join } from "node:path";
import { BundleScript } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🤝️ Runs the live sign-in gate: a real `os-hub` child against a fresh data root, the operator
 * bootstrap verb, and the whole two-principal session lifecycle over HTTP. Credential sign-in is
 * enabled for the child here because the gate exists to exercise exactly that policy — it is not a
 * default any other route inherits. */
export class HubLiveSignInScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("live-sign-in-check accepts no arguments");
    await runOwnedCommand("bun", [join(this.repoRoot, "🌎️hub/🔐️auth/🧪️tests/🤝️live-sign-in/🟦️.ts")], this.repoRoot, "hub-live-sign-in", 300_000, {
      env: { ...process.env, OS_HUB_CREDENTIAL_SIGN_IN: "1" },
    });
  }
}
