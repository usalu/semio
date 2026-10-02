/** 🧩️ Native Puzzle5d parent identities and exact scalar domains share neutral oracle vectors. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv';
import laws from '../../🧫️fixtures/🪶️sqlite/🔣️.json';
import * as artifact from '../../../🟦️.ts';
import * as snapshot from '../../🟦️.ts';
import {Database} from 'bun:sqlite';
import {fileURLToPath} from 'node:url';
import {PUZZLE5D_SQLITE_SCHEMA} from '../../🪶️sqlite/🗄️schema/🟦️.ts';
import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase} from '../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts';
import type {ArtifactSqliteOptions} from '../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts';
import artifactSchema from '../../../🔣️.json';
import snapshotSchema from '../../🔣️.json';
import ioSchema from '../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json';
import type {DragSelection2d as StandaloneDrag2d} from '../../../🧬️mutations/✋️drag-selection2d/🦠️mutation/🟦️.ts';
import type {DragSelection3d as StandaloneDrag3d} from '../../../🧬️mutations/🚚️drag-selection3d/🦠️mutation/🟦️.ts';
import type {RotateSelection3d as StandaloneRotate3d} from '../../../🧬️mutations/🔄️rotate-selection3d/🦠️mutation/🟦️.ts';
import type {ScaleSelection3d as StandaloneScale3d} from '../../../🧬️mutations/🔍️scale-selection3d/🦠️mutation/🟦️.ts';
import type {CreatePart,AddPartGrip,CreateTargetVolume} from '../../../🧬️mutations/🟦️.ts';
function specimen(bits:bigint):snapshot.Puzzle5dSnapshot {
 const word={bits};
 const part:snapshot.Puzzle5dPart={id:'',partKind:null,anchor:'derived','2d':{x:word,y:word,shape:'',radius:word,width:null,height:word,text:null,iconKind:'',hidden:null,locked:false},'3d':{origin:[word,word,word],meshUrl:'',orientation:[word,word,word,word],scale:word,label:null},grips:[{id:'',gripKind:'unresolved','2d':{angle:word,gripKind:null,radius:word},'3d':{position:[word,word,word],direction:[word,word,word],radius:null,label:''}}]};
 const partKind:snapshot.Puzzle5dCatalogPartKind={id:'',name:'',label:'',description:'',icon:'',image:'',unit:'',abstract:false,baseKinds:['',''],representations:[{id:'',name:'',url:'',mime:'',tags:['',''],lod:null,description:''}],grips:[{id:'',name:'',label:'',description:'',icon:'',gripKind:null,point:[word,word,word],direction:[word,word,word],t:word,mandatory:false,radius:null}],attributes:[{id:'',key:'',value:'',definition:null}],authors:[{id:'',name:'',email:'',role:null,rank:-2147483648}]};
 return{schema:'',domain:'',label:'',meta:{description:'literal\u0000😀'},kindCatalogs:laws.kindCatalogChild,kindCatalogsExtra:{parts:[partKind],grips:[{id:'',code:null,label:'',order:2147483647,compatibleWith:['',''],description:'',icon:'',color:'',defaultRopeKind:''}],fasteners:[{id:'',name:'',label:null}],ropes:[{id:'',name:'',label:'',defaultFastenerKind:''}]},kindCompatibility:[{source:'unresolved',target:'',bidirectional:false,important:true,specificity:'rope'}],parts:[part],fasteners:[{id:'',source:'',target:'unresolved',fastenerKind:null,gap:word,shift:word,rise:word,rotation:word,turn:word,tilt:word,x:word,y:word}],targetVolumes:[{id:'',origin:[word,word,word],orientation:null,scale:[word,word,word],hidden:false,locked:true}]};
}
test('SQLite Puzzle5d neutral child and IEEE boundaries agree with independent engines',()=>{
 expect(new Ajv({strict:true}).compile(laws.childSchema)(laws.kindCatalogChild)).toBe(true);
 for(const raw of laws.binary64Bits){const bytes=Buffer.from(raw,'hex'),view=new DataView(bytes.buffer,bytes.byteOffset,8);expect(view.getBigUint64(0)).toBe(BigInt('0x'+raw));}
});
test('SQLite Puzzle5d artifact catalog owns the actual independent persisted child',()=>{
 const row={schema:'literal',domain:'',label:null,meta:{description:''},kindCatalogs:laws.kindCatalogChild,kindCatalogsExtra:null,kindCompatibility:[],parts:[],fasteners:[],targetVolumes:[]};
 expect(artifact.parsePuzzle5dArtifact(row).kindCatalogs).toEqual(laws.kindCatalogChild);
});
test('SQLite Puzzle5d canonical part owns every native IEEE word and both scale variants',()=>{
 for(const raw of laws.binary64Bits){
  const word={bits:BigInt('0x'+raw)};
  const part: snapshot.Puzzle5dPart={id:'',partKind:null,anchor:'derived','2d':{x:word,y:word,shape:null,radius:word,width:null,height:word,text:null,iconKind:null,hidden:null,locked:false},'3d':{origin:[word,word,word],meshUrl:null,orientation:[word,word,word,word],scale:word,label:null},grips:[{id:'',gripKind:null,'2d':{angle:word,gripKind:null,radius:word},'3d':{position:[word,word,word],direction:null,radius:word,label:null}}]};
  expect(artifact.parsePuzzle5dPart(part)).toEqual(part);
  expect(artifact.parsePuzzle5dPart({...part,'3d':{...part['3d'],scale:[word,word,word]}})['3d'].scale).toEqual([word,word,word]);
 }
});
test('SQLite Puzzle5d author rank and grip order retain the actual signed32 domain',()=>{
 for(const rank of [-2147483648,2147483647,null])expect(artifact.parsePuzzle5dAuthor({id:'',name:'',email:'',role:null,rank}).rank).toBe(rank);
 for(const rank of [-2147483649,2147483648,1.5,undefined])expect(()=>artifact.parsePuzzle5dAuthor({id:'',name:'',email:'',role:null,rank})).toThrow();
});
test('SQLite Puzzle5d typed part rejects numeric alternatives in its canonical double fields',()=>{
 expect(()=>artifact.parsePuzzle5dPart({id:'literal','2d':{x:1,y:0},'3d':{origin:[0,0,0]},grips:[]})).toThrow();
});
test('SQLite Puzzle5d exposes a canonical whole persisted snapshot parser',()=>{
 expect(typeof Reflect.get(snapshot,'parsePuzzle5dSnapshot')).toBe('function');
});
test('SQLite Puzzle5d complete parent retains empty collections, optional values and native catalog fields',()=>{
 const word={bits:0x7ff0000000000042n};
 const partKind:snapshot.Puzzle5dCatalogPartKind={id:'',name:'',label:'',description:'',icon:'',image:'',unit:'',abstract:false,baseKinds:['',''],representations:[{id:'',name:'',url:'',mime:'',tags:['',''],lod:null,description:''}],grips:[{id:'',name:'',label:'',description:'',icon:'',gripKind:null,point:[word,word,word],direction:[word,word,word],t:word,mandatory:false,radius:null}],attributes:[{id:'',key:'',value:'',definition:null}],authors:[{id:'',name:'',email:'',role:null,rank:-2147483648}]};
 const gripKind:snapshot.Puzzle5dCatalogGripKind={id:'',code:null,label:'',order:2147483647,compatibleWith:['',''],description:'',icon:'',color:'',defaultRopeKind:''};
 const row:snapshot.Puzzle5dSnapshot={schema:'',domain:'',label:'',meta:{description:''},kindCatalogs:laws.kindCatalogChild,kindCatalogsExtra:{parts:[partKind],grips:[gripKind],fasteners:[{id:'',name:'',label:null}],ropes:[{id:'',name:'',label:'',defaultFastenerKind:''}]},kindCompatibility:[{source:'',target:'',bidirectional:false,important:true,specificity:'rope'}],parts:[],fasteners:[{id:'',source:'',target:'',fastenerKind:null,gap:word,shift:word,rise:word,rotation:word,turn:word,tilt:word,x:word,y:word}],targetVolumes:[{id:'',origin:[word,word,word],orientation:null,scale:[word,word,word],hidden:false,locked:true}]};
 expect(snapshot.parsePuzzle5dSnapshot(row)).toEqual(row);
 for(const order of [-2147483649,2147483648,0.5,undefined])expect(()=>snapshot.parsePuzzle5dCatalogGripKind({...gripKind,order})).toThrow();
 expect(()=>snapshot.parsePuzzle5dSnapshot({...row,label:undefined})).toThrow();
 expect(()=>snapshot.parsePuzzle5dCatalogPartKind({...partKind,baseKinds:undefined})).toThrow();
});
test('SQLite Puzzle5d handcrafted entities are independently queryable with real foreign keys',async()=>{
 expect(await Bun.file(new URL('../../🪶️sqlite/🗄️.sql',import.meta.url)).text()).toBe(PUZZLE5D_SQLITE_SCHEMA);
 const db=new Database(':memory:');try{db.exec(PUZZLE5D_SQLITE_SCHEMA);expect(db.query("SELECT count(*) AS n FROM sqlite_schema WHERE type='table'").get()).toEqual({n:25});expect(db.query('PRAGMA integrity_check').get()).toEqual({integrity_check:'ok'});expect(db.query('PRAGMA foreign_key_check').all()).toEqual([]);}finally{db.close()}
});
test('SQLite Puzzle5d actual persisted parent crosses an independent SQLite reserialization',async()=>{
 const to=Reflect.get(artifact,'puzzle5dSnapshotToSqliteDatabase') as ((value:snapshot.Puzzle5dSnapshot)=>Promise<SqliteDatabase>)|undefined;
 const from=Reflect.get(artifact,'puzzle5dSnapshotFromSqliteDatabase') as ((value:SqliteDatabase)=>Promise<snapshot.Puzzle5dSnapshot>)|undefined;
 expect(typeof to).toBe('function');expect(typeof from).toBe('function');
 const value:snapshot.Puzzle5dSnapshot={schema:'',domain:'',label:null,meta:{description:'literal\u0000😀'},kindCatalogs:laws.kindCatalogChild,kindCatalogsExtra:{parts:[],grips:[],fasteners:[],ropes:[]},kindCompatibility:[],parts:[],fasteners:[],targetVolumes:[]};
 const bytes=await exportSqliteDatabase(await to!(value)),db=Database.deserialize(bytes);try{expect(db.query('PRAGMA integrity_check').get()).toEqual({integrity_check:'ok'});expect(db.query('PRAGMA foreign_key_check').all()).toEqual([]);expect(db.query('SELECT child_id,target_artifact_id FROM puzzle5_catalog_child').get()).toEqual({child_id: laws.kindCatalogChild.childId,target_artifact_id:''});expect(await from!(await importSqliteDatabase(db.serialize()))).toEqual(value);}finally{db.close()}
});
test('SQLite Puzzle5d every authored entity and exact IEEE state survive independent reserialization',async()=>{
 for(const raw of laws.binary64Bits){const bits=BigInt('0x'+raw),value=specimen(bits),database=await artifact.puzzle5dSnapshotToSqliteDatabase(value);expect(database.tables.every(t=>t.rows.length>0)).toBe(true);const db=Database.deserialize(await exportSqliteDatabase(database));try{for(const [name,count] of Object.entries(laws.tableRowCounts))expect(db.query('SELECT count(*) AS n FROM "'+name+'"').get()).toEqual({n:count});expect(db.query('PRAGMA integrity_check').get()).toEqual({integrity_check:'ok'});expect(db.query('PRAGMA foreign_key_check').all()).toEqual([]);expect(db.query('SELECT CAST(x_ieee754_bits AS TEXT) AS bits FROM puzzle5_part_board').get()).toEqual({bits:BigInt.asIntN(64,bits).toString()});expect(await artifact.puzzle5dSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(value);}finally{db.close()}}
});
test('SQLite Puzzle5d independent semantic edits and consistent surrogate renumbering are retained',async()=>{
 const value=specimen(0n),db=Database.deserialize(await exportSqliteDatabase(await artifact.puzzle5dSnapshotToSqliteDatabase(value)));try{
  db.exec("UPDATE puzzle5_part_board SET x=2,x_ieee754_bits=4611686018427387904,x_numeric_class='finite'");
  const edited={...value,parts:[{...value.parts[0]!, '2d':{...value.parts[0]!['2d'],x:{bits:0x4000000000000000n}}}]};
  expect(await artifact.puzzle5dSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(edited);
  const tables=db.query("SELECT name FROM sqlite_schema WHERE type='table'").all() as {name:string}[];
  for(const {name}of tables){db.exec('UPDATE '+name+' SET id=id+1000');for(const fk of db.query('PRAGMA foreign_key_list('+name+')').all() as {from:string}[])db.exec('UPDATE '+name+' SET '+fk.from+'='+fk.from+'+1000')}
  expect(db.query('PRAGMA foreign_key_check').all()).toEqual([]);
  expect(await artifact.puzzle5dSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(edited);
 }finally{db.close()}
});
test('SQLite Puzzle5d rejects valid-SQL partial values and unowned or duplicate entities',async()=>{
 for(const sql of ["UPDATE puzzle5_part_board SET x_ieee754_bits=1","UPDATE puzzle5_part_world SET orientation_x_ieee754_bits=NULL,orientation_x_numeric_class=NULL,orientation_x=NULL","UPDATE puzzle5_part_scale SET kind='vec3'","UPDATE puzzle5_grip SET ordinal=2","INSERT INTO puzzle5_catalog_extra(document_id) VALUES(1)","UPDATE puzzle5_author SET rank=2147483648"]){
  const db=Database.deserialize(await exportSqliteDatabase(await artifact.puzzle5dSnapshotToSqliteDatabase(specimen(0n))));try{db.exec('PRAGMA ignore_check_constraints=ON');db.exec(sql);await expect(artifact.puzzle5dSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).rejects.toThrow()}finally{db.close()}
 }
});
test('SQLite Puzzle5d exact row frontier and real projection/reconstruction cancellation remain controlled',async()=>{
 const value=specimen(0n),database=await artifact.puzzle5dSnapshotToSqliteDatabase(value),count=database.tables.reduce((n,t)=>n+t.rows.length,0);
 expect(await artifact.puzzle5dSnapshotToSqliteDatabase(value,{maxRows:count})).toEqual(database);
 await expect(artifact.puzzle5dSnapshotToSqliteDatabase(value,{maxRows:count-1})).rejects.toThrow('row');
 const initial=new AbortController();initial.abort();await expect(artifact.puzzle5dSnapshotToSqliteDatabase(value,{signal:initial.signal})).rejects.toThrow();
 const wide={...value,parts:Array.from({length:1024},()=>value.parts[0]!)},projected=await artifact.puzzle5dSnapshotToSqliteDatabase(wide);
 for(const phase of ['projectSnapshot','reconstructSnapshot'] as const){const controller=new AbortController();let seen=false;const options:ArtifactSqliteOptions={signal:controller.signal,onProgress:p=>{if(p.phase===phase&&p.completed>=256){seen=true;controller.abort()}}};await expect(phase==='projectSnapshot'?artifact.puzzle5dSnapshotToSqliteDatabase(wide,options):artifact.puzzle5dSnapshotFromSqliteDatabase(projected,options)).rejects.toThrow();expect(seen).toBe(true)}
});
test('SQLite Puzzle5d declared JSON schema admits the exact word-backed persisted parent',()=>{
 const ajv=new Ajv({strict:false});ajv.addSchema(ioSchema);ajv.addSchema(artifactSchema);const validate=ajv.compile(snapshotSchema);
 const json=JSON.parse(JSON.stringify(specimen(0x7ff0000000000042n),(_,v)=>typeof v==='bigint'?v.toString(16).padStart(16,'0'):v));
 expect(validate(json)).toBe(true);
 expect(validate({...json,parts:[{...json.parts[0],'2d':{...json.parts[0]['2d'],x:{bits:'fffffffffffffffff'}}}]})).toBe(false);
});
test('SQLite Puzzle5d declared JSON leaf preserves every native word without canonical numeric alternatives',()=>{
 const encode=Reflect.get(artifact,'puzzle5dSnapshotToJsonText') as ((value:snapshot.Puzzle5dSnapshot)=>string)|undefined;
 const decode=Reflect.get(artifact,'puzzle5dSnapshotFromJsonText') as ((value:string)=>snapshot.Puzzle5dSnapshot)|undefined;
 expect(typeof encode).toBe('function');expect(typeof decode).toBe('function');
 for(const raw of laws.binary64Bits){const value=specimen(BigInt('0x'+raw)),json=encode!(value);expect(JSON.parse(json).parts[0]['2d'].x).toEqual({bits:raw});expect(decode!(json)).toEqual(value)}
 expect(decode!('{"schema":"","parts":[{"id":"","2d":{"x":1}}]}').parts[0]!['2d'].x).toEqual({bits:0x3ff0000000000000n});
});
test('SQLite Puzzle5d published mutation consumers share canonical words and native integer positions',()=>{
 const value=specimen(0x7ff0000000000001n),word=value.parts[0]!['2d'].x;
 const drag2d:StandaloneDrag2d={targets:[''],dx:word,dy:word};
 const drag3d:StandaloneDrag3d={targets:[''],offset:[word,word,word]};
 const rotate:StandaloneRotate3d={targets:[''],axis:[word,word,word],angle:word};
 const scale:StandaloneScale3d={targets:[''],factors:[word,word,word]};
 const create:CreatePart={part:value.parts[0]!,index:18446744073709551615n};
 const grip:AddPartGrip={partId:'',grip:value.parts[0]!.grips[0]!,index:0n};
 const volume:CreateTargetVolume={targetVolume:value.targetVolumes[0]!,index:null};
 expect([drag2d.dx,drag3d.offset[0],rotate.angle,scale.factors[0]]).toEqual([word,word,word,word]);
 expect([create.index,grip.index,volume.index]).toEqual([18446744073709551615n,0n,null]);
});
test('SQLite Puzzle5d equivalent case and quoted table identities survive independent SQLite rewriting',async()=>{
 const value=specimen(0n),db=Database.deserialize(await exportSqliteDatabase(await artifact.puzzle5dSnapshotToSqliteDatabase(value)));
 try{
  for(const {name} of db.query("SELECT name FROM sqlite_schema WHERE type='table'").all() as {name:string}[]){db.exec('ALTER TABLE '+name+' RENAME TO "temporary_'+name+'"');db.exec('ALTER TABLE "temporary_'+name+'" RENAME TO "'+name.toUpperCase()+'"')}
  expect(db.query('PRAGMA integrity_check').get()).toEqual({integrity_check:'ok'});expect(db.query('PRAGMA foreign_key_check').all()).toEqual([]);
  expect(await artifact.puzzle5dSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(value);
  const imported=await importSqliteDatabase(db.serialize());expect(await artifact.puzzle5dSnapshotFromSqliteDatabase({tables:[...imported.tables].reverse()})).toEqual(value);
 }finally{db.close()}
});
test('SQLite Puzzle5d authored schema admission precedes borrowed semantic traversal',async()=>{
 const value=specimen(0n);let touched=false;
 const borrowed={...value,parts:new Proxy(value.parts,{get(target,key,receiver){if(key===Symbol.iterator){touched=true;throw new Error('borrowed traversal before schema admission')}return Reflect.get(target,key,receiver)}})};
 await expect(artifact.puzzle5dSnapshotToSqliteDatabase(borrowed,{maxSchemaBytes:0})).rejects.toThrow('schema');expect(touched).toBe(false);
});
test('SQLite Puzzle5d long Unicode cells publish known interior work in both semantic directions',async()=>{
 const value={...specimen(0n),meta:{description:'😀é'.repeat(65536)}},database=await artifact.puzzle5dSnapshotToSqliteDatabase(value);
 expect(Buffer.byteLength(value.meta.description,'utf8')).toBe(6*65536);
 for(const phase of ['projectSnapshot','reconstructSnapshot'] as const){let seen=false;const controller=new AbortController(),options:ArtifactSqliteOptions={signal:controller.signal,onProgress:p=>{if(p.phase===phase&&p.total===value.meta.description.length&&p.completed>0&&p.completed<p.total){seen=true;controller.abort()}}};await expect(phase==='projectSnapshot'?artifact.puzzle5dSnapshotToSqliteDatabase(value,options):artifact.puzzle5dSnapshotFromSqliteDatabase(database,options)).rejects.toThrow();expect(seen).toBe(true)}
});
test('SQLite Puzzle5d exact signed IEEE cells cannot wrap outside SQLite INTEGER width',async()=>{
 const database=await artifact.puzzle5dSnapshotToSqliteDatabase(specimen(0n));
 for(const word of [18446744073709551616n,-18446744073709551616n]){const altered={tables:database.tables.map(table=>table.name!=='puzzle5_part_board'?table:{...table,rows:table.rows.map(row=>({...row,values:row.values.map((cell,index)=>index===3?word:cell)}))})};await expect(artifact.puzzle5dSnapshotFromSqliteDatabase(altered)).rejects.toThrow(/signed64|INTEGER/);}
});
test('SQLite Puzzle5d every persisted field retains its authored artifact state axis',()=>{
 for(const field of laws.persistedFields)expect(Reflect.get(artifactSchema.properties,field)['x-semio-state']).toBe('artifact');
});
for(let batch=0;batch<12;batch++)test('SQLite Puzzle5d neutral mutation snapshot batch '+batch+' retains declared JSON state through relational reconstruction',async()=>{
 const root=new URL('../../../../🧫️fixtures/',import.meta.url),ajv=new Ajv({strict:false});ajv.addSchema(ioSchema);const validate=ajv.compile(artifactSchema);let index=0,count=0;
 for await(const path of new Bun.Glob('**/📸️snapshot/**/🔣️.json').scan({cwd:fileURLToPath(root)})){if(index++%12!==batch)continue;const source=await Bun.file(new URL(path,root)).text(),original=JSON.parse(source);expect(validate(original)).toBe(true);const value=artifact.puzzle5dSnapshotFromJsonText(source),database=await artifact.puzzle5dSnapshotToSqliteDatabase(value),restored=await artifact.puzzle5dSnapshotFromSqliteDatabase(database);expect(restored).toEqual(value);expect(artifact.puzzle5dSnapshotFromJsonText(artifact.puzzle5dSnapshotToJsonText(restored))).toEqual(value);count++}
 expect(index).toBe(138);expect(count).toBe(Math.floor((laws.snapshotAssetCount+11-batch)/12));
});
