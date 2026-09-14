//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { defineTestAdapter, type AdapterContext } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧭️Oracle

type Input = { kind: string; peer: string; generation: number; requestId?: string; payload: string };

/** 🔤️ Escapes a string the way the schema's canonical rendering requires, HTML escaping included. */
function escaped(value: string): string {
  let out = '"';
  for (const character of value) {
    if (character === '"') out += '\\"';
    else if (character === "\\") out += "\\\\";
    else if (character === "\n") out += "\\n";
    else if (character === "\r") out += "\\r";
    else if (character === "\t") out += "\\t";
    else if (character === "<") out += "\\u003c";
    else if (character === ">") out += "\\u003e";
    else if (character === "&") out += "\\u0026";
    else if (character === "\u2028") out += "\\u2028";
    else if (character === "\u2029") out += "\\u2029";
    else if (character.codePointAt(0)! < 0x20) out += `\\u${character.codePointAt(0)!.toString(16).padStart(4, "0")}`;
    else out += character;
  }
  return `${out}"`;
}

/** 🗜️ Strips insignificant whitespace and applies the same escaping to an already-valid JSON document. */
function compact(raw: string): string {
  let out = "";
  let inString = false;
  let wasEscaped = false;
  for (const character of raw) {
    if (inString) {
      if (wasEscaped) wasEscaped = false;
      else if (character === "\\") wasEscaped = true;
      else if (character === '"') inString = false;
    } else if (character === '"') inString = true;
    else if (character === " " || character === "\t" || character === "\n" || character === "\r") continue;
    if (character === "<") out += "\\u003c";
    else if (character === ">") out += "\\u003e";
    else if (character === "&") out += "\\u0026";
    else if (character === "\u2028") out += "\\u2028";
    else if (character === "\u2029") out += "\\u2029";
    else out += character;
  }
  return out;
}

/** 🔗️ Renders one `semio.mcp.event/1` record with the given hash value. */
function render(event: { sequence: number; kind: string; peer: string; generation: number; requestId: string; payload: string; previous: string }, hash: string): string {
  let out = `{"schema":${escaped("semio.mcp.event/1")},"sequence":${event.sequence},"kind":${escaped(event.kind)},"peer":${escaped(event.peer)},"generation":${event.generation}`;
  if (event.requestId !== "") out += `,"requestId":${escaped(event.requestId)}`;
  out += `,"payload":${event.payload}`;
  if (event.previous !== "") out += `,"previous":${escaped(event.previous)}`;
  return `${out},"hash":${escaped(hash)}}`;
}

/** 🔗️ Builds the whole chain with Node's `crypto` — the third-party digest oracle. */
function chain(inputs: Input[]): { lines: string[]; hashes: string[] } {
  const lines: string[] = [];
  const hashes: string[] = [];
  let previous = "";
  inputs.forEach((input, index) => {
    const event = { sequence: index + 1, kind: input.kind, peer: input.peer, generation: input.generation, requestId: input.requestId ?? "", payload: compact(input.payload), previous };
    const hash = createHash("sha256").update(render(event, ""), "utf8").digest("hex");
    lines.push(render(event, hash));
    hashes.push(hash);
    previous = hash;
  });
  return { lines, hashes };
}

/** ✅️ Replays a JSONL chain, refusing any break in the sequence or in the digests. */
function replay(jsonl: string): boolean {
  let previous = "";
  let sequence = 0;
  for (const line of jsonl.split("\n").filter((entry) => entry.length > 0)) {
    sequence += 1;
    const parsed = JSON.parse(line) as Record<string, unknown>;
    const payload = line.slice(line.indexOf('"payload":') + '"payload":'.length, line.indexOf(parsed.previous === undefined ? ',"hash":' : ',"previous":'));
    const event = { sequence, kind: String(parsed.kind ?? ""), peer: String(parsed.peer ?? ""), generation: Number(parsed.generation ?? 0), requestId: String(parsed.requestId ?? ""), payload, previous };
    if (parsed.schema !== "semio.mcp.event/1" || parsed.sequence !== sequence || String(parsed.previous ?? "") !== previous) return false;
    if (createHash("sha256").update(render(event, ""), "utf8").digest("hex") !== parsed.hash) return false;
    previous = String(parsed.hash ?? "");
  }
  return true;
}

function fixture(ctx: AdapterContext): { inputs: Input[]; hashes: string[]; jsonl: string } {
  return JSON.parse(readFileSync(ctx.fixture("shared://🔗️event-chain.json"), "utf8")) as { inputs: Input[]; hashes: string[]; jsonl: string };
}

//#endregion 🧭️Oracle

//#region 🧭️Adapter

/** 🟦️ TypeScript oracle for the hash chain, backed by Node's `crypto` SHA-256. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "chain-digests-match-the-golden": {
      oracle: (ctx) => {
        const { lines, hashes } = chain(fixture(ctx).inputs);
        return { projection: { hashes, jsonl: `${lines.join("\n")}\n` } };
      },
    },
    "a-tampered-chain-is-refused": {
      oracle: (ctx) => {
        const { jsonl } = fixture(ctx);
        return { projection: { intactAccepted: replay(jsonl), tamperedRefused: !replay(jsonl.replace("session.opened", "session.tampered")) } };
      },
    },
  },
});

//#endregion 🧭️Adapter
