import { buildModeEnvironment, readBuildMode } from "../../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";

/** 🎚️ Carries original compiler intent or selects prototype for the public schema producer. */
export function schemaCompilerEnvironmentV1(environment: Readonly<Record<string, string | undefined>>, selectedMode?: unknown): Record<string, string | undefined> {
  return buildModeEnvironment(readBuildMode(selectedMode ?? environment.SEMIO_BUILD_MODE ?? "prototype"), environment);
}
