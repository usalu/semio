/** 🌗️ Colour never carries a name or a state alone: the ink of another learner's name label reaches 4.5 : 1 on every
 * palette slot in both appearances — judged by `colord`'s WCAG contrast as the third-party oracle — and every state the
 * stylesheet paints with a background, a shadow or a custom colour has a rule for forced colours, read from the
 * stylesheet as parsed by `lightningcss`.
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
import states from "../../🧫️fixtures/🌗️contrast-states/🔣️.json";

extend([a11y]);

type Appearance = "light" | "dark";

interface Fixture {
  readonly labels: { readonly minimumContrast: number; readonly slots: number; readonly appearances: readonly string[] };
  readonly forcedColors: readonly { readonly state: string; readonly selector: string; readonly properties: readonly string[] }[];
}

const fixture: Fixture = states;

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

  for (const entry of fixture.forcedColors) {
    it(`keeps ${entry.state} visible with forced colours`, () => {
      const rules = forcedColorRules(stylesheet);
      expect([...rules.keys()]).toContain(entry.selector);
      for (const property of entry.properties) expect(rules.get(entry.selector), `${entry.selector} { ${property} }`).toContain(property);
    });
  }
});
