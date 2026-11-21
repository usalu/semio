import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import fixture from "./🔣️.json" with {type:"json"};
import {cacheLedgerV1,cacheSizeChargeV1,readCacheSizeLedgerV1} from "../../🟦️.ts";

test("cached sizes charge only records the current ledger owns",()=>{
 const policy=cacheLedgerV1(),db=new Database(":memory:");
 try{
  db.exec(`
   create table roots(id text);
   create table areas(id text);
   create table homes(root text,path text,category text);
   create table ignored(root text,path text);
   create table readings(key text,bytes integer,category text,area text,root text,path text);
  `);
  for(const [root,row] of Object.entries(policy.roots)){
   db.run("insert into roots values (?)",[root]);
   for(const path of row.ignored)db.run("insert into ignored values (?,?)",[root,path]);
  }
  for(const row of policy.areas){
   db.run("insert into areas values (?)",[row.id]);
   if(row.directory!==undefined&&row.kind!=="nested")db.run("insert into homes values (?,?,'area')",[row.root,row.directory]);
  }
  for(const row of policy.retirements)db.run("insert into homes values (?,?,'retired')",[row.root,row.directory]);
  for(const row of fixture.records)db.run("insert into readings values (?,?,?,?,?,?)",[row.key,row.usedBytes,row.category,row.area??null,row.root??null,row.path??null]);
  const selected=db.query<{key:string;bytes:number},[]>(`
   select r.key,r.bytes from readings r
   where (r.category='area' and exists(select 1 from areas a where a.id=r.area))
      or (r.category='retired' and exists(select 1 from homes h where h.root=r.root and h.category='retired' and h.path=r.path))
      or (r.category='undeclared' and exists(select 1 from roots n where n.id=r.root)
          and instr('/'||r.path||'/','/../')=0
          and not exists(select 1 from ignored i where i.root=r.root and i.path=r.path)
          and not exists(select 1 from homes h where h.root=r.root and
                         (substr(r.path||'/',1,length(h.path)+1)=h.path||'/' or substr(h.path||'/',1,length(r.path)+1)=r.path||'/')))
   order by r.key
  `).all();
  expect(selected.map(row=>row.key)).toEqual(fixture.expected);
  const raw={version:1,records:Object.fromEntries(fixture.records.map(row=>[row.key,{usedBytes:row.usedBytes,partial:false,atMs:7}]))};
  const readings=readCacheSizeLedgerV1(raw),accepted=new Map(selected.map(row=>[row.key,row.bytes]));
  expect(Object.keys(readings.records)).toHaveLength(fixture.records.length);
  expect(cacheSizeChargeV1(policy,readings)).toBe(selected.reduce((total,row)=>total+row.bytes,0));
  for(const [key,measurement] of Object.entries(readings.records)){
   expect(cacheSizeChargeV1(policy,{version:1,records:{[key]:measurement}}),key).toBe(accepted.get(key)??0);
  }
 }finally{db.close();}
});
