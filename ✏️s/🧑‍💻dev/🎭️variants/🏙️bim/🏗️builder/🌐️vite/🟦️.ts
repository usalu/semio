import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { OwnedBuildPlugin } from "@semio-tech/ui-react/build-tooling";
import { specificOsDevBuildInventoryV1 } from "../../../../🧩️catalog/📦️packages/🟦️typescript/🏗️builder/🟦️.ts";
import { createFrameworkOsDevConfig } from "../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🏗️builder/🌐️vite/🟦️.ts";
import { STDIO_FORMAT_CLAIMS_PATH, stdioFormatClaimsTextV1 } from "../../../../../🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧭️format-claims/📜️generate.ts";
import contribution from "../../🧬️schema/🔣️.json" with { type: "json" };

const workspace = fileURLToPath(new URL("../../../../../..", import.meta.url));

/** 🧭️ Supplies the same stdio claims from their authored inputs during BIM browser preparation. */
export function bimStdioFormatClaimsVitePlugin(): OwnedBuildPlugin {
  const source = "./🤖️generated/stdio-format-claims.json";
  const catalog = resolve(workspace, dirname(STDIO_FORMAT_CLAIMS_PATH), "../🟦️.ts").replaceAll("\\", "/");
  const module = "\0bim-stdio-format-claims.json";
  return {
    name: "semio-bim-stdio-format-claims",
    enforce: "pre",
    resolveId(id, importer) {
      if (id === source && importer?.replaceAll("\\", "/") === catalog) return module;
    },
    load(id) {
      if (id === module) return stdioFormatClaimsTextV1(workspace);
    },
  };
}

const config = createFrameworkOsDevConfig({
  inventory: specificOsDevBuildInventoryV1(),
  brands: [],
  browserSessionFactories: [],
  browserEntry: "/@fs/" + resolve(workspace, contribution.browserEntry).replaceAll("\\", "/"),
});

export default async (...args: Parameters<typeof config>) => {
  const resolved = await config(...args);
  return { ...resolved, plugins: [bimStdioFormatClaimsVitePlugin(), ...(resolved.plugins ?? [])] };
};
