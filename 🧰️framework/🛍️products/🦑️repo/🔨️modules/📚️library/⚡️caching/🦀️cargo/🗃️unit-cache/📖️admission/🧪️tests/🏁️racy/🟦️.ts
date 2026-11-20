import{test,expect}from"bun:test";
import{mkdtempSync,rmSync,writeFileSync,readFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{recallAdmissionV1,rememberAdmissionV1,type AdmissionStampV1}from"../../🟦️.ts";
import corpus from"./🔣️.json"with{type:"json"};

/** 🏁️ A file touched during the admission is remembered with the digest of the bytes read; the frozen stamp below stands for a coarse clock that cannot tell a rewrite apart, so only the digest can. */
test("a recently touched file is remembered with its digest and recalled only while its bytes still hash to it",()=>{
 for(const row of corpus.cases){const directory=mkdtempSync(join(tmpdir(),"rf-racy-")),source=join(directory,"lib.rs"),root=join(directory,"memo");
  try{writeFileSync(source,row.bytes);const digest=new Bun.CryptoHasher("sha256").update(row.bytes).digest("hex"),at=String(BigInt(Date.now())*1_000_000n),frozen=(path:string):AdmissionStampV1=>({path,present:true,dev:"1",ino:"2",size:"14",mtimeNs:at,ctimeNs:at});
   const result={inputs:[{path:source,digest,bytes:row.bytes.length}],cfg:["unix"],environment:null,probed:[source],startedMs:Date.now()};
   expect(rememberAdmissionV1(root,"ab"+row.name,result,frozen),row.name).toBe(true);
   writeFileSync(source,row.after);
   expect(recallAdmissionV1(root,"ab"+row.name,frozen)!==undefined,row.name).toBe(row.recalls);
   expect(rememberAdmissionV1(root,"cd"+row.name,{...result,inputs:[]},frozen),"racy without a digest").toBe(false);
  }finally{rmSync(directory,{recursive:true,force:true});}}
});

test("a racy stamp is stored as such and old files keep plain stamps",()=>{
 const directory=mkdtempSync(join(tmpdir(),"rf-racy-")),fresh=join(directory,"fresh.rs"),old=join(directory,"old.rs"),root=join(directory,"memo");
 try{writeFileSync(fresh,"a");writeFileSync(old,"b");const now=BigInt(Date.now())*1_000_000n,stamp=(path:string):AdmissionStampV1=>({path,present:true,dev:"1",ino:path===fresh?"2":"3",size:"1",mtimeNs:String(path===fresh?now:now-3_600_000_000_000n),ctimeNs:String(path===fresh?now:now-3_600_000_000_000n)}),sha=(text:string)=>new Bun.CryptoHasher("sha256").update(text).digest("hex");
  expect(rememberAdmissionV1(root,"ef01",{inputs:[{path:fresh,digest:sha("a"),bytes:1},{path:old,digest:sha("b"),bytes:1}],cfg:[],environment:null,probed:[fresh,old],startedMs:Date.now()},stamp)).toBe(true);
  const memo=recallAdmissionV1(root,"ef01",stamp)!;expect(memo.stamps.filter(row=>row.racy).map(row=>row.path)).toEqual([fresh]);expect(readFileSync(join(root,"a","ef","ef01.json"),"utf8")).toContain('"racy":true');}finally{rmSync(directory,{recursive:true,force:true});}
});
