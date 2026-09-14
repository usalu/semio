// #region 🔖️Schema
type Inline =
  | { readonly kind: "text"; readonly value: string }
  | { readonly kind: "emphasis"; readonly children: readonly Inline[] }
  | { readonly kind: "strong"; readonly children: readonly Inline[] }
  | { readonly kind: "code"; readonly value: string }
  | { readonly kind: "link"; readonly href: string; readonly title?: string; readonly children: readonly Inline[] }
  | { readonly kind: "break" };
type Alignment = "left" | "center" | "right" | undefined;
type Block =
  | { readonly kind: "paragraph"; readonly children: readonly Inline[] }
  | { readonly kind: "heading"; readonly depth: number; readonly children: readonly Inline[] }
  | { readonly kind: "code"; readonly language?: string; readonly value: string }
  | { readonly kind: "list"; readonly ordered: boolean; readonly start: number; readonly items: readonly ListItem[] }
  | { readonly kind: "table"; readonly alignments: readonly Alignment[]; readonly header: readonly (readonly Inline[])[]; readonly rows: readonly (readonly (readonly Inline[])[])[] };
interface ListItem {
  readonly blocks: readonly Block[];
}
interface Document {
  readonly kind: "document";
  readonly blocks: readonly Block[];
}
interface Marker {
  readonly indent: number;
  readonly ordered: boolean;
  readonly start: number;
  readonly content: string;
}
// #endregion 🔖️Schema

// #region 🔗️UrlPolicy
const SAFE_SCHEMES = new Set(["http", "https", "mailto", "tel"]);
function safeHref(target: string): boolean {
  const href = target.trim();
  if (href.length === 0 || /[\u0000-\u001f\u007f]/u.test(href) || href.startsWith("\\") || href.startsWith("//")) return false;
  const scheme = /^([A-Za-z][A-Za-z\d+.-]*):/u.exec(href)?.[1]?.toLowerCase();
  return scheme === undefined || SAFE_SCHEMES.has(scheme);
}
// #endregion 🔗️UrlPolicy

// #region 🧩️InlineParser
function unescaped(source: string, token: string, start: number): number {
  for (let index = start; index <= source.length - token.length; index += 1) {
    if (source.startsWith(token, index) && source[index - 1] !== "\\") return index;
  }
  return -1;
}
function appendText(nodes: Inline[], value: string): void {
  if (value.length === 0) return;
  const previous = nodes.at(-1);
  if (previous?.kind === "text") nodes[nodes.length - 1] = { kind: "text", value: previous.value + value };
  else nodes.push({ kind: "text", value });
}
function destination(source: string, start: number): { readonly href: string; readonly title?: string; readonly end: number } | undefined {
  let depth = 1;
  let escaped = false;
  for (let index = start; index < source.length; index += 1) {
    const character = source[index]!;
    if (escaped) {
      escaped = false;
      continue;
    }
    if (character === "\\") {
      escaped = true;
      continue;
    }
    if (character === "(") depth += 1;
    if (character === ")") depth -= 1;
    if (depth !== 0) continue;
    const body = source.slice(start, index).trim();
    const match = /^(?:<([^>]+)>|([^\s]+))(?:\s+(?:"([^"]*)"|'([^']*)'|\(([^)]*)\)))?$/u.exec(body);
    if (match === null) return undefined;
    return { href: (match[1] ?? match[2] ?? "").replace(/\\([\\()[\]])/gu, "$1"), title: match[3] ?? match[4] ?? match[5], end: index + 1 };
  }
  return undefined;
}
function codeSpan(value: string): string {
  const normalized = value.replace(/[\r\n]+/gu, " ");
  return normalized.startsWith(" ") && normalized.endsWith(" ") && normalized.trim().length > 0 ? normalized.slice(1, -1) : normalized;
}
function parseInline(source: string): readonly Inline[] {
  const nodes: Inline[] = [];
  let index = 0;
  while (index < source.length) {
    if (source[index] === "\\" && index + 1 < source.length && /[!"#$%&'()*+,\-./:;<=>?@[\\\]^_\x60{|}~]/u.test(source[index + 1]!)) {
      appendText(nodes, source[index + 1]!);
      index += 2;
      continue;
    }
    if (source[index] === "\x60") {
      let width = 1;
      while (source[index + width] === "\x60") width += 1;
      const delimiter = "\x60".repeat(width);
      const end = source.indexOf(delimiter, index + width);
      if (end >= 0) {
        nodes.push({ kind: "code", value: codeSpan(source.slice(index + width, end)) });
        index = end + width;
        continue;
      }
    }
    const strong = source.startsWith("**", index) ? "**" : source.startsWith("__", index) ? "__" : undefined;
    if (strong !== undefined) {
      const end = unescaped(source, strong, index + 2);
      if (end > index + 2) {
        nodes.push({ kind: "strong", children: parseInline(source.slice(index + 2, end)) });
        index = end + 2;
        continue;
      }
      appendText(nodes, strong);
      index += 2;
      continue;
    }
    const emphasis = source[index] === "*" || source[index] === "_" ? source[index]! : undefined;
    if (emphasis !== undefined) {
      const end = unescaped(source, emphasis, index + 1);
      if (end > index + 1) {
        nodes.push({ kind: "emphasis", children: parseInline(source.slice(index + 1, end)) });
        index = end + 1;
        continue;
      }
    }
    if (source[index] === "[") {
      const labelEnd = unescaped(source, "]", index + 1);
      if (labelEnd >= 0 && source[labelEnd + 1] === "(") {
        const parsed = destination(source, labelEnd + 2);
        if (parsed !== undefined) {
          const children = parseInline(source.slice(index + 1, labelEnd));
          if (safeHref(parsed.href)) nodes.push({ kind: "link", href: parsed.href, title: parsed.title, children });
          else nodes.push(...children);
          index = parsed.end;
          continue;
        }
      }
    }
    if (source[index] === "<") {
      const match = /^<(https?:\/\/[^ >]+|mailto:[^ >]+)>/iu.exec(source.slice(index));
      if (match !== null && safeHref(match[1]!)) {
        nodes.push({ kind: "link", href: match[1]!, children: [{ kind: "text", value: match[1]!.replace(/^mailto:/iu, "") }] });
        index += match[0].length;
        continue;
      }
    }
    if ((index === 0 || /[\s(]/u.test(source[index - 1]!)) && /^https?:\/\//iu.test(source.slice(index))) {
      const raw = /^[^\s<>]+/u.exec(source.slice(index))?.[0] ?? "";
      const href = raw.replace(/[.,]$/u, "").replace(/\)$/u, "");
      if (href.length > 0 && safeHref(href)) {
        nodes.push({ kind: "link", href, children: [{ kind: "text", value: href }] });
        index += href.length;
        continue;
      }
    }
    if (source.startsWith("  \n", index)) {
      nodes.push({ kind: "break" });
      index += 3;
      continue;
    }
    appendText(nodes, source[index]!);
    index += 1;
  }
  return nodes;
}
// #endregion 🧩️InlineParser

// #region 🧱️BlockParser
function marker(line: string): Marker | undefined {
  const match = /^(\s*)([-+*]|(\d+)[.)])\s+(.*)$/u.exec(line);
  if (match === null) return undefined;
  return { indent: match[1]!.replace(/\t/gu, "    ").length, ordered: match[3] !== undefined, start: Number(match[3] ?? 1), content: match[4]! };
}
function splitRow(line: string): readonly string[] {
  let source = line.trim();
  if (source.startsWith("|")) source = source.slice(1);
  if (source.endsWith("|") && !source.endsWith("\\|")) source = source.slice(0, -1);
  const cells: string[] = [];
  let cell = "";
  let escaped = false;
  let code = 0;
  for (let index = 0; index < source.length; index += 1) {
    const character = source[index]!;
    if (escaped) {
      cell += "\\" + character;
      escaped = false;
    } else if (character === "\\") {
      escaped = true;
    } else if (character === "\x60") {
      let width = 1;
      while (source[index + width] === "\x60") width += 1;
      code = code === width ? 0 : code === 0 ? width : code;
      cell += "\x60".repeat(width);
      index += width - 1;
    } else if (character === "|" && code === 0) {
      cells.push(cell.trim());
      cell = "";
    } else {
      cell += character;
    }
  }
  cells.push(cell.trim());
  return cells;
}
function delimiter(line: string): readonly Alignment[] | undefined {
  const cells = splitRow(line);
  if (cells.length === 0 || cells.some((cell) => !/^:?-+:?$/u.test(cell))) return undefined;
  return cells.map((cell) => (cell.startsWith(":") && cell.endsWith(":") ? "center" : cell.startsWith(":") ? "left" : cell.endsWith(":") ? "right" : undefined));
}
function htmlBlock(line: string): boolean {
  return /^\s*<\/?[A-Za-z][^>]*>/u.test(line);
}
function startsBlock(lines: readonly string[], index: number): boolean {
  const line = lines[index] ?? "";
  if (line.trim().length === 0 || /^ {0,3}(\x60{3,}|~{3,})/u.test(line) || htmlBlock(line) || /^ {0,3}#{1,6}\s+/u.test(line) || marker(line) !== undefined) return true;
  return delimiter(lines[index + 1] ?? "") !== undefined && splitRow(line).length === splitRow(lines[index + 1] ?? "").length;
}
function indent(line: string): number {
  return /^\s*/u.exec(line)?.[0].replace(/\t/gu, "    ").length ?? 0;
}
function parseList(lines: readonly string[], start: number): { readonly block: Block; readonly next: number } {
  const first = marker(lines[start]!)!;
  const items: ListItem[] = [];
  let index = start;
  while (index < lines.length) {
    const current = marker(lines[index]!);
    if (current === undefined || current.indent !== first.indent || current.ordered !== first.ordered) break;
    index += 1;
    const paragraph = [current.content];
    const blocks: Block[] = [];
    while (index < lines.length) {
      const nested = marker(lines[index]!);
      if (nested !== undefined && nested.indent > first.indent) {
        const parsed = parseList(lines, index);
        blocks.push({ kind: "paragraph", children: parseInline(paragraph.join("\n")) }, parsed.block);
        index = parsed.next;
        break;
      }
      if (nested !== undefined || lines[index]!.trim().length === 0 || indent(lines[index]!) <= first.indent) break;
      paragraph.push(lines[index]!.trim());
      index += 1;
    }
    if (blocks.length === 0) blocks.push({ kind: "paragraph", children: parseInline(paragraph.join("\n")) });
    items.push({ blocks });
    while (lines[index]?.trim().length === 0 && marker(lines[index + 1] ?? "")?.indent === first.indent) index += 1;
  }
  return { block: { kind: "list", ordered: first.ordered, start: first.start, items }, next: index };
}
function parseMarkdown(markdown: string): Document {
  const lines = markdown.replace(/\r\n?/gu, "\n").split("\n");
  const blocks: Block[] = [];
  let index = 0;
  while (index < lines.length) {
    const line = lines[index]!;
    if (line.trim().length === 0) {
      index += 1;
      continue;
    }
    const fence = /^ {0,3}(\x60{3,}|~{3,})\s*([^\s]*)?.*$/u.exec(line);
    if (fence !== null) {
      const mark = fence[1]!;
      const content: string[] = [];
      index += 1;
      const closing = new RegExp("^ {0,3}" + mark[0] + "{" + mark.length + ",}\\s*$", "u");
      while (index < lines.length && !closing.test(lines[index]!)) {
        content.push(lines[index]!);
        index += 1;
      }
      if (index < lines.length) index += 1;
      blocks.push({ kind: "code", language: fence[2] || undefined, value: content.length > 0 ? content.join("\n") + "\n" : "" });
      continue;
    }
    if (htmlBlock(line)) {
      index += 1;
      continue;
    }
    const heading = /^ {0,3}(#{1,6})\s+(.+?)\s*#*\s*$/u.exec(line);
    if (heading !== null) {
      blocks.push({ kind: "heading", depth: heading[1]!.length, children: parseInline(heading[2]!) });
      index += 1;
      continue;
    }
    if (marker(line) !== undefined) {
      const parsed = parseList(lines, index);
      blocks.push(parsed.block);
      index = parsed.next;
      continue;
    }
    const alignments = delimiter(lines[index + 1] ?? "");
    const header = alignments === undefined ? [] : splitRow(line);
    if (alignments !== undefined && header.length === alignments.length) {
      index += 2;
      const rows: (readonly (readonly Inline[])[])[] = [];
      while (index < lines.length && lines[index]!.trim().length > 0 && splitRow(lines[index]!).length > 1 && !startsBlock(lines, index)) {
        const cells = [...splitRow(lines[index]!)];
        while (cells.length < alignments.length) cells.push("");
        rows.push(cells.slice(0, alignments.length).map(parseInline));
        index += 1;
      }
      blocks.push({ kind: "table", alignments, header: header.map(parseInline), rows });
      continue;
    }
    const paragraph = [line];
    index += 1;
    while (index < lines.length && lines[index]!.trim().length > 0 && !startsBlock(lines, index)) {
      paragraph.push(lines[index]!);
      index += 1;
    }
    blocks.push({ kind: "paragraph", children: parseInline(paragraph.join("\n")) });
  }
  return { kind: "document", blocks };
}
// #endregion 🧱️BlockParser

// #region 🧾️Serializer
function text(value: string): string {
  return value.replace(/&/gu, "&#x26;").replace(/</gu, "&#x3C;");
}
function attribute(value: string): string {
  return text(value).replace(/"/gu, "&#x22;").replace(/'/gu, "&#x27;");
}
function inline(nodes: readonly Inline[]): string {
  return nodes
    .map((node) => {
      if (node.kind === "text") return text(node.value);
      if (node.kind === "emphasis") return "<em>" + inline(node.children) + "</em>";
      if (node.kind === "strong") return "<strong>" + inline(node.children) + "</strong>";
      if (node.kind === "code") return "<code>" + text(node.value) + "</code>";
      if (node.kind === "break") return "<br>\n";
      const title = node.title === undefined ? "" : ' title="' + attribute(node.title) + '"';
      return '<a href="' + attribute(node.href) + '"' + title + ">" + inline(node.children) + "</a>";
    })
    .join("");
}
function align(value: Alignment): string {
  return value === undefined ? "" : ' align="' + value + '"';
}
function listItem(item: ListItem): string {
  const [first, ...rest] = item.blocks;
  if (first?.kind === "paragraph") return "<li>" + inline(first.children) + (rest.length === 0 ? "" : "\n" + rest.map(block).join("\n") + "\n") + "</li>";
  return "<li>" + item.blocks.map(block).join("\n") + "</li>";
}
function block(value: Block): string {
  if (value.kind === "paragraph") return "<p>" + inline(value.children) + "</p>";
  if (value.kind === "heading") return "<h" + value.depth + ">" + inline(value.children) + "</h" + value.depth + ">";
  if (value.kind === "code") return "<pre><code" + (value.language === undefined ? "" : ' class="language-' + attribute(value.language) + '"') + ">" + text(value.value) + "</code></pre>";
  if (value.kind === "list") {
    const tag = value.ordered ? "ol" : "ul";
    const start = value.ordered && value.start !== 1 ? ' start="' + value.start + '"' : "";
    return "<" + tag + start + ">\n" + value.items.map(listItem).join("\n") + "\n</" + tag + ">";
  }
  const header = value.header.map((cell, index) => "<th" + align(value.alignments[index]) + ">" + inline(cell) + "</th>").join("\n");
  const rows = value.rows.map((row) => "<tr>\n" + row.map((cell, index) => "<td" + align(value.alignments[index]) + ">" + inline(cell) + "</td>").join("\n") + "\n</tr>").join("\n");
  return "<table>\n<thead>\n<tr>\n" + header + "\n</tr>\n</thead>" + (rows.length === 0 ? "" : "\n<tbody>\n" + rows + "\n</tbody>") + "\n</table>";
}
// #endregion 🧾️Serializer

// #region 🔌️Compiler
/** @emoji 📝️ Compiles the owned presentation CommonMark/GFM subset to a safe HTML fragment. */
export async function compileOwnedMarkdownToHtml(markdown: string): Promise<string> {
  return parseMarkdown(markdown).blocks.map(block).join("\n");
}
// #endregion 🔌️Compiler

// #region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const tick = "\x60";
  const fixtures = [
    {
      name: "prose",
      markdown: "# Heading\n\nA *small* and **strong** paragraph with " + tick + "x < y" + tick + " and [docs](https://example.com/a?q=1&b=2).",
      html: '<h1>Heading</h1>\n<p>A <em>small</em> and <strong>strong</strong> paragraph with <code>x &#x3C; y</code> and <a href="https://example.com/a?q=1&#x26;b=2">docs</a>.</p>',
    },
    { name: "fenced code", markdown: tick.repeat(3) + "ts\nconst x = " + tick + "<tag>" + tick + ";\n" + tick.repeat(3), html: '<pre><code class="language-ts">const x = ' + tick + "&#x3C;tag>" + tick + ";\n</code></pre>" },
    {
      name: "lists",
      markdown: "- alpha\n- beta\n  - nested\n\n3. third\n4. fourth",
      html: '<ul>\n<li>alpha</li>\n<li>beta\n<ul>\n<li>nested</li>\n</ul>\n</li>\n</ul>\n<ol start="3">\n<li>third</li>\n<li>fourth</li>\n</ol>',
    },
    {
      name: "table",
      markdown: "| Left | Center | Right |\n| :--- | :----: | ----: |\n| a & b | **c** | " + tick + "d" + tick + " |",
      html: '<table>\n<thead>\n<tr>\n<th align="left">Left</th>\n<th align="center">Center</th>\n<th align="right">Right</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td align="left">a &#x26; b</td>\n<td align="center"><strong>c</strong></td>\n<td align="right"><code>d</code></td>\n</tr>\n</tbody>\n</table>',
    },
  ] as const;
  describe("owned markdown html compiler", () => {
    for (const fixture of fixtures) {
      it("matches the installed compiler for " + fixture.name, async () => {
        expect(await compileOwnedMarkdownToHtml(fixture.markdown)).toBe(fixture.html);
      });
    }
    it("escapes text and drops raw HTML blocks", async () => {
      expect(await compileOwnedMarkdownToHtml("<script>alert(1)</script>\n\nA < B & C > D")).toBe("<p>A &#x3C; B &#x26; C > D</p>");
    });
    it("handles malformed input deterministically", async () => {
      const markdown = "**open *nested\n\n[missing](https://example.com\n\n" + tick.repeat(3) + "js\nunterminated";
      expect(await compileOwnedMarkdownToHtml(markdown)).toBe('<p>**open *nested</p>\n<p>[missing](<a href="https://example.com">https://example.com</a></p>\n<pre><code class="language-js">unterminated\n</code></pre>');
    });
    it("rejects executable and opaque URL schemes", async () => {
      expect(await compileOwnedMarkdownToHtml("[js](javascript:alert(1)) [data](data:text/html,x) [mail](mailto:a@example.com) [relative](/deck?q=1&x=2)")).toBe(
        '<p>js data <a href="mailto:a@example.com">mail</a> <a href="/deck?q=1&#x26;x=2">relative</a></p>',
      );
    });
  });
}
// #endregion 🧪️Tests
