/** 🧬️ TypeScript twin of the cast manifest schema: the closed vocabularies, the types and a dependency-free validator that agrees with the JSON Schema.
 * @see ./🔣️.schema.json — the normative schema `semio.staffage.cast/v1`
 * @see ../🟦️.ts — the exporter */
import { VIEWPOINTS, SCALE_DENOMINATORS, type ScaleDenominator, type ViewSection, type Slug, type Viewpoint } from "../../../../🧬️schema/🟦️.ts";
import { TRANSPORT_LAYERS, VIEWS, type TransportLayer, type View } from "../../../🔗️contracts/🟦️.ts";
import type { StructureIndex } from "../../../🏗️structure/🧾️summary/🟦️.ts";

/** 🌗️ The themes of the analytic views. */
export const CAST_THEMES = ["dark", "light"] as const;

/** 🌓️ One of {@link CAST_THEMES}. */
export type CastTheme = (typeof CAST_THEMES)[number];

/** 🆔️ The schema id of the cast manifest. */
export const CAST_SCHEMA_ID = "semio.staffage.cast/v1";

/** 🌤️ One key of a transient environment: a preset slug from time `t` seconds on. */
export type CastTimelineKey = { readonly t: number; readonly environment: Slug; readonly ease: "step" | "linear" };

/** 🎭️ One cast entry: exactly one of `pose` and `clip`. */
export type Cast = {
  readonly asset: Slug;
  readonly pose?: Slug;
  readonly clip?: Slug;
  readonly view: View;
  readonly theme: CastTheme;
  readonly transport?: readonly TransportLayer[];
  readonly transportAgeS?: number;
  readonly transportStrokeMm?: number;
  readonly transportTrailS?: number;
  readonly viewpoint: Viewpoint;
  readonly scale: ScaleDenominator;
  readonly fps?: number;
  readonly environment?: Slug;
  readonly timeline?: readonly CastTimelineKey[];
  readonly thermalSpanK?: number;
  readonly mirror?: boolean;
};

/** 🗃️ The cast manifest. */
export type CastManifest = { readonly version: 1; readonly casts: Readonly<Record<Slug, Cast>> };

/** 🧾️ The index written next to the frames of one cast. `heightM` is the physical height of the figure in metres (the evaluated y extent over all frames) in every viewpoint; the paper extents of the viewpoint are `boundsM` and `figureBoundsM`. */
export type CastIndex = {
  readonly version: 1;
  readonly cast: Slug;
  readonly asset: Slug;
  readonly view: View;
  readonly viewpoint: Viewpoint;
  readonly section?: ViewSection;
  readonly frames: readonly string[];
  readonly fps: number;
  readonly durationS: number;
  readonly loop: boolean;
  readonly mmPerMetre: number;
  readonly originMm: readonly [number, number];
  readonly boundsM: { readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number };
  readonly figureBoundsM?: { readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number };
  readonly heightM: number;
  readonly structure?: StructureIndex;
  readonly provenance: CastProvenance;
};

/** 🔖️ A content hash, `sha256:` and 64 lower-case hex digits. */
export type Revision = `sha256:${string}`;

/** 📤️ The scenario of a cast: a thermal environment (one preset or a timeline of presets), the structural model, or geometry only. */
export type CastScenario =
  | { readonly kind: "geometry" }
  | { readonly kind: "structural"; readonly dynamics: "quasi-static" | "inertial"; readonly gravityMS2: number }
  | { readonly kind: "thermal"; readonly environment: { readonly id: Slug; readonly revision: Revision }; readonly spanK: number | null; readonly transport: readonly TransportLayer[] }
  | { readonly kind: "thermal"; readonly timeline: readonly { readonly t: number; readonly id: Slug; readonly ease: "step" | "linear"; readonly revision: Revision }[]; readonly spanK: number | null; readonly transport: readonly TransportLayer[] };

/** 📏️ Where one rendered cast came from: asset id and revision, view, authored scale, pose or clip, clock and scenario. `clock.rate` is added by a scene; the exporter does not know it.
 * @see ../../../../../../../.🧬semio — ticket r3-contract.md */
export type CastProvenance = {
  readonly version: 1;
  readonly asset: { readonly id: Slug; readonly category: Slug; readonly revision: Revision };
  readonly view: { readonly style: View; readonly viewpoint: Viewpoint; readonly theme: CastTheme; readonly mirror: boolean };
  readonly scale: { readonly denominator: ScaleDenominator };
  readonly state: { readonly kind: "pose" | "clip"; readonly id: Slug };
  readonly clock: { readonly fps: number; readonly durationS: number; readonly loop: boolean; readonly offsetS: number; readonly frames: number; readonly rate?: number };
  readonly scenario: CastScenario;
};

const SLUG = /^[a-z0-9]+(?:-[a-z0-9]+)*$/u;
const KEYS = ["asset", "pose", "clip", "view", "viewpoint", "scale", "fps", "environment", "timeline", "thermalSpanK", "mirror", "theme", "transport", "transportAgeS", "transportStrokeMm", "transportTrailS"];
const isSlug = (value: unknown): boolean => typeof value === "string" && value.length <= 64 && SLUG.test(value);
const isObject = (value: unknown): value is Record<string, unknown> => typeof value === "object" && value !== null && !Array.isArray(value);

function castIssues(path: string, cast: unknown): string[] {
  if (!isObject(cast)) return [`${path}: must be an object`];
  const issues: string[] = [];
  for (const key of Object.keys(cast)) if (!KEYS.includes(key)) issues.push(`${path}: unknown property ${key}`);
  for (const key of ["asset", "pose", "clip", "environment"]) if (key in cast && !isSlug(cast[key])) issues.push(`${path}/${key}: must be a slug`);
  if (!("asset" in cast)) issues.push(`${path}: asset is required`);
  if (!("theme" in cast)) issues.push(`${path}: theme is required`);
  else if (!(CAST_THEMES as readonly unknown[]).includes(cast.theme)) issues.push(`${path}/theme: must be one of ${CAST_THEMES.join(", ")}`);
  if ("transport" in cast && !(Array.isArray(cast.transport) && cast.transport.every((layer: unknown) => (TRANSPORT_LAYERS as readonly unknown[]).includes(layer)) && new Set(cast.transport).size === cast.transport.length)) issues.push(`${path}/transport: must be unique layers of ${TRANSPORT_LAYERS.join(", ")}`);
  for (const key of ["transportAgeS", "transportStrokeMm", "transportTrailS"]) {
    if (key in cast && !(typeof cast[key] === "number" && Number.isFinite(cast[key]) && (cast[key] as number) > 0)) issues.push(`${path}/${key}: must be a positive number`);
    if (key in cast && !("transport" in cast)) issues.push(`${path}: ${key} needs transport layers`);
  }
  if ("transportAgeS" in cast && !("clip" in cast)) issues.push(`${path}: transportAgeS needs a clip`);
  if ("transport" in cast && !("transportAgeS" in cast)) issues.push(`${path}: transport layers need transportAgeS, the explicit start of the field`);
  if ("transport" in cast && cast.view !== "thermal") issues.push(`${path}: transport layers exist for the thermal view only`);
  if (!(VIEWS as readonly unknown[]).includes(cast.view)) issues.push(`${path}/view: must be one of ${VIEWS.join(", ")}`);
  if (!(VIEWPOINTS as readonly unknown[]).includes(cast.viewpoint)) issues.push(`${path}/viewpoint: is required and must be one of ${VIEWPOINTS.join(", ")}`);
  if (cast.mirror === true && cast.viewpoint !== "elevation") issues.push(`${path}: mirror needs viewpoint elevation, a mirrored front or plan would fake a view`);
  if (!(SCALE_DENOMINATORS as readonly unknown[]).includes(cast.scale)) issues.push(`${path}/scale: is required and must be one of ${SCALE_DENOMINATORS.join(", ")}`);
  if ("fps" in cast && !(Number.isInteger(cast.fps) && (cast.fps as number) >= 1 && (cast.fps as number) <= 120)) issues.push(`${path}/fps: must be an integer in 1..120`);
  if ("thermalSpanK" in cast && !(typeof cast.thermalSpanK === "number" && Number.isFinite(cast.thermalSpanK) && cast.thermalSpanK > 0)) issues.push(`${path}/thermalSpanK: must be a positive number of kelvin`);
  if ("thermalSpanK" in cast && cast.view !== "thermal") issues.push(`${path}: thermalSpanK exists for the thermal view only`);
  if ("mirror" in cast && typeof cast.mirror !== "boolean") issues.push(`${path}/mirror: must be a boolean`);
  if (("pose" in cast) === ("clip" in cast)) issues.push(`${path}: give exactly one of pose and clip`);
  if (("environment" in cast || "timeline" in cast) && cast.view !== "thermal") issues.push(`${path}: environment and timeline exist for the thermal view only`);
  if ("environment" in cast && "timeline" in cast) issues.push(`${path}: environment and timeline exclude each other`);
  if ("fps" in cast && !("clip" in cast || "timeline" in cast)) issues.push(`${path}/fps: needs a clip or a timeline`);
  if (!("fps" in cast) && ("clip" in cast || "timeline" in cast)) issues.push(`${path}/fps: is required for a clip or a timeline`);
  if (cast.view === "thermal" && !("environment" in cast || "timeline" in cast)) issues.push(`${path}: the thermal view needs an environment or a timeline`);
  if ("timeline" in cast) {
    const timeline = cast.timeline;
    if (!Array.isArray(timeline) || timeline.length === 0) issues.push(`${path}/timeline: must be a non-empty array`);
    else
      timeline.forEach((key: unknown, index) => {
        const at = `${path}/timeline/${index}`;
        if (!isObject(key)) return void issues.push(`${at}: must be an object`);
        for (const name of Object.keys(key)) if (!["t", "environment", "ease"].includes(name)) issues.push(`${at}: unknown property ${name}`);
        if (!(typeof key.t === "number" && key.t >= 0)) issues.push(`${at}/t: must be a number >= 0`);
        if (!isSlug(key.environment)) issues.push(`${at}/environment: must be a slug`);
        if (key.ease !== "step" && key.ease !== "linear") issues.push(`${at}/ease: is required and must be step or linear`);
      });
  }
  return issues;
}

/** ✅️ Every violation of the cast manifest schema as `path: message`; empty when the document is valid. */
export function castManifestIssues(document: unknown): readonly string[] {
  if (!isObject(document)) return ["/: must be an object"];
  const issues: string[] = [];
  for (const key of Object.keys(document)) if (key !== "version" && key !== "casts") issues.push(`/: unknown property ${key}`);
  if (document.version !== 1) issues.push("/version: must be 1");
  if (!isObject(document.casts)) return [...issues, "/casts: must be an object"];
  for (const [id, cast] of Object.entries(document.casts)) {
    if (!isSlug(id)) issues.push(`/casts/${id}: the cast id must be a slug`);
    issues.push(...castIssues(`/casts/${id}`, cast));
  }
  return issues;
}

/** 📥️ Returns the document as a {@link CastManifest} or throws one error naming every violation. */
export function parseCastManifest(document: unknown): CastManifest {
  const issues = castManifestIssues(document);
  if (issues.length > 0) throw new Error(`invalid cast manifest\n${issues.join("\n")}`);
  return document as CastManifest;
}
