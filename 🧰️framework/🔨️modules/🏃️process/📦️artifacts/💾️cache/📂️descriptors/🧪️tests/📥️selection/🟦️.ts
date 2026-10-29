import{withOwnedCommandChannelV1,readOwnedCommandChannelLineV1,writeOwnedCommandChannelV1}from"../../../../../🎛️owned-execution/🟦️.ts";
import{scriptResourceProcessEnvironmentV1}from"../../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{test,expect}from"bun:test";
import Ajv from"ajv";
import ts from"typescript";
import{readFileSync,constants}from"node:fs";
import{mkdtemp,open,lstat,writeFile,rm}from"node:fs/promises";
import{join,dirname}from"node:path";
import corpus from"../🔣️.json"with{type:"json"};
import schema from"../📋️schema.json"with{type:"json"};
import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../../../../../🧪️testing/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{captureOwnedCommandResultV1}from"../../../../../🎛️owned-execution/📥️capture/🟦️.ts";
import{withArtifactDescriptorV1}from"../../../../../../📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import{assertNativeSelectedDescriptorCustodyV1}from"../../🟦️.ts";

declareBuildTestEnvelopeV1({owner:"test:selected-native-search",maximumElapsedMilliseconds:60000,capabilities:{artifacts:true}});
function selector(){const source=readFileSync(new URL("../../../../🏗️native-build/💾️storage/🧹️cleanup/♻️retirement/📋️contract/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("contract.ts",source,ts.ScriptTarget.Latest,true),matches:ts.ConditionalExpression[]=[];const visit=(node:ts.Node):void=>{if(ts.isConditionalExpression(node)&&node.condition.getText(tree)==="mounted"&&node.whenTrue.getText(tree).startsWith('["+f"'))matches.push(node);ts.forEachChild(node,visit);};visit(tree);expect(matches.length).toBe(1);return new Function("mounted","image","return "+matches[0]!.getText(tree))as(mounted:boolean,image:{root:string})=>string[];}
test("neutral selected Native search policy preserves mounted selection and ignores only unmatched recursive paths",()=>{const select=selector(),validate=new Ajv({strict:true}).compile(schema);expect(validate(corpus)).toBe(true);for(const row of corpus)expect(select(row.mounted,{root:row.path})).toEqual(row.arguments);});
test("actual selected Native census keeps complete positive custody when closed paths do not match",async()=>{if(process.platform!=="linux")return;await withBuildTestInvocationV1(async original=>{
 const control=createIssuedResourceControlV1(original,"test:selected-native-search",30000),directory=await mkdtemp(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"sel-")),select=selector();
 try{await withArtifactDescriptorV1(()=>open(directory,constants.O_RDONLY|constants.O_NOFOLLOW|constants.O_NONBLOCK),async handle=>{
  const before=await handle.stat({bigint:true}),guard={selection:"filesystem"as const,pid:process.pid,files:[{fd:handle.fd,path:directory,type:"DIR"as const}]};
  const observe=async(args:string[])=>{const result=await captureOwnedCommandResultV1("lsof",["-nP","-F0pcfatn",...args],dirname(directory),{control,output:{maximumBytes:32768,maximumLines:256}}),opened=await handle.stat({bigint:true}),named=await lstat(directory,{bigint:true});expect([opened.dev,opened.ino,named.dev,named.ino]).toEqual([before.dev,before.ino,before.dev,before.ino]);expect(named.isDirectory()&&!named.isSymbolicLink()).toBe(true);return{...result,reason:"exit"as const};};
  const empty=await observe(select(false,{root:directory}));expect(()=>assertNativeSelectedDescriptorCustodyV1(empty,guard)).not.toThrow();
  const closed=join(directory,"closed");await writeFile(closed,"bounded original fixture");
  const legacy=await observe(["+D",directory]);expect(legacy.status).toBe(1);expect(legacy.stderr).toBe("");expect(legacy.stdout).toBe(empty.stdout);expect(()=>assertNativeSelectedDescriptorCustodyV1(legacy,guard)).toThrow();
  const populated=await observe(select(false,{root:directory}));expect(populated.status).toBe(0);expect(populated.stdout).toBe(empty.stdout);expect(()=>assertNativeSelectedDescriptorCustodyV1(populated,guard)).not.toThrow();
  const missing=await observe(["-Q","-f","--",closed]);expect(missing.status).toBe(0);expect(()=>assertNativeSelectedDescriptorCustodyV1(missing,guard)).toThrow();
  const invalid=await observe(["-Q","--invalid-semio-option"]);expect(invalid.status).toBe(1);expect(invalid.stderr.length).toBeGreaterThan(0);expect(()=>assertNativeSelectedDescriptorCustodyV1(invalid,guard)).toThrow();
  await withArtifactDescriptorV1(()=>open(closed,"r"),async()=>{const extra=await observe(select(false,{root:directory}));expect(extra.status).toBe(0);expect(()=>assertNativeSelectedDescriptorCustodyV1(extra,guard)).toThrow();});

  const program='const fs=require("node:fs"),fd=fs.openSync(process.env.SEMIO_SELECTED_DESCRIPTOR_FILE,"r");console.log("ready");process.stdin.once("data",()=>{fs.closeSync(fd);process.stdin.destroy();});process.stdin.resume();';
  const held=await withOwnedCommandChannelV1(process.execPath,["-e",program],directory,{control,env:scriptResourceProcessEnvironmentV1(control,{...process.env,SEMIO_SELECTED_DESCRIPTOR_FILE:closed}),output:{maximumBytes:32768,maximumLines:256}},async channel=>{expect(await readOwnedCommandChannelLineV1(channel)).toBe("ready");const foreign=await observe(select(false,{root:directory}));expect(foreign.status).toBe(0);expect(()=>assertNativeSelectedDescriptorCustodyV1(foreign,guard)).toThrow("Exact own selected Native process required");await writeOwnedCommandChannelV1(channel,{version:1,text:"release\n"});expect(await readOwnedCommandChannelLineV1(channel)).toBeNull();});expect(held.capture.status).toBe(0);
  expect(()=>assertNativeSelectedDescriptorCustodyV1(populated,{...guard,pid:process.pid+1})).toThrow();
  expect(()=>assertNativeSelectedDescriptorCustodyV1({...populated,stdout:populated.stdout.slice(0,-1)},guard)).toThrow();
 });}finally{await rm(directory,{recursive:true,force:true});}
});},45000);
