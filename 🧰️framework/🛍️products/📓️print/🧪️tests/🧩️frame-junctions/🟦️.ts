import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { evaluate } from "mathjs";
import Ajv from "ajv";
import fixture from "./🔣️.json";

const schema = {
  type: "object",
  additionalProperties: false,
  required: ["stroke", "neutral", "junctions", "row", "list", "edge", "page"],
  properties: {
    stroke: { type: "number", exclusiveMinimum: 0 },
    neutral: { type: "string", minLength: 1 },
    junctions: {
      type: "array",
      minItems: 1,
      items: {
        type: "object",
        additionalProperties: false,
        required: ["name", "previousBottom", "gap", "previousColor", "ownColor", "top", "sharedColor"],
        properties: {
          name: { type: "string", minLength: 1 },
          previousBottom: { type: "number" },
          gap: { type: "number" },
          previousColor: { type: "string", minLength: 1 },
          ownColor: { type: "string", minLength: 1 },
          top: { type: "number" },
          sharedColor: { type: "string", minLength: 1 },
        },
      },
    },
    row: {
      type: "object",
      additionalProperties: false,
      required: ["width", "weights", "lefts", "widths"],
      properties: {
        width: { type: "number", exclusiveMinimum: 0 },
        weights: { type: "array", minItems: 1, items: { type: "number", exclusiveMinimum: 0 } },
        lefts: { type: "array", minItems: 1, items: { type: "number" } },
        widths: { type: "array", minItems: 1, items: { type: "number", exclusiveMinimum: 0 } },
      },
    },
    list: {
      type: "object",
      additionalProperties: false,
      required: ["points", "dividers"],
      properties: { points: { type: "integer", minimum: 1 }, dividers: { type: "integer", minimum: 2 } },
    },
    edge: {
      type: "object",
      additionalProperties: false,
      required: ["reach", "samples"],
      properties: {
        reach: { type: "integer", exclusiveMinimum: 0 },
        samples: {
          type: "array",
          minItems: 1,
          items: {
            type: "object",
            additionalProperties: false,
            required: ["name", "edge", "bottom", "closed"],
            properties: { name: { type: "string", minLength: 1 }, edge: { type: "integer" }, bottom: { type: "integer" }, closed: { type: "boolean" } },
          },
        },
      },
    },
    page: {
      type: "object",
      additionalProperties: false,
      required: ["headSeparation", "footerHeight", "footPadding", "footSkip"],
      properties: { headSeparation: { type: "number" }, footerHeight: { type: "number" }, footPadding: { type: "number", minimum: 0 }, footSkip: { type: "number" } },
    },
  },
} as const;

const near = (left: number, right: number): boolean => Math.abs(left - right) < 0.01;

/** 🧩️ Where a frame lands when it opens on a pending frame: the glue below the previous frame is cancelled and the frame moves up one hairline. */
export function frameJunctionTop(previousBottom: number, gap: number, stroke: number): number {
  return previousBottom + gap - gap - stroke;
}

/** 🎨️ The colour of a frame edge: a horizontal edge is shared by the two sections it separates and stays neutral; a vertical edge belongs to its part and may carry the part's accent. */
export function frameEdgeColor(orientation: "horizontal" | "vertical", accent: string, neutral: string): string {
  return orientation === "horizontal" ? neutral : accent;
}

/** 📄️ Whether a bottom edge shipped at `edge` (scaled points above the paper bottom) lies on the text bottom, so the footer rule closes its frame instead. */
export function edgeClosedByFooter(edge: number, bottom: number, reach: number): boolean {
  return edge - bottom < reach;
}

/** 🧩️ Checks the frame junction rules against neutral vectors, independent arithmetic and the LaTeX sources. */
export function verifyFrameJunctions(): void {
  assert.ok(new Ajv({ strict: true }).compile(schema)(fixture));
  for (const row of fixture.junctions) {
    const top = frameJunctionTop(row.previousBottom, row.gap, fixture.stroke);
    const oracle = Number(evaluate("bottom + gap + (-gap - stroke)", { bottom: row.previousBottom, gap: row.gap, stroke: fixture.stroke }));
    assert.ok(near(top, oracle) && near(top, row.top), row.name);
    assert.ok(near(row.previousBottom - top, fixture.stroke), row.name);
    assert.equal(frameEdgeColor("horizontal", row.previousColor, fixture.neutral), row.sharedColor, row.name);
    assert.equal(frameEdgeColor("horizontal", row.ownColor, fixture.neutral), row.sharedColor, row.name);
    assert.equal(frameEdgeColor("vertical", row.ownColor, fixture.neutral), row.ownColor, row.name);
  }
  const total = fixture.row.weights.reduce((sum, weight) => sum + weight, 0), tiles = fixture.row.weights.length;
  const usable = Number(evaluate("width + (tiles - 1) * stroke", { width: fixture.row.width, tiles, stroke: fixture.stroke }));
  let left = 0;
  fixture.row.weights.forEach((weight, index) => {
    const width = usable * weight / total;
    assert.ok(near(left, fixture.row.lefts[index]!) && near(width, fixture.row.widths[index]!), "row tile " + index);
    left += width - fixture.stroke;
  });
  assert.ok(near(left + fixture.stroke, fixture.row.width));
  assert.equal(fixture.list.dividers, Number(evaluate("points + 1", { points: fixture.list.points })));
  assert.equal(fixture.edge.reach, Number(evaluate("2 * 2^16")));
  for (const sample of fixture.edge.samples) {
    assert.equal(edgeClosedByFooter(sample.edge, sample.bottom, fixture.edge.reach), sample.closed, sample.name);
    assert.equal(Boolean(evaluate("edge - bottom < reach", { ...sample, reach: fixture.edge.reach })), sample.closed, sample.name);
  }
  assert.ok(near(fixture.page.headSeparation, -fixture.stroke));
  assert.ok(near(fixture.page.footSkip, Number(evaluate("height - stroke + padding", { height: fixture.page.footerHeight, stroke: fixture.stroke, padding: fixture.page.footPadding }))));
  const read = (name: string): string => readFileSync(new URL("../../🖋️latex/" + name, import.meta.url), "utf8");
  const core = read("semio-core.sty"), window = read("semio-window.sty"), table = read("semio-table.sty");
  for (const text of [
    String.raw`\skip@\glueexpr\semio@frame@pending@gap\relax
    \vskip\glueexpr-\skip@-\semio@stroke@hairline\relax`,
    String.raw`\xdef\semio@frame@pending@gap{\the\glueexpr#1\relax}%`,
    String.raw`\AddToHook{para/begin}{\semio@frame@release}`,
    String.raw`\AddToHook{cmd/vspace/before}{\semio@frame@release}`,
    String.raw`\newcommand{\semio@frame@open}[1]{`,
    String.raw`\newcommand{\semio@frame@record}[1]{`,
    String.raw`\newcommand{\semio@column@inset}`,
    String.raw`\newcommand{\semio@column@bleed}`,
    String.raw`\AddToHook{env/itemize/begin}{\semio@list@install}`,
    String.raw`\AddToHook{env/enumerate/end}{\semio@list@close}`,
    String.raw`\ifsemio@frame@pending
    \semio@frame@open\z@`,
    String.raw`\nobreak\vskip\dimexpr\itemsep+\parsep\relax
    \dimen@\z@`,
    String.raw`\newcommand{\semio@frame@seal}[1]{`,
    String.raw`\cleaders\hbox{\kern-\dimexpr#2\relax\textcolor{semio-chrome-border-normal}{\vrule width\dimexpr#3\relax height\semio@stroke@hairline}}\vskip\semio@stroke@hairline`,
    String.raw`\semio@frame@edge{\penalty-\@lowpenalty}{\dimen@ii}{\hsize+2\dimen@ii}%`,
    String.raw`\@itempenalty\@M`,
    String.raw`\semio@frame@edge{}{\z@}{\columnwidth}%`,
    String.raw`\semio@frame@edge{\penalty\@endparpenalty}{\z@}{\hsize}%
    \@endparpenalty\@M`,
    String.raw`\newcommand{\semio@frame@edge@reach}{${fixture.edge.reach}}`,
    String.raw`\ifnum\numexpr#2-\semio@frame@page@bottom\relax<\semio@frame@edge@reach\relax`,
    String.raw`\ifcsname semio@frame@edge@end@\the\semio@frame@edge@count\endcsname
    \vskip\semio@stroke@hairline
  \else`,
    String.raw`\vskip\semio@stroke@hairline
  \fi
  \kern\z@
}`,
    String.raw`\noexpand\semio@frame@edge@at{\the\semio@frame@edge@count}{\noexpand\the\noexpand\pdflastypos}`,
    String.raw`\noexpand\pdflastypos-\number\ht\@cclv\noexpand\relax`,
    String.raw`\setbox\@cclv\vbox to\ht\@cclv{\boxmaxdepth\maxdimen\pdfsavepos\semio@frame@page@write\unvbox\@cclv}%`,
    String.raw`\AddToHook{cmd/@makecol/before}{\semio@frame@page@end}`,
  ]) assert.ok(core.includes(text), text);
  for (const text of ["semio@frame@mark", "semio@frame@trim", "edge@found"]) assert.ok(!core.includes(text), text);
  const assignment = read("semio-assignment.sty");
  assert.equal((window.match(/semio@frame@seal/g) ?? []).length, 2);
  for (const text of [
    String.raw`! \l_semio_window_in_item_bool && \dim_compare_p:n { \l_semio_window_arrangement_height_dim >= \textheight }`,
    String.raw`\dim_set:Nn \l_semio_window_arrangement_height_dim { \textheight + \use:c { semio@window@bodypad } }`,
    String.raw`\vbox to \textheight { \semio_window_arrangement_body:n {#2} \vss }`,
  ]) assert.ok(window.includes(text), text);
  assert.equal((table.match(/\\semio@frame@seal\{\\semio@spacing@table@outer\}%/g) ?? []).length, 2);
  assert.equal((assignment.match(/\\use:c \{ semio@frame@seal \} \{ 0pt \}/g) ?? []).length, 2);
  assert.ok(!table.includes(String.raw`\semio@frame@close`) && !assignment.includes("semio@frame@close"));
  assert.ok(!core.includes("[DEBUG]") && !window.includes("[DEBUG]"));
  for (const text of [
    String.raw`\setlength{\semio@window@gap}{-\semio@stroke@hairline}`,
    String.raw`headsep=-\semio@stroke@hairline`,
    String.raw`\newcommand{\semio@chrome@column@side}`,
    String.raw`height~fixed~for=first~and~middle`,
    String.raw`enlarge~bottom~at~break~by=-\semio@window@bodypad`,
    String.raw`footskip=\dimexpr\semio@chrome@footer@reserve+\semio@window@bodypad\relax`,
    String.raw`ignore~nobreak=true,`,
    String.raw`\renewcommand*{\chapterheadendvskip}{}%`,
  ]) assert.ok(window.includes(text), text);
  const start = window.indexOf(String.raw`\newcommand{\semio@heading@frame@row}`), row = window.slice(start, window.indexOf("\n}", start));
  assert.ok(start >= 0);
  assert.equal((row.match(/\\hrule/g) ?? []).length, 1);
  assert.ok(row.includes(String.raw`\textcolor{semio-chrome-border-normal}{\vbox to \semio@heading@frame@h{\hrule`) && !row.includes(String.raw`\vss\hrule`));
  const edges = window.slice(window.indexOf(String.raw`\newcommand{\semio@chip@edges}`), window.indexOf(String.raw`\newcommand{\semio@chip@box}`));
  assert.equal((edges.match(/\\textcolor\{semio-chrome-border-normal\}\{\\vbox/g) ?? []).length, 2);
  assert.equal((edges.match(/\\textcolor\{#2\}\{\\rule\{\\semio@stroke@hairline\}/g) ?? []).length, 2);
  for (const source of [core, window, table]) assert.doesNotMatch(source, /frame@top@color|frame@bottom@color|frame@edge@neutral|window_top_color|frame@top@shared|long@shared@edge/);
  assert.match(table, /\\endhead\s*\\semio@table@long@break@foot\s*\\endfoot\s*\\endlastfoot/);
  assert.ok(table.includes(String.raw`\textcolor{semio-chrome-interior}{\rule{\dimexpr\linewidth-2\arrayrulewidth\relax}{\arrayrulewidth}}`));
}
