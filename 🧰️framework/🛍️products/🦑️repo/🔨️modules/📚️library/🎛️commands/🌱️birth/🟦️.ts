import { rootDevMcpFreshCapabilitiesV1, rootDevPlaygroundFreshCapabilitiesV1, rootDevPlaygroundRequestV1 } from "../🛠️dev/🌱️birth/🟦️.ts";
import { rootTestFreshCapabilitiesV1 } from "../🧪️test/🌱️birth/🟦️.ts";

/** 🧭️ Composes owned command birth declarations only for genuinely fresh invocations. */
export async function rootCommandFreshCapabilitiesV1(root: string, argv: readonly string[], environment: Record<string, string | undefined>): Promise<Readonly<Record<string, unknown>>> {
  return await rootDevMcpFreshCapabilitiesV1(root, argv, environment)
    ?? await rootDevPlaygroundFreshCapabilitiesV1(root, argv, environment)
    ?? await rootTestFreshCapabilitiesV1(root, argv, environment);
}

/** 🔁️ Allows a new Original only for standalone, restartable native commands with the same argv. */
export function rootCommandRetriesFreshBuildV1(argv:readonly string[]):boolean {
  return rootDevPlaygroundRequestV1(argv) !== null || argv[0] === "components" && argv[1] === "describe" && argv.length > 2;
}
