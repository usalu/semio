import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import ts from"typescript";
import{readFileSync,mkdtempSync,writeFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join,dirname,relative,isAbsolute,posix}from"node:path";
import{isClosedOwnerRegistryInvalidationV1,vanishedOwnerRootsV1,withOwnerRegistryCustodyV1}from"../../../../📥️load/🧭️registry-follow/🟦️.ts";
import{buildInputSourceRoleV1,buildNativeSourceObservationV1}from"../../../../🚪️admission/🟦️.ts";
import admissionSchema from"../../../../../🧬️schema/🚪️admission.json";
import schema from"./🧬️schema/🔣️.json";
import fixture from"./🔣️.json";
import generatorSchema from"./🧬️schema/🏭️.json";
import generatorFixture from"./🏭️.json";
import{evaluateExtractedFunctionV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧩️extracted-function/🟦️.ts";
test("actual owner discovery constructs local native targets once per observation and lets Print catalog targets win as project assembly does",async()=>{
 const source=ts.createSourceFile("owner.mjs",readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),declaration=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="discoverBuildOwnerAdmissionFactsV1");if(!declaration)throw Error("Actual owner discovery required");expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);const db=new Database(":memory:");
 try{for(const row of fixture.cases){let calls=0,physicalReads=0;const owner={name:"fixture",root:"owner",manifest:"owner/"+(row.cargo?"Cargo.toml":"📋️project.json"),packages:[]},declaration={name:owner.name,package:{name:owner.name},targets:row.authored?{build:{marker:"authored"}}:{}},context={workspaceRoot:"/workspace"},registry={owners:[owner],catalogProviders:[]},ports={libraryBootstrap:Promise.resolve(),join,dirname,relative,isAbsolute,buildPath:(value:string)=>value.replaceAll("\\","/"),readPhysicalSource:()=>{physicalReads++;return Buffer.from(JSON.stringify(declaration));},readToml:()=>declaration,createScriptInputCache:()=>({}),cargoTargets:async()=>{calls++;return row.native?{build:{marker:"native"}}:{};},componentBuildTargets:()=>row.component?{build:{marker:"component"}}:{},printDocumentTargets:async()=>row.print?{build:{marker:"print"},"build-flyer":{marker:"print"}}:{},existsSync:()=>false,readTaxonomyProjectionV1:()=>({generatorContracts:{}}),yieldInferenceV1:async()=>{},buildInputSourceRoleV1,buildNativeSourceObservationV1,ownerAdmissionSchema:admissionSchema,readBuildGeneratedCompileOutputV1:()=>undefined,askOwnerRegistryV1:()=>null,observedOwnerRegistryViewV1:()=>({byName:new Map(),byPackage:new Map(),byRoot:new Map(),relativeProject:()=>undefined}),parserTypescriptV1:()=>({sys:{}}),observedTypeScriptHost:()=>({})};
  const execute=evaluateExtractedFunctionV1(declarationNodeText(source),"discoverBuildOwnerAdmissionFactsV1",ports),actual=await execute(owner,[],registry,context);const expected=db.query("select case when ?3 then 'print' when ?1 then 'component' when ?2 then 'authored' else 'native' end marker, 1 calls").get(Number(row.component),Number(row.authored),Number(row.print))as{marker:string;calls:number};expect(calls,row.id).toBe(expected.calls);expect(actual.project.targets.build.marker,row.id).toBe(expected.marker);expect(expected.marker,row.id).toBe(row.expected);expect(Boolean(actual.project.targets["build-flyer"]),row.id).toBe(row.print);expect(actual.files,row.id).toEqual([owner.manifest]);expect(actual.dependencies,row.id).toEqual([]);expect(actual.dependencyInputs,row.id).toEqual({});expect(physicalReads,row.id).toBe(1);
 }}finally{db.close();}
 function declarationNodeText(file:ts.SourceFile){const node=file.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="discoverBuildOwnerAdmissionFactsV1")!;return node.getText(file).replace(/^export\s+/,"").replace("createRequire(import.meta.url)","createRequire('fixture-module')");}
});

test("generated import lookup visits only matching authored roots and checks physical alternatives anew",()=>{
 const file=ts.createSourceFile("owner.mjs",readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
 const start=file.statements.findIndex(node=>ts.isVariableStatement(node)&&node.declarationList.declarations[0].name.getText(file)==="UNSAFE_GENERATED_PATH_V1");
 const end=file.statements.findIndex(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="readBuildGeneratedCompileOutputV1");
 if(start<0||end<start)throw Error("Actual generated import resolver required");
 const declarations=file.statements.slice(start,end+1).map(node=>node.getText(file).replace(/^export\s+/,"")).join("\n");
 expect(new Ajv({strict:true}).validate(generatorSchema,generatorFixture)).toBe(true);
 const workspace=join(process.cwd(),"fixture"),db=new Database(":memory:"),resolution=admissionSchema.$defs.generatedImportResolution.const;
 db.exec("create table output(path text); create table physical(path text)");
 try{for(const row of generatorFixture.cases){
  const physical=new Set("physical" in row?row.physical:[]);
  let unrelatedReads=0,manifestReads=0;
  const unrelated=Array.from({length:generatorFixture.unrelatedRoots},(_,index)=>({inclusion:"ignored",get path(){unrelatedReads++;return "unrelated/"+index+".ts";}}));
  const outputRoots=[...unrelated,...row.outputs.map(path=>({inclusion:"ignored",path}))];
  const contracts={generator:{ownership:"owned",target:"fixture:generate",ownerPath:"made",outputRoots}};
  const buildPath=(path:string)=>path.replaceAll("\\","/");
  const ports={
   join,relative,isAbsolute,posix,buildPath,ownerAdmissionSchema:admissionSchema,PROJECT_BASENAME:"📋️project.json",
   existsSync:(path:string)=>physical.has(buildPath(relative(workspace,path))),
   lstatSync:()=>({isDirectory:()=>false}),
   readPhysicalSource:()=>{
    manifestReads++;
    return Buffer.from(JSON.stringify({name:"fixture",targets:{generate:{}}}));
   },
  };
  const create=evaluateExtractedFunctionV1(`function createGeneratedResolver(){${declarations}\nreturn readBuildGeneratedCompileOutputV1;}`,"createGeneratedResolver",ports);
  const resolve=create(),request=join(workspace,row.request);
  const outcome=()=>{
   try{return resolve(request,workspace,contracts)?.output??"";}
   catch(error){
    const message=(error as Error).message;
    if(message.startsWith("Canonical generated import path required"))return "unsafe-path";
    if(message==="Ambiguous generated compile output producer")return "ambiguous-producer";
    if(message==="Ambiguous generated compile output resolution")return "ambiguous-resolution";
    throw error;
   }
  };
  db.exec("delete from output; delete from physical");
  for(const path of row.outputs)db.run("insert into output values(?)",[path]);
  for(const path of physical)db.run("insert into physical values(?)",[path]);
  const matches=db.query(`
   select path, path=?1 or substr(?1,1,length(path)+1)=path||'/' exact
   from output
   where exact or (?2 and substr(path,1,length(?1))=?1 and substr(path,length(?1)+1) in (select value from json_each(?3)))
  `).all(row.request,Number(posix.extname(row.request)===""),JSON.stringify(resolution.files)) as {path:string;exact:number}[];
  const match=matches[0];
  const competitor=matches.length===1&&!match.exact&&db.query(`
   select 1 from (select path from output union select path from physical)
   where path<>?1 and substr(path,1,length(?2))=?2 and substr(path,length(?2)+1) in (select value from json_each(?3))
  `).get(match.path,row.request,JSON.stringify(resolution.probes));
  let oracle=match?.path??"";
  if(competitor)oracle="ambiguous-resolution";
  if(matches.length>1)oracle="ambiguous-producer";
  if(row.request.startsWith("../")||match&&row.request.includes("?"))oracle="unsafe-path";
  expect(oracle,row.id).toBe(row.expected);
  expect(outcome(),row.id).toBe(oracle);
  const initializedReads=unrelatedReads;
  expect(outcome(),row.id).toBe(oracle);
  expect(unrelatedReads-initializedReads,row.id).toBe(0);
  if(row.id==="extensionless"){
   physical.add("made/file.d.ts");expect(outcome()).toBe("ambiguous-resolution");
   physical.clear();expect(outcome()).toBe(oracle);expect(manifestReads).toBe(3);
  }
 }}finally{db.close();}
});

test("a vanished declared catalog Manifest enters the existing owner-registry repair after custody settles",async()=>{
 const file=ts.createSourceFile("owner.mjs",readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
 const declaration=file.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="catalogNativeManifestCandidatesV1");
 if(!declaration)throw Error("Actual native catalog candidates required");
 const db=new Database(":memory:"),signal=new AbortController().signal,root="/workspace",path="component/Cargo.toml";
 const oracle=db.query<{verdict:string},[string,string]>(`select case when ?2='file' then 'admitted' when ?2='symlink' then 'unsafe'
   when ?2 not in ('ENOENT','ENOTDIR') then 'error' when ?1='implicit' then 'omitted' else 'registry' end verdict`);
 try{for(const row of fixture.catalog){
   const expected=oracle.get(row.scope,row.status)!.verdict;
   expect(row.expected).toBe(expected);
   const owner=row.scope==="manifest"?{root:"component",manifest:path}:row.scope==="native-root"?{root:"other",manifest:"other/project.json",nativeRoots:["component"]}:{root:"component",manifest:"component/project.json"};
   const execute=evaluateExtractedFunctionV1(declaration.getText(file).replace(/^export\s+/,""),"catalogNativeManifestCandidatesV1",{
     join:posix.join,yieldInferenceV1:async()=>undefined,lstatSync:()=>{
       if(row.status==="file"||row.status==="symlink")return{isFile:()=>true,isSymbolicLink:()=>row.status==="symlink"};
       throw Object.assign(Error(row.status),{code:row.status});
     },
   });
   let custodyReleased=false,result:string;
   try{
     const candidates=await withOwnerRegistryCustodyV1<string[]>(signal,async contain=>{try{return await contain(()=>execute(root,{owners:[owner]},(candidate:string)=>candidate===path,{}));}finally{custodyReleased=true;}});
     result=candidates.includes(path)?"admitted":"omitted";
   }catch(error){
     if(isClosedOwnerRegistryInvalidationV1(error)){
       expect(vanishedOwnerRootsV1((error as Error).message)).toEqual(["component"]);
       result="registry";
     }else result=String(error).includes("must be a real file")?"unsafe":"error";
   }
   expect(custodyReleased).toBe(true);
   expect(result,row.scope+"/"+row.status).toBe(expected);
 }}finally{db.close();}
});

test("a package manifest vanishing between existence check and read is absent while other failures surface",()=>{
 const file=ts.createSourceFile("owner.mjs",readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
 const declaration=file.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="readVanishableTextV1");
 if(!declaration)throw Error("Actual vanishable text reader required");
 const dir=mkdtempSync(join(tmpdir(),"vanish-")),present=join(dir,"package.json");
 try{
  writeFileSync(present,"{}");
  const real=evaluateExtractedFunctionV1(declaration.getText(file).replace(/^export\s+/,""),"readVanishableTextV1",{readFileSync});
  expect(real(present)).toBe("{}");
  rmSync(present);
  expect(real(present)).toBeUndefined();
  expect(real(join(present,"below"))).toBeUndefined();
  for(const code of["EACCES","EIO"]){
   const failing=evaluateExtractedFunctionV1(declaration.getText(file).replace(/^export\s+/,""),"readVanishableTextV1",{readFileSync:()=>{throw Object.assign(Error(code),{code});}});
   expect(()=>failing(present)).toThrow(code);
  }
 }finally{rmSync(dir,{recursive:true,force:true});}
});
