/** ✅️ Structural and semantic validation of a `semio.model/v1` document; findings are bilingual issues with JSON-pointer paths.
 * @see ../🧬️schema/🔣️.json */
import type { Issue, ModelDocument, Operation, Slug, Text } from "../🧬️schema/🟦️.ts";
import { Pace, silentOperation } from "../⏱️operation/🟦️.ts";
import { revolveOutline } from "../🔺️tessellation/🌀️revolved/🟦️.ts";
import { cleanRing, holesFault, ringFault } from "../🔺️tessellation/🔺️polygon/🟦️.ts";
import * as portable from "../🧮️math/🎯️portable/🟦️.ts";

const SLUG = /^[a-z0-9]+(?:-[a-z0-9]+)*$/u;
const COMPONENTS = { translation: 3, rotation: 4, scale: 3, baseColor: 4, emissive: 3 } as const;

const MESSAGES: Record<string, Text> = {
  "bad-version": { en: "The version must be semio.model/v1.", de: "Die Version muss semio.model/v1 sein." },
  "bad-type": { en: "The value has the wrong type or length.", de: "Der Wert hat den falschen Typ oder die falsche Länge." },
  "non-finite": { en: "Numbers must be finite.", de: "Zahlen müssen endlich sein." },
  "bad-slug": { en: "The identifier is not a kebab-case slug.", de: "Der Bezeichner ist kein Kebab-Case-Slug." },
  "duplicate-id": { en: "The identifier is not unique in its scope.", de: "Der Bezeichner ist in seinem Bereich nicht eindeutig." },
  "unknown-parent": { en: "The parent node does not exist.", de: "Der Elternknoten existiert nicht." },
  "parent-order": { en: "A parent must precede its children.", de: "Ein Elternknoten muss vor seinen Kindern stehen." },
  "non-unit-quaternion": { en: "The quaternion is not of unit length.", de: "Das Quaternion hat nicht die Länge eins." },
  "zero-scale": { en: "The scale must not be zero.", de: "Die Skalierung darf nicht null sein." },
  "joint-axis": { en: "The joint axis must not be zero.", de: "Die Gelenkachse darf nicht null sein." },
  "joint-limits": { en: "The lower limit exceeds the upper limit.", de: "Die untere Grenze liegt über der oberen." },
  "unknown-node": { en: "The node does not exist.", de: "Der Knoten existiert nicht." },
  "unknown-material": { en: "The material does not exist.", de: "Das Material existiert nicht." },
  "unknown-mesh": { en: "The mesh does not exist.", de: "Das Netz existiert nicht." },
  "unknown-skin": { en: "The skin does not exist.", de: "Die Haut existiert nicht." },
  "invalid-dimension": { en: "A dimension is out of its valid range.", de: "Eine Abmessung liegt außerhalb des gültigen Bereichs." },
  "bad-weights": { en: "The weights must be at most four and have a positive sum.", de: "Die Gewichte müssen höchstens vier sein und eine positive Summe haben." },
  "bad-repeat": { en: "The repeat is invalid.", de: "Die Wiederholung ist ungültig." },
  "self-intersection": { en: "The outline touches or crosses itself, folds back or has no area.", de: "Der Umriss berührt oder kreuzt sich, klappt zurueck oder hat keine Fläche." },
  "bad-holes": { en: "A hole touches or crosses the outline or another hole, or lies outside the outline.", de: "Ein Loch berührt oder kreuzt den Umriss oder ein anderes Loch oder liegt außerhalb des Umrisses." },
  "bad-solids": { en: "The solid provenance ranges must be contiguous, in order and cover all indices.", de: "Die Solid-Herkunftsbereiche müssen zusammenhängend, geordnet sein und alle Indizes abdecken." },
  "bad-primitive": { en: "The primitive arrays are inconsistent.", de: "Die Primitiv-Arrays sind inkonsistent." },
  "index-range": { en: "An index points outside the vertices.", de: "Ein Index zeigt außerhalb der Eckpunkte." },
  "joint-index": { en: "A joint index points outside the skin.", de: "Ein Gelenkindex zeigt außerhalb der Haut." },
  "weight-sum": { en: "The vertex weights do not sum to one.", de: "Die Eckpunktgewichte ergeben nicht eins." },
  "bad-skin": { en: "The inverse bind matrices do not match the joints.", de: "Die inversen Bindematrizen passen nicht zu den Gelenken." },
  "bad-color": { en: "A colour component is out of range.", de: "Eine Farbkomponente liegt außerhalb des Bereichs." },
  "bad-channel": { en: "The channel keys and values are inconsistent.", de: "Die Kanalschlüssel und -werte sind inkonsistent." },
  "bad-times": { en: "Key times must strictly increase and lie within the duration.", de: "Schlüsselzeiten müssen streng steigen und in der Dauer liegen." },
  "bad-duration": { en: "The duration must be positive.", de: "Die Dauer muss positiv sein." },
};

/** 🩺️ Findings of a document; empty when valid. Errors make `tessellate` refuse the document. Synchronous; the optional `operation` is polled at a bounded interval (per solid and inside every primitive array and outline pair loop) and a cancellation throws `CancelledError`. */
export function validateModel(document: ModelDocument, operation: Operation = silentOperation): Issue[] {
  const pace = new Pace(operation);
  const issues: Issue[] = [];
  const add = (severity: Issue["severity"], code: string, path: string) => issues.push({ severity, code, path, message: MESSAGES[code]! });
  const error = (code: string, path: string) => add("error", code, path);
  const isNum = (v: unknown): v is number => typeof v === "number";
  const vec = (v: unknown, n: number, path: string): v is number[] => {
    if (!Array.isArray(v) || v.length !== n || !v.every(isNum)) return error("bad-type", path), false;
    if (!v.every(Number.isFinite)) return error("non-finite", path), false;
    return true;
  };
  const doc = document as unknown as Record<string, unknown>;
  if (doc.version !== "semio.model/v1") error("bad-version", "/version");
  for (const key of ["nodes", "solids", "meshes", "skins", "materials", "variants", "poses", "animations"]) if (!Array.isArray(doc[key])) error("bad-type", `/${key}`);
  if (issues.length > 0) return issues;
  const ids = (key: string, list: readonly { id: Slug }[]): Set<Slug> => {
    const seen = new Set<Slug>();
    list.forEach((item, i) => {
      if (typeof item.id !== "string" || !SLUG.test(item.id)) error("bad-slug", `/${key}/${i}/id`);
      else if (seen.has(item.id)) error("duplicate-id", `/${key}/${i}/id`);
      seen.add(item.id);
    });
    return seen;
  };
  const nodeIds = ids("nodes", document.nodes), materialIds = ids("materials", document.materials), meshIds = ids("meshes", document.meshes), skinIds = ids("skins", document.skins);
  ids("solids", document.solids);
  ids("variants", document.variants);
  ids("poses", document.poses);
  ids("animations", document.animations);
  const position = new Map(document.nodes.map((n, i) => [n.id, i]));

  document.nodes.forEach((n, i) => {
    const p = `/nodes/${i}`;
    if (n.parent !== null) {
      if (!nodeIds.has(n.parent)) error("unknown-parent", `${p}/parent`);
      else if (position.get(n.parent)! >= i) error("parent-order", `${p}/parent`);
    }
    if (vec(n.translation, 3, `${p}/translation`) && vec(n.rotation, 4, `${p}/rotation`) && vec(n.scale, 3, `${p}/scale`)) {
      if (Math.abs(portable.hypot(...n.rotation) - 1) > 1e-3) error("non-unit-quaternion", `${p}/rotation`);
      if (n.scale.some((s) => s === 0)) error("zero-scale", `${p}/scale`);
    }
    if (n.mesh !== undefined && !meshIds.has(n.mesh)) error("unknown-mesh", `${p}/mesh`);
    if (n.skin !== undefined && !skinIds.has(n.skin)) error("unknown-skin", `${p}/skin`);
    const j = n.joint;
    if (j) {
      if (vec(j.axis, 3, `${p}/joint/axis`) && portable.hypot(...j.axis) < 1e-9) error("joint-axis", `${p}/joint/axis`);
      if (j.min !== undefined && j.max !== undefined && j.min > j.max) error("joint-limits", `${p}/joint`);
      if (j.twistMin !== undefined && j.twistMax !== undefined && j.twistMin > j.twistMax) error("joint-limits", `${p}/joint`);
    }
  });

  const positive = (v: unknown, path: string) => { if (!isNum(v) || !(v > 0) || !Number.isFinite(v)) error("invalid-dimension", path); };
  const ring = (r: unknown, min: number, path: string, nonNegativeSecond = false) => {
    if (!Array.isArray(r) || r.length < min) return error("invalid-dimension", path);
    r.forEach((p, k) => {
      if (vec(p, 2, `${path}/${k}`) && nonNegativeSecond && p[1]! < 0) error("invalid-dimension", `${path}/${k}`);
    });
  };
  const isRing = (r: unknown): r is [number, number][] => Array.isArray(r) && r.every((q) => Array.isArray(q) && q.length === 2 && q.every((x) => typeof x === "number" && Number.isFinite(x)));
  let mark = 0;
  document.solids.forEach((s, i) => {
    pace.tick(64, i / Math.max(1, document.solids.length) / 2);
    const p = `/solids/${i}`;
    if (!nodeIds.has(s.node)) error("unknown-node", `${p}/node`);
    if (!materialIds.has(s.material)) error("unknown-material", `${p}/material`);
    if (s.weights) {
      if (s.weights.length === 0 || s.weights.length > 4 || !(s.weights.reduce((a, w) => a + (w.weight > 0 ? w.weight : 0), 0) > 0)) error("bad-weights", `${p}/weights`);
      s.weights.forEach((w, k) => { if (!nodeIds.has(w.node)) error("unknown-node", `${p}/weights/${k}/node`); });
    }
    if (s.repeat && (!Number.isInteger(s.repeat.count) || s.repeat.count < 1 || s.repeat.count > 100000)) error("bad-repeat", `${p}/repeat/count`);
    if (s.transform) {
      if (s.transform.translation) vec(s.transform.translation, 3, `${p}/transform/translation`);
      if (s.transform.scale && vec(s.transform.scale, 3, `${p}/transform/scale`) && s.transform.scale.some((x) => x === 0)) error("zero-scale", `${p}/transform/scale`);
      if (s.transform.rotation && vec(s.transform.rotation, 4, `${p}/transform/rotation`) && Math.abs(portable.hypot(...s.transform.rotation) - 1) > 1e-3) error("non-unit-quaternion", `${p}/transform/rotation`);
    }
    mark = issues.length;
    switch (s.kind) {
      case "box":
        if (vec(s.size, 3, `${p}/size`)) s.size.forEach((x, k) => positive(x, `${p}/size/${k}`));
        if (s.cornerRadius !== undefined && (s.cornerRadius < 0 || (Array.isArray(s.size) && s.cornerRadius > Math.min(...s.size) / 2 + 1e-12))) error("invalid-dimension", `${p}/cornerRadius`);
        break;
      case "ellipsoid":
        if (vec(s.radii, 3, `${p}/radii`)) s.radii.forEach((x, k) => positive(x, `${p}/radii/${k}`));
        break;
      case "capsule":
        vec(s.p0, 3, `${p}/p0`);
        vec(s.p1, 3, `${p}/p1`);
        if (!(s.r0 >= 0) || !(s.r1 >= 0) || s.r0 + s.r1 <= 0) error("invalid-dimension", `${p}/r0`);
        if (s.lateralScale !== undefined) positive(s.lateralScale, `${p}/lateralScale`);
        break;
      case "revolve":
        ring(s.profile, 2, `${p}/profile`, true);
        if (issues.length === mark && isRing(s.profile)) {
          const { points, loop } = revolveOutline(s.profile);
          if (points.length < 2 || ringFault(points, loop, (q) => q[1] <= 1e-9, pace)) error("self-intersection", `${p}/profile`);
        }
        if (vec(s.axis, 3, `${p}/axis`) && portable.hypot(...s.axis) < 1e-9) error("invalid-dimension", `${p}/axis`);
        vec(s.origin, 3, `${p}/origin`);
        if (s.sweepRad !== undefined && !(s.sweepRad > 0 && s.sweepRad <= Math.PI * 2 + 1e-9)) error("invalid-dimension", `${p}/sweepRad`);
        if (s.lateralScale !== undefined) positive(s.lateralScale, `${p}/lateralScale`);
        break;
      case "extrude":
        ring(s.profile, 3, `${p}/profile`);
        (s.holes ?? []).forEach((h, k) => ring(h, 3, `${p}/holes/${k}`));
        if (issues.length === mark && isRing(s.profile) && (s.holes ?? []).every(isRing)) {
          const outer = cleanRing(s.profile), holes = (s.holes ?? []).map(cleanRing);
          const faulty = [outer, ...holes].map((r) => ringFault(r, true, undefined, pace));
          faulty.forEach((bad, k) => { if (bad) error("self-intersection", k === 0 ? `${p}/profile` : `${p}/holes/${k - 1}`); });
          if (!faulty.some(Boolean) && holesFault(outer, holes, pace)) error("bad-holes", `${p}/holes`);
        }
        if (!["xy", "yz", "zx"].includes(s.plane)) error("invalid-dimension", `${p}/plane`);
        if (!isNum(s.from) || !isNum(s.to) || s.from === s.to) error("invalid-dimension", `${p}/to`);
        if (s.taperTo && (!vec(s.taperTo, 2, `${p}/taperTo`) || s.taperTo.some((x) => x < 0))) error("invalid-dimension", `${p}/taperTo`);
        break;
      case "loft":
        if (!Array.isArray(s.sections) || s.sections.length < 2) error("invalid-dimension", `${p}/sections`);
        else s.sections.forEach((sec, k) => {
          vec(sec.at, 3, `${p}/sections/${k}/at`);
          positive(sec.rx, `${p}/sections/${k}/rx`);
          positive(sec.ry, `${p}/sections/${k}/ry`);
          if (sec.exponent !== undefined) positive(sec.exponent, `${p}/sections/${k}/exponent`);
          if (sec.rotation && vec(sec.rotation, 4, `${p}/sections/${k}/rotation`) && Math.abs(portable.hypot(...sec.rotation) - 1) > 1e-3) error("non-unit-quaternion", `${p}/sections/${k}/rotation`);
        });
        break;
      default:
        error("bad-type", `${p}/kind`);
    }
  });

  const skinJointCount = new Map<Slug, number>(document.skins.map((s) => [s.id, s.joints.length]));
  const skinOfMesh = new Map<Slug, Slug>();
  document.nodes.forEach((n) => { if (n.mesh && n.skin) skinOfMesh.set(n.mesh, n.skin); });
  const every = (values: ArrayLike<number>, ok: (x: number, at: number) => boolean): boolean => {
    for (let q = 0; q < values.length; q++) {
      pace.tick();
      if (!ok(values[q]!, q)) return false;
    }
    return true;
  };
  document.meshes.forEach((m, i) => m.primitives.forEach((pr, k) => {
    pace.step(0.5 + (0.5 * i) / Math.max(1, document.meshes.length));
    const p = `/meshes/${i}/primitives/${k}`;
    if (!materialIds.has(pr.material)) error("unknown-material", `${p}/material`);
    const stride = pr.mode === "lines" ? 2 : 3;
    if (pr.positions.length % 3 !== 0 || pr.indices.length % stride !== 0) return error("bad-primitive", p);
    const vertices = pr.positions.length / 3;
    if (!every(pr.positions, Number.isFinite)) error("non-finite", `${p}/positions`);
    if (pr.normals && pr.normals.length !== pr.positions.length) error("bad-primitive", `${p}/normals`);
    if (pr.colors && pr.colors.length !== vertices * 4) error("bad-primitive", `${p}/colors`);
    if (!!pr.joints !== !!pr.weights || (pr.joints && (pr.joints.length !== vertices * 4 || pr.weights!.length !== vertices * 4))) error("bad-primitive", `${p}/joints`);
    if (!every(pr.indices, (x) => Number.isInteger(x) && x >= 0 && x < vertices)) error("index-range", `${p}/indices`);
    if (pr.solids !== undefined) {
      let cursor = 0, valid = Array.isArray(pr.solids);
      if (valid) for (const range of pr.solids) {
        valid = valid && typeof range?.id === "string" && SLUG.test(range.id) && Number.isInteger(range.first) && Number.isInteger(range.count) && range.count >= 1 && range.first === cursor && range.count % stride === 0;
        cursor += valid ? range.count : 0;
      }
      if (!valid || cursor !== pr.indices.length) error("bad-solids", `${p}/solids`);
    }
    if (pr.joints && pr.weights) {
      const count = skinJointCount.get(skinOfMesh.get(m.id) ?? "");
      if (count !== undefined && !every(pr.joints, (j, q) => !(pr.weights![q]! > 0) || (j >= 0 && j < count))) error("joint-index", `${p}/joints`);
      for (let v = 0; v < vertices; v++) {
        const sum = pr.weights[v * 4]! + pr.weights[v * 4 + 1]! + pr.weights[v * 4 + 2]! + pr.weights[v * 4 + 3]!;
        if (Math.abs(sum - 1) > 1e-3) { add("warning", "weight-sum", `${p}/weights`); break; }
      }
    }
  }));
  document.skins.forEach((s, i) => {
    s.joints.forEach((j, k) => { if (!nodeIds.has(j)) error("unknown-node", `/skins/${i}/joints/${k}`); });
    if (s.inverseBind.length !== s.joints.length || s.inverseBind.some((m) => !Array.isArray(m) || m.length !== 16 || !m.every(Number.isFinite))) error("bad-skin", `/skins/${i}/inverseBind`);
  });
  document.materials.forEach((m, i) => {
    const p = `/materials/${i}`;
    if (vec(m.baseColor, 4, `${p}/baseColor`) && m.baseColor.some((c) => c < 0 || c > 1)) error("bad-color", `${p}/baseColor`);
    if (vec(m.emissive, 3, `${p}/emissive`) && m.emissive.some((c) => c < 0)) error("bad-color", `${p}/emissive`);
    if (!(m.metallic >= 0 && m.metallic <= 1)) error("bad-color", `${p}/metallic`);
    if (!(m.roughness >= 0 && m.roughness <= 1)) error("bad-color", `${p}/roughness`);
  });
  document.variants.forEach((v, i) => {
    for (const [from, to] of Object.entries(v.bindings)) {
      if (!materialIds.has(from)) error("unknown-material", `/variants/${i}/bindings/${from}`);
      if (!materialIds.has(to)) error("unknown-material", `/variants/${i}/bindings/${from}`);
    }
  });
  document.poses.forEach((pose, i) => {
    for (const [id, t] of Object.entries(pose.nodes)) {
      const p = `/poses/${i}/nodes/${id}`;
      if (!nodeIds.has(id)) error("unknown-node", p);
      if (t.translation) vec(t.translation, 3, `${p}/translation`);
      if (t.scale) vec(t.scale, 3, `${p}/scale`);
      if (t.rotation && vec(t.rotation, 4, `${p}/rotation`) && Math.abs(portable.hypot(...t.rotation) - 1) > 1e-3) error("non-unit-quaternion", `${p}/rotation`);
    }
  });
  document.animations.forEach((a, i) => {
    const p = `/animations/${i}`;
    if (!(a.durationS > 0)) error("bad-duration", `${p}/durationS`);
    a.channels.forEach((c, k) => {
      const cp = `${p}/channels/${k}`;
      const target = c.target as { node?: Slug; material?: Slug; path: keyof typeof COMPONENTS };
      if (target.node !== undefined && !nodeIds.has(target.node)) error("unknown-node", `${cp}/target/node`);
      if (target.material !== undefined && !materialIds.has(target.material)) error("unknown-material", `${cp}/target/material`);
      const width = COMPONENTS[target.path];
      if (!width || c.values.length !== c.times.length * width || !c.values.every(Number.isFinite)) error("bad-channel", cp);
      if (c.times.some((t, q) => !Number.isFinite(t) || t < 0 || (q > 0 && t <= c.times[q - 1]!))) error("bad-times", `${cp}/times`);
      else if (a.durationS > 0 && c.times.length > 0 && c.times[c.times.length - 1]! > a.durationS + 1e-9) error("bad-times", `${cp}/times`);
      if (target.path === "rotation" && width === 4) for (let q = 0; q < c.times.length; q++) if (Math.abs(portable.hypot(c.values[q * 4]!, c.values[q * 4 + 1]!, c.values[q * 4 + 2]!, c.values[q * 4 + 3]!) - 1) > 1e-3) { error("non-unit-quaternion", `${cp}/values`); break; }
    });
    a.events.forEach((e, k) => { if (!(e.t >= 0 && e.t <= a.durationS + 1e-9)) error("bad-times", `${p}/events/${k}/t`); });
  });
  return issues;
}
