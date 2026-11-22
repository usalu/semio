import {readPlayPageCohortV1,requirePlayPageCohortMemberV1} from "../📋️contract/🟦️.ts";
import {withPublishedPlayPagesV1} from "../🟦️.ts";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-rs/index";
import {checkResourceLeaseControl} from "@semio-tech/process/leases";
import type {ScriptInvocation} from "@semio-tech/process/routing";
import publication from "../🧬️schema/🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {mkdir,readdir,readFile,rename,rm,writeFile} from "node:fs/promises";
import {createHash} from "node:crypto";
import {dirname,join,resolve,sep} from "node:path";

const protocol=schema.definitions.Protocol.const,publicationProtocol=publication.definitions.Protocol.const;
type Cdn=Readonly<{version:1;mode:"release";cohortSha256:string;families:readonly Family[];rules:Readonly<{serveFamilyAtHostRoot:true;missingFileStatus:404;spaFallback:false;crossOriginIsolation:false;uploadOneGeneration:true;exactContentTypes:true}>}>;
type Family=Readonly<{family:string;host:string;directory:string;files:number;bytes:number;headers:readonly Readonly<{name:string;value:string}>[];contentTypes:readonly Readonly<{extension:string;type:string}>[];unmappedExtensions:readonly string[]}>;
/** 📊️ One progress step of an export: files and bytes verified so far. */
export type PlayExportProgressV1=Readonly<{family:string;files:number;total:number;bytes:number}>;
const types:Readonly<Record<string,string>>={html:"text/html",js:"text/javascript",mjs:"text/javascript",css:"text/css",json:"application/json",map:"application/json",wasm:"application/wasm",woff2:"font/woff2",woff:"font/woff",png:"image/png",jpg:"image/jpeg",jpeg:"image/jpeg",webp:"image/webp",svg:"image/svg+xml",ico:"image/x-icon",pbf:"application/x-protobuf",glb:"model/gltf-binary",gltf:"model/gltf+json",txt:"text/plain",mp4:"video/mp4"};
const digest=(bytes:Uint8Array):string=>createHash("sha256").update(bytes).digest("hex");
const fail=(en:string,de:string):never=>{throw Error(`${en} / ${de}`);};
const extensionOf=(path:string):string=>{const name=path.slice(path.lastIndexOf("/")+1),dot=name.lastIndexOf(".");return dot>0?name.slice(dot+1).toLowerCase():"";};
const headersOf=(text:string):Family["headers"]=>text.split("\n").filter(line=>/^\s+\S[^:]*:\s*\S/u.test(line)).map(line=>{const at=line.indexOf(":");return{name:line.slice(0,at).trim(),value:line.slice(at+1).trim()};});

/** 📤️ Copies the exact bytes of every held family into a plain CDN folder, rechecking each file against its manifest digest, and writes the cohort manifest and `🔣️cdn.json` beside the families.
 * The folder appears atomically: a staging sibling is renamed on success and removed on any refusal or cancellation.
 * @see ../🟦️.ts withPublishedPlayPagesV1 */
export async function exportPlayPagesV1(input:Readonly<{root:string;out:string;check:()=>void;progress?:(row:PlayExportProgressV1)=>void}>):Promise<Cdn>{
 const {root,out,check,progress}=input,staging=`${out}.partial`;
 if(await readdir(out).then(rows=>rows.length>0,()=>false))fail(`Export folder ${out} is not empty`,`Exportordner ${out} ist nicht leer`);
 const cohortBytes=await readFile(join(root,"cohort",protocol.cohort)).catch(()=>fail("Published Play cohort missing, build the release first","Veröffentlichte Play-Kohorte fehlt, zuerst den Release bauen"));
 let cohort;try{cohort=readPlayPageCohortV1(JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(cohortBytes)),"release");}catch{return fail("Release cohort required, found no valid release cohort","Release-Kohorte erforderlich, keine gültige gefunden");}
 await rm(staging,{recursive:true,force:true});
 try{
  const manifests=await Promise.all(cohort.pages.map(async row=>{const bytes=await readFile(join(root,row.family,publicationProtocol.manifest)).catch(()=>fail(`Manifest of family ${row.family} missing`,`Manifest der Familie ${row.family} fehlt`));try{return{row,bytes,manifest:requirePlayPageCohortMemberV1(cohort,row.family,bytes,"release")};}catch{return fail(`Manifest of family ${row.family} does not match the cohort`,`Manifest der Familie ${row.family} passt nicht zur Kohorte`);}}));
  const total=manifests.reduce((sum,row)=>sum+row.manifest.files.length+1,0),families:Family[]=[];let files=0,bytes=0;
  for(const {row,bytes:manifestBytes,manifest} of manifests){
   const copy=async(path:string,content:Uint8Array,expected:string):Promise<void>=>{
    check();const target=resolve(staging,row.family,path);if(!target.startsWith(resolve(staging,row.family)+sep))fail(`Path ${path} leaves its family`,`Pfad ${path} verlässt seine Familie`);
    if(digest(content)!==expected)fail(`Digest mismatch before copying ${row.family}/${path}`,`Prüfsumme weicht vor dem Kopieren ab: ${row.family}/${path}`);
    await mkdir(dirname(target),{recursive:true});await writeFile(target,content,{flag:"wx"});check();
    const written=await readFile(target);if(written.length!==content.length||digest(written)!==expected)fail(`Digest mismatch after copying ${row.family}/${path}`,`Prüfsumme weicht nach dem Kopieren ab: ${row.family}/${path}`);
    files+=1;bytes+=written.length;if(files%250===0||files===total)progress?.({family:row.family,files,total,bytes});
   };
   progress?.({family:row.family,files,total,bytes});
   for(const file of manifest.files){const content=await readFile(join(root,row.family,file.path));if(content.length!==file.bytes)fail(`Size mismatch of ${row.family}/${file.path}`,`Größe weicht ab: ${row.family}/${file.path}`);await copy(file.path,content,file.sha256);}
   await copy(publicationProtocol.manifest,manifestBytes,row.manifestSha256);
   const read=async(name:string):Promise<string>=>manifest.files.some(file=>file.path===name)?readFile(join(staging,row.family,name),"utf8"):"";
   const host=(await read("CNAME")).trim()||fail(`Family ${row.family} publishes no host`,`Familie ${row.family} veröffentlicht keinen Host`),extensions=[...new Set(manifest.files.map(file=>extensionOf(file.path)).filter(Boolean))].sort();
   families.push({family:row.family,host,directory:row.family,files:manifest.files.length+1,bytes:manifest.files.reduce((sum,file)=>sum+file.bytes,manifestBytes.length),headers:headersOf(await read("_headers")),contentTypes:extensions.filter(extension=>types[extension]).map(extension=>({extension,type:types[extension]})),unmappedExtensions:extensions.filter(extension=>!types[extension])});
  }
  await writeFile(join(staging,protocol.cohort),cohortBytes,{flag:"wx"});
  const descriptor:Cdn={version:1,mode:"release",cohortSha256:digest(cohortBytes),families,rules:{serveFamilyAtHostRoot:true,missingFileStatus:404,spaFallback:false,crossOriginIsolation:false,uploadOneGeneration:true,exactContentTypes:true}};
  if(validateJsonSchemaSubset(schema.definitions.Cdn,descriptor).length)fail("Closed CDN descriptor required","Geschlossener CDN-Deskriptor erforderlich");
  await writeFile(join(staging,protocol.descriptor),JSON.stringify(descriptor,null,2)+"\n",{flag:"wx"});check();
  await rm(out,{recursive:true,force:true});await mkdir(dirname(out),{recursive:true});await rename(staging,out);return descriptor;
 }catch(error){await rm(staging,{recursive:true,force:true});throw error;}
}

/** 🏃️ Runs `export` under the caller's roots: `[folder]` defaults to `🏢️semio-tech/🎡️play/dist/cdn`. Honours SIGINT and SIGTERM and the held lease control; reports progress on the console. */
export async function runExportScript(context:Readonly<{repoRoot:string;invocation:ScriptInvocation}>,args:string[]):Promise<void>{
 if(args.length>1||args[0]?.startsWith("-"))throw Error("export accepts at most one output folder / export akzeptiert höchstens einen Ausgabeordner");
 const out=args[0]?resolve(process.cwd(),args[0]):join(context.repoRoot,protocol.defaultDirectory),controller=new AbortController(),cancel=():void=>{controller.abort(Error("Play export cancelled / Play-Export abgebrochen"));process.exitCode=130;};
 process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
 try{await withPublishedPlayPagesV1(context.repoRoot,context.invocation,async pages=>{
  const descriptor=await exportPlayPagesV1({root:pages.root,out,check:()=>{controller.signal.throwIfAborted();checkResourceLeaseControl(pages.control);},progress:row=>console.log(`Play export ${row.family} ${row.files}/${row.total} files, ${row.bytes} bytes`)});
  console.log(`Play export complete: ${descriptor.families.length} families in ${out}; rules in ${join(out,protocol.descriptor)}`);
 });}finally{process.removeListener("SIGINT",cancel);process.removeListener("SIGTERM",cancel);}
}
