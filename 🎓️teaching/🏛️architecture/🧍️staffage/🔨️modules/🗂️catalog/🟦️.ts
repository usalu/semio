/** 🗂️ Semantic validation of staffage assets and of the manifest that plans them.
 *
 * The JSON Schema (`🧬️schema/🔣️.json`) checks shape; this module checks meaning: unique ids, resolvable references, a single-root rig, joint limits, physical plausibility,
 * dimensions against the evaluated default pose, clip timing and bilingual texts. Every finding is an {@link Issue} with a JSON-path-like location, a stable code and an `{ en, de }` message.
 *
 * @see ../../🧬️schema/🔣️.json — the normative structural contract
 * @see ../🧮️evaluation/🟦️.ts — `evaluate`, used for the dimension check
 */
import { CATEGORIES, PARAMETER_SOURCES } from "../../🧬️schema/🟦️.ts";
import type { Asset, BoneOffset, Manifest, Slug, Text } from "../../🧬️schema/🟦️.ts";
import { sampleClip } from "../🎞️animation/🟦️.ts";
import { evaluate } from "../🧮️evaluation/🟦️.ts";
import { effectiveOffset } from "../🦴️rig/🟦️.ts";

/** 🚨️ One validation finding. */
export type Issue = { readonly path: string; readonly code: string; readonly message: Text };

/** 📏️ Allowed deviation of the nominal height from the evaluated default pose. */
export const HEIGHT_TOLERANCE = 0.05;

/** 📏️ Allowed deviation of the nominal width from the evaluated default pose. */
export const WIDTH_TOLERANCE = 0.1;

/** 🔁️ Allowed mismatch in metres between `advanceM` and the planted root travel of one cycle. */
export const ADVANCE_TOLERANCE_M = 0.002;

const REGION_FIELDS = ["exposedFraction", "surfaceTempC", "coreTempC", "wettedness"];
const ANGLE_EPS = 1e-9;

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

/** ✅️ Every semantic problem of an asset; an empty list means the asset is valid. */
export function validateAsset(asset: Asset): readonly Issue[] {
  const out: Issue[] = [];
  const add = (path: string, code: string, en: string, de: string): void => void out.push({ path, code, message: text(en, de) });
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
  const operating = new Set(asset.thermal.regions.flatMap((region) => (region.operatingKey === undefined ? [] : [region.operatingKey])));
  const poses = new Set(asset.poses.map((pose) => pose.id));
  const clips = new Set(asset.clips.map((clip) => clip.id));
  const loads = new Set(asset.poses.flatMap((pose) => pose.loads.map((load) => load.id)));
  const references = new Set(asset.provenance.references.map((reference) => reference.id));
  const shapes = [...asset.geometry.elevation, ...(asset.geometry.plan ?? [])];

  unique("rig.bones", asset.rig.bones.map((bone) => bone.id));
  unique("anchors", asset.anchors.map((anchor) => anchor.id));
  unique("geometry", shapes.map((shape) => shape.id));
  unique("poses", asset.poses.map((pose) => pose.id));
  unique("clips", asset.clips.map((clip) => clip.id));
  unique("materials", asset.materials.map((material) => material.id));
  unique("thermal.regions", asset.thermal.regions.map((region) => region.id));
  unique("thermal.emitters", asset.thermal.emitters.map((emitter) => emitter.id));
  unique("provenance.references", asset.provenance.references.map((reference) => reference.id));
  asset.poses.forEach((pose, index) => unique(`poses[${index}].loads`, pose.loads.map((load) => load.id)));

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
  if (asset.geometry.elevation.length === 0) add("geometry.elevation", "elevation-empty", "The elevation geometry must not be empty.", "Die Ansichtsgeometrie darf nicht leer sein.");
  shapes.forEach((shape, index) => {
    const path = `geometry.shapes[${index}]`;
    need(`${path}.bone`, shape.bone, bones, "Bone");
    need(`${path}.region`, shape.region, regions, "Thermal region");
    need(`${path}.material`, shape.material, materials, "Material");
  });
  for (const region of regions) if (!shapes.some((shape) => shape.region === region)) add(`thermal.regions.${region}`, "region-unbound", `Thermal region "${region}" is not bound by any shape.`, `Thermische Region "${region}" ist an keine Form gebunden.`);

  asset.thermal.regions.forEach((region, index) => {
    const path = `thermal.regions[${index}]`;
    need(`${path}.material`, region.material, materials, "Material");
    need(`${path}.reference`, region.reference, references, "Reference");
    if (!PARAMETER_SOURCES.includes(region.source)) add(path, "source-missing", `Region "${region.id}" needs a source.`, `Region "${region.id}" braucht eine Quelle.`);
    if (!(region.areaM2 > 0) || !(region.characteristicLengthM > 0)) add(path, "region-size", `Region "${region.id}" needs a positive area and characteristic length.`, `Region "${region.id}" braucht positive Fläche und charakteristische Länge.`);
    const required: Readonly<Record<string, readonly (keyof typeof region)[]>> = { fixed: ["surfaceTempC"], "core-resistance": ["coreTempC", "insulationM2KW"], leaf: ["stomatalConductanceMS"], passive: [], operating: ["operatingKey", "riseK"] };
    for (const field of required[region.model] ?? []) if (region[field] === undefined) add(`${path}.${field}`, "thermal-parameter-missing", `Model "${region.model}" of region "${region.id}" needs "${field}".`, `Modell "${region.model}" von Region "${region.id}" braucht "${field}".`);
  });
  const shares = asset.thermal.regions.flatMap((region) => (region.metabolicShare === undefined ? [] : [region.metabolicShare]));
  if (shares.length > 0 && Math.abs(shares.reduce((a, b) => a + b, 0) - 1) > 0.02) add("thermal.regions", "metabolic-share", "Metabolic shares of the regions must sum to 1.", "Die Stoffwechselanteile der Regionen müssen sich zu 1 addieren.");
  asset.thermal.emitters.forEach((emitter, index) => {
    const path = `thermal.emitters[${index}]`;
    need(`${path}.anchor`, emitter.anchor, anchors, "Anchor");
    need(`${path}.reference`, emitter.reference, references, "Reference");
    if (!PARAMETER_SOURCES.includes(emitter.source)) add(path, "source-missing", `Emitter "${emitter.id}" needs a source.`, `Emitter "${emitter.id}" braucht eine Quelle.`);
  });
  asset.structure.supports.forEach((support, index) => need(`structure.supports[${index}]`, support, anchors, "Anchor"));
  need("presentation.defaultPose", asset.presentation.defaultPose, poses, "Pose");
  need("presentation.defaultClip", asset.presentation.defaultClip, clips, "Clip");

  if (asset.poses.filter((pose) => !pose.key).length < 3) add("poses", "too-few-poses", "An asset needs at least 3 named poses.", "Ein Asset braucht mindestens 3 benannte Posen.");
  if (asset.clips.length < 2) add("clips", "too-few-clips", "An asset needs at least 2 clips.", "Ein Asset braucht mindestens 2 Clips.");
  asset.poses.forEach((pose, index) => {
    const path = `poses[${index}]`;
    for (const id of Object.keys(pose.bones)) need(`${path}.bones.${id}`, id, bones, "Bone");
    pose.contacts.forEach((contact, at) => need(`${path}.contacts[${at}]`, contact, anchors, "Anchor"));
    for (const id of Object.keys(pose.thermal.regions ?? {})) need(`${path}.thermal.regions.${id}`, id, regions, "Thermal region");
    for (const id of Object.keys(pose.thermal.emitters ?? {})) need(`${path}.thermal.emitters.${id}`, id, emitters, "Emitter");
    for (const id of Object.keys(pose.thermal.operating ?? {})) need(`${path}.thermal.operating.${id}`, id, operating, "Operating key");
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
      const known = track.target === "activityMet" || (kind === "emitter" && emitters.has(id ?? "")) || (kind === "operating" && operating.has(id ?? "")) || (kind === "load" && loads.has(id ?? "")) || (kind === "region" && regions.has(id ?? "") && REGION_FIELDS.includes(field ?? ""));
      if (!known) add(tp, "track-target", `Track target "${track.target}" does not resolve.`, `Spurziel "${track.target}" ist nicht auflösbar.`);
      track.keys.forEach((key, k) => {
        if (!(key.t >= 0 && key.t <= clip.durationS) || (k > 0 && !(key.t > track.keys[k - 1]!.t))) add(`${path}.tracks[${at}].keys[${k}].t`, "key-order", "Track key times must ascend within the clip.", "Spur-Schlüsselzeiten müssen innerhalb des Clips aufsteigen.");
      });
    });
  });

  if (asset.structure.supports.length === 0) {
    asset.clips.forEach((clip, index) => {
      const merge = (spans: readonly { readonly fromS: number; readonly toS: number }[]): { from: number; to: number }[] => {
        const merged: { from: number; to: number }[] = [];
        for (const span of [...spans].sort((p, q) => p.fromS - q.fromS)) {
          const last = merged[merged.length - 1];
          if (last !== undefined && span.fromS <= last.to + 1e-9) last.to = Math.max(last.to, span.toS);
          else merged.push({ from: span.fromS, to: span.toS });
        }
        return merged;
      };
      const held = merge(clip.contacts);
      const airborne = merge(clip.phases.filter((phase) => phase.ballistic === true));
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
  if (rigOk && out.every((issue) => issue.code !== "unresolved-reference" && issue.code !== "key-order" && issue.code !== "clip-duration")) {
    try {
      const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose });
      const height = state.bounds.maxY - state.bounds.minY;
      const width = state.bounds.maxX - state.bounds.minX;
      if (!(Math.abs(height - asset.dimensions.heightM) <= HEIGHT_TOLERANCE * asset.dimensions.heightM)) add("dimensions.heightM", "dimension-height", `Nominal height ${asset.dimensions.heightM} m differs from the evaluated ${height.toFixed(3)} m by more than 5 %.`, `Nennhöhe ${asset.dimensions.heightM} m weicht um mehr als 5 % von den ausgewerteten ${height.toFixed(3)} m ab.`);
      if (!(Math.abs(width - asset.dimensions.widthM) <= WIDTH_TOLERANCE * asset.dimensions.widthM)) add("dimensions.widthM", "dimension-width", `Nominal width ${asset.dimensions.widthM} m differs from the evaluated ${width.toFixed(3)} m by more than 10 %.`, `Nennbreite ${asset.dimensions.widthM} m weicht um mehr als 10 % von den ausgewerteten ${width.toFixed(3)} m ab.`);
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

/** 📋️ Every inconsistency between the manifest and the asset documents; entries and assets must agree on id, category, named (non-key) poses, clips, viewpoints, exports, completion, heights (5 %) and widths (10 %), and every category lists as many entries as it has assets. */
export function validateManifest(manifest: Manifest, assets: readonly Asset[]): readonly Issue[] {
  const out: Issue[] = [];
  const add = (path: string, code: string, en: string, de: string): void => void out.push({ path, code, message: text(en, de) });
  const byId = new Map(assets.map((asset) => [asset.id, asset]));
  textIssues(manifest, "$", out);
  for (const id of duplicates(manifest.entries.map((entry) => entry.id))) add("entries", "duplicate-id", `Manifest entry "${id}" is listed more than once.`, `Manifest-Eintrag "${id}" ist mehrfach aufgeführt.`);
  for (const id of duplicates(assets.map((asset) => asset.id))) add("assets", "duplicate-id", `Asset "${id}" exists more than once.`, `Asset "${id}" existiert mehrfach.`);
  const same = (a: readonly string[], b: readonly string[]): boolean => a.length === b.length && [...a].sort().join("\u0000") === [...b].sort().join("\u0000");
  manifest.entries.forEach((entry, index) => {
    const path = `entries[${index}]`;
    const asset = byId.get(entry.id);
    if (asset === undefined) return add(path, "manifest-asset-missing", `Manifest entry "${entry.id}" has no asset document.`, `Manifest-Eintrag "${entry.id}" hat kein Asset-Dokument.`);
    const mismatch = (field: string): void => add(`${path}.${field}`, "manifest-mismatch", `Entry "${entry.id}" and its asset disagree on ${field}.`, `Eintrag "${entry.id}" und sein Asset widersprechen sich bei ${field}.`);
    if (entry.category !== asset.category) mismatch("category");
    if (!same(entry.poses, asset.poses.filter((pose) => !pose.key).map((pose) => pose.id))) mismatch("poses");
    if (!same(entry.clips, asset.clips.map((clip) => clip.id))) mismatch("clips");
    if (!same(entry.viewpoints, asset.presentation.viewpoints)) mismatch("viewpoints");
    if (!same(entry.exports, asset.exports)) mismatch("exports");
    if (entry.completion !== asset.completion) mismatch("completion");
    if (!(Math.abs(entry.heightM - asset.dimensions.heightM) <= HEIGHT_TOLERANCE * asset.dimensions.heightM)) mismatch("heightM");
    if (!(Math.abs(entry.widthM - asset.dimensions.widthM) <= WIDTH_TOLERANCE * asset.dimensions.widthM)) mismatch("widthM");
    const thermal = asset.thermal.regions.length > 0;
    if (entry.analytics.includes("thermal") !== thermal) mismatch("analytics");
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
