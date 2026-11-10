import {createHash,randomBytes} from "node:crypto";
import {dirname,isAbsolute,join,resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {parserTypescriptV1} from "../../🚀️bootstrap/📦️parsers/🟨️.mjs";
import type {ReadFsV1} from "./🟦️.ts";

declare const Bun:{plugin(options:{name:string;setup(build:any):void}):void;resolveSync(specifier:string,from:string):string};

const SCHEME="semio-capture",ENTRY="semio-capture/";
const LOADERS:Readonly<Record<string,string>>=Object.freeze({".ts":"ts",".mts":"ts",".cts":"ts",".tsx":"tsx",".js":"js",".mjs":"js",".cjs":"js",".jsx":"jsx",".json":"json"});
const CANDIDATES=Object.freeze(["",".ts",".tsx",".mts",".js",".mjs",".cjs",".json","/index.ts","/index.tsx","/index.js","/index.mjs"]);

/** 🧩️ One plan's view for the module loader: the captured port and the files its modules were loaded from. */
export type ModuleScopeV1=Readonly<{port:ReadFsV1;captured(path:string):boolean;loaded:Set<string>}>;

const scopes=new Map<string,ModuleScopeV1>();
let installed=false;

const split=(identity:string):{id:string;path:string}=>{const at=identity.indexOf("|");return{id:identity.slice(0,at),path:identity.slice(at+1)};};
const extensionOf=(path:string):string=>{const at=path.lastIndexOf(".");return at<path.lastIndexOf("/")?"":path.slice(at);};
const literal=(value:string):string=>JSON.stringify(value);

const external=(specifier:string):boolean=>specifier.startsWith("node:")||specifier.startsWith("bun:")||specifier.startsWith(SCHEME)||specifier.startsWith(ENTRY);

const resolveCaptured=(scope:ModuleScopeV1,base:string):string|undefined=>{
 for(const suffix of CANDIDATES){
  const candidate=base+suffix;
  try{if(scope.port.statSync(candidate).isFile())return candidate;}catch{}
 }
 return undefined;
};

/** 🧩️ Where an import of `from` leads: a workspace file the capture serves (relative, absolute or reached through a workspace package) as the capture's own specifier, anything else as the file it resolves to on disk. `undefined` leaves the specifier to Bun (built-ins). */
const target=(scope:ModuleScopeV1,id:string,from:string,specifier:string):string|undefined=>{
 if(external(specifier))return undefined;
 let file:string|undefined;
 if(specifier.startsWith(".")||isAbsolute(specifier)){
  const base=resolve(dirname(from),specifier);
  file=scope.captured(base)?resolveCaptured(scope,base):undefined;
  if(file===undefined)try{file=Bun.resolveSync(base,dirname(from));}catch{return undefined;}
 }else{
  try{file=Bun.resolveSync(specifier,dirname(from));}catch{return undefined;}
  if(!isAbsolute(file))return undefined;
 }
 return scope.captured(file)&&LOADERS[extensionOf(file)]!==undefined?ENTRY+id+"|"+file:file;
};

const META:Readonly<Record<string,(path:string)=>string>>=Object.freeze({
 url:path=>pathToFileURL(path).href,dir:path=>dirname(path),dirname:path=>dirname(path),path:path=>path,filename:path=>path,file:path=>path.slice(path.lastIndexOf("/")+1),
});

/** 🧩️ Rewrites a captured module on its syntax tree: the module specifier of each import and export declaration, the string argument of `import()` and `require()` and each `import.meta.url|dir|dirname|path|filename|file` become the specifier they resolve to or the real location; string literals, template text and comments are never touched. Bun asks plugins to resolve neither bare package specifiers nor every extension-less relative one of a module in a plugin namespace, hence the specifiers. */
export function pinModuleV1(source:string,path:string,loader:string,resolveSpecifier:(specifier:string)=>string|undefined):string{
 if(loader==="json")return source;
 const ts=parserTypescriptV1(),kinds:Record<string,number>={ts:ts.ScriptKind.TS,tsx:ts.ScriptKind.TSX,js:ts.ScriptKind.JS,jsx:ts.ScriptKind.JSX},file=ts.createSourceFile(path,source,ts.ScriptTarget.Latest,false,kinds[loader]??ts.ScriptKind.JS);
 const edits:{start:number;end:number;text:string}[]=[];
 const specifier=(node:any):void=>{
  if(!node||!ts.isStringLiteralLike(node))return;
  const replacement=resolveSpecifier(node.text);
  if(replacement!==undefined&&replacement!==node.text)edits.push({start:node.getStart(file),end:node.getEnd(),text:literal(replacement)});
 };
 const visit=(node:any):void=>{
  if((ts.isImportDeclaration(node)||ts.isExportDeclaration(node))&&node.moduleSpecifier)specifier(node.moduleSpecifier);
  else if(ts.isImportEqualsDeclaration(node)&&ts.isExternalModuleReference(node.moduleReference))specifier(node.moduleReference.expression);
  else if(ts.isCallExpression(node)&&node.arguments.length>0&&(node.expression.kind===ts.SyntaxKind.ImportKeyword||ts.isIdentifier(node.expression)&&node.expression.text==="require"))specifier(node.arguments[0]);
  else if(ts.isPropertyAccessExpression(node)&&ts.isMetaProperty(node.expression)&&node.expression.keywordToken===ts.SyntaxKind.ImportKeyword&&META[node.name.text]!==undefined){edits.push({start:node.getStart(file),end:node.getEnd(),text:literal(META[node.name.text]!(path))});return;}
  ts.forEachChild(node,visit);
 };
 visit(file);
 let result=source;
 for(const edit of edits.sort((left,right)=>right.start-left.start))result=result.slice(0,edit.start)+edit.text+result.slice(edit.end);
 return result;
}

const resolveHook=(args:{path:string;importer?:string}):{path:string;namespace?:string}|undefined=>{
 if(args.path.startsWith(ENTRY))return{path:args.path.slice(ENTRY.length),namespace:SCHEME};
 if(args.importer?.startsWith(SCHEME+":")!==true)return undefined;
 const {id,path:from}=split(args.importer.slice(SCHEME.length+1)),scope=scopes.get(id),found=scope===undefined?undefined:target(scope,id,from,args.path);
 if(found===undefined)return undefined;
 return found.startsWith(ENTRY)?{path:found.slice(ENTRY.length),namespace:SCHEME}:{path:found};
};

const loadHook=(args:{path:string}):{contents:string;loader:string}=>{
 const {id,path}=split(args.path),scope=scopes.get(id);
 if(scope===undefined)throw Error("The plan of this module is over: "+path);
 const text=scope.port.readFileSync(path,"utf8") as string,loader=LOADERS[extensionOf(path)]??"js";
 scope.loaded.add(path);
 if(loader==="json")return{contents:"export default "+JSON.stringify(JSON.parse(text))+";",loader:"js"};
 return{contents:pinModuleV1(text,path,loader,specifier=>target(scope,id,path,specifier)),loader};
};

/** 🧩️ Registers the resolve and load of the plan namespace with Bun and exposes the same two hooks to a sealed loader, which owns every resolution of the implementation it runs and delegates the plan namespace (plan data, not implementation) to them. */
const install=():void=>{
 if(installed)return;
 installed=true;
 Object.assign(globalThis,{__semioPlanResolveV1:resolveHook,__semioPlanLoadV1:loadHook});
 Bun.plugin({name:SCHEME,setup(build){
  build.onResolve({filter:/.*/},resolveHook);
  build.onLoad({filter:/.*/,namespace:SCHEME},loadHook);
 }});
};

const cache=new Map<string,{id:string;digests:ReadonlyMap<string,string>;module:unknown}>(),CACHE_LIMIT=64;
const forget=(path:string):void=>{const old=cache.get(path);if(old!==undefined){scopes.delete(old.id);cache.delete(path);}};

/**
 * 🧩️ Imports a workspace module through the captured port of one plan: the module and every relative or workspace import below it is resolved and read through `port` (staged on first read, served from the staged bytes after, so an edit mid-plan never changes the loaded module and the plan's pin lists them), bare specifiers resolve through the installed dependencies and are read live. A module whose captured files are byte-equal to a previous load is reused, so a long-lived host holds one generation per provider.
 * @see ./📸️capture.ts
 */
export async function importCapturedModuleV1(scope:Omit<ModuleScopeV1,"loaded">,path:string,limit=CACHE_LIMIT):Promise<unknown>{
 install();
 const known=cache.get(path);
 if(known!==undefined){
  let same=true;
  for(const [file,hash] of known.digests){
   let bytes:Buffer|undefined;try{bytes=scope.port.readFileSync(file) as Buffer;}catch{}
   if(bytes===undefined||createHash("sha256").update(bytes).digest("hex")!==hash){same=false;break;}
  }
  if(same){cache.delete(path);cache.set(path,known);return known.module;}
  forget(path);
 }
 const id=randomBytes(8).toString("hex"),loaded=new Set<string>();
 scopes.set(id,{...scope,loaded});
 try{
  const module=await import(ENTRY+id+"|"+path);
  const digests=new Map<string,string>();
  for(const file of loaded)digests.set(file,createHash("sha256").update(scope.port.readFileSync(file) as Buffer).digest("hex"));
  forget(path);cache.set(path,{id,digests,module});
  while(cache.size>limit)forget(cache.keys().next().value!);
  return module;
 }catch(error){scopes.delete(id);throw error;}
}
