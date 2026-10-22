export { BAZEL_HERMETIC_RUNNER } from "./🍞️bun/🟦️.ts";


/** 🏭️ The self-contained program that builds the content-addressed dependency tree of a hermetic action: it copies the named store entries of the installed generations into one directory `<store index>/<identity>`, keeps every symbolic link relative and inside its store (so the tree is relocatable) and fails on an absent entry or an escaping link. It reads the installed stores by the absolute paths of its manifest, so it is a host-state action; its output is what hermetic actions take as an input by content. */
export const BAZEL_HERMETIC_DEPENDENCIES=String.raw`import{constants,copyFileSync,lstatSync,mkdirSync,readFileSync,readdirSync,readlinkSync,symlinkSync}from"node:fs";
import{dirname,isAbsolute,join,relative,resolve,sep}from"node:path";
const[manifestPath,outputArgument]=process.argv.slice(2),fail=message=>{throw new Error("[hermetic-dependencies] "+message);};
if(!manifestPath||!outputArgument)fail("manifest and output required");
const manifest=JSON.parse(readFileSync(manifestPath,"utf8")),output=resolve(outputArgument);
if(manifest.version!==1)fail("manifest version");
const copy=(from,to,store)=>{
 const info=lstatSync(from);
 if(info.isSymbolicLink()){const target=readlinkSync(from);if(isAbsolute(target))fail("an absolute link is not relocatable: "+from);const inner=relative(store,resolve(dirname(from),target));if(inner===".."||inner.startsWith(".."+sep)||isAbsolute(inner))fail("a link leaves its store: "+from);mkdirSync(dirname(to),{recursive:true});symlinkSync(target,to);return;}
 if(info.isDirectory()){mkdirSync(to,{recursive:true});for(const name of readdirSync(from).sort())copy(join(from,name),join(to,name),store);return;}
 if(!info.isFile())fail("a dependency entry must be a file, directory or link: "+from);
 mkdirSync(dirname(to),{recursive:true});try{copyFileSync(from,to,constants.COPYFILE_FICLONE);}catch{copyFileSync(from,to);}
};
mkdirSync(output,{recursive:true});
for(const id of manifest.ids){const cut=id.indexOf("/"),store=manifest.stores[Number(id.slice(0,cut))];if(!store)fail("store index "+id);copy(join(store,id.slice(cut+1)),join(output,id),store);}
`;
