import * as artifactSqlite0 from "../🟦️.ts";
import binary64Schema from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json";
/** 🧊️ Actual native Puzzle3d fields share independent neutral scalar and owning parser laws. */
import {expect,test} from 'bun:test';
import {fileURLToPath} from 'node:url';
import Ajv from 'ajv';
import artifactSchema from "../../../../🧬️schema/🔣️.json";
import * as io from "../../../🟦️.ts";
import laws from "../🧫️fixtures/🔣️.json";
import * as artifact from "../../../../🧬️schema/🟦️.ts";
import * as snapshot from "../../../../🧬️schema/📸️snapshot/🟦️.ts";

import {Database} from 'bun:sqlite';
import {PUZZLE3D_SQLITE_SCHEMA} from "../🗄️schema/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type {ArtifactSqliteOptions} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
function specimen(bits:bigint):snapshot.Puzzle3dSnapshot {
 const word={bits};
 const kind:snapshot.Puzzle3dCatalogObjectKind={id:'',name:'',label:'',description:'',icon:'',image:'',unit:'',abstract:false,baseKinds:['',''],representations:[{id:'',name:'',url:'',mime:'',tags:['',''],lod:null,description:''}],vortices:[{id:'',name:'',label:'',description:'',icon:'',vortexKind:null,point:[word,word,word],direction:[word,word,word],t:word,mandatory:false,radius:null}],attributes:[{id:'',key:'',value:'',definition:null}],authors:[{id:'',name:'',email:'',role:null,rank:-2147483648}]};
 return{schema:'literal\0世界',domain:'',meta:{kindCatalogs:{objects:[kind],vortices:[{id:'',code:null,label:'',order:2147483647,compatibleWith:['',''],description:'',icon:'',color:'',defaultCableKind:'unresolved'}],cables:[{id:'',label:'',name:'',defaultAttractionKind:'unresolved'}],attractions:[{id:'',label:'',name:''}]},kindCompatibility:[{source:'unresolved',target:'',bidirectional:false,important:true,specificity:'cable'}]},objects:[{id:'',label:null,objectKind:'unresolved',anchor:'derived',origin:[word,word,word],orientation:[word,word,word,word],scale:word,meshUrl:'',vortices:[{id:'',vortexKind:null,label:'',position:[word,word,word],direction:null,radius:word,hidden:false,locked:true}],hidden:true,locked:false}],attractions:[{id:'',attracting:'unresolved',attracted:'',gap:word,shift:word,rise:word,rotation:word,turn:word,tilt:word,x:word,y:word}],targetVolumes:[{id:'',origin:[word,word,word],orientation:null,scale:[word,word,word],hidden:false,locked:true}],references:[{id:'',source:{url:'literal\0not-a-uri',mediaKind:null},origin:[word,word,word],widthWorld:word,locked:false,hidden:true}]};
}
function providers(){
 const project=Reflect.get(artifactSqlite0,'puzzle3dSnapshotToSqliteDatabase') as ((snapshot:snapshot.Puzzle3dSnapshot,options?:ArtifactSqliteOptions)=>Promise<SqliteDatabase>)|undefined;
 const restore=Reflect.get(artifactSqlite0,'puzzle3dSnapshotFromSqliteDatabase') as ((db:SqliteDatabase,options?:ArtifactSqliteOptions)=>Promise<snapshot.Puzzle3dSnapshot>)|undefined;
 expect(typeof project).toBe('function');expect(typeof restore).toBe('function');return{project:project!,restore:restore!};
}
test('SQLite Puzzle3d authored twenty-three tables have independent valid relational DDL',()=>{
 const db=new Database(':memory:');try{db.exec(PUZZLE3D_SQLITE_SCHEMA);expect(db.query("SELECT count(*) AS n FROM sqlite_schema WHERE type='table'").get()).toEqual({n:23});expect(db.query('PRAGMA integrity_check').values()).toEqual([['ok']]);expect(db.query('PRAGMA foreign_key_check').values()).toEqual([]);}finally{db.close()}
});
test('SQLite Puzzle3d complete typed state remains independently queryable for every IEEE word',async()=>{
 const {project,restore}=providers();
 for(const raw of laws.binary64Bits){
  const expected=specimen(BigInt('0x'+raw)),relational=await project(expected),bytes=await exportSqliteDatabase(relational),db=Database.deserialize(bytes);try{
   expect(db.query('PRAGMA integrity_check').values()).toEqual([['ok']]);expect(db.query('PRAGMA foreign_key_check').values()).toEqual([]);
   expect(db.query('SELECT o.native_id,v.label FROM puzzle3_object o JOIN puzzle3_vortex v ON v.object_id=o.id').values()).toEqual([['','']]);
   const stmt=db.query('SELECT origin_x_ieee754_bits FROM puzzle3_object') as unknown as {safeIntegers(enabled:boolean):void;values():bigint[][]};stmt.safeIntegers(true);expect(stmt.values()).toEqual([[BigInt.asIntN(64,BigInt('0x'+raw))]]);
   expect(db.query('SELECT url,media_kind FROM puzzle3_reference_source').values() as unknown).toEqual([['literal\0not-a-uri',null]]);
   expect(await restore(await importSqliteDatabase(db.serialize()))).toEqual(expected);
  }finally{db.close()}
 }
});

test('SQLite Puzzle3d neutral IEEE words agree with independent Ajv and DataView',()=>{
 const validate=new Ajv({strict:true}).compile(binary64Schema.$defs.Binary64);
 for(const raw of laws.binary64Bits){expect(validate({bits:raw})).toBe(true);const bytes=Buffer.from(raw,'hex');expect(new DataView(bytes.buffer,bytes.byteOffset,8).getBigUint64(0)).toBe(BigInt('0x'+raw));}
});
test('SQLite Puzzle3d canonical object retains every native word and both scale variants',()=>{
 for(const raw of laws.binary64Bits){const word={bits:BigInt('0x'+raw)},object: snapshot.Puzzle3dObject={id:'literal\0😀',label:null,objectKind:null,anchor:'derived',origin:[word,word,word],orientation:[word,word,word,word],scale:word,meshUrl:'',vortices:[{id:'',vortexKind:null,label:null,position:[word,word,word],direction:[word,word,word],radius:word,hidden:false,locked:true}],hidden:false,locked:true};expect(artifact.parsePuzzle3dObject(object)).toEqual(object);expect(artifact.parsePuzzle3dObject({...object,scale:[word,word,word]}).scale).toEqual([word,word,word])}
});
test('SQLite Puzzle3d canonical object refuses raw numeric alternatives',()=>{
 expect(()=>artifact.parsePuzzle3dObject({id:'',origin:[0,0,0]})).toThrow();
});
test('SQLite Puzzle3d target and reference records validate their actual complete native shape',()=>{
 expect(()=>artifact.parsePuzzle3dTargetVolume({id:''})).toThrow();
 expect(()=>artifact.parsePuzzle3dReference({id:''})).toThrow();
});
test('SQLite Puzzle3d author rank retains the native signed32 domain',()=>{
 for(const rank of laws.signed32)expect(artifact.parsePuzzle3dAuthor({id:'',name:'',email:'',role:null,rank})).toEqual({id:'',name:'',email:'',role:null,rank});
 for(const rank of laws.invalidSigned32)expect(()=>artifact.parsePuzzle3dAuthor({id:'',name:'',email:'',role:null,rank})).toThrow();
});
test('SQLite Puzzle3d owns one complete snapshot parser instead of duplicated permissive models',()=>{
 const parse=Reflect.get(snapshot,'parsePuzzle3dSnapshot') as ((value:unknown)=>unknown)|undefined;expect(typeof parse).toBe('function');
 const value={schema:'',domain:'',meta:{kindCatalogs:{objects:[],vortices:[],cables:[{id:'',label:'',name:'',defaultAttractionKind:'unresolved'}],attractions:[{id:'',label:'',name:''}]},kindCompatibility:[]},objects:[],attractions:[],targetVolumes:[],references:[]};expect(parse!(value)).toEqual(value);expect(artifact.parsePuzzle3dArtifact(value)).toEqual(value);
});

test('SQLite Puzzle3d independent semantic edits reconstruct actual typed fields',async()=>{
 const {project,restore}=providers(),expected=specimen(0n),bytes=await exportSqliteDatabase(await project(expected));
 for(const edit of laws.sqlEdits){const db=Database.deserialize(bytes);try{db.exec(edit.sql);expect(db.query('PRAGMA integrity_check').values()).toEqual([['ok']]);expect(db.query('PRAGMA foreign_key_check').values()).toEqual([]);const actual=await restore(await importSqliteDatabase(db.serialize()));if('url' in edit)expect(actual.references[0]!.source.url).toBe(edit.url!);if('rank' in edit)expect(actual.meta.kindCatalogs!.objects[0]!.authors[0]!.rank).toBe(edit.rank!)}finally{db.close()}}
});
test('SQLite Puzzle3d rejects independently edited invalid semantic relationships and companions',async()=>{
 const {project,restore}=providers(),bytes=await exportSqliteDatabase(await project(specimen(0n)));
 for(const sql of laws.malformedSql){const db=Database.deserialize(bytes);try{db.exec(sql);expect(db.query('PRAGMA integrity_check').values()).toEqual([['ok']]);await expect(restore(await importSqliteDatabase(db.serialize()))).rejects.toThrow()}finally{db.close()}}
});
test('SQLite Puzzle3d admits schema and exact rows before domain traversal and honors cancellation',async()=>{
 const {project,restore}=providers(),value=specimen(0n),db=await project(value),rows=db.tables.reduce((n,t)=>n+t.rows.length,0);
 expect(await project(value,{maxRows:rows})).toEqual(db);await expect(project(value,{maxRows:rows-1})).rejects.toThrow();
 await expect(restore(db,{maxRows:rows-1})).rejects.toThrow();
 let visited=false;const guarded={...value,get objects():snapshot.Puzzle3dObject[]{visited=true;throw new Error('domain visited')}};
 await expect(project(guarded,{maxSchemaBytes:0})).rejects.toThrow();expect(visited).toBe(false);
 const stopped=new AbortController();stopped.abort();await expect(project(value,{signal:stopped.signal})).rejects.toThrow();await expect(restore(db,{signal:stopped.signal})).rejects.toThrow();
 for(const phase of ['projectSnapshot','reconstructSnapshot'] as const){const many={...value,objects:Array.from({length:1024},()=>value.objects[0]!)},source=phase==='reconstructSnapshot'?await project(many):db,abort=new AbortController();let interior=false;const options:ArtifactSqliteOptions={signal:abort.signal,onProgress:p=>{if(p.phase===phase&&p.completed>=256&&p.completed<p.total){interior=true;abort.abort()}}};await expect(phase==='projectSnapshot'?project(many,options):restore(source,options)).rejects.toThrow();expect(interior).toBe(true)}
});
test('SQLite Puzzle3d preserves case-equivalent quoted table identities and reordered physical tables',async()=>{
 const {project,restore}=providers(),expected=specimen(0n),bytes=await exportSqliteDatabase(await project(expected)),db=Database.deserialize(bytes);
 try{db.exec('ALTER TABLE puzzle3_object RENAME TO puzzle3_object_temp');db.exec('ALTER TABLE puzzle3_object_temp RENAME TO "PUZZLE3_OBJECT"');expect(db.query('PRAGMA integrity_check').values()).toEqual([['ok']]);expect(db.query('PRAGMA foreign_key_check').values()).toEqual([]);const imported=await importSqliteDatabase(db.serialize());expect(await restore({...imported,tables:[...imported.tables].reverse()})).toEqual(expected)}finally{db.close()}
});

test('SQLite Puzzle3d declared JSON schema accepts the complete exact word parent',()=>{
 const ajv=new Ajv({strict:false}),validate=ajv.compile(artifactSchema);
 for(const raw of laws.binary64Bits){const transport=JSON.parse(JSON.stringify(specimen(BigInt('0x'+raw)),(_,v)=>typeof v==='bigint'?v.toString(16).padStart(16,'0'):v));expect(validate(transport)).toBe(true)}
});
test('SQLite Puzzle3d declared JSON facade roundtrips the complete word parent',()=>{
 const encode=Reflect.get(io,'puzzle3dSnapshotToJsonText') as ((v:snapshot.Puzzle3dSnapshot)=>string)|undefined,decode=Reflect.get(io,'puzzle3dSnapshotFromJsonText') as ((v:string)=>snapshot.Puzzle3dSnapshot)|undefined;
 expect(typeof encode).toBe('function');expect(typeof decode).toBe('function');
 for(const raw of laws.binary64Bits){const expected=specimen(BigInt('0x'+raw)),text=encode!(expected);expect(JSON.parse(text).objects[0].origin[0]).toEqual({bits:raw});expect(decode!(text)).toEqual(expected)}
});

test('SQLite Puzzle3d distinguishes absent and present empty native catalogs',async()=>{
 const {project,restore}=providers();
 for(const kindCatalogs of [null,{objects:[],vortices:[],cables:[],attractions:[]}] as (snapshot.Puzzle3dKindCatalogs|null)[]){const expected:snapshot.Puzzle3dSnapshot={...specimen(0n),meta:{kindCatalogs,kindCompatibility:[]}};const db=Database.deserialize(await exportSqliteDatabase(await project(expected)));try{expect(db.query('SELECT count(*) AS n FROM puzzle3_catalog').get()).toEqual({n:kindCatalogs===null?0:1});expect(await restore(await importSqliteDatabase(db.serialize()))).toEqual(expected)}finally{db.close()}}
});
test('SQLite Puzzle3d consistently renumbered surrogate identities preserve the logical state',async()=>{
 const {project,restore}=providers(),expected=specimen(0n),relational=await project(expected),db=Database.deserialize(await exportSqliteDatabase(relational));
 const links=[['puzzle3_meta','document_id'],['puzzle3_catalog','meta_id'],['puzzle3_object_kind','catalog_id'],['puzzle3_object_kind_base','kind_id'],['puzzle3_representation','kind_id'],['puzzle3_representation_tag','representation_id'],['puzzle3_vortex_template','kind_id'],['puzzle3_attribute','kind_id'],['puzzle3_author','kind_id'],['puzzle3_vortex_kind','catalog_id'],['puzzle3_vortex_kind_compatible','kind_id'],['puzzle3_cable_kind','catalog_id'],['puzzle3_attraction_kind','catalog_id'],['puzzle3_compatibility','meta_id'],['puzzle3_object','document_id'],['puzzle3_object_scale','object_id'],['puzzle3_vortex','object_id'],['puzzle3_attraction','document_id'],['puzzle3_target','document_id'],['puzzle3_target_scale','target_id'],['puzzle3_reference','document_id'],['puzzle3_reference_source','reference_id']] as const;
 try{db.exec('PRAGMA foreign_keys=OFF');for(const table of relational.tables)db.exec('UPDATE "'+table.name+'" SET id=id+1000');for(const [table,column] of links)db.exec('UPDATE "'+table+'" SET "'+column+'"="'+column+'"+1000');expect(db.query('PRAGMA integrity_check').values()).toEqual([['ok']]);expect(db.query('PRAGMA foreign_key_check').values()).toEqual([]);expect(await restore(await importSqliteDatabase(db.serialize()))).toEqual(expected)}finally{db.close()}
});
for(let batch=0;batch<12;batch++)test('SQLite Puzzle3d actual declared neutral snapshot assets batch '+batch,async()=>{
 const root=new URL('../../../../🧫️fixtures',import.meta.url),validate=new Ajv({strict:false}).compile(artifactSchema),{project,restore}=providers();let index=0,count=0;
 for await(const path of new Bun.Glob('**/📸️snapshot/**/🔣️.json').scan({cwd:fileURLToPath(root)})){if(index++%12!==batch)continue;const source=await Bun.file(new URL(path,root)).text();expect(validate(JSON.parse(source))).toBe(true);const value=io.puzzle3dSnapshotFromJsonText(source),db=await project(value);expect(await restore(db)).toEqual(value);expect(io.puzzle3dSnapshotFromJsonText(io.puzzle3dSnapshotToJsonText(value))).toEqual(value);count++}
 expect(count).toBeGreaterThan(0);
});

for(const branch of ['catalogObjects','representations'] as const)test('SQLite Puzzle3d known borrowed forecast remains cancellable inside '+branch,async()=>{
 const {project}=providers(),value=specimen(0n),catalog=value.meta.kindCatalogs!,kind=catalog.objects[0]!;
 const objects=branch==='catalogObjects'?Array.from({length:1024},()=>kind):[{...kind,representations:Array.from({length:1024},()=>kind.representations[0]!)}],many={...value,meta:{...value.meta,kindCatalogs:{...catalog,objects}}},abort=new AbortController();let known=false;
 await expect(project(many,{signal:abort.signal,onProgress:p=>{if(p.phase==='projectSnapshot'&&p.completed>=256&&p.completed<p.total){known=true;abort.abort()}}})).rejects.toThrow();expect(known).toBe(true);
});
