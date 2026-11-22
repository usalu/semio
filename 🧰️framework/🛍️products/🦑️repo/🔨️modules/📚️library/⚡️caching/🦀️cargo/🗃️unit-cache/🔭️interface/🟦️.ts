import {createHash,randomBytes} from "node:crypto";
import {readFileSync,readdirSync,renameSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {basename,dirname,join,resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import type {RustcUnitV1} from "../🔏️key/🟦️.ts";

/** 🧀️ The authored interface policy: which units may be identified by their interface, which source bodies a dependant cannot observe, where the sidecars live.
 * @see ./🧬️schema/🔣️.json */
export const INTERFACE_POLICY_V1=schema.definitions.Policy.const;
const policy=INTERFACE_POLICY_V1,source=policy.source;
const sha=(value:string|Uint8Array):string=>createHash("sha256").update(value).digest("hex");
const set=(list:readonly string[])=>new Set(list);
const ATTRIBUTES=set(source.attributesAllowed),MODIFIERS=set(source.modifiersAllowed),FORBIDDEN=set(source.bodyKeywordsForbidden),FORBIDDEN_ATTRIBUTES=set(source.bodyAttributesForbidden),MACROS=set(source.bodyMacrosAllowed),FEATURES=set(source.featuresVoiding);
const ITEM_MODIFIERS=set(["unsafe","async","const","extern","default","safe","auto","gen"]);

type Token=Readonly<{kind:"id"|"lt"|"str"|"chr"|"num"|"p";start:number;end:number;text:string}>;
const identifierStart=/[A-Za-z_\u0080-￿]/u,identifierPart=/[A-Za-z0-9_\u0080-￿]/u;

/** 🧁️ Splits Rust source into tokens without comments and white space; `undefined` when a comment, string or character literal does not end. */
export function lexRustV1(text:string):readonly Token[]|undefined{
 const out:Token[]=[],length=text.length;let at=0;
 const push=(kind:Token["kind"],start:number,end:number)=>{out.push({kind,start,end,text:text.slice(start,end)});};
 const quoted=(from:number,quote:string):number=>{let index=from+1;while(index<length){const c=text[index]!;if(c==="\\")index+=2;else if(c===quote)return index+1;else index++;}return -1;};
 while(at<length){
  const c=text[at]!;
  if(c===" "||c==="\t"||c==="\n"||c==="\r"||c==="\f"||c==="\v"){at++;continue;}
  if(c==="/"&&text[at+1]==="/"){while(at<length&&text[at]!=="\n")at++;continue;}
  if(c==="/"&&text[at+1]==="*"){let depth=1;at+=2;while(at<length&&depth>0){if(text[at]==="/"&&text[at+1]==="*"){depth++;at+=2;}else if(text[at]==="*"&&text[at+1]==="/"){depth--;at+=2;}else at++;}if(depth>0)return undefined;continue;}
  if(c==="r"||c==="b"||c==="c"){
   let index=at+1;
   if(c==="b"&&text[index]==="'"){const end=quoted(index,"'");if(end<0)return undefined;push("chr",at,end);at=end;continue;}
   const prefixRaw=(c==="b"||c==="c")&&text[index]==="r";if(prefixRaw)index++;
   if(c==="r"||prefixRaw){
    let hashes=0;while(text[index]==="#"){hashes++;index++;}
    if(text[index]==="\""){const close="\""+"#".repeat(hashes),end=text.indexOf(close,index+1);if(end<0)return undefined;push("str",at,end+close.length);at=end+close.length;continue;}
    if(c==="r"&&hashes===1&&identifierStart.test(text[index]??"")){let end=index;while(end<length&&identifierPart.test(text[end]!))end++;push("id",at,end);at=end;continue;}
   }else if(text[index]==="\""){const end=quoted(index,"\"");if(end<0)return undefined;push("str",at,end);at=end;continue;}
  }
  if(c==="\""){const end=quoted(at,"\"");if(end<0)return undefined;push("str",at,end);at=end;continue;}
  if(c==="'"){
   if(text[at+1]==="\\"){const end=quoted(at,"'");if(end<0)return undefined;push("chr",at,end);at=end;continue;}
   const width=(text.codePointAt(at+1)??0)>0xffff?2:1;
   if(text[at+1+width]==="'"){push("chr",at,at+2+width);at+=2+width;continue;}
   let end=at+1;while(end<length&&identifierPart.test(text[end]!))end++;
   if(end===at+1){push("p",at,at+1);at++;continue;}
   push("lt",at,end);at=end;continue;
  }
  if(c>="0"&&c<="9"){let end=at+1;while(end<length){const d=text[end]!;if(identifierPart.test(d))end++;else if(d==="."&&text[end+1]!=="."&&text[end+1]!==undefined&&text[end+1]!>="0"&&text[end+1]!<="9")end++;else break;}push("num",at,end);at=end;continue;}
  if(identifierStart.test(c)){let end=at+1;while(end<length&&identifierPart.test(text[end]!))end++;push("id",at,end);at=end;continue;}
  push("p",at,at+1);at++;
 }
 return out;
}

/** 🧃️ What the scanner found in one source file: the text with the unobservable function bodies replaced, how many it replaced and whether the file stops the whole crate from being masked. */
export type MaskedSourceV1=Readonly<{masked:string;bodies:number;void:boolean;splice:boolean}>;

const OPEN:Readonly<Record<string,string>>={"(":")","[":"]","{":"}"};

/** 🧄️ Replaces the body of every function that no dependant can observe by a fixed placeholder, and nothing else. A body is unobservable when the function is a plain, non-generic, non-const, non-async, non-inline function with no attribute but the allowed ones, no `impl Trait` in its signature, in a module or a non-generic `impl` block, and the body declares no item and calls only known expression macros. Whatever the scanner does not recognise stays in the text, so a misreading can only keep bytes, never hide them; a file that cannot be read (unbalanced brackets, an unfinished literal) or that enables a language feature through which a body can leak into a signature is `void`, and a file that splices another with `include!` is `splice`. */
export function maskRustSourceV1(text:string):MaskedSourceV1{
 const unmasked:MaskedSourceV1={masked:text,bodies:0,void:true,splice:false},tokens=lexRustV1(text);
 if(tokens===undefined)return unmasked;
 const partner=new Int32Array(tokens.length).fill(-1),stack:number[]=[];
 for(let index=0;index<tokens.length;index++){
  const t=tokens[index]!;if(t.kind!=="p")continue;
  if(OPEN[t.text]!==undefined)stack.push(index);
  else if(t.text===")"||t.text==="]"||t.text==="}"){const open=stack.pop();if(open===undefined||OPEN[tokens[open]!.text]!==t.text)return unmasked;partner[open]=index;partner[index]=open;}
 }
 if(stack.length>0)return unmasked;
 const is=(index:number,text:string)=>index>=0&&index<tokens.length&&tokens[index]!.kind==="p"&&tokens[index]!.text===text;
 const word=(index:number)=>index>=0&&index<tokens.length&&tokens[index]!.kind==="id"?tokens[index]!.text:undefined;
 let voided=false,splice=false;const cuts:Array<[number,number]>=[];
 for(let index=0;index<tokens.length;index++){
  if(word(index)==="include"&&is(index+1,"!"))splice=true;
 }
 const attribute=(from:number):{names:string[];next:number}=>{
  const names:string[]=[];let at=from;
  while(is(at,"#")){
   let open=at+1;const inner=is(open,"!");if(inner)open++;
   if(!is(open,"["))return{names:["⟨malformed⟩"],next:at+1};
   const close=partner[open]!,name=word(open+1)??"⟨unknown⟩";names.push(name);
   if(name==="feature")for(let k=open+2;k<close;k++){const feature=word(k);if(feature!==undefined&&FEATURES.has(feature))voided=true;}
   at=close+1;
  }
  return{names,next:at};
 };
 const headerEnd=(from:number,to:number):{at:number;kind:"{"|";"|"end"}=>{
  let at=from,angle=0;
  while(at<to){
   const t=tokens[at]!;
   if(t.kind==="p"){
    if(t.text==="("||t.text==="["){at=partner[at]!+1;continue;}
    if(t.text==="<")angle++;
    else if(t.text===">"&&!is(at-1,"-")&&!is(at-1,"=")&&angle>0)angle--;
    else if(t.text==="{"){if(angle===0)return{at,kind:"{"};at=partner[at]!+1;continue;}
    else if(t.text===";"&&angle===0)return{at,kind:";"};
   }
   at++;
  }
  return{at:to,kind:"end"};
 };
 const statementEnd=(from:number,to:number):number=>{
  let at=from;
  while(at<to){
   const t=tokens[at]!;
   if(t.kind==="p"&&(t.text==="("||t.text==="["||t.text==="{")){at=partner[at]!+1;continue;}
   if(t.kind==="p"&&t.text===";")return at+1;
   at++;
  }
  return to;
 };
 const skipItem=(from:number,to:number):number=>{
  const end=headerEnd(from,to);
  return end.kind==="{"?partner[end.at]!+1:end.kind===";"?end.at+1:to;
 };
 const bodyObservable=(open:number,close:number):boolean=>{
  for(let k=open+1;k<close;k++){
   const t=tokens[k]!;
   if(t.kind==="id"){
    if(FORBIDDEN.has(t.text))return true;
    if(is(k+1,"!")&&(is(k+2,"(")||is(k+2,"[")||is(k+2,"{"))&&!MACROS.has(t.text))return true;
    if(FORBIDDEN_ATTRIBUTES.has(t.text)&&[1,2,3,4].some(back=>is(k-back,"#")))return true;
   }
  }
  return false;
 };
 const block=(from:number,to:number,context:"module"|"impl"|"opaque"):void=>{
  let at=from;
  while(at<to){
   const attrs=attribute(at);at=attrs.next;
   if(at>=to)break;
   if(is(at,";")){at++;continue;}
   let start=at;
   if(word(start)==="pub"){start++;if(is(start,"("))start=partner[start]!+1;}
   const modifiers:string[]=[];
   let probe=start;
   while(true){
    const m=word(probe);
    if(m===undefined||!ITEM_MODIFIERS.has(m))break;
    if(m==="extern"){
     const after=tokens[probe+1]?.kind==="str"?probe+2:probe+1,next=word(after);
     if(next!=="fn"&&next!=="unsafe")break;
     modifiers.push(m);probe=after;continue;
    }
    if(m==="const"){const next=word(probe+1);if(next!=="fn"&&next!=="unsafe"&&next!=="async"&&next!=="extern")break;}
    modifiers.push(m);probe++;
   }
   const keyword=word(probe);
   if(keyword==="fn"&&tokens[probe+1]?.kind==="id"){
    const end=headerEnd(probe+2,to);
    if(end.kind===";"){at=end.at+1;continue;}
    if(end.kind==="end"){at=to;continue;}
    const open=end.at,close=partner[open]!;
    const generic=is(probe+2,"<");let wherePresent=false,opaqueSignature=false;
    for(let k=probe+2;k<open;k++){const w=word(k);if(w==="where")wherePresent=true;if(w==="impl")opaqueSignature=true;}
    const eligible=context!=="opaque"&&modifiers.every(m=>MODIFIERS.has(m))&&attrs.names.every(n=>ATTRIBUTES.has(n))&&!generic&&!wherePresent&&!opaqueSignature&&!bodyObservable(open,close);
    if(eligible)cuts.push([tokens[open]!.start,tokens[close]!.end]);
    at=close+1;continue;
   }
   if(keyword==="mod"&&tokens[probe+1]?.kind==="id"){
    const end=headerEnd(probe+2,to);
    if(end.kind==="{"){const close=partner[end.at]!;block(end.at+1,close,"module");at=close+1;}else at=end.at+1;
    continue;
   }
   if(keyword==="impl"){
    const end=headerEnd(probe+1,to);
    if(end.kind!=="{"){at=end.kind==="end"?to:end.at+1;continue;}
    const close=partner[end.at]!;let mono=!is(probe+1,"<")&&attrs.names.every(n=>ATTRIBUTES.has(n))&&modifiers.every(m=>m==="unsafe");
    for(let k=probe+1;k<end.at;k++){const w=word(k);if(w==="where"||w==="const"||w==="impl")mono=false;}
    block(end.at+1,close,mono?"impl":"opaque");at=close+1;continue;
   }
   if(keyword==="type"){
    const end=headerEnd(probe+1,to);
    for(let k=probe+1;k<end.at;k++)if(word(k)==="impl")voided=true;
    at=end.kind==="end"?to:end.at+1;continue;
   }
   at=keyword==="const"||keyword==="static"||keyword==="use"?statementEnd(start,to):skipItem(start,to);
  }
 };
 block(0,tokens.length,"module");
 if(voided)return{masked:text,bodies:0,void:true,splice};
 cuts.sort((a,b)=>a[0]-b[0]);
 let result="",last=0;
 for(const [from,to] of cuts){if(from<last)return{masked:text,bodies:0,void:true,splice};result+=text.slice(last,from)+source.maskedBody;last=to;}
 result+=text.slice(last);
 return{masked:result,bodies:cuts.length,void:false,splice};
}

/** 🧆️ The digest of the files a unit read, as a dependant can observe them: a `.rs` file by its text without the unobservable bodies, any other file by its bytes. One file that cannot be masked, or one `include!`, makes every `.rs` file count by its bytes, so the digest then equals a digest of the bytes. The names are the caller's neutral spellings. */
export function interfaceDigestV1(files:readonly Readonly<{name:string;bytes:Uint8Array|null}>[]):Readonly<{digest:string;bodies:number;masked:boolean}>{
 const rows=files.map(file=>{
  const rust=file.name.endsWith(".rs")&&file.bytes!==null;
  return{file,rust,analysis:rust?maskRustSourceV1(Buffer.from(file.bytes!).toString("utf8")):undefined};
 });
 const whole=rows.some(row=>row.analysis!==undefined&&(row.analysis.void||row.analysis.splice));
 let bodies=0;
 const listed=rows.map(row=>{
  if(row.file.bytes===null)return[row.file.name,"missing",""];
  if(row.analysis!==undefined&&!whole){bodies+=row.analysis.bodies;return[row.file.name,"masked",sha(row.analysis.masked)];}
  return[row.file.name,"bytes",sha(row.file.bytes)];
 }).sort((a,b)=>a[0]!<b[0]!?-1:a[0]!>b[0]!?1:0);
 return Object.freeze({digest:sha(JSON.stringify([policy.namespace,whole,listed])),bodies,masked:!whole});
}

/** 🧇️ Whether a command line may be identified by its interface: `undefined` when it may, else the reason. Only a library that emits metadata and no code qualifies (Cargo's `check` and `clippy` units): its dependants read nothing but metadata, whereas a unit that emits code is linked to by its dependants, and rebasing code onto a rebuilt dependency is unsound (a dependant may call an instance of a standard generic over the dependency's own types that the old object defined, see the compiler law of the tests). It must build no test harness, must not be optimised (an optimised build may inline small functions across crates, so their bodies become observable) and may use only unstable flags that do not change what is encoded. */
export function interfaceEligibilityV1(unit:RustcUnitV1,args:readonly string[]):string|undefined{
 const rules=policy.unit,types=unit.crateTypes.length?unit.crateTypes:["bin"];
 if(types.some(type=>!(rules.crateTypes as readonly string[]).includes(type)))return"crate-type";
 if(args.includes("--test"))return"test-harness";
 const emits=new Set(unit.emit);
 if(!emits.has("metadata"))return"no-metadata";
 if([...emits].some(kind=>kind!=="metadata"&&kind!=="dep-info"))return"emits-code";
 const flags=args.flatMap((argument,index)=>{
  if(argument==="-C"||argument==="--codegen")return[args[index+1]??""];
  if(argument.startsWith("-C"))return[argument.slice(2)];
  if(argument==="-Z")return["Z:"+(args[index+1]??"")];
  if(argument.startsWith("-Z"))return["Z:"+argument.slice(2)];
  if(argument==="-O")return["opt-level=2"];
  return[];
 });
 for(const flag of flags){
  if(flag.startsWith("opt-level=")&&flag.slice("opt-level=".length)!=="0")return"optimised";
  if(flag.startsWith("Z:")){const name=flag.slice(2).split("=")[0]!;if(!(rules.allowedUnstable as readonly string[]).includes(name))return"unstable:"+name;}
 }
 return undefined;
}

/** 🧉️ Reads what `rustc -Z ls=root` prints about a library's metadata: the crate hash (SVH) and the hash of every crate it was compiled against, by the crate's name and filename suffix. */
export function parseListRootV1(text:string):Readonly<{svh:string;dependencies:readonly (readonly [string,string])[]}>|undefined{
 const own=/^hash ([0-9a-f]{32}) stable_crate_id/mu.exec(text);if(own===null)return undefined;
 const dependencies=[...text.matchAll(/^\d+ (\S+) hash ([0-9a-f]{32}) host_hash/gmu)].map(match=>[match[1]!,match[2]!] as const);
 return Object.freeze({svh:own[1]!,dependencies:Object.freeze(dependencies)});
}

/** 🧋️ Asks the compiler itself for the crate hashes of one `.rmeta` file; `undefined` when it cannot say. */
export function listRootV1(inner:readonly string[],path:string,environment:Readonly<Record<string,string|undefined>>):ReturnType<typeof parseListRootV1>{
 const run=spawnSync(inner[0]!,[...inner.slice(1),"-Z","ls=root",path],{encoding:"utf8",env:{...environment,RUSTC_BOOTSTRAP:"1"} as NodeJS.ProcessEnv,timeout:policy.unit.listTimeoutMilliseconds});
 return run.status===0?parseListRootV1(run.stdout):undefined;
}

/** 🧌️ The bytes a crate hash has in metadata: the 128-bit value in little-endian order. */
export const svhBytesV1=(hex:string):Buffer=>Buffer.from(hex,"hex").reverse();

/** 🧎️ Rewrites crate hashes inside metadata bytes: each old hash must occur exactly once (a hash that is absent or repeated means the layout is not the one measured, so nothing is patched); all positions are found before any byte is written, so exchanged hashes cannot chain. `undefined` when any pair does not hold. */
export function rebaseMetadataV1(bytes:Uint8Array,pairs:readonly (readonly [string,string])[]):Buffer|undefined{
 const out=Buffer.from(bytes),found:Array<[number,Buffer]>=[];
 for(const [then,now] of pairs){
  const needle=svhBytesV1(then),first=out.indexOf(needle);
  if(first<0||out.indexOf(needle,first+1)>=0)return undefined;
  found.push([first,svhBytesV1(now)]);
 }
 for(const [at,replacement] of found)replacement.copy(out,at);
 return out;
}

const dependenciesPath=(path:string)=>path+policy.sidecars.dependencies,identityPath=(path:string)=>path+policy.sidecars.identity;
/** 🧏️ The sidecar that travels with a unit and holds its crate hash and the hashes it was compiled against. */
export const dependenciesFileV1=dependenciesPath;
/** 🧔️ The local sidecar that names the current bytes of a metadata file, its crate hash and the identity it enters dependants' keys by. */
export const identityFileV1=identityPath;

function atomicWrite(path:string,text:string):void{const temporary=path+"."+randomBytes(3).toString("hex")+".tmp";writeFileSync(temporary,text);renameSync(temporary,path);}
const readJson=<T>(path:string):T|undefined=>{try{return JSON.parse(readFileSync(path,"utf8")) as T;}catch{return undefined;}};
type IdentityRow=Readonly<{version:1;sha256:string;basis?:string;svh?:string;identity?:string}>;
type DependenciesRow=Readonly<{version:1;svh:string;dependencies:readonly (readonly [string,string])[]}>;

/** 🧕️ The identity a metadata file enters its dependants' keys by instead of its bytes: the interface identity of the unit that built it, trusted only while the sidecar still names the file's current digest. `undefined` for any other file. */
export function interfaceIdentityV1(path:string,sha256:string):string|undefined{
 if(!path.endsWith(".rmeta"))return undefined;
 const row=readJson<IdentityRow>(identityPath(path));
 if(row===undefined||row.version!==1||row.sha256!==sha256||typeof row.identity!=="string"||!/^[0-9a-f]{64}$/u.test(row.identity))return undefined;
 return"iface:"+row.identity;
}

/** 🧖️ Writes the sidecar that travels with a unit (the unit's own crate hash and the hashes it saw): `undefined` when the compiler cannot say, else the path written. */
export function writeDependenciesV1(rmeta:string,inner:readonly string[],environment:Readonly<Record<string,string|undefined>>):string|undefined{
 const listing=listRootV1(inner,rmeta,environment);if(listing===undefined)return undefined;
 const path=dependenciesPath(rmeta);
 try{atomicWrite(path,JSON.stringify({version:1,svh:listing.svh,dependencies:listing.dependencies}));return path;}catch{return undefined;}
}

/** 🧚️ Writes the local identity sidecar of every `.rmeta` output of a produced or restored unit: the digest of the file, the unit's crate hash (kept from the sidecar already beside the file while it names the same bytes, else read from the travelling sidecar, else asked of the compiler) and, for a unit that qualifies as a producer, the identity; the identity is recomputed only when `basis` (what the identity is a function of) differs from the one recorded. */
export function writeIdentitiesV1(input:Readonly<{outputs:readonly string[];digest:(path:string)=>string|null;basis:string;identity:(()=>string|undefined)|undefined;inner:readonly string[];environment:Readonly<Record<string,string|undefined>>}>):void{
 for(const path of input.outputs){
  if(!path.endsWith(".rmeta"))continue;
  const digest=input.digest(path);if(digest===null)continue;
  const before=readJson<IdentityRow>(identityPath(path)),same=before!==undefined&&before.version===1&&before.sha256===digest;
  const svh=(same?before.svh:undefined)??readJson<DependenciesRow>(dependenciesPath(path))?.svh??listRootV1(input.inner,path,input.environment)?.svh;
  const identity=input.identity===undefined?undefined:same&&before.basis===input.basis&&typeof before.identity==="string"?before.identity:input.identity();
  try{atomicWrite(identityPath(path),JSON.stringify({version:1,sha256:digest,basis:input.basis,...(svh===undefined?{}:{svh}),...(identity===undefined?{}:{identity})}));}catch{}
 }
}

/** 🧛️ Settles the sidecars of a unit that was just produced or restored (`entry` is what the cache recorded for it): the local identity sidecars, with the identity of a producer computed from the interface digest of the files the entry recorded. The wrapper supplies how a digest becomes an identity (`identityOf`, the portable key of the unit), how a recorded path is named (`neutralName`) and read (`readFile`). */
export function settleUnitInterfaceV1(input:Readonly<{outputs:readonly string[];entry:Readonly<{dependencies:readonly Readonly<{path:string;sha256:string|null}>[];environment:readonly Readonly<{name:string;value:string|null}>[]}>;producer:boolean;identityOf:(interfaceDigest:string)=>string;neutralName:(path:string)=>string;readFile:(path:string)=>Buffer|null;digest:(path:string)=>string|null;inner:readonly string[];environment:Readonly<Record<string,string|undefined>>}>):void{
 const rmetas=input.outputs.filter(path=>path.endsWith(".rmeta"));if(rmetas.length===0)return;
 const basis=sha(JSON.stringify([input.entry.dependencies.map(row=>[row.path,row.sha256]),input.entry.environment.map(row=>[row.name,row.value])]));
 writeIdentitiesV1({outputs:rmetas,digest:input.digest,basis,inner:input.inner,environment:input.environment,identity:input.producer?()=>input.identityOf(interfaceDigestV1(input.entry.dependencies.map(row=>({name:input.neutralName(row.path),bytes:row.sha256===null?null:input.readFile(row.path)}))).digest):undefined});
}

/** 🧝️ The crate hash a dependency's `.rmeta` file has now: the local sidecar when it still names the file's digest, else the compiler's answer (the sidecar is then renewed). */
export function currentHashV1(path:string,digest:(path:string)=>string|null,inner:readonly string[],environment:Readonly<Record<string,string|undefined>>):string|undefined{
 const current=digest(path);if(current===null)return undefined;
 const row=readJson<IdentityRow>(identityPath(path));
 if(row!==undefined&&row.version===1&&row.sha256===current&&typeof row.svh==="string")return row.svh;
 const listing=listRootV1(inner,path,environment);if(listing===undefined)return undefined;
 try{atomicWrite(identityPath(path),JSON.stringify({version:1,sha256:current,...(row?.basis===undefined||row.sha256!==current?{}:{basis:row.basis}),svh:listing.svh,...(row?.identity===undefined||row.sha256!==current?{}:{identity:row.identity})}));}catch{}
 return listing.svh;
}

const LIBRARY_FILES=["lib{id}.rmeta","lib{id}.dylib","lib{id}.so","{id}.dll"] as const;
const outputLayout=policy.rebase.cargoOutputLayout,unitSuffix=new RegExp(outputLayout.suffix,"u");
const [archiveHead,archiveTail]=policy.rebase.embeddedMetadataLibrary.split("{id}") as [string,string];

function managedUnitSuffixV1(directory:string):string|undefined{
 const unit=dirname(directory),container=dirname(dirname(unit)),suffix=basename(unit);
 return basename(directory)===outputLayout.output&&basename(container)===outputLayout.container&&unitSuffix.test(suffix)?suffix:undefined;
}

function dependencyFilesV1(directories:readonly string[]){
 const metadata=new Map<string,string>(),archives=new Map<string,string[]>(),managed=new Set<string>();
 for(const directory of directories){
  const suffix=managedUnitSuffixV1(directory);if(suffix!==undefined)managed.add(suffix);
  let names:string[];try{names=readdirSync(directory);}catch{continue;}
  for(const name of names){
   if(name.startsWith(archiveHead)&&name.endsWith(archiveTail)){
    const id=name.slice(archiveHead.length,name.length-archiveTail.length),path=join(directory,name);
    archives.set(id,[...(archives.get(id)??[]),path]);
   }
   for(const pattern of LIBRARY_FILES){
    const [head,tail]=pattern.split("{id}") as [string,string];
    if(name.startsWith(head)&&name.endsWith(tail)&&name.length>head.length+tail.length){const id=name.slice(head.length,name.length-tail.length);if(!metadata.has(id))metadata.set(id,join(directory,name));}
   }
  }
 }
 return {metadata,archives,managed};
}

function missingManagedLibraryV1(id:string,files:ReturnType<typeof dependencyFilesV1>):boolean{
 if(!files.managed.has(id.slice(id.lastIndexOf("-")+1)))return false;
 return !(files.archives.get(id)??[]).some(path=>{try{return statSync(path,{throwIfNoEntry:false})?.isFile()===true;}catch{return false;}});
}
/** 🗂️ The library files of a unit's dependency directories by crate identifier (`<name>-<suffix>`): the metadata file of a library, the dynamic library of a proc-macro; the first directory that holds one wins. */
export function dependencyLibrariesV1(directories:readonly string[]):ReadonlyMap<string,string>{
 return dependencyFilesV1(directories).metadata;
}

const hashesOf=(path:string):DependenciesRow|undefined=>{const row=readJson<DependenciesRow>(dependenciesPath(path));return row!==undefined&&row.version===1?row:undefined;};

/** 🧞️ Makes a unit restored from the cache consistent with the dependencies in the directory it is restored into: every crate hash its metadata records for a library or proc-macro library found there is replaced by that library's current hash. Returns `false` (the unit must be compiled) when the metadata cannot be proven consistent: the unit travels without its sidecar, a declared Cargo library is absent, a library's hash is unknown, or a hash does not occur exactly once. Crates resolved outside the declared directories, such as the standard library, keep their hashes. */
export function rebaseRestoredUnitV1(input:Readonly<{outputs:readonly string[];directories:readonly string[];digest:(path:string)=>string|null;inner:readonly string[];environment:Readonly<Record<string,string|undefined>>}>):boolean{
 const libraries=dependencyFilesV1(input.directories);
 for(const path of input.outputs){
  if(!path.endsWith(".rmeta"))continue;
  const travelling=hashesOf(path);
  if(travelling===undefined)return false;
  const pairs:Array<[string,string]>=[];
  for(const [id,then] of travelling.dependencies){
   const candidate=libraries.metadata.get(id);
   if(candidate===undefined){if(missingManagedLibraryV1(id,libraries))return false;continue;}
   if(candidate===path)continue;
   const now=currentHashV1(candidate,input.digest,input.inner,input.environment);
   if(now===undefined)return false;
   if(now!==then)pairs.push([then,now]);
  }
  if(pairs.length===0)continue;
  let patched:Buffer|undefined;
  try{patched=rebaseMetadataV1(readFileSync(path),pairs);}catch{return false;}
  if(patched===undefined)return false;
  try{
   const temporary=path+"."+randomBytes(3).toString("hex")+".tmp";
   writeFileSync(temporary,patched);const when=new Date();utimesSync(temporary,when,when);renameSync(temporary,path);
   const table=new Map(pairs);
   atomicWrite(dependenciesPath(path),JSON.stringify({version:1,svh:travelling.svh,dependencies:travelling.dependencies.map(([id,hash])=>[id,table.get(hash)??hash])}));
  }catch{return false;}
 }
 return true;
}

/** 🚧️ Admits a restored unit that cannot be rebased (it emits code) only when its whole dependency closure is the one it was compiled against: an explicitly declared Cargo library must be available, and every crate hash its metadata records (from the sidecar that travels with the unit, else asked of the compiler) for metadata or a proc-macro library found in the dependency directories must equal that library's current crate hash. `undefined` when the closure matches, else the first difference as text. A unit restored against another generation of a dependency cannot be loaded by its dependants (`E0463 can't find crate`), so the caller compiles it instead. */
export function restoredClosureViolationV1(input:Readonly<{outputs:readonly string[];directories:readonly string[];digest:(path:string)=>string|null;inner:readonly string[];environment:Readonly<Record<string,string|undefined>>}>):string|undefined{
 const libraries=dependencyFilesV1(input.directories);
 for(const path of input.outputs){
  if(!path.endsWith(".rmeta"))continue;
  const recorded=hashesOf(path)?.dependencies??listRootV1(input.inner,path,input.environment)?.dependencies;
  if(recorded===undefined)return"the crate hashes of "+basename(path)+" are unknown";
  for(const [id,then] of recorded){
   const candidate=libraries.metadata.get(id);
   if(candidate===undefined){if(missingManagedLibraryV1(id,libraries))return id+" is absent from the Cargo dependency directories";continue;}
   if(candidate===path)continue;
   const now=currentHashV1(candidate,input.digest,input.inner,input.environment);
   if(now===undefined)return"the crate hash of "+id+" is unknown";
   if(now!==then)return id+" has the crate hash "+now+" here, "+basename(path)+" was compiled against "+then;
  }
 }
 return undefined;
}

/** 🧡️ The directories a unit's dependencies are found in: the `-L dependency=` directories and the directories of the `--extern` files, resolved against the compiler's working directory. */
export function dependencyDirectoriesV1(args:readonly string[],unit:RustcUnitV1,cwd:string):readonly string[]{
 const found=new Set<string>();
 for(let index=0;index<args.length;index++){
  const argument=args[index]!,value=argument==="-L"?args[index+1]:argument.startsWith("-L")?argument.slice(2):undefined;
  if(value!==undefined&&value.startsWith("dependency="))found.add(resolve(cwd,value.slice("dependency=".length)));
 }
 for(const row of unit.externs)if(row.path!=="")found.add(dirname(resolve(cwd,row.path)));
 return[...found];
}
