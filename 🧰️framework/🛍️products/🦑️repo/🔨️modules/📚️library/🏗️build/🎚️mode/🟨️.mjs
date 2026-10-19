import { buildModesV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";

/** 🎯️ Fixed preparation and release targets retain the mode encoded by their workflow. */
export function fixedTargetBuildMode(name) {
  if (/^(?:prepare|activate|serve|dev|run|smoke|native-build|wasm|build|mcp)(?:-|$)/.test(name) && /-release$/.test(name)) return "release";
  if (/^(?:prepare|activate|serve|dev|run|smoke|native-build|wasm|build|mcp)(?:-|$)/.test(name) && /-prototype$/.test(name) || /^(?:prepare|activate|serve|dev|run|smoke)(?:-|$)/.test(name) && /-dev$/.test(name) || ["dev", "serve", "prepare-e2e", "serve-e2e"].includes(name)) return "prototype";
  return undefined;
}

/** 🧬️ Every configured dependency can inherit the initiating task's compiler mode. */
export function targetBuildModes(name, target) {
  const buildModes = buildModesV1();
  const fixed = fixedTargetBuildMode(name), configurations = { ...target.configurations };
  for (const mode of buildModes) configurations[mode] = { ...configurations[mode], env: { ...target.options?.env, ...configurations[mode]?.env, SEMIO_BUILD_MODE: mode } };
  const inputs = [...(target.inputs ?? ["default", "^default"])];
  if (!inputs.some(input => input?.env === "SEMIO_BUILD_MODE")) inputs.push({ env: "SEMIO_BUILD_MODE" });
  return { ...target, defaultConfiguration: fixed ?? "prototype", configurations, ...(target.cache ? { inputs } : {}), metadata: { ...target.metadata, semio: { ...target.metadata?.semio, buildMode: { ...(fixed ? { fixed } : {}), modes: buildModes } } } };
}
