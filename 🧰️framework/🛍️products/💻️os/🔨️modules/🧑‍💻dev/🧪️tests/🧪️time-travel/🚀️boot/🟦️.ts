import {readBuildMode} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type {DevServeSpawnRequestV1} from "../../../🚀️local-hub/🏃️execution/🟦️.ts";
import schema from "../../../🧬️schema/🔣️.json" with {type:"json"};

export type TimeTravelPlaygroundV1 = Readonly<{ variant: string; aliases: readonly string[]; ports: Readonly<{ react: number; wgpu: number }>; compositionConfigPath?: string }>;
export type TimeTravelServeSelectionV1 = Readonly<{ universal: boolean; url: string; renderer: "react" | "wgpu"; explicit?: string; scenarioVariant?: string }>;

/** 🚀️ Selects a declared playground for a history serve: the scenario's own for a specialized journey, else the one a universal route, explicit variant or catalog port names. */
export function timeTravelServeVariantV1(selection: TimeTravelServeSelectionV1, catalog: readonly TimeTravelPlaygroundV1[]): string {
  const admit = (name: string) => {
    const rows = catalog.filter(row => row.variant === name || row.aliases.includes(name));
    if (rows.length !== 1) throw Error(`A uniquely declared playground is required: ${name}`);
    return rows[0]!.variant;
  };
  if (!selection.universal) {
    if (selection.scenarioVariant === undefined) throw Error("A specialized journey names the playground it drives");
    return admit(selection.scenarioVariant);
  }
  const url = new URL(selection.url);
  const plugin = url.searchParams.get("plugin");
  const route = plugin === null ? undefined : admit(plugin);
  const explicit = selection.explicit === undefined ? undefined : admit(selection.explicit);
  if (route !== undefined && explicit !== undefined && route !== explicit) throw Error("The route and explicit playground disagree");
  if (explicit !== undefined || route !== undefined) return explicit ?? route!;
  const port = Number(url.port || (url.protocol === "https:" ? 443 : 80));
  const rows = catalog.filter(row => row.ports[selection.renderer] === port);
  if (rows.length !== 1) throw Error("A universal serve needs a declared route plugin, variant, or catalog port");
  return rows[0]!.variant;
}


/** 🛎️ Projects one admitted catalog row and issued build intent into the actual local serve contract. */
export function timeTravelServeRequestV1(selection: TimeTravelServeSelectionV1 & Readonly<{profile:unknown;port:number;logPath:string}>, catalog: readonly TimeTravelPlaygroundV1[]): DevServeSpawnRequestV1 {
  const profile = readBuildMode(selection.profile);
  const variant = timeTravelServeVariantV1(selection, catalog);
  const rows = catalog.filter(row => row.variant === variant);
  if (rows.length !== 1) throw Error("One declared time-travel playground is required");
  const compositionConfigPath = rows[0]!.compositionConfigPath;
  const request: DevServeSpawnRequestV1 = {
    variant, renderer: selection.renderer, profile, port: selection.port, hubUrl: null, logPath: selection.logPath,
    ...(compositionConfigPath === undefined ? {} : {compositionConfigPath}),
  };
  const errors = validateJsonSchemaSubset(schema.$defs.DevServeSpawnRequestV1, request);
  if (errors.length) throw Error("Declared time-travel serve request refused: " + errors.join("; "));
  return Object.freeze(request);
}
