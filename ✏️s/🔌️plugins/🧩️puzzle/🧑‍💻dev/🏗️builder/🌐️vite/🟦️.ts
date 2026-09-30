import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createFrameworkOsDevConfig } from "../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts";
import contribution from "../../🧬️schema/🔣️.json";
const workspace = fileURLToPath(new URL("../../../../../../", import.meta.url));
export default createFrameworkOsDevConfig({ brands: [], browserEntry: "/@fs/" + resolve(workspace, contribution.browserEntry).replaceAll("\\", "/") });
