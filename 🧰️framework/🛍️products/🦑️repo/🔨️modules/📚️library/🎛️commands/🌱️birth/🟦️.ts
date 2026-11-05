import { rootDevMcpFreshCapabilitiesV1 } from "../🛠️dev/🌱️birth/🟦️.ts";
import { rootTestFreshCapabilitiesV1 } from "../🧪️test/🌱️birth/🟦️.ts";

/** 🧭️ Composes owned command birth declarations only for genuinely fresh invocations. */
export async function rootCommandFreshCapabilitiesV1(root: string, argv: readonly string[], environment: Record<string, string | undefined>): Promise<Readonly<Record<string, unknown>>> {
  return await rootDevMcpFreshCapabilitiesV1(root, argv, environment) ?? await rootTestFreshCapabilitiesV1(root, argv, environment);
}
