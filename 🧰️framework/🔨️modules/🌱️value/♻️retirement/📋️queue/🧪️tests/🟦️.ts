import{test,expect}from"bun:test";
import Ajv from"ajv";
import stableStringify from"fast-json-stable-stringify";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";

type Corpus={ownerCounts:number[],copyGrants:number[],cases:{text:string,reservedCapacity:number}[],cancelCuts:number[]};
const fixture:Corpus=await Bun.file(new URL("../🧫️fixtures/🔣️.json",import.meta.url)).json();
const schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();
const grants=await Bun.file(new URL("../../../🗂️ordered/♻️retirement/🧬️schema/🔣️.json",import.meta.url)).json();
const sharedSchema=await Bun.file(new URL("../../🧬️contract/🧬️schema/🔣️.json",import.meta.url)).json();
const sharedFixture=await Bun.file(new URL("../../🧫️fixtures/📏️shared-physical/🔣️.json",import.meta.url)).json();
test("shared retirement independently pins strong and weak ownership before physical backing release",()=>{
    const ajv=new Ajv({strict:true});ajv.addSchema(grants).addSchema(sharedSchema);const custody=ajv.compile({$ref:sharedSchema.$id+"#/$defs/SharedCustody"});
    for(const row of sharedFixture.cases)for(const copy of sharedFixture.workBytes){
        const wire=stableStringify({text:row.text});expect(JSON.parse(wire)).toEqual({text:row.text});expect(Buffer.byteLength(row.text)).toBe(new TextEncoder().encode(row.text).length);
        const state={strongLeases:2,weakLeases:1,leaseOnly:false,demand:{copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1}};expect(custody(state)).toBe(true);state.strongLeases=1;expect(custody(state)).toBe(true);state.weakLeases=0;state.demand.copyBytes=Math.min(copy,Buffer.byteLength(row.text));expect(custody(state)).toBe(true);expect(custody({...state,weakLeases:-1})).toBe(false);
    }
});
test("typed retirement queue independently separates reservations, original frames, work and physical releases",()=>{
    const ajv=new Ajv({strict:true});ajv.addSchema(grants).addSchema(schema);
    const grant=ajv.compile({$ref:schema.$id+"#/$defs/Grant"});const admission=ajv.compile({$ref:schema.$id+"#/$defs/Admission"});const demand=ajv.compile({$ref:schema.$id+"#/$defs/Demand"});
    expect(admission({frameCapacityBytes:0,reservedSlot:false})).toBe(true);expect(admission({frameCapacityBytes:-1,reservedSlot:true})).toBe(false);expect(demand({copyBytes:1,capacityBytes:0,releaseBytes:257,depth:1})).toBe(true);expect(grant({maximumItems:1,maximumBytes:257})).toBe(false);
    for(const row of fixture.cases)for(const count of fixture.ownerCounts)for(const copy of fixture.copyGrants){
        expect(Buffer.byteLength(row.text)).toBe(new TextEncoder().encode(row.text).length);expect(row.reservedCapacity).toBeGreaterThan(Buffer.byteLength(row.text));
        const queue=Array.from({length:count},()=>row.text);expect(JSON.parse(stableStringify(queue))).toEqual(queue);
        expect(grant({maximumItems:1,maximumCopyBytes:copy,maximumCapacityBytes:0,maximumReleaseBytes:row.reservedCapacity,maximumDepth:count+1})).toBe(true);
        let bytes=0;for(const source of queue){let remaining=Buffer.byteLength(source);while(remaining>0){const processed=Math.min(copy,remaining);remaining-=processed;bytes+=processed;}}
        expect(bytes).toBe(new TextEncoder().encode(row.text).length*count);
    }
});


test("original snapshot frames enter reserved custody without another allocation or premature parent completion",async()=>{
    const corpus=await Bun.file(new URL("../🧫️fixtures/🔣️.json",import.meta.url)).json();const ajv=new Ajv({strict:true});ajv.addSchema(grants).addSchema(schema);const transfer=ajv.compile({$ref:schema.$id+"#/$defs/Transfer"});
    const db=new Database(":memory:");db.run("CREATE TABLE owners(ordinal INTEGER PRIMARY KEY, kind TEXT NOT NULL, terminal INTEGER NOT NULL)");
    for(const row of corpus.transfers){expect(transfer(row)).toBe(true);expect(row.accepted).toBe(row.reservedSlot&&row.items>0&&row.depth>0);}
    for(const [ordinal,kind]of corpus.frameKinds.entries())db.query("INSERT INTO owners VALUES(?,?,0)").run(ordinal,kind);
    expect(db.query("SELECT kind FROM owners ORDER BY ordinal DESC").all()).toEqual([...corpus.frameKinds].reverse().map(kind=>({kind})));
    db.run("UPDATE owners SET terminal=1 WHERE ordinal=(SELECT MAX(ordinal) FROM owners)");expect(db.query("SELECT COUNT(*) AS pending FROM owners WHERE terminal=0").get()).toEqual({pending:1});
    expect(JSON.parse(stableStringify(corpus.snapshotCase))).toEqual(corpus.snapshotCase);expect(corpus.snapshotCase.reservedCapacity).toBeGreaterThan(Buffer.byteLength(corpus.snapshotCase.text));db.close();
    const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("pub fn admit_retirement(");expect(source).toContain("PagedList<Box<dyn ErasedSnapshotRetirement>");expect(source).toContain("size_of_val(owner.as_ref())");expect(source).toContain("RetainedCloneStep::Progress(progress)");expect(source).not.toContain("ErasedControlledRetirement");
});

test("original queue parent denies every independently underfunded frontier before adaptive child effects",async()=>{
 const corpus=await Bun.file(new URL("../🧫️fixtures/🔣️.json",import.meta.url)).json(),d=corpus.guardedDemand;const ajv=new Ajv({strict:true});ajv.addSchema(grants).addSchema(schema);expect(ajv.compile({$ref:schema.$id+"#/$defs/Demand"})(d)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE grants(items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER,depth INTEGER)");for(const row of [[0,4096,65536,262144,64],[1,0,65536,262144,64],[1,4096,23,262144,64],[1,4096,65536,15,64],[1,4096,65536,262144,13],[1,4096,65536,262144,64]])db.query("INSERT INTO grants VALUES(?,?,?,?,?)").run(...row);expect(db.query("SELECT count(*) AS n FROM grants WHERE items>0 AND copy>=? AND capacity>=? AND release>=? AND depth>=?").get(d.copyBytes,d.capacityBytes,d.releaseBytes,d.depth)).toEqual({n:1});}finally{db.close();}const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8"),body=source.slice(source.indexOf("fn step_original("),source.indexOf("let index=self.frames.len()-1"));expect(body.includes("self.next_copy_byte_demand()?")).toBe(true);expect(body.includes("self.next_capacity_byte_demand(grant.maximum_copy_bytes)?")).toBe(true);expect(body.includes("self.next_release_byte_demand()?")).toBe(true);console.log("[DEBUG] Ajv/SQLite independently underfunded original queue frontier refuses before child adaptation or metadata allocation");
});
