/** 🗂️ Semantic validation of staffage assets and of the manifest that plans them.
 *
 * The JSON Schema (`🧬️schema/🔣️.json`) checks shape; this module checks meaning: unique ids, resolvable references, a single-root rig, joint limits, clip timing, bilingual texts and — new in v2 —
 * the representation decisions of every preset scale (authored pens, decisions and valid ranges; unavailable decisions with resolvable alternatives; shapes bound to existing bones, regions, hatches
 * and materials; every thermal region drawn at every authored scale; the default-pose extent of every authored drawing against `dimensions`), symbol semantics, exhaustive pose thermal bindings,
 * respiration consistency and the sums of the body metabolic shares and skin fractions. Every finding is an {@link Issue} with a JSON-path-like location, a stable code and an `{ en, de }` message.
 *
 * @see ../../🧬️schema/🔣️.json — the normative structural contract
 * @see ../🧮️evaluation/🟦️.ts — `evaluate`, used for the extent and plant checks
 */
import { CATEGORIES, PARAMETER_SOURCES, SCALE_DENOMINATORS } from "../../🧬️schema/🟦️.ts";
import type { Asset, AuthoredRepresentation, BoneOffset, Manifest, Shape, Slug, SymbolRepresentation, Text } from "../../🧬️schema/🟦️.ts";
import { sampleClip } from "../🎞️animation/🟦️.ts";
import { evaluate } from "../🧮️evaluation/🟦️.ts";
import { compareRepresentations } from "../🖨️representation/🪞️copies/🟦️.ts";
import { effectiveOffset } from "../🦴️rig/🟦️.ts";

/** 🚨️ One validation finding. */
export type Issue = { readonly path: string; readonly code: string; readonly message: Text };

/** 📏️ Relative tolerance of the default-pose extent of every authored representation against `dimensions` (2 %). */
export const EXTENT_TOLERANCE = 0.02;

/** 📏️ Absolute tolerance of the same comparison in real metres (15 mm); the larger of the two applies. */
export const EXTENT_TOLERANCE_M = 0.015;

/** 📏️ Allowed deviation of manifest heights and widths from the asset dimensions (5 % and 10 %). */
export const HEIGHT_TOLERANCE = 0.05;
export const WIDTH_TOLERANCE = 0.1;

/** 🔁️ Allowed mismatch in metres between `advanceM` and the planted root travel of one cycle. */
export const ADVANCE_TOLERANCE_M = 0.002;

/** ➕️ Allowed deviation of the body metabolic shares, skin fractions and breathing phase fractions from 1. */
export const SUM_TOLERANCE = 0.02;
export const PHASE_TOLERANCE = 1e-3;

/** 🌐️ What a single asset cannot know alone: the ids of the other assets, to resolve `asset` alternatives. */
export type AssetContext = { readonly assets?: ReadonlySet<Slug> };

const REGION_FIELDS = ["exposedFraction", "sunlitFraction", "wettedness"];
const ANGLE_EPS = 1e-9;
const PAPER_MIN_MM = 0.03;
const PAPER_MAX_PEN_MM = 2;
const PAPER_MAX_SPACING_MM = 20;
const DECISION_KEYS = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"] as const;
const PEN_ROLES = ["body", "detail", "texture", "glazing", "ground"] as const;
const SYMBOL_UNITS = { "true-extent": "model-m", "true-position": "paper-mm", nominal: "paper-mm" } as const;

function text(en: string, de: string): Text {
  return { en, de };
}

function duplicates(ids: readonly string[]): readonly string[] {
  const seen = new Set<string>();
  const repeated = new Set<string>();
  for (const id of ids) (seen.has(id) ? repeated : seen).add(id);
  return [...repeated];
}

function textIssues(value: unknown, path: string, out: Issue[]): void {
  if (Array.isArray(value)) value.forEach((item, index) => textIssues(item, `${path}[${index}]`, out));
  else if (typeof value === "object" && value !== null) {
    const record = value as Record<string, unknown>;
    const keys = Object.keys(record);
    if (keys.length > 0 && keys.length <= 2 && keys.every((key) => key === "en" || key === "de")) {
      for (const language of ["en", "de"] as const) {
        const entry = record[language];
        if (typeof entry !== "string" || entry.trim() === "") out.push({ path: `${path}.${language}`, code: "text-empty", message: text(`Text needs a non-empty English and German version; "${language}" is missing or empty.`, `Text braucht eine nicht leere englische und deutsche Fassung; "${language}" fehlt oder ist leer.`) });
      }
    } else for (const [key, entry] of Object.entries(record)) textIssues(entry, `${path}.${key}`, out);
  }
}

function jointIssues(asset: Asset, offsets: Readonly<Record<Slug, BoneOffset>> | undefined, path: string, out: Issue[]): void {
  for (const [id, offset] of Object.entries(offsets ?? {})) {
    const bone = asset.rig.bones.find((candidate) => candidate.id === id);
    if (bone === undefined) continue;
    const joint = bone.joint;
    const effective = effectiveOffset(bone, offset);
    const bad = (en: string, de: string): void => void out.push({ path: `${path}.${id}`, code: "joint-limit", message: text(en, de) });
    if (joint.type === "fixed" && ((offset.angleDeg ?? 0) !== 0 || (offset.dx ?? 0) !== 0 || (offset.dy ?? 0) !== 0)) bad(`Bone "${id}" has a fixed joint and cannot be offset.`, `Knochen "${id}" hat ein festes Gelenk und kann nicht versetzt werden.`);
    else if (joint.type === "revolute" && Math.abs((offset.angleDeg ?? 0) - effective.angleDeg) > ANGLE_EPS) bad(`Offset ${offset.angleDeg} deg of bone "${id}" leaves its limits ${joint.minDeg ?? "-inf"}..${joint.maxDeg ?? "inf"}.`, `Versatz ${offset.angleDeg} Grad von Knochen "${id}" verlässt die Grenzen ${joint.minDeg ?? "-inf"}..${joint.maxDeg ?? "inf"}.`);
    else if (joint.type === "prismatic" && Math.hypot((offset.dx ?? 0) - effective.dx, (offset.dy ?? 0) - effective.dy) > 1e-9) bad(`Travel of bone "${id}" leaves its limits ${joint.minM ?? "-inf"}..${joint.maxM ?? "inf"} m or its axis.`, `Verschiebung von Knochen "${id}" verlässt die Grenzen ${joint.minM ?? "-inf"}..${joint.maxM ?? "inf"} m oder die Achse.`);
  }
}

function mergeSpans(spans: readonly { readonly fromS: number; readonly toS: number }[]): { from: number; to: number }[] {
  const merged: { from: number; to: number }[] = [];
  for (const span of [...spans].sort((p, q) => p.fromS - q.fromS)) {
    const last = merged[merged.length - 1];
    if (last !== undefined && span.fromS <= last.to + 1e-9) last.to = Math.max(last.to, span.toS);
    else merged.push({ from: span.fromS, to: span.toS });
  }
  return merged;
}

/** ✅️ Every semantic problem of an asset; an empty list means the asset is valid. `context.assets` lets `asset` alternatives of unavailable representations resolve. */
export function validateAsset(source: Asset, context: AssetContext = {}): readonly Issue[] {
  const out: Issue[] = [];
  const add = (path: string, code: string, en: string, de: string): void => void out.push({ path, code, message: text(en, de) });
  const missing = (value: unknown, path: string): boolean => {
    if (value !== undefined) return false;
    add(path, "required-missing", `"${path}" is required by the schema but absent.`, `"${path}" ist laut Schema erforderlich, fehlt aber.`);
    return true;
  };
  const absent = missing((source.thermal as { passengers?: unknown }).passengers, "thermal.passengers");
  const asset: Asset = absent ? { ...source, thermal: { ...source.thermal, passengers: [] } } : source;
  const unique = (path: string, ids: readonly string[]): void => {
    for (const id of duplicates(ids)) add(path, "duplicate-id", `Id "${id}" is used more than once in ${path}.`, `Id "${id}" wird in ${path} mehrfach verwendet.`);
  };
  const need = (path: string, id: string | undefined, known: ReadonlySet<string>, what: string): void => {
    if (id !== undefined && !known.has(id)) add(path, "unresolved-reference", `${what} "${id}" does not exist.`, `${what} "${id}" existiert nicht.`);
  };
  textIssues(asset, "$", out);

  const bones = new Set(asset.rig.bones.map((bone) => bone.id));
  const anchors = new Set(asset.anchors.map((anchor) => anchor.id));
  const materials = new Set(asset.materials.map((material) => material.id));
  const regions = new Set(asset.thermal.regions.map((region) => region.id));
  const emitters = new Set(asset.thermal.emitters.map((emitter) => emitter.id));
  const nonBreath = asset.thermal.emitters.filter((emitter) => emitter.kind !== "breath").map((emitter) => emitter.id);
  const nonBreathSet = new Set(nonBreath);
  const operatingKeys = new Set([...asset.thermal.operating.map((key) => key.id), ...asset.thermal.regions.flatMap((region) => (region.operating === undefined ? [] : [region.operating.key]))]);
  const declaredOperating = new Set(asset.thermal.operating.map((key) => key.id));
  const poses = new Set(asset.poses.map((pose) => pose.id));
  const clips = new Set(asset.clips.map((clip) => clip.id));
  const loads = new Set(asset.poses.flatMap((pose) => pose.loads.map((load) => load.id)));
  const references = new Set(asset.provenance.references.map((reference) => reference.id));
  const symbols = new Set(asset.symbols.map((symbol) => symbol.id));
  const respiration = asset.thermal.respiration;
  const patterns = new Set((respiration?.patterns ?? []).map((pattern) => pattern.id));
  const passengers = new Set(asset.thermal.passengers.map((passenger) => passenger.id));
  const isBody = asset.thermal.body !== undefined || asset.thermal.regions.some((region) => region.model === "body");

  unique("rig.bones", asset.rig.bones.map((bone) => bone.id));
  unique("anchors", asset.anchors.map((anchor) => anchor.id));
  unique("poses", asset.poses.map((pose) => pose.id));
  unique("clips", asset.clips.map((clip) => clip.id));
  unique("materials", asset.materials.map((material) => material.id));
  unique("symbols", asset.symbols.map((symbol) => symbol.id));
  unique("thermal.regions", asset.thermal.regions.map((region) => region.id));
  unique("thermal.emitters", asset.thermal.emitters.map((emitter) => emitter.id));
  unique("thermal.operating", asset.thermal.operating.map((key) => key.id));
  unique("thermal.respiration.patterns", (respiration?.patterns ?? []).map((pattern) => pattern.id));
  unique("thermal.passengers", asset.thermal.passengers.map((passenger) => passenger.id));
  unique("provenance.references", asset.provenance.references.map((reference) => reference.id));
  asset.poses.forEach((pose, index) => unique(`poses[${index}].loads`, pose.loads.map((load) => load.id)));
  asset.thermal.regions.forEach((region, index) => unique(`thermal.regions[${index}].layers`, region.layers.map((layer) => layer.id)));

  const roots = asset.rig.bones.filter((bone) => bone.parent === null);
  if (!bones.has(asset.rig.root)) add("rig.root", "rig-root", `Rig root "${asset.rig.root}" is not a bone.`, `Rig-Wurzel "${asset.rig.root}" ist kein Knochen.`);
  if (roots.length !== 1 || roots[0]!.id !== asset.rig.root) add("rig.bones", "rig-multiple-roots", "The rig must be a single tree whose only parentless bone is the root.", "Das Rig muss ein einzelner Baum sein, dessen einziger Knochen ohne Eltern die Wurzel ist.");
  asset.rig.bones.forEach((bone, index) => {
    if (bone.parent !== null && !bones.has(bone.parent)) add(`rig.bones[${index}].parent`, "rig-missing-parent", `Bone "${bone.id}" has missing parent "${bone.parent}".`, `Knochen "${bone.id}" hat den fehlenden Elternknochen "${bone.parent}".`);
  });
  const byId = new Map(asset.rig.bones.map((bone) => [bone.id, bone]));
  for (const bone of asset.rig.bones) {
    const trail = new Set<Slug>();
    for (let cursor: typeof bone | undefined = bone; cursor !== undefined && cursor.parent !== null; cursor = byId.get(cursor.parent)) {
      if (trail.has(cursor.id)) {
        add("rig.bones", "rig-cycle", `Bone "${bone.id}" is part of a parent cycle.`, `Knochen "${bone.id}" ist Teil eines Eltern-Zyklus.`);
        break;
      }
      trail.add(cursor.id);
    }
  }
  const rigOk = !out.some((issue) => issue.code.startsWith("rig-") || issue.code === "duplicate-id");

  asset.rig.bones.forEach((bone, index) => {
    const path = `rig.bones[${index}]`;
    if (bone.mass !== undefined) {
      if (!(bone.mass.kg >= 0)) add(`${path}.mass.kg`, "mass-negative", `Mass of bone "${bone.id}" must not be negative.`, `Masse von Knochen "${bone.id}" darf nicht negativ sein.`);
      if (!PARAMETER_SOURCES.includes(bone.mass.source)) add(`${path}.mass`, "source-missing", `Mass of bone "${bone.id}" needs a source.`, `Masse von Knochen "${bone.id}" braucht eine Quelle.`);
      need(`${path}.mass.reference`, bone.mass.reference, references, "Reference");
    }
    if (bone.section !== undefined) need(`${path}.section.material`, bone.section.material, materials, "Material");
  });
  asset.anchors.forEach((anchor, index) => need(`anchors[${index}].bone`, anchor.bone, bones, "Bone"));
  asset.materials.forEach((material, index) => {
    const path = `materials[${index}]`;
    if (!(material.emissivity > 0 && material.emissivity <= 1)) add(`${path}.emissivity`, "emissivity-range", `Emissivity of "${material.id}" must lie in (0, 1].`, `Emissionsgrad von "${material.id}" muss in (0, 1] liegen.`);
    if (!PARAMETER_SOURCES.includes(material.source)) add(path, "source-missing", `Material "${material.id}" needs a source.`, `Material "${material.id}" braucht eine Quelle.`);
    need(`${path}.reference`, material.reference, references, "Reference");
  });

  const checkShapes = (path: string, list: readonly Shape[], hatchIds: ReadonlySet<Slug>, pens: Record<string, number | null>): void => {
    unique(path, list.map((shape) => shape.id));
    list.forEach((shape, index) => {
      const sp = `${path}[${index}]`;
      need(`${sp}.bone`, shape.bone, bones, "Bone");
      need(`${sp}.region`, shape.region, regions, "Thermal region");
      for (const region of shape.regions ?? []) need(`${sp}.regions`, region, regions, "Thermal region");
      need(`${sp}.material`, shape.material, materials, "Material");
      need(`${sp}.hatch`, shape.hatch, hatchIds, "Hatch");
      if (shape.region !== undefined && shape.regions !== undefined) add(sp, "region-ambiguous", `Shape "${shape.id}" names both region and regions.`, `Form "${shape.id}" nennt sowohl region als auch regions.`);
      if (shape.penMm !== undefined && !(shape.penMm >= PAPER_MIN_MM && shape.penMm <= PAPER_MAX_PEN_MM)) add(sp, "pen-units", `Shape "${shape.id}" penMm ${shape.penMm} is not a plausible paper millimetre value (${PAPER_MIN_MM}…${PAPER_MAX_PEN_MM}).`, `Form "${shape.id}" penMm ${shape.penMm} ist kein plausibler Papiermillimeterwert (${PAPER_MIN_MM}…${PAPER_MAX_PEN_MM}).`);
      if (pens[shape.role] === null && shape.penMm === undefined) add(sp, "shape-role-undrawn", `Shape "${shape.id}" has role "${shape.role}" whose pen is null at this scale; it would never be drawn.`, `Form "${shape.id}" hat die Rolle "${shape.role}", deren Stift bei diesem Maßstab null ist; sie würde nie gezeichnet.`);
    });
  };

  const checkPens = (path: string, pens: AuthoredRepresentation["pens"] | SymbolRepresentation["pens"]): void => {
    for (const role of PEN_ROLES) {
      const pen = pens[role];
      if (pen !== null && !(pen > 0)) add(`${path}.pens.${role}`, "pen-invalid", `Pen "${role}" must be positive or null.`, `Stift "${role}" muss positiv oder null sein.`);
      else if (pen !== null && !(pen >= PAPER_MIN_MM && pen <= PAPER_MAX_PEN_MM)) add(`${path}.pens.${role}`, "pen-units", `Pen "${role}" = ${pen} is not a plausible paper millimetre value (${PAPER_MIN_MM}…${PAPER_MAX_PEN_MM}); pens are paper mm, not model metres.`, `Stift "${role}" = ${pen} ist kein plausibler Papiermillimeterwert (${PAPER_MIN_MM}…${PAPER_MAX_PEN_MM}); Stifte sind Papier-mm, keine Modellmeter.`);
    }
    if (pens.body === null || pens.ground === null) add(`${path}.pens`, "pen-required", "The body and ground pens must be set.", "Die Stifte body und ground müssen gesetzt sein.");
  };

  const extentIssues = (path: string, rep: AuthoredRepresentation): void => {
    const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: rep.denominator } });
    const height = state.bounds.maxY - state.bounds.minY;
    const width = state.bounds.maxX - state.bounds.minX;
    const within = (measured: number, nominal: number): boolean => Math.abs(measured - nominal) <= Math.max(EXTENT_TOLERANCE * nominal, EXTENT_TOLERANCE_M);
    if (!within(height, asset.dimensions.heightM)) add(`${path}.geometry`, "extent-height", `The default-pose height ${height.toFixed(3)} m of the 1:${rep.denominator} drawing differs from dimensions.heightM ${asset.dimensions.heightM} m by more than 2 % or 15 mm.`, `Die Höhe ${height.toFixed(3)} m der Zeichnung 1:${rep.denominator} in der Standardpose weicht um mehr als 2 % oder 15 mm von dimensions.heightM ${asset.dimensions.heightM} m ab.`);
    if (!within(width, asset.dimensions.widthM)) add(`${path}.geometry`, "extent-width", `The default-pose width ${width.toFixed(3)} m of the 1:${rep.denominator} drawing differs from dimensions.widthM ${asset.dimensions.widthM} m by more than 2 % or 15 mm.`, `Die Breite ${width.toFixed(3)} m der Zeichnung 1:${rep.denominator} in der Standardpose weicht um mehr als 2 % oder 15 mm von dimensions.widthM ${asset.dimensions.widthM} m ab.`);
  };

  const representationPending: AuthoredRepresentation[] = [];
  for (const denominator of SCALE_DENOMINATORS) {
    const path = `representations.${denominator}`;
    const rep = (asset.representations as Partial<Record<string, Asset["representations"][keyof Asset["representations"]]>>)[`${denominator}`];
    if (rep === undefined) {
      add(path, "rep-undecided", `The scale 1:${denominator} is not decided: author it or declare it unavailable.`, `Der Maßstab 1:${denominator} ist nicht entschieden: zeichnen oder als nicht verfügbar erklären.`);
      continue;
    }
    if (rep.denominator !== denominator) add(`${path}.denominator`, "rep-key-mismatch", `The representation under key "${denominator}" declares denominator ${rep.denominator}.`, `Die Darstellung unter dem Schlüssel "${denominator}" deklariert den Nenner ${rep.denominator}.`);
    if (rep.status === "unavailable") {
      if (!(rep.limitingSizeM > 0)) add(`${path}.limitingSizeM`, "limiting-size", "An unavailable representation states the real size of its limiting feature in metres (> 0).", "Eine nicht verfügbare Darstellung nennt die reale Größe ihres begrenzenden Merkmals in Metern (> 0).");
      if (rep.alternatives.length === 0) add(`${path}.alternatives`, "alternative-missing", "An unavailable representation needs at least one alternative.", "Eine nicht verfügbare Darstellung braucht mindestens eine Alternative.");
      let resolvable = 0;
      rep.alternatives.forEach((alternative, index) => {
        const ap = `${path}.alternatives[${index}]`;
        if (alternative.kind === "scale") {
          const target = asset.representations[`${alternative.denominator}`];
          if (alternative.denominator === denominator || target?.status !== "authored") add(ap, "alternative-unresolved", `Alternative scale 1:${alternative.denominator} is not an authored representation of this asset.`, `Alternativer Maßstab 1:${alternative.denominator} ist keine gezeichnete Darstellung dieses Assets.`);
          else resolvable++;
        } else if (alternative.kind === "symbol") {
          if (!symbols.has(alternative.symbol)) add(ap, "alternative-unresolved", `Alternative symbol "${alternative.symbol}" does not exist.`, `Alternatives Symbol "${alternative.symbol}" existiert nicht.`);
          else resolvable++;
        } else if (alternative.asset === asset.id || (context.assets !== undefined && !context.assets.has(alternative.asset))) add(ap, "alternative-unresolved", `Alternative asset "${alternative.asset}" does not exist.`, `Alternatives Asset "${alternative.asset}" existiert nicht.`);
        else resolvable++;
      });
      if (rep.alternatives.length > 0 && resolvable === 0) add(`${path}.alternatives`, "alternative-unresolved", "No alternative of this unavailable representation resolves.", "Keine Alternative dieser nicht verfügbaren Darstellung ist auflösbar.");
      continue;
    }
    representationPending.push(rep);
    const { minDenominator, maxDenominator } = rep.validRange;
    if (!(minDenominator <= denominator && denominator <= maxDenominator)) add(`${path}.validRange`, "valid-range", `The valid range ${minDenominator}…${maxDenominator} must contain the own denominator ${denominator}.`, `Der Gültigkeitsbereich ${minDenominator}…${maxDenominator} muss den eigenen Nenner ${denominator} enthalten.`);
    checkPens(path, rep.pens);
    for (const hatch of rep.hatches) if (!(hatch.spacingMm >= PAPER_MIN_MM && hatch.spacingMm <= PAPER_MAX_SPACING_MM && hatch.penMm >= PAPER_MIN_MM && hatch.penMm <= PAPER_MAX_PEN_MM)) add(`${path}.hatches`, "pen-units", `Hatch "${hatch.id}" must be in paper millimetres (spacing ${PAPER_MIN_MM}…${PAPER_MAX_SPACING_MM}, pen ${PAPER_MIN_MM}…${PAPER_MAX_PEN_MM}).`, `Schraffur "${hatch.id}" muss in Papiermillimetern angegeben sein (Abstand ${PAPER_MIN_MM}…${PAPER_MAX_SPACING_MM}, Stift ${PAPER_MIN_MM}…${PAPER_MAX_PEN_MM}).`);
    for (const key of DECISION_KEYS) if (rep.decisions[key] === undefined) add(`${path}.decisions.${key}`, "decision-missing", `The decision "${key}" is missing.`, `Die Entscheidung "${key}" fehlt.`);
    const hatchIds = new Set(rep.hatches.map((hatch) => hatch.id));
    unique(`${path}.hatches`, rep.hatches.map((hatch) => hatch.id));
    if (rep.geometry.elevation.length === 0) add(`${path}.geometry.elevation`, "elevation-empty", "The elevation geometry must not be empty.", "Die Ansichtsgeometrie darf nicht leer sein.");
    checkShapes(`${path}.geometry.elevation`, rep.geometry.elevation, hatchIds, rep.pens as unknown as Record<string, number | null>);
    checkShapes(`${path}.geometry.plan`, rep.geometry.plan ?? [], hatchIds, rep.pens as unknown as Record<string, number | null>);
    const drawn = new Set(rep.geometry.elevation.flatMap((shape) => [...(shape.region === undefined ? [] : [shape.region]), ...(shape.regions ?? [])]));
    for (const region of asset.thermal.regions) if (region.hidden === undefined && !drawn.has(region.id)) add(`${path}.geometry.elevation`, "region-undrawn", `Thermal region "${region.id}" is drawn by no shape of the 1:${denominator} representation (declare \`hidden\` if it is not visible).`, `Thermische Region "${region.id}" wird von keiner Form der Darstellung 1:${denominator} gezeichnet.`);
  }
  const authoredReps = representationPending.slice().sort((p, q) => p.denominator - q.denominator);
  authoredReps.forEach((rep, i) => {
    const next = authoredReps[i + 1];
    if (next !== undefined && rep.validRange.maxDenominator > next.validRange.minDenominator) add(`representations.${rep.denominator}.validRange`, "valid-range-overlap", `The valid range ${rep.validRange.minDenominator}…${rep.validRange.maxDenominator} of 1:${rep.denominator} overlaps ${next.validRange.minDenominator}…${next.validRange.maxDenominator} of 1:${next.denominator}.`, `Der Gültigkeitsbereich ${rep.validRange.minDenominator}…${rep.validRange.maxDenominator} von 1:${rep.denominator} überlappt ${next.validRange.minDenominator}…${next.validRange.maxDenominator} von 1:${next.denominator}.`);
  });
  const coarseFirst = authoredReps.slice().sort((p, q) => q.denominator - p.denominator);
  for (let i = 0; i < coarseFirst.length; i++)
    for (let j = i + 1; j < coarseFirst.length; j++) {
      const similarity = compareRepresentations(coarseFirst[i]!, coarseFirst[j]!);
      if (similarity.kind !== "distinct") add(`representations.${coarseFirst[i]!.denominator}`, "representation-copied", `The drawing 1:${coarseFirst[i]!.denominator} is a copy of 1:${coarseFirst[j]!.denominator} (${similarity.kind}: ${similarity.sharedShapes} of ${similarity.coarseShapes} shapes identical); every scale must be redrawn.`, `Die Zeichnung 1:${coarseFirst[i]!.denominator} ist eine Kopie von 1:${coarseFirst[j]!.denominator} (${similarity.kind}: ${similarity.sharedShapes} von ${similarity.coarseShapes} Formen identisch); jeder Maßstab muss neu gezeichnet werden.`);
    }
  asset.symbols.forEach((symbol, index) => {
    const path = `symbols[${index}]`;
    if (SYMBOL_UNITS[symbol.semantics] !== symbol.units) add(`${path}.units`, "symbol-units", `A symbol of semantics "${symbol.semantics}" must use units "${SYMBOL_UNITS[symbol.semantics]}".`, `Ein Symbol der Semantik "${symbol.semantics}" muss die Einheit "${SYMBOL_UNITS[symbol.semantics]}" verwenden.`);
    checkPens(path, symbol.pens);
    const hatchIds = new Set(symbol.hatches.map((hatch) => hatch.id));
    if (symbol.geometry.elevation.length === 0 && (symbol.geometry.plan ?? []).length === 0) add(`${path}.geometry`, "symbol-empty", "A symbol needs elevation or plan geometry.", "Ein Symbol braucht Ansichts- oder Grundrissgeometrie.");
    for (const shape of [...symbol.geometry.elevation, ...(symbol.geometry.plan ?? [])]) if (shape.bone !== asset.rig.root) add(`${path}.geometry`, "symbol-bone", `Symbol shape "${shape.id}" must be bound to the rig root "${asset.rig.root}" (symbols are static).`, `Symbolform "${shape.id}" muss an die Rig-Wurzel "${asset.rig.root}" gebunden sein (Symbole sind statisch).`);
    checkShapes(`${path}.geometry.elevation`, symbol.geometry.elevation, hatchIds, symbol.pens as unknown as Record<string, number | null>);
    checkShapes(`${path}.geometry.plan`, symbol.geometry.plan ?? [], hatchIds, symbol.pens as unknown as Record<string, number | null>);
  });

  asset.thermal.regions.forEach((region, index) => {
    const path = `thermal.regions[${index}]`;
    need(`${path}.material`, region.material, materials, "Material");
    need(`${path}.reference`, region.reference, references, "Reference");
    region.layers.forEach((layer, at) => {
      need(`${path}.layers[${at}].material`, layer.material, materials, "Material");
      need(`${path}.layers[${at}].reference`, layer.reference, references, "Reference");
      if (!PARAMETER_SOURCES.includes(layer.source)) add(`${path}.layers[${at}]`, "source-missing", `Layer "${layer.id}" needs a source.`, `Schicht "${layer.id}" braucht eine Quelle.`);
    });
    if (!PARAMETER_SOURCES.includes(region.source)) add(path, "source-missing", `Region "${region.id}" needs a source.`, `Region "${region.id}" braucht eine Quelle.`);
    if (!(region.areaM2 > 0) || !(region.characteristicLengthM > 0)) add(path, "region-size", `Region "${region.id}" needs a positive area and characteristic length.`, `Region "${region.id}" braucht positive Fläche und charakteristische Länge.`);
    const required: Readonly<Record<string, "body" | "leaf" | "operating" | "conditioned" | "fixed" | null>> = { body: "body", leaf: "leaf", operating: "operating", conditioned: "conditioned", fixed: "fixed", passive: null };
    const field = required[region.model] ?? null;
    if (field !== null && region[field] === undefined) add(`${path}.${field}`, "thermal-parameter-missing", `Model "${region.model}" of region "${region.id}" needs "${field}".`, `Modell "${region.model}" von Region "${region.id}" braucht "${field}".`);
    if (region.operating !== undefined && !declaredOperating.has(region.operating.key)) add(`${path}.operating.key`, "unresolved-reference", `Operating key "${region.operating.key}" is not declared in thermal.operating.`, `Betriebsschlüssel "${region.operating.key}" ist nicht in thermal.operating deklariert.`);
  });
  const subjects = new Set<Slug | undefined>(asset.thermal.regions.filter((region) => region.body !== undefined).map((region) => region.body!.subject));
  for (const subject of subjects) {
    const group = asset.thermal.regions.filter((region) => region.body !== undefined && region.body.subject === subject);
    const label = subject ?? "primary body";
    const shares = group.reduce((sum, region) => sum + region.body!.metabolicShare, 0);
    const skin = group.reduce((sum, region) => sum + region.body!.skinFraction, 0);
    if (Math.abs(shares - 1) > SUM_TOLERANCE) add("thermal.regions", "metabolic-share", `Metabolic shares of the body regions of ${label} sum to ${shares.toFixed(3)}, not 1.`, `Die Stoffwechselanteile der Körperregionen von ${label} summieren sich zu ${shares.toFixed(3)}, nicht zu 1.`);
    if (Math.abs(skin - 1) > SUM_TOLERANCE) add("thermal.regions", "skin-fraction", `Skin fractions of the body regions of ${label} sum to ${skin.toFixed(3)}, not 1.`, `Die Hautanteile der Körperregionen von ${label} summieren sich zu ${skin.toFixed(3)}, nicht zu 1.`);
    if (subject === undefined && asset.thermal.body === undefined) add("thermal.body", "body-missing", "Body regions need thermal.body physiology.", "Körperregionen brauchen die Physiologie thermal.body.");
    if (subject !== undefined && !passengers.has(subject)) add("thermal.regions", "unresolved-reference", `Region subject "${subject}" is not a passenger.`, `Regionssubjekt "${subject}" ist kein Passagier.`);
  }
  if (asset.thermal.body !== undefined && !subjects.has(undefined)) add("thermal.body", "body-regions-missing", "thermal.body needs regions with a body segment.", "thermal.body braucht Regionen mit einem Körpersegment.");
  for (const passenger of asset.thermal.passengers) if (!subjects.has(passenger.id)) add("thermal.passengers", "passenger-regions-missing", `Passenger "${passenger.id}" has no body regions naming it as subject.`, `Passagier "${passenger.id}" hat keine Körperregionen, die ihn als Subjekt nennen.`);
  if (asset.thermal.body !== undefined) need("thermal.body.reference", asset.thermal.body.reference, references, "Reference");

  const breathByRespiration = new Set([respiration, ...asset.thermal.passengers.map((passenger) => passenger.respiration)].flatMap((r) => [r?.noseEmitter, r?.mouthEmitter]).filter((id): id is Slug => id !== undefined));
  asset.thermal.emitters.forEach((emitter, index) => {
    const path = `thermal.emitters[${index}]`;
    need(`${path}.anchor`, emitter.anchor, anchors, "Anchor");
    for (const id of emitter.anchors ?? []) need(`${path}.anchors`, id, anchors, "Anchor");
    need(`${path}.region`, emitter.region, regions, "Thermal region");
    need(`${path}.operatingKey`, emitter.operatingKey, operatingKeys, "Operating key");
    need(`${path}.reference`, emitter.reference, references, "Reference");
    if (!PARAMETER_SOURCES.includes(emitter.source)) add(path, "source-missing", `Emitter "${emitter.id}" needs a source.`, `Emitter "${emitter.id}" braucht eine Quelle.`);
    if (emitter.kind === "breath" && !breathByRespiration.has(emitter.id)) add(path, "breath-emitter", `Breath emitter "${emitter.id}" must be the nose or mouth emitter of thermal.respiration.`, `Atem-Emitter "${emitter.id}" muss der Nasen- oder Mund-Emitter von thermal.respiration sein.`);
  });
  if (respiration !== undefined) {
    need("thermal.respiration.noseEmitter", respiration.noseEmitter, emitters, "Emitter");
    need("thermal.respiration.mouthEmitter", respiration.mouthEmitter, emitters, "Emitter");
    for (const id of breathByRespiration) if (asset.thermal.emitters.find((emitter) => emitter.id === id)?.kind !== undefined && asset.thermal.emitters.find((emitter) => emitter.id === id)!.kind !== "breath") add("thermal.respiration", "breath-emitter", `Respiration emitter "${id}" must be of kind "breath".`, `Atem-Emitter "${id}" muss die Art "breath" haben.`);
    respiration.patterns.forEach((pattern, index) => {
      const path = `thermal.respiration.patterns[${index}]`;
      const sum = pattern.inhaleFraction + pattern.pauseAfterInhaleFraction + pattern.exhaleFraction + pattern.pauseAfterExhaleFraction;
      if (Math.abs(sum - 1) > PHASE_TOLERANCE) add(path, "phase-fractions", `The phase fractions of pattern "${pattern.id}" sum to ${sum.toFixed(4)}, not 1.`, `Die Phasenanteile des Musters "${pattern.id}" summieren sich zu ${sum.toFixed(4)}, nicht zu 1.`);
      need(`${path}.reference`, pattern.reference, references, "Reference");
      const nose = pattern.route !== "mouth";
      const mouth = pattern.route !== "nose";
      if ((nose && respiration.noseEmitter === undefined) || (mouth && respiration.mouthEmitter === undefined)) add(`${path}.route`, "route-emitter", `Route "${pattern.route}" of pattern "${pattern.id}" has no matching emitter.`, `Weg "${pattern.route}" des Musters "${pattern.id}" hat keinen passenden Emitter.`);
    });
    respiration.motion.forEach((motion, index) => need(`thermal.respiration.motion[${index}].bone`, motion.bone, bones, "Bone"));
    need("thermal.respiration.reference", respiration.reference, references, "Reference");
    if (!isBody) add("thermal.respiration", "respiration-body", "Respiration needs a body.", "Atmung braucht einen Körper.");
  } else if (asset.thermal.emitters.some((emitter) => emitter.kind === "breath")) add("thermal.emitters", "breath-emitter", "Breath emitters need thermal.respiration.", "Atem-Emitter brauchen thermal.respiration.");
  asset.thermal.passengers.forEach((passenger, index) => {
    const r = passenger.respiration;
    if (r === undefined) return;
    need(`thermal.passengers[${index}].respiration.noseEmitter`, r.noseEmitter, emitters, "Emitter");
    need(`thermal.passengers[${index}].respiration.mouthEmitter`, r.mouthEmitter, emitters, "Emitter");
    r.patterns.forEach((pattern, at) => {
      const sum = pattern.inhaleFraction + pattern.pauseAfterInhaleFraction + pattern.exhaleFraction + pattern.pauseAfterExhaleFraction;
      if (Math.abs(sum - 1) > PHASE_TOLERANCE) add(`thermal.passengers[${index}].respiration.patterns[${at}]`, "phase-fractions", `The phase fractions of pattern "${pattern.id}" sum to ${sum.toFixed(4)}, not 1.`, `Die Phasenanteile des Musters "${pattern.id}" summieren sich zu ${sum.toFixed(4)}, nicht zu 1.`);
    });
  });
  asset.structure.supports.forEach((support, index) => need(`structure.supports[${index}]`, support, anchors, "Anchor"));
  need("presentation.defaultPose", asset.presentation.defaultPose, poses, "Pose");
  need("presentation.defaultClip", asset.presentation.defaultClip, clips, "Clip");
  if (!missing(source.presentation.defaultScale, "presentation.defaultScale") && asset.representations[`${asset.presentation.defaultScale}`]?.status !== "authored") add("presentation.defaultScale", "default-scale-unavailable", `The default scale 1:${asset.presentation.defaultScale} is not an authored representation.`, `Der Standardmaßstab 1:${asset.presentation.defaultScale} ist keine gezeichnete Darstellung.`);

  if (asset.poses.filter((pose) => !pose.key).length < 3) add("poses", "too-few-poses", "An asset needs at least 3 named poses.", "Ein Asset braucht mindestens 3 benannte Posen.");
  if (asset.clips.length < 2) add("clips", "too-few-clips", "An asset needs at least 2 clips.", "Ein Asset braucht mindestens 2 Clips.");
  asset.poses.forEach((pose, index) => {
    const path = `poses[${index}]`;
    for (const id of Object.keys(pose.bones)) need(`${path}.bones.${id}`, id, bones, "Bone");
    pose.contacts.forEach((contact, at) => need(`${path}.contacts[${at}]`, contact, anchors, "Anchor"));
    const binding = pose.thermal;
    for (const id of Object.keys(binding.regions)) need(`${path}.thermal.regions.${id}`, id, regions, "Thermal region");
    for (const id of Object.keys(binding.emitters)) need(`${path}.thermal.emitters.${id}`, id, nonBreathSet, "Non-respiratory emitter");
    for (const id of Object.keys(binding.operating ?? {})) need(`${path}.thermal.operating.${id}`, id, operatingKeys, "Operating key");
    for (const id of regions) if (binding.regions[id] === undefined) add(`${path}.thermal.regions`, "binding-region-missing", `Pose "${pose.id}" binds no state for thermal region "${id}".`, `Pose "${pose.id}" bindet keinen Zustand für die thermische Region "${id}".`);
    for (const id of nonBreath) if (binding.emitters[id] === undefined) add(`${path}.thermal.emitters`, "binding-emitter-missing", `Pose "${pose.id}" binds no strength for emitter "${id}".`, `Pose "${pose.id}" bindet keine Stärke für den Emitter "${id}".`);
    for (const id of operatingKeys) if (binding.operating?.[id] === undefined) add(`${path}.thermal.operating`, "binding-operating-missing", `Pose "${pose.id}" binds no level for operating key "${id}".`, `Pose "${pose.id}" bindet keine Stufe für den Betriebsschlüssel "${id}".`);
    if (asset.thermal.body !== undefined && binding.activityMet === undefined) add(`${path}.thermal.activityMet`, "binding-activity-missing", `Pose "${pose.id}" of a body binds no activityMet.`, `Pose "${pose.id}" eines Körpers bindet kein activityMet.`);
    if (respiration !== undefined) {
      if (binding.breathing === undefined) add(`${path}.thermal.breathing`, "binding-breathing-missing", `Pose "${pose.id}" binds no breathing pattern.`, `Pose "${pose.id}" bindet kein Atemmuster.`);
      else need(`${path}.thermal.breathing`, binding.breathing, patterns, "Breathing pattern");
    } else if (binding.breathing !== undefined) add(`${path}.thermal.breathing`, "binding-breathing-unknown", "A pose binds a breathing pattern but the asset has no respiration.", "Eine Pose bindet ein Atemmuster, aber das Asset hat keine Atmung.");
    for (const passenger of asset.thermal.passengers) {
      const bound = binding.passengers?.[passenger.id];
      if (bound === undefined) add(`${path}.thermal.passengers`, "binding-passenger-missing", `Pose "${pose.id}" binds no state for passenger "${passenger.id}".`, `Pose "${pose.id}" bindet keinen Zustand für den Passagier "${passenger.id}".`);
      else if (passenger.respiration !== undefined) {
        if (bound.breathing === undefined || !passenger.respiration.patterns.some((pattern) => pattern.id === bound.breathing)) add(`${path}.thermal.passengers.${passenger.id}.breathing`, "binding-breathing-missing", `Pose "${pose.id}" binds no valid breathing pattern for passenger "${passenger.id}".`, `Pose "${pose.id}" bindet kein gültiges Atemmuster für den Passagier "${passenger.id}".`);
      }
    }
    for (const id of Object.keys(binding.passengers ?? {})) need(`${path}.thermal.passengers.${id}`, id, passengers, "Passenger");
    pose.loads.forEach((load, at) => {
      const lp = `${path}.loads[${at}]`;
      need(`${lp}.anchor`, load.anchor, anchors, "Anchor");
      need(`${lp}.bone`, load.bone, bones, "Bone");
      need(`${lp}.distributed.bone`, load.distributed?.bone, bones, "Bone");
      need(`${lp}.reference`, load.reference, references, "Reference");
      if (load.anchor === undefined && load.bone === undefined && load.distributed === undefined) add(lp, "load-unplaced", `Load "${load.id}" needs an anchor, a bone or a distributed bone.`, `Last "${load.id}" braucht einen Anker, einen Knochen oder einen verteilten Knochen.`);
      if (!PARAMETER_SOURCES.includes(load.source)) add(lp, "source-missing", `Load "${load.id}" needs a source.`, `Last "${load.id}" braucht eine Quelle.`);
    });
    jointIssues(asset, pose.bones, `${path}.bones`, out);
  });
  asset.clips.forEach((clip, index) => {
    const path = `clips[${index}]`;
    if (!(clip.durationS > 0) || !(clip.fps > 0)) add(path, "clip-duration", `Clip "${clip.id}" needs a positive duration and fps.`, `Clip "${clip.id}" braucht positive Dauer und fps.`);
    if (clip.keys.length === 0) add(`${path}.keys`, "key-order", `Clip "${clip.id}" needs at least one key.`, `Clip "${clip.id}" braucht mindestens einen Schlüssel.`);
    clip.keys.forEach((key, at) => {
      const kp = `${path}.keys[${at}]`;
      if (at > 0 && !(key.t > clip.keys[at - 1]!.t)) add(`${kp}.t`, "key-order", `Key times of clip "${clip.id}" must strictly ascend.`, `Schlüsselzeiten von Clip "${clip.id}" müssen streng aufsteigen.`);
      if (!(key.t >= 0 && key.t <= clip.durationS)) add(`${kp}.t`, "key-range", `Key time ${key.t} of clip "${clip.id}" lies outside [0, ${clip.durationS}].`, `Schlüsselzeit ${key.t} von Clip "${clip.id}" liegt außerhalb [0, ${clip.durationS}].`);
      need(`${kp}.pose`, key.pose, poses, "Pose");
      for (const id of Object.keys(key.bones ?? {})) need(`${kp}.bones.${id}`, id, bones, "Bone");
      const pose = asset.poses.find((candidate) => candidate.id === key.pose);
      const merged: Record<Slug, BoneOffset> = {};
      for (const [id, offset] of Object.entries(key.bones ?? {})) merged[id] = { ...pose?.bones[id], ...offset };
      jointIssues(asset, merged, `${kp}.bones`, out);
    });
    if (clip.loop && clip.keys.length > 1 && clip.keys[clip.keys.length - 1]!.t >= clip.durationS) {
      const merged = (key: (typeof clip.keys)[number]): Record<Slug, BoneOffset> => Object.fromEntries(asset.rig.bones.map((bone) => [bone.id, { ...asset.poses.find((pose) => pose.id === key.pose)?.bones[bone.id], ...key.bones?.[bone.id] }]));
      const head = merged(clip.keys[0]!);
      const tail = merged(clip.keys[clip.keys.length - 1]!);
      for (const bone of asset.rig.bones) {
        const unlimited = bone.joint.type === "revolute" && bone.joint.minDeg === undefined && bone.joint.maxDeg === undefined;
        const turns = ((tail[bone.id]!.angleDeg ?? 0) - (head[bone.id]!.angleDeg ?? 0)) / 360;
        const angleOk = unlimited ? Math.abs(turns - Math.round(turns)) < 1e-9 : Math.abs(turns) < 1e-9;
        const shiftOk = Math.abs((tail[bone.id]!.dx ?? 0) - (head[bone.id]!.dx ?? 0)) < 1e-6 && Math.abs((tail[bone.id]!.dy ?? 0) - (head[bone.id]!.dy ?? 0)) < 1e-6;
        if (!angleOk || !shiftOk) add(`${path}.keys[${clip.keys.length - 1}].bones.${bone.id}`, "loop-closing", `The closing key of looping clip "${clip.id}" must equal the first key for bone "${bone.id}"${unlimited ? " up to whole turns of 360 degrees" : ""}.`, `Der Schlusskey des Schleifen-Clips "${clip.id}" muss für Knochen "${bone.id}" dem ersten Key entsprechen${unlimited ? " bis auf ganze Umdrehungen von 360 Grad" : ""}.`);
      }
    }
    clip.contacts.forEach((span, at) => {
      need(`${path}.contacts[${at}].anchor`, span.anchor, anchors, "Anchor");
      if (!(span.fromS >= 0 && span.toS > span.fromS && span.toS <= clip.durationS)) add(`${path}.contacts[${at}]`, "contact-range", `Contact span of "${span.anchor}" must satisfy 0 <= from < to <= duration.`, `Kontaktspanne von "${span.anchor}" muss 0 <= von < bis <= Dauer erfüllen.`);
    });
    clip.phases.forEach((phase, at) => {
      if (!(phase.fromS >= 0 && phase.toS > phase.fromS && phase.toS <= clip.durationS)) add(`${path}.phases[${at}]`, "phase-range", `Phase "${phase.id}" must satisfy 0 <= from < to <= duration.`, `Phase "${phase.id}" muss 0 <= von < bis <= Dauer erfüllen.`);
    });
    clip.events.forEach((event, at) => {
      if (!(event.t >= 0 && event.t <= clip.durationS)) add(`${path}.events[${at}].t`, "event-range", `Event time ${event.t} lies outside the clip.`, `Ereigniszeit ${event.t} liegt außerhalb des Clips.`);
    });
    clip.tracks.forEach((track, at) => {
      const tp = `${path}.tracks[${at}].target`;
      const [kind, id, field] = track.target.split(":");
      const known = track.target === "activityMet" || track.target === "externalWorkW" || (track.target === "breathing" && respiration !== undefined) || (kind === "emitter" && nonBreathSet.has(id ?? "")) || (kind === "operating" && operatingKeys.has(id ?? "")) || (kind === "load" && loads.has(id ?? "")) || (kind === "region" && regions.has(id ?? "") && REGION_FIELDS.includes(field ?? ""));
      if (!known) add(tp, "track-target", `Track target "${track.target}" does not resolve.`, `Spurziel "${track.target}" ist nicht auflösbar.`);
      track.keys.forEach((key, k) => {
        if (!(key.t >= 0 && key.t <= clip.durationS) || (k > 0 && !(key.t > track.keys[k - 1]!.t))) add(`${path}.tracks[${at}].keys[${k}].t`, "key-order", "Track key times must ascend within the clip.", "Spur-Schlüsselzeiten müssen innerhalb des Clips aufsteigen.");
      });
    });
  });

  if (asset.structure.supports.length === 0) {
    asset.clips.forEach((clip, index) => {
      const held = mergeSpans(clip.contacts);
      const airborne = mergeSpans(clip.phases.filter((phase) => phase.ballistic === true));
      const gaps: { from: number; to: number }[] = [];
      let cursor = 0;
      for (const span of held) {
        if (span.from > cursor + 1e-9) gaps.push({ from: cursor, to: span.from });
        cursor = Math.max(cursor, span.to);
      }
      if (cursor < clip.durationS - 1e-9) gaps.push({ from: cursor, to: clip.durationS });
      for (const gap of gaps) {
        if (!airborne.some((phase) => phase.from <= gap.from + 1e-9 && phase.to >= gap.to - 1e-9)) add(`clips[${index}].contacts`, "unsupported-instant", `Clip "${clip.id}" has no contact between ${gap.from.toFixed(3)} s and ${gap.to.toFixed(3)} s and the asset has no supports: mark that time as a ballistic phase.`, `Clip "${clip.id}" hat zwischen ${gap.from.toFixed(3)} s und ${gap.to.toFixed(3)} s keinen Kontakt und das Asset hat keine Auflager: diese Zeit muss eine ballistische Phase sein.`);
      }
    });
  }
  const soft = new Set(["unresolved-reference", "key-order", "clip-duration", "rep-undecided", "rep-key-mismatch"]);
  if (rigOk && out.every((issue) => !soft.has(issue.code))) {
    try {
      for (const rep of representationPending) extentIssues(`representations.${rep.denominator}`, rep);
      for (const clip of asset.clips) {
        if (clip.rootMotion !== "plant" || !clip.loop) continue;
        const travel = sampleClip(asset, clip, clip.durationS - 1e-9).root.x - sampleClip(asset, clip, 0).root.x;
        if (!(Math.abs(travel - (clip.advanceM ?? 0)) <= ADVANCE_TOLERANCE_M)) add(`clips.${clip.id}.advanceM`, "clip-advance", `advanceM ${clip.advanceM ?? 0} m differs from the planted travel ${travel.toFixed(4)} m of one cycle.`, `advanceM ${clip.advanceM ?? 0} m weicht von der geplanten Wegstrecke ${travel.toFixed(4)} m eines Zyklus ab.`);
      }
    } catch (error) {
      add("$", "evaluation-failed", `Evaluating the asset failed: ${(error as Error).message}`, `Auswerten des Assets ist fehlgeschlagen: ${(error as Error).message}`);
    }
  }
  const { heightM, widthM, depthM, massKg } = asset.dimensions;
  if (!(heightM > 0 && widthM > 0 && depthM > 0 && massKg > 0)) add("dimensions", "dimension-invalid", "Dimensions and mass must be positive.", "Abmessungen und Masse müssen positiv sein.");
  return out;
}

/** 📋️ Every inconsistency between the manifest and the asset documents; entries and assets must agree on id, category, named (non-key) poses, clips, per-scale representation status, symbols, viewpoints, exports, completion, heights (5 %) and widths (10 %), analytics follow from the asset, every `asset` alternative of an unavailable representation exists, every category has a folder and lists as many entries as it has assets. */
export function validateManifest(manifest: Manifest, assets: readonly Asset[]): readonly Issue[] {
  const out: Issue[] = [];
  const add = (path: string, code: string, en: string, de: string): void => void out.push({ path, code, message: text(en, de) });
  const byId = new Map(assets.map((asset) => [asset.id, asset]));
  textIssues(manifest, "$", out);
  for (const id of duplicates(manifest.entries.map((entry) => entry.id))) add("entries", "duplicate-id", `Manifest entry "${id}" is listed more than once.`, `Manifest-Eintrag "${id}" ist mehrfach aufgeführt.`);
  for (const id of duplicates(assets.map((asset) => asset.id))) add("assets", "duplicate-id", `Asset "${id}" exists more than once.`, `Asset "${id}" existiert mehrfach.`);
  for (const id of duplicates(manifest.categories.map((category) => category.id))) add("categories", "duplicate-id", `Category "${id}" is listed more than once.`, `Kategorie "${id}" ist mehrfach aufgeführt.`);
  for (const id of CATEGORIES) if (!manifest.categories.some((category) => category.id === id)) add("categories", "category-missing", `Category "${id}" is not in the manifest.`, `Kategorie "${id}" fehlt im Manifest.`);
  manifest.categories.forEach((category, index) => {
    if (category.folder.trim() === "") add(`categories[${index}].folder`, "category-folder", `Category "${category.id}" needs a folder.`, `Kategorie "${category.id}" braucht einen Ordner.`);
  });
  const same = (a: readonly string[], b: readonly string[]): boolean => a.length === b.length && [...a].sort().join("\u0000") === [...b].sort().join("\u0000");
  manifest.entries.forEach((entry, index) => {
    const path = `entries[${index}]`;
    const asset = byId.get(entry.id);
    if (asset === undefined) return add(path, "manifest-asset-missing", `Manifest entry "${entry.id}" has no asset document.`, `Manifest-Eintrag "${entry.id}" hat kein Asset-Dokument.`);
    const mismatch = (field: string): void => add(`${path}.${field}`, "manifest-mismatch", `Entry "${entry.id}" and its asset disagree on ${field}.`, `Eintrag "${entry.id}" und sein Asset widersprechen sich bei ${field}.`);
    if (entry.category !== asset.category) mismatch("category");
    if (!same(entry.poses, asset.poses.filter((pose) => !pose.key).map((pose) => pose.id))) mismatch("poses");
    if (!same(entry.clips, asset.clips.map((clip) => clip.id))) mismatch("clips");
    if (!same(entry.symbols, asset.symbols.map((symbol) => symbol.id))) mismatch("symbols");
    if (!same(entry.viewpoints, asset.presentation.viewpoints)) mismatch("viewpoints");
    if (!same(entry.exports, asset.exports)) mismatch("exports");
    if (entry.completion !== asset.completion) mismatch("completion");
    if (SCALE_DENOMINATORS.some((denominator) => entry.representations[`${denominator}`] !== asset.representations[`${denominator}`]?.status)) mismatch("representations");
    if (!(Math.abs(entry.heightM - asset.dimensions.heightM) <= HEIGHT_TOLERANCE * asset.dimensions.heightM)) mismatch("heightM");
    if (!(Math.abs(entry.widthM - asset.dimensions.widthM) <= WIDTH_TOLERANCE * asset.dimensions.widthM)) mismatch("widthM");
    if (entry.analytics.includes("thermal") !== asset.thermal.regions.length > 0) mismatch("analytics");
    if (entry.analytics.includes("transport") !== asset.thermal.emitters.length > 0) mismatch("analytics");
    for (const denominator of SCALE_DENOMINATORS) {
      const rep = asset.representations[`${denominator}`];
      if (rep?.status !== "unavailable") continue;
      for (const alternative of rep.alternatives) if (alternative.kind === "asset" && !byId.has(alternative.asset)) add(`${path}.representations.${denominator}`, "alternative-unresolved", `Alternative asset "${alternative.asset}" of "${entry.id}" does not exist in the archive.`, `Alternatives Asset "${alternative.asset}" von "${entry.id}" existiert nicht im Archiv.`);
    }
  });
  const listed = new Set(manifest.entries.map((entry) => entry.id));
  for (const asset of assets) if (!listed.has(asset.id)) add("entries", "manifest-entry-missing", `Asset "${asset.id}" has no manifest entry.`, `Asset "${asset.id}" hat keinen Manifest-Eintrag.`);
  for (const category of CATEGORIES) {
    const planned = manifest.entries.filter((entry) => entry.category === category).length;
    const authored = assets.filter((asset) => asset.category === category).length;
    if (planned !== authored) add(`categories.${category}`, "manifest-count", `Category "${category}" plans ${planned} entries but has ${authored} assets.`, `Kategorie "${category}" plant ${planned} Einträge, hat aber ${authored} Assets.`);
  }
  manifest.collections.forEach((collection, index) => {
    for (const id of collection.assets) if (!listed.has(id)) add(`collections[${index}]`, "unresolved-reference", `Collection "${collection.id}" lists unknown asset "${id}".`, `Sammlung "${collection.id}" listet das unbekannte Asset "${id}".`);
  });
  return out;
}
