/** 🕷️ Spider diagram geometry, label layout and text alternative: the shared vectors, the radial scale against
 * `d3-scale`'s `scaleLinear` (clamped) and the label widths against the brand font's real advance widths read with
 * `opentype.js` as the third-party oracles, and the accessible rendering at the width the chart gets.
 *
 * @see ../../🧫️fixtures/🕷️radar-geometry/🔣️.json
 * @see ../../../../🔨️modules/🖼️assets/🔤️fonts/🚀️anta — the semio sans the site renders labels in
 */

import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { scaleLinear } from "d3-scale";
import opentype from "opentype.js";
import { afterEach, describe, expect, it, vi } from "vitest";
import {
  RADAR_LABEL_EM,
  RADAR_METRICS,
  RadarChart,
  estimateTextWidth,
  formatNumber,
  quizText,
  radarAngle,
  radarFraction,
  radarLayout,
  radarPoint,
  radarPolygon,
  withUnit,
  wrapLabel,
  type RadarFrame,
  type RadarLayout,
  type TextMeasure,
} from "@semio-tech/quiz-react";
import antaLatin from "../../../../🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf?inline";
import antaExtended from "../../../../🔨️modules/🖼️assets/🔤️fonts/🚀️anta/➕️latin-ext/📖️regular/🔤️outline.ttf?inline";
import antaMath from "../../../../🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🧮️math/📖️regular/🔤️outline.ttf?inline";
import antaSymbols from "../../../../🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🔣️symbols/📖️regular/🔤️outline.ttf?inline";
import geometry from "../../🧫️fixtures/🕷️radar-geometry/🔣️.json";

interface Fixture {
  readonly frame: RadarFrame;
  readonly axes: readonly { readonly id: string; readonly min: number; readonly max: number }[];
  readonly profiles: readonly { readonly id: string; readonly values: Readonly<Record<string, number>>; readonly fractions: readonly number[]; readonly points: readonly (readonly number[])[] }[];
  readonly wraps: readonly { readonly id: string; readonly label: string; readonly width: number; readonly lines: readonly string[] }[];
  readonly layouts: readonly { readonly id: string; readonly width: number; readonly fontSize: number; readonly labels: readonly string[] }[];
}

const fixture: Fixture = geometry;
const TOLERANCE = 1e-9;
const EPSILON = 1e-6;
const ANTA = [antaLatin, antaExtended, antaMath, antaSymbols].map((url) => {
  const bytes = Buffer.from(url.slice(url.indexOf(",") + 1), "base64");
  return opentype.parse(bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength));
});

function monospace(text: string): number {
  return [...text].length;
}

/** 🔤️ Advance widths of the semio sans (Anta) with its GPOS kerning, falling back per character through its subsets
 * the way the site's `font-family` list does. */
function antaMeasure(fontSize: number): TextMeasure {
  return (text) => {
    let width = 0;
    let previous: { readonly font: opentype.Font; readonly glyph: opentype.Glyph } | undefined;
    for (const char of text) {
      const font = ANTA.find((candidate) => candidate.charToGlyphIndex(char) !== 0) ?? ANTA[0]!;
      const glyph = font.charToGlyph(char);
      if (previous?.font === font) width += (font.getKerningValue(previous.glyph, glyph) * fontSize) / font.unitsPerEm;
      width += ((glyph.advanceWidth ?? 0) * fontSize) / font.unitsPerEm;
      previous = { font, glyph };
    }
    return width;
  };
}

function estimate(fontSize: number): TextMeasure {
  return (text) => estimateTextWidth(text, fontSize);
}

function withoutHyphens(text: string): string {
  return text.replace(/\s+/gu, " ").trim().replaceAll("-", "");
}

function rejoined(lines: readonly string[]): string {
  return lines.reduce((text, line) => (text === "" ? line : text.endsWith("-") ? `${text}${line}` : `${text} ${line}`), "");
}

/** 🧐️ Every way `layout` breaks the laws of the fixture's `layoutsDescription`, as readable strings. */
function violations(layout: RadarLayout, labels: readonly string[], fontSize: number, measure: TextMeasure): readonly string[] {
  const found: string[] = [];
  const { frame, width, height, lineHeight } = layout;
  const gap = fontSize * RADAR_METRICS.gap;
  const room = width - 2 * Math.max(2, fontSize * RADAR_METRICS.pad);
  if (layout.fontSize !== fontSize) found.push(`font size ${layout.fontSize}`);
  if (frame.cx - frame.radius < -EPSILON || frame.cx + frame.radius > width + EPSILON || frame.cy - frame.radius < -EPSILON || frame.cy + frame.radius > height + EPSILON) found.push("circle outside");
  if (frame.radius > RADAR_METRICS.maxRadius * fontSize + EPSILON) found.push(`radius ${frame.radius} too large`);
  if (room / 2 - gap - RADAR_METRICS.minSide * fontSize >= RADAR_METRICS.minRadius * fontSize && frame.radius < RADAR_METRICS.wordRadius * fontSize - EPSILON) found.push(`radius ${frame.radius} too small`);
  const keyed = layout.legend.length > 0;
  if (keyed && layout.legend.length !== labels.length) found.push("legend incomplete");
  const texts = keyed ? layout.legend.map((entry) => entry.text) : layout.labels;
  texts.forEach((label, index) => {
    if (withoutHyphens(rejoined(label.lines)) !== withoutHyphens(labels[index] ?? "")) found.push(`label ${index} text ${JSON.stringify(label.lines)}`);
  });
  if (keyed && layout.labels.some((label, index) => label.lines.join("") !== String(index + 1))) found.push("spokes without their numbers");
  const named = [
    ...layout.labels.map((label, index) => ({ name: `spoke ${index}`, label, spoke: true })),
    ...layout.legend.flatMap((entry, index) => [
      { name: `key ${index}`, label: entry.key, spoke: false },
      { name: `legend ${index}`, label: entry.text, spoke: false },
    ]),
  ].filter((item) => item.label.lines.length > 0);
  named.forEach(({ name, label, spoke }, index) => {
    const { box, lines } = label;
    if (box.x < -EPSILON || box.y < -EPSILON || box.x + box.width > width + EPSILON || box.y + box.height > height + EPSILON) found.push(`${name} outside ${JSON.stringify(box)} of ${width}×${height}`);
    for (const line of lines) if (measure(line) > box.width + EPSILON) found.push(`${name} line ${JSON.stringify(line)} wider than its box`);
    if (Math.abs(box.height - lines.length * lineHeight) > EPSILON) found.push(`${name} height`);
    const dx = Math.max(box.x - frame.cx, frame.cx - box.x - box.width, 0);
    const dy = Math.max(box.y - frame.cy, frame.cy - box.y - box.height, 0);
    if (Math.hypot(dx, dy) < frame.radius + (spoke ? gap / 2 : 0) - EPSILON) found.push(`${name} touches the circle`);
    const expected = label.anchor === "start" ? box.x : label.anchor === "end" ? box.x + box.width : box.x + box.width / 2;
    if (Math.abs(label.x - expected) > EPSILON) found.push(`${name} anchor`);
    named.slice(index + 1).forEach((other) => {
      const b = other.label.box;
      if (box.x >= b.x + b.width - EPSILON || b.x >= box.x + box.width - EPSILON) return;
      if (box.y + box.height <= b.y + EPSILON || b.y + b.height <= box.y + EPSILON) return;
      found.push(`${name} and ${other.name} overlap`);
    });
  });
  return found;
}

const SWEEP_WIDTHS = Array.from({ length: 101 }, (_, index) => 200 + index * 12);
const SWEEP_LABELS: readonly (readonly string[])[] = [...new Map(fixture.layouts.map((layout) => [JSON.stringify(layout.labels), layout.labels])).values()];

afterEach(() => {
  vi.restoreAllMocks();
});

describe("🕷️ radar geometry", () => {
  for (const profile of fixture.profiles) {
    it(`places the shared profile ${profile.id}`, () => {
      fixture.axes.forEach((axis, index) => expect(radarFraction(profile.values[axis.id] ?? axis.min, axis.min, axis.max)).toBeCloseTo(profile.fractions[index] ?? Number.NaN, 12));
      const points = radarPolygon(fixture.frame, fixture.axes, profile.values);
      points.forEach((point, index) => {
        expect(Math.abs(point.x - (profile.points[index]?.[0] ?? Number.NaN))).toBeLessThan(TOLERANCE);
        expect(Math.abs(point.y - (profile.points[index]?.[1] ?? Number.NaN))).toBeLessThan(TOLERANCE);
      });
    });
  }

  it("scales every spoke exactly as a clamped d3 linear scale from centre to rim", () => {
    const frame: RadarFrame = { cx: 50, cy: 70, radius: 40 };
    let compared = 0;
    for (let count = 3; count <= 9; count += 1) {
      for (let index = 0; index < count; index += 1) {
        const min = -20 + index * 7.5;
        const max = min + 10 + index * 33.25;
        const scale = scaleLinear().domain([min, max]).range([0, frame.radius]).clamp(true);
        for (const value of [min - 5, min, min + (max - min) / 3, (min + max) / 2, max, max + 17]) {
          const distance = scale(value);
          const angle = (2 * Math.PI * index) / count;
          const point = radarPoint(frame, index, count, radarFraction(value, min, max));
          expect(Math.abs(point.x - (frame.cx + distance * Math.sin(angle)))).toBeLessThan(TOLERANCE);
          expect(Math.abs(point.y - (frame.cy - distance * Math.cos(angle)))).toBeLessThan(TOLERANCE);
          compared += 1;
        }
      }
    }
    expect(compared).toBe(6 * (3 + 4 + 5 + 6 + 7 + 8 + 9));
  });

  it("starts at twelve o'clock and turns clockwise", () => {
    expect(radarAngle(0, 5)).toBe(0);
    const frame: RadarFrame = { cx: 0, cy: 0, radius: 1 };
    const right = radarPoint(frame, 1, 4, 1);
    expect(right.x).toBeCloseTo(1, 12);
    expect(right.y).toBeCloseTo(0, 12);
    const top = radarPoint(frame, 0, 4, 1);
    expect(top.y).toBeCloseTo(-1, 12);
    expect(radarFraction(5, 3, 3)).toBe(0);
  });

  it("renders one named image with the exact values as a table alternative", async () => {
    const text = quizText("de");
    const axes = [
      { id: "heat", label: "Wärme", unit: "kWh/m²", min: 0, max: 200 },
      { id: "light", label: "Licht", unit: "%", min: 0, max: 100 },
      { id: "mass", label: "Masse", unit: "t", min: 100, max: 1000 },
    ];
    render(<RadarChart name="Passivhaus" axes={axes} values={{ heat: 15, light: 80, mass: 550 }} text={text} locale="de" />);
    const image = screen.getByRole("img", { name: "Netzdiagramm von Passivhaus" });
    expect(image.querySelectorAll("polygon")).toHaveLength(5);
    await userEvent.setup().click(screen.getByText("Werte als Tabelle"));
    const table = screen.getByRole("table", { name: "Netzdiagramm von Passivhaus" });
    const rows = within(table).getAllByRole("row");
    expect(rows).toHaveLength(4);
    expect(within(rows[1]!).getByRole("rowheader").textContent).toBe("Wärme");
    expect(
      within(rows[1]!)
        .getAllByRole("cell")
        .map((cell) => cell.textContent),
    ).toEqual([withUnit(formatNumber(15, "de"), "kWh/m²"), withUnit("0", "kWh/m²"), withUnit("200", "kWh/m²")]);
    expect(
      within(rows[3]!)
        .getAllByRole("cell")
        .map((cell) => cell.textContent),
    ).toEqual([withUnit("550", "t"), withUnit("100", "t"), withUnit("1.000", "t")]);
  });

  it("draws a profile whose keys are hidden as shares of each range, said as percentages without values, units or ranges", async () => {
    const text = quizText("de");
    const axes = [
      { id: "heat", label: "Wärme", unit: "", min: 0, max: 1 },
      { id: "light", label: "Licht", unit: "", min: 0, max: 1 },
      { id: "mass", label: "Masse", unit: "", min: 0, max: 1 },
    ];
    const shares = { heat: 0.075, light: 0.8, mass: 0.5 };
    const { container } = render(<RadarChart name="Passivhaus" axes={axes} values={shares} normalised text={text} locale="de" />);
    const image = screen.getByRole("img", { name: "Netzdiagramm von Passivhaus" });
    const scale = scaleLinear().domain([0, 1]).range([0, 1]).clamp(true);
    const frame = { cx: 0, cy: 0, radius: 1 };
    expect(radarPolygon(frame, axes, shares).map((point) => Math.hypot(point.x, point.y))).toEqual(axes.map((axis) => expect.closeTo(scale(shares[axis.id as keyof typeof shares]), 12)));
    const percent = (share: number): string => new Intl.NumberFormat("de", { style: "percent", maximumFractionDigits: 1 }).format(share);
    expect(document.getElementById(image.getAttribute("aria-describedby") ?? "")?.textContent).toBe(`Wärme: ${percent(0.075)}; Licht: ${percent(0.8)}; Masse: ${percent(0.5)}. Die Werte folgen als Tabelle.`);
    await userEvent.setup().click(screen.getByText("Werte als Tabelle"));
    const table = screen.getByRole("table", { name: "Netzdiagramm von Passivhaus" });
    expect(within(table).getAllByRole("columnheader").map((head) => head.textContent)).toEqual(["Achse", "Anteil am Bereich"]);
    expect(within(table).getAllByRole("cell").map((cell) => cell.textContent)).toEqual([percent(0.075), percent(0.8), percent(0.5)]);
    expect(container.textContent).not.toMatch(/Minimum|Maximum/u);
  });
});

describe("🏷️ radar labels", () => {
  for (const vector of fixture.wraps) {
    it(`wraps the shared label ${vector.id}`, () => {
      expect(wrapLabel(vector.label, vector.width, monospace)).toEqual(vector.lines);
    });
  }

  for (const layout of fixture.layouts) {
    it(`lays out the shared case ${layout.id} inside its view box, measured by the estimate and by the brand font`, () => {
      for (const measure of [estimate(layout.fontSize), antaMeasure(layout.fontSize)]) {
        const result = radarLayout(layout.labels, layout.width, layout.fontSize, measure);
        expect(result.width).toBe(layout.width);
        expect(violations(result, layout.labels, layout.fontSize, measure)).toEqual([]);
      }
    });
  }

  it("keeps every label inside the view box at every width from 200 to 1400 px, at normal and at large text", () => {
    let checked = 0;
    for (const labels of SWEEP_LABELS) {
      for (const fontSize of [13.6, 20.4]) {
        for (const measure of [estimate(fontSize), antaMeasure(fontSize)]) {
          for (const width of SWEEP_WIDTHS) {
            const found = violations(radarLayout(labels, width, fontSize, measure), labels, fontSize, measure);
            if (found.length > 0) expect(found, `${width} px at ${fontSize} px: ${labels.join(" | ")}`).toEqual([]);
            checked += 1;
          }
        }
      }
    }
    expect(checked).toBe(SWEEP_LABELS.length * 2 * 2 * SWEEP_WIDTHS.length);
  });

  it("lays out three to twelve spokes of long labels soundly", () => {
    const phrase = "Netto-Energiekosten für Heizung, Warmwasser und Hilfsenergie (nach PV-Gutschrift)".split(" ");
    for (let count = 3; count <= 12; count += 1) {
      const labels = Array.from({ length: count }, (_, index) => phrase.slice(0, 1 + ((index * 5) % phrase.length)).join(" "));
      for (const width of [240, 287, 309, 400, 512, 800]) {
        const measure = antaMeasure(13.6);
        expect(violations(radarLayout(labels, width, 13.6, measure), labels, 13.6, measure), `${count} spokes at ${width} px`).toEqual([]);
      }
    }
  });

  it("never estimates a text narrower than the brand font sets it, nor more than a third wider", () => {
    const measure = antaMeasure(16);
    const texts = new Set(fixture.layouts.flatMap((layout) => layout.labels.flatMap((label) => [label, ...label.split(/\s+/u)])).filter((text) => text !== ""));
    for (const code of [...Array.from({ length: 95 }, (_, index) => 32 + index), ...Array.from({ length: 96 }, (_, index) => 160 + index), 0x2013, 0x2014, 0x20ac]) texts.add(String.fromCodePoint(code));
    for (const text of texts) {
      if ([...text].some((char) => ANTA.every((font) => font.charToGlyphIndex(char) === 0))) continue;
      const real = measure(text);
      expect(estimateTextWidth(text, 16), JSON.stringify(text)).toBeGreaterThanOrEqual(real - EPSILON);
      if ([...text].length > 8) expect(estimateTextWidth(text, 16), JSON.stringify(text)).toBeLessThanOrEqual(real * (4 / 3));
    }
  });

  it("keeps the brand font's real lines inside the boxes laid out from the estimate", () => {
    for (const layout of fixture.layouts) {
      const real = antaMeasure(layout.fontSize);
      for (const label of radarLayout(layout.labels, layout.width, layout.fontSize, estimate(layout.fontSize)).labels) {
        for (const line of label.lines) expect(real(line), `${layout.id}: ${line}`).toBeLessThanOrEqual(label.box.width + EPSILON);
      }
    }
  });

  it("gives short labels the full circle and lets long side labels wrap beside a smaller one", () => {
    const wide = radarLayout(["A", "B", "C"], 900, 13.6, estimate(13.6));
    expect(wide.frame.radius).toBeCloseTo(RADAR_METRICS.maxRadius * 13.6, 9);
    expect(wide.labels.map((label) => label.lines)).toEqual([["A"], ["B"], ["C"]]);
    expect(wide.legend).toEqual([]);
    const phone = radarLayout(fixture.layouts[0]!.labels, 287, 13.6, antaMeasure(13.6));
    for (const width of [287, 309, 512]) {
      const branded = radarLayout(fixture.layouts[0]!.labels, width, 13.6, antaMeasure(13.6));
      expect(branded.legend, `${width} px`).toEqual([]);
      expect(
        branded.labels.map((label) => rejoined(label.lines)),
        `${width} px`,
      ).toEqual(fixture.layouts[0]!.labels);
      expect(branded.frame.radius, `${width} px`).toBeGreaterThanOrEqual(RADAR_METRICS.wordRadius * 13.6);
    }
    expect(phone.labels.map((label) => label.anchor)).toEqual(["middle", "start", "middle", "end"]);
    expect(phone.labels[3]!.lines.length).toBeGreaterThan(3);
    expect(phone.labels[0]!.box.y + phone.labels[0]!.box.height).toBeLessThanOrEqual(phone.frame.cy - phone.frame.radius);
    expect(phone.labels[2]!.box.y).toBeGreaterThanOrEqual(phone.frame.cy + phone.frame.radius);
    const lopsided = radarLayout(["Heizwärmebedarf", "Kühlbedarf", "Lüftung", fixture.layouts[0]!.labels[3]!], 512, 13.6, estimate(13.6));
    expect(lopsided.labels[1]!.lines).toEqual(["Kühlbedarf"]);
    expect(lopsided.frame.cx).toBeGreaterThan(512 / 2 + 13.6 * 2);
    expect(lopsided.labels[3]!.box.width).toBeGreaterThan(512 / 2 - lopsided.frame.radius);
  });

  it("numbers the spokes and lists the labels below across the whole width once a side label's longest word no longer fits", () => {
    const labels = fixture.layouts[0]!.labels;
    const fontSize = 1.5 * 16 * RADAR_LABEL_EM;
    const measure = antaMeasure(fontSize);
    const layout = radarLayout(labels, 245, fontSize, measure);
    expect(layout.labels.map((label) => label.lines)).toEqual([["1"], ["2"], ["3"], ["4"]]);
    expect(layout.legend.map((entry) => entry.key.lines)).toEqual([["1"], ["2"], ["3"], ["4"]]);
    expect(layout.legend.map((entry) => withoutHyphens(rejoined(entry.text.lines)))).toEqual(labels.map(withoutHyphens));
    expect(layout.legend.map((entry) => rejoined(entry.text.lines))[3]).toBe(labels[3]);
    expect(layout.legend[0]!.key.box.y).toBeGreaterThanOrEqual(layout.frame.cy + layout.frame.radius);
    expect(layout.frame.radius).toBeGreaterThanOrEqual(RADAR_METRICS.minRadius * fontSize);
    expect(Math.max(...layout.legend.map((entry) => entry.text.box.x + entry.text.box.width))).toBeGreaterThan(245 * 0.75);
    expect(violations(layout, labels, fontSize, measure)).toEqual([]);
    expect(radarLayout(labels, 287, 13.6, antaMeasure(13.6)).legend).toEqual([]);
  });

  for (const { mode, chart, em, keys } of [
    { mode: "labels at the spokes", chart: 512.4, em: 16, keys: 0 },
    { mode: "numbered spokes and a legend", chart: 287.6, em: 24, keys: 8 },
  ]) {
    it(`draws at the width it gets, one user unit per pixel, with ${mode} at the reading size, line by line`, () => {
      const labels = fixture.layouts[0]!.labels;
      const axes = labels.map((label, index) => ({ id: `axis-${index}`, label, unit: "kWh/(m²·a)", min: 0, max: 100 }));
      vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockImplementation(function (this: HTMLElement): DOMRect {
        const width = this.classList.contains("quiz-radar-ruler") ? 10 * em : this.classList.contains("quiz-radar") ? chart : 0;
        return { width, height: 0, x: 0, y: 0, top: 0, left: 0, right: width, bottom: 0, toJSON: () => ({}) };
      });
      render(<RadarChart name="Passivhaus" axes={axes} values={{ "axis-0": 15, "axis-1": 2, "axis-2": 10, "axis-3": 20 }} text={quizText("de")} locale="de" />);
      const image = screen.getByRole("img", { name: "Netzdiagramm von Passivhaus" });
      const [x, y, width, height] = (image.getAttribute("viewBox") ?? "").split(" ").map(Number);
      expect([x, y, width]).toEqual([0, 0, Math.floor(chart)]);
      expect(image.getAttribute("width")).toBe(String(Math.floor(chart)));
      expect(Number(image.getAttribute("height"))).toBe(height);
      expect(Number(image.querySelector(".quiz-radar-labels")?.getAttribute("font-size"))).toBe(em * RADAR_LABEL_EM);
      expect(image.querySelectorAll("text.quiz-radar-key")).toHaveLength(keys);
      const texts = [...image.querySelectorAll("text.quiz-radar-label")];
      expect(texts).toHaveLength(4);
      texts.forEach((node, index) => {
        const lines = [...node.querySelectorAll("tspan")];
        expect(withoutHyphens(rejoined(lines.map((line) => line.textContent ?? "")))).toBe(withoutHyphens(labels[index]!));
        for (const line of lines) {
          expect(Number(line.getAttribute("y"))).toBeGreaterThan(0);
          expect(Number(line.getAttribute("y"))).toBeLessThan(height!);
          expect(Number(line.getAttribute("x"))).toBeGreaterThanOrEqual(0);
          expect(Number(line.getAttribute("x"))).toBeLessThanOrEqual(width!);
        }
      });
      expect(texts[3]!.querySelectorAll("tspan").length).toBeGreaterThan(1);
    });
  }

  it("lays out for 18 em at the default text size until it can measure", () => {
    const axes = fixture.layouts[0]!.labels.map((label, index) => ({ id: `axis-${index}`, label, unit: "", min: 0, max: 1 }));
    render(<RadarChart name="Passivhaus" axes={axes} values={{}} text={quizText("en")} locale="en" />);
    const image = screen.getByRole("img", { name: "Spider diagram of Passivhaus" });
    expect(image.getAttribute("viewBox")?.split(" ").slice(0, 3)).toEqual(["0", "0", "288"]);
    expect(Number(image.querySelector(".quiz-radar-labels")?.getAttribute("font-size"))).toBe(16 * RADAR_LABEL_EM);
  });
});
