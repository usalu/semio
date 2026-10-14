/** 🫴️ Authored TXT rows are independently interpreted before original receiving qualification. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import {readFileSync} from "node:fs";
import fixture from "./🧫️fixtures/🔣️.json";
import {semioSchemaAjvV1} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";

test("original TXT semantic receiving preserves authored rows, literal metadata and distinct empty shapes",async()=>{
 const grantSchema=JSON.parse(readFileSync(new URL("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json",import.meta.url),"utf8"));const admitGrant=semioSchemaAjvV1({strict:true}).compile(grantSchema);for(const grant of [fixture.bodyGrant,fixture.closeGrant,fixture.deniedCloseGrant])expect(admitGrant(grant)).toBe(true);
 for(const row of fixture.cases){const db=new Database(":memory:");try{db.exec("PRAGMA foreign_keys=ON");for(const table of row.database.tables)db.exec(table.sql);for(const table of row.database.tables)for(const record of table.rows)db.query("INSERT INTO "+table.name+" VALUES("+record.values.map(()=>"?").join(",")+")").run(...record.values);
 expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT name,sql FROM sqlite_master WHERE type='table' ORDER BY name").all()).toEqual(row.database.tables.map(table=>({name:table.name,sql:table.sql})).sort((a,b)=>a.name.localeCompare(b.name)));for(const table of row.database.tables){const rows=db.query("SELECT rowid,* FROM "+table.name+" ORDER BY id").values();expect(rows).toEqual([...table.rows].sort((a,b)=>a.rowid-b.rowid).map(record=>[record.rowid,...record.values]));}const document=db.query("SELECT schema,trailing_newline,line_ending FROM text_document").get()as{schema:string,trailing_newline:number,line_ending:string};const lines=db.query("SELECT content FROM text_line ORDER BY ordinal").all()as{content:string}[];expect({schema:document.schema,lines:lines.map(line=>line.content),trailingNewline:document.trailing_newline===1,lineEnding:document.line_ending==="crlf"?"crLf":"lf"}).toEqual(row.snapshot);expect(db.query("SELECT count(*) AS count FROM text_line").get()).toEqual({count:row.snapshot.lines.length});const ending=row.snapshot.lineEnding==="crLf"?"\r\n":"\n";expect(row.snapshot.lines.join(ending)+(row.snapshot.trailingNewline?ending:"")).toBe(row.body);expect(new TextEncoder().encode(row.body)).toEqual(new Uint8Array(Buffer.from(row.body)));}finally{db.close();}}
 expect(fixture.cases[0]!.snapshot.lines).toEqual([]);expect(fixture.cases[1]!.snapshot.lines).toEqual([""]);expect(Buffer.byteLength(fixture.source.unit.repeat(fixture.source.repeat))).toBe(fixture.source.bytes);
 console.log("[DEBUG] Original TXT semantic receiving six complete authored shapes match independent SQLite, Buffer, TextEncoder and strict Ajv; empty[] and one empty line remain distinct");
});

test("original TXT malformed rows are independently refused by authored relational predicates",()=>{
 for(const row of fixture.malformed){const db=new Database(":memory:");let refused=false;try{db.exec("PRAGMA foreign_keys=ON");for(const table of row.database.tables)db.exec(table.sql);for(const table of row.database.tables)for(const record of table.rows)db.query("INSERT INTO "+table.name+" VALUES("+record.values.map(()=>"?").join(",")+")").run(...record.values);
 const documents=row.database.tables[0]!.rows,lines=row.database.tables[1]!.rows;const root=documents[0];refused=documents.length!==1||!root||root.rowid<=0||root.values.length!==4||root.values[0]!==root.rowid||typeof root.values[1]!=="string"||![0,1].includes(root.values[2] as number)||!["lf","crlf"].includes(root.values[3]as string)||lines.some(line=>line.rowid<=0||line.values.length!==4||line.values[0]!==line.rowid||line.values[1]!==root.rowid||typeof line.values[3]!=="string")||new Set(lines.map(line=>line.rowid)).size!==lines.length||lines.map(line=>line.values[2]as number).sort((a,b)=>a-b).some((ordinal,index)=>ordinal!==index);
 const declared=db.query("PRAGMA table_info(text_line)").all()as{name:string,type:string}[];refused||=declared.some(column=>column.name==="content"&&column.type!=="TEXT");}catch{refused=true;}finally{db.close();}expect(refused).toBe(true);}
 console.log("[DEBUG] Original TXT eighteen complete malformed databases are independently rejected by SQLite constraints and explicit identity/type/ordinal predicates");
});

test("original TXT mounts both semantic receiving directions and original directional validators",()=>{
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");for(const hook of ["to_sqlite_database_receiving","from_sqlite_database_receiving","validate_sqlite_snapshot_subset_decoding","validate_sqlite_snapshot_subset_encoding"])expect(source).toContain("fn "+hook);
 const receiving=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(receiving).toContain("owner.receive::<Projection,SqliteDatabase>");expect(receiving).toContain("owner.receive::<Reconstruction,TxtSnapshot>");expect(receiving).not.toContain("ordered_row_refs");expect(receiving).not.toContain("NativeDecodeControl::new");expect(receiving).not.toContain("NativeEncodeControl::new");expect(receiving).toContain("post_body.set(true)");expect(receiving).toContain('dialect.artifact_kind!="s.stdio.txt"');expect(receiving).toContain('dialect.standard!="utf-8"');const native=readFileSync(new URL("./🦀️.rs",import.meta.url),"utf8");expect(native.match(/#\[test\]\s*fn txt_original_receiving_/g)?.length).toBe(5);
 console.log("[DEBUG] Original TXT semantic receiving current hooks preserve actual original owners; runtime custody still requires Native law execution");
});

test("original TXT directional input policy independently distinguishes kind, standard and subset",()=>{
 const admit=new Ajv2020({strict:true}).compile({type:"object",required:["kind","standard","subset"],additionalProperties:false,properties:{kind:{const:"s.stdio.txt"},standard:{const:"utf-8"},subset:{const:"*"}}});
 for(const row of fixture.validation)expect(admit({kind:row.kind,standard:row.standard,subset:row.subset})).toBe(row.supported);
 console.log("[DEBUG] Original TXT six directional input cases agree with independent closed semantic dialect schema; exact Native/SQL callbacks and zero backing remain separately mounted Native laws");
});
