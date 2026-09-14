//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔮️Oracle
type Rewrite = { oldName: string; newName: string; newDescription: string };
type LineRewrite = { line: number; newName: string; newDescription: string };
type Vectors = {
  markdown: string;
  markdownPath: string;
  source: string;
  sourcePath: string;
  markdownRewrites: Rewrite[];
  sourceRewrites: LineRewrite[];
  openerPaths: string[];
};

const MARKDOWN_PREFIX = "- TODO ";

/** 📰️ The `.todos.md` item grammar, restated as a regular expression rather than a hand written scanner. */
const markdownItem = (line: string): { name: string; description: string } | null => {
  const match = /^\s*-\sTODO\s+([^:\n]*?)(?::\s*(.*?))?\s*$/u.exec(line);
  if (match === null) return null;
  return { name: match[1]!.trim(), description: (match[2] ?? "").trim() };
};

/** 💬️ The comment grammar, restated as a regular expression: opener, `TODO`, a colon-free name, a colon, the rest. */
const commentItem = (line: string): { prefix: string; name: string; description: string } | null => {
  const match = /^(\s*(?:\/\/|#|--)\s*TODO\s+)([^:]*?):(.*)$/u.exec(line);
  if (match === null) return null;
  if (match[2]!.trim() === "") return null;
  return { prefix: match[1]!, name: match[2]!.trim(), description: match[3]!.trim() };
};

const parseMarkdown = (document: string, parent: string): { name: string; description: string; parentId: string; location: string }[] =>
  document
    .split("\n")
    .map(markdownItem)
    .filter((item): item is { name: string; description: string } => item !== null)
    .map((item) => ({ ...item, parentId: parent, location: `${parent === "" ? ".todos.md" : `${parent}/.todos.md`}:0:0` }));

const parseComments = (document: string, path: string): { name: string; description: string; parentId: string; location: string }[] =>
  document
    .split("\n")
    .map((line, index) => ({ item: commentItem(line), index }))
    .filter((entry): entry is { item: { prefix: string; name: string; description: string }; index: number } => entry.item !== null)
    .map((entry) => ({ name: entry.item.name, description: entry.item.description, parentId: path, location: `${path}:${entry.index + 1}:1` }));

const opener = (path: string): string => {
  const name = path.split(/[/\\]/u).pop() ?? path;
  const dot = name.lastIndexOf(".");
  const extension = dot > 0 ? name.slice(dot) : "";
  if ([".py", ".sh", ".yaml", ".yml"].includes(extension)) return "#";
  if ([".sql", ".lua"].includes(extension)) return "--";
  return "//";
};

/**
 * 🔮️ TypeScript oracle of the todo line grammar case.
 *
 * `ajv` is a real draft 2020-12 validator and decides whether the committed vectors satisfy
 * `🧬️schema/🔣️.json`, so a drifted schema fails here instead of agreeing with itself. The grammar
 * itself is restated with JavaScript regular expressions — a different engine from either hand
 * written scanner — and every rewrite and removal is re-derived from the committed documents,
 * which is what makes the round trip claim mean something.
 */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "rewriting-a-line-and-reading-it-back-agrees": {
      oracle: (ctx) => {
        const schema = JSON.parse(readFileSync(ctx.fixture("asset://🧬️schema/🔣️.json"), "utf8")) as Record<string, unknown>;
        const vectors = JSON.parse(readFileSync(ctx.fixture("shared://📝️line-vectors.json"), "utf8")) as Vectors;
        const ajv = new Ajv2020({ strict: false, allErrors: true });
        const validate = ajv.compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/LineVectors" });
        if (!validate(vectors)) throw new Error(`📝️line-vectors.json does not satisfy LineVectors: ${ajv.errorsText(validate.errors)}`);

        const parent = vectors.markdownPath.includes("/") ? vectors.markdownPath.slice(0, vectors.markdownPath.lastIndexOf("/")) : "";
        const render = (item: { name: string; description: string; parentId: string; location: string }): string => `${item.name}|${item.description}|${item.parentId}|${item.location}`;
        const names = (document: string, inMarkdown: boolean): string =>
          (inMarkdown ? parseMarkdown(document, parent) : parseComments(document, vectors.sourcePath)).map((item) => `${item.name}:${item.description}`).join(",");

        const markdownLines = vectors.markdown.split("\n");
        const markdownRewrites: string[] = [];
        const markdownReparsed: string[] = [];
        for (const rewrite of vectors.markdownRewrites) {
          const index = markdownLines.findIndex((line) => line.trimStart().startsWith(`${MARKDOWN_PREFIX}${rewrite.oldName}:`));
          if (index === -1) {
            markdownRewrites.push("err:not-found");
            markdownReparsed.push("-");
            continue;
          }
          const rewritten = [...markdownLines];
          rewritten[index] = `${MARKDOWN_PREFIX}${rewrite.newName}: ${rewrite.newDescription}`;
          const document = rewritten.join("\n");
          markdownRewrites.push(`ok:${document}`);
          markdownReparsed.push(names(document, true));
        }

        const sourceLines = vectors.source.split("\n");
        const sourceRewrites: string[] = [];
        const sourceReparsed: string[] = [];
        for (const rewrite of vectors.sourceRewrites) {
          if (rewrite.line <= 0 || rewrite.line > sourceLines.length) {
            sourceRewrites.push("err:no-location");
            sourceReparsed.push("-");
            continue;
          }
          const parts = commentItem(sourceLines[rewrite.line - 1]!);
          if (parts === null) {
            sourceRewrites.push("err:not-found");
            sourceReparsed.push("-");
            continue;
          }
          const rewritten = [...sourceLines];
          rewritten[rewrite.line - 1] = `${parts.prefix}${rewrite.newName}: ${rewrite.newDescription}`;
          const document = rewritten.join("\n");
          sourceRewrites.push(`ok:${document}`);
          sourceReparsed.push(names(document, false));
        }

        const markdownRemovals = vectors.markdownRewrites.map((rewrite) =>
          markdownLines.filter((line) => !line.trimStart().startsWith(`${MARKDOWN_PREFIX}${rewrite.oldName}:`)).join("\n"),
        );
        const sourceRemovals = vectors.sourceRewrites.map((rewrite) => {
          if (rewrite.line <= 0 || rewrite.line > sourceLines.length) return sourceLines.join("\n");
          return sourceLines.filter((_, index) => index !== rewrite.line - 1).join("\n");
        });

        return {
          projection: {
            markdownParse: parseMarkdown(vectors.markdown, parent).map(render),
            sourceParse: parseComments(vectors.source, vectors.sourcePath).map(render),
            markdownRewrites,
            markdownReparsed,
            sourceRewrites,
            sourceReparsed,
            markdownRemovals,
            sourceRemovals,
            parts: sourceLines.map((line) => {
              const item = commentItem(line);
              return item === null ? "-" : `${item.prefix}|${item.name}|${item.description}`;
            }),
            openers: vectors.openerPaths.map((path) => `${path}=${opener(path)}`),
          },
        };
      },
    },
  },
});
//#endregion 🔮️Oracle
