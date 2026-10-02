/** 🧱️ Exact Block5d canonical parent laws share independent neutral primitive evidence. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv';
import laws from '../../🧫️fixtures/🪶️sqlite/🔣️.json';
import * as artifact from '../../../🟦️.ts';
import * as snapshot from '../../🟦️.ts';
function specimen(bits:bigint):snapshot.Block5dSnapshot{const word={bits};return{schema:'literal\0世界',partKind:{id:'',name:'',label:'',variant:null,description:'',icon:null,unit:null},part2d:{shape:null,radius:word,width:null,height:word,color:null,iconKind:null},part3d:{orientation:[word,word,word,word],scale:[word,word,word]},representations:[{id:'',name:'',meshUrl:null,tags:['',''],lod:null,description:'',attributes:[{key:'',value:'',definition:null}]}],gripKinds:[{id:'',name:'',label:'',color:'',defaultRopeKind:'unresolved'}],grips:[{id:'',gripKind:'unresolved',angle:word,radius2d:word,position:[word,word,word],direction:[word,word,word],radius3d:word}],compatibility:[{id:'',source:'unresolved',target:'',bidirectional:false}],attributes:[{key:'',value:'',definition:null}],authors:[{id:'',name:'',email:null}],camera2d:{x:word,y:word,zoom:word},camera3d:{position:[word,word,word],target:[word,word,word],zoom:word},meta:{description:'literal\0😀'}}}
test('SQLite Block5d neutral full IEEE words agree with independent Ajv and DataView',()=>{
 const validate=new Ajv({strict:true}).compile(laws.wordSchema);for(const raw of laws.binary64Bits){expect(validate({bits:raw})).toBe(true);const bytes=Buffer.from(raw,'hex');expect(new DataView(bytes.buffer,bytes.byteOffset,8).getBigUint64(0)).toBe(BigInt('0x'+raw))}
});
test('SQLite Block5d canonical geometry rejects raw numeric alternatives',()=>{
 expect(()=>artifact.parseBlock5dPart2d({shape:null,radius:0,width:null,height:null,color:null,iconKind:null})).toThrow();
 expect(()=>artifact.parseBlockCamera2d({x:0,y:0,zoom:1})).toThrow();
});
test('SQLite Block5d domain records refuse incomplete native required fields',()=>{
 expect(()=>artifact.parseBlock5dGripKind({id:''})).toThrow();expect(()=>artifact.parseBlockRepresentation({id:''})).toThrow();expect(()=>artifact.parseBlockCamera3d({})).toThrow();
});
test('SQLite Block5d owns one complete canonical persisted parent parser',()=>{
 const parse=Reflect.get(snapshot,'parseBlock5dSnapshot') as ((v:unknown)=>unknown)|undefined;expect(typeof parse).toBe('function');
 for(const raw of laws.binary64Bits){const value=specimen(BigInt('0x'+raw));expect(parse!(value)).toEqual(value);expect(artifact.parseBlock5dArtifact(value)).toEqual(snapshot.parseBlock5dSnapshot(value))}
});

import {Database} from 'bun:sqlite';
test('SQLite Block5d handwritten tables satisfy the independent native schema oracle',async()=>{
 const sql=await Bun.file(new URL('../../🪶️sqlite/🗄️.sql',import.meta.url)).text(),db=new Database(':memory:');try{db.exec(sql);expect(db.query('PRAGMA integrity_check').values()).toEqual([['ok']]);expect(db.query('PRAGMA foreign_key_check').values()).toEqual([]);expect(db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").values().map(r=>r[0])).toEqual(Object.keys(laws.tableRowCounts).sort());expect(db.query('PRAGMA table_info(block5_grip)').all().length).toBe(32)}finally{db.close()}
});
test('SQLite Block5d public provider must retain the complete every-entity parent',async()=>{
 const project=Reflect.get(artifact,'block5dSnapshotToSqliteDatabase') as ((v:snapshot.Block5dSnapshot)=>Promise<unknown>)|undefined;
 const restore=Reflect.get(artifact,'block5dSnapshotFromSqliteDatabase') as ((v:unknown)=>Promise<unknown>)|undefined;
 expect(typeof project).toBe('function');expect(typeof restore).toBe('function');
 const value=snapshot.parseBlock5dSnapshot(specimen(BigInt('0x'+laws.binary64Bits[0]!)));expect(await restore!(await project!(value))).toEqual(value);
});

import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase} from '../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts';

async function oracle(value:snapshot.Block5dSnapshot):Promise<Database>{return Database.deserialize(await exportSqliteDatabase(await artifact.block5dSnapshotToSqliteDatabase(value)))}
for(const raw of laws.binary64Bits)test('SQLite Block5d independent SQLite reserialization exact word '+raw,async()=>{
 const value=specimen(BigInt('0x'+raw)),db=await oracle(value);try{
  expect(db.query('PRAGMA integrity_check').values()).toEqual([['ok']]);expect(db.query('PRAGMA foreign_key_check').values()).toEqual([]);
  for(const [name,count] of Object.entries(laws.tableRowCounts))expect(db.query('SELECT count(*) FROM '+name).values()[0]![0]).toBe(count);
  const query=db.query('SELECT angle_ieee754_bits FROM block5_grip') as unknown as {safeIntegers(v:boolean):void;values():unknown[][]};query.safeIntegers(true);expect(query.values()).toEqual([[BigInt.asIntN(64,BigInt('0x'+raw))]]);
  expect(db.query('SELECT r.native_id,g.grip_kind FROM block5_representation r JOIN block5_grip g ON r.document_id=g.document_id').values()).toEqual([['','unresolved']]);
  expect(await artifact.block5dSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(value);
 }finally{db.close()}
});
test('SQLite Block5d SQL edits change literal semantic entities without resolving native references',async()=>{
 const value=specimen(0n),db=await oracle(value);try{db.exec("UPDATE block5_grip SET grip_kind='other unresolved'; UPDATE block5_representation_tag SET value='edited' WHERE ordinal=1");const query=db.query('UPDATE block5_meta SET description=?');query.run('edited\0😀');value.grips[0]!.gripKind='other unresolved';value.representations[0]!.tags[1]='edited';value.meta.description='edited\0😀';expect(await artifact.block5dSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(value)}finally{db.close()}
});
test('SQLite Block5d structural identities may be independently consistently renumbered',async()=>{
 const value=specimen(0x8000000000000000n),db=await oracle(value);try{
  db.exec('PRAGMA foreign_keys=OFF');for(const name of Object.keys(laws.tableRowCounts))db.exec('UPDATE '+name+' SET id=id+1000');
  db.exec('UPDATE block5_kind SET document_id=document_id+1000; UPDATE block5_part2d SET document_id=document_id+1000; UPDATE block5_part3d SET document_id=document_id+1000; UPDATE block5_representation SET document_id=document_id+1000; UPDATE block5_grip_kind SET document_id=document_id+1000; UPDATE block5_grip SET document_id=document_id+1000; UPDATE block5_compatibility SET document_id=document_id+1000; UPDATE block5_attribute SET document_id=document_id+1000; UPDATE block5_author SET document_id=document_id+1000; UPDATE block5_camera2d SET document_id=document_id+1000; UPDATE block5_camera3d SET document_id=document_id+1000; UPDATE block5_meta SET document_id=document_id+1000; UPDATE block5_representation_tag SET representation_id=representation_id+1000; UPDATE block5_representation_attribute SET representation_id=representation_id+1000');
  expect(db.query('PRAGMA foreign_key_check').values()).toEqual([]);expect(await artifact.block5dSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(value);
 }finally{db.close()}
});
for(const sql of laws.malformedSql)test('SQLite Block5d malformed independent edit '+sql,async()=>{
 const db=await oracle(specimen(0x7ff8000000000001n));try{db.exec(sql);expect(db.query('PRAGMA integrity_check').values()).toEqual([['ok']]);const file=await importSqliteDatabase(db.serialize());await expect(artifact.block5dSnapshotFromSqliteDatabase(file)).rejects.toThrow()}finally{db.close()}
});
test('SQLite Block5d native absence differs from present NaN and empty arrays remain exact',async()=>{
 const value=specimen(0x7ff0000000000001n);value.part2d.radius=null;value.part3d.orientation=null;value.part3d.scale=null;value.representations=[];value.gripKinds=[];value.grips=[];value.compatibility=[];value.attributes=[];value.authors=[];
 const db=await artifact.block5dSnapshotToSqliteDatabase(value);expect(await artifact.block5dSnapshotFromSqliteDatabase(db)).toEqual(value);
});
test('SQLite Block5d schema and complete row limits admit before semantic ownership',async()=>{
 const value=specimen(0n),rows=Object.values(laws.tableRowCounts).reduce((a,n)=>a+n,0);await expect(artifact.block5dSnapshotToSqliteDatabase(value,{maxRows:rows-1})).rejects.toThrow('row limit');expect(await artifact.block5dSnapshotFromSqliteDatabase(await artifact.block5dSnapshotToSqliteDatabase(value,{maxRows:rows}),{maxRows:rows})).toEqual(value);
 for(const restore of [false,true]){const options={maxSchemaBytes:1};await expect(restore?artifact.block5dSnapshotFromSqliteDatabase(await artifact.block5dSnapshotToSqliteDatabase(value),options):artifact.block5dSnapshotToSqliteDatabase(value,options)).rejects.toThrow('schema');}
});
test('SQLite Block5d projection and reconstruction provide interior known work cancellation',async()=>{
 const value=specimen(0n);value.grips=Array.from({length:1024},(_,i)=>({...value.grips[0]!,id:String(i)}));
 const db=await artifact.block5dSnapshotToSqliteDatabase(value);for(const restore of [false,true]){const controller=new AbortController();let known=false;const options={signal:controller.signal,onProgress:(p:{completed:number;total:number})=>{if(p.total>512&&p.completed>=256&&p.completed<p.total){known=true;controller.abort()}}};await expect(restore?artifact.block5dSnapshotFromSqliteDatabase(db,options):artifact.block5dSnapshotToSqliteDatabase(value,options)).rejects.toThrow();expect(known).toBe(true)}
});

import {block5dToJsonText} from '../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts';

import {block5dFromJsonText} from '../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts';

test('SQLite Block5d actual declared JSON boundary preserves every native IEEE word',()=>{
 for(const raw of laws.binary64Bits){const value=specimen(BigInt('0x'+raw)),text=block5dToJsonText(value),wire=JSON.parse(text);expect(wire.grips[0].angle).toEqual({bits:raw});expect(block5dFromJsonText(text)).toEqual(value);}
});

import * as two from '../../../../../../../../../◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';

import * as three from '../../../../../../../../../🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';

test('SQLite shared Block2d snapshot uses the same exact common word and required record authority',()=>{
 const parse=Reflect.get(two,'parseBlock2dSnapshot') as ((v:unknown)=>unknown)|undefined;expect(typeof parse).toBe('function');const base=specimen(0x7ff0000000000001n),value={schema:'literal',nodeKind:base.partKind,presentation:base.part2d,handleKinds:[{id:'',name:'',label:'',color:'',defaultWireKind:''}],handles:[{id:'',handleKind:'unresolved',angle:{bits:0x7ff0000000000001n},radius:{bits:0n}}],compatibility:base.compatibility,attributes:base.attributes,authors:base.authors,camera2d:base.camera2d,meta:base.meta};expect(parse!(value)).toEqual(value);expect(()=>parse!({...value,camera2d:{x:0,y:0,zoom:1}})).toThrow();
});
test('SQLite shared Block3d snapshot retains the literal composed child and exact owned vortex words',()=>{
 const parse=Reflect.get(three,'parseBlock3dSnapshot') as ((v:unknown)=>unknown)|undefined;expect(typeof parse).toBe('function');const base=specimen(0x8000000000000000n),value={schema:'literal',objectKind:base.partKind,representations:base.representations,catalog:{childId:'local',target:{artifactId:'different target',artifactKind:'literal',standard:'',subset:''}},vortexKindExtra:[{id:'',name:'',label:'',color:'',defaultCableKind:''}],vortices:[{id:'',vortexKind:'unresolved',position:base.camera3d.position,direction:base.camera3d.target,radius:{bits:0x8000000000000000n},label:null}],compatibility:base.compatibility,attributes:base.attributes,authors:base.authors,camera3d:base.camera3d,meta:base.meta};expect(parse!(value)).toEqual(value);expect(()=>parse!({...value,vortices:[{...value.vortices[0],radius:1}]})).toThrow();
});

test('SQLite Block5d authored schema and exact row refusal precede untouched domain fields',async()=>{
 const value=specimen(0n);let touched=false;Object.defineProperty(value,'meta',{get(){touched=true;throw new Error('domain touched')}});await expect(artifact.block5dSnapshotToSqliteDatabase(value,{maxRows:15})).rejects.toThrow('row limit');expect(touched).toBe(false);
 const schemaValue=specimen(0n);Object.defineProperty(schemaValue,'representations',{get(){touched=true;throw new Error('domain touched')}});await expect(artifact.block5dSnapshotToSqliteDatabase(schemaValue,{maxSchemaBytes:1})).rejects.toThrow('schema');expect(touched).toBe(false);
});
