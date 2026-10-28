import {describe, test, expect} from "bun:test";
import {readFile} from "node:fs/promises";
import {createHash} from "node:crypto";
import {parseIfc4x3Step, writeIfc4x3Step, ifcReal} from "../step.ts";

const fixture = () => readFile(new URL("triangulated.ifc", import.meta.url), "utf8");
const envelope = (data: string) => `ISO-10303-21;HEADER;FILE_DESCRIPTION(('test'),'2;1');FILE_NAME('a','',(''),(''),'','','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;${data}ENDSEC;END-ISO-10303-21;`;

describe("IFC4x3 STEP exchange", () => {
  test.each(["alignment", "structural"])("roundtrips complete official %s example without graph loss", async name => {
    const bytes = await readFile(new URL(name + ".ifc", import.meta.url));
    const provenance = JSON.parse(await readFile(new URL(name + ".provenance.json", import.meta.url), "utf8"));
    expect(createHash("sha256").update(bytes).digest("hex")).toBe(provenance.sha256);
    const control={batchSize:4096},snapshot = await parseIfc4x3Step(bytes.toString("utf8"),control);
    expect(snapshot.entities.length).toBe(provenance.entities);
    expect(await parseIfc4x3Step(await writeIfc4x3Step(snapshot,control),control)).toEqual(snapshot);
  });
  test("pins official source bytes and roundtrips every typed graph value", async () => {
    const text = await fixture();
    expect(createHash("sha256").update(text).digest("hex")).toBe("935acf08d1f1c93035c9f2454928f8ff7ba89378f60e6560fcc07c82f7eb8fd1");
    const snapshot = await parseIfc4x3Step(text);
    expect(await parseIfc4x3Step(await writeIfc4x3Step(snapshot))).toEqual(snapshot);
    expect(snapshot.entities.find(entity => entity.id === 1021n)?.name).toBe("IFCTRIANGULATEDFACESET");
  });
  test("retains typed values, complex records, big ids, escapes and negative zero", async () => {
    const snapshot = await parseIfc4x3Step(envelope("#9007199254740993=(IFCA('a''b\\\\c\\X2\\00E4\\X0\\\\X4\\0001F3D7\\X0\\',-0.,IFCLENGTHMEASURE(1.E-12)) IFCB((#9007199254740993,$,*),.T.));"));
    expect(snapshot.entities[0]?.id).toBe(9007199254740993n);
    expect(snapshot.entities[0]?.args[1]).toEqual(ifcReal(-0));
    expect(snapshot.entities[0]?.args[0]).toEqual({kind: "string", value: "a'b\\cä🏗"});
    expect(await parseIfc4x3Step(await writeIfc4x3Step(snapshot))).toEqual(snapshot);
  });
  test("retains binary lengths and leading zero bits exactly", async () => {
    const snapshot = await parseIfc4x3Step(envelope('#1=IFCA("0","30","31","210","10FF","0abcd",IFCBINARY("003"));'));
    expect(snapshot.entities[0]?.args[5]).toEqual({kind:"binary",value:"0ABCD"});
    expect(await parseIfc4x3Step(await writeIfc4x3Step(snapshot))).toEqual(snapshot);
    for (const binary of ['"4FF"','"3F"','"1"','"01G"']) await expect(parseIfc4x3Step(envelope(`#1=IFCA(${binary});`))).rejects.toHaveProperty("code","step.binary");
  });
  test("refuses raw malformed Unicode before publication",async()=>{
    await expect(parseIfc4x3Step(envelope("#1=IFCA('\uD800');"))).rejects.toHaveProperty("code","step.string");
    await expect(parseIfc4x3Step(envelope("#1=IFCA('\uDC00');"))).rejects.toHaveProperty("code","step.string");
  });
  test.each([
    ["#1=IFCA();#1=IFCA();", "step.duplicate-id"],
    ["#1=IFCA(#2);", "step.reference"],
    ["#1=IFCA('unterminated);", "step.string"],
    ["/*unterminated", "step.comment"],
    ["#1=IFCA(1.E999);", "step.real"],
    ["#0=IFCA();", "step.id"],
  ])("refuses %s atomically", async (data, code) => {
    await expect(parseIfc4x3Step(envelope(data!))).rejects.toHaveProperty("code", code);
  });
  test("rejects foreign schema and unsupported headers", async () => {
    await expect(parseIfc4x3Step(envelope("").replace("IFC4X3_ADD2", "IFC4"))).rejects.toHaveProperty("code", "step.schema");
    await expect(parseIfc4x3Step(envelope("").replace("HEADER;", "HEADER;EXTRA();"))).rejects.toHaveProperty("code", "step.header");
  });
  test("cancels before publication at progress checkpoints", async () => {
    const abort = new AbortController();
    await expect(parseIfc4x3Step(await fixture(), {signal: abort.signal, batchSize: 1, progress: p => {if (p.completed) abort.abort();}})).rejects.toHaveProperty("code", "io.cancelled");
    const snapshot = await parseIfc4x3Step(await fixture()), before = structuredClone(snapshot);
    const cancel = new AbortController();
    await expect(writeIfc4x3Step(snapshot, {signal: cancel.signal, batchSize: 1, progress: () => cancel.abort()})).rejects.toHaveProperty("code", "io.cancelled");
    expect(snapshot).toEqual(before);
  });
});
