//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Both TypeScript halves of the coordinator envelope case. The SUBJECT is the coordinator's own
// Next.js package, whose `📡️events.ts` writes out FIPS 180-4 and the canonical encoding itself. The
// ORACLE is `node:crypto`, which reaches OpenSSL and decides the checksum from outside this
// repository — two independent readings of the same standard, never one library judging itself.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { canonicalEventLine, canonicalJson, eventChecksum } from "../../📦️packages/🟦️typescript/📡️events.ts";
import { defineTestAdapter } from "../../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔮️Oracle
type Envelope = { stream: string; sequence: number; id: string; generation: number; type: string; payload: unknown; checksum: string };

const readGolden = (ctx: { fixtureBytes(uri: string): Uint8Array }): Envelope =>
  JSON.parse(new TextDecoder().decode(ctx.fixtureBytes("shared://📜️g3-event-log.jsonl")).trimEnd());

const readSchema = (ctx: { fixtureBytes(uri: string): Uint8Array }): Record<string, unknown> =>
  Object.fromEntries(Object.entries((JSON.parse(new TextDecoder().decode(ctx.fixtureBytes("schema://repo.server.coordinator/G3EventLogContract"))) as { $defs: { G3EventLogContract: { properties: Record<string, { const: unknown }> } } }).$defs.G3EventLogContract.properties).map(([key, property]) => [key, property.const]));

/** ♻️ The payload as the store persists it: object keys sorted, no insignificant whitespace. */
const canonical = (value: unknown): string => {
  if (value === null || typeof value !== "object") return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(canonical).join(",")}]`;
  const entries = Object.entries(value as Record<string, unknown>).sort(([left], [right]) => (left < right ? -1 : left > right ? 1 : 0));
  return `{${entries.map(([key, item]) => `${JSON.stringify(key)}:${canonical(item)}`).join(",")}}`;
};

/** 🔏️ OpenSSL's SHA-256 over the NUL-separated preimage the schema declares. */
const envelopeChecksum = (event: Envelope): string =>
  createHash("sha256")
    .update(`${event.stream}\u0000${event.sequence}\u0000${event.id}\u0000${event.generation}\u0000${event.type}\u0000`, "utf8")
    .update(canonical(event.payload), "utf8")
    .digest("hex");

/** 📄️ The single canonical line, with the seven fields in the order the schema freezes. */
const canonicalLine = (event: Envelope): string =>
  `{"stream":${JSON.stringify(event.stream)},"sequence":${event.sequence},"id":${JSON.stringify(event.id)},"generation":${event.generation},"type":${JSON.stringify(event.type)},"payload":${canonical(event.payload)},"checksum":${JSON.stringify(envelopeChecksum(event))}}`;
//#endregion 🔮️Oracle

//#region 🧭️Adapter
/** 🟦️ The coordinator's Next.js package as the subject, and node:crypto as the oracle. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "golden-line-is-canonical": {
      subject: (ctx) => {
        const event = readGolden(ctx);
        const schema = readSchema(ctx);
        return {
          projection: {
            line: canonicalEventLine(event),
            reproduced: true,
            fields: schema.fields,
            formula: schema.checksum,
            encoding: schema.encoding,
            schema: schema.schema,
          },
        };
      },
      oracle: (ctx) => {
        const event = readGolden(ctx);
        const schema = readSchema(ctx);
        return {
          projection: {
            line: canonicalLine(event),
            reproduced: true,
            fields: schema.fields,
            formula: schema.checksum,
            encoding: schema.encoding,
            schema: schema.schema,
          },
        };
      },
    },
    "checksum-is-sha256-of-the-preimage": {
      subject: (ctx) => {
        const event = readGolden(ctx);
        return {
          projection: {
            stream: event.stream,
            sequence: event.sequence,
            id: event.id,
            generation: event.generation,
            type: event.type,
            payloadJson: canonicalJson(event.payload),
            checksum: eventChecksum(event),
          },
        };
      },
      oracle: (ctx) => {
        const event = readGolden(ctx);
        return {
          projection: {
            stream: event.stream,
            sequence: event.sequence,
            id: event.id,
            generation: event.generation,
            type: event.type,
            payloadJson: canonical(event.payload),
            checksum: envelopeChecksum(event),
          },
        };
      },
    },
  },
});
//#endregion 🧭️Adapter

