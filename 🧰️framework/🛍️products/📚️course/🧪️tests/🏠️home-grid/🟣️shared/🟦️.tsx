/** 🟣️ The shared declarations of the 🏠️ home overview suite tests. */
import { act } from "@testing-library/react";
import { transform } from "lightningcss";
import { vi } from "vitest";
import { type PeerCursor } from "@semio-tech/course-react";
import { PAGES } from "../../🛠️home-grid-kit/🟦️.tsx";

export const SNOW_TEXT = String.fromCodePoint(0x2744, 0xfe0e);

export const DESKTOP_CELLS_OF_THE_REFERENCE = Object.fromEntries(PAGES.map((page, index) => [page, { column: index % 3, row: Math.floor(index / 3) }]));

export const TABLET_CELLS_OF_THE_REFERENCE = Object.fromEntries(PAGES.map((page, index) => [page, { column: index % 2, row: Math.floor(index / 2) }]));

//#region 🎨️Tracks
/** 🎨️ One declaration of the card layer or its cells in the stylesheet, with the minimum viewport width its media query
 * asks for (0 outside one): a track list as its track count, a variable as `var(--name)`, anything else as its name. */
export interface LayerRule {
  readonly minWidth: number;
  readonly selector: string;
  readonly property: string;
  readonly value: number | string;
}

export type Declaration = {
  readonly property: string;
  readonly value: { readonly items?: readonly unknown[]; readonly propertyId?: { readonly property: string }; readonly value?: readonly { readonly type: string; readonly value?: { readonly name?: { readonly ident: string } } }[] };
};

export const SPACING = ["gap", "row-gap", "column-gap", "padding", "padding-top", "padding-right", "padding-bottom", "padding-left", "padding-inline", "padding-block", "margin"];

export function selectorText(selector: readonly { readonly type: string; readonly name?: string; readonly value?: string; readonly operation?: { readonly value: string } }[]): string {
  return selector.map((part) => (part.type === "class" ? `.${part.name}` : part.type === "combinator" ? ">" : part.type === "attribute" ? `[${part.name}=${part.operation?.value}]` : part.type)).join("");
}

export function declared(declaration: Declaration): LayerRule["value"] {
  const variable = Array.isArray(declaration.value.value) ? declaration.value.value.find((token) => token.type === "var")?.value?.name?.ident : undefined;
  if (declaration.property === "unparsed" && variable !== undefined) return `var(${variable})`;
  return declaration.value.items?.length ?? declaration.property;
}

/** 🎨️ Every declaration of `.course-home-grid` and `.course-home-cell` with the minimum viewport width it applies from. */
export function layerRules(css: string): readonly LayerRule[] {
  const found: LayerRule[] = [];
  const collect = (rules: readonly { readonly type: string; readonly value: never }[], minWidth: number): void => {
    for (const rule of rules as readonly {
      readonly type: string;
      readonly value: { readonly selectors?: readonly never[][]; readonly declarations?: { readonly declarations: readonly Declaration[] }; readonly query?: never; readonly rules?: never[] };
    }[]) {
      if (rule.type === "media") {
        const query = rule.value.query as unknown as {
          readonly mediaQueries: readonly { readonly condition: { readonly value: { readonly name: string; readonly operator: string; readonly value: { readonly value: { readonly value: { readonly unit: string; readonly value: number } } } } } }[];
        };
        const feature = query.mediaQueries[0]?.condition.value;
        if (feature?.name === "width") {
          if (feature.operator !== "greater-than-equal" || feature.value.value.value.unit !== "px") throw new Error(`unexpected width query ${JSON.stringify(query)}`);
          collect(rule.value.rules ?? [], feature.value.value.value.value);
        } else {
          const before = found.length;
          collect(rule.value.rules ?? [], Number.POSITIVE_INFINITY);
          if (found.length !== before) throw new Error(`card layer rules under a media query that is not about width: ${JSON.stringify(query)}`);
        }
        continue;
      }
      if (rule.type !== "style") continue;
      for (const selector of rule.value.selectors ?? []) {
        const name = selectorText(selector);
        if (!name.startsWith(".course-home-grid") && !name.startsWith(".course-home-cell")) continue;
        for (const declaration of rule.value.declarations?.declarations ?? []) found.push({ minWidth, selector: name, property: declaration.value.propertyId?.property ?? declaration.property, value: declared(declaration) });
      }
    }
  };
  transform({
    filename: "🎨️.css",
    code: new TextEncoder().encode(css),
    visitor: {
      StyleSheet(sheet) {
        collect(sheet.rules as never, 0);
      },
    },
  });
  return found;
}

/** 🔳️ The tracks the card layer takes at `width`: counted, or the overview's variables. */
export function layerAt(rules: readonly LayerRule[], width: number): Readonly<Record<string, number | string>> {
  const tracks: Record<string, number | string> = {};
  for (const rule of rules) {
    if (width < rule.minWidth || rule.selector !== ".course-home-grid") continue;
    if (rule.property === "grid-template-columns") tracks.columns = rule.value;
    if (rule.property === "grid-template-rows") tracks.rows = rule.value;
  }
  return tracks;
}

/** ⚖️ The fr weights of a CSS track list, as parsed by lightningcss (which keeps them as 32-bit floats, hence the
 * rounding to 10⁻⁶). */
export function weights(template: string): readonly number[] {
  const found: number[] = [];
  transform({
    filename: "tracks.css",
    code: new TextEncoder().encode(`.tracks { grid-template-columns: ${template}; }`),
    visitor: {
      Declaration: {
        "grid-template-columns"(declaration) {
          const value = declaration.value as { readonly items?: readonly { readonly type: string; readonly value: { readonly type: string; readonly value?: number; readonly max?: { readonly type: string; readonly value: number } } }[] };
          for (const item of value.items ?? []) {
            const size = item.value.type === "min-max" ? item.value.max : item.value;
            if (item.type !== "track-size" || size?.type !== "flex") throw new Error(`not a flexible track: ${JSON.stringify(item)}`);
            found.push(Math.round((size.value as number) * 1e6) / 1e6);
          }
        },
      },
    },
  });
  return found;
}
//#endregion 🎨️Tracks

/** 🖱️ Gives the overview a size and moves the mouse to the fractions `fx`, `fy` of it, then lets the pan settle. */
export function moveMouseOver(fx: number, fy: number): void {
  const root = document.querySelector<HTMLElement>('[data-grid-active] > [data-layered-overview]')!;
  vi.spyOn(root, "getBoundingClientRect").mockReturnValue({ left: 0, top: 0, width: 1400, height: 850, right: 1400, bottom: 850, x: 0, y: 0, toJSON: () => ({}) });
  act(() => void window.dispatchEvent(new PointerEvent("pointermove", { clientX: fx * 1400, clientY: fy * 850, pointerType: "mouse" })));
  for (let spent = 0; spent < 3000; spent += 10) act(() => void vi.advanceTimersByTime(10));
}

export function peer(anchor: string): PeerCursor {
  return { session: `w-${anchor}`, tag: "0badc0de", label: "Mira K.", colour: 3, place: { screen: "home" }, home: undefined, cursor: { anchor, x: 0.5, y: 0.5 }, focus: undefined, drag: undefined };
}

export function cards(): readonly HTMLElement[] {
  return [...document.querySelectorAll<HTMLElement>('[data-grid-active] > [data-layered-overview] > [data-layered-overlay].course-home-grid section[data-card]')];
}

/** 🧭 The strip directly under one overview. */
export function gridStrip(id: string): HTMLElement {
  return document.querySelector<HTMLElement>(`[data-course-grid="${id}"] > [data-layered-overview] > [data-layered-strip]`)!;
}

/** 🪟 The glass directly under one overview. */
export function gridVeil(id: string): HTMLElement {
  return document.querySelector<HTMLElement>(`[data-course-grid="${id}"] > [data-layered-overview] > [data-layered-veil]`)!;
}
