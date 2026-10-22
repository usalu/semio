import{expect,test}from"bun:test";
import{existsSync,readFileSync}from"node:fs";
import{dirname,join,posix,resolve}from"node:path";
import{fileURLToPath}from"node:url";
import{createRequire}from"node:module";
import{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1,nxAuthorizerCapturedImportsV1}from"../../🟦️.ts";
import{closureRefusalV1,closureSpecifierKindV1,closureTargetSchemesV1}from"../../🧭️sites/🟦️.ts";
import{astSitesV1}from"../../🧭️sites/🧪️tests/🔭️oracle/🟦️.ts";
import schema from"../../🧬️schema/🔣️.json";
import selection from"../../../../../../🏗️build/⌨️command/🧭️selection/🧬️schema/🔣️.json";

const posixRelative=(from:string,to:string)=>posix.relative(from,to),here=dirname(fileURLToPath(import.meta.url)),root=resolve(here,"../../../../../../../../../../.."),library="\u{1F9F0}️framework/\u{1F6CD}️products/\u{1F991}️repo/\u{1F528}️modules/\u{1F4DA}️library/",command=library+"\u{1F3D7}️build/⌨️command/";
type Row={owner:string;count:number;roots:readonly string[]};
const table=schema.$defs.ComputedImportSites.const as readonly Row[],kernels=Object.values(selection.properties.kernels.const as Record<string,string>).map(path=>command+path);
const compare=(left:string,right:string)=>Buffer.compare(Buffer.from(left),Buffer.from(right));

/** 📦️ Locates the installed manifest of one package through the module system of the checkout, so isolated and hoisted installs both answer. */
function installedManifestV1(name:string):string{
 const resolver=createRequire(import.meta.url);
 try{return resolver.resolve(name+"/package.json");}catch{}
 let directory=dirname(resolver.resolve(name));
 for(;;){const manifest=join(directory,"package.json");if(existsSync(manifest)&&(JSON.parse(readFileSync(manifest,"utf8"))as{name?:string}).name===name)return manifest;const parent=dirname(directory);if(parent===directory)throw Error("Installed package manifest required: "+name);directory=parent;}
}

/** 🔭️ The union of every file a repository command kernel can load, with the import maps the loader enforces. */
async function kernelClosuresV1(){
 const files=new Map<string,Uint8Array>(),imports=new Map<string,Readonly<Record<string,string>>>(),roots=new Map(table.filter(row=>row.roots.length).map(row=>[row.owner,row.roots]as const));
 for(const entry of kernels){const source=await captureNxAuthorizerSourcesV1(root,entry,new AbortController().signal,{installed:true});for(const[path,bytes]of nxAuthorizerCapturedBytesV1(source))files.set(path,bytes);for(const[path,map]of nxAuthorizerCapturedImportsV1(root,source))imports.set(path,map);}
 return{files,imports,roots};
}

/** 🧪️ No literal import, require, createRequire(...)(x) or resolve site of any command closure can be refused by the loader, and every computed site is declared once. */
test("the command closures contain no site the capture would refuse",async()=>{
 const{files,imports}=await kernelClosuresV1(),undeclared:string[]=[],computed=new Map<string,number>();
 for(const[path,bytes]of files){
  if(!/\.[cm]?[jt]sx?$/u.test(path)||/\.d\.[cm]?ts$/u.test(path))continue;
  const map=imports.get(path)!,sites=astSitesV1(path,Buffer.from(bytes).toString("utf8"));
  for(const site of sites){
   if(site.form==="computed"){computed.set(path,(computed.get(path)??0)+1);continue;}if(site.anchor==="other")continue;
   const specifier=site.specifier!,kind=closureSpecifierKindV1(specifier);
   if(kind==="builtin"||kind==="absolute"||kind==="scheme")continue;
   if(!Object.hasOwn(map,specifier))undeclared.push(path+" -> "+specifier);
  }
 }
 expect(undeclared.map(row=>closureRefusalV1("unresolved",{specifier:row.split(" -> ")[1],importer:row.split(" -> ")[0]},{LANG:"C"}).message.split(". ")[0]),"literal sites the loader would refuse").toEqual([]);
 const declared=new Map(table.map(row=>[row.owner,row.count]));
 const missing=[...computed].filter(([path,count])=>count>0&&declared.get(path)!==count).map(([path,count])=>closureRefusalV1("computed-site",{importer:path,detail:count+" computed site(s), table says "+(declared.get(path)??0)},{LANG:"C"}).message.split(". ")[0]);
 expect(missing.sort(compare),"computed sites outside the declared table").toEqual([]);
 const stale=table.filter(row=>row.count>0?!(computed.get(row.owner)??0):row.roots.length===0).map(row=>row.owner);
 expect(stale.sort(compare),"declared rows without a computed site in any command closure").toEqual([]);
},600000);

/** 🧪️ A data companion row the capture derives anyway is redundant and fails. */
test("declared data companions are not derivable",()=>{
 for(const row of schema.$defs.RuntimeDataCompanions.const){const owner=readFileSync(join(root,row.owner),"utf8"),base=dirname(row.owner);for(const path of row.paths){const relative=posixRelative(base,path),literal=new RegExp("([\"'`])"+(relative.startsWith(".")?relative:"./"+relative).replace(/[.*+?^${}()|[\]\\]/gu,"\\$&")+"\\1","u").test(owner);expect(path.startsWith(base+"/\u{1F9EC}\uFE0Fschema/")||literal,"derivable companion "+path).toBe(false);}}
});

/** 🧪️ Declared roots exist, enter every closure through their owner, and the derived package markers match the installed packages. */
test("declared roots and installed package markers hold",async()=>{
 const{files,imports,roots}=await kernelClosuresV1();
 for(const[owner,list]of roots){expect(files.has(owner),owner).toBe(true);for(const target of list){expect(existsSync(join(root,target)),target).toBe(true);expect(files.has(target),target).toBe(true);expect(imports.get(owner)![closureTargetSchemesV1.root+target],target).toBe(target);}}
 const markers=new Set<string>();for(const map of imports.values())for(const target of Object.values(map))if(target.startsWith(closureTargetSchemesV1.installed))markers.add(target);
 expect(markers.size).toBeGreaterThan(0);
 for(const marker of markers){const body=marker.slice(closureTargetSchemesV1.installed.length),at=body.lastIndexOf("@"),name=body.slice(0,at),version=body.slice(at+1),manifest=JSON.parse(readFileSync(installedManifestV1(name),"utf8"))as{version:string};expect(manifest.version,marker).toBe(version);}
},600000);
