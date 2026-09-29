import { interactivityDbCliFailures } from "../../../../../../../📜️script.ts";

/** ⌨️ Executes the DB CLI process-entry wait census assertions: 18 production waits plus one test-only `seed_document` wait
 * pass; a 19th production wait, a production `seed_document`, and a test-only `seed_document` without its wait are refused. */
export function interactivityDbCliSelfTests(): void {
  const productionCliWaits = "db::actor::block_on(work)\n".repeat(18);
  const goodCli = `${productionCliWaits}#[cfg(test)]\nfn seed_document() {\n  db::actor::block_on(work)\n}`;
  if (interactivityDbCliFailures(goodCli).length !== 0) throw new Error("[verify interactivity] DB CLI self-test retained census was falsely rejected.");
  const extraWaitCli = `${productionCliWaits}db::actor::block_on(work)\n#[cfg(test)]\nfn seed_document() {\n  db::actor::block_on(work)\n}`;
  if (interactivityDbCliFailures(extraWaitCli).length === 0) throw new Error("[verify interactivity] DB CLI self-test 19th production wait was falsely accepted.");
  const productionSeedCli = `${productionCliWaits}fn seed_document() {\n  db::actor::block_on(work)\n}`;
  if (interactivityDbCliFailures(productionSeedCli).length === 0) throw new Error("[verify interactivity] DB CLI self-test production seed_document was falsely accepted.");
  const missingSeedWaitCli = `${productionCliWaits}#[cfg(test)]\nfn seed_document() {\n  drop(work)\n}`;
  if (interactivityDbCliFailures(missingSeedWaitCli).length === 0) throw new Error("[verify interactivity] DB CLI self-test missing test-only seed_document wait was falsely accepted.");
}
