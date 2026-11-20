/** 🖋️ Projects the Three host's mesh geometry into layered shot linework. */
import * as THREE from "three";
import type { IconRenderRequest } from "@semio-tech/ui-styling";

import { Occluders, clipEdge, clipTriangle, projectPoint, signedArea, type ClipPoint, type Edge } from "../📐️line-visibility/🟦️.ts";

const SVG_NAMESPACE = "http://www.w3.org/2000/svg";
const HOST_YIELD_BUDGET = 1024;

type Drawing = NonNullable<IconRenderRequest["drawing"]>;
type Point2 = readonly [number, number];
type Line = readonly [Point2, Point2];

interface LineStyle {
  readonly color: string;
  readonly width: number;
}

/** 🪞️ Materializes instantiated and deformed mesh positions for the SVG renderer and linework alike. */
export async function prepareIconSvgMeshes(scene: THREE.Scene, signal?: AbortSignal): Promise<THREE.BufferGeometry[]> {
  const sources: THREE.Mesh[] = [];
  const owned: THREE.BufferGeometry[] = [];
  const instanceMatrix = new THREE.Matrix4();
  const vertex = new THREE.Vector3();
  let work = 0;
  scene.updateMatrixWorld(true);
  scene.traverseVisible((object) => {
    if (object instanceof THREE.Mesh && (object instanceof THREE.InstancedMesh || object instanceof THREE.SkinnedMesh || object.morphTargetInfluences?.some((weight) => weight !== 0))) {
      sources.push(object);
    }
  });
  const bake = async (source: THREE.Mesh) => {
    const count = source.geometry.getAttribute("position").count;
    const positions = new Float32Array(count * 3);
    for (let index = 0; index < count; index++) {
      if (++work % HOST_YIELD_BUDGET === 0) await yieldToHost(signal);
      source.getVertexPosition(index, vertex).toArray(positions, index * 3);
    }
    const geometry = source.geometry.clone();
    owned.push(geometry);
    geometry.setAttribute("position", new THREE.BufferAttribute(positions, 3));
    geometry.morphAttributes = {};
    geometry.boundingBox = null;
    geometry.boundingSphere = null;
    return geometry;
  };
  try {
    for (const source of sources) {
      signal?.throwIfAborted();
      const parent = source.parent;
      if (!parent) continue;
      const group = new THREE.Group().copy(source, false);
      if (source instanceof THREE.InstancedMesh) {
        for (let index = 0; index < source.count; index++) {
          if (++work % HOST_YIELD_BUDGET === 0) await yieldToHost(signal);
          const instance = new THREE.Mesh(source.geometry, source.material);
          instance.layers.mask = source.layers.mask;
          instance.renderOrder = source.renderOrder;
          source.getMatrixAt(index, instanceMatrix);
          instance.matrix.copy(instanceMatrix);
          instance.matrixAutoUpdate = false;
          if (source.morphTexture) {
            source.getMorphAt(index, instance);
            instance.geometry = await bake(instance);
            instance.morphTargetInfluences = undefined;
          }
          group.add(instance);
        }
      } else {
        const mesh = new THREE.Mesh(await bake(source), source.material);
        mesh.layers.mask = source.layers.mask;
        mesh.renderOrder = source.renderOrder;
        group.add(mesh);
      }
      for (const child of [...source.children]) group.add(child);
      parent.remove(source);
      parent.add(group);
    }
  } catch (fault) {
    for (const geometry of owned) geometry.dispose();
    throw fault;
  }
  return owned;
}

export async function paintIconDrawing(
  svg: SVGElement,
  scene: THREE.Scene,
  camera: THREE.Camera,
  request: IconRenderRequest,
  edgesFor: (geometry: THREE.BufferGeometry) => THREE.EdgesGeometry,
  fallbackStroke: string | null,
  signal?: AbortSignal,
): Promise<void> {
  signal?.throwIfAborted();
  const drawing = request.drawing ?? {};
  const original = Array.from(svg.childNodes);
  svg.replaceChildren();
  svg.removeAttribute("style");
  const authoredBackground = request.background?.trim();
  const rgba = authoredBackground?.match(/^#([a-f\d]{6})([a-f\d]{2})$/i);
  const background = authoredBackground && authoredBackground.toLowerCase() !== "transparent"
    ? new THREE.Color(rgba ? `#${rgba[1]}` : authoredBackground)
    : scene.background;
  if (drawing.background !== false && background instanceof THREE.Color) {
    const layer = svgLayer(svg, "background");
    const rect = document.createElementNS(SVG_NAMESPACE, "rect");
    for (const [name, value] of Object.entries({ x: -request.width / 2, y: -request.height / 2, width: request.width, height: request.height, fill: background.getStyle() })) {
      rect.setAttribute(name, String(value));
    }
    if (rgba?.[2]) rect.setAttribute("fill-opacity", String(Number.parseInt(rgba[2], 16) / 255));
    layer.append(rect);
  }
  if (drawing.fill !== false && original.length > 0) {
    const layer = svgLayer(svg, "fills");
    for (const node of original) layer.append(node);
    if (drawing.fillOpacity !== undefined && drawing.fillOpacity < 1) layer.setAttribute("opacity", String(drawing.fillOpacity));
    if (drawing.fillColor) {
      for (const path of layer.querySelectorAll("path")) path.style.fill = drawing.fillColor;
    }
  }
  const visibleColor = drawing.visibleColor ?? fallbackStroke;
  const visible = drawing.visibleLines !== false && visibleColor ? { color: visibleColor, width: drawing.visibleWidth ?? 1 } : undefined;
  const hidden = drawing.hiddenLines ? { color: drawing.hiddenColor ?? "#808080", width: drawing.hiddenWidth ?? 1 } : undefined;
  if (!visible && !hidden) return;
  const lines = await lineGeometry(scene, camera, request.width, request.height, edgesFor, signal);
  const easing = drawing.lineEasing ?? 0;
  if (hidden) await paintLines(svg, "hidden-lines", lines.hidden, hidden, drawing.mergeLines === true, easing, true, signal);
  if (visible) await paintLines(svg, "visible-lines", lines.visible, visible, drawing.mergeLines === true, easing, false, signal);
}

function svgLayer(svg: SVGElement, id: string): SVGGElement {
  const group = document.createElementNS(SVG_NAMESPACE, "g");
  group.id = id;
  svg.append(group);
  return group;
}

async function lineGeometry(
  scene: THREE.Scene,
  camera: THREE.Camera,
  width: number,
  height: number,
  edgesFor: (geometry: THREE.BufferGeometry) => THREE.EdgesGeometry,
  signal?: AbortSignal,
): Promise<{ visible: Line[]; hidden: Line[] }> {
  const occluders = new Occluders(width, height);
  const meshes: THREE.Mesh[] = [];
  const visible: Line[] = [], hidden: Line[] = [];
  scene.updateMatrixWorld(true);
  camera.updateMatrixWorld(true);
  scene.traverseVisible((object) => {
    if (object instanceof THREE.Mesh && camera.layers.test(object.layers)) {
      const materials = Array.isArray(object.material) ? object.material : [object.material];
      if (materials.some((material) => material.visible)) meshes.push(object);
    }
  });
  const viewProjection = new THREE.Matrix4().multiplyMatrices(camera.projectionMatrix, camera.matrixWorldInverse);
  const matrix = new THREE.Matrix4();
  const position = new THREE.Vector3();
  const clip = new THREE.Vector4();
  const project = (point: ClipPoint) => projectPoint(point, width, height);
  const clipVertex = (xyz: THREE.Vector3): ClipPoint => {
    clip.set(xyz.x, xyz.y, xyz.z, 1).applyMatrix4(matrix);
    return [clip.x, clip.y, clip.z, clip.w];
  };
  let work = 0;
  signal?.throwIfAborted();
  for (const mesh of meshes) {
    matrix.multiplyMatrices(viewProjection, mesh.matrixWorld);
    const geometry = mesh.geometry;
    const index = geometry.getIndex();
    const count = index?.count ?? geometry.getAttribute("position")?.count ?? 0;
    const ranges = Array.isArray(mesh.material) ? geometry.groups : [{ start: 0, count, materialIndex: 0 }];
    const start = geometry.drawRange.start;
    const end = Math.min(count, start + geometry.drawRange.count);
    for (const range of ranges) {
      const material = Array.isArray(mesh.material) ? mesh.material[range.materialIndex ?? 0] : mesh.material;
      if (!material?.visible || material.opacity < 1) continue;
      const stop = Math.min(end, range.start + range.count);
      for (let i = Math.max(start, range.start); i + 2 < stop; i += 3) {
        if (++work % HOST_YIELD_BUDGET === 0) await yieldToHost(signal);
        const vertex = (corner: number) => clipVertex(mesh.getVertexPosition(index?.getX(corner) ?? corner, position));
        const polygon = clipTriangle([vertex(i), vertex(i + 1), vertex(i + 2)]).map(project);
        for (let j = 1; j + 1 < polygon.length; j++) {
          const triangle = [polygon[0]!, polygon[j]!, polygon[j + 1]!] as const;
          const area = signedArea(...triangle);
          if ((material.side === THREE.FrontSide && area > 0) || (material.side === THREE.BackSide && area < 0)) continue;
          occluders.add(triangle);
        }
      }
    }
  }
  for (const mesh of meshes) {
    matrix.multiplyMatrices(viewProjection, mesh.matrixWorld);
    const edges = edgesFor(mesh.geometry).getAttribute("position");
    for (let i = 0; i + 1 < edges.count; i += 2) {
      if (++work % HOST_YIELD_BUDGET === 0) await yieldToHost(signal);
      const vertex = (index: number) => clipVertex(position.fromBufferAttribute(edges, index));
      const segment = clipEdge([vertex(i), vertex(i + 1)]);
      if (!segment) continue;
      const edge: Edge = [project(segment[0]), project(segment[1])];
      if (!edge.every((point) => point.every(Number.isFinite)) || (edge[0][0] === edge[1][0] && edge[0][1] === edge[1][1])) continue;
      const scan = occluders.classify(edge);
      let step = scan.next();
      while (!step.done) {
        await yieldToHost(signal);
        step = scan.next();
      }
      for (const span of step.value) {
        const [start, end] = span.range;
        (span.hidden ? hidden : visible).push([linePoint(edge, start), linePoint(edge, end)]);
      }
    }
  }
  return { visible, hidden };
}

function linePoint([a, b]: Edge, t: number): Point2 {
  return [a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])];
}

async function yieldToHost(signal?: AbortSignal): Promise<void> {
  await new Promise<void>((resolve) => setTimeout(resolve, 0));
  signal?.throwIfAborted();
}

async function paintLines(svg: SVGElement, id: string, lines: Line[], style: LineStyle, merge: boolean, easing: number, hidden: boolean, signal?: AbortSignal): Promise<void> {
  if (lines.length === 0) return;
  const commands = merge || easing > 0 ? await assembleLines(lines, merge, easing, signal) : await individualLines(lines, signal);
  if (!commands.length) return;
  const layer = svgLayer(svg, id);
  layer.setAttribute("fill", "none");
  layer.setAttribute("stroke", style.color);
  layer.setAttribute("stroke-width", String(style.width));
  layer.setAttribute("stroke-linecap", "round");
  layer.setAttribute("stroke-linejoin", "round");
  if (hidden) layer.setAttribute("stroke-dasharray", `${style.width * 4} ${style.width * 3}`);
  const path = document.createElementNS(SVG_NAMESPACE, "path");
  path.setAttribute("d", commands.join(""));
  layer.append(path);
}

interface LineNode {
  readonly point: Point2;
  neighbors: Set<LineNode>;
  degree: number;
  corners?: readonly [LineNode, LineNode];
}

interface Corner {
  readonly vertex: Point2;
  readonly entry: Point2;
  readonly exit: Point2;
  readonly midpoint: Point2;
  readonly control: Point2;
}

async function individualLines(lines: readonly Line[], signal?: AbortSignal): Promise<string[]> {
  const paths: string[] = [];
  for (const [first, second] of lines) {
    if (paths.length && paths.length % HOST_YIELD_BUDGET === 0) await yieldToHost(signal);
    paths.push(move(first) + line(second));
  }
  return paths;
}

async function assembleLines(lines: readonly Line[], merge: boolean, easing: number, signal?: AbortSignal): Promise<string[]> {
  const nodes = new Map<string, LineNode>();
  const node = (point: Point2) => {
    const round = (value: number) => Math.sign(value) * Math.round(Math.abs(value * 1024)) / 1024;
    const rounded: Point2 = [round(point[0]), round(point[1])];
    const key = rounded.join(",");
    const existing = nodes.get(key);
    if (existing) return existing;
    const next: LineNode = { point: rounded, neighbors: new Set(), degree: 0 };
    nodes.set(key, next);
    return next;
  };
  let work = 0;
  for (const [first, second] of lines) {
    if (++work % HOST_YIELD_BUDGET === 0) await yieldToHost(signal);
    const a = node(first), b = node(second);
    if (a !== b) {
      a.neighbors.add(b);
      b.neighbors.add(a);
    }
  }
  const compare = (a: LineNode, b: LineNode) => a.point[0] - b.point[0] || a.point[1] - b.point[1];
  const ordered = [...nodes.values()].filter((current) => current.neighbors.size).sort(compare);
  const ends: LineNode[] = [];
  for (const current of ordered) {
    if (++work % HOST_YIELD_BUDGET === 0) await yieldToHost(signal);
    const neighbors = [...current.neighbors].sort(compare);
    current.neighbors = new Set(neighbors);
    current.degree = neighbors.length;
    if (current.degree === 2) current.corners = [neighbors[0]!, neighbors[1]!];
    else ends.push(current);
  }
  const paths: string[] = [];
  let chainStart: LineNode | undefined;
  let cursor: LineNode | undefined;
  let endIndex = 0;
  let orderedIndex = 0;
  while (orderedIndex < ordered.length) {
    if (++work % HOST_YIELD_BUDGET === 0) await yieldToHost(signal);
    while (endIndex < ends.length && !ends[endIndex]!.neighbors.size) endIndex++;
    while (orderedIndex < ordered.length && !ordered[orderedIndex]!.neighbors.size) orderedIndex++;
    const from: LineNode | undefined = cursor ?? ends[endIndex] ?? ordered[orderedIndex];
    const to: LineNode | undefined = from?.neighbors.values().next().value;
    if (!from || !to) break;
    const opening = !cursor;
    const closing = chainStart === to;
    if (opening) chainStart = from;
    cursor = merge && to.degree === 2 && !closing ? to : undefined;
    if (!cursor) chainStart = undefined;
    const fromCorner = roundedCorner(from, to, easing);
    const toCorner = roundedCorner(to, from, easing);
    let path = "";
    if (opening) {
      path = !merge && fromCorner
        ? move(fromCorner.midpoint) + curve(fromCorner.control, fromCorner.entry)
        : move(fromCorner?.entry ?? from.point);
    }
    path += toCorner
      ? line(toCorner.entry) + (merge ? curve(toCorner.vertex, toCorner.exit) : curve(toCorner.control, toCorner.midpoint))
      : line(to.point);
    paths.push(merge && closing ? path + "Z" : path);
    from.neighbors.delete(to);
    to.neighbors.delete(from);
  }
  return paths;
}

function roundedCorner(node: LineNode, toward: LineNode, radius: number): Corner | undefined {
  if (radius <= 0 || !node.corners) return undefined;
  const vertex = node.point;
  const other = node.corners[0] === toward ? node.corners[1] : node.corners[0];
  const [ax, ay] = [toward.point[0] - vertex[0], toward.point[1] - vertex[1]];
  const [bx, by] = [other.point[0] - vertex[0], other.point[1] - vertex[1]];
  const a = Math.hypot(ax, ay), b = Math.hypot(bx, by);
  if (Math.abs(ax * by - ay * bx) <= 1e-5 * a * b) return undefined;
  const trim = Math.min(radius, Math.min(a, b) / 2);
  const entry: Point2 = [vertex[0] + ax * trim / a, vertex[1] + ay * trim / a];
  const exit: Point2 = [vertex[0] + bx * trim / b, vertex[1] + by * trim / b];
  return {
    vertex,
    entry,
    exit,
    midpoint: [(entry[0] + 2 * vertex[0] + exit[0]) / 4, (entry[1] + 2 * vertex[1] + exit[1]) / 4],
    control: [(entry[0] + vertex[0]) / 2, (entry[1] + vertex[1]) / 2],
  };
}

function coordinates(point: Point2): string {
  return point.map((value) => Number(value.toFixed(6))).join(",");
}
const move = (point: Point2) => `M${coordinates(point)}`;
const line = (point: Point2) => `L${coordinates(point)}`;
const curve = (control: Point2, endpoint: Point2) => `Q${coordinates(control)} ${coordinates(endpoint)}`;
