/** 🎨️ Typologies: specs, display styles, construct kits, and the attribute and property definitions that apply to them. */
import { typologyStyleCache, modelDefinitionTypologyCatalog, modelDefinitionAttributeCatalog, modelDefinitionPropertyCatalog, type TypologyStylePatternKind, type TypologyStylePatternSpec, type ResolvedTypologyStyle, type TypologyConstructKit, typologyConstructKitByInteractionCache } from "../📥️definition-catalog/🟦️.ts";
import type { ModelEntityKind, TypologyPrimitiveKind } from "../🟦️.ts";

/** 🎨️ Optional authored display style on a typology asset. */
export interface TypologyStyleSpec {
  readonly color?: string;
  readonly edgeColor?: string;
  readonly opacity?: number;
  readonly pattern?: TypologyStylePatternSpec;
}

/** 🏷️ Parsed typology asset (`spatial.typology/v1`). */
export interface TypologySpec {
  readonly schema: "spatial.typology";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly primitiveKinds: readonly TypologyPrimitiveKind[];
  readonly actions: readonly string[];
  readonly interactions: readonly string[];
  readonly properties?: readonly string[];
  readonly attributes?: readonly string[];
  readonly style?: TypologyStyleSpec;
}

/** 🧭️ Infers default `primitiveKinds` from a shipped typology id when the asset omits the field. */
export function inferTypologyPrimitiveKinds(typology: string): readonly TypologyPrimitiveKind[] {
  const id = typology.toLowerCase();
  if (id.includes(".selection.") || id.includes(".command.")) return [];
  if (id.includes(".measure.") && id.includes("volume")) return [];
  if (id.includes(".entity.") || id.includes("create-anchor")) return ["anchor"];
  if (id.includes(".measure.")) return ["anchor"];
  if (id.includes(".curve.") || id.includes(".linefem") || id.includes("lineelement")) return ["curve"];
  if (id.includes(".surface.") || id.includes("surfacefem") || id.includes("surfaceelement")) return ["surface"];
  if (id.includes(".primitive.") || id.includes(".solid.") || id.includes("solidelement")) return ["solid"];
  if (id.includes(".feature.extrude")) return ["solid"];
  if (id.includes(".feature.offset")) return ["surface", "solid"];
  if (id.includes(".transform.") || id.includes(".edit.")) return ["solid", "surface", "curve"];
  if (id.includes(".beam") || id.includes(".railing")) return ["curve"];
  if (id.includes(".column") || id.includes(".stair") || id.includes(".hull")) return ["solid"];
  if (id.includes("wall") || id.includes("slab") || id.includes(".roof") || id.includes("baseplate") || id.includes("ceiling") || id.includes("foundation") || id.includes(".door") || id.includes("window")) {
    return ["surface"];
  }
  return ["solid"];
}

function parseTypologyPrimitiveKinds(raw: unknown, typology: string): readonly TypologyPrimitiveKind[] {
  if (!Array.isArray(raw) || raw.length === 0) return inferTypologyPrimitiveKinds(typology);
  const allowed = new Set<TypologyPrimitiveKind>(["anchor", "solid", "surface", "curve"]);
  const kinds: TypologyPrimitiveKind[] = [];
  for (const entry of raw) {
    if (typeof entry !== "string") continue;
    const k = entry as TypologyPrimitiveKind;
    if (allowed.has(k)) kinds.push(k);
  }
  return kinds.length ? kinds : inferTypologyPrimitiveKinds(typology);
}

/** 🧭️ Maps a standalone typology primitive kind to the kernel entity kind used for picking. */
export function typologyPrimitiveToEntityKind(kind: TypologyPrimitiveKind): ModelEntityKind {
  if (kind === "surface") return "face";
  if (kind === "curve") return "wire";
  return kind;
}

const TYPOLOGY_STYLE_PATTERN_KINDS = new Set<TypologyStylePatternKind>(["none", "hatch", "crosshatch", "dots"]);

function parseTypologyStylePatternSpec(raw: unknown): TypologyStylePatternSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  const kind = r.kind;
  if (typeof kind !== "string" || !TYPOLOGY_STYLE_PATTERN_KINDS.has(kind as TypologyStylePatternKind)) return null;
  return {
    kind: kind as TypologyStylePatternKind,
    direction: typeof r.direction === "number" ? r.direction : undefined,
    spacing: typeof r.spacing === "number" ? r.spacing : undefined,
    lineWidth: typeof r.lineWidth === "number" ? r.lineWidth : undefined,
    color: typeof r.color === "string" ? r.color : undefined,
  };
}

function parseTypologyStyleSpec(raw: unknown): TypologyStyleSpec | undefined {
  if (!raw || typeof raw !== "object") return undefined;
  const r = raw as Record<string, unknown>;
  const pattern = r.pattern !== undefined ? parseTypologyStylePatternSpec(r.pattern) : undefined;
  if (r.pattern !== undefined && !pattern) return undefined;
  return {
    color: typeof r.color === "string" ? r.color : undefined,
    edgeColor: typeof r.edgeColor === "string" ? r.edgeColor : undefined,
    opacity: typeof r.opacity === "number" ? r.opacity : undefined,
    pattern: pattern ?? undefined,
  };
}

/** 🧾️ Parses `spatial.typology/v1` JSON or returns `null`. */
export function parseTypologySpec(raw: unknown): TypologySpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.typology") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  const actions = Array.isArray(r.actions) ? r.actions.filter((row): row is string => typeof row === "string") : [];
  const interactions = Array.isArray(r.interactions) ? r.interactions.filter((row): row is string => typeof row === "string") : [];
  return {
    schema: "spatial.typology",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    primitiveKinds: parseTypologyPrimitiveKinds(r.primitiveKinds, r.id),
    actions,
    interactions,
    properties: Array.isArray(r.properties) ? (r.properties as string[]) : undefined,
    attributes: Array.isArray(r.attributes) ? (r.attributes as string[]) : undefined,
    style: parseTypologyStyleSpec(r.style),
  };
}

// #region 🎨️TypologyStyle
function hashTypologyId(typology: string): number {
  let h = 2166136261;
  for (let i = 0; i < typology.length; i++) {
    h ^= typology.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  return h >>> 0;
}

function hslToHex(h: number, s: number, l: number): string {
  const hue = ((h % 360) + 360) % 360;
  const sat = Math.min(1, Math.max(0, s));
  const lit = Math.min(1, Math.max(0, l));
  const c = (1 - Math.abs(2 * lit - 1)) * sat;
  const x = c * (1 - Math.abs(((hue / 60) % 2) - 1));
  const m = lit - c / 2;
  let r = 0;
  let g = 0;
  let b = 0;
  if (hue < 60) {
    r = c;
    g = x;
  } else if (hue < 120) {
    r = x;
    g = c;
  } else if (hue < 180) {
    g = c;
    b = x;
  } else if (hue < 240) {
    g = x;
    b = c;
  } else if (hue < 300) {
    r = x;
    b = c;
  } else {
    r = c;
    b = x;
  }
  const toByte = (v: number) =>
    Math.round((v + m) * 255)
      .toString(16)
      .padStart(2, "0");
  return `#${toByte(r)}${toByte(g)}${toByte(b)}`;
}

function darkenHexColor(hex: string, amount = 0.28): string {
  const raw = hex.replace("#", "");
  if (raw.length !== 6) return hex;
  const r = Number.parseInt(raw.slice(0, 2), 16);
  const g = Number.parseInt(raw.slice(2, 4), 16);
  const b = Number.parseInt(raw.slice(4, 6), 16);
  const scale = 1 - amount;
  const toByte = (v: number) =>
    Math.round(v * scale)
      .toString(16)
      .padStart(2, "0");
  return `#${toByte(r)}${toByte(g)}${toByte(b)}`;
}

function autoTypologyStyle(typology: string): ResolvedTypologyStyle {
  const hash = hashTypologyId(typology);
  const hue = (hash * 137.508) % 360;
  const color = hslToHex(hue, 0.58, 0.52);
  return {
    color,
    edgeColor: darkenHexColor(color, 0.32),
    opacity: 0.72,
    pattern: { kind: "none", direction: 0, spacing: 0.35, lineWidth: 0.03, color: darkenHexColor(color, 0.18) },
  };
}

function mergeTypologyStyle(typology: string, authored?: TypologyStyleSpec): ResolvedTypologyStyle {
  const base = autoTypologyStyle(typology);
  const pattern = authored?.pattern;
  const fill = authored?.color ?? base.color;
  return {
    color: fill,
    edgeColor: authored?.edgeColor ?? darkenHexColor(fill, 0.32),
    opacity: authored?.opacity ?? base.opacity,
    pattern: {
      kind: pattern?.kind ?? base.pattern.kind,
      direction: pattern?.direction ?? base.pattern.direction,
      spacing: pattern?.spacing ?? base.pattern.spacing,
      lineWidth: pattern?.lineWidth ?? base.pattern.lineWidth,
      color: pattern?.color ?? darkenHexColor(fill, 0.18),
    },
  };
}

/** 🎨️ Stable cache key for renderer material/pattern reuse. */
export function typologyStyleCacheKey(style: ResolvedTypologyStyle): string {
  const p = style.pattern;
  return `${style.color}|${style.edgeColor}|${style.opacity}|${p.kind}|${p.direction}|${p.spacing}|${p.lineWidth}|${p.color}`;
}

/** 🎨️ Resolves display style for a typology (deterministic auto fallback + optional asset override). */
export function resolveTypologyStyle(typology: string): ResolvedTypologyStyle {
  if (typologyStyleCache.current?.has(typology)) return typologyStyleCache.current.get(typology)!;
  const authored = loadTypology(typology)?.style;
  const resolved = mergeTypologyStyle(typology, authored);
  if (!typologyStyleCache.current) typologyStyleCache.current = new Map();
  typologyStyleCache.current.set(typology, resolved);
  return resolved;
}
// #endregion 🎨️TypologyStyle

export function shippedTypologyCatalog(): readonly TypologySpec[] {
  return dedupeDefinitionCatalog(
    modelDefinitionTypologyCatalog()
      .map((raw) => parseTypologySpec(raw))
      .filter((spec): spec is TypologySpec => spec !== null),
  );
}

/** 📚️ Lists typologies from shipped spatial/asset/modelDefinition assets. */
export function listModelDefinitionTypologies(): readonly TypologySpec[] {
  return shippedTypologyCatalog();
}

/** 📚️ Loads a model-definition typology by stable `id`. */
export function loadTypology(typology: string): TypologySpec | null {
  return shippedTypologyCatalog().find((t) => t.id === typology) ?? null;
}

/** 📚️ Resolves the typology whose `interactions` list includes `interactionId`. */
export function typologyForInteraction(interactionId: string): TypologySpec | null {
  return shippedTypologyCatalog().find((t) => t.interactions.some((id) => id === interactionId)) ?? null;
}

/** 🏷️ PascalCase object name from a typology label (`External Wall` → `ExternalWall`). */
function typologyObjectPascalFromLabel(label: string): string {
  return label
    .replace(/[^a-zA-Z0-9]+/g, " ")
    .trim()
    .split(/\s+/)
    .filter(Boolean)
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join("");
}

/** 🧭️ Stable ids: three `construct*From*` actions and one `construct*` interaction. */
function typologyConstructAssetIds(typology: string, label: string): TypologyConstructKit & { readonly construct: string } {
  const parts = typology.split(".");
  const prefix = parts.length > 1 ? `${parts.slice(0, -1).join(".")}.` : "";
  const pascal = typologyObjectPascalFromLabel(label);
  const interaction = `${prefix}construct${pascal}`;
  return {
    typology,
    interaction,
    construct: interaction,
    constructFrom2PointsAndHeight: `${prefix}construct${pascal}From2PointsAndHeight`,
    constructFromCurveAndHeight: `${prefix}construct${pascal}FromCurveAndHeight`,
    constructFromSurface: `${prefix}construct${pascal}FromSurface`,
  };
}

/** 🧭️ Maps each typology construct interaction id to its mode actions (not the interaction id). */
export function typologyConstructKitByInteraction(): ReadonlyMap<string, TypologyConstructKit> {
  if (typologyConstructKitByInteractionCache.current) return typologyConstructKitByInteractionCache.current;
  const map = new Map<string, TypologyConstructKit>();
  for (const typology of listModelDefinitionTypologies()) {
    const ids = typologyConstructAssetIds(typology.id, typology.label);
    map.set(ids.interaction, {
      typology: ids.typology,
      interaction: ids.interaction,
      constructFrom2PointsAndHeight: ids.constructFrom2PointsAndHeight,
      constructFromCurveAndHeight: ids.constructFromCurveAndHeight,
      constructFromSurface: ids.constructFromSurface,
    });
  }
  typologyConstructKitByInteractionCache.current = map;
  return map;
}

/** 🏷️ Parsed attribute definition (`spatial.attribute/v1`). */
export interface AttributeDefinitionSpec {
  readonly schema: "spatial.attribute";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly field: string;
  readonly targets: readonly string[];
  readonly value: unknown;
  readonly geometrySelector?: { readonly kinds: readonly string[] };
}

/** 🧾️ Parses `spatial.attribute/v1` JSON or returns `null`. */
export function parseAttributeDefinitionSpec(raw: unknown): AttributeDefinitionSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.attribute") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  if (typeof r.field !== "string" || !Array.isArray(r.targets) || r.targets.length === 0) return null;
  if (!("value" in r)) return null;
  const selector = r.geometrySelector;
  const geometrySelector = selector && typeof selector === "object" && Array.isArray((selector as { kinds?: unknown }).kinds) ? { kinds: (selector as { kinds: string[] }).kinds } : undefined;
  return {
    schema: "spatial.attribute",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    field: r.field,
    targets: r.targets as string[],
    value: r.value,
    geometrySelector,
  };
}

/** 🏷️ Parsed property definition (`spatial.property/v1`). */
export interface PropertyDefinitionSpec {
  readonly schema: "spatial.property";
  readonly id: string;
  readonly version: string;
  readonly label: string;
  readonly description?: string;
  readonly unit?: string;
  readonly sources?: Readonly<Record<string, unknown>>;
  readonly output?: Readonly<Record<string, unknown>>;
}

/** 🧾️ Parses `spatial.property/v1` JSON or returns `null`. */
export function parsePropertyDefinitionSpec(raw: unknown): PropertyDefinitionSpec | null {
  if (!raw || typeof raw !== "object") return null;
  const r = raw as Record<string, unknown>;
  if (r.schema !== "spatial.property") return null;
  if (typeof r.id !== "string" || typeof r.version !== "string" || typeof r.label !== "string") return null;
  return {
    schema: "spatial.property",
    id: r.id,
    version: r.version,
    label: r.label,
    description: typeof r.description === "string" ? r.description : undefined,
    unit: typeof r.unit === "string" ? r.unit : undefined,
    sources: r.sources && typeof r.sources === "object" ? (r.sources as Record<string, unknown>) : undefined,
    output: r.output && typeof r.output === "object" ? (r.output as Record<string, unknown>) : undefined,
  };
}

export function dedupeDefinitionCatalog<T extends { readonly id: string }>(rows: readonly T[]): readonly T[] {
  const seen = new Set<string>();
  const out: T[] = [];
  for (const row of rows) {
    if (seen.has(row.id)) continue;
    seen.add(row.id);
    out.push(row);
  }
  return out;
}

export function shippedAttributeDefinitionCatalog(): readonly AttributeDefinitionSpec[] {
  return dedupeDefinitionCatalog(
    modelDefinitionAttributeCatalog()
      .map((raw) => parseAttributeDefinitionSpec(raw))
      .filter((spec): spec is AttributeDefinitionSpec => spec !== null),
  );
}

export function shippedPropertyDefinitionCatalog(): readonly PropertyDefinitionSpec[] {
  return dedupeDefinitionCatalog(
    modelDefinitionPropertyCatalog()
      .map((raw) => parsePropertyDefinitionSpec(raw))
      .filter((spec): spec is PropertyDefinitionSpec => spec !== null),
  );
}

/** 📚️ Lists attribute definitions from model-definition assets. */
export function listModelDefinitionAttributeDefinitions(): readonly AttributeDefinitionSpec[] {
  return shippedAttributeDefinitionCatalog();
}

/** 📚️ Lists property definitions from model-definition assets. */
export function listModelDefinitionPropertyDefinitions(): readonly PropertyDefinitionSpec[] {
  return shippedPropertyDefinitionCatalog();
}

/** 📚️ Loads an attribute definition by stable `id`. */
export function loadAttributeDefinition(attributeId: string): AttributeDefinitionSpec | null {
  return shippedAttributeDefinitionCatalog().find((row) => row.id === attributeId) ?? null;
}

/** 📚️ Loads a property definition by stable `id`. */
export function loadPropertyDefinition(propertyId: string): PropertyDefinitionSpec | null {
  return shippedPropertyDefinitionCatalog().find((row) => row.id === propertyId) ?? null;
}
