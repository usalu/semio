import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {externIdentityV1,markExternIdentityV1,markProducerKeyV1,producerKeyV1} from "../🟦️.ts";
import {buildRunKeyV1,buildScriptKeyV1,markBuildRunKeyV1,markBuildScriptIdentityV1} from "../../🏗️buildscript/🔑️run-key/🟦️.ts";

const sha=(bytes:string)=>createHash("sha256").update(bytes).digest("hex");
test("a library enters by the identity of its unit and its crate hash, only while the file is the one that was marked",()=>{
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"extern-")));
 try{
  const library=join(base,"libpm-abc.dylib"),rlib=join(base,"libx-abc.rlib"),hash="f".repeat(32),other="e".repeat(32);
  writeFileSync(library,"one");writeFileSync(rlib,"r");
  expect(externIdentityV1(library,sha("one"),undefined)).toBeUndefined();
  markExternIdentityV1([library,rlib],"a".repeat(64),[],undefined,()=>hash);
  const first=externIdentityV1(library,sha("one"),undefined);
  expect(first).toMatch(/^unit:[0-9a-f]{64}:[0-9a-f]{32}$/u);
  expect(externIdentityV1(rlib,sha("r"),undefined),"only dynamic libraries").toBeUndefined();
  markExternIdentityV1([library],"a".repeat(64),[],undefined,()=>other);
  expect(externIdentityV1(library,sha("one"),undefined),"the crate hash is part of the identity: another generation of one unit is another identity").not.toBe(first);
  markExternIdentityV1([library],"a".repeat(64),[],undefined,()=>undefined);
  expect(externIdentityV1(library,sha("one"),undefined),"a library whose crate hash is unknown keeps its last identity only while it names the same bytes").toBeDefined();
  writeFileSync(library,"two");
  expect(externIdentityV1(library,sha("two"),undefined),"a changed library is not trusted").toBeUndefined();
  markExternIdentityV1([library],"b".repeat(64),[],undefined,()=>undefined);
  expect(externIdentityV1(library,sha("two"),undefined),"no crate hash, no identity: the bytes count").toBeUndefined();
  markExternIdentityV1([library],"b".repeat(64),[],undefined,()=>hash);
  const second=externIdentityV1(library,sha("two"),undefined);
  expect(second).not.toBe(first);
  markExternIdentityV1([library],"b".repeat(64),["/x"],undefined,()=>hash);
  expect(externIdentityV1(library,sha("two"),undefined),"real paths are part of the identity").not.toBe(second);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a library, a build script executable and an OUT_DIR name the entry that produced them, only while the bytes are the ones that were marked",()=>{
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"producer-")));
 try{
  const key="c".repeat(64),library=join(base,"libpm-abc.dylib"),rmeta=join(base,"libx-abc.rmeta"),text=join(base,"x.d"),script=join(base,"build_script_build-abc.real"),outDir=join(base,"build","pkg","out");
  writeFileSync(library,"one");writeFileSync(rmeta,"m");writeFileSync(text,"d");writeFileSync(script,"exe");
  expect(producerKeyV1(library,sha("one"))).toBeUndefined();
  markProducerKeyV1([library,rmeta,text],key,undefined);
  expect(producerKeyV1(library,sha("one")),"a proc-macro library names its producer").toBe(key);
  expect(producerKeyV1(rmeta,sha("m")),"metadata too").toBe(key);
  expect(producerKeyV1(text,sha("d")),"only libraries").toBeUndefined();
  writeFileSync(rmeta,"rebased");
  expect(producerKeyV1(rmeta,sha("rebased")),"a rebased file is not trusted until it is marked again").toBeUndefined();
  markProducerKeyV1([rmeta],key,undefined);expect(producerKeyV1(rmeta,sha("rebased"))).toBe(key);
  markBuildScriptIdentityV1(script,"p".repeat(64),[],sha("exe"),key);
  expect(buildScriptKeyV1(script,sha("exe"))).toBe(key);expect(buildScriptKeyV1(script,sha("other"))).toBeUndefined();
  mkdirSync(outDir,{recursive:true});expect(buildRunKeyV1(outDir)).toBeUndefined();expect(buildRunKeyV1(undefined)).toBeUndefined();
  markBuildRunKeyV1(outDir,"d".repeat(64));expect(buildRunKeyV1(outDir)).toBe("d".repeat(64));
 }finally{rmSync(base,{recursive:true,force:true});}
});
