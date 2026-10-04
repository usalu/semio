import {requireRecord,requireString,requireStringArray,requireExactKeys} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import {inspectRustBindingFacts,rustExternProviders,type RustBindingFacts} from "../🔗️binding/🟦️.ts";
import { projectCargoProviderManifest, rustModuleScopeProof, type RustModuleGraph } from "../../../../🔍️discovery/🟦️.ts";
import { rustTokens, rustTokenPairs, rustIdentifierSymbol, type RustToken } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import {rustSourceTargets,rustSourceTargetProblem,type RustSourceInputInventory} from "../🟦️.ts";

export const COMPUTE_OWNERSHIP_CONTRACT_PATH="🧰️framework/🔨️modules/◻️2d/🧮️compute/🧫️fixtures/📍️binding-origin/🔣️.json";
const COMPUTE_FAMILY_SYMBOLS=["EngineKey","EngineHandle","EngineFault","EngineHandles","EngineCache","EngineRep"] as const;
export const COMPUTE_OWNERSHIP_DECLARATION_PATH=COMPUTE_OWNERSHIP_CONTRACT_PATH.split("/🧫️fixtures/")[0]+"/🦀️.rs";

function localEnumFacts(source:string,tokens:readonly RustToken[],pairs:ReadonlyMap<number,number>,facts:RustBindingFacts):readonly Readonly<{declaration:RustBindingFacts["declarations"][number];variants:ReadonlyMap<string,number>}>[] {
 const result:Readonly<{declaration:RustBindingFacts["declarations"][number];variants:ReadonlyMap<string,number>}>[]=[];
 for(let index=0;index<tokens.length;index++)if(tokens[index]!.text==="enum"&&tokens[index+2]?.text==="{"){
  const end=pairs.get(index+2),start=Buffer.byteLength(source.slice(0,tokens[index]!.start)),declaration=facts.declarations.find(row=>row.span.start===start);
  if(end===undefined||!declaration)continue;
  const variants=new Map<string,number>();let expecting=true;
  for(let cursor=index+3;cursor<end;cursor++){
   const token=tokens[cursor]!;if(token.text===","){expecting=true;continue;}
   if(expecting&&token.kind==="identifier"){variants.set(rustIdentifierSymbol(token)!,token.start);expecting=false;}
   const nested=pairs.get(cursor);if(nested!==undefined&&nested>cursor)cursor=nested;
  }
  result.push({declaration,variants});
 }
 return result;
}


function lexicalScope(facts:RustBindingFacts,tokens:readonly RustToken[],pairs:ReadonlyMap<number,number>,offset:number):Readonly<{modulePath:readonly string[];blocks:readonly number[]}>|null {
 const scopes=facts.graphFacts.scopes.filter(scope=>scope.bodyStartOffset<=offset&&offset<scope.bodyEndOffset).sort((a,b)=>b.modulePath.length-a.modulePath.length),selected=scopes[0];
 if(!selected||rustModuleScopeProof(facts.graphFacts,selected.modulePath).state!=="resolved")return null;
 const blocks=tokens.flatMap((token,index)=>{const close=pairs.get(index);return token.text==="{"&&token.start>=selected.bodyStartOffset&&token.start<offset&&close!==undefined&&offset<tokens[close]!.end?[token.start]:[];});
 return {modulePath:selected.modulePath,blocks};
}

function localEnumVariant(enums:ReturnType<typeof localEnumFacts>,parts:readonly string[],offset:number,facts:RustBindingFacts,tokens:readonly RustToken[],pairs:ReadonlyMap<number,number>):boolean {
 const scope=lexicalScope(facts,tokens,pairs,offset);if(!scope||parts.length!==2)return false;
 const visible=(row:Readonly<{modulePath:readonly string[];blockScope:readonly number[]}>)=>row.modulePath.join("::")===scope.modulePath.join("::")&&row.blockScope.length<=scope.blocks.length&&row.blockScope.every((block,index)=>block===scope.blocks[index]);
 const matches=enums.filter(row=>row.declaration.name===parts[0]&&row.variants.has(parts[1]!)&&visible(row.declaration));
 if(matches.length!==1)return false;
 const declaration=matches[0]!.declaration;
 return !facts.declarations.some(row=>row!==declaration&&row.name===parts[0]&&visible(row)&&row.blockScope.length>=declaration.blockScope.length)&&!facts.imports.some(row=>visible(row)&&row.blockScope.length>=declaration.blockScope.length&&(!row.glob&&(row.alias??row.path.at(-1))===parts[0]));
}

function engineFamilyItem(tokens:readonly RustToken[],pairs:ReadonlyMap<number,number>,index:number):boolean {
 const kind=tokens[index]?.text;if(kind!=="trait"&&kind!=="impl"||kind==="trait"&&rustIdentifierSymbol(tokens[index+1])!=="Engine")return false;
 let body=index+1;while(body<tokens.length&&!["{",";"].includes(tokens[body]!.text))body++;
 const end=pairs.get(body);if(end===undefined)return false;
 const header=tokens.slice(index+1,body);if(kind==="impl"&&(!header.some(token=>token.text==="for")||!header.some(token=>rustIdentifierSymbol(token)==="Engine")))return false;
 for(let cursor=body+1;cursor<end;cursor++){
  if(tokens[cursor]!.text==="const"&&rustIdentifierSymbol(tokens[cursor+1])==="ENGINE_ID")return true;
  const nested=pairs.get(cursor);if(nested!==undefined&&nested>cursor)cursor=nested;
 }
 return false;
}

/** 🔎️ Activates only present compute authority or actual family syntax, preserving generic owner deletion. */
export function rustFamilyOwnershipActive(input:Readonly<{sources:ReadonlyMap<string,string>;graph:RustModuleGraph;inventory:RustSourceInputInventory;checkCancellation?:()=>void}>):boolean {
 const {sources,graph,inventory,checkCancellation}=input;
 const sourceFiles=new Set(sources.keys());
 if(sources.has(COMPUTE_OWNERSHIP_CONTRACT_PATH)||sources.has(COMPUTE_OWNERSHIP_DECLARATION_PATH))return true;
 const manifestsBySource=new Map<string,Set<string>>(),aliasesByManifest=new Map<string,readonly string[]>();
 for(const origin of graph.participations)if(origin.target.kind==="source"&&"context"in origin&&origin.context.manifestPath){const manifests=manifestsBySource.get(origin.target.path)??new Set<string>();manifests.add(origin.context.manifestPath);manifestsBySource.set(origin.target.path,manifests);}
 const aliases=(path:string):readonly string[]=>{
  if(aliasesByManifest.has(path))return aliasesByManifest.get(path)!;
  let values:readonly string[]=[];
  try{
   const read=(locator:string)=>rustSourceTargetProblem(locator,false,inventory,sourceFiles)===null?projectCargoProviderManifest({locator,source:sources.get(locator)??""}):undefined;
   const manifest=read(path);if(!manifest)return [];
   let workspace:ReturnType<typeof read>;
   if(manifest.package?.workspaceLocator!==undefined){
    const target=rustSourceTargets(path,[{kind:"include",path:manifest.package.workspaceLocator+"/Cargo.toml",line:1}])[0]!;
    if(target.directories.every(directory=>rustSourceTargetProblem(directory,true,inventory,sourceFiles)===null))workspace=read(target.to);
   }else{
    let directory=path.split("/").slice(0,-1);
    for(;;){const candidate=read([...directory,"Cargo.toml"].join("/"));if(candidate?.workspaceDeclared){workspace=candidate;break;}if(!directory.length)break;directory=directory.slice(0,-1);}
   }
   values=[...manifest.dependencies,...manifest.developmentDependencies,...manifest.buildDependencies].filter(dependency=>{
    if(dependency.packageOverride==="semio-framework-2d")return true;
    const inherited=workspace?.workspaceDeclared&&dependency.workspaceInherited?workspace.workspaceDependencies.filter(row=>row.key===dependency.key&&!row.targetCondition):[];
    return inherited.length===1&&inherited[0]!.packageOverride==="semio-framework-2d";
   }).map(dependency=>dependency.key.replaceAll("-","_"));
  }catch{}
  aliasesByManifest.set(path,values);return values;
 };
 for(const [path,source]of sources){
  checkCancellation?.();if(!path.endsWith(".rs")||![...COMPUTE_FAMILY_SYMBOLS,"Engine"].some(name=>source.includes(name)))continue;
  const tokens=rustTokens(source),pairs=rustTokenPairs(tokens),facts=inspectRustBindingFacts(source,{checkCancellation}),enums=localEnumFacts(source,tokens,pairs,facts);
  const variantOnly=(token:RustToken,index:number):boolean=>enums.some(row=>row.variants.get(rustIdentifierSymbol(token)??"")===token.start||tokens[index-1]?.text==="::"&&localEnumVariant(enums,[rustIdentifierSymbol(tokens[index-2])??"",rustIdentifierSymbol(token)??""],token.start,facts,tokens,pairs));
  if(tokens.some((token,index)=>token.kind==="identifier"&&tokens[index-1]?.text!=="'"&&COMPUTE_FAMILY_SYMBOLS.some(name=>name===rustIdentifierSymbol(token))&&!variantOnly(token,index)))return true;
  const roots=["semio_framework_2d","semio_framework_os_kernel","store",...[...(manifestsBySource.get(path)??[])].flatMap(aliases)];
  const familyRoute=(path:readonly string[]):boolean=>roots.includes(path[0]??"")&&(path.includes("Engine")||path[0]==="semio_framework_2d"&&path.includes("compute")&&tokens.some(token=>rustIdentifierSymbol(token)==="Engine"));
  if(facts.imports.some(fact=>familyRoute(fact.path))||facts.problems.some(problem=>problem.kind==="unproven-path-namespace"&&familyRoute(rustTokens(problem.path).filter(token=>token.kind==="identifier").map(token=>rustIdentifierSymbol(token)!))))return true;
  if(tokens.some((_,index)=>engineFamilyItem(tokens,pairs,index)))return true;
 }
 return false;
}

export type RustFamilyOwnershipContract=Readonly<{version:1;ownerManifest:string;ownerRoot:string;declarationSource:string;modulePath:readonly string[];externName:string;packageName:string;symbols:readonly string[];traitName:string}>;
export type RustFamilyOwnershipProblem=Readonly<{code:"invalid-ownership-contract"|"declaration-origin"|"family-alias"|"unproven-family-context"|"unproven-family-provider"|"unproven-family-binding"|"unproven-family-declaration";from:string;detail:string;to?:string}>;

/** 📜️ Decodes the closed portable family contract without caller-owned parser types. */
export function readRustFamilyOwnershipContract(value:unknown):RustFamilyOwnershipContract {
 const label="Rust family ownership",row=requireRecord(value,label);
 requireExactKeys(row,["version","ownerManifest","ownerRoot","declarationSource","modulePath","externName","packageName","symbols","traitName"],label);
 if(row.version!==1)throw Error(label+".version must be 1");
 const field=(key:string):string=>requireString(row[key],label+"."+key);
 const path=(key:string):string=>{const value=field(key);if(value.includes("\\")||value.includes("\0")||value.startsWith("/")||/^[A-Za-z]:/.test(value)||value.split("/").some(p=>!p||p==="."||p===".."))throw Error(label+" requires a canonical relative "+key);return value;};
 const symbols=requireStringArray(row.symbols,label+".symbols"),modulePath=requireStringArray(row.modulePath,label+".modulePath");
 if(!symbols.length||new Set(symbols).size!==symbols.length||!modulePath.length||[...symbols,...modulePath].some(value=>typeof value!=="string")||[...symbols,...modulePath,field("traitName"),field("externName")].some(name=>!/^[_\p{L}][_\p{L}\p{N}]*$/u.test(name)))throw Error(label+" requires distinct canonical Rust symbols");
 return {version:1,ownerManifest:path("ownerManifest"),ownerRoot:path("ownerRoot"),declarationSource:path("declarationSource"),modulePath,externName:field("externName"),packageName:field("packageName"),symbols,traitName:field("traitName")};
}

/** 🧬️ Proves every captured family candidate against its actual declaration mount and direct Cargo provider. */
export function inspectRustFamilyOwnership(input:Readonly<{contract:RustFamilyOwnershipContract;sources:ReadonlyMap<string,string>;graph:RustModuleGraph;inventory:RustSourceInputInventory;checkCancellation?:()=>void}>):readonly RustFamilyOwnershipProblem[] {
 const sourceFiles=new Set(input.sources.keys());
 const {contract,graph}=input,problems:RustFamilyOwnershipProblem[]=[];
 const names=new Set(contract.symbols),facts=new Map<string,RustBindingFacts>(),providers=new Map<string,ReturnType<typeof rustExternProviders>>();
 const fact=(path:string):RustBindingFacts=>{if(!facts.has(path))facts.set(path,inspectRustBindingFacts(input.sources.get(path)??"",{checkCancellation:input.checkCancellation}));return facts.get(path)!;};
 const add=(code:RustFamilyOwnershipProblem["code"],from:string,detail:string,to?:string):void=>{const problem={code,from,detail,...(to?{to}:{})};if(!problems.some(p=>JSON.stringify(p)===JSON.stringify(problem)))problems.push(problem);};
 const ownerContexts=graph.contexts.get(contract.declarationSource)??[];
 const canonical=(context:typeof ownerContexts[number]):boolean=>context.manifestPath===contract.ownerManifest&&context.crateRoot===contract.ownerRoot;
 const owner=ownerContexts.find(c=>canonical(c)&&c.modulePath.join("::")===contract.modulePath.join("::")&&graph.targets.get(c.crateRoot+"\0"+c.modulePath.join("::"))===contract.declarationSource);
 if(!owner||!input.sources.has(contract.ownerManifest)||!input.sources.has(contract.ownerRoot))add("unproven-family-declaration",contract.declarationSource,"Canonical declaration source lacks its exact admitted Cargo/module owner",contract.ownerManifest);
 try {
  const metadata=projectCargoProviderManifest({locator:contract.ownerManifest,source:input.sources.get(contract.ownerManifest)??""});
  if(metadata.package?.name!==contract.packageName||metadata.library?.name!==contract.externName||metadata.library.procMacro||!(graph.namedCrates.get(contract.externName)??[]).includes(contract.ownerRoot))add("unproven-family-declaration",contract.ownerManifest,"Canonical package and authored library identity do not match the binding contract");
 }catch(error){add("unproven-family-declaration",contract.ownerManifest,error instanceof Error?error.message:String(error));}
 const ownerFacts=fact(contract.declarationSource);
 for(const name of [...contract.symbols,contract.traitName])if(ownerFacts.declarations.filter(d=>d.name===name&&d.kind==="type"&&d.modulePath.length===0&&d.blockScope.length===0).length!==1)add("unproven-family-declaration",contract.declarationSource,"Canonical declaration must define exactly one "+name);
 for(const [from,source]of input.sources){
  input.checkCancellation?.();
  if(!from.endsWith(".rs")||![...names,contract.traitName].some(name=>source.includes(name)))continue;
  const tokens=rustTokens(source),pairs=rustTokenPairs(tokens),actual=new Set(tokens.filter(token=>token.kind==="identifier").map(rustIdentifierSymbol));
  if(![...names].some(name=>actual.has(name))&&!actual.has(contract.traitName))continue;
  const observed=fact(from),contexts=graph.contexts.get(from)??[];
  const typeNames=new Set(names);
  if(tokens.some((_,index)=>engineFamilyItem(tokens,pairs,index)))typeNames.add(contract.traitName);
  const declarations=observed.declarations.filter(d=>names.has(d.name)||d.name===contract.traitName&&tokens.some((token,index)=>token.text==="trait"&&engineFamilyItem(tokens,pairs,index)));
  for(const declaration of declarations)if(from!==contract.declarationSource||declaration.modulePath.length>0||declaration.blockScope.length>0||contexts.some(c=>!canonical(c)))add("declaration-origin",from,"Owned family declaration outside canonical owner: "+declaration.name,contract.declarationSource);
  const imports=observed.imports.filter(f=>f.path.some(name=>typeNames.has(name))||f.path.includes(contract.traitName)&&[contract.externName,"store","semio_framework_os_kernel"].includes(f.path[0]!)||f.path[0]===contract.externName&&f.path.slice(1).join("::")===contract.modulePath.join("::"));
  const qualified=observed.problems.filter(p=>p.kind==="unproven-path-namespace"&&rustTokens(p.path).some(token=>typeNames.has(rustIdentifierSymbol(token)??"")||rustIdentifierSymbol(token)===contract.traitName&&[contract.externName,"store","semio_framework_os_kernel"].some(root=>p.path.startsWith(root+"::"))));
  const byteCharacters=new Map<number,number>();
  let byte=0,character=0;for(const value of source){byteCharacters.set(byte,character);byte+=Buffer.byteLength(value);character+=value.length;}byteCharacters.set(byte,character);
  const scopeAt=(offset:number)=>lexicalScope(observed,tokens,pairs,offset);
  const visible=(fact:RustBindingFacts["imports"][number],offset:number):boolean=>{const scope=scopeAt(offset);return scope!==null&&fact.modulePath.join("::")===scope.modulePath.join("::")&&fact.blockScope.length<=scope.blocks.length&&fact.blockScope.every((block,index)=>block===scope.blocks[index]);};
  const direct=(name:string,offset:number):boolean=>observed.imports.some(f=>visible(f,offset)&&f.path[0]===contract.externName&&!f.glob&&!f.alias&&f.path.at(-1)===name&&f.path.slice(1,-1).join("::")===contract.modulePath.join("::"));
  const shadowed=(name:string,offset:number):boolean=>{const scope=scopeAt(offset);return scope===null||observed.declarations.some(d=>d.name===name&&d.modulePath.join("::")===scope.modulePath.join("::")&&d.blockScope.length<=scope.blocks.length&&d.blockScope.every((block,index)=>block===scope.blocks[index]))||observed.imports.some(f=>f.alias===name&&f.path[0]!==name&&visible(f,offset));};
  const ownedContext=contexts.length>0&&contexts.every(canonical);
  const localDirect=(name:string,offset:number):boolean=>{
   if(!ownedContext)return false;
   const scope=scopeAt(offset);if(!scope)return false;
   if(from===contract.declarationSource)return scope.modulePath.length===0&&!observed.declarations.some(row=>row.name===name&&row.blockScope.length>0&&row.modulePath.join("::")===scope.modulePath.join("::")&&row.blockScope.length<=scope.blocks.length&&row.blockScope.every((block,index)=>block===scope.blocks[index]))&&!observed.imports.some(row=>visible(row,offset)&&(row.alias??row.path.at(-1))===name)&&observed.declarations.filter(row=>row.name===name&&row.modulePath.length===0&&row.blockScope.length===0).length===1;
   return contexts.filter(context=>context.sourceScope.length<=scope.modulePath.length&&context.sourceScope.every((part,index)=>part===scope.modulePath[index])).every(context=>observed.imports.some(row=>visible(row,offset)&&row.glob&&row.path.every(part=>part==="super")&&[...context.modulePath,...scope.modulePath.slice(context.sourceScope.length)].slice(0,-row.path.length).join("::")===contract.modulePath.join("::")&&graph.targets.get(context.crateRoot+"\0"+contract.modulePath.join("::"))===contract.declarationSource))&&!observed.declarations.some(row=>row.name===name)&&!observed.imports.some(row=>visible(row,offset)&&(row.glob&&!row.path.every(part=>part==="super")||!row.glob&&(row.alias??row.path.at(-1))===name));
  };
  const familyRoute=(parts:readonly string[],offset:number):boolean=>{
   const index=parts.findIndex(name=>names.has(name)||name===contract.traitName);if(index<0)return false;
   if(parts[0]===contract.externName)return parts.slice(1,index).join("::")===contract.modulePath.join("::")&&!shadowed(contract.externName,offset);
   return index===0&&(localDirect(parts[0]!,offset)||direct(parts[0]!,offset)&&!shadowed(parts[0]!,offset));
  };
  const enums=localEnumFacts(source,tokens,pairs,observed),variantOffsets=new Set(enums.flatMap(row=>[...row.variants.values()])),unrelatedQualified=new Set<typeof qualified[number]>();
  for(const imported of imports){
   if(imported.path[0]===contract.externName&&shadowed(contract.externName,imported.characterSpan.start))add("unproven-family-binding",from,"Canonical extern root is lexically shadowed");
   if(imported.alias&&imported.alias!=="_"||imported.glob&&imported.path[0]===contract.externName||/\bpub(?:\([^)]*\))?\s+use\s*$/.test(source.slice(0,imported.characterSpan.start)))add("family-alias",from,"Family forwarding/renaming is refused: "+imported.path.join("::"));
   if(imported.path[0]!==contract.externName&&!direct(imported.path[0]!,imported.characterSpan.start)&&!(from.startsWith(contract.declarationSource.slice(0,-"🦀️.rs".length))&&contexts.every(canonical)))add("unproven-family-binding",from,"Family import does not name canonical source directly: "+imported.path.join("::"));
  }
  for(const path of qualified){const offset=path.span?byteCharacters.get(path.span.start):undefined;if(offset===undefined){add("unproven-family-binding",from,"Qualified family span is unavailable");continue;}const parts=rustTokens(path.path).filter(token=>token.kind==="identifier").map(token=>rustIdentifierSymbol(token)!),index=parts.findIndex(name=>names.has(name)||name===contract.traitName);if(index<0)continue;if(localEnumVariant(enums,parts,offset,observed,tokens,pairs)){unrelatedQualified.add(path);continue;}if(!familyRoute(parts,offset))add("unproven-family-binding",from,"Qualified family path is not a direct canonical route: "+path.path);}
  for(const problem of observed.problems)if(problem.kind==="unproven-generic-scope"&&problem.span){
   const offset=byteCharacters.get(problem.span.start),start=tokens.findIndex(token=>token.start===offset);
   if(start<0){add("unproven-family-binding",from,"Generic owner span is unavailable");continue;}
   let cursor=start;while(cursor<tokens.length&&tokens[cursor]!.text!=="<")cursor++;
   let depth=0,binder=true;for(;cursor<tokens.length;cursor++){
    const token=tokens[cursor]!;if(token.text==="<"){depth++;continue;}if(token.text===">"){if(--depth===0)break;continue;}if(token.text===","&&depth===1){binder=true;continue;}
    if(depth===1&&binder&&token.kind==="identifier"&&token.text!=="const"){if(names.has(rustIdentifierSymbol(token)!))add("unproven-family-binding",from,"Reserved family generic parameter has an unresolved lexical origin: "+rustIdentifierSymbol(token));binder=false;}
   }
  }
  const bare=tokens.filter((token,index)=>!variantOffsets.has(token.start)&&typeNames.has(rustIdentifierSymbol(token)??"")&&tokens[index-1]?.text!=="::"&&!["struct","enum","trait","type","union"].includes(tokens[index-1]?.text??"")&&["&",":","<","=","[","(",",","->","impl","dyn","as","+","{",";","return"].includes(tokens[index-1]?.text??""));
  for(const token of bare)if(!familyRoute([rustIdentifierSymbol(token)!],token.start))add("unproven-family-binding",from,"Bare family type lacks a direct canonical import: "+rustIdentifierSymbol(token));
  for(let index=0;index<tokens.length;index++)if(tokens[index]!.text==="type"){
   let end=index+2;while(end<tokens.length&&tokens[end]!.text!==";")end++;
   if(end<tokens.length&&tokens.slice(index+2,end).some(token=>names.has(rustIdentifierSymbol(token)??"")))add("family-alias",from,"Family type aliases are refused");
  }
  const ownedExpressionMacro=(problem:RustBindingFacts["problems"][number]):boolean=>{
   if(!ownedContext||!problem.span||problem.path!=="assert_eq")return false;
   const offset=byteCharacters.get(problem.span.start),scope=offset===undefined?null:scopeAt(offset);if(offset===undefined||!scope)return false;
   if(tokens.some((token,index)=>token.text==="!"&&(tokens[index-1]?.start??-1)>offset&&Buffer.byteLength(source.slice(0,token.end))<=problem.span!.end&&tokens[index-1]?.kind==="identifier"&&["(","[","{"].includes(tokens[index+1]?.text??"")))return false;
   if(observed.imports.some(row=>visible(row,offset)&&(row.glob&&!row.path.every(part=>part==="super")||(row.alias??row.path.at(-1))==="assert_eq")))return false;
   const namespacePaths=new Set(contexts.map(context=>[...context.modulePath,...scope.modulePath.slice(context.sourceScope.length)].join("::")));
   let changed=true;
   while(changed){
    changed=false;
    for(const context of contexts)for(const path of context.sourceChain){
     const parsed=fact(path),origins=path===from?[context]:(graph.contexts.get(path)??[]).filter(parent=>parent.crateRoot===context.crateRoot&&parent.manifestPath===context.manifestPath);
     for(const row of parsed.imports)if(row.glob&&row.path.every(part=>part==="super")&&(path===from?visible(row,offset):row.blockScope.length===0))for(const parent of origins){
      const actual=[...parent.modulePath,...row.modulePath.slice(parent.sourceScope.length)];
      if(row.modulePath.length>=parent.sourceScope.length&&parent.sourceScope.every((part,index)=>part===row.modulePath[index])&&namespacePaths.has(actual.join("::"))&&row.path.length<=actual.length){const target=actual.slice(0,-row.path.length).join("::");if(!namespacePaths.has(target)){namespacePaths.add(target);changed=true;}}
     }
    }
   }
   for(const context of contexts)for(const path of context.sourceChain){
    const bytes=input.sources.get(path)??"",stream=path===from?tokens:rustTokens(bytes),paired=path===from?pairs:rustTokenPairs(stream),parsed=fact(path);
    const origins=path===from?[context]:(graph.contexts.get(path)??[]).filter(parent=>parent.crateRoot===context.crateRoot&&parent.manifestPath===context.manifestPath&&context.sourceChain.includes(path)&&context.modulePath.slice(0,parent.modulePath.length).join("::")===parent.modulePath.join("::"));
    for(const imported of parsed.imports)if((imported.alias??imported.path.at(-1))==="assert_eq"||imported.glob&&!imported.path.every(part=>part==="super")){
     if(origins.some(parent=>{
      const actual=[...parent.modulePath,...imported.modulePath.slice(parent.sourceScope.length)],invocation=[...context.modulePath,...scope.modulePath.slice(context.sourceScope.length)];
      return imported.modulePath.length>=parent.sourceScope.length&&parent.sourceScope.every((part,position)=>part===imported.modulePath[position])&&namespacePaths.has(actual.join("::"))&&(path!==from?imported.blockScope.length===0:visible(imported,offset));
     }))return false;
    }
    for(let index=0;index<stream.length;index++)if(stream[index]!.text==="macro_rules"&&rustIdentifierSymbol(stream[index+2])==="assert_eq"){
     const declarationScope=lexicalScope(parsed,stream,paired,stream[index]!.start);if(!declarationScope)return false;
     if(origins.some(parent=>{
      const actual=[...parent.modulePath,...declarationScope.modulePath.slice(parent.sourceScope.length)],invocation=[...context.modulePath,...scope.modulePath.slice(context.sourceScope.length)];
      return actual.length<=invocation.length&&actual.every((part,position)=>part===invocation[position])&&(path!==from?declarationScope.blocks.length===0:stream[index]!.start<offset&&declarationScope.blocks.length<=scope.blocks.length&&declarationScope.blocks.every((block,position)=>block===scope.blocks[position]));
     }))return false;
    }
   }
   return tokens.every((token,index)=>{
    if(!names.has(rustIdentifierSymbol(token)??"")||Buffer.byteLength(source.slice(0,token.start))<problem.span!.start||Buffer.byteLength(source.slice(0,token.end))>problem.span!.end)return true;
    const parts=[rustIdentifierSymbol(token)!];let cursor=index;
    while(tokens[cursor-1]?.text==="::"){
     const previous=tokens[cursor-2];if(previous?.kind!=="identifier")return false;
     parts.unshift(rustIdentifierSymbol(previous)!);cursor-=2;
    }
    return familyRoute(parts,token.start);
   });
  };
  for(const problem of observed.problems)if(problem.kind==="unproven-macro-output"&&problem.span&&tokens.some(token=>names.has(rustIdentifierSymbol(token)??"")&&Buffer.byteLength(source.slice(0,token.start))>=problem.span!.start&&Buffer.byteLength(source.slice(0,token.end))<=problem.span!.end)&&!ownedExpressionMacro(problem))add("unproven-family-binding",from,"Opaque macro contains family syntax: "+problem.path);
  const consumer=bare.length>0||imports.length>0||qualified.some(path=>!unrelatedQualified.has(path));
  if(!consumer&&declarations.length===0)continue;
  if(!contexts.length){add("unproven-family-context",from,"Family candidate has no admitted module context");continue;}
  for(const context of contexts){
   input.checkCancellation?.();
   const scope=rustModuleScopeProof(observed.graphFacts,context.sourceScope);
   if(scope.state!=="resolved"||!context.manifestPath||graph.invalidManifests.has(context.manifestPath)){add("unproven-family-context",from,"Family context has no unique admitted scope/manifest",context.manifestPath??undefined);continue;}
   if(from===contract.declarationSource){if(!canonical(context))add("declaration-origin",from,"Canonical bytes were mounted under a foreign provider",context.manifestPath);continue;}
   if(canonical(context))continue;
   const key=context.manifestPath;
   if(!providers.has(key))providers.set(key,rustExternProviders({context,compileKind:"test",files:input.sources,sourceFiles,inventory:input.inventory,checkCancellation:input.checkCancellation}));
   const selected=providers.get(key)!.providers.filter(p=>p.externName===contract.externName&&p.dependencyKind==="normal"&&!p.optional&&!p.targetCondition&&p.manifestPath===contract.ownerManifest&&p.librarySource===contract.ownerRoot&&p.packageName===contract.packageName);
   if(selected.length!==1||!owner)add("unproven-family-provider",from,"Every family mount requires exactly one unconditional nonoptional direct canonical normal Cargo provider",context.manifestPath);
  }
 }
 return problems;
}
