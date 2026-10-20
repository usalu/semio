/** 🔔️ Directory client stream: scoped directory workflows over a fake web socket, socket grants and reconnects.
 *
 * @see ../🧮️directory-client-helpers/🟦️.ts — the shared helpers
 * @see ../../🟦️.ts — the package entry the suite reads fixtures relative to
 */
import { describe } from "vitest";
import { registerRoundTripsScopedDirectoryTests } from "./🔶️round-trips-scoped-directory/🟦️.ts";
import { registerKeepsInstalledServiceStatusTests } from "./🔸️keeps-installed-service-status/🟦️.ts";
import { registerDropAfterSustainedHealthTests } from "./🟠️drop-after-sustained-health/🟦️.ts";

describe("DirectoryClient.stream", () => {
  registerRoundTripsScopedDirectoryTests();
  registerKeepsInstalledServiceStatusTests();
  registerDropAfterSustainedHealthTests();
});
