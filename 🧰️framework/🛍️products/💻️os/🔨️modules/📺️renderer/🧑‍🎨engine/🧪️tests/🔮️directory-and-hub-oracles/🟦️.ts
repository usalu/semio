/** 📇️ Runs the language-neutral Node oracles of the directory and hub contracts; the host package script runs this file instead of importing the oracles. */
import { expect, test } from "bun:test";
import { resolve } from "node:path";
import { directoryHomeBootstrapOracle, directoryInviteCapabilityOracle, hubAuthContractOracle, scopedPresenceOracle } from "../../🔮️oracles/📇️directory-and-hub-contracts/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../../..");

for (const [name, oracle] of [["directory-home-bootstrap-oracle", directoryHomeBootstrapOracle], ["hub-auth-contract-oracle", hubAuthContractOracle], ["directory-invite-capability-oracle", directoryInviteCapabilityOracle], ["scoped-presence-oracle", scopedPresenceOracle]] as const) {
  test(name, () => {
    const checks = oracle(repoRoot);
    expect(checks).toBeGreaterThan(0);
    console.log(`${name}: checks=${checks} clean`);
  });
}
