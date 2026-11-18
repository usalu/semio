/** 🤝️ Contract: the observations an inference published are handed to its own caller, not looked up again by the path of its record: another look or run of the same owner that starts or releases in between (the take pass beside the owner loop) cannot take them away, and when they are lost anyway the failure names the owner and the record. */
import{afterAll,expect,spyOn,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import*as fs from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{observeInferenceSnapshotV1,observeProjectInferenceV1,observedFs,releaseProjectInferenceRetentionV1}from"../../🟨️.mjs";

const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

test("a release of the same record path between the publication and the return does not lose the observations",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"obs-capture-")));roots.push(root);
 mkdirSync(join(root,"src"),{recursive:true});writeFileSync(join(root,"src","a.txt"),"a\n");await new Promise(resolve=>setTimeout(resolve,2300));
 const cachePath=join(root,"np","alpha.json");let released=0;
 const real=fs.mkdirSync.bind(fs),spy=spyOn(fs,"mkdirSync").mockImplementation(((path:string,options?:unknown)=>{if(String(path)===join(root,"np")&&released===0){released++;releaseProjectInferenceRetentionV1(cachePath);}return real(path as never,options as never);}) as never);
 try{
  const value=await observeInferenceSnapshotV1(()=>observeProjectInferenceV1({cachePath,key:"k",owner:"libs/alpha",stamps:[]},async()=>observedFs.readFileSync(join(root,"src","a.txt"),"utf8")));
  expect(value).toBe("a\n");expect(released).toBe(1);
 }finally{spy.mockRestore();}
},20000);
