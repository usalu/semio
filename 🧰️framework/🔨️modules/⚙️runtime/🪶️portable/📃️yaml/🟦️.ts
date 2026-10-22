import { runtimeMessageV1 } from "../../🗣️messages/🟦️.ts";
interface LineV1 { readonly indent: number; readonly text: string; readonly raw: string; readonly number: number }

const NULLS = new Set(["", "~", "null", "Null", "NULL"]);
const TRUES = new Set(["true", "True", "TRUE"]);
const FALSES = new Set(["false", "False", "FALSE"]);

function fail(line: LineV1 | undefined, id: string, parameters: Readonly<Record<string, string | number>> = {}): never { throw new SyntaxError(runtimeMessageV1("syntax", { format: "YAML", detail: id, position: line === undefined ? "-" : String(line.number), ...parameters })); }

function stripComment(text: string): string {
  let quote = "";
  for (let index = 0; index < text.length; index++) {
    const character = text[index]!;
    if (quote !== "") { if (character === "\\" && quote === '"') index++; else if (character === quote) { if (quote === "'" && text[index + 1] === "'") index++; else quote = ""; } continue; }
    if ((character === '"' || character === "'") && (index === 0 || /[\s[{,:-]/u.test(text[index - 1]!))) { quote = character; continue; }
    if (character === "#" && (index === 0 || /\s/u.test(text[index - 1]!))) return text.slice(0, index).trimEnd();
  }
  return text.trimEnd();
}

function plainScalar(text: string): unknown {
  if (NULLS.has(text)) return null;
  if (TRUES.has(text)) return true;
  if (FALSES.has(text)) return false;
  if (/^[-+]?[0-9]+$/u.test(text)) return Number(text);
  if (/^0o[0-7]+$/u.test(text)) return Number.parseInt(text.slice(2), 8);
  if (/^0x[0-9a-fA-F]+$/u.test(text)) return Number.parseInt(text.slice(2), 16);
  if (/^[-+]?(?:\.[0-9]+|[0-9]+(?:\.[0-9]*)?)(?:[eE][-+]?[0-9]+)?$/u.test(text)) return Number(text);
  if (/^[-+]?\.(?:inf|Inf|INF)$/u.test(text)) return text.startsWith("-") ? Number.NEGATIVE_INFINITY : Number.POSITIVE_INFINITY;
  if (/^\.(?:nan|NaN|NAN)$/u.test(text)) return Number.NaN;
  return text;
}

function unquoteDouble(text: string, line: LineV1 | undefined): string {
  let output = "";
  for (let index = 1; index < text.length - 1; index++) {
    const character = text[index]!;
    if (character !== "\\") { output += character; continue; }
    const code = text[++index]!;
    const simple: Record<string, string> = { n: "\n", t: "\t", r: "\r", "0": "\0", '"': '"', "\\": "\\", "/": "/", b: "\b", f: "\f", " ": " ", e: "\u001b", a: "\u0007", v: "\v", N: "\u0085", _: " " };
    if (code in simple) output += simple[code]!;
    else if (code === "x" || code === "u" || code === "U") { const width = code === "x" ? 2 : code === "u" ? 4 : 8; output += String.fromCodePoint(Number.parseInt(text.slice(index + 1, index + 1 + width), 16)); index += width; }
    else fail(line, "yaml.invalidEscape");
  }
  return output;
}

class FlowReaderV1 {
  private index = 0;
  private readonly text: string;
  private readonly line: LineV1 | undefined;
  constructor(text: string, line: LineV1 | undefined) { this.text = text; this.line = line; }
  private blank(): void { while (/\s/u.test(this.text[this.index] ?? "x")) this.index++; }
  parse(): unknown { const value = this.value(); this.blank(); if (this.index < this.text.length) fail(this.line, "yaml.unexpectedFlow"); return value; }
  private value(): unknown {
    this.blank();
    const character = this.text[this.index];
    if (character === "[") return this.sequence();
    if (character === "{") return this.mapping();
    if (character === '"' || character === "'") return this.quoted();
    const start = this.index;
    while (this.index < this.text.length && !/[,\]}]/u.test(this.text[this.index]!) && !(this.text[this.index] === ":" && /[\s,\]}]/u.test(this.text[this.index + 1] ?? " "))) this.index++;
    return plainScalar(this.text.slice(start, this.index).trim());
  }
  private quoted(): string {
    const quote = this.text[this.index]!, start = this.index++;
    while (this.index < this.text.length) {
      const character = this.text[this.index]!;
      if (quote === '"' && character === "\\") { this.index += 2; continue; }
      if (character === quote) { if (quote === "'" && this.text[this.index + 1] === "'") { this.index += 2; continue; } this.index++; break; }
      this.index++;
    }
    const raw = this.text.slice(start, this.index);
    return quote === '"' ? unquoteDouble(raw, this.line) : raw.slice(1, -1).replaceAll("''", "'");
  }
  private sequence(): unknown[] {
    this.index++;
    const items: unknown[] = [];
    for (;;) {
      this.blank();
      if (this.text[this.index] === "]") { this.index++; return items; }
      items.push(this.value()); this.blank();
      if (this.text[this.index] === ",") this.index++; else if (this.text[this.index] !== "]") fail(this.line, "yaml.expectedFlowSequence");
    }
  }
  private mapping(): Record<string, unknown> {
    this.index++;
    const result: Record<string, unknown> = {};
    for (;;) {
      this.blank();
      if (this.text[this.index] === "}") { this.index++; return result; }
      const key = String(this.value()); this.blank();
      if (this.text[this.index] === ":") { this.index++; Object.defineProperty(result, key, { value: this.value(), enumerable: true, writable: true, configurable: true }); }
      else Object.defineProperty(result, key, { value: null, enumerable: true, writable: true, configurable: true });
      this.blank();
      if (this.text[this.index] === ",") this.index++; else if (this.text[this.index] !== "}") fail(this.line, "yaml.expectedFlowMapping");
    }
  }
}

class YamlParserV1 {
  private readonly lines: LineV1[];
  private position = 0;
  private readonly anchors = new Map<string, unknown>();
  constructor(text: string) {
    this.lines = [];
    const rows = text.replace(/^﻿/u, "").split(/\r?\n/u);
    rows.forEach((raw, index) => { this.lines.push({ indent: raw.length - raw.trimStart().length, text: raw.trim(), raw, number: index + 1 }); });
  }
  private skipBlank(): void { while (this.position < this.lines.length && (this.lines[this.position]!.text === "" || this.lines[this.position]!.text.startsWith("#"))) this.position++; }
  parse(): unknown {
    this.skipBlank();
    if (this.lines[this.position]?.text === "---") this.position++;
    else if (this.lines[this.position]?.text.startsWith("--- ")) { this.lines[this.position] = { ...this.lines[this.position]!, indent: 0, text: this.lines[this.position]!.text.slice(4).trim(), raw: this.lines[this.position]!.raw.slice(4) }; }
    this.skipBlank();
    if (this.position >= this.lines.length || this.lines[this.position]!.text === "...") return null;
    const value = this.block(this.lines[this.position]!.indent);
    this.skipBlank();
    if (this.position < this.lines.length && this.lines[this.position]!.text !== "..." && this.lines[this.position]!.text !== "---") fail(this.lines[this.position], "yaml.unexpectedContent");
    return value;
  }
  private block(indent: number): unknown {
    this.skipBlank();
    const line = this.lines[this.position];
    if (line === undefined || line.indent < indent) return null;
    const body = stripComment(line.text);
    if (body === "-" || body.startsWith("- ")) return this.sequence(line.indent);
    if (this.findKey(body) >= 0) return this.mapping(line.indent);
    this.position++;
    return this.inline(body, line, line.indent);
  }
  private findKey(body: string): number {
    if (body.startsWith('"') || body.startsWith("'")) {
      const quote = body[0]!;
      for (let index = 1; index < body.length; index++) { if (quote === '"' && body[index] === "\\") { index++; continue; } if (body[index] === quote) { if (quote === "'" && body[index + 1] === "'") { index++; continue; } return /^\s*:(\s|$)/u.test(body.slice(index + 1)) ? body.indexOf(":", index + 1) : -1; } }
      return -1;
    }
    if (body.startsWith("[") || body.startsWith("{")) return -1;
    const match = /:(?:\s|$)/u.exec(body);
    return match === null ? -1 : match.index;
  }
  private sequence(indent: number): unknown[] {
    const items: unknown[] = [];
    for (;;) {
      this.skipBlank();
      const line = this.lines[this.position];
      if (line === undefined || line.indent !== indent) { if (line !== undefined && line.indent > indent) fail(line, "yaml.badIndentation"); return items; }
      const body = stripComment(line.text);
      if (body !== "-" && !body.startsWith("- ")) return items;
      const rest = body.slice(1).trimStart(), offset = line.indent + (line.text.length - line.text.slice(1).trimStart().length);
      if (rest === "") { this.position++; items.push(this.nested(indent)); continue; }
      const nested: LineV1 = { indent: offset, text: rest, raw: " ".repeat(offset) + rest, number: line.number };
      this.lines[this.position] = nested;
      items.push(this.block(offset));
    }
  }
  private nested(parent: number): unknown {
    this.skipBlank();
    const next = this.lines[this.position];
    if (next === undefined || next.indent <= parent) return null;
    return this.block(next.indent);
  }
  private mapping(indent: number): Record<string, unknown> {
    const result: Record<string, unknown> = {};
    for (;;) {
      this.skipBlank();
      const line = this.lines[this.position];
      if (line === undefined || line.indent !== indent) { if (line !== undefined && line.indent > indent) fail(line, "yaml.badIndentation"); return result; }
      const body = stripComment(line.text), colon = this.findKey(body);
      if (colon < 0 || body === "---" || body === "...") return result;
      const rawKey = body.slice(0, colon).trim(), key = rawKey.startsWith('"') ? unquoteDouble(rawKey, line) : rawKey.startsWith("'") ? rawKey.slice(1, -1).replaceAll("''", "'") : String(plainScalar(rawKey));
      const rest = body.slice(colon + 1).trim();
      if (Object.hasOwn(result, key)) fail(line, "yaml.duplicateKey", { key });
      this.position++;
      let value: unknown;
      if (rest === "") {
        this.skipBlank();
        const next = this.lines[this.position];
        value = next !== undefined && next.indent === indent && /^-(\s|$)/u.test(stripComment(next.text)) ? this.sequence(indent) : this.nested(indent);
      } else value = this.inline(rest, line, indent);
      Object.defineProperty(result, key, { value, enumerable: true, writable: true, configurable: true });
    }
  }
  private inline(rest: string, line: LineV1, indent: number): unknown {
    if (/^[|>][+-]?[0-9]?[+-]?$/u.test(rest)) return this.blockScalar(rest, indent);
    if (rest.startsWith("!")) fail(line, "yaml.tags");
    const anchored = /^&(\S+)(?:\s+(.*))?$/u.exec(rest);
    if (anchored !== null) { const value = (anchored[2] ?? "") === "" ? this.nested(indent) : this.inline(anchored[2]!, line, indent); this.anchors.set(anchored[1]!, value); return value; }
    if (rest.startsWith("*")) { const name = rest.slice(1).trim(); if (!this.anchors.has(name)) fail(line, "yaml.unknownAlias", { name }); return structuredClone(this.anchors.get(name)); }
    if (rest.startsWith("[") || rest.startsWith("{")) {
      let text = rest;
      while (!this.balanced(text) && this.position < this.lines.length) { text += " " + stripComment(this.lines[this.position]!.text); this.position++; }
      return new FlowReaderV1(text, line).parse();
    }
    if (rest.startsWith('"') || rest.startsWith("'")) {
      let text = rest;
      const closed = (value: string): boolean => { const quote = value[0]!; for (let index = 1; index < value.length; index++) { if (quote === '"' && value[index] === "\\") { index++; continue; } if (value[index] === quote) { if (quote === "'" && value[index + 1] === "'") { index++; continue; } return true; } } return false; };
      while (!closed(text) && this.position < this.lines.length) { const next = this.lines[this.position]!; text += next.text === "" ? "\n" : (text.endsWith("\n") ? "" : " ") + next.text; this.position++; }
      return text[0] === '"' ? unquoteDouble(text, line) : text.slice(1, -1).replaceAll("''", "'");
    }
    let text = rest;
    for (;;) {
      const next = this.lines[this.position];
      if (next === undefined || next.text === "" || next.indent <= indent || next.text.startsWith("#") || this.findKey(stripComment(next.text)) >= 0 && next.indent <= indent) break;
      text += " " + stripComment(next.text); this.position++;
    }
    return plainScalar(text);
  }
  private balanced(text: string): boolean {
    let depth = 0, quote = "";
    for (let index = 0; index < text.length; index++) {
      const character = text[index]!;
      if (quote !== "") { if (character === "\\" && quote === '"') index++; else if (character === quote) quote = ""; continue; }
      if (character === '"' || character === "'") quote = character; else if (character === "[" || character === "{") depth++; else if (character === "]" || character === "}") depth--;
    }
    return depth <= 0;
  }
  private blockScalar(header: string, indent: number): string {
    const folded = header[0] === ">", chomp = header.includes("-") ? "strip" : header.includes("+") ? "keep" : "clip";
    const rows: LineV1[] = [];
    let blockIndent = -1;
    while (this.position < this.lines.length) {
      const line = this.lines[this.position]!;
      if (line.text !== "" && line.indent <= indent) break;
      if (line.text !== "" && blockIndent < 0) blockIndent = line.indent;
      rows.push(line); this.position++;
    }
    const content = rows.map(row => (row.text === "" ? "" : row.raw.slice(Math.min(blockIndent < 0 ? 0 : blockIndent, row.raw.length))));
    while (content.length > 0 && content.at(-1) === "" && chomp !== "keep") content.pop();
    let text = "";
    if (!folded) text = content.join("\n");
    else for (let index = 0; index < content.length; index++) { const row = content[index]!; text += row; if (index < content.length - 1) text += row === "" || content[index + 1] === "" || /^\s/u.test(row) || /^\s/u.test(content[index + 1]!) ? "\n" : " "; }
    return chomp === "strip" ? text : text === "" ? "" : `${text}\n`;
  }
}

/** 📃️ Parses one YAML 1.2 document of block and flow collections with plain, quoted and block scalars; the portable counterpart of a runtime's native parser, with `yaml` as test oracle. Anchors and aliases are resolved without merge keys; tags and complex keys are refused. */
export function parseYamlV1(text: string): any { return new YamlParserV1(text).parse(); }
