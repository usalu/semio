/** 📊️ Full authored chart entities are independently readable and editable as SQLite. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import "../../../🧪️tests/🏛️ownership/🟦️.ts";

import Ajv2020 from "ajv/dist/2020";
import {sum} from "d3-array";
import corpus from "../../../../🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🔣️.json";

import chartSchema from "../../../../🧬️schema/🔣️.json";
import snapshotSchema from "../../../../🧬️schema/📸️snapshot/🔣️.json";
import * as snapshotOwner from "../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase} from "../../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type {ArtifactSqliteOptions} from "../../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";

type Snapshot={chart:Record<string,unknown>};
type Owner={CHART_SQLITE_SCHEMA:string;chartToSqliteDatabase:(snapshot:Snapshot,options?:ArtifactSqliteOptions)=>Promise<SqliteDatabase>;chartFromSqliteDatabase:(database:SqliteDatabase,options?:ArtifactSqliteOptions)=>Promise<Snapshot>};
function owner():Owner{const result=snapshotOwner as unknown as Owner;for(const name of ["chartToSqliteDatabase","chartFromSqliteDatabase"]as const)expect(typeof result[name]).toBe("function");expect(typeof result.CHART_SQLITE_SCHEMA).toBe("string");return result;}
test("Chart shared persistence corpus is closed and complete cases obey the independent authored domain",()=>{
 
 const specification={$schema:chartSchema.$schema,$defs:chartSchema.$defs,$ref:"#/$defs/ChartSpecification"};const admit=new Ajv2020({strict:false}).compile(specification);
 for(const case_ of corpus.cases)expect(admit(case_.snapshot.chart)).toBe(case_.complete);
 const authored=new Ajv2020({strict:false}).addSchema(chartSchema).compile(snapshotSchema);for(const case_ of corpus.cases)expect(authored(case_.snapshot),case_.id).toBe(true);
 const full=corpus.cases[0]!.snapshot.chart;expect(sum(full.tables!.flatMap(table=>table.rows.map(row=>row.x)))).toBe(-1.5);
 console.log("[DEBUG] Chart shared corpus: four authored states, independent AJV domain and D3 numeric cell oracle");
});
test("Chart semantic entities preserve every authored field through physical SQLite",async()=>{
 const codec=owner();expect(codec.CHART_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());
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

/** 🧮️ Independent SQLite edits distinguish a valid stale companion from malformed metadata. */
test("Chart exact numeric companions reject malformed metadata before a REAL-only edit",async()=>{
 const codec=owner(),snapshot=corpus.cases.find(case_=>case_.id===corpus.companionCases.baseCase)!.snapshot as Snapshot,specification={$schema:chartSchema.$schema,$defs:chartSchema.$defs,$ref:"#/$defs/ChartSpecification"},admit=new Ajv2020({strict:false}).compile(specification);
 const bytes=await exportSqliteDatabase(await codec.chartToSqliteDatabase(snapshot));
 for(const case_ of corpus.companionCases.cases){const db=Database.deserialize(bytes);try{
  db.exec("UPDATE chart_document SET width=120,width_kind='uint',width_exact='120'");db.exec(case_.sql);expect((db.query("SELECT width FROM chart_document").get() as{width:number}).width).toBe(case_.accept?case_.value!:120);
  const logical=await importSqliteDatabase(db.serialize());if(case_.accept){const expected=structuredClone(snapshot);expected.chart.width=case_.value;expect(admit(expected.chart),case_.id).toBe(true);expect(await codec.chartFromSqliteDatabase(logical),case_.id).toEqual(expected);}else await expect(codec.chartFromSqliteDatabase(logical),case_.id).rejects.toThrow();
 }finally{db.close();}}
 console.log("[DEBUG] Chart shared nineteen companion edits: four accepted exact/stale scalar edits and fifteen malformed metadata refusals");
});

/** 🧊️ Neutral IEEE words are checked against the system Buffer oracle and physical SQL companions. */
test("Chart physical numeric cells preserve exact IEEE words and producer companion spellings",async()=>{
 const codec=owner(),snapshot=structuredClone(corpus.cases[0]!.snapshot) as Snapshot,values:Record<string,number>={};
 const word=(value:number)=>{const buffer=new ArrayBuffer(8),view=new DataView(buffer);view.setFloat64(0,value,false);return view.getBigUint64(0,false).toString(16).padStart(16,"0");};
 for(const case_ of corpus.numericWords){const bytes=Uint8Array.from(case_.bits.match(/../g)!.map(byte=>parseInt(byte,16))),value=new DataView(bytes.buffer).getFloat64(0,false);expect(Object.is(value,Buffer.from(case_.bits,"hex").readDoubleBE(0)),case_.id).toBe(true);values[case_.id]=value;}
 const tables=snapshot.chart.tables as{rows:Record<string,unknown>[];columns:string[]}[];tables.push({name:"ieee-exact",columns:corpus.numericWords.map(case_=>case_.id),rows:[values]} as typeof tables[number]);
 const db=Database.deserialize(await exportSqliteDatabase(await codec.chartToSqliteDatabase(snapshot)));let actual:Snapshot;
 try{const emitted=db.query("SELECT name,number_kind,number_exact FROM chart_cell WHERE name IN ("+corpus.numericWords.map(()=>"?").join(",")+") ORDER BY name").all(...corpus.numericWords.map(case_=>case_.id));actual=await codec.chartFromSqliteDatabase(await importSqliteDatabase(db.serialize()));expect(actual).toEqual(snapshot);
  const reopened=Database.deserialize(await exportSqliteDatabase(await codec.chartToSqliteDatabase(actual)));try{expect(reopened.query("SELECT name,number_kind,number_exact FROM chart_cell WHERE name IN ("+corpus.numericWords.map(()=>"?").join(",")+") ORDER BY name").all(...corpus.numericWords.map(case_=>case_.id))).toEqual(emitted);}finally{reopened.close();}
 }finally{db.close();}
 const row=(actual.chart.tables as{rows:Record<string,number>[]}[]).at(-1)!.rows[0]!;for(const case_ of corpus.numericWords)expect(word(row[case_.id]!),case_.id).toBe(case_.bits);
 console.log("[DEBUG] Chart five closed IEEE words, including negative zero and finite extremes, retain exact bits and producer-emitted companions over physical SQLite");
});
