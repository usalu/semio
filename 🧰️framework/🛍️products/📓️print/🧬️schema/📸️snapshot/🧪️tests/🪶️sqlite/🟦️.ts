/** 📊️ Full authored chart entities are independently readable and editable as SQLite. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import {sum} from "d3-array";
import corpus from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import schema from "../../🧫️fixtures/🪶️sqlite/🧬️schema/🔣️.json";
import chartSchema from "../../../🔣️.json";
import snapshotSchema from "../../🔣️.json";
import * as snapshotOwner from "../../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase} from "../../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type {ArtifactSqliteOptions} from "../../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
type Snapshot={chart:Record<string,unknown>};
type Owner={CHART_SQLITE_SCHEMA:string;chartToSqliteDatabase:(snapshot:Snapshot,options?:ArtifactSqliteOptions)=>Promise<SqliteDatabase>;chartFromSqliteDatabase:(database:SqliteDatabase,options?:ArtifactSqliteOptions)=>Promise<Snapshot>};
function owner():Owner{const result=snapshotOwner as unknown as Owner;for(const name of ["chartToSqliteDatabase","chartFromSqliteDatabase"]as const)expect(typeof result[name]).toBe("function");expect(typeof result.CHART_SQLITE_SCHEMA).toBe("string");return result;}
test("Chart shared persistence corpus is closed and complete cases obey the independent authored domain",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(corpus)).toBe(true);expect(validate({...corpus,unknown:true})).toBe(false);expect(validateJsonSchemaSubset(schema,corpus)).toEqual([]);
 const specification={$schema:chartSchema.$schema,$defs:chartSchema.$defs,$ref:"#/$defs/ChartSpecification"};const admit=new Ajv2020({strict:false}).compile(specification);
 for(const case_ of corpus.cases)expect(admit(case_.snapshot.chart)).toBe(case_.complete);
 const authored=new Ajv2020({strict:false}).addSchema(chartSchema).compile(snapshotSchema);for(const case_ of corpus.cases)expect(authored(case_.snapshot),case_.id).toBe(true);
 const full=corpus.cases[0]!.snapshot.chart;expect(sum(full.tables!.flatMap(table=>table.rows.map(row=>row.x)))).toBe(-1.5);
 console.log("[DEBUG] Chart shared corpus: four authored states, independent AJV domain and D3 numeric cell oracle");
});
test("Chart semantic entities preserve every authored field through physical SQLite",async()=>{
 const codec=owner();expect(codec.CHART_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text());
 for(const case_ of corpus.cases){const snapshot=case_.snapshot as Snapshot;const logical=await codec.chartToSqliteDatabase(snapshot);const bytes=await exportSqliteDatabase(logical);expect(new TextDecoder().decode(bytes.subarray(0,16))).toBe("SQLite format 3\0");const db=Database.deserialize(bytes);
  try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(await codec.chartFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(snapshot);if(case_.id==="authored-default-without-language")expect(db.query("SELECT language FROM chart_document").get()).toEqual({language:null});}
  finally{db.close();}
 }
 console.log("[DEBUG] Chart complete fields, absent/empty optionals and no default language preserve the physical owner");
});
test("Chart independent SQL exposes domain relationships and edits the complete owner",async()=>{
 const codec=owner(),snapshot=structuredClone(corpus.cases[0]!.snapshot)as Snapshot,db=Database.deserialize(await exportSqliteDatabase(await codec.chartToSqliteDatabase(snapshot)));
 try{const tables=db.query("SELECT name FROM sqlite_schema WHERE type='table'").all()as{name:string}[];for(const name of corpus.requiredTables)expect(tables.map(row=>row.name)).toContain(name);for(const query of corpus.queries)expect(db.query(query.sql).all()).toEqual(query.rows);db.exec(corpus.edit.sql);snapshot.chart.width=corpus.edit.expectedWidth;expect(await codec.chartFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(snapshot);expect(db.query("SELECT COUNT(*) AS count FROM chart_column").get()).toEqual({count:corpus.expectedColumnOccurrences});expect(db.query("SELECT COUNT(*) AS count FROM chart_row").get()).toEqual({count:corpus.expectedRowOccurrences});}
 finally{db.close();}
 console.log("[DEBUG] Independent Chart SQL: domain tables, duplicate declared columns, row cells beyond columns, unresolved named references and complete scalar edit");
});
test("Chart complete positive SQL surrogates preserve every declared owner relationship",async()=>{
 const codec=owner(),snapshot=corpus.cases[0]!.snapshot as Snapshot,db=Database.deserialize(await exportSqliteDatabase(await codec.chartToSqliteDatabase(snapshot)));
 try{db.exec("PRAGMA foreign_keys=OFF");for(const{name}of db.query("SELECT name FROM sqlite_schema WHERE type='table'").all()as{name:string}[]){const key=(db.query("PRAGMA table_info("+name+")").all()as{name:string;pk:number}[]).find(column=>column.pk===1)!.name;const links=db.query("PRAGMA foreign_key_list("+name+")").all()as{from:string}[];db.exec("UPDATE "+name+" SET "+[key+"="+key+"+"+corpus.surrogateOffset,...links.filter(link=>link.from!==key).map(link=>link.from+"="+link.from+"+"+corpus.surrogateOffset)].join(","));}expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(await codec.chartFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(snapshot);}
 finally{db.close();}
 console.log("[DEBUG] Chart complete positive surrogate renumbering preserves full semantic owner");
});
test("Chart reconstruction refuses orphaned and unordered ownership and honors cancellation",async()=>{
 const codec=owner(),snapshot=corpus.cases[0]!.snapshot as Snapshot,logical=await codec.chartToSqliteDatabase(snapshot),bytes=await exportSqliteDatabase(logical);
 for(const sql of corpus.corruptions){const db=Database.deserialize(bytes);try{db.exec("PRAGMA ignore_check_constraints=ON");db.exec(sql);await expect(codec.chartFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).rejects.toThrow();}finally{db.close();}}
 const cancellation=new AbortController();cancellation.abort();await expect(codec.chartToSqliteDatabase(snapshot,{signal:cancellation.signal})).rejects.toThrow();await expect(codec.chartFromSqliteDatabase(logical,{signal:cancellation.signal})).rejects.toThrow();await expect(codec.chartToSqliteDatabase(snapshot,{maxAllocationBytes:0})).rejects.toThrow();await expect(codec.chartFromSqliteDatabase(logical,{maxAllocationBytes:0})).rejects.toThrow();console.log("[DEBUG] Chart corruption, cancellation and zero ownership refuse through the real provider");
});

/** 🚫️ Every declared scalar admission case agrees with the independent complete Chart norm. */
test("Chart SQLite admits no schema-invalid known scalar property",async()=>{
 const codec=owner(),specification={$schema:chartSchema.$schema,$defs:chartSchema.$defs,$ref:"#/$defs/ChartSpecification"},admit=new Ajv2020({strict:false}).compile(specification);
 for(const case_ of corpus.invalidCases){const snapshot=structuredClone(corpus.cases[0]!.snapshot) as Snapshot;let parent=snapshot.chart as any;for(const segment of case_.path.slice(0,-1))parent=parent[segment];parent[case_.path.at(-1)!]=case_.value;expect(admit(snapshot.chart),case_.id).toBe(case_.expectedValid);await expect(codec.chartToSqliteDatabase(snapshot)).rejects.toThrow();}
 console.log("[DEBUG] Chart eighteen known scalar admission refusals match the independent complete domain norm");
});
