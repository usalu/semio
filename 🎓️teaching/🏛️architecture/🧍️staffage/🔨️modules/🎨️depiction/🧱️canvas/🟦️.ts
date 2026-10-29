/** 🧱️ The drawing canvas: collects items per layer while tracking the paper-space extent, then translates everything into its margins.
 *
 * Items are built in a local paper frame (millimetres, y down, model origin at (0, 0)); {@link Canvas.finish} shifts the frame so the content
 * box starts at the margin and reports where the model origin ended up.
 *
 * @see ../../🔗️contracts/🟦️.ts — {@link DrawingScene}
 */
import type { Slug } from "../../../🧬️schema/🟦️.ts";
import type { Language } from "../../../🧬️schema/🟦️.ts";
import { LAYERS, type DepictionOptions, type DrawingItem, type DrawingLayer, type EvaluatedState, type Fill, type ItemValue, type LayerId, type PathCommand, type Point, type Stroke } from "../../🔗️contracts/🟦️.ts";
import { tx, type Theme, type ThemeColours } from "../🖍️palette/🟦️.ts";
import type { Box, Vec } from "../📐️geometry/🟦️.ts";

const LAYER_TITLES = {
  background: tx("Background", "Hintergrund"),
  ground: tx("Ground", "Gelände"),
  fill: tx("Fills", "Flächen"),
  field: tx("Analysis field", "Analysefeld"),
  body: tx("Body outlines", "Körperkonturen"),
  detail: tx("Detail lines", "Detaillinien"),
  texture: tx("Texture lines", "Texturlinien"),
  glazing: tx("Glazing", "Verglasung"),
  diagrams: tx("Diagrams", "Diagramme"),
  forces: tx("Forces and supports", "Kräfte und Lager"),
  annotations: tx("Annotations", "Beschriftung"),
  legend: tx("Legend", "Legende"),
} as const satisfies Record<LayerId, ReturnType<typeof tx>>;

/** 🔤️ Average glyph advance relative to the text height, used to reserve room for text. */
export const GLYPH_ADVANCE = 0.56;

/** 🧭️ What every view builder needs: state, options, scale factor and the model-to-paper mapping `(x, y) → (x·k, −y·k)`. */
export type Ctx = { readonly state: EvaluatedState; readonly options: DepictionOptions; readonly language: Language; readonly k: number; readonly theme: Theme; readonly colours: ThemeColours; readonly paper: (p: Point) => Vec; readonly labels: Labels };

/** 🧩️ The optional parts of a path item. */
export type PathOptions = { readonly stroke?: Stroke; readonly fill?: Fill; readonly value?: ItemValue; readonly bone?: Slug; readonly region?: Slug };

/** 🔤️ The optional parts of a text item. */
export type TextOptions = { readonly anchor?: "start" | "middle" | "end"; readonly weight?: "normal" | "bold" };

/** 🧱️ Layer store with extent tracking. */
export class Canvas {
  private readonly layers = new Map<LayerId, DrawingItem[]>();
  private readonly used = new Map<string, number>();
  private extent: { minX: number; minY: number; maxX: number; maxY: number } | null = null;

  /** 🆔️ A scene-unique id derived from a base. */
  unique(base: string): string {
    const n = this.used.get(base) ?? 0;
    this.used.set(base, n + 1);
    return n === 0 ? base : `${base}~${n}`;
  }

  /** 📍️ Grows the extent to cover a point. */
  reach(p: Vec): void {
    const e = this.extent;
    this.extent = e === null ? { minX: p.x, minY: p.y, maxX: p.x, maxY: p.y } : { minX: Math.min(e.minX, p.x), minY: Math.min(e.minY, p.y), maxX: Math.max(e.maxX, p.x), maxY: Math.max(e.maxY, p.y) };
  }

  /** 📦️ The current extent, `null` when empty. */
  get box(): Box | null {
    return this.extent;
  }

  /** ✒️ Adds a path item and grows the extent over its points. */
  path(layer: LayerId, id: string, commands: readonly PathCommand[], options: PathOptions = {}): void {
    for (const c of commands) if (c.op !== "Z") this.reach({ x: c.x, y: c.y });
    this.push(layer, { kind: "path", id: this.unique(id), commands, ...options });
  }

  /** 🔤️ Adds a text item and grows the extent over its estimated box. */
  text(layer: LayerId, id: string, at: Vec, text: string, sizeMm: number, color: string, options: TextOptions = {}): void {
    const anchor = options.anchor ?? "start";
    const w = text.length * sizeMm * GLYPH_ADVANCE;
    const x0 = anchor === "start" ? at.x : anchor === "end" ? at.x - w : at.x - w / 2;
    this.reach({ x: x0, y: at.y - sizeMm * 0.8 });
    this.reach({ x: x0 + w, y: at.y + sizeMm * 0.25 });
    this.push(layer, { kind: "text", id: this.unique(id), x: at.x, y: at.y, text, sizeMm, color, anchor, ...(options.weight ? { weight: options.weight } : {}) });
  }

  private push(layer: LayerId, item: DrawingItem): void {
    const list = this.layers.get(layer);
    if (list) list.push(item);
    else this.layers.set(layer, [item]);
  }

  /** 🏁️ The non-empty layers in contract order, translated so the extent starts at `margin`; returns the size and the shifted model origin. */
  finish(margin: number): { readonly layers: DrawingLayer[]; readonly widthMm: number; readonly heightMm: number; readonly origin: Vec } {
    const e = this.extent ?? { minX: 0, minY: 0, maxX: 0, maxY: 0 };
    const dx = margin - e.minX;
    const dy = margin - e.minY;
    const move = (c: PathCommand): PathCommand => (c.op === "Z" ? c : c.op === "C" ? { op: "C", x1: c.x1 + dx, y1: c.y1 + dy, x2: c.x2 + dx, y2: c.y2 + dy, x: c.x + dx, y: c.y + dy } : { op: c.op, x: c.x + dx, y: c.y + dy });
    const layers = LAYERS.flatMap((id): DrawingLayer[] => {
      const items = this.layers.get(id);
      if (!items?.length) return [];
      return [{ id, title: LAYER_TITLES[id], items: items.map((item) => (item.kind === "text" ? { ...item, x: item.x + dx, y: item.y + dy } : { ...item, commands: item.commands.map(move) })) }];
    });
    return { layers, widthMm: e.maxX - e.minX + 2 * margin, heightMm: e.maxY - e.minY + 2 * margin, origin: { x: dx, y: dy } };
  }
}

/** 🔢️ The most value labels an analytic view prints at a scale: 8 at 1:50, 6 at 1:100, 4 at 1:200, 3 at 1:500 and smaller scales; every other value stays in the item `value` metadata. */
export function labelBudget(denominator: number): number {
  return denominator <= 50 ? 8 : denominator <= 100 ? 6 : denominator <= 200 ? 4 : 3;
}

/** 🏷️ How important a label is: required labels (reactions, the largest moment) are always printed, the rest compete by magnitude. */
export type LabelRank = { readonly required: boolean; readonly magnitude: number };

/** 🏷️ Deferred value labels placed into a column with leader lines, so labels never sit on the drawing or on each other, within a budget. */
export class Labels {
  private readonly pending: { readonly id: string; readonly at: Vec; readonly text: string; readonly rank: LabelRank }[] = [];

  /** ➕️ Requests a label for a point. */
  add(id: string, at: Vec, text: string, rank: LabelRank = { required: false, magnitude: 0 }): void {
    this.pending.push({ id, at, text, rank });
  }

  /** 🧮️ The number of pending labels. */
  get size(): number {
    return this.pending.length;
  }

  /** 🏁️ Keeps the required labels plus the largest others up to `budget`, places them top-down in one column at `x` (at least one line apart) with leaders; returns how many requests were left out. */
  flush(canvas: Canvas, layer: LayerId, x: number, sizeMm: number, color: string, leaderMm: number, budget: number): number {
    const required = this.pending.filter((l) => l.rank.required);
    const rest = this.pending.filter((l) => !l.rank.required).sort((a, b) => b.rank.magnitude - a.rank.magnitude);
    const kept = new Set([...required, ...rest.slice(0, Math.max(0, budget - required.length))]);
    const sorted = this.pending.filter((l) => kept.has(l)).sort((a, b) => a.at.y - b.at.y || a.at.x - b.at.x);
    const pitch = sizeMm * 1.4;
    let floor = Number.NEGATIVE_INFINITY;
    for (const label of sorted) {
      const y = Math.max(label.at.y, floor);
      floor = y + pitch;
      canvas.path(layer, `${label.id}:leader`, [{ op: "M", x: label.at.x, y: label.at.y }, { op: "L", x: x - 1, y: y - sizeMm * 0.3 }], { stroke: { color, widthMm: leaderMm, opacity: 0.6, cap: "round" } });
      canvas.text(layer, label.id, { x, y }, label.text, sizeMm, color);
    }
    const omitted = this.pending.length - kept.size;
    this.pending.length = 0;
    return omitted;
  }
}
