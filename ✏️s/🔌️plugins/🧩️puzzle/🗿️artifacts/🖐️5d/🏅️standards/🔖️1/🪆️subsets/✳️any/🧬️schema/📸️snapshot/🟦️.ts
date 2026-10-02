/** 🧩️ Exact persisted Puzzle5d fields shared by its artifact facade and SQLite provider. */
import { parseBinary64, type Binary64 } from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";

export interface ArtifactDialect { artifactKind: string; standard: string; subset: string }
export interface ArtifactRef { artifactId: string; dialect: ArtifactDialect }
export interface ArtifactChildHandle { childId: string; target: ArtifactRef }
export type Puzzle5dPartAnchor = "fixed" | "derived";
export type Puzzle5dCompatSpecificity = "general" | "part" | "fastener" | "grip" | "rope";
export type Puzzle5dVector3 = [Binary64, Binary64, Binary64];
export type Puzzle5dVector4 = [Binary64, Binary64, Binary64, Binary64];
export type Puzzle5dScale = Binary64 | Puzzle5dVector3;
export interface Puzzle5dMeta { description: string }
export interface Puzzle5dPart2d { x: Binary64; y: Binary64; shape: string | null; radius: Binary64 | null; width: Binary64 | null; height: Binary64 | null; text: string | null; iconKind: string | null; hidden: boolean | null; locked: boolean | null }
export interface Puzzle5dPart3d { origin: Puzzle5dVector3; meshUrl: string | null; orientation: Puzzle5dVector4 | null; scale: Puzzle5dScale | null; label: string | null }
export interface Puzzle5dGrip2d { angle: Binary64; gripKind: string | null; radius: Binary64 | null }
export interface Puzzle5dGrip3d { position: Puzzle5dVector3; direction: Puzzle5dVector3 | null; radius: Binary64 | null; label: string | null }
export interface Puzzle5dGrip { id: string; gripKind: string | null; "2d": Puzzle5dGrip2d; "3d": Puzzle5dGrip3d }
export interface Puzzle5dPart { id: string; partKind: string | null; anchor: Puzzle5dPartAnchor; "2d": Puzzle5dPart2d; "3d": Puzzle5dPart3d; grips: Puzzle5dGrip[] }
export interface Puzzle5dFastener { id: string; source: string; target: string; fastenerKind: string | null; gap: Binary64; shift: Binary64; rise: Binary64; rotation: Binary64; turn: Binary64; tilt: Binary64; x: Binary64; y: Binary64 }
export interface Puzzle5dTargetVolume { id: string; origin: Puzzle5dVector3; orientation: Puzzle5dVector4 | null; scale: Puzzle5dScale | null; hidden: boolean; locked: boolean }
export interface Puzzle5dKindCompatibility { source: string; target: string; bidirectional: boolean; important: boolean; specificity: Puzzle5dCompatSpecificity }
export interface Puzzle5dAttribute { id: string; key: string; value: string; definition: string | null }
export interface Puzzle5dAuthor { id: string; name: string; email: string; role: string | null; rank: number | null }
export interface Puzzle5dRepresentation { id: string; name: string; url: string; mime: string; tags: string[]; lod: string | null; description: string }
export interface Puzzle5dGripTemplate { id: string; name: string; label: string; description: string; icon: string; gripKind: string | null; point: Puzzle5dVector3; direction: Puzzle5dVector3; t: Binary64 | null; mandatory: boolean | null; radius: Binary64 | null }
export interface Puzzle5dCatalogPartKind { id: string; name: string; label: string; description: string; icon: string; image: string; unit: string; abstract: boolean; baseKinds: string[]; representations: Puzzle5dRepresentation[]; grips: Puzzle5dGripTemplate[]; attributes: Puzzle5dAttribute[]; authors: Puzzle5dAuthor[] }
export interface Puzzle5dCatalogGripKind { id: string; code: string | null; label: string | null; order: number | null; compatibleWith: string[]; description: string; icon: string; color: string; defaultRopeKind: string }
export interface Puzzle5dCatalogFastenerKind { id: string; name: string; label: string | null }
export interface Puzzle5dCatalogRopeKind { id: string; name: string; label: string; defaultFastenerKind: string }
export interface Puzzle5dKindCatalogs { parts: Puzzle5dCatalogPartKind[]; grips: Puzzle5dCatalogGripKind[]; fasteners: Puzzle5dCatalogFastenerKind[]; ropes: Puzzle5dCatalogRopeKind[] }
export interface Puzzle5dCatalogPartKindExtra extends Puzzle5dCatalogPartKind {}
export interface Puzzle5dCatalogGripKindExtra extends Puzzle5dCatalogGripKind {}
export interface Puzzle5dCatalogFastenerKindExtra extends Puzzle5dCatalogFastenerKind {}
export interface Puzzle5dCatalogRopeKindExtra extends Puzzle5dCatalogRopeKind {}
export interface Puzzle5dKindCatalogsExtra { parts: Puzzle5dCatalogPartKindExtra[]; grips: Puzzle5dCatalogGripKindExtra[]; fasteners: Puzzle5dCatalogFastenerKindExtra[]; ropes: Puzzle5dCatalogRopeKindExtra[] }
export interface Puzzle5dSnapshot { schema: string; domain: string; label: string | null; meta: Puzzle5dMeta; kindCatalogs: ArtifactChildHandle | null; kindCatalogsExtra: Puzzle5dKindCatalogsExtra | null; kindCompatibility: Puzzle5dKindCompatibility[]; parts: Puzzle5dPart[]; fasteners: Puzzle5dFastener[]; targetVolumes: Puzzle5dTargetVolume[] }

/** 🛂️ Report the literal persisted field that failed admission. */
export class Puzzle5dGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) { super(at + ": " + why); }
}
const fail = (at: string, why: string): never => { throw new Puzzle5dGuardRefusal(at, why); };
const object = (value: unknown, at: string): Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value) ? value as Record<string, unknown> : fail(at, "requires an object");
const text = (value: unknown, at: string): string => typeof value === "string" ? value : fail(at, "requires text");
const bool = (value: unknown, at: string): boolean => typeof value === "boolean" ? value : fail(at, "requires a boolean");
const int32 = (value: unknown, at: string): number => typeof value === "number" && Number.isInteger(value) && value >= -2147483648 && value <= 2147483647 ? value : fail(at, "requires a signed32 integer");
const word = (value: unknown, at: string): Binary64 => { try { return parseBinary64(value); } catch { return fail(at, "requires an owned binary64 word"); } };
const optional = <T>(value: unknown, at: string, parse: (value: unknown, at: string) => T): T | null => value === null ? null : parse(value, at);
const list = <T>(value: unknown, at: string, parse: (value: unknown, at: string) => T): T[] => Array.isArray(value) ? value.map((item, i) => parse(item, at + "[" + i + "]")) : fail(at, "requires an array");
const member = <T extends string>(value: unknown, at: string, values: readonly T[]): T => values.includes(value as T) ? value as T : fail(at, "requires one of " + values.join(", "));
function vector3(value: unknown, at: string): Puzzle5dVector3 {
  if (!Array.isArray(value) || value.length !== 3) return fail(at, "requires three binary64 words");
  return [word(value[0], at + "[0]"), word(value[1], at + "[1]"), word(value[2], at + "[2]")];
}
function vector4(value: unknown, at: string): Puzzle5dVector4 {
  if (!Array.isArray(value) || value.length !== 4) return fail(at, "requires four binary64 words");
  return [word(value[0], at + "[0]"), word(value[1], at + "[1]"), word(value[2], at + "[2]"), word(value[3], at + "[3]")];
}
/** 📐️ Preserve the native scalar and vector scale branches. */
export function parsePuzzle5dScale(value: unknown, at = "$"): Puzzle5dScale { return Array.isArray(value) ? vector3(value, at) : word(value, at); }
/** 🧭️ Admit the independently persisted three dialect strings. */
export function parseArtifactDialect(value: unknown, at = "$"): ArtifactDialect {
  const r = object(value, at);
  return { artifactKind: text(r.artifactKind, at + ".artifactKind"), standard: text(r.standard, at + ".standard"), subset: text(r.subset, at + ".subset") };
}
/** 🪪️ Admit a literal target identity without URI normalization. */
export function parseArtifactRef(value: unknown, at = "$"): ArtifactRef {
  const r = object(value, at);
  return { artifactId: text(r.artifactId, at + ".artifactId"), dialect: parseArtifactDialect(r.dialect, at + ".dialect") };
}
/** 🌉️ Keep local child identity independent from its persisted target. */
export function parseArtifactChildHandle(value: unknown, at = "$"): ArtifactChildHandle {
  const r = object(value, at);
  return { childId: text(r.childId, at + ".childId"), target: parseArtifactRef(r.target, at + ".target") };
}
/** 📝️ Admit the native description string. */
export function parsePuzzle5dMeta(value: unknown, at = "$"): Puzzle5dMeta { const r = object(value, at); return { description: text(r.description, at + ".description") }; }
/** ◻️ Admit the complete native board placement. */
export function parsePuzzle5dPart2d(value: unknown, at = "$"): Puzzle5dPart2d {
  const r = object(value, at);
  return { x: word(r.x, at + ".x"), y: word(r.y, at + ".y"), shape: optional(r.shape, at + ".shape", text), radius: optional(r.radius, at + ".radius", word), width: optional(r.width, at + ".width", word), height: optional(r.height, at + ".height", word), text: optional(r.text, at + ".text", text), iconKind: optional(r.iconKind, at + ".iconKind", text), hidden: optional(r.hidden, at + ".hidden", bool), locked: optional(r.locked, at + ".locked", bool) };
}
/** 🧊️ Admit the complete native world placement. */
export function parsePuzzle5dPart3d(value: unknown, at = "$"): Puzzle5dPart3d {
  const r = object(value, at);
  return { origin: vector3(r.origin, at + ".origin"), meshUrl: optional(r.meshUrl, at + ".meshUrl", text), orientation: optional(r.orientation, at + ".orientation", vector4), scale: optional(r.scale, at + ".scale", parsePuzzle5dScale), label: optional(r.label, at + ".label", text) };
}
/** 🔘️ Admit the native grip board fields. */
export function parsePuzzle5dGrip2d(value: unknown, at = "$"): Puzzle5dGrip2d {
  const r = object(value, at);
  return { angle: word(r.angle, at + ".angle"), gripKind: optional(r.gripKind, at + ".gripKind", text), radius: optional(r.radius, at + ".radius", word) };
}
/** 📍️ Admit the native grip world fields. */
export function parsePuzzle5dGrip3d(value: unknown, at = "$"): Puzzle5dGrip3d {
  const r = object(value, at);
  return { position: vector3(r.position, at + ".position"), direction: optional(r.direction, at + ".direction", vector3), radius: optional(r.radius, at + ".radius", word), label: optional(r.label, at + ".label", text) };
}
/** 🧲️ Admit an independently ordered grip occurrence. */
export function parsePuzzle5dGrip(value: unknown, at = "$"): Puzzle5dGrip {
  const r = object(value, at);
  return { id: text(r.id, at + ".id"), gripKind: optional(r.gripKind, at + ".gripKind", text), "2d": parsePuzzle5dGrip2d(r["2d"], at + ".2d"), "3d": parsePuzzle5dGrip3d(r["3d"], at + ".3d") };
}
/** ⚓️ Admit the native anchor branch. */
export function parsePuzzle5dPartAnchor(value: unknown, at = "$"): Puzzle5dPartAnchor { return member(value, at, ["fixed", "derived"]); }
/** 🧱️ Admit the complete part and its ordered owned grips. */
export function parsePuzzle5dPart(value: unknown, at = "$"): Puzzle5dPart {
  const r = object(value, at);
  return { id: text(r.id, at + ".id"), partKind: optional(r.partKind, at + ".partKind", text), anchor: parsePuzzle5dPartAnchor(r.anchor, at + ".anchor"), "2d": parsePuzzle5dPart2d(r["2d"], at + ".2d"), "3d": parsePuzzle5dPart3d(r["3d"], at + ".3d"), grips: list(r.grips, at + ".grips", parsePuzzle5dGrip) };
}
/** 🔗️ Admit all eight exact native transform fields. */
export function parsePuzzle5dFastener(value: unknown, at = "$"): Puzzle5dFastener {
  const r = object(value, at);
  return { id: text(r.id, at + ".id"), source: text(r.source, at + ".source"), target: text(r.target, at + ".target"), fastenerKind: optional(r.fastenerKind, at + ".fastenerKind", text), gap: word(r.gap, at + ".gap"), shift: word(r.shift, at + ".shift"), rise: word(r.rise, at + ".rise"), rotation: word(r.rotation, at + ".rotation"), turn: word(r.turn, at + ".turn"), tilt: word(r.tilt, at + ".tilt"), x: word(r.x, at + ".x"), y: word(r.y, at + ".y") };
}
/** 🧊️ Admit the actual target-volume value without spatial restrictions. */
export function parsePuzzle5dTargetVolume(value: unknown, at = "$"): Puzzle5dTargetVolume {
  const r = object(value, at);
  return { id: text(r.id, at + ".id"), origin: vector3(r.origin, at + ".origin"), orientation: optional(r.orientation, at + ".orientation", vector4), scale: optional(r.scale, at + ".scale", parsePuzzle5dScale), hidden: bool(r.hidden, at + ".hidden"), locked: bool(r.locked, at + ".locked") };
}
/** 🔖️ Admit the native compatibility specificity branch. */
export function parsePuzzle5dCompatSpecificity(value: unknown, at = "$"): Puzzle5dCompatSpecificity { return member(value, at, ["general", "part", "fastener", "grip", "rope"]); }
/** 🔁️ Admit an ordered native compatibility row. */
export function parsePuzzle5dKindCompatibility(value: unknown, at = "$"): Puzzle5dKindCompatibility {
  const r = object(value, at);
  return { source: text(r.source, at + ".source"), target: text(r.target, at + ".target"), bidirectional: bool(r.bidirectional, at + ".bidirectional"), important: bool(r.important, at + ".important"), specificity: parsePuzzle5dCompatSpecificity(r.specificity, at + ".specificity") };
}
/** 🏷️ Admit the actual attribute fields. */
export function parsePuzzle5dAttribute(value: unknown, at = "$"): Puzzle5dAttribute {
  const r = object(value, at);
  return { id: text(r.id, at + ".id"), key: text(r.key, at + ".key"), value: text(r.value, at + ".value"), definition: optional(r.definition, at + ".definition", text) };
}
/** ✍️ Admit native author fields and optional signed32 rank. */
export function parsePuzzle5dAuthor(value: unknown, at = "$"): Puzzle5dAuthor {
  const r = object(value, at);
  return { id: text(r.id, at + ".id"), name: text(r.name, at + ".name"), email: text(r.email, at + ".email"), role: optional(r.role, at + ".role", text), rank: optional(r.rank, at + ".rank", int32) };
}
/** 🖼️ Admit ordered representations and literal tags. */
export function parsePuzzle5dRepresentation(value: unknown, at = "$"): Puzzle5dRepresentation {
  const r = object(value, at);
  return { id: text(r.id, at + ".id"), name: text(r.name, at + ".name"), url: text(r.url, at + ".url"), mime: text(r.mime, at + ".mime"), tags: list(r.tags, at + ".tags", text), lod: optional(r.lod, at + ".lod", text), description: text(r.description, at + ".description") };
}
/** 🌱️ Admit the complete native grip template. */
export function parsePuzzle5dGripTemplate(value: unknown, at = "$"): Puzzle5dGripTemplate {
  const r = object(value, at);
  return { id: text(r.id, at + ".id"), name: text(r.name, at + ".name"), label: text(r.label, at + ".label"), description: text(r.description, at + ".description"), icon: text(r.icon, at + ".icon"), gripKind: optional(r.gripKind, at + ".gripKind", text), point: vector3(r.point, at + ".point"), direction: vector3(r.direction, at + ".direction"), t: optional(r.t, at + ".t", word), mandatory: optional(r.mandatory, at + ".mandatory", bool), radius: optional(r.radius, at + ".radius", word) };
}
/** 🧱️ Admit every explicitly owned part-kind field and ordered collection. */
export function parsePuzzle5dCatalogPartKind(value: unknown, at = "$"): Puzzle5dCatalogPartKind {
  const r = object(value, at);
  return { id: text(r.id, at + ".id"), name: text(r.name, at + ".name"), label: text(r.label, at + ".label"), description: text(r.description, at + ".description"), icon: text(r.icon, at + ".icon"), image: text(r.image, at + ".image"), unit: text(r.unit, at + ".unit"), abstract: bool(r.abstract, at + ".abstract"), baseKinds: list(r.baseKinds, at + ".baseKinds", text), representations: list(r.representations, at + ".representations", parsePuzzle5dRepresentation), grips: list(r.grips, at + ".grips", parsePuzzle5dGripTemplate), attributes: list(r.attributes, at + ".attributes", parsePuzzle5dAttribute), authors: list(r.authors, at + ".authors", parsePuzzle5dAuthor) };
}
/** 🔘️ Admit every explicitly owned grip-kind field. */
export function parsePuzzle5dCatalogGripKind(value: unknown, at = "$"): Puzzle5dCatalogGripKind {
  const r = object(value, at);
  return { id: text(r.id, at + ".id"), code: optional(r.code, at + ".code", text), label: optional(r.label, at + ".label", text), order: optional(r.order, at + ".order", int32), compatibleWith: list(r.compatibleWith, at + ".compatibleWith", text), description: text(r.description, at + ".description"), icon: text(r.icon, at + ".icon"), color: text(r.color, at + ".color"), defaultRopeKind: text(r.defaultRopeKind, at + ".defaultRopeKind") };
}
/** 🔗️ Admit the native fastener-kind fields. */
export function parsePuzzle5dCatalogFastenerKind(value: unknown, at = "$"): Puzzle5dCatalogFastenerKind {
  const r = object(value, at);
  return { id: text(r.id, at + ".id"), name: text(r.name, at + ".name"), label: optional(r.label, at + ".label", text) };
}
/** 🧵️ Admit the native rope-kind fields. */
export function parsePuzzle5dCatalogRopeKind(value: unknown, at = "$"): Puzzle5dCatalogRopeKind {
  const r = object(value, at);
  return { id: text(r.id, at + ".id"), name: text(r.name, at + ".name"), label: text(r.label, at + ".label"), defaultFastenerKind: text(r.defaultFastenerKind, at + ".defaultFastenerKind") };
}
/** 🗂️ Admit the actual mutation catalog payload, distinct from the persisted child. */
export function parsePuzzle5dKindCatalogs(value: unknown, at = "$"): Puzzle5dKindCatalogs {
  const r = object(value, at);
  return { parts: list(r.parts, at + ".parts", parsePuzzle5dCatalogPartKind), grips: list(r.grips, at + ".grips", parsePuzzle5dCatalogGripKind), fasteners: list(r.fasteners, at + ".fasteners", parsePuzzle5dCatalogFastenerKind), ropes: list(r.ropes, at + ".ropes", parsePuzzle5dCatalogRopeKind) };
}
/** 🧩️ Admit the puzzle-owned part-kind overflow with the same actual native fields. */
export function parsePuzzle5dCatalogPartKindExtra(value: unknown, at = "$"): Puzzle5dCatalogPartKindExtra { return parsePuzzle5dCatalogPartKind(value, at); }
/** 🧩️ Admit the puzzle-owned grip-kind overflow. */
export function parsePuzzle5dCatalogGripKindExtra(value: unknown, at = "$"): Puzzle5dCatalogGripKindExtra { return parsePuzzle5dCatalogGripKind(value, at); }
/** 🧩️ Admit the puzzle-owned fastener-kind overflow. */
export function parsePuzzle5dCatalogFastenerKindExtra(value: unknown, at = "$"): Puzzle5dCatalogFastenerKindExtra { return parsePuzzle5dCatalogFastenerKind(value, at); }
/** 🧩️ Admit the puzzle-owned rope-kind overflow. */
export function parsePuzzle5dCatalogRopeKindExtra(value: unknown, at = "$"): Puzzle5dCatalogRopeKindExtra { return parsePuzzle5dCatalogRopeKind(value, at); }
/** 🗃️ Admit the independent native catalog overflow collections. */
export function parsePuzzle5dKindCatalogsExtra(value: unknown, at = "$"): Puzzle5dKindCatalogsExtra {
  const r = object(value, at);
  return { parts: list(r.parts, at + ".parts", parsePuzzle5dCatalogPartKindExtra), grips: list(r.grips, at + ".grips", parsePuzzle5dCatalogGripKindExtra), fasteners: list(r.fasteners, at + ".fasteners", parsePuzzle5dCatalogFastenerKindExtra), ropes: list(r.ropes, at + ".ropes", parsePuzzle5dCatalogRopeKindExtra) };
}
/** 📸️ Admit all ten persisted fields without native file defaults or normalization. */
export function parsePuzzle5dSnapshot(value: unknown, at = "$"): Puzzle5dSnapshot {
  const r = object(value, at);
  return { schema: text(r.schema, at + ".schema"), domain: text(r.domain, at + ".domain"), label: optional(r.label, at + ".label", text), meta: parsePuzzle5dMeta(r.meta, at + ".meta"), kindCatalogs: optional(r.kindCatalogs, at + ".kindCatalogs", parseArtifactChildHandle), kindCatalogsExtra: optional(r.kindCatalogsExtra, at + ".kindCatalogsExtra", parsePuzzle5dKindCatalogsExtra), kindCompatibility: list(r.kindCompatibility, at + ".kindCompatibility", parsePuzzle5dKindCompatibility), parts: list(r.parts, at + ".parts", parsePuzzle5dPart), fasteners: list(r.fasteners, at + ".fasteners", parsePuzzle5dFastener), targetVolumes: list(r.targetVolumes, at + ".targetVolumes", parsePuzzle5dTargetVolume) };
}
