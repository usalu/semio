//#region 🔌️Adapters
import { loadConfigFromFile } from "vite";
import type { OwnedTestProjectConfig } from "../../../../../../🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️build-tooling.ts";
//#endregion 🔌️Adapters

//#region 🗄️Configuration
/** @emoji 🧪️ Loads one test project through the temporary Vite implementation without leaking its types. */
export async function loadOwnedTestProjectConfig(file: string, root: string): Promise<OwnedTestProjectConfig | null> {
  const loaded = await loadConfigFromFile({ command: "serve", mode: "test" }, file, root);
  return loaded ? (loaded.config as OwnedTestProjectConfig) : null;
}
//#endregion 🗄️Configuration
