import { registerStateStoreTests } from "./🗂️state-stores/🟦️.ts";
import { registerPlaygroundActionAndPluginSourceTests } from "./🧭️playground-actions-and-plugin-sources/🟦️.ts";
import { registerMachineTests } from "./🎰️machines/🟦️.ts";
import { registerAsyncHelperTests } from "./⏱️async-helpers/🟦️.ts";

type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: typeof import("@semio-tech/framework"), source: TestSource): Promise<void> {
  await registerStateStoreTests(vitest, dependencies);
  await registerPlaygroundActionAndPluginSourceTests(vitest, dependencies);
  await registerMachineTests(vitest, dependencies);
  await registerAsyncHelperTests(vitest, dependencies);
}

if (import.meta.vitest) {
  const framework = await import("@semio-tech/framework");
  await registerTests1(import.meta.vitest, framework, { directory: import.meta.dir, url: import.meta.url });
}
