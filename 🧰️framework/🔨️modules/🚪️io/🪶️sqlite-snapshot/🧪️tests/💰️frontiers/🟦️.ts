/** 🧵️ Shared authored-row, grammar and table-index cancellation laws. */
import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import fixture from "../../🧫️fixtures/💰️frontiers/🔣️.json";
import schema from "../../🧬️schema/💰️frontiers/🔣️.json";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/🟦️.ts";
import {artifactSqliteDatabase,artifactSqliteTables} from "../../🧩️artifact/🟦️.ts";
import {exportSqliteDatabase,SqliteOperation,parseSqliteDatabaseSchema,validateSqliteDatabaseSchemaControlled} from "../../🟦️.ts";

test("shared authored-row frontier corpus is closed and independently admitted",()=>{
 const oracle=new Ajv({strict:true}).compile(schema);expect(oracle(fixture)).toBe(true);expect(validateJsonSchemaSubset(schema,fixture)).toEqual([]);for(const hostile of [{...fixture,extra:1},{...fixture,builderRows:fixture.builderRows-1}]){expect(oracle(hostile)).toBe(false);expect(validateJsonSchemaSubset(schema,hostile).length).toBeGreaterThan(0);}
});
test("actual shared authored-row builder cancels inside its schema corpus validation",async()=>{
 const rows=Array.from({length:fixture.builderRows},(_,i)=>({rowid:BigInt(i+1),values:[BigInt(i+1),null]})),value=await artifactSqliteDatabase(fixture.builderSql,[rows],{}),db=Database.deserialize(await exportSqliteDatabase(value));try{expect(db.query("SELECT count(*) AS n FROM entity").get()).toEqual({n:fixture.builderRows});}finally{db.close();}
 const abort=new AbortController();let reached=false;await expect(Promise.resolve().then(()=>artifactSqliteDatabase(fixture.builderSql,[rows],{signal:abort.signal,onProgress:p=>{if(p.phase===fixture.phases[1]&&p.total===fixture.builderRows&&p.completed>=fixture.rowFrontier&&p.completed<p.total){reached=true;abort.abort();}}}))).rejects.toHaveProperty("kind",fixture.refusal);expect(reached).toBe(true);
});

import finalFixture from "../../🧫️fixtures/🔎️frontiers/🔣️.json";
import finalSchema from "../../🧬️schema/🔎️frontiers/🔣️.json";
function referenceSql(type:string){const columns=Array.from({length:finalFixture.referenceEntries},(_,i)=>`c${i}`),names=columns.join(",");return `CREATE TABLE entity (${columns.map((name,i)=>`${name} ${i===0?type:"integer"}`).join(",")}, FOREIGN KEY (${names}) REFERENCES entity (${names}))`;}
function indexedCorpus(){const names=Array.from({length:finalFixture.tableCount},(_,i)=>i<2?finalFixture.nameUnit.repeat(finalFixture.nameLength)+finalFixture.nameSuffixes[i]:`table_${i}`),sql=names.map(name=>`CREATE TABLE "${name}" (id INTEGER PRIMARY KEY, payload TEXT)`).join(";");const parsed=parseSqliteDatabaseSchema(sql);return {sql,names,value:{tables:parsed.tables.map((table,i)=>({...table,name:table.name.replace(/[A-Z]/g,c=>c.toLowerCase()),rows:[{rowid:1n,values:[1n,`value_${i}`]}]})).reverse()}};}
test("final frontier corpus is closed and independently distinguishes literal table names",()=>{
 const oracle=new Ajv({strict:true}).compile(finalSchema);expect(oracle(finalFixture)).toBe(true);expect(validateJsonSchemaSubset(finalSchema,finalFixture)).toEqual([]);for(const hostile of [{...finalFixture,extra:1},{...finalFixture,referenceFrontier:1}]){expect(oracle(hostile)).toBe(false);expect(validateJsonSchemaSubset(finalSchema,hostile).length).toBeGreaterThan(0);}
 for(let i=0;i<finalFixture.referenceTypes.length;i++){const sql=referenceSql(finalFixture.referenceTypes[i]!),tokens=sql.match(/[A-Za-z_0-9]+|[(),]/g)!;expect(tokens.indexOf("REFERENCES")%2).toBe(finalFixture.referenceParities[i]);const db=new Database(":memory:");try{db.exec(sql);expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_list(entity)").all().length).toBe(finalFixture.referenceEntries);expect(parseSqliteDatabaseSchema(sql).tables.length).toBe(1);}finally{db.close();}}
 const input=indexedCorpus(),db=new Database(":memory:");try{db.exec(input.sql);for(let i=0;i<2;i++){db.query(`INSERT INTO "${input.names[i]}" VALUES (1,?)`).run(`value_${i}`);expect(db.query(`SELECT payload FROM "${input.names[i]!.replace(/[A-Z]/g,c=>c.toLowerCase())}"`).get()).toEqual({payload:`value_${i}`});}expect(db.query("SELECT count(*) AS n FROM sqlite_schema WHERE type='table'").get()).toEqual({n:finalFixture.tableCount});}finally{db.close();}
});
for(const type of finalFixture.referenceTypes)test(`reference classification cancels within integer work units for ${type}`,async()=>{
 const sql=referenceSql(type),value=parseSqliteDatabaseSchema(sql),abort=new AbortController(),add=Set.prototype.add;let numeric=0,referenced=0,burst=0,reached=false;Set.prototype.add=function(value){if(typeof value==="number"){numeric++;if(numeric>finalFixture.referenceEntries+2){referenced++;burst++;}}return add.call(this,value);};
 try{await expect(validateSqliteDatabaseSchemaControlled(value,sql,{signal:abort.signal,onProgress:p=>{expect(Number.isSafeInteger(p.completed)&&Number.isSafeInteger(p.total)).toBe(true);if(p.phase==="validateSchema"&&referenced>0){reached=true;abort.abort();}}})).rejects.toHaveProperty("kind",finalFixture.refusal);expect(reached).toBe(true);expect(burst).toBeGreaterThan(0);expect(burst).toBeLessThanOrEqual(finalFixture.referenceFrontier);}finally{Set.prototype.add=add;}
});
test("actual artifact lookup retains SQLite ASCII case and literal Unicode distinctions",async()=>{
 const input=indexedCorpus(),tables=await artifactSqliteTables(input.value,input.sql,{});expect(tables.length).toBe(finalFixture.tableCount);for(let i=0;i<tables.length;i++)expect(tables[i]![0]!.values).toEqual([1n,`value_${i}`]);
});
test("actual artifact name index cancels within a long literal identifier",async()=>{
 const input=indexedCorpus(),abort=new AbortController();let reached=false;await expect(artifactSqliteTables(input.value,input.sql,{signal:abort.signal,onProgress:p=>{if(p.phase===finalFixture.indexPhase&&p.total===finalFixture.nameLength+finalFixture.nameSuffixes[0]!.length&&p.completed===finalFixture.nameFrontier){reached=true;abort.abort();}}})).rejects.toHaveProperty("kind",finalFixture.refusal);expect(reached).toBe(true);
});
test("actual artifact name index cancels within reversed table census",async()=>{
 const input=indexedCorpus(),abort=new AbortController();let reached=false;await expect(artifactSqliteTables(input.value,input.sql,{signal:abort.signal,onProgress:p=>{if(p.phase===finalFixture.indexPhase&&p.total===finalFixture.tableCount&&p.completed===finalFixture.tableFrontier){reached=true;abort.abort();}}})).rejects.toHaveProperty("kind",finalFixture.refusal);expect(reached).toBe(true);
});
