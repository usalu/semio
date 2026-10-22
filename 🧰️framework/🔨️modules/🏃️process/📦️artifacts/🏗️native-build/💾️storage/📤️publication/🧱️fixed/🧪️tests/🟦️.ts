import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import ts from"typescript";
import{readFileSync}from"node:fs";
import{join,resolve,dirname}from"node:path";
import{readNativeFixedPublicationMappingV1}from"../🟦️.ts";
import{withBuildTestInvocationV1}from"../../../../../../🧪️testing/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{checkResourceLeaseControl}from"../../../../../../🔒️leases/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import imageSchema from"../../../🧱️fixed/🧬️schema/🔣️.json";
import mountSchema from"../../../🌐️platform/📋️contract/📂️mount/🧬️schema/🔣️.json";
import{nativeDirectoryProtocolV1}from"../../../📁️directory/🟦️.ts";
import schema from"../🧬️schema/🔣️.json";
import corpus from"./🔣️.json";
const protocol=nativeDirectoryProtocolV1();
function actual(name:string,ports:Record<string,unknown>,path=resolve(import.meta.dir,"../🟦️.ts")):Function{const source=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),fn=source.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text===name);if(!fn)throw Error("Actual function required");return new Function(...Object.keys(ports),ts.transpileModule(fn.getText(source).replace(/^export\s+/u,"")+";return "+name+";",{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText)(...Object.values(ports));}
const mapping=()=>({kind:"directory-partition",anchor:[1,3,159],volume:{dev:"1",ino:"4",mode:16877}});

test("Fixed publication complete mapping DATA matches independent Ajv and SQLite on both directory hosts",()=>{
 const ajv=new Ajv({strict:true}),db=new Database(":memory:");try{for(const row of corpus.mappings){expect(ajv.validate(schema.definitions.Case,row),row.id).toBe(true);const image="/fixture/fixed.dir",value:Record<string,unknown>=mapping();
  if(row.change==="backing")value.anchor=[1,3];if(row.change==="image")value.kind="loop-mount";if(row.change==="volume")value.volume={dev:"1",ino:"0",mode:16877};if(row.change==="extra")value.foreign=1;
  let accepted=false;try{const output=readNativeFixedPublicationMappingV1(value,image,33554432);expect(JSON.parse(output)).toEqual(value);expect(ajv.validate(schema.definitions.DirectoryMapping,JSON.parse(output))).toBe(true);accepted=true;}catch{}
  const oracle=db.query("select json_extract(?1,'$.kind')='directory-partition' and json_array_length(?1,'$.anchor')=3 and json_extract(?1,'$.volume.ino') glob '[1-9]*' and (select count(*) from json_each(?1))=3 accepted").get(JSON.stringify(value))as{accepted:number};
  expect(accepted,row.id).toBe(row.accepted);expect(Boolean(oracle.accepted),row.id).toBe(row.accepted);}}finally{db.close();}
});

test("Fixed publication actual observation bodies reject duplicate association and uncertain absence",()=>withBuildTestInvocationV1(async original=>{
 const control=createIssuedResourceControlV1(original,"publication:fixed-observation",15000),db=new Database(":memory:");
 try{for(const row of corpus.observations){expect(new Ajv({strict:true}).validate(schema.definitions.ObservationCase,row),row.id).toBe(true);
  let checks=0;const image="/fixture/"+protocol.anchor,inactive=row.effect==="inactive",backing={isFile:()=>true,isSymbolicLink:()=>false,nlink:1,dev:1,ino:3,mtimeMs:1,ctimeMs:1,size:33554432,blocks:65537},mounted={isDirectory:()=>true,isSymbolicLink:()=>false,dev:inactive?1:2,ino:4},
  text=inactive||row.effect==="missing-association"?"":row.effect==="duplicate"?"attached\nattached":row.effect==="foreign-text"?"foreign":"attached",observe=()=>{checkResourceLeaseControl(control);checks++;if(row.effect==="observer"&&checks===3)throw undefined;},
  ports={process:{platform:row.platform},join,resolve,nativeImageRootV1:()=>"/fixture/m",lstatSync:(path:string)=>path===image?backing:mounted,nativeImageAttachedV1:()=>!inactive,nativeImageFilesystemBytesV1:()=>33554432,nativeImageBackingFilenameV1:()=>protocol.anchor,nativeDirectoryMappingV1:()=>JSON.stringify(mapping()),commandSchema:{definitions:{Protocol:{const:{maximumOutputBytes:1048576}}}},readNativeFixedPublicationMappingV1},run=actual("observeNativeFixedPublicationV1",ports);
  let accepted=false,kind:string|undefined;try{kind=(await run({directory:"/fixture",filesystemBytes:33554432,maximumPhysicalBytes:34603008,control},"information",text,observe)).kind;accepted=true;}catch{}
  const oracle=db.query("select (?1=1 and ?2='') or (?1=0 and ?2='attached') accepted").get(inactive?1:0,text)as{accepted:number};
  expect(accepted,row.id).toBe(row.accepted);if(row.effect!=="observer")expect(Boolean(oracle.accepted),row.id).toBe(row.accepted);if(accepted)expect(kind,row.id).toBe(row.kind);}}finally{db.close();}
}));

test("Fixed publication command plans spawn the owned partition script on both directory hosts and the receipt carries the host platform",()=>withBuildTestInvocationV1(async original=>{
 const phase=createIssuedResourceControlV1(original,"publication:fixed-command-data",15000),db=new Database(":memory:");
 try{for(const platform of["darwin","linux","win32"]){const options={directory:"/fixture",filesystemBytes:33554432,maximumPhysicalBytes:34603008,control:phase},
  spec=(options:unknown,action:string)=>({program:"/fixture/bun",args:["/fixture/script.ts",action,(options as{directory:string}).directory],cwd:"/",env:{}}),run=actual("nativeFixedPublicationCommandV1",{process:{platform},nativeFixedImageCommandSpecV1:spec});
  for(const action of schema.definitions.Protocol.const.actions){const command=run(options,action,"mounted")as{program:string;args:string[]},expected=(db.query("select case when ? in ('information','inactive','verify') then 'inactive' when ?='detach' then 'deactivate' else 'activate' end program").get(action,action)as{program:string}).program;expect(command.args,platform+":"+action).toEqual(["/fixture/script.ts",expected,"/fixture"]);}
  expect(()=>run(options,"attach","other")).toThrow("Explicit publication initial state");expect(()=>actual("nativeFixedPublicationCommandV1",{process:{platform:"freebsd"},nativeFixedImageCommandSpecV1:spec})(options,"attach","mounted")).toThrow("Actual Fixed publication platform");
  const create=actual("createNativeFixedPublicationReceiptV1",{process:{platform},validateJsonSchemaSubset,imageSchema,mountSchema}),underlying={dev:"1",ino:"4",mode:16832},receipt=create(options,{backing:{dev:"1",ino:"3",size:33554432}},underlying),ajv=new Ajv({strict:true});
  ajv.addSchema(mountSchema);expect(ajv.validate(imageSchema.definitions.ImageReceipt,receipt)).toBe(true);underlying.ino="99";expect(receipt.mountDirectory.ino).toBe("4");expect(receipt.platform).toBe(platform);expect("mapping"in receipt).toBe(false);expect(receipt.mountDirectory).toEqual({dev:"1",ino:"4",mode:16832});}}finally{db.close();}
}));
