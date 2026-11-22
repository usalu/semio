/** ✍️ Canonical textFieldEditingSelfTests: the TypeScript half of the text-field editing law, answering the very same
 * `🧪️tests/🧫️fixtures/🧫️text-field-editing/🔣️.json` rows the Rust `🔬️targets-wgpu-events-unit` law answers through
 * `route_edit_key`. The third-party oracle is `Intl.Segmenter`: every word motion the corpus declares must land on the
 * word boundaries the platform's own UAX #29 word segmentation reports, and a small independent model replays every key row. */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";

type Expected = { readonly text: string; readonly caret: number; readonly anchor: number; readonly clipboard?: Readonly<Record<string, string>> };
type KeyRow = { readonly case: string; readonly kind: "text" | "longText"; readonly text: string; readonly caret: number; readonly anchor: number; readonly key: string; readonly shift: boolean; readonly ctrl: boolean; readonly alt: boolean; readonly meta: boolean; readonly expected: Expected };
type PasteRow = { readonly case: string; readonly kind: "text" | "longText"; readonly text: string; readonly anchor: number; readonly caret: number; readonly pasted: string; readonly expected: { readonly text: string; readonly caret: number } };
type Fixture = { readonly keys: readonly KeyRow[]; readonly pastes: readonly PasteRow[] };

const read = (path: string): unknown => JSON.parse(readFileSync(fileURLToPath(new URL(path, import.meta.url)), "utf8"));
const segmenter = new Intl.Segmenter("und", { granularity: "word" });
const bytes = (value: string): number => Buffer.byteLength(value);
const cut = (value: string, byte: number): number => Buffer.from(value).subarray(0, byte).toString().length;
const marks = (value: string): { readonly start: number; readonly end: number; readonly blank: boolean }[] =>
  Array.from(segmenter.segment(value), (part) => ({ start: bytes(value.slice(0, part.index)), end: bytes(value.slice(0, part.index + part.segment.length)), blank: /^\s+$/u.test(part.segment) }));
const previousStart = (value: string, at: number): number => marks(value).filter((mark) => !mark.blank && mark.start < at).at(-1)?.start ?? 0;
const nextEnd = (value: string, at: number): number => marks(value).find((mark) => !mark.blank && mark.end > at)?.end ?? bytes(value);
const nextStart = (value: string, at: number): number => {
  const all = marks(value), current = all.find((mark) => mark.end > at && !mark.blank);
  return (current ? all.find((mark) => mark.start >= current.end && !mark.blank)?.start : undefined) ?? bytes(value);
};
const lineStart = (value: string, at: number): number => bytes(value.slice(0, cut(value, at)).replace(/[^\n]*$/u, ""));
const lineEnd = (value: string, at: number): number => at + bytes(value.slice(cut(value, at)).replace(/\n[\s\S]*$/u, ""));
const column = (value: string, at: number): number => Array.from(value.slice(0, cut(value, at)).replace(/^[\s\S]*\n/u, "")).length;
const vertical = (value: string, at: number, down: boolean): number | null => {
  const lines = value.split("\n"), row = value.slice(0, cut(value, at)).split("\n").length - 1, target = row + (down ? 1 : -1);
  if (target < 0 || target >= lines.length) return null;
  const offset = bytes(Array.from(lines[target]!).slice(0, column(value, at)).join(""));
  return bytes(lines.slice(0, target).join("\n")) + 1 * Number(target > 0) + offset;
};
const splice = (value: string, from: number, to: number, inserted: string): string => value.slice(0, cut(value, from)) + inserted + value.slice(cut(value, to));
const chars = (value: string, at: number, step: 1 | -1): number => bytes(Array.from(value.slice(0, cut(value, at))).slice(0, step === 1 ? undefined : -1).join("")) + (step === 1 ? bytes(Array.from(value.slice(cut(value, at)))[0] ?? "") : 0);

/** 🧮️ The independent model: the state after `row.key` on `row.text`, plus the clipboard it fills. */
function model(row: KeyRow): Expected {
  const { text, key, shift } = row, word = row.ctrl || row.alt;
  const low = Math.min(row.anchor, row.caret), high = Math.max(row.anchor, row.caret), selected = low !== high;
  const move = (caret: number): Expected => ({ text, caret, anchor: shift ? row.anchor : caret });
  const remove = (from: number, to: number): Expected => ({ text: splice(text, from, to, ""), caret: from, anchor: from });
  if (key === "ArrowLeft" || key === "ArrowRight") {
    const forward = key === "ArrowRight";
    if (row.meta) return move(forward ? lineEnd(text, row.caret) : lineStart(text, row.caret));
    if (row.alt) return move(forward ? nextEnd(text, row.caret) : previousStart(text, row.caret));
    if (row.ctrl) return move(forward ? nextStart(text, row.caret) : previousStart(text, row.caret));
    if (selected && !shift) return move(forward ? high : low);
    return move(forward ? chars(text, row.caret, 1) : chars(text, row.caret, -1));
  }
  if (key === "ArrowUp" || key === "ArrowDown") {
    const target = row.meta ? (key === "ArrowDown" ? bytes(text) : 0) : vertical(text, row.caret, key === "ArrowDown");
    return target === null ? { text, caret: row.caret, anchor: row.anchor } : move(target);
  }
  if (key === "Home" || key === "End") return move(row.ctrl ? (key === "End" ? bytes(text) : 0) : key === "End" ? lineEnd(text, row.caret) : lineStart(text, row.caret));
  if (key === "Backspace") {
    if (selected) return remove(low, high);
    return remove(row.meta ? Math.min(lineStart(text, row.caret), chars(text, row.caret, -1)) : word ? previousStart(text, row.caret) : chars(text, row.caret, -1), row.caret);
  }
  if (key === "Delete") return selected ? remove(low, high) : remove(row.caret, word ? Math.max(nextEnd(text, row.caret), chars(text, row.caret, 1)) : chars(text, row.caret, 1));
  if (key.toLowerCase() === "a") return { text, caret: bytes(text), anchor: 0 };
  if (key.toLowerCase() === "c") return { text, caret: row.caret, anchor: row.anchor, ...(selected ? { clipboard: { copy: text.slice(cut(text, low), cut(text, high)) } } : {}) };
  if (key.toLowerCase() === "x" && selected) return { ...remove(low, high), clipboard: { cut: text.slice(cut(text, low), cut(text, high)) } };
  throw new Error(`${row.case}: unmodelled key ${key}`);
}

/** 📝️ Answers every shared fixture row, returning how many assertions the corpus carried. */
export function textFieldEditingSelfTests(): number {
  const fixture = read("../🧫️fixtures/🧫️text-field-editing/🔣️.json") as Fixture;
  let checks = 0;
  for (const row of fixture.keys) {
    assert.deepEqual(model(row), row.expected, row.case);
    checks += 1;
    if (row.key === "ArrowLeft" || row.key === "ArrowRight") {
      const caret = row.expected.caret;
      if (row.alt || row.ctrl) {
        const boundaries = new Set(marks(row.text).flatMap((mark) => [mark.start, mark.end]));
        assert.ok(boundaries.has(caret) || caret === 0 || caret === bytes(row.text), `${row.case}: a word motion lands on a UAX #29 word boundary`);
        checks += 1;
      }
    }
  }
  for (const row of fixture.pastes) {
    const text = row.kind === "text" ? row.pasted.replace(/\r\n|\r|\n/gu, " ") : row.pasted;
    const low = Math.min(row.anchor, row.caret), high = Math.max(row.anchor, row.caret);
    assert.deepEqual({ text: splice(row.text, low, high, text), caret: low + bytes(text) }, row.expected, row.case);
    checks += 1;
  }
  assert.ok(new Set(fixture.keys.map((row) => row.key)).size >= 8);
  return checks + 1;
}
