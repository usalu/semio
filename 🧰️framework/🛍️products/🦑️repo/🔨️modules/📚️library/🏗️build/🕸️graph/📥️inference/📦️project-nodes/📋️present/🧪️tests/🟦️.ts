/** 📋️ Contract: one directory listing tells how many owners of a wave already have a stored record. The key of a record is a pure function of the owner root (equal to an independent Python implementation on seeded roots, including emoji paths), a record of any age, writer or machine counts as present, a leftover temporary file or a marker does not, 5101 owners are counted in well under a second, and the line is bilingual.
 * @see ../🟨️.mjs */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{spawnSync}from"node:child_process";
import{ownerRecordKeyV1,ownerRecordsPresentTextV1,ownerRecordsPresentV1}from"../🟨️.mjs";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};

const roots:string[]=[],oracle=join(import.meta.dir,"🐍️.py");
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const directory=()=>{const root=mkdtempSync(join(realpathSync(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"present-"));roots.push(root);return root;};

test("the key of a record is the first 24 hex digits of the sha256 of the root and equals the fixture",()=>{
 for(const row of fixture.keys)expect(ownerRecordKeyV1(row.root),row.root).toBe(row.key);
 expect(schema.definitions.Policy.const.keyHexDigits).toBe(24);
});

test("the owners present and the count equal the independent Python model on 60 seeded directories",()=>{
 let seed=11;const random=(n:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%n;};
 const cases:unknown[]=[],actual:unknown[]=[];
 for(let index=0;index<60;index++){
  const owners=Array.from({length:1+random(40)},(_,at)=>["repo","🧰️framework/🛍️products/📦️p"+index+"/"+at,"✏️s/🔌️plugins/x"+random(1000)][random(3)]+"/"+index+"-"+at),dir=directory(),names:string[]=[];
  for(const owner of owners){const mode=random(6);const key=ownerRecordKeyV1(owner);
   if(mode<3){names.push(key+".json");writeFileSync(join(dir,key+".json"),"x");}
   else if(mode===3){names.push(key+".json.fleet");writeFileSync(join(dir,key+".json.fleet"),"x");}
   else if(mode===4){names.push(key+".json.123.456.tmp");writeFileSync(join(dir,key+".json.123.456.tmp"),"x");}}
  const census=ownerRecordsPresentV1(dir,owners);cases.push({roots:owners,names});actual.push([...census.present].sort());expect(census.total).toBe(owners.length);
 }
 const file=join(directory(),"cases.json");writeFileSync(file,JSON.stringify({cases}));
 const run=spawnSync("python3",["-I",oracle,file],{encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);expect(actual).toEqual(JSON.parse(run.stdout));
});

test("5101 owners are counted from one listing in well under a second, and a missing directory holds nothing",()=>{
 const dir=directory(),owners=Array.from({length:5101},(_,index)=>"🧰️framework/🔨️modules/owner-"+index);
 for(const owner of owners.slice(0,3000))writeFileSync(join(dir,ownerRecordKeyV1(owner)+".json"),"x");
 const started=performance.now(),census=ownerRecordsPresentV1(dir,owners),milliseconds=performance.now()-started;
 expect(census.present.size).toBe(3000);expect(census.total).toBe(5101);expect(milliseconds).toBeLessThan(1000);
 expect(ownerRecordsPresentV1(join(dir,"absent"),owners).present.size).toBe(0);
 mkdirSync(join(dir,"sub"));expect(ownerRecordsPresentV1(dir,[]).total).toBe(0);
});

test("the line names the counts in English and German and falls back to English",()=>{
 for(const row of fixture.texts)expect(ownerRecordsPresentTextV1(row.language,{present:new Set(Array.from({length:row.present},(_,index)=>"r"+index)),total:row.total}),row.language).toBe(row.line);
});
