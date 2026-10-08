import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { EventEmitter } from "node:events";
import { emitKeypressEvents } from "node:readline";
import { join } from "node:path";
import convert from "color-convert";

type DecodingRow = { id: string; input: string; events: string[]; oracle?: string[]; expire?: boolean };
type QuantizationRow = { rgb: [number, number, number]; ansi256: number; ansi16: number };
type Fixture = { decoding: DecodingRow[]; quantization: QuantizationRow[] };

const fixture: Fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/⌨️input-decoding/🔣️.json"), "utf8"));

const specialNames: Record<string, string> = {
  up: "Up", down: "Down", left: "Left", right: "Right", home: "Home", end: "End", pageup: "PageUp", pagedown: "PageDown",
  insert: "Insert", delete: "Delete", backspace: "Backspace", return: "Enter", space: "Space",
};

type Keypress = { sequence: string; name?: string; ctrl: boolean; meta: boolean; shift: boolean };

function readlineKeys(input: string): Keypress[] {
  const stream = new EventEmitter();
  emitKeypressEvents(stream as never);
  const keys: Keypress[] = [];
  stream.on("keypress", (_text: string | undefined, key: Keypress) => keys.push(key));
  stream.emit("data", Buffer.from(input, "utf8"));
  return keys;
}

function modifierSuffix(key: { ctrl: boolean; meta: boolean; shift: boolean }, withShift: boolean): string {
  return `${key.ctrl ? "+ctrl" : ""}${key.meta ? "+alt" : ""}${withShift && key.shift ? "+shift" : ""}`;
}

function projectReadline(key: Keypress): string {
  const name = key.ctrl && key.name === "`" ? "space" : (key.name ?? "");
  if (name === "tab") return key.shift ? `key:BackTab${modifierSuffix(key, false)}` : `key:Tab${modifierSuffix(key, false)}`;
  const functionKey = /^f(\d+)$/.exec(name);
  if (functionKey) return `key:F${functionKey[1]}${modifierSuffix(key, true)}`;
  if (name in specialNames) return `key:${specialNames[name]}${modifierSuffix(key, true)}`;
  const codePoints = [...key.sequence];
  if (codePoints.length === 1 && codePoints[0] >= " ") return `key:${codePoints[0]}${modifierSuffix(key, false)}`;
  return `key:${name}${modifierSuffix(key, false)}`;
}

const buttons = ["left", "middle", "right"];

function mouseMods(code: number): string {
  return `${code & 16 ? "+ctrl" : ""}${code & 8 ? "+alt" : ""}${code & 4 ? "+shift" : ""}`;
}

function projectMouse(code: number, column: number, row: number, release: boolean): string | undefined {
  if (code & 128) return undefined;
  const at = `@${column - 1},${row - 1}#1${mouseMods(code)}`;
  const low = code & 3;
  if (code & 64) {
    const deltas = ["0,-1", "0,1", "-1,0", "1,0"];
    return `mouse:scroll:${deltas[low]}${at}`;
  }
  if (code & 32) return low === 3 ? `mouse:move${at}` : `mouse:drag:${buttons[low]}${at}`;
  return `mouse:${release || low === 3 ? "up" : "down"}:${buttons[low === 3 ? 0 : low]}${at}`;
}

function specMouseEvents(input: string): string[] {
  const events: string[] = [];
  const pattern = /\x1b\[<(\d+);(\d+);(\d+)([Mm])|\x1b\[M([\s\S])([\s\S])([\s\S])|\x1b\[(\d+);(\d+);(\d+)M/g;
  for (const match of input.matchAll(pattern)) {
    let event: string | undefined;
    if (match[1] !== undefined) event = projectMouse(Number(match[1]), Number(match[2]), Number(match[3]), match[4] === "m");
    else if (match[5] !== undefined) event = projectMouse(match[5].charCodeAt(0) - 32, match[6].charCodeAt(0) - 32, match[7].charCodeAt(0) - 32, false);
    else event = projectMouse(Number(match[8]) - 32, Number(match[9]), Number(match[10]), false);
    if (event) events.push(event);
  }
  return events;
}

function textDecoderEvents(input: string): string[] {
  const events: string[] = [];
  const bytes = Buffer.from(input, "utf8");
  const open = Buffer.from("\x1b[200~");
  const close = Buffer.from("\x1b[201~");
  let at = 0;
  while (at < bytes.length) {
    const start = bytes.indexOf(open, at);
    if (start < 0) break;
    for (const character of bytes.subarray(at, start).toString("utf8")) events.push(`key:${character}`);
    const end = bytes.indexOf(close, start + open.length);
    events.push(`paste:${new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(start + open.length, end))}`);
    at = end + close.length;
  }
  for (const character of bytes.subarray(at).toString("utf8")) events.push(`key:${character}`);
  return events;
}

const rowsFor = (oracle: string) => fixture.decoding.filter((row) => row.oracle?.includes(oracle));

describe("terminal input decoding oracles", () => {
  test("the corpus is closed, unique and broad", () => {
    const ids = fixture.decoding.map((row) => row.id);
    expect(new Set(ids).size).toBe(ids.length);
    expect(fixture.decoding.length).toBeGreaterThanOrEqual(50);
    expect(rowsFor("readline").length).toBeGreaterThanOrEqual(15);
    expect(rowsFor("spec").length).toBeGreaterThanOrEqual(8);
    expect(rowsFor("textdecoder").length).toBeGreaterThanOrEqual(4);
  });

  test("Node readline decodes the key vectors to the same events", () => {
    for (const row of rowsFor("readline")) {
      expect(readlineKeys(row.input).map(projectReadline), row.id).toEqual(row.events);
    }
  });

  test("the xterm control sequence specification decodes the pointer vectors to the same events", () => {
    for (const row of rowsFor("spec")) {
      expect(specMouseEvents(row.input), row.id).toEqual(row.events);
    }
  });

  test("TextDecoder decodes the bracketed paste vectors to the same text", () => {
    for (const row of rowsFor("textdecoder")) {
      expect(textDecoderEvents(row.input), row.id).toEqual(row.events);
    }
  });

  test("color-convert quantizes truecolor to the same palette indices", () => {
    for (const row of fixture.quantization) {
      expect(convert.rgb.ansi256(row.rgb), JSON.stringify(row.rgb)).toBe(row.ansi256);
      expect(convert.rgb.ansi16(row.rgb), JSON.stringify(row.rgb)).toBe(row.ansi16);
    }
  });
});
