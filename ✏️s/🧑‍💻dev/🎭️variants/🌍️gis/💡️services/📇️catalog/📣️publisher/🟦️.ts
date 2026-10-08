import { runRepositoryCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import type { DevHubCatalogPublisherV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts";

/** 📣️ Binds the services composition to the real Hub catalog producer. */
export function serviceHubCatalogPublisher(repoRoot: string): DevHubCatalogPublisherV1 {
  return (dataDir, packages, signal, onLine) => runRepositoryCommand(process.execPath, ["nx", "run", "os-hub:trusted-catalog-bootstrap", "--skip-nx-cache", ...(packages ? ["--", "--packages", packages] : [])], repoRoot, "services-hub-catalog", 3_600_000, { env: { ...process.env, OS_HUB_DATA: dataDir }, signal, onLine });
}
