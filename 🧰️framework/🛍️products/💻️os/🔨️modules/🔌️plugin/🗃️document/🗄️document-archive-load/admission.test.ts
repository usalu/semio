import sharp from "sharp";
import { DOCUMENT_ARCHIVE_MAXIMUM_BYTES, encodeDocumentArchiveBytes, decodeDocumentArchiveBytes } from "@semio-tech/framework/wire-contracts";
import{test,expect}from"bun:test";import{createHash}from"node:crypto";import largeLaw from"../../../📡️spr/🧵️channel/🧪️tests/🧫️fixtures/📦️large-archive/🔣️.json";import largeSchema from"../../../📡️spr/🧵️channel/🧪️tests/🧫️fixtures/📦️large-archive/🧬️schema/🔣️.json";import Ajv from"ajv";import{Database}from"bun:sqlite";import law from"./admission.json";import schema from"./admission.schema.json";test("recursive archive admission retains every donor until the canonical ordinal set is validated",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE member(ordinal INTEGER)");for(const row of law.cases){db.run("DELETE FROM member");for(const ordinal of row.ordinals)db.run("INSERT INTO member VALUES(?)",[ordinal]);const status=db.query("SELECT COUNT(*)=COUNT(DISTINCT ordinal) AND (COUNT(*)=0 OR (MIN(ordinal)=0 AND MAX(ordinal)=COUNT(*)-1)) AS accepted FROM member").get()as{accepted:number};expect(Boolean(status.accepted),row.name).toBe(row.accepted);if(status.accepted)expect(db.query("SELECT ordinal FROM member ORDER BY ordinal DESC").all().map(x=>(x as{ordinal:number}).ordinal),row.name).toEqual(row.expected);}}finally{db.close()}});

test("archive history admits frame and binary payloads within the outer archive bound", async () => {
  const { archiveLimit, transportLimit, historyAllocationLimit, cases, image } = law.history;
  expect(DOCUMENT_ARCHIVE_MAXIMUM_BYTES).toBe(archiveLimit);
  expect(historyAllocationLimit).toBe(archiveLimit * 2);
  using database = new Database(":memory:");
  const admit = database.query(`SELECT (?1 <= ?4 AND ?2 <= ?4 AND ?3 <= ?4) AS accepted`);
  for (const row of cases) {
    const verdict = admit.get(row.archiveBytes, row.frameBodyBytes, row.binaryBytes, archiveLimit) as { accepted: number };
    expect(Boolean(verdict.accepted), row.name).toBe(row.accepted);
  }
  const [r, g, b, alpha] = image.rgba;
  const rgba = await sharp({ create: {
    width: image.width, height: image.height, channels: 4, background: { r, g, b, alpha: alpha! / 255 },
  } }).raw().toBuffer();
  const pixelVarints = rgba.reduce((bytes, channel) => bytes + (channel < 64 ? 1 : 2), 0);
  expect(pixelVarints).toBe(image.pixelVarintBytes);
  expect(pixelVarints).toBeGreaterThan(1024 * 1024);
  const archive = encodeDocumentArchiveBytes({ parent_pack: [1], parent_spr: Array(pixelVarints).fill(0), members: [] });
  expect(decodeDocumentArchiveBytes(archive).parent_spr.length).toBe(pixelVarints);
  expect(Buffer.alloc(archiveLimit).toString("base64").length).toBeLessThan(transportLimit);
});

const MASK = (1n << 64n) - 1n;
const splitmix = (seed: string, length: number) => {
  const out = new Uint8Array(Math.ceil(length / 8) * 8), view = new DataView(out.buffer);
  let state = BigInt(seed);
  for (let at = 0; at < out.length; at += 8) {
    state = (state + 0x9E3779B97F4A7C15n) & MASK;
    let z = ((state ^ (state >> 30n)) * 0xBF58476D1CE4E5B9n) & MASK;
    z = ((z ^ (z >> 27n)) * 0x94D049BB133111EBn) & MASK;
    view.setBigUint64(at, (z ^ (z >> 31n)) & MASK, true);
  }
  return out.subarray(0, length);
};
const fnv1a64 = (bytes: Uint8Array) => {
  let hash = 0xCBF29CE484222325n;
  for (const byte of bytes) hash = ((hash ^ BigInt(byte)) * 0x100000001B3n) & MASK;
  return hash.toString(16).padStart(16, "0");
};

test("archives above the former four mebibyte ceiling encode and decode exactly as the neutral law frames them", () => {
  expect(new Ajv({ strict: true }).compile(largeSchema)(largeLaw)).toBe(true);
  expect(largeLaw.ceiling).toBe(DOCUMENT_ARCHIVE_MAXIMUM_BYTES);
  for (const row of largeLaw.cases) {
    const parent_pack = splitmix(row.packSeed, row.packBytes), parent_spr = splitmix(row.sprSeed, row.sprBytes);
    if (row.outcome === "refused") {
      expect(() => encodeDocumentArchiveBytes({ parent_pack: Array.from(parent_pack), parent_spr: Array.from(parent_spr), members: [] }), row.name).toThrow();
      continue;
    }
    const encoded = encodeDocumentArchiveBytes({ parent_pack: Array.from(parent_pack), parent_spr: Array.from(parent_spr), members: [] });
    expect(encoded.length, row.name).toBe(row.archiveBytes);
    expect(fnv1a64(encoded), row.name).toBe(row.fnv1a64);
    expect(createHash("sha256").update(encoded).digest("hex"), row.name).toBe(row.sha256);
    const decoded = decodeDocumentArchiveBytes(encoded);
    expect(decoded.parent_pack.length).toBe(row.packBytes);
    expect(Buffer.from(decoded.parent_pack).equals(Buffer.from(parent_pack)), row.name).toBe(true);
    expect(Buffer.from(decoded.parent_spr).equals(Buffer.from(parent_spr)), row.name).toBe(true);
  }
});
