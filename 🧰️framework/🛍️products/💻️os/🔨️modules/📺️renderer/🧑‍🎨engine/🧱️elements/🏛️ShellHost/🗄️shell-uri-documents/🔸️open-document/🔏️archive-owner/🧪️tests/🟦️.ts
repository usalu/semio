/** 🔐️ Ownership vectors agree with an independent SQLite relation before any archive is returned. */
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import schema from "./🔣️schema.json";
import { readOwnedDocumentArchiveV1 } from "../🟦️.ts";
type Sample = { name:string; identities:{app_instance_id:number;parent_document_id:string|null}[]; expectedArchive:boolean;expectedReads:number;missingIdentity?:boolean;replaceDuringRead?:boolean;duplicate?:{samePlugin:boolean;documentId:string} };
test("document archive ownership schema", () => expect(new Ajv().compile(schema)(fixture)).toBe(true));
for (const sample of fixture.cases as Sample[]) test(sample.name, async () => {
    let reads = 0, identities = 0;
    const archive = { source: fixture.documentId };
    const sessions = new Map();
    const plugin = {
        readAppDocumentArchive: async (_instanceId:number) => {
            reads++;
            if (sample.replaceDuringRead) sessions.set("active", {...entry});
            return archive;
        },
        ...(sample.missingIdentity ? {} : { readAppDocumentIdentity:async (_instanceId:number) => sample.identities[Math.min(identities++, sample.identities.length - 1)]! }),
    };
    const entry = {documentId:fixture.documentId,session:{instanceId:fixture.instanceId},plugin};
    sessions.set("active",entry);
    if (sample.duplicate) sessions.set("other",{documentId:sample.duplicate.documentId,session:{instanceId:fixture.instanceId},plugin:sample.duplicate.samePlugin?plugin:{}});
    const result = await readOwnedDocumentArchiveV1(plugin,entry,"active",sessions);
    expect(result).toEqual(sample.expectedArchive?archive:null);
    expect(reads).toBe(sample.expectedReads);
    const sql = new Database(":memory:");
    try {
        const before = sample.identities[0], after = sample.identities.at(-1);
        const row = sql.query("SELECT COALESCE(? = ? AND ? = ? AND ? = ? AND ? = ? AND ? = 0 AND ? = 0 AND ? = 0, 0) AS admitted").get(before?.app_instance_id??null,fixture.instanceId,after?.app_instance_id??null,fixture.instanceId,before?.parent_document_id??null,fixture.documentId,after?.parent_document_id??null,fixture.documentId,Number(Boolean(sample.missingIdentity)),Number(Boolean(sample.replaceDuringRead)),Number(Boolean(sample.duplicate?.samePlugin))) as {admitted:number};
        expect(Boolean(row.admitted)).toBe(sample.expectedArchive);
    } finally { sql.close(); }
});
