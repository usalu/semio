/** ✂️ The model edit diffs of the brepjs kernel: edge segments, join, explode, split, trim and the corner connector.
 *
 * @see ../🟦️.ts — the brepjs kernel module, kept ONLY as a vitest differential oracle
 */
import { type ModelDiff, type EdgeRecordDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { type EdgeCurve, type Vec3 } from "@semio-tech/framework-3d-js";
import { geom, type EdgeRecord, type EdgeRef, type ShellRef, type SolidRef, type VertexRecord, type VertexRef, type WireRecord, type WireRef } from "@semio-tech/s-spatial-kernel-ts/preview/brep-records";
import { vec3Add, vec3Distance, vec3Dot, vec3Normalize, vec3Scale, vec3Sub } from "@semio-tech/s-spatial-kernel-ts/preview/vec3";
import { edgeIdsFromPicks, freshEditRef, picksFromValue } from "../🔗️wires-and-selection/🟦️.ts";

// #region 🔌️BrepModelBridge
// #region ✂️EditTopologyOps
type EdgeSegment = { readonly p0: Vec3; readonly p1: Vec3 };

function edgeSegment(model: Model, id: string): EdgeSegment | null {
  const g = geom(model);
  const e = g.edges[id];
  if (!e || e.vertexIds.length < 2) return null;
  const p0 = g.vertices[String(e.vertexIds[0])]?.position;
  const p1 = g.vertices[String(e.vertexIds[1])]?.position;
  if (!p0 || !p1) return null;
  return { p0, p1 };
}

function clampNumber(x: number, lo: number, hi: number): number {
  return x < lo ? lo : x > hi ? hi : x;
}

/** 📏️ Closest points between two 3D segments (Ericson's segment-segment algorithm); `t` is the parameter (0..1) on the first segment. */
function closestPointsOnSegments(p1: Vec3, p2: Vec3, q1: Vec3, q2: Vec3): { readonly onFirst: Vec3; readonly onSecond: Vec3; readonly t: number } {
  const d1 = vec3Sub(p2, p1);
  const d2 = vec3Sub(q2, q1);
  const r = vec3Sub(p1, q1);
  const a = vec3Dot(d1, d1);
  const e = vec3Dot(d2, d2);
  const f = vec3Dot(d2, r);
  const EPS = 1e-12;
  let s = 0;
  let t = 0;
  if (a <= EPS && e <= EPS) {
    s = 0;
    t = 0;
  } else if (a <= EPS) {
    s = 0;
    t = clampNumber(f / e, 0, 1);
  } else {
    const c = vec3Dot(d1, r);
    if (e <= EPS) {
      t = 0;
      s = clampNumber(-c / a, 0, 1);
    } else {
      const b = vec3Dot(d1, d2);
      const denom = a * e - b * b;
      s = denom !== 0 ? clampNumber((b * f - c * e) / denom, 0, 1) : 0;
      t = (b * s + f) / e;
      if (t < 0) {
        t = 0;
        s = clampNumber(-c / a, 0, 1);
      } else if (t > 1) {
        t = 1;
        s = clampNumber((b - c) / a, 0, 1);
      }
    }
  }
  return { onFirst: vec3Add(p1, vec3Scale(d1, s)), onSecond: vec3Add(q1, vec3Scale(d2, t)), t: s };
}

/** 🧷️ `edit.join`: merges the edges of the selected wires/edges/faces into one new wire (topological grouping, no geometric coincidence required). */
export function editJoinDiff(model: Model, params: Record<string, unknown>): ModelDiff {
  const g = geom(model);
  const picks = picksFromValue(params.targets);
  const removedWires: WireRef[] = [];
  const edgeIds: EdgeRef[] = [];
  const seen = new Set<string>();
  const pushEdges = (ids: readonly EdgeRef[]) => {
    for (const id of ids) {
      if (seen.has(id)) continue;
      seen.add(id);
      edgeIds.push(id);
    }
  };
  for (const pick of picks) {
    if (pick.kind === "wire") {
      const w = g.wires[pick.id];
      if (!w) continue;
      pushEdges(w.edgeIds);
      removedWires.push(pick.id as WireRef);
      continue;
    }
    if (pick.kind === "edge") {
      if (g.edges[pick.id]) pushEdges([pick.id as EdgeRef]);
      continue;
    }
    if (pick.kind === "face") {
      const f = g.faces[pick.id];
      if (!f) continue;
      for (const wireId of f.wireIds) {
        const w = g.wires[wireId];
        if (!w) continue;
        pushEdges(w.edgeIds);
        removedWires.push(wireId);
      }
    }
  }
  if (edgeIds.length < 2) return {};
  const joined: WireRecord = { id: freshEditRef("wire-join") as WireRef, edgeIds };
  return removedWires.length ? { wires: { added: [joined], removed: removedWires } } : { wires: { added: [joined] } };
}

/** 💥️ `edit.explode`: inverse of `edit.join` — decomposes each selected wire into one single-edge wire per member edge; shells/solids explode by dropping their container record. */
export function editExplodeDiff(model: Model, params: Record<string, unknown>): ModelDiff {
  const g = geom(model);
  const picks = picksFromValue(params.targets);
  const addedWires: WireRecord[] = [];
  const removedWires: WireRef[] = [];
  const removedShells: ShellRef[] = [];
  const removedSolids: SolidRef[] = [];
  for (const pick of picks) {
    if (pick.kind === "wire") {
      const w = g.wires[pick.id];
      if (!w || w.edgeIds.length < 2) continue;
      for (const edgeId of w.edgeIds) addedWires.push({ id: freshEditRef("wire-explode") as WireRef, edgeIds: [edgeId] });
      removedWires.push(pick.id as WireRef);
      continue;
    }
    if (pick.kind === "shell") {
      if (g.shells[pick.id]) removedShells.push(pick.id as ShellRef);
      continue;
    }
    if (pick.kind === "solid") {
      if (g.solids[pick.id]) removedSolids.push(pick.id as SolidRef);
    }
  }
  const diff: { -readonly [K in keyof ModelDiff]?: ModelDiff[K] } = {};
  if (addedWires.length || removedWires.length) diff.wires = { ...(addedWires.length ? { added: addedWires } : {}), ...(removedWires.length ? { removed: removedWires } : {}) };
  if (removedShells.length) diff.shells = { removed: removedShells };
  if (removedSolids.length) diff.solids = { removed: removedSolids };
  return diff;
}

/** ✂️ Splits `edgeId` at parameter `s` (0..1 along its segment), rewiring the containing wire if any. New sub-edges default to straight lines (honest chord approximation for non-line curves). */
function splitEdgeAt(model: Model, edgeId: EdgeRef, seg: EdgeSegment, s: number): ModelDiff {
  const g = geom(model);
  const edge = g.edges[edgeId];
  if (!edge) return {};
  const v0 = edge.vertexIds[0]! as VertexRef;
  const v1 = edge.vertexIds[1]! as VertexRef;
  const splitPos = vec3Add(seg.p0, vec3Scale(vec3Sub(seg.p1, seg.p0), s));
  const vSplit: VertexRecord = { id: freshEditRef("v-split") as VertexRef, position: splitPos };
  const eA: EdgeRecord = { id: freshEditRef("e-split") as EdgeRef, vertexIds: [v0, vSplit.id] };
  const eB: EdgeRecord = { id: freshEditRef("e-split") as EdgeRef, vertexIds: [vSplit.id, v1] };
  const wire = Object.values(g.wires).find((w) => w.edgeIds.includes(edgeId));
  const diff: { -readonly [K in keyof ModelDiff]?: ModelDiff[K] } = {
    vertices: { added: [vSplit] },
    edges: { added: [eA, eB], removed: [edgeId] },
  };
  if (wire) {
    const idx = wire.edgeIds.indexOf(edgeId);
    const newIds = [...wire.edgeIds.slice(0, idx), eA.id, eB.id, ...wire.edgeIds.slice(idx + 1)];
    diff.wires = { modified: [{ id: wire.id, edgeIds: newIds }] };
  }
  return diff;
}

/** ✂️ `edit.split`: cuts the split-object edge closest to the cutting reference into two edges at their closest-approach point. */
export function editSplitDiff(model: Model, params: Record<string, unknown>): ModelDiff {
  const targetIds = edgeIdsFromPicks(model, picksFromValue(params.splitObjects));
  const cutterIds = edgeIdsFromPicks(model, picksFromValue(params.cutters));
  if (!targetIds.length || !cutterIds.length) return {};
  const cutterSeg = edgeSegment(model, cutterIds[0]!);
  if (!cutterSeg) return {};
  let bestId: EdgeRef | null = null;
  let bestSeg: EdgeSegment | null = null;
  let bestS = 0.5;
  let bestDist = Infinity;
  for (const id of targetIds) {
    const seg = edgeSegment(model, id);
    if (!seg) continue;
    const cp = closestPointsOnSegments(seg.p0, seg.p1, cutterSeg.p0, cutterSeg.p1);
    const dist = vec3Distance(cp.onFirst, cp.onSecond);
    if (dist < bestDist) {
      bestDist = dist;
      bestId = id;
      bestSeg = seg;
      bestS = clampNumber(cp.t, 0.15, 0.85);
    }
  }
  if (!bestId || !bestSeg) return {};
  return splitEdgeAt(model, bestId, bestSeg, bestS);
}

/** ✂️ `edit.trim`: trims the object edge closest to the cutting reference, discarding the shorter side of the closest-approach split point. */
export function editTrimDiff(model: Model, params: Record<string, unknown>): ModelDiff {
  const g = geom(model);
  const cutterIds = edgeIdsFromPicks(model, picksFromValue(params.cutters));
  const targetIds = edgeIdsFromPicks(model, picksFromValue(params.trimmedObjects));
  if (!cutterIds.length || !targetIds.length) return {};
  let bestTargetId: EdgeRef | null = null;
  let bestSeg: EdgeSegment | null = null;
  let bestS = 0.5;
  let bestDist = Infinity;
  for (const targetId of targetIds) {
    const targetSeg = edgeSegment(model, targetId);
    if (!targetSeg) continue;
    for (const cutterId of cutterIds) {
      const cutterSeg = edgeSegment(model, cutterId);
      if (!cutterSeg) continue;
      const cp = closestPointsOnSegments(targetSeg.p0, targetSeg.p1, cutterSeg.p0, cutterSeg.p1);
      const dist = vec3Distance(cp.onFirst, cp.onSecond);
      if (dist < bestDist) {
        bestDist = dist;
        bestTargetId = targetId;
        bestSeg = targetSeg;
        bestS = clampNumber(cp.t, 0.15, 0.85);
      }
    }
  }
  if (!bestTargetId || !bestSeg) return {};
  const edge = g.edges[bestTargetId];
  if (!edge) return {};
  const v0 = edge.vertexIds[0]! as VertexRef;
  const v1 = edge.vertexIds[1]! as VertexRef;
  const splitPos = vec3Add(bestSeg.p0, vec3Scale(vec3Sub(bestSeg.p1, bestSeg.p0), bestS));
  const vSplit: VertexRecord = { id: freshEditRef("v-trim") as VertexRef, position: splitPos };
  const keepFirstHalf = bestS >= 0.5;
  const keptEdge: EdgeRecord = { id: freshEditRef("e-trim") as EdgeRef, vertexIds: keepFirstHalf ? [v0, vSplit.id] : [vSplit.id, v1] };
  const droppedVertexId = keepFirstHalf ? v1 : v0;
  const stillReferenced = Object.values(g.edges).some((e) => e.id !== bestTargetId && e.vertexIds.includes(droppedVertexId));
  const wire = Object.values(g.wires).find((w) => w.edgeIds.includes(bestTargetId!));
  const diff: { -readonly [K in keyof ModelDiff]?: ModelDiff[K] } = {
    vertices: stillReferenced ? { added: [vSplit] } : { added: [vSplit], removed: [droppedVertexId] },
    edges: { added: [keptEdge], removed: [bestTargetId] },
  };
  if (wire) {
    const idx = wire.edgeIds.indexOf(bestTargetId);
    const newIds = [...wire.edgeIds.slice(0, idx), keptEdge.id, ...wire.edgeIds.slice(idx + 1)];
    diff.wires = { modified: [{ id: wire.id, edgeIds: newIds }] };
  }
  return diff;
}

function otherVertexOf(vertexIds: readonly VertexRef[], id: VertexRef): VertexRef {
  return (vertexIds[0] === id ? vertexIds[1] : vertexIds[0])! as VertexRef;
}

/** 📐️ Replaces the span between `startId` and `endId` in `wire.edgeIds` (inclusive ends, kept) with `replacement`, in whichever array order they appear. Returns `null` if either id is absent. */
function spliceWireSpan(wire: WireRecord, startId: EdgeRef, endId: EdgeRef, replacement: EdgeRef): readonly EdgeRef[] | null {
  const ids = wire.edgeIds;
  const i0 = ids.indexOf(startId);
  const i1 = ids.indexOf(endId);
  if (i0 === -1 || i1 === -1 || i0 === i1) return null;
  if (i0 < i1) return [...ids.slice(0, i0 + 1), replacement, ...ids.slice(i1)];
  return [...ids.slice(0, i1 + 1), replacement, ...ids.slice(i0)];
}

/** 📐️ Shared builder for `edit.chamfer` (straight connector) and `edit.fillet` (tangent-arc connector): bridges two curves at their shared vertex, their single connecting edge, or (failing that) their nearest endpoints — extending both lines to a virtual corner when no direct link exists. */
export function cornerConnectorDiff(model: Model, edgeAId: EdgeRef, edgeBId: EdgeRef, style: "chamfer" | "fillet"): ModelDiff {
  const g = geom(model);
  const edgeA = g.edges[edgeAId];
  const edgeB = g.edges[edgeBId];
  if (!edgeA || !edgeB || edgeA.vertexIds.length < 2 || edgeB.vertexIds.length < 2 || edgeAId === edgeBId) return {};
  const aIds = edgeA.vertexIds as VertexRef[];
  const bIds = edgeB.vertexIds as VertexRef[];
  const posOf = (id: VertexRef): Vec3 | null => g.vertices[id]?.position ?? null;

  let nearA: VertexRef;
  let farA: VertexRef;
  let nearB: VertexRef;
  let farB: VertexRef;
  let bridgeId: EdgeRef | null = null;
  const shared = aIds.find((id) => bIds.includes(id));
  if (shared) {
    nearA = shared;
    farA = otherVertexOf(aIds, shared);
    nearB = shared;
    farB = otherVertexOf(bIds, shared);
  } else {
    let bridge: { a: VertexRef; b: VertexRef; id: EdgeRef } | null = null;
    outer: for (const a of aIds) {
      for (const b of bIds) {
        const cand = Object.values(g.edges).find((e) => e.id !== edgeAId && e.id !== edgeBId && e.vertexIds.includes(a) && e.vertexIds.includes(b));
        if (cand) {
          bridge = { a, b, id: cand.id };
          break outer;
        }
      }
    }
    if (bridge) {
      nearA = bridge.a;
      farA = otherVertexOf(aIds, bridge.a);
      nearB = bridge.b;
      farB = otherVertexOf(bIds, bridge.b);
      bridgeId = bridge.id;
    } else {
      let bestDist = Infinity;
      let bestA: VertexRef = aIds[0]!;
      let bestB: VertexRef = bIds[0]!;
      for (const a of aIds) {
        for (const b of bIds) {
          const pa = posOf(a);
          const pb = posOf(b);
          if (!pa || !pb) continue;
          const dist = vec3Distance(pa, pb);
          if (dist < bestDist) {
            bestDist = dist;
            bestA = a;
            bestB = b;
          }
        }
      }
      nearA = bestA;
      farA = otherVertexOf(aIds, bestA);
      nearB = bestB;
      farB = otherVertexOf(bIds, bestB);
    }
  }
  const pNearA = posOf(nearA);
  const pFarA = posOf(farA);
  const pNearB = posOf(nearB);
  const pFarB = posOf(farB);
  if (!pNearA || !pFarA || !pNearB || !pFarB) return {};

  const dirA = vec3Normalize(vec3Sub(pFarA, pNearA));
  const dirB = vec3Normalize(vec3Sub(pFarB, pNearB));
  let corner: Vec3;
  if (nearA === nearB) {
    corner = pNearA;
  } else {
    const big = 1e6;
    const cp = closestPointsOnSegments(vec3Sub(pNearA, vec3Scale(dirA, big)), vec3Add(pNearA, vec3Scale(dirA, big)), vec3Sub(pNearB, vec3Scale(dirB, big)), vec3Add(pNearB, vec3Scale(dirB, big)));
    corner = vec3Distance(cp.onFirst, cp.onSecond) < 1e-3 ? vec3Scale(vec3Add(cp.onFirst, cp.onSecond), 0.5) : vec3Scale(vec3Add(pNearA, pNearB), 0.5);
  }

  const lenA = vec3Distance(pFarA, pNearA);
  const lenB = vec3Distance(pFarB, pNearB);
  const d = Math.max(Math.min(lenA, lenB) * 0.3, 1e-6);
  const tA = clampNumber(vec3Dot(vec3Sub(vec3Add(corner, vec3Scale(dirA, d)), pNearA), dirA), lenA * 0.05, lenA * 0.95);
  const tB = clampNumber(vec3Dot(vec3Sub(vec3Add(corner, vec3Scale(dirB, d)), pNearB), dirB), lenB * 0.05, lenB * 0.95);
  const TA = vec3Add(pNearA, vec3Scale(dirA, tA));
  const TB = vec3Add(pNearB, vec3Scale(dirB, tB));

  const vTA: VertexRecord = { id: freshEditRef("v-corner") as VertexRef, position: TA };
  const vTB: VertexRecord = { id: freshEditRef("v-corner") as VertexRef, position: TB };

  let connectorCurve: EdgeCurve = { kind: "line" };
  if (style === "fillet") {
    const uA = vec3Normalize(vec3Sub(TA, corner));
    const uB = vec3Normalize(vec3Sub(TB, corner));
    const cosTheta = clampNumber(vec3Dot(uA, uB), -1, 1);
    const theta = Math.acos(cosTheta);
    if (theta > 1e-3 && theta < Math.PI - 1e-3) {
      const bis = vec3Normalize(vec3Add(uA, uB));
      const centerDist = d / Math.cos(theta / 2);
      connectorCurve = { kind: "arc", center: vec3Add(corner, vec3Scale(bis, centerDist)) };
    }
  }
  const connector: EdgeRecord = { id: freshEditRef("e-corner") as EdgeRef, vertexIds: [vTA.id, vTB.id], curve: connectorCurve };

  const modA: EdgeRecordDiff = { id: edgeAId, vertexIds: aIds.map((id) => (id === nearA ? vTA.id : id)) };
  const modB: EdgeRecordDiff = { id: edgeBId, vertexIds: bIds.map((id) => (id === nearB ? vTB.id : id)) };

  const stillUsed = (vid: VertexRef): boolean => Object.values(g.edges).some((e) => e.id !== edgeAId && e.id !== edgeBId && e.id !== bridgeId && e.vertexIds.includes(vid));
  const removedVertexIds = [...new Set<VertexRef>([nearA, nearB])].filter((vid) => !stillUsed(vid));

  const wire = Object.values(g.wires).find((w) => w.edgeIds.includes(edgeAId) && w.edgeIds.includes(edgeBId) && (!bridgeId || w.edgeIds.includes(bridgeId)));
  const splicedIds = wire ? spliceWireSpan(wire, edgeAId, edgeBId, connector.id) : null;
  const wireDiff = wire && splicedIds ? { modified: [{ id: wire.id, edgeIds: splicedIds }] } : undefined;

  const diff: { -readonly [K in keyof ModelDiff]?: ModelDiff[K] } = {
    vertices: removedVertexIds.length ? { added: [vTA, vTB], removed: removedVertexIds } : { added: [vTA, vTB] },
    edges: bridgeId ? { added: [connector], modified: [modA, modB], removed: [bridgeId] } : { added: [connector], modified: [modA, modB] },
  };
  if (wireDiff) diff.wires = wireDiff;
  return diff;
}
// #endregion ✂️EditTopologyOps
// #endregion 🔌️BrepModelBridge
