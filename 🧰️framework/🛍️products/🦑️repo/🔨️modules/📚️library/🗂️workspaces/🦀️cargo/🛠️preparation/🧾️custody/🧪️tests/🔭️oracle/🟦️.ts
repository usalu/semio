import{build}from"esbuild";
import{createRequire}from"node:module";
import{isAbsolute,resolve,relative}from"node:path";
/** 🔭️ Traverses all workspace imports with an independent compiler while installed tools remain external. */
export async function cargoPreparationSourceOracleV1(root:string,entries:readonly string[],outdir:string):Promise<readonly string[]> {
 const oracle=(await build({absWorkingDir:root,entryPoints:[...entries],outdir,bundle:true,write:false,metafile:true,platform:"node",format:"esm",logLevel:"silent",treeShaking:false,conditions:["bun","import"],plugins:[{name:"workspace-source",setup(builder){builder.onResolve({filter:/^[^./]/},args=>{if(args.kind==="entry-point"||isAbsolute(args.path))return;if(args.path.startsWith("node:")||args.path.startsWith("bun:"))return{external:true};const selected=createRequire(args.importer||import.meta.url).resolve(args.path);return!isAbsolute(selected)||selected.replaceAll("\\","/").split("/").includes("node_modules")?{external:true}:{path:selected};});}}]})).metafile!,queue=entries.map(path=>relative(root,path).replaceAll("\\","/")),reached=new Set<string>();
 for(const path of queue){if(reached.has(path))continue;reached.add(path);for(const edge of oracle.inputs[path]!.imports)if(!edge.external&&["import-statement","require-call"].includes(edge.kind))queue.push(edge.path);}
 return[...reached].map(path=>resolve(root,path)).sort();
}
