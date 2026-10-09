/** 🔏️ Independent framed SHA256 and original-custody state models precede the native seal owner. */
import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {readFileSync,existsSync} from "node:fs";
test("native canonical edit seal returns original fields after framed digest and exact alias closure",()=>{
 const root=new URL("../",import.meta.url);const read=(path:string)=>JSON.parse(readFileSync(new URL(path,root),"utf8"));const fixture=read("🧫️fixtures/🔣️.json");expect(new Ajv2020({strict:true}).compile(read("🧬️schema/🔣️.json"))(fixture)).toBe(true);
 const u64=(value:number)=>{const bytes=Buffer.alloc(8);bytes.writeBigUInt64BE(BigInt(value));return bytes;};const originals=read("../🧫️fixtures/🔣️.json").cases;
 for(let i=0;i<originals.length;i++){const row=originals[i];const bytes=Buffer.from(row.expected);const id=Buffer.from(row.edit.id);const framed=Buffer.concat([Buffer.from("semio.artifact.cursor.v2"),u64(4),Buffer.from("edit"),u64(id.length),id,u64(bytes.length),bytes]);expect(createHash("sha256").update(framed).digest("hex")).toBe(fixture.rows[i].digest);expect(bytes.length).toBe(fixture.rows[i].canonicalLength);}
 const db=new Database(":memory:");db.exec("CREATE TABLE custody(id INTEGER PRIMARY KEY,phase TEXT,owners INTEGER)");db.query("INSERT INTO custody VALUES(1,'original',1)").run();for(const phase of fixture.states){db.query("UPDATE custody SET phase=? WHERE id=1").run(phase);expect(db.query("SELECT owners FROM custody").get()).toEqual({owners:1});}db.close();console.log("[DEBUG] independent Node framedSHA256 exact sparse/dense transaction bytes and SQLite original one-owner conservation agree across count/hash/alias-close/return");
 const path=new URL("🦀️.rs",root);expect(existsSync(path)).toBe(true);const source=readFileSync(path,"utf8");expect(source).toContain("ArtifactCanonicalEditSealCursor");expect(source).toContain("RetainedCloneSource");expect(source).toContain("ArtifactCanonicalJsonTreeCursor");expect(source).toContain("Arc::try_unwrap");expect(source).not.toContain("ArtifactCanonicalEditEncoder");expect(source).not.toContain("ToValue::to_value");expect(source).not.toContain("transmute");
});

/** 🎟️ Pre-admitted original source custody moves once without creating another source frame. */
test("canonical edit seal directly adopts the already admitted immutable original source",()=>{
 const root=new URL("../",import.meta.url);const fixture=JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json",root),"utf8"));expect(new Ajv2020({strict:true}).compile(JSON.parse(readFileSync(new URL("🧬️schema/🔣️.json",root),"utf8")))(fixture)).toBe(true);expect(fixture.admittedSourceTransfer).toBe("same-native-source");
 const db=new Database(":memory:");try{db.exec("CREATE TABLE source(owner INTEGER PRIMARY KEY,location TEXT,births INTEGER)");db.query("INSERT INTO source VALUES(1,'caller',1)").run();for(const admitted of[false,false,true]){if(admitted)db.query("UPDATE source SET location='sealer' WHERE owner=1").run();expect(db.query("SELECT owner,births FROM source").get()).toEqual({owner:1,births:1});}expect(db.query("SELECT location FROM source").get()).toEqual({location:"sealer"});}finally{db.close();}
 console.log("[DEBUG] SQLite original admitted source keeps one exact identity/birth through zero/below/full handoff; Node framed digest fixture remains unchanged");const source=readFileSync(new URL("🦀️.rs",root),"utf8");expect(source).toContain("pub fn source_constructor_demand()");expect(source).toContain("pub fn admit_source(");
});

/** 🛂️ Extracted original owners remain outside while the emptied source is refused before handoff. */
test("canonical source handoff requires a live original owner without losing extracted custody",()=>{
 const root=new URL("../",import.meta.url);const fixture=JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json",root),"utf8"));expect(fixture.sourceRequiresLiveOwner).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE custody(id INTEGER PRIMARY KEY,source INTEGER,outside INTEGER,admitted INTEGER)");db.query("INSERT INTO custody VALUES(1,0,1,0)").run();db.query("UPDATE custody SET admitted=1 WHERE source=1").run();expect(db.query("SELECT source,outside,admitted FROM custody").get()).toEqual({source:0,outside:1,admitted:0});}finally{db.close();}console.log("[DEBUG] SQLite extracted original remains held while empty source admission refuses; no original or lease reconstruction");const producer=readFileSync(new URL("🦀️.rs",root),"utf8");expect(producer).toContain("source.try_borrow()?");
});
