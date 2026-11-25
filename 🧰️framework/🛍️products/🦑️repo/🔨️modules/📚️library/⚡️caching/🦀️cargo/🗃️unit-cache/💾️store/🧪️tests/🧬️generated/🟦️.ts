/** 🧬️ Tests for the generated-source directory (`OUT_DIR`) of a unit: the dependency file names it with Cargo's machine-specific hash, so it is stored and restored through a token. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {generatedNeutralV1,generatedRestoreV1,lookupUnitV1,publishUnitV1,restoreUnitV1} from "../../🟦️.ts";

test("two machines whose Cargo hash directories differ publish one entry and each restores its own directory (oracle: Python text replacement)",()=>{
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"unit-generated-"))),root=join(base,"cache");
 try{
  const compile=(hash:string)=>{
   const work=join(base,"w"+hash),generated=join(work,"build","serde","0123"+hash,"out");
   mkdirSync(generated,{recursive:true});mkdirSync(join(work,"deps"),{recursive:true});writeFileSync(join(generated,"private.rs"),"pub fn x(){}\n");writeFileSync(join(work,"lib.rs"),"include!(concat!(env!(\"OUT_DIR\"),\"/private.rs\"));\n");
   const lib=join(work,"deps","libs.rlib"),info=join(work,"deps","s.d");writeFileSync(lib,"object bytes");
   writeFileSync(info,`${lib}: ${join(work,"lib.rs")} ${join(generated,"private.rs")}\n`);
   return{work,generated,lib,info};
  };
  const [a,b]=[compile("aaaaaaaaaaaa"),compile("bbbbbbbbbbbb")],key="cd"+"0".repeat(62);
  const first=publishUnitV1(root,{key,crate:"s",cwd:a.work,outDir:join(a.work,"deps"),generated:a.generated,laneRoot:a.work,outputs:[a.lib,a.info],stdout:"",stderr:"",stampDirectory:undefined});
  const second=publishUnitV1(root,{key,crate:"s",cwd:b.work,outDir:join(b.work,"deps"),generated:b.generated,laneRoot:b.work,outputs:[b.lib,b.info],stdout:"",stderr:"",stampDirectory:undefined});
  expect(first!.outputs.find(row=>row.name==="s.d")!.sha256,"the dependency file is the same bytes whichever hash directory built it").toBe(second!.outputs.find(row=>row.name==="s.d")!.sha256);
  expect(first!.dependencies.map(row=>row.path)).toEqual(second!.dependencies.map(row=>row.path));
  const other=join(base,"restored","deps"),target=join(base,"restored","build","serde","0123cccccccccccc","out");mkdirSync(other,{recursive:true});mkdirSync(target,{recursive:true});writeFileSync(join(target,"private.rs"),"pub fn x(){}\n");writeFileSync(join(base,"restored","lib.rs"),"include!(concat!(env!(\"OUT_DIR\"),\"/private.rs\"));\n");
  const found=lookupUnitV1(root,key,{cwd:join(base,"restored"),laneRoot:join(base,"restored"),environment:{OUT_DIR:target},stampDirectory:undefined});
  expect(found,"the entry validates against the OUT_DIR of a third machine").toBeDefined();
  restoreUnitV1(root,found!,{outDir:other,laneRoot:join(base,"restored"),stampDirectory:undefined,generated:target});
  const text=readFileSync(join(other,"s.d"),"utf8");
  expect(text).toContain(join(target,"private.rs"));expect(text).not.toContain("0123aaaaaaaaaaaa");
  const oracle=spawnSync("python3",["-I","-c","import sys;t=sys.stdin.read();print(t.replace(sys.argv[1],sys.argv[2]).replace(sys.argv[3],sys.argv[4]).replace(sys.argv[5],sys.argv[6]),end='')",a.generated,target,join(a.work,"deps"),other,a.work,join(base,"restored")],{input:readFileSync(a.info,"utf8"),encoding:"utf8"});
  expect(oracle.status,oracle.stderr).toBe(0);expect(text).toBe(oracle.stdout);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the token round trip is the identity, and no OUT_DIR leaves the text as it is",()=>{
 const text="/l/build/x/0011223344556677/out/a.rs: /l/build/x/0011223344556677/out/a.rs\n";
 expect(generatedRestoreV1(generatedNeutralV1(text,"/l/build/x/0011223344556677/out"),"/l/build/x/0011223344556677/out")).toBe(text);
 expect(generatedNeutralV1(text,"/l/build/x/0011223344556677/out")).toBe("@@semio-gen@@/a.rs: @@semio-gen@@/a.rs\n");
 expect(generatedNeutralV1(text,undefined)).toBe(text);expect(generatedNeutralV1(text,"")).toBe(text);expect(generatedRestoreV1(text,undefined)).toBe(text);
});
