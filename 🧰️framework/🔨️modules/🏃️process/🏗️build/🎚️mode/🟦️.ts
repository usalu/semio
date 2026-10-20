import modeSchema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🏗️ Owns the portable compiler intent admitted by the build-mode schema. */
export type BuildMode = "prototype" | "release";

/** 🎛️ Enumerates every admitted compiler intent without exposing private schema storage. */
export function buildModesV1(): readonly BuildMode[] {
  const values: unknown = modeSchema.enum;
  if (!Array.isArray(values)) throw Error("Owned build mode schema enumeration required");
  return Object.freeze(values.map(readBuildMode));
}

/** 🎚️ Selects one build intent without accepting aliases or implicit optimization. */
export function readBuildMode(value: unknown): BuildMode {
  if (value !== "prototype" && value !== "release") throw Error("Build mode must be prototype or release");
  return value;
}

/** ⚙️ Maps compiler intent to native, browser, and managed tool policies. */
export function buildModePolicy(mode: BuildMode) {
  const optimize = readBuildMode(mode) === "release";
  return Object.freeze({
    cargoProfile: optimize ? "release" as const : "prototype" as const,
    cmakeConfiguration: optimize ? "Release" as const : "Debug" as const,
    dotnetConfiguration: optimize ? "Release" as const : "Debug" as const,
    optimize,
    sourceMap: !optimize,
  });
}

/** 🌱 Carries admitted compiler intent through an isolated child environment. */
export function buildModeEnvironment(mode: BuildMode, environment: Readonly<Record<string, string | undefined>> = process.env): Record<string, string | undefined> {
  return { ...environment, SEMIO_BUILD_MODE: readBuildMode(mode) };
}
