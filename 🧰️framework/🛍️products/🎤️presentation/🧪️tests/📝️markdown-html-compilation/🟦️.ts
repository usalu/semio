// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import rehypeStringify from "rehype-stringify";
import remarkGfm from "remark-gfm";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import { unified } from "unified";
import { type AdapterContext, type AdapterOutcome, defineTestAdapter } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileOwnedMarkdownToHtml } from "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🔮️Reference
/** 🔮️ The CommonMark + GFM reading of the ecosystem, assembled once and reused by every vector. */
const reference = unified().use(remarkParse).use(remarkGfm).use(remarkRehype).use(rehypeStringify);

/** 🔮️ The reference fragment of one markdown source. @see https://github.com/remarkjs/remark */
async function compileReferenceMarkdownToHtml(markdown: string): Promise<string> {
  return String(await reference.process(markdown));
}
// #endregion 🔮️Reference

// #region 🧫️Vectors
/** ⚖️ Line endings and the fragment's outer whitespace are producer freedom; nothing else is. */
function normalizeFragment(html: string): string {
  return html.replace(/\r\n?/gu, "\n").trim();
}

/** 🧫️ The scenario's data table as records — the feature owns every vector this case compares. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...body] = table;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index] ?? ""])));
}

/** 🧫️ The markdown source of one vector, read from the immutable fixture the feature names. */
function markdownOf(ctx: AdapterContext, uri: string): string {
  return new TextDecoder().decode(ctx.fixtureBytes(uri));
}

/** 🎛️ Compiles every vector of the scenario with one compiler and keys the fragments by vector. */
async function fragments(ctx: AdapterContext, compile: (markdown: string) => Promise<string>): Promise<AdapterOutcome> {
  const projection: Record<string, string> = {};
  for (const row of rows(ctx)) {
    const vector = row.vector ?? "";
    const fixture = row.fixture ?? "";
    if (vector.length === 0 || fixture.length === 0) throw new Error(`scenario ${ctx.scenario.id} has a vector row without a name or a fixture`);
    projection[`markdown/${vector}`] = normalizeFragment(await compile(markdownOf(ctx, fixture)));
  }
  return { raw: JSON.stringify(projection, null, 2), projection };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
/** 🎛️ Every scenario asks the same question of a different vector set, so they share one shape. */
const markdownScenario = () => ({
  oracle: (ctx: AdapterContext) => fragments(ctx, compileReferenceMarkdownToHtml),
  subject: (ctx: AdapterContext) => fragments(ctx, compileOwnedMarkdownToHtml),
});

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    /** 🔮️/🎯️ Paragraphs, soft breaks and the six heading levels. */
    prose: markdownScenario(),
    /** 🔮️/🎯️ Emphasis, strong, inline code, backslash escapes and hard breaks. */
    inline: markdownScenario(),
    /** 🔮️/🎯️ Bullet markers, ordered starts, nesting and inline marks inside items. */
    lists: markdownScenario(),
    /** 🔮️/🎯️ Inline links, titles, mailto destinations and autolinks. */
    links: markdownScenario(),
    /** 🔮️/🎯️ Fenced code: language class, escaping and the trailing newline. */
    code: markdownScenario(),
    /** 🔮️/🎯️ GFM tables and their per-column alignment attributes. */
    tables: markdownScenario(),
    /** 🔮️/🎯️ A whole slide body, as a deck actually authors one. */
    slide: markdownScenario(),
  },
});
// #endregion 🧭️Adapter
