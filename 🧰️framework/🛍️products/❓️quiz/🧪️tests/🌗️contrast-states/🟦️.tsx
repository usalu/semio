/** 🌗️ Colour never carries a name or a state alone: the ink of another learner's name label reaches 4.5 : 1 on every
 * palette slot in both appearances — judged by `colord`'s WCAG contrast as the third-party oracle —, so does the text of
 * the challenges' hints, miss marks and the clock's last seconds and end on the page's ground and tint, the design
 * system's tokens read from its own stylesheets, and every state the stylesheet paints with a background, a shadow or
 * a custom colour has a rule for forced colours, read from the stylesheet as parsed by `lightningcss`.
 *
 * @see ../../🧫️fixtures/🌗️contrast-states/🔣️.json
 * @see https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html
 * @see https://www.w3.org/TR/css-color-adjust-1/#forced — forced colours mode
 */

import { render } from "@testing-library/react";
import { colord, extend } from "colord";
import a11y from "colord/plugins/a11y";
import { transform } from "lightningcss";
import { afterEach, describe, expect, it } from "vitest";
import { presenceColor } from "@semio-tech/ui-react/chrome";
import { EMPTY_PRESENCE_VIEW, PresenceOverlay, paintStyle, peerInk, type PeerCursor } from "@semio-tech/quiz-react";
import stylesheet from "../../🎯️targets/⚛️react/🎨️.css?raw";
import palette from "../../../../🔨️modules/🖱️ui/🎨️styling/🎨️palette/🎨️.css?raw";
import chrome from "../../../../🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css?raw";
import states from "../../🧫️fixtures/🌗️contrast-states/🔣️.json";

extend([a11y]);

type Appearance = "light" | "dark";

interface Fixture {
  readonly labels: { readonly minimumContrast: number; readonly slots: number; readonly appearances: readonly string[] };
  readonly forcedColors: readonly { readonly state: string; readonly selector: string; readonly properties: readonly string[] }[];
  readonly texts: {
    readonly minimumContrast: number;
    readonly appearances: readonly string[];
    readonly ground: string;
    readonly states: readonly { readonly state: string; readonly selector: string; readonly ink: string; readonly tint?: { readonly colour: string; readonly share: number } }[];
  };
}

const fixture: Fixture = states;

/** 🧪️ The colour a token of the design system takes in `appearance` with no theme of its own: its declaration in the
 * appearance's block of the chrome stylesheet (`:root` for light, `.dark` for dark), followed through its `var()`
 * fallback to the palette's colour; a palette colour is read directly. */
function resolved(token: string, appearance: Appearance): string {
  const own = new RegExp(`(?:^|[\\s;{])${token}:\\s*(#[0-9a-fA-F]{6})\\s*;`, "u").exec(palette)?.[1];
  if (own !== undefined) return own;
  const block = (appearance === "light" ? /:root,\s*\[data-ui-theme\],\s*\.semio-scope\s*\{([^}]*)\}/u : /\.dark\s*\{([^}]*--base:[^}]*)\}/u).exec(chrome)?.[1] ?? "";
  const declared = new RegExp(`${token}:\\s*([^;]+);`, "u").exec(block)?.[1] ?? "";
  const fallback = [...declared.matchAll(/var\((--[\w-]+)/gu)].at(-1)?.[1];
  if (fallback === undefined || fallback === token) throw Error(`${token} (${appearance}) does not resolve: ${declared}`);
  return resolved(fallback, appearance);
}

/** 🫗️ `colour` laid at `share` over `ground`, as `color-mix(in srgb, colour share, transparent)` composites over it:
 * per sRGB channel `share × colour + (1 − share) × ground`. */
function composited(colour: string, share: number, ground: string): string {
  const [top, under] = [colord(colour).toRgb(), colord(ground).toRgb()];
  return colord({ r: share * top.r + (1 - share) * under.r, g: share * top.g + (1 - share) * under.g, b: share * top.b + (1 - share) * under.b }).toHex();
}

/** 📜️ Every rule at the top of `css` (not inside an at-rule) whose selector list holds `selector`, as its body. */
function topRules(css: string, selector: string): readonly string[] {
  const written = selector.replace(/=([\w-]+)\]/gu, '="$1"]');
  return [...css.matchAll(/^([^\s@{}][^{}]*)\{([^{}]*)\}/gmu)].filter((rule) => rule[1]!.split(",").some((part) => part.trim() === written)).map((rule) => rule[2]!);
}

function slotColour(slot: number, appearance: Appearance): string {
  const { h, s, l } = presenceColor(slot, appearance);
  return colord({ h, s: s * 100, l: l * 100 }).toHex();
}

type Component = { readonly type: string; readonly name?: string; readonly kind?: string; readonly value?: string; readonly operation?: { readonly value: string } | null };
type Declaration = { readonly property: string; readonly value: { readonly name?: string; readonly propertyId?: { readonly property: string } } };
type Rule = { readonly type: string; readonly value: { readonly selectors?: readonly (readonly Component[])[]; readonly declarations?: { readonly declarations: readonly Declaration[] }; readonly rules?: readonly Rule[]; readonly query?: unknown } };

function selectorText(selector: readonly Component[]): string {
  return selector
    .map((part) => {
      if (part.type === "class") return `.${part.name}`;
      if (part.type === "combinator") return part.value === "descendant" ? " " : part.value === "child" ? " > " : ` ${part.value} `;
      if (part.type === "attribute") return part.operation == null ? `[${part.name}]` : `[${part.name}=${part.operation.value}]`;
      if (part.type === "pseudo-class") return `:${part.kind}`;
      return part.name ?? part.type;
    })
    .join("");
}

function propertyName(declaration: Declaration): string {
  if (declaration.property === "custom") return declaration.value.name ?? "custom";
  if (declaration.property === "unparsed") return declaration.value.propertyId?.property ?? "unparsed";
  return declaration.property;
}

/** 🎨️ Every rule inside `@media (forced-colors: active)`: its selector and the properties it declares. */
function forcedColorRules(css: string): ReadonlyMap<string, readonly string[]> {
  const found = new Map<string, string[]>();
  transform({
    filename: "🎨️.css",
    code: new TextEncoder().encode(css),
    visitor: {
      StyleSheet(sheet) {
        for (const rule of sheet.rules as unknown as readonly Rule[]) {
          if (rule.type !== "media" || !JSON.stringify(rule.value.query).includes('"forced-colors"') || !JSON.stringify(rule.value.query).includes('"active"')) continue;
          for (const inner of rule.value.rules ?? []) {
            if (inner.type !== "style") continue;
            const properties = (inner.value.declarations?.declarations ?? []).map(propertyName);
            for (const selector of inner.value.selectors ?? []) found.set(selectorText(selector), [...(found.get(selectorText(selector)) ?? []), ...properties]);
          }
        }
      },
    },
  });
  return found;
}

afterEach(() => {
  document.documentElement.classList.remove("dark");
});

describe("🌗️ contrast and states", () => {
  for (const appearance of fixture.labels.appearances as readonly Appearance[]) {
    it(`gives every learner's name label an ink of at least ${fixture.labels.minimumContrast} : 1 on its palette colour (${appearance})`, () => {
      const failing: string[] = [];
      for (let slot = 0; slot < fixture.labels.slots; slot += 1) {
        const ink = peerInk(slot, appearance);
        const ratio = colord(slotColour(slot, appearance)).contrast(ink);
        if (ratio < fixture.labels.minimumContrast) failing.push(`slot ${slot}: ${ink} on ${slotColour(slot, appearance)} = ${ratio}`);
        expect((paintStyle(slot) as Readonly<Record<string, string>>)[`--quiz-peer-ink-${appearance}`], `slot ${slot}`).toBe(ink);
      }
      expect(failing).toEqual([]);
    });
  }

  it("paints a name label in the ink of its slot, in the appearance the document shows", () => {
    const peers: readonly PeerCursor[] = [2, 8, 14].map((colour) => ({ session: `s-${colour}`, tag: `0000000${colour % 10}`, label: `Learner ${colour}`, colour, cursor: { anchor: "home:learner", x: 0.5, y: 0.5 }, focus: undefined, drag: undefined }));
    render(<PresenceOverlay view={{ ...EMPTY_PRESENCE_VIEW, peers }} show />);
    for (const peer of peers) {
      const holder = document.querySelector<HTMLElement>(`[data-tag="${peer.tag}"]`)!.parentElement!;
      for (const appearance of fixture.labels.appearances as readonly Appearance[]) expect(holder.style.getPropertyValue(`--quiz-peer-ink-${appearance}`), `slot ${peer.colour}`).toBe(peerInk(peer.colour, appearance));
    }
    expect(stylesheet).toMatch(/\.quiz-peer-label \{[^}]*color: var\(--quiz-peer-ink-light\);/u);
    expect(stylesheet).toMatch(/\.dark \.quiz-peer-label \{\s*color: var\(--quiz-peer-ink-dark\);/u);
  });

  for (const appearance of fixture.texts.appearances as readonly Appearance[]) {
    it(`gives the text of every state the challenges add an ink of at least ${fixture.texts.minimumContrast} : 1 on its ground (${appearance})`, () => {
      const failing: string[] = [];
      for (const entry of fixture.texts.states) {
        const ground = resolved(fixture.texts.ground, appearance);
        const behind = entry.tint === undefined ? ground : composited(resolved(entry.tint.colour, appearance), entry.tint.share, ground);
        const ink = resolved(entry.ink, appearance);
        const ratio = colord(ink).contrast(behind);
        if (ratio < fixture.texts.minimumContrast) failing.push(`${entry.state}: ${ink} on ${behind} = ${ratio}`);
      }
      expect(failing).toEqual([]);
    });
  }

  for (const entry of fixture.texts.states) {
    it(`paints ${entry.state} in the inherited ink${entry.tint === undefined ? "" : " over its tint"}, never in a colour of its own`, () => {
      const rules = topRules(stylesheet, entry.selector);
      expect(rules.length, entry.selector).toBeGreaterThan(0);
      for (const body of rules) expect(body, entry.selector).not.toMatch(/(?:^|[\s;])color\s*:/u);
      if (entry.tint !== undefined) expect(rules.some((body) => body.includes(`background: color-mix(in srgb, var(${entry.tint!.colour}) ${entry.tint!.share * 100}%, transparent)`)), entry.selector).toBe(true);
    });
  }

  it("reads the design system's tokens as the browser does: the light ground and ink of the palette, and the dark ones", () => {
    expect([resolved("--base", "light"), resolved("--foreground", "light"), resolved("--base", "dark"), resolved("--foreground", "dark")]).toEqual(["#f7f3e3", "#001117", "#001117", "#f7f3e3"]);
    expect(composited("#ffffff", 0.5, "#000000")).toBe(colord({ r: 127.5, g: 127.5, b: 127.5 }).toHex());
  });

  for (const entry of fixture.forcedColors) {
    it(`keeps ${entry.state} visible with forced colours`, () => {
      const rules = forcedColorRules(stylesheet);
      expect([...rules.keys()]).toContain(entry.selector);
      for (const property of entry.properties) expect(rules.get(entry.selector), `${entry.selector} { ${property} }`).toContain(property);
    });
  }
});
