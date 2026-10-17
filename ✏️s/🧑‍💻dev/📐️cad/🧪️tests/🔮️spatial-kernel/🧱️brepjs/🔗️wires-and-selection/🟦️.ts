/** 🔗️ The wire and selection helpers of the brepjs kernel: model edges and wires as brep shapes, oriented faces, selection picks and the edge ids of picks.
 *
 * @see ../🟦️.ts — the brepjs kernel module, kept ONLY as a vitest differential oracle
 */
import { bsplineApprox, circle, extrude, face, filledFace, healSolid, loft, thicken, translate, wire, isOk, isSolid, isValidSolid, line, threePointArc, wireLoop, type OwnedBrepEdge as Edge, type OwnedBrepOrientedFace as OrientedFace, type OwnedBrepShape as Shape3D, type OwnedBrepSolid as ValidSolid, type OwnedBrepWire as Wire } from "@semio-tech/cad-engine-ts/brepjs";
import { type ModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { type Vec3 } from "@semio-tech/framework-3d-js";
import { arcSamplePoints, ellipseSamplePoints, nurbsDisplaySamplePoints } from "@semio-tech/s-spatial-kernel-ts/preview/curve-sampling";
import { geom, type EdgeRecord, type EdgeRef, type SolidRef, type WireRef } from "@semio-tech/s-spatial-kernel-ts/preview/brep-records";
import { vec3Distance, vec3Normalize, vec3Sub } from "@semio-tech/s-spatial-kernel-ts/preview/vec3";
import { extrudeDirection } from "../🧩️opencascade-and-scratch/🟦️.ts";

// #region 🔌️BrepModelBridge
/** 🔗️ Builds a brepjs `Edge` from a model edge record. */
export function geomEdgeToBrepEdge(model: Model, edge: EdgeRecord): Edge | null {
  const ids = edge.vertexIds;
  if (ids.length < 1) return null;
  const p0 = geom(model).vertices[String(ids[0])]?.position;
  const p1 = geom(model).vertices[String(ids[1] ?? ids[0])]?.position;
  if (!p0 || !p1) return null;
  const c = edge.curve;
  if (!c || c.kind === "line") return line(p0, p1);
  if (c.kind === "circle") {
    const e = circle(c.radius, { at: c.center, axis: c.normal });
    return e;
  }
  if (c.kind === "arc") {
    const mid = arcSamplePoints(c.center, p0, p1, 4)[2] ?? p1;
    return threePointArc(p0, mid, p1);
  }
  if (c.kind === "nurbs" && c.poles.length >= 2) {
    const fitPoints = c.through ? [...nurbsDisplaySamplePoints(c.poles, 16)] : [...c.poles];
    const r = bsplineApprox(fitPoints);
    if (isOk(r)) return r.value;
  }
  if (c.kind === "ellipse") {
    const samples = ellipseSamplePoints(c.center, c.normal, c.majorAxis, c.majorRadius, c.minorRadius, 32);
    const r = bsplineApprox([...(samples.length >= 2 ? samples : [p0, p1])]);
    if (isOk(r)) return r.value;
  }
  return line(p0, p1);
}

/** 🔗️ brepjs wire from a model wire (closed `wireLoop` or open `wire`). */
export function geomWireToBrepWire(model: Model, wireId: WireRef): Wire | null {
  const w = geom(model).wires[wireId];
  if (!w?.edgeIds.length) return null;
  const edges: Edge[] = [];
  for (const eid of w.edgeIds) {
    const rec = geom(model).edges[eid];
    if (!rec) return null;
    const be = geomEdgeToBrepEdge(model, rec);
    if (!be) return null;
    edges.push(be);
  }
  const loop = wireLoop(edges);
  if (isOk(loop)) return loop.value;
  const open = wire(edges);
  return isOk(open) ? open.value : null;
}

/** 🔗️ Closed planar brepjs face from a model wire (`wireLoop` + `face`). */
export function geomWireToOrientedFace(model: Model, wireId: WireRef): OrientedFace | null {
  const w = geom(model).wires[wireId];
  if (!w?.edgeIds.length) return null;
  const edges: Edge[] = [];
  for (const eid of w.edgeIds) {
    const rec = geom(model).edges[eid];
    if (!rec) return null;
    const be = geomEdgeToBrepEdge(model, rec);
    if (!be) return null;
    edges.push(be);
  }
  const cw = wireLoop(edges);
  if (!isOk(cw)) return null;
  const f = face(cw.value as Parameters<typeof face>[0]);
  return isOk(f) ? f.value : null;
}

/** 🔗️ Oriented face from a model wire; uses `filledFace` when planar `face` fails (deformed boxes). */
export function geomWireToOrientedFaceLoose(model: Model, wireId: WireRef): OrientedFace | null {
  const planar = geomWireToOrientedFace(model, wireId);
  if (planar) return planar;
  const w = geom(model).wires[wireId];
  if (!w?.edgeIds.length) return null;
  const edges: Edge[] = [];
  for (const eid of w.edgeIds) {
    const rec = geom(model).edges[eid];
    if (!rec) return null;
    const be = geomEdgeToBrepEdge(model, rec);
    if (!be) return null;
    edges.push(be);
  }
  const cw = wireLoop(edges);
  if (!isOk(cw)) return null;
  const filled = filledFace(cw.value as Parameters<typeof filledFace>[0]);
  return isOk(filled) ? filled.value : null;
}

/** 🔗️ Extrudes a model wire to a `ValidSolid` via brepjs (planar face or open-curve loft). */
export function extrudeModelWire(model: Model, wireId: string, direction: Vec3, distance: number): ValidSolid | null {
  const wid = wireId as WireRef;
  const vec = extrudeDirection(direction, distance);
  const planar = geomWireToOrientedFace(model, wid) ?? geomWireToOrientedFaceLoose(model, wid);
  if (planar) {
    const solid = extrude(planar as Parameters<typeof extrude>[0], vec);
    return isOk(solid) ? (solid.value as ValidSolid) : null;
  }
  const profile = geomWireToBrepWire(model, wid);
  if (!profile) return null;
  const moved = translate(profile, vec);
  const lofted = loft([profile, moved], { ruled: true });
  if (!isOk(lofted)) return null;
  const shape = lofted.value as Shape3D;
  if (isSolid(shape)) {
    if (isValidSolid(shape)) return shape as ValidSolid;
    const healed = healSolid(shape);
    if (isOk(healed) && isValidSolid(healed.value)) return healed.value as ValidSolid;
    return shape as ValidSolid;
  }
  const thickened = thicken(shape as Parameters<typeof thicken>[0], 1e-3);
  if (isOk(thickened) && isSolid(thickened.value)) return thickened.value as ValidSolid;
  return null;
}

type SelectionPick = { readonly kind: string; readonly id: string };

export function selectionPicksFromParams(params: Record<string, unknown>): SelectionPick[] {
  const raw = params.curves ?? params.targets ?? [];
  if (!Array.isArray(raw)) return [];
  const out: SelectionPick[] = [];
  for (const row of raw) {
    if (!row || typeof row !== "object") continue;
    const kind = (row as { kind?: unknown }).kind;
    const id = (row as { id?: unknown }).id;
    if (typeof kind === "string" && typeof id === "string") out.push({ kind, id });
  }
  return out;
}

/** 🧵️ Resolves wire ids from curve selection targets (`wire` or parent wire of `edge`). */
export function wireIdsFromSelectionPicks(model: Model, picks: readonly SelectionPick[]): WireRef[] {
  const g = geom(model);
  const out: WireRef[] = [];
  const seen = new Set<string>();
  for (const pick of picks) {
    if (pick.kind === "wire") {
      if (g.wires[pick.id] && !seen.has(pick.id)) {
        seen.add(pick.id);
        out.push(pick.id as WireRef);
      }
      continue;
    }
    if (pick.kind !== "edge") continue;
    for (const wire of Object.values(g.wires)) {
      if (!wire.edgeIds.includes(pick.id as EdgeRef) || seen.has(wire.id)) continue;
      seen.add(wire.id);
      out.push(wire.id);
    }
  }
  return out;
}

export function mergeSolidAdds(diffs: readonly ModelDiff[]): ModelDiff {
  const added = diffs.flatMap((d) => d.solids?.added ?? []);
  return added.length ? { solids: { added } } : {};
}

/** 🧩️ Parses raw `SelectionTarget[]`-shaped context data into `{kind,id}` picks (any field). */
export function picksFromRaw(raw: unknown): SelectionPick[] {
  if (!Array.isArray(raw)) return [];
  const out: SelectionPick[] = [];
  for (const row of raw) {
    if (!row || typeof row !== "object") continue;
    const kind = (row as { kind?: unknown }).kind;
    const id = (row as { id?: unknown }).id;
    if (typeof kind === "string" && typeof id === "string") out.push({ kind, id });
  }
  return out;
}

/** 🧱️ Resolves solid ids from raw `SelectionTarget[]`-shaped selection context (boolean operands). */
export function solidRefsFromSelectionRaw(model: Model, raw: unknown): SolidRef[] {
  const g = geom(model);
  const out: SolidRef[] = [];
  const seen = new Set<string>();
  for (const pick of picksFromRaw(raw)) {
    if (pick.kind !== "solid" || !g.solids[pick.id] || seen.has(pick.id)) continue;
    seen.add(pick.id);
    out.push(pick.id as SolidRef);
  }
  return out;
}

/** 🧵️ Resolves wire ids from a raw `targets` context field that may be a flat array or `command.addSelection`'s keyed sub-object (e.g. `{rail:[...]}`, `{railA:[...],railB:[...]}`). */
export function wireIdsFromKeyedRaw(model: Model, raw: unknown): WireRef[] {
  if (Array.isArray(raw)) return wireIdsFromSelectionPicks(model, picksFromRaw(raw));
  if (!raw || typeof raw !== "object") return [];
  const out: WireRef[] = [];
  const seen = new Set<string>();
  for (const value of Object.values(raw as Record<string, unknown>)) {
    for (const id of wireIdsFromSelectionPicks(model, picksFromRaw(value))) {
      if (seen.has(id)) continue;
      seen.add(id);
      out.push(id);
    }
  }
  return out;
}

/** 🧵️ Synthesizes a small circular cross-section wire when `surface.sweep{1,2}` receives no explicit profile curve. */
export function defaultSweepProfileWire(model: Model, railWireId: WireRef): Wire | null {
  const g = geom(model);
  const rail = g.wires[railWireId];
  const firstEdgeId = rail?.edgeIds[0];
  const firstEdge = firstEdgeId ? g.edges[firstEdgeId] : undefined;
  if (!firstEdge) return null;
  const p0 = g.vertices[String(firstEdge.vertexIds[0])]?.position;
  if (!p0) return null;
  const p1 = g.vertices[String(firstEdge.vertexIds[1] ?? firstEdge.vertexIds[0])]?.position;
  const tangent = p1 && vec3Distance(p0, p1) > 1e-9 ? vec3Normalize(vec3Sub(p1, p0)) : ([0, 0, 1] as Vec3);
  let radius = 0.25;
  let maxD = 0;
  for (const eid of rail?.edgeIds ?? []) {
    const e = g.edges[eid];
    if (!e) continue;
    for (const vid of e.vertexIds) {
      const p = g.vertices[String(vid)]?.position;
      if (p) maxD = Math.max(maxD, vec3Distance(p0, p));
    }
  }
  if (maxD > 1e-9) radius = Math.max(maxD * 0.15, 0.05);
  const profileEdge = circle(radius, { at: p0, axis: tangent });
  const loopResult = wireLoop([profileEdge]);
  return isOk(loopResult) ? (loopResult.value as Wire) : null;
}

// #region ✂️EditTopologyOps
/** 🧷️ Parses a raw selection-target array (`{kind,id}[]`) from an `edit.*` command param. */
export function picksFromValue(raw: unknown): SelectionPick[] {
  if (!Array.isArray(raw)) return [];
  const out: SelectionPick[] = [];
  for (const row of raw) {
    if (!row || typeof row !== "object") continue;
    const kind = (row as { kind?: unknown }).kind;
    const id = (row as { id?: unknown }).id;
    if (typeof kind === "string" && typeof id === "string") out.push({ kind, id });
  }
  return out;
}

export function freshEditRef(kind: string): string {
  return `brepjs-${kind}-${Math.random().toString(36).slice(2, 9)}`;
}

/** 🧷️ Flattens picks (`edge` verbatim, `wire`/`face` expand to member edges) into concrete edge ids. */
export function edgeIdsFromPicks(model: Model, picks: readonly SelectionPick[]): EdgeRef[] {
  const g = geom(model);
  const out: EdgeRef[] = [];
  for (const pick of picks) {
    if (pick.kind === "edge") {
      if (g.edges[pick.id]) out.push(pick.id as EdgeRef);
      continue;
    }
    if (pick.kind === "wire") {
      const w = g.wires[pick.id];
      if (w) out.push(...w.edgeIds);
      continue;
    }
    if (pick.kind === "face") {
      const f = g.faces[pick.id];
      if (!f) continue;
      for (const wireId of f.wireIds) {
        const w = g.wires[wireId];
        if (w) out.push(...w.edgeIds);
      }
    }
  }
  return out;
}
// #endregion ✂️EditTopologyOps
// #endregion 🔌️BrepModelBridge
