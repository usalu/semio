import Ajv2020 from "ajv/dist/2020";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import assert from "node:assert/strict";
import { applyPatch } from "fast-json-patch";

/** 📤️ Independent RFC6902 move semantics validate the shared original diagnostic transfer corpus. */
export function testDiagnosticTransferOracle(): void {
  const selection=JSON.parse(readFileSync(new URL("../../../✂️clamp/🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
  for(const row of selection.cases){
    const db=new Database(":memory:");db.exec("CREATE TABLE rows(ordinal INTEGER PRIMARY KEY,severity INTEGER,bytes INTEGER)");
    for(const[index,message]of row.rows.entries())db.query("INSERT INTO rows VALUES(?,?,?)").run(index,message.level,selection.rowCharge+Buffer.byteLength("mutation.invariant")+Buffer.byteLength(message.text.repeat(message.repeat))+message.targets.reduce((bytes:number,target:string)=>bytes+selection.targetCharge+Buffer.byteLength(target),0));
    const sorted=db.query("SELECT * FROM rows ORDER BY severity DESC,ordinal ASC").all()as{ordinal:number,severity:number,bytes:number}[];
    let kept:number[]=[];
    if(sorted.reduce((bytes,message)=>bytes+message.bytes,Buffer.byteLength(row.editId))<=selection.entryBytes)kept=sorted.map(message=>message.ordinal);
    else{const budget=selection.entryBytes-Buffer.byteLength(row.editId)-selection.rowCharge-Buffer.byteLength("mutation.cascade")-Buffer.byteLength(`${sorted.length} more messages`);let used=0;for(const message of sorted){if(used+message.bytes>budget)break;used+=message.bytes;kept.push(message.ordinal);}if(!kept.length)kept.push(sorted[0].ordinal);}
    kept.sort((left,right)=>left-right);assert.deepEqual(kept,row.expectedKept);assert.equal(row.rows.length-kept.length,row.dropped);
    const original=kept.map(index=>row.rows[index]);const copied=applyPatch({original,report:[]},[{op:"copy",from:"/original",path:"/report"}],true,false).newDocument;assert.deepEqual(copied.original,copied.report);assert.notEqual(copied.original,copied.report);db.close();
  }

  const law = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  for (const row of law.transferCases) {
    const source = law.cases[0].messages;
    const initial = { source, destination: [] };
    const result = applyPatch(structuredClone(initial), [{ op: "move", from: "/source", path: "/destination" }, { op: "add", path: "/source", value: [] }], true).newDocument;
    assert.deepEqual(result, { source: [], destination: source });
    assert.deepEqual(initial, { source, destination: [] });
    const outcome = row.closeFirst ? "refused" : row.items === 0 || row.copy !== "exact" ? "blocked" : "transferred";
    assert.equal(outcome, row.expected);
    if (outcome !== "transferred") assert.deepEqual(initial.source, source);
  }
  for (const row of law.publicationCases) {
    const source = row.rows.flatMap((index: number) => law.cases[index].messages);
    const initial = { source, report: [] };
    const copied = applyPatch(structuredClone(initial), [{ op: "copy", from: "/source", path: "/report" }], true).newDocument;
    assert.deepEqual(copied, { source, report: source });
    assert.notEqual(copied.source, copied.report);
    if (row.cancelAt === null) assert.equal(copied.report.length, row.expectedCopies);
    else assert.equal(row.expectedCopies, 0);
    assert.deepEqual(initial, { source, report: [] });
  }
  for (const row of law.stampCases) {
    const source = row.indices.map((opIndex: number) => ({ opIndex }));
    const actual = applyPatch(source, row.expected.map((opIndex: number, index: number) => ({ op: "replace" as const, path: `/${index}/opIndex`, value: opIndex })), true, false).newDocument;
    assert.deepEqual(actual.map((message: { opIndex: number }) => message.opIndex), row.indices.map((index: number) => row.offset + index));
    assert.deepEqual(source.map((message: { opIndex: number }) => message.opIndex), row.indices);
  }
  for (const row of law.joinCases) {
    const source = row.rows.flatMap((index: number) => law.cases[index].messages);
    const joined = applyPatch({ rows: [] }, source.map((value: unknown) => ({ op: "add" as const, path: "/rows/-", value })), true, false).newDocument;
    assert.deepEqual(joined.rows, source);
    assert.equal(joined.rows.length, row.expectedRows);
  }
  for (const row of law.borrowCases) {
    const source = law.cases[row.sourceCase].messages;
    const copied = applyPatch({ committed: source, durable: [], report: [] }, [{ op: "copy", from: "/committed", path: "/durable" }, { op: "copy", from: "/committed", path: "/report" }], true, false).newDocument;
    assert.deepEqual(copied.durable, source); assert.deepEqual(copied.report, source);
    assert.notEqual(copied.durable, copied.committed); assert.notEqual(copied.report, copied.durable);
  }
  const stage = JSON.parse(readFileSync(new URL("../../../../../../../🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📨️diagnostics/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const severity = ["info", "warning", "error", "fatal"];
  for (const row of stage.cases) {
    const original = { editId: row.editId, messages: row.rows };
    const copied = applyPatch({ durable: original, report: [] }, [{ op: "copy", from: "/durable", path: "/report/0" }], true, false).newDocument;
    assert.deepEqual(copied.report, [original]);
    assert.notEqual(copied.report[0].messages, copied.durable.messages);
    assert.equal(row.rows.reduce((worst: string | null, message: { level: string }) => worst === null || severity.indexOf(message.level) > severity.indexOf(worst) ? message.level : worst, null), row.worst);
    assert.deepEqual(original.messages, row.rows);
  }
  const group = JSON.parse(readFileSync(new URL("../../../../../../../🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const admitted = { editId: group.edit, messages: group.diagnostics };
  const delivered = applyPatch({ durable: admitted, report: [] }, [{ op: "copy", from: "/durable", path: "/report/0" }], true, false).newDocument;
  assert.deepEqual(delivered.report, [admitted]);
  assert.equal(group.diagnostics.reduce((worst: string, row: { level: string }) => severity.indexOf(row.level) > severity.indexOf(worst) ? row.level : worst, "info"), group.worst);
  assert.deepEqual(applyPatch({ n: group.before }, [{ op: "replace", path: "/n", value: group.after }], true, false).newDocument, { n: group.after });
  const custodyRoot = "../../../../../../../🛍️products/💻️os/🔨️modules/🏪️store/📦️prepared/🔗️binding/";
  const custody = JSON.parse(readFileSync(new URL(`${custodyRoot}🧫️fixtures/🔣️.json`, import.meta.url), "utf8"));
  const custodySchema = JSON.parse(readFileSync(new URL(`${custodyRoot}🧬️schema/🔣️.json`, import.meta.url), "utf8"));
  assert(new Ajv2020({ strict: true }).compile(custodySchema)(custody));
  const original = { edit: { mutationMeta: [{}], forwards: ["forward"], inverses: ["inverse"] }, postSnapshot: "post", authority: "original", localActor: "actor", appliedEditId: "edit", tailEditId: "edit", outcomeMessages: [{ level: "warning", code: "NOTICE", message: "original warning" }], foreignStepPresence: true };
  for (const phase of custody.refusalPhases) {
    const admitted = applyPatch(structuredClone(original), [{ op: "add", path: "/edit/mutationMeta/0/groupId", value: "group" }], true, false).newDocument;
    const restored = applyPatch(admitted, [{ op: "remove", path: "/edit/mutationMeta/0/groupId" }], true, false).newDocument;
    assert.deepEqual(restored, original, phase);
  }
  const bound = applyPatch(structuredClone(original), [{ op: "add", path: "/edit/mutationMeta/0/groupId", value: "group" }, { op: "replace", path: "/authority", value: "group-authority" }], true, false).newDocument;
  for (const piece of custody.originalPieces.filter((piece: string) => !["edit", "authority"].includes(piece))) assert.deepEqual(bound[piece as keyof typeof bound], original[piece as keyof typeof original]);
  assert.deepEqual(bound.edit.forwards, original.edit.forwards); assert.deepEqual(bound.edit.inverses, original.edit.inverses);
  console.log("[DEBUG] Prepared group-binding custody: Ajv neutral contract and RFC6902 seven refusal restorations preserve all eight original pieces; successful binding changes only group/seal authority");

}
