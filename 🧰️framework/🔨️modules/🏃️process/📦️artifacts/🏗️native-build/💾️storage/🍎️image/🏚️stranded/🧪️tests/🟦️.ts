/** 🏚️ Tests for the recovery of an image attached at a foreign mount point. */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{join}from"node:path";
import{testPythonV1}from"../../../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import texts from"../../../../../../📖️texts/🔣️.json"with{type:"json"};
import{nativeImageInformationV1}from"../../📥️information/🟦️.ts";
import fixture from"./🔣️.json"with{type:"json"};
import{nativeImageStrandedMountV1,recoverNativeImageStrandedV1}from"../🟦️.ts";
const oracle=()=>{
 const python=testPythonV1();
 if(python===undefined)throw Error("Python required by the stranded oracle");
 const result=spawnSync(python,["-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"🔣️.json")],{encoding:"utf8"});
 if(result.status!==0)throw result.error??Error(result.stderr);
 return JSON.parse(result.stdout) as unknown[];
};
test("the decision equals the fixture and the independent plist oracle on every case",()=>{
 const ours=fixture.cases.map(row=>nativeImageStrandedMountV1(nativeImageInformationV1(row.plist),fixture.image,fixture.root)??null);
 expect(ours).toEqual(fixture.cases.map(row=>row.expected));
 expect(oracle()).toEqual(ours);
});
test("recovery detaches exactly the stranded whole disk once and leaves canonical, absent and foreign images alone",async()=>{
 for(const row of fixture.cases){
  const detached:string[]=[],stranded=await recoverNativeImageStrandedV1({inventory:async()=>nativeImageInformationV1(row.plist),detach:async device=>{detached.push(device);}},fixture.image,fixture.root);
  expect(detached,row.name).toEqual(row.expected===null?[]:[row.expected.device]);
  expect(stranded??null,row.name).toEqual(row.expected);
 }
});
test("a busy device refuses in both languages naming the image, the mount, the device and the canonical mount point",async()=>{
 const row=fixture.cases[0]!,cause=Error("hdiutil: couldn't unmount - Resource busy");
 const error=await recoverNativeImageStrandedV1({inventory:async()=>nativeImageInformationV1(row.plist),detach:async()=>{throw cause;}},fixture.image,fixture.root).catch(failure=>failure);
 expect(error.cause).toBe(cause);
 expect(error.key).toBe("process.nativeBuild.image-stranded");
 expect(error.values).toEqual({image:fixture.image,device:"/dev/disk4",mountPoints:row.expected!.mountPoints.join(", "),canonical:fixture.root});
 const entry=texts.messages["nativeBuild.image-stranded"];
 for(const text of[entry.en,entry.de])for(const name of["image","device","mountPoints","canonical"])expect(text).toContain("{"+name+"}");
});
