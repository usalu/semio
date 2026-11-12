/** 🔍️ The independent references of the catalog silhouette guard: sharp (libvips with librsvg) rasterises the very polygons the owned scanline raster fills, and polygon-clipping computes their exact union and intersection areas. Test oracles only; the measure itself imports neither.
 *
 * @see ../🥒️.feature
 */
import polygonClipping from "polygon-clipping";
import sharp from "sharp";
import type { Box, Group, Measured, Point } from "../../🧰️support/🌓️silhouette/🟦️.ts";

const area = (ring: readonly Point[]): number => {
  let s = 0;
  for (let i = 0; i < ring.length; i++) {
    const [a, b] = [ring[i]!, ring[(i + 1) % ring.length]!];
    s += a.x * b.y - b.x * a.y;
  }
  return s / 2;
};

/** 🖼️ Rasterises groups with librsvg on a `width` x `height` grid whose cell `(i, j)` covers `[minX + i cell, minX + (i + 1) cell] x [minY + j cell, ...]`: a pixel is set where at least half of it is covered. Single triangles and quads go into one non-zero path (wound alike), groups with holes into one even-odd path each. */
export async function raster(groups: readonly Group[], box: Box, cell: number, width: number, height: number): Promise<Uint8Array> {
  const x = (p: Point): string => ((p.x - box.minX) / cell).toFixed(3);
  const y = (p: Point): string => ((p.y - box.minY) / cell).toFixed(3);
  const simple: string[] = [];
  const holes: string[] = [];
  for (const g of groups) {
    if (g.length === 1) {
      const ring = area(g[0]!) < 0 ? [...g[0]!].reverse() : g[0]!;
      simple.push(`M${ring.map((p) => `${x(p)} ${y(p)}`).join("L")}Z`);
    } else holes.push(`<path fill-rule="evenodd" d="${g.map((ring) => `M${ring.map((p) => `${x(p)} ${y(p)}`).join("L")}Z`).join("")}"/>`);
  }
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}"><rect width="${width}" height="${height}" fill="black"/><g fill="white" shape-rendering="geometricPrecision"><path d="${simple.join("")}"/>${holes.join("")}</g></svg>`;
  const { data } = await sharp(Buffer.from(svg), { limitInputPixels: false }).greyscale().raw().toBuffer({ resolveWithObject: true });
  const out = new Uint8Array(width * height);
  for (let i = 0; i < out.length; i++) out[i] = data[i]! >= 128 ? 1 : 0;
  return out;
}

/** 🧮️ The agreement of a measured view by rasterising with sharp: the same measure as the owned one (IoU of the drawing with the volume, for a section |(filled ∩ visible) ∪ (outlines ∩ full)| / |drawing ∪ cut|) on the same grid. */
export async function sharpIou(m: Measured): Promise<number> {
  const width = Math.max(1, Math.ceil((m.box.maxX - m.box.minX) / m.cell));
  const height = Math.max(1, Math.ceil((m.box.maxY - m.box.minY) / m.cell));
  const [filled, ink, cut] = await Promise.all([raster(m.filled, m.box, m.cell, width, height), raster(m.outlines, m.box, m.cell, width, height), raster(m.reference, m.box, m.cell, width, height)]);
  const [visible, full] = m.section ? await Promise.all([raster(m.visible, m.box, m.cell, width, height), raster(m.full, m.box, m.cell, width, height)]) : [cut, cut];
  let both = 0;
  let either = 0;
  for (let i = 0; i < cut.length; i++) {
    both += (filled[i]! & visible[i]!) | (ink[i]! & full[i]!);
    either += filled[i]! | ink[i]! | cut[i]!;
  }
  return either === 0 ? 1 : both / either;
}

type Multi = polygonClipping.MultiPolygon;
const SNAPS = [1e-5, 1e-4, 5e-4];
const snapped = (ring: readonly Point[], unit: number): [number, number][] => ring.map((p) => [Math.round(p.x / unit) * unit, Math.round(p.y / unit) * unit] as [number, number]).filter((p, i, all) => i === 0 || p[0] !== all[i - 1]![0] || p[1] !== all[i - 1]![1]);
const total = (multi: Multi): number => multi.reduce((sum, poly) => sum + poly.reduce((a, ring, i) => a + (i === 0 ? 1 : -1) * Math.abs(area(ring.map(([x, y]) => ({ x, y })))), 0), 0);

function union(groups: readonly Group[], unit: number): Multi {
  let level: polygonClipping.Geom[] = [];
  for (const g of groups) {
    const rings = g.map((ring) => snapped(ring, unit)).filter((ring) => ring.length >= 3 && Math.abs(area(ring.map(([x, y]) => ({ x, y })))) > 4 * unit * unit).map((ring) => [...ring, ring[0]!] as polygonClipping.Ring);
    if (rings.length === 0) continue;
    level.push(rings.length === 1 ? [rings[0]!] : polygonClipping.xor([rings[0]!], ...rings.slice(1).map((r) => [r] as polygonClipping.Polygon)));
  }
  while (level.length > 1) {
    const next: polygonClipping.Geom[] = [];
    for (let i = 0; i < level.length; i += 16) next.push(polygonClipping.union(level[i]!, ...level.slice(i + 1, i + 16)));
    level = next;
  }
  return (level[0] ?? []) as Multi;
}

/** 📐️ The exact agreement by polygon-clipping, coordinates snapped to 10 micrometres (and, where its sweep line fails on near-coincident edges, to 0.1 and 0.5 millimetres). */
export function exactIou(m: Measured): number {
  let failure: unknown;
  for (const unit of SNAPS) {
    try {
      const [filled, ink, cut] = [union(m.filled, unit), union(m.outlines, unit), union(m.reference, unit)];
      const [visible, full] = m.section ? [union(m.visible, unit), union(m.full, unit)] : [cut, cut];
      const drawn = polygonClipping.union(filled, ink);
      const inside = polygonClipping.union(polygonClipping.intersection(filled, visible), polygonClipping.intersection(ink, full));
      const either = total(drawn) + total(cut) - total(polygonClipping.intersection(drawn, cut));
      return either === 0 ? 1 : total(inside) / either;
    } catch (error) {
      failure = error;
    }
  }
  throw failure;
}
