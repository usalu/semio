/** 🧠 Owned resident pool, metadata, scope, payload, page, reader, builder, copied range, evidence and binding laws. */
import { type TypedWireVitest, type TypedWireDependencies } from "../🪛️typedwire-helpers/🟦️.tsx";
import { createResidentTestKit } from "./🧰️resident-test-kit/🟦️.tsx";
import { registerTypedWireSuite } from "./🧾️typedwire-suite/🟦️.tsx";

export async function registerResidentPoolAndMetadataTests(vitest: TypedWireVitest, dependencies: TypedWireDependencies): Promise<void> {
  const kit = await createResidentTestKit(vitest, dependencies);
  //#region 🧾️TypedWireTests
  registerTypedWireSuite(kit);
  //#endregion 🧾️TypedWireTests
}
