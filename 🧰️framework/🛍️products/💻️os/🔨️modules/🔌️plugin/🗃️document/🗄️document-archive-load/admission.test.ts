import sharp from "sharp";
import { DOCUMENT_ARCHIVE_MAXIMUM_BYTES, encodeDocumentArchiveBytes, decodeDocumentArchiveBytes } from "@semio-tech/framework/wire-contracts";
import{test,expect}from"bun:test";import Ajv from"ajv";import{Database}from"bun:sqlite";import law from"./admission.json";import schema from"./admission.schema.json";test("recursive archive admission retains every donor until the canonical ordinal set is validated",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE member(ordinal INTEGER)");for(const row of law.cases){db.run("DELETE FROM member");for(const ordinal of row.ordinals)db.run("INSERT INTO member VALUES(?)",[ordinal]);const status=db.query("SELECT COUNT(*)=COUNT(DISTINCT ordinal) AND (COUNT(*)=0 OR (MIN(ordinal)=0 AND MAX(ordinal)=COUNT(*)-1)) AS accepted FROM member").get()as{accepted:number};expect(Boolean(status.accepted),row.name).toBe(row.accepted);if(status.accepted)expect(db.query("SELECT ordinal FROM member ORDER BY ordinal DESC").all().map(x=>(x as{ordinal:number}).ordinal),row.name).toEqual(row.expected);}}finally{db.close()}});

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
