import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";
import Ajv from "ajv/dist/2020";
const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
const segmenter = new Intl.Segmenter("und", { granularity: "grapheme" });
const size = (value: string) => Buffer.byteLength(value);
const boundaries = (value: string) => [0, ...Array.from(segmenter.segment(value), part => size(value.slice(0, part.index + part.segment.length)))];
const width = (value: string) => Array.from(segmenter.segment(value)).reduce((cells, { segment }) => segment === "\t" ? cells + 4 - cells % 4 : cells + (/^[\p{Mark}\p{Control}]/u.test(segment) ? 0 : /[中\p{Extended_Pictographic}]/u.test(segment) ? 2 : 1), 0);
const slice = (value: string, start: number, end?: number) => Buffer.from(value).subarray(start, end).toString();

test("multiline editor vectors agree with Ajv, Unicode segmentation and independent SQLite mutations", () => {
  const validate = new Ajv({ strict: false }).compile(schema); expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  const db = new Database(":memory:");
  try {
    for (const row of fixture.cases) {
      let value = row.initial, cursor = row.cursor, anchor: number | null = null;
      const undo: Array<[string, number]> = [], redo: Array<[string, number]> = [], newline = value.includes("\r\n") ? "\r\n" : "\n";
      const replace = (from: number, to: number, inserted: string) => {
        if (size(value) - (to - from) + size(inserted) > (row.limit ?? fixture.maximumBytes)) return;
        undo.push([value, cursor]); redo.length = 0;
        const before = Array.from(slice(value, 0, from)).length, after = Array.from(slice(value, 0, to)).length;
        value = (db.query("SELECT substr(?1,1,?2) || ?3 || substr(?1,?4) value").get(value, before, inserted, after + 1) as { value: string }).value;
        cursor = from + size(inserted); anchor = null;
      };
      for (const command of row.commands) {
        if (row.readOnly && ["insert", "delete", "backspace", "enter", "undo", "redo"].includes(command.op)) continue;
        const text = slice(value, 0, cursor), start = size(text.slice(0, text.lastIndexOf("\n") + 1)), remaining = slice(value, cursor), end = cursor + size(remaining.includes("\n") ? remaining.slice(0, remaining.indexOf("\n")) : remaining), points = boundaries(value);
        if (command.op === "insert" || command.op === "enter") {
          const input = command.op === "enter" ? "\n" : command.text;
          const clean = input.replaceAll("\r\n", "\n").replaceAll("\r", "\n").replace(/[\p{Control}]/gu, (c: string) => c === "\n" || c === "\t" ? c : "").replaceAll("\n", newline);
          replace(Math.min(cursor, anchor ?? cursor), Math.max(cursor, anchor ?? cursor), clean);
        } else if (command.op === "selectall") { anchor = 0; cursor = size(value); }
        else if (command.op === "left") cursor = points.filter(point => point < cursor).at(-1) ?? 0;
        else if (command.op === "right") cursor = points.find(point => point > cursor) ?? size(value);
        else if (command.op === "home") cursor = start;
        else if (command.op === "end") cursor = end - Number(slice(value, start, end).endsWith("\r"));
        else if (command.op === "backspace") replace(points.filter(point => point < cursor).at(-1) ?? 0, cursor, "");
        else if (command.op === "delete") replace(cursor, points.find(point => point > cursor) ?? size(value), "");
        else if (command.op === "undo" || command.op === "redo") { const source = command.op === "undo" ? undo : redo, destination = command.op === "undo" ? redo : undo, old = source.pop(); if (old) { destination.push([value, cursor]); [value, cursor] = old; } }
        else {
          const lines = value.split("\n"), rowIndex = text.split("\n").length - 1, target = rowIndex + (command.op === "up" ? -1 : 1);
          if (target < 0 || target >= lines.length) continue;
          const wanted = width(slice(value, start, cursor)), line = lines[target].replace(/\r$/u, ""); let offset = 0;
          for (const part of segmenter.segment(line)) { if (width(line.slice(0, part.index + part.segment.length)) > wanted) break; offset = size(line.slice(0, part.index + part.segment.length)); }
          cursor = size(lines.slice(0, target).join("\n")) + Number(target > 0) + offset;
        }
      }
      expect(value, row.name).toBe(row.expected); expect(cursor, row.name).toBe(row.expectedCursor);
      expect((db.query("SELECT length(cast(? AS blob)) bytes").get(value) as { bytes: number }).bytes).toBe(size(value));
    }
  } finally { db.close(); }
});
