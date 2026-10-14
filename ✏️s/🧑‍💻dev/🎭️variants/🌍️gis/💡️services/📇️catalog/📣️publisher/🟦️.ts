import type {ScriptInvocation} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import type { DevHubCatalogPublisherV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts";

/** 📣️ Binds the services composition to the real Hub catalog producer. */
export function serviceHubCatalogPublisher(invocation:ScriptInvocation,repoRoot: string): DevHubCatalogPublisherV1 {
  return (dataDir, packages, signal, onLine) => {
    if(signal!==invocation.control.signal)throw Error("Original catalog publication signal required");
    return runRepositoryCommand(invocation,process.execPath, ["repo", "run", "os-hub:trusted-catalog-bootstrap", "--skip-cache", ...(packages ? ["--", "--packages", packages] : [])], repoRoot, "services-hub-catalog", {version:1,maximumElapsedMilliseconds:3600000,output:{maximumBytes:134217728,maximumLines:65536}}, { env: { ...process.env, OS_HUB_DATA: dataDir }, onLine });
  };
}
