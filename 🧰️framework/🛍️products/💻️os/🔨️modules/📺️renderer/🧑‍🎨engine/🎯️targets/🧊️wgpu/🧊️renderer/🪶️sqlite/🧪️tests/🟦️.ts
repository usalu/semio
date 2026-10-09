import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020.js";
import {readFileSync} from "node:fs";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import identityFixture from "../🧫️fixtures/🪪️identity/🔣️.json";
import identitySchema from "../../../../../../../🏪️store/🧬️schema/📸️native-identity/🔣️.json";
import bindingFixture from "../../../../../../../🚪️io/🪪️bindings/🧫️fixtures/🔣️.json";
import bindingSchema from "../../../../../../../🚪️io/🪪️bindings/🧬️schema/🔣️.json";
import catalogFixture from "../../../../../../../🚪️io/🪪️bindings/🏛️catalog/🧫️fixtures/🔣️.json";
import catalogSchema from "../../../../../../../🚪️io/🪪️bindings/🏛️catalog/🧬️schema/🔣️.json";
import {catalogBindingKey,proposeCatalogBindings,type ArtifactCatalogBinding} from "../../../../../../../🚪️io/🪪️bindings/🏛️catalog/🟦️.ts";
import {SOCKET_PROBE_NATIVE_DIALECT,socketProbeToSqliteDatabase as project,socketProbeFromSqliteDatabase as restore} from "../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase,type SqliteValue} from "../../../../../../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
const large=fixture.large.unit.repeat(fixture.large.repetitions);
test("native_socket_sqlite_snapshot_original_catalog_declarations_preserve_owner_and_capability",()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile(catalogSchema);
 const rows=catalogFixture.cases.map(entry=>entry.row as ArtifactCatalogBinding);
 for(const row of rows){expect(validate(row)).toBe(true);for(const field of catalogSchema.required){const changed:Record<string,unknown>={...row};delete changed[field];expect(validate(changed)).toBe(false);}expect(validate({...row,extra:1})).toBe(false);}
 const linked=rows[0]!,unlinked=rows[1]!,hosted=rows[2]!,otherWindow=rows[3]!;
 expect(validate({...unlinked,capability:{kind:"unlinkedGuest",sqliteSchema:null}})).toBe(false);expect(validate({...linked,capability:{...linked.capability,factory:""}})).toBe(false);
 expect(catalogBindingKey(hosted)).not.toBe(catalogBindingKey(otherWindow));expect(hosted.contributor.pluginId===hosted.owner.pluginId).toBe(catalogFixture.contributorIsOwner);
 const proposed=proposeCatalogBindings(new Map(),[linked,hosted,otherWindow]);expect(proposed.size).toBe(3);expect(proposeCatalogBindings(new Map(),[unlinked]).size).toBe(1);
 expect(()=>proposeCatalogBindings(new Map(),[linked,linked])).toThrow();expect(proposeCatalogBindings(proposed,[linked]).get(catalogBindingKey(linked))).toEqual(linked);
 expect(()=>proposeCatalogBindings(proposed,[{...linked,capability:{kind:"unlinkedGuest"}}])).toThrow();expect(()=>proposeCatalogBindings(proposed,[{...hosted,owner:hosted.contributor}])).toThrow();
 expect(()=>proposeCatalogBindings(proposed,[{...linked,contributor:{...linked.contributor,componentSha256:"6".repeat(64)}}])).toThrow();expect(()=>proposeCatalogBindings(proposed,[{...hosted,target:{...hosted.target!,grant:{...hosted.target!.grant,write:false}}}])).toThrow();
 for(const malformed of [{...linked,extra:1},{...linked,artifact:{...linked.artifact,packSchemaHash:"0".repeat(64)}},{...hosted,target:{...hosted.target!,parentDialect:"invalid"}},{...linked,capability:{kind:"linked",nativeIdentity:{kind:"guest",pluginId:"foreign",packageHash:"0".repeat(64),schema:linked.artifact.schema},sqliteSchema:null,factory:null}}])expect(()=>proposeCatalogBindings(new Map(),[malformed as ArtifactCatalogBinding])).toThrow();
 const preserved=structuredClone(linked),owned=proposeCatalogBindings(new Map(),[preserved]);preserved.owner.pluginId="changed";expect(owned.get(catalogBindingKey(linked))).toEqual(linked);
 const oracle=new Database(":memory:");try{oracle.exec("CREATE TABLE catalog_binding(id TEXT PRIMARY KEY, contributor TEXT NOT NULL, owner TEXT NOT NULL, installed INTEGER NOT NULL CHECK(installed IN (0,1)), sql_schema TEXT); CREATE TABLE catalog_target(binding TEXT PRIMARY KEY REFERENCES catalog_binding(id), window_kind TEXT NOT NULL); PRAGMA foreign_keys=ON;");for(const row of [unlinked,hosted,otherWindow]){const key=catalogBindingKey(row);oracle.query("INSERT INTO catalog_binding VALUES(?,?,?,?,?)").run(key,row.contributor.pluginId,row.owner.pluginId,row.capability.kind==="linked"?1:0,row.capability.kind==="linked"?row.capability.sqliteSchema:null);if(row.target)oracle.query("INSERT INTO catalog_target VALUES(?,?)").run(key,row.target.surface.windowKindId);}expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(oracle.query("SELECT installed,sql_schema FROM catalog_binding WHERE installed=0").all()).toEqual([{installed:0,sql_schema:null}]);expect(oracle.query("SELECT owner,contributor FROM catalog_binding WHERE installed=1 ORDER BY id").all()).toEqual([{owner:"owner",contributor:"host"},{owner:"owner",contributor:"host"}]);expect(oracle.query("SELECT window_kind FROM catalog_target ORDER BY window_kind").all()).toEqual([{window_kind:"main"},{window_kind:"secondary"}]);}finally{oracle.close();}
 console.error("[DEBUG] Original catalog neutral Source rows=4 unknownGuestRetained=true distinctWindows=true independentAjvSqlite=true");
});
test("native_socket_sqlite_snapshot_original_binding_descriptor_contract",()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile(bindingSchema);
 const original={channel:"directNative",contributor:identityFixture.typedOwner,artifactKind:fixture.dialect.split("@")[0]!,apps:bindingFixture.appLess,schema:fixture.artifactSchema,dialect:fixture.dialect,nativeIdentity:{kind:"typed",owner:identityFixture.typedOwner},sqliteSchema:fixture.sql,factory:bindingFixture.unjoinedFactory};
 expect(validate(original)).toBe(true);expect(validate({...original,sqliteSchema:null})).toBe(bindingFixture.missingSql==="retain");
 for(const field of bindingSchema.required){const changed:Record<string,unknown>={...original};delete changed[field];expect(validate(changed)).toBe(false);}
 expect(validate({...original,extra:1})).toBe(false);expect(validate({...original,nativeIdentity:{kind:"erased"}})).toBe(false);expect(validate({...original,dialect:null})).toBe(false);
 for(const channel of bindingFixture.channels){
  const row:Record<string,unknown>={...original,channel};
  if(channel==="foreignApp"){row.artifactKind=null;row.apps=["viewer"];}
  if(channel==="schemaOnlyDocument"){row.artifactKind=null;row.dialect=null;row.apps=[];}
  if(channel==="treeSubset")row.apps=bindingFixture.treeApps;
  if(channel==="guest")continue;
  expect(validate(row)).toBe(true);
 }
 const schemaOnly={...original,channel:"schemaOnlyDocument",artifactKind:null,dialect:null,apps:[]};
 for(const extra of [{apps:["viewer"]},{artifactKind:original.artifactKind},{dialect:original.dialect},{factory:"unjoined"}])expect(validate({...schemaOnly,...extra})).toBe(false);
 expect(validate({...original,channel:"foreignApp",artifactKind:null,apps:[]})).toBe(false);expect(validate({...original,apps:["viewer","viewer"]})).toBe(false);
 expect(bindingFixture.duplicateProposal).toBe("refuse");expect(bindingFixture.identicalInstalled).toBe("idempotent");expect(bindingFixture.aliasCoordinates).toBe("distinct");
 console.error("[DEBUG] independent Ajv closed binding rows preserve app-less and missing SQL descriptors; guessed joins and opaque owners refuse");
});
test("native_socket_sqlite_snapshot_closed_native_identity_policy",()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile(identitySchema);
 expect(validate({kind:"typed",owner:identityFixture.typedOwner})).toBe(true);
 expect(validate({kind:"typed",owner:identityFixture.typedOwner,packageHash:"00".repeat(32)})).toBe(false);
 expect(validate({kind:"typed",owner:""})).toBe(false);
 expect(validate({kind:"erased"})).toBe(identityFixture.opaqueFallbackAllowed);
 expect(validate({kind:"guest",pluginId:"schema-specimen",packageHash:"00".repeat(32),schema:fixture.artifactSchema})).toBe(true);
 for(const field of identityFixture.guestFields){const row:Record<string,string>={kind:"guest",pluginId:"schema-specimen",packageHash:"00".repeat(32),schema:fixture.artifactSchema};delete row[field];expect(validate(row)).toBe(false);}
});
test("native_socket_sqlite_snapshot_closed_raw_text_policy",()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,extra:1})).toBe(false);for(const name of Object.keys(fixture.callerGrant)){const changed=structuredClone(fixture);delete (changed.callerGrant as Record<string,number>)[name];expect(validate(changed)).toBe(false);}expect(readFileSync(new URL("../🗄️.sql",import.meta.url),"utf8")).toBe(fixture.sql);
 expect(SOCKET_PROBE_NATIVE_DIALECT).toBe(fixture.dialect);
 for(const row of fixture.cases){expect(new TextEncoder().encode(row.text).length).toBe(row.utf8Bytes);expect(row.valueBytes).toBe(row.utf8Bytes+8);}expect(new TextEncoder().encode(large).length).toBe(fixture.large.utf8Bytes);
});
test("native_socket_sqlite_snapshot_original_raw_values_and_independent_edit",async()=>{
 for(const text of [...fixture.cases.map(row=>row.text),large]){
  const database=await project(text),bytes=await exportSqliteDatabase(database);const independent=Database.deserialize(bytes);
  try{expect(independent.query("PRAGMA integrity_check").all()).toEqual([{integrity_check:"ok"}]);expect(independent.query("PRAGMA table_info(socket_probe)").all().map(row=>(row as {name:string;type:string}).type)).toEqual(["INTEGER","TEXT"]);expect(independent.query("SELECT id,text FROM socket_probe").all()).toEqual([{id:1,text}]);independent.query("UPDATE socket_probe SET text=? WHERE id=1").run(fixture.edit);expect(await restore(await importSqliteDatabase(new Uint8Array(independent.serialize())))).toBe(fixture.edit);}finally{independent.close();}
  expect(await restore(await importSqliteDatabase(bytes))).toBe(text);
 }console.error("[DEBUG] Socket original Source raw UTF8/null/nonJSON/full text independent SQLite edit/reimport");
});
test("native_socket_sqlite_snapshot_exact_semantic_limits_and_cancellation",async()=>{
 for(const row of [...fixture.cases,{id:"large",text:large,utf8Bytes:fixture.large.utf8Bytes,valueBytes:fixture.large.valueBytes}]){
  const exact={maxRows:1,maxTables:1,maxColumns:2,maxValueBytes:row.valueBytes};const database=await project(row.text,exact);expect(await restore(database,exact)).toBe(row.text);
  for(const short of[{...exact,maxValueBytes:row.valueBytes-1},{...exact,maxRows:0},{...exact,maxTables:0},{...exact,maxColumns:1}]){await expect(project(row.text,short)).rejects.toThrow();await expect(restore(database,short)).rejects.toThrow();}
 }
 for(const phase of["projectSnapshot","reconstructSnapshot"]as const){const cancel=new AbortController();let interior=false;const options={signal:cancel.signal,onProgress:(event:{phase:string;completed:number;total:number})=>{if(event.phase===phase&&event.completed>0&&event.completed<event.total&&event.total>fixture.large.cancelThreshold){interior=true;cancel.abort();}}};const database=await project(large);await expect(phase==="projectSnapshot"?project(large,options):restore(database,options)).rejects.toThrow();expect(interior).toBe(true);}
});
test("native_socket_sqlite_snapshot_malformed_declared_single_entity",async()=>{
 const original=await project("probe");for(const kind of fixture.malformed){const database=structuredClone(original) as {tables:{name:string;sql:string;rows:{rowid:bigint;values:SqliteValue[]}[]}[]};const rows=database.tables[0]!.rows;const row=rows[0]!;switch(kind){case"missingRow":rows.length=0;break;case"multipleRows":rows.push(structuredClone(row));break;case"wrongRowid":row.rowid=2n;break;case"wrongIdentity":row.values[0]=2n;break;case"nullText":row.values[1]=null;break;case"extraCell":row.values.push("extra");break;}await expect(restore(database as SqliteDatabase)).rejects.toThrow();}
 await expect(project("\ud800")).rejects.toThrow();
});
