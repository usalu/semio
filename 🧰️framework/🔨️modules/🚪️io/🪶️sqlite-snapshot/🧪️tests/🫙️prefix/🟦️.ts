/** 🫙️ Validates original recipient custody and UTF8 against independent SQLite. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
const owner=resolve(import.meta.dir,"../.."),fixture=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🫙️prefix/🔣️.json"),"utf8")) as {schemaSql:string;samples:{text:string;octets:number[]}[]};

test("schema validation retains original lexical and declaration workspaces",()=>{
 const f=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🏛️validation/🔣️.json"),"utf8")) as {cases:{name:string;declared:string;actual:string;columns:string[];accepted:boolean}[]};
 const validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(resolve(owner,"🧬️schema/🏛️validation/🔣️.json"),"utf8")));expect(validate(f)).toBe(true);
 const declaredShape=(sql:string)=>{const db=new Database(":memory:");try{db.exec(sql);return(db.query("SELECT name,type,\"notnull\",dflt_value,pk FROM pragma_table_info((SELECT name FROM sqlite_schema WHERE type='table')) ORDER BY cid").all() as {name:string;type:string;notnull:number;dflt_value:string|null;pk:number}[]).map(row=>({...row,name:row.name.replace(/[A-Z]/g,value=>value.toLowerCase()),type:row.type.toUpperCase()}));}finally{db.close();}};
 for(const row of f.cases){const db=new Database(":memory:");try{db.exec(row.declared);expect((db.query("SELECT name FROM pragma_table_info((SELECT name FROM sqlite_schema WHERE type='table')) ORDER BY cid").all() as {name:string}[]).map(value=>value.name)).toEqual(row.columns);}finally{db.close();}}for(const row of f.cases)expect(JSON.stringify(declaredShape(row.actual))===JSON.stringify(declaredShape(row.declared))).toBe(row.accepted);
 const source=readFileSync(resolve(owner,"🔁️transfer/🦀️.rs"),"utf8");expect(source).toContain("SchemaValidationStorage");expect(source).toContain("validate_database_into");expect(source).toContain("validate_component_into");expect(source).toContain("validate_table_into");
 console.log("[DEBUG] Independent SQLite agrees with quoted identifiers, escaped literals, column declarations and original validation corpus");
});
test("closed original recipient corpus matches independent SQLite UTF8",()=>{
 const validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(resolve(owner,"🧬️schema/🫙️prefix/🔣️.json"),"utf8")));
 expect(validate(fixture)).toBe(true);
 const db=new Database(":memory:");try{db.exec(fixture.schemaSql);for(const[index,row]of fixture.samples.entries())db.run("INSERT INTO original_values VALUES(?,?)",[index+1,row.text]);const rows=db.query("SELECT value,hex(CAST(value AS BLOB)) AS octets FROM original_values ORDER BY id").all() as {value:string;octets:string}[];expect(rows).toEqual(fixture.samples.map(row=>({value:row.text,octets:Buffer.from(row.octets).toString("hex").toUpperCase()})));}finally{db.close();}
 console.log("[DEBUG] Independent SQLite preserves original empty, embedded NUL and multibyte UTF8 fields");
});
test("SQLite scopes retain the original recipient with explicit active admission",()=>{
 const source=readFileSync(resolve(owner,"🦀️.rs"),"utf8"),recipient=readFileSync(resolve(owner,"../../🌱️value/🛬️decode/🫴️recipient/🦀️.rs"),"utf8");
 expect(recipient).toContain("pub fn with_reserved_owner");expect(source).toContain("pub fn with_retirement_owner");expect(source).toContain("retirement_active");expect(source).toContain("recipient.with_reserved_owner");
});
test("relational original-field helpers expose retained text and blob destinations",()=>{
 const source=readFileSync(resolve(owner,"🧩️artifact/🦀️.rs"),"utf8");expect(source).toContain("pub fn reconstruct_text_into");expect(source).toContain("pub fn reconstruct_blob_into");
});
test("nested original recipients expose their actual typed field retirement",()=>{
 const recipient=resolve(owner,"../../🌱️value/🛬️decode/🫴️recipient"),source=readFileSync(resolve(recipient,"♻️retirement/🦀️.rs"),"utf8");expect(readFileSync(resolve(recipient,"🦀️.rs"),"utf8")).toContain('mod retirement;');expect(source).toContain("impl RetireOwned for NativeDecodeRetirementRecipient");expect(source).toContain("impl RetirementCursor for NativeDecodeRetirementRecipient");
});
test("relational row views borrow genuine caller-owned scalar storage",()=>{
 const f=JSON.parse(readFileSync(resolve(owner,"🧩️artifact/🧫️fixtures/🗂️paid-row-index/🔣️.json"),"utf8")) as {schema:string;rows:{id:number;owner:number;ordinal:number;literal:string;parent:number|null}[];identityIds:number[];groupedIds:number[]};const db=new Database(":memory:");try{db.exec(f.schema);for(const row of f.rows)db.run("INSERT INTO neutral_row VALUES(?,?,?,?,?)",[row.id,row.owner,row.ordinal,row.literal,row.parent]);expect((db.query("SELECT id FROM neutral_row ORDER BY id").all() as {id:number}[]).map(row=>row.id)).toEqual(f.identityIds);expect((db.query("SELECT id FROM neutral_row ORDER BY owner,ordinal").all() as {id:number}[]).map(row=>row.id)).toEqual(f.groupedIds);}finally{db.close();}
 const source=readFileSync(resolve(owner,"🧩️artifact/🦀️.rs"),"utf8");expect(source).toContain("pub struct RowIndexStorage");expect(source).toContain("storage:&'a mut RowIndexStorage");expect(source).toContain("pub fn all_indices_into");expect(source).toContain("pub fn grouped_by_into");expect(source).toContain("pub fn parent_positions_into");
});

test("Complete original database fields agree with independent SQLite",()=>{
 const f=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🫙️prefix/🪶️database.json"),"utf8")) as {schemaSql:string;rows:{id:number;text:string;octets:number[];optional:number|null;value:number}[];deniedAxes:string[]};
 const validate=new Ajv({strict:true}).compile({type:"object",required:["schemaSql","rows","deniedAxes"],additionalProperties:false,properties:{schemaSql:{type:"string"},rows:{type:"array",items:{type:"object",required:["id","text","octets","optional","value"],additionalProperties:false,properties:{id:{type:"integer"},text:{type:"string"},octets:{type:"array",items:{type:"integer",minimum:0,maximum:255}},optional:{type:["number","null"]},value:{type:"number"}}}},deniedAxes:{type:"array",items:{enum:["items","copy","capacity","release","depth"]},minItems:5,maxItems:5,uniqueItems:true}}});
 expect(validate(f)).toBe(true);const db=new Database(":memory:");
 try{db.exec(f.schemaSql);for(const row of f.rows)db.run("INSERT INTO original_database VALUES(?,?,?,?,?)",[row.id,row.text,new Uint8Array(row.octets),row.optional,row.value]);
 expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
 expect(db.query("SELECT id,text,hex(payload) AS payload,optional,value FROM original_database ORDER BY id").all()).toEqual(f.rows.map(row=>({id:row.id,text:row.text,payload:Buffer.from(row.octets).toString("hex").toUpperCase(),optional:row.optional,value:row.value})));
 }finally{db.close();}
 console.log("[DEBUG] Independent SQLite complete database corpus preserves empty, NUL, Unicode, exact blobs, nulls and scalar cells");
});

test("Nested conversion producers borrow the original transfer ledger and callback",()=>{
 const source=readFileSync(resolve(owner,"🦀️.rs"),"utf8");
 expect(source).toContain("pub fn with_retirement_child");
 expect(source).toContain("Borrowed(&'a mut SqliteTransferLedger)");
 expect(source).toContain("TransferLedgerStorage::Borrowed");
});
