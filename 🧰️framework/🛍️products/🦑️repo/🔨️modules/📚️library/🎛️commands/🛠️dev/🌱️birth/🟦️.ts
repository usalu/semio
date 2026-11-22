import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

const protocol = schema.$defs.Protocol.const;

/** 🌱️ Declares only the native gateway and explicit catalog descendants of the canonical OS MCP launch. */
export function rootDevMcpBuildRequestsV1(argv: readonly string[]): readonly (readonly string[])[] | null {
  if (!protocol.prefix.every((word, index) => argv[index] === word) || argv.some(word => protocol.helpFlags.includes(word))) return null;
  const targets = [protocol.buildTarget], at = argv.indexOf(protocol.catalogFlag, protocol.prefix.length);
  if (at >= 0) {
    const target = argv[at + 1];
    if (argv.indexOf(protocol.catalogFlag, at + 1) >= 0 || validateJsonSchemaSubset(schema.$defs.Target, target).length) throw Error("--catalog-target requires one declared build target");
    targets.push(target!);
  }
  return Object.freeze([...new Set(targets)].map(target => Object.freeze(["run", target, protocol.outputFlag])));
}

/** 🎟️ Issues native resource DATA and exact descendant declarations before the canonical dev invocation is born. */
export async function rootDevMcpFreshCapabilitiesV1(root: string, argv: readonly string[], environment: Record<string, string | undefined>): Promise<Readonly<Record<string, unknown>> | null> {
  const requests = rootDevMcpBuildRequestsV1(argv);
  if (!requests) return null;
  const { issueRepositoryScriptProvidersV1 } = await import("../../../🏃️process/🔌️providers/🟦️.ts");
  const { createBuildBootstrapCapabilitiesV1 } = await import("../../../🏗️build/📥️caller/📤️child/🟦️.ts");
  const { createNativeBuildCallerCapabilitiesV1 } = await import("../../../🏗️build/📥️caller/🟦️.ts");
  issueRepositoryScriptProvidersV1(environment);
  const descendants = await Promise.all(requests.map(request => createBuildBootstrapCapabilitiesV1(root, request, environment)));
  const first = descendants[0]!, original = createNativeBuildCallerCapabilitiesV1(root, argv, first.tool, { ...environment, SEMIO_BUILD_MODE: first.buildMode });
  const launches = new Map(original.launches.map(launch => [JSON.stringify(launch), launch]));
  for (const child of descendants) {
    if (child.tool !== original.tool || child.buildMode !== original.buildMode || JSON.stringify(child.native) !== JSON.stringify(original.native)) throw Error("Same original native gateway resources required");
    for (const launch of child.launches) launches.set(JSON.stringify(launch), launch);
  }
  return Object.freeze({ ...original, launches: Object.freeze([...launches.values()]) });
}

type PlaygroundServeRequest = Readonly<{ variant: string; renderer: "react" | "wgpu" | "native"; profile: "prototype" | "release" }>;

/** 🎮️ Admits only the complete root request for the playground owner. */
export function rootDevPlaygroundRequestV1(argv: readonly string[]): PlaygroundServeRequest | null {
  const { prefix, operands } = schema.$defs.PlaygroundProtocol.const;
  if (argv.length !== operands || !prefix.every((word, index) => argv[index] === word)) return null;
  const [, , variant, renderer, profile] = argv;
  const request = { variant, renderer, profile };
  if (validateJsonSchemaSubset(schema.$defs.PlaygroundRequest, request).length) return null;
  return Object.freeze(request as PlaygroundServeRequest);
}

/** 🌱️ Declares exactly the root playground request as Native caller DATA without planning a build graph. */
export async function rootDevPlaygroundFreshCapabilitiesV1(root: string, argv: readonly string[], environment: NodeJS.ProcessEnv): Promise<Readonly<Record<string, unknown>> | null> {
  const request = rootDevPlaygroundRequestV1(argv);
  if (!request) return null;
  if (environment.SEMIO_BUILD_MODE !== undefined && environment.SEMIO_BUILD_MODE !== request.profile) {
    throw Error("Playground profile disagrees with the requested build mode");
  }
  const [{ createNativeBuildCallerCapabilitiesV1 }, { parseBuildInvocationV1 }] = await Promise.all([
    import("../../../🏗️build/📥️caller/🟦️.ts"),
    import("../../../🏗️build/🧭️invocation/🟦️.ts"),
  ]);
  const { tool } = parseBuildInvocationV1([], environment);
  const capabilities = createNativeBuildCallerCapabilitiesV1(root, argv, tool, { ...environment, SEMIO_BUILD_MODE: request.profile });
  const { generatedPrerequisiteArgumentsV1 } = await import("../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🏭️generated-owner/🟦️.ts");
  const launches = [...capabilities.launches, ...generatedPrerequisiteArgumentsV1(root).map(child => ["repo", ...child])];
  return Object.freeze({ ...capabilities, launches });
}
