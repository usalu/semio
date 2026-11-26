import{afterAll,expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,symlinkSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join,resolve}from"node:path";
import ignore from"ignore";
import schema from"../🧬️schema/🔣️.json";
import{directoryFactOfV1}from"../../🟨️.mjs";
import{sourceTreeFilterV1}from"../🟨️.mjs";
import{declaredModuleGlobsV1}from"../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";

const rules=schema.$defs.rules.const,module=join(import.meta.dir,"..","..","🟨️.mjs"),inputs=join(import.meta.dir,"..","..","..","🕸️graph","📥️inference","🧭️script-inputs","🟨️.mjs"),assembly=join(import.meta.dir,"..","..","..","🕸️graph","📥️inference","🧱️project-assembly","🟨️.mjs"),root=(()=>{let directory=import.meta.dir;while(!existsSync(join(directory,"MODULE.bazel"))&&dirname(directory)!==directory)directory=dirname(directory);return directory;})();
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(names:readonly string[])=>createHash("sha256").update(JSON.stringify([...names].sort())).digest("hex");
const write=(base:string,path:string,text="x\n")=>{mkdirSync(dirname(join(base,path)),{recursive:true});writeFileSync(join(base,path),text);};

/** 🧱️ One machine: the same source tree, with or without the output a build and the operating system leave beside it. */
function machine(label:string,local:boolean):string{
 const base=mkdtempSync(join(tmpdir(),`source-names-${label}-`));scratch.push(base);
 for(const path of["package.json","📋️project.json","📜️script.ts","src/🟦️.ts","src/lib/🟦️.ts"])write(base,path);
 if(local){
  for(const path of["dist/index.js","📤️dist/index.js","node_modules/left-pad/index.js",".cache/b/x","🗑️generated/log.txt",".DS_Store","src/.DS_Store","Thumbs.db","notes.txt.swp"])write(base,path);
  mkdirSync(join(base,"target"));
 }
 return base;
}

const KINDS=["readdir","readdirNames","directories"] as const;

test("two machines with the same source tree and different build output produce the same directory fact for every kind that reads a directory",()=>{
 const clean=machine("clean",false),dirty=machine("dirty",true);
 for(const kind of KINDS)for(const directory of["",  "src"])expect(directoryFactOfV1(kind,join(dirty,directory)),kind+":"+directory).toBe(directoryFactOfV1(kind,join(clean,directory)));
});

test("a real source file, directory or rename changes the fact; a file that only shares the name of a generated directory is source",()=>{
 const base=machine("source",true),before=Object.fromEntries(KINDS.map(kind=>[kind,directoryFactOfV1(kind,base)]));
 write(base,"extra.ts");
 for(const kind of["readdir","readdirNames"] as const)expect(directoryFactOfV1(kind,base),kind).not.toBe(before[kind]);
 mkdirSync(join(base,"🧩️more"));expect(directoryFactOfV1("directories",base)).not.toBe(before.directories);
 const named=machine("named",false),plain=directoryFactOfV1("readdirNames",named);
 write(named,"build","a file\n");expect(directoryFactOfV1("readdirNames",named)).not.toBe(plain);
 const withBuild=directoryFactOfV1("readdirNames",named);write(named,"dist/x");expect(directoryFactOfV1("readdirNames",named)).toBe(withBuild);
});

test("a link named like a generated directory is machine-local too, and a missing directory keeps its error fact",()=>{
 const clean=machine("link-clean",false),linked=machine("linked",false),target=machine("target",false);
 symlinkSync(target,join(linked,"node_modules"));
 expect(directoryFactOfV1("readdirNames",linked)).toBe(directoryFactOfV1("readdirNames",clean));
 expect(directoryFactOfV1("readdirNames",join(clean,"absent"))).toBe("!ENOENT");
});

test("the fact equals the digest of the names an independent .gitignore reader keeps (the ignore library, the root .gitignore)",()=>{
 const reference=ignore().add(readFileSync(join(root,".gitignore"),"utf8")),dirty=machine("reference",true);
 const kept=readdirSync(dirty,{withFileTypes:true}).filter(entry=>!reference.ignores(entry.name+(entry.isDirectory()?"/":""))).map(entry=>entry.name);
 expect(kept.sort()).toEqual(["package.json","src","📋️project.json","📜️script.ts"].sort());
 expect(directoryFactOfV1("readdirNames",dirty)).toBe(sha(kept));
});

test("every declared noise name is ignored by the root .gitignore, so the build never sees it as source either",()=>{
 const reference=ignore().add(readFileSync(join(root,".gitignore"),"utf8"));
 for(const name of rules.machineLocalFileNames.filter((name:string)=>name!==".localized"))expect(reference.ignores(name),name).toBe(true);
 for(const suffix of rules.machineLocalFileSuffixes.filter((suffix:string)=>suffix!=="~"))expect(reference.ignores("a"+suffix),suffix).toBe(true);
});

test("the filter keeps what it must: directory names only leave where the entry is a directory",()=>{
 const names=["dist","src","build",".DS_Store","a.swp","a~"],isDirectory=(name:string)=>name!=="build";
 expect(sourceTreeFilterV1(names,name=>name,isDirectory)).toEqual(["src","build"]);
});

/** ⚖️ The law: the fact kinds that read a directory are exactly the declared ones, and no directory listing reaches a fact or a row without the rule. */
test("law: every fact kind that reads a directory is declared in the schema and goes through the source tree rule",()=>{
 const text=readFileSync(module,"utf8"),block=text.slice(text.indexOf("const facts = {"),text.indexOf("\n};",text.indexOf("const facts = {"))),declared=rules.readers.find((row:{module:string})=>row.module==="👁️observation")!.kinds;
 const entries=block.split(/\n(?=\s{2}[A-Za-z]+:)/u),found=entries.map(entry=>/^\s{2}([A-Za-z]+):/u.exec(entry)?.[1]).filter((name,index)=>name!==undefined&&/readdirSync|typescriptDirectoriesV1|typedListingV1/u.test(entries[index]!));
 expect(found.sort()).toEqual([...declared].sort());
 for(const entry of entries.filter(entry=>/readdirSync|typescriptDirectoriesV1|typedListingV1/u.test(entry)))expect(entry,entry).toContain("directoryFactV1(");
 const all=text.split("\n").filter(line=>/(?<![\w.])readdirSync\(|world\.readdirSync\(/u.test(line)&&!/^\s*(?:\/\*\*|\*)/u.test(line)&&!/function typescriptDirectoriesV1|world\.readdirSync\(path\|\|"\."/u.test(line)&&!/^(?:function|export function) directoryFactOfV1/u.test(line)&&!/hollowDirectoryV1\(/u.test(line));
 const listing=text.slice(text.indexOf("function typedListingV1("),text.indexOf("\n}\n",text.indexOf("function typedListingV1(")));
 const sanctioned=new Set(listing.split("\n")),reads=all.filter(line=>!sanctioned.has(line));
 for(const line of reads)expect(line,line).toContain("directoryFactV1(");
 expect(text).toContain("getDirectories: (path) => probe(\"directories\",path,()=>sys.getDirectories(path),value=>directoryFactV1(value))");
 expect(declared.length).toBe(3);
 const project=readFileSync(assembly,"utf8");
 expect(rules.readers.find((row:{module:string})=>row.module==="🧱️project-assembly")!.kinds).toEqual(["directories"]);
 for(const line of project.split("\n").filter(line=>/getDirectories\(path\)/u.test(line)&&/JSON\.stringify/u.test(line)))expect(line,line).toContain("sourceTreeFilterV1(");
});

test("law: the module names no list of generated directories of its own",()=>{
 const text=readFileSync(join(import.meta.dir,"..","🟨️.mjs"),"utf8");
 for(const name of["node_modules","📤️dist","\"dist\"",".cache"])expect(text,name).not.toContain(name);
});

test("a declared module glob matches only source: build output, dependencies and noise are never listed",()=>{
 const tree:Record<string,readonly string[]>={"":["src","dist","node_modules","📤️dist",".DS_Store","🧪️tests"],src:["a.ts","build","b.ts"],"src/build":["c.ts"],dist:["a.ts"],node_modules:["x.ts"],"📤️dist":["z.ts"],"🧪️tests":["t.ts","local~","t.ts.swp"]};
 const directories=new Set(["src","src/build","dist","node_modules","📤️dist","🧪️tests"]);
 const readdir=(directory:string)=>{const key=directory.replace(/^\/w\/?/u,"");return(tree[key]??[]).map(name=>{const path=key?key+"/"+name:name;return{name,directory:directories.has(path),file:!directories.has(path)};});};
 const found=declaredModuleGlobsV1(["{workspaceRoot}/**/*.ts"],"/w",".",{readdir,Glob:Bun.Glob as never,join:(...parts:string[])=>parts.filter(Boolean).join("/")});
 expect(found).toEqual(["src/a.ts","src/b.ts","🧪️tests/t.ts"]);
});

test("law: no second list of skipped directories remains in the script inputs, and every listing it reads for a glob or a walk is filtered",()=>{
 const text=readFileSync(inputs,"utf8");
 expect(text).not.toContain("DECLARED_SKIPPED_DIRECTORIES_V1");
 expect(text).toContain("sourceTreeFilterV1(readdir(join(workspaceRoot, relative))");
 expect(text).toContain("list = directory => { try { return sourceTreeFilterV1(observedFs.readdirSync(");
 expect(rules.readers.find((row:{module:string})=>row.module==="🧭️script-inputs")!.kinds).toEqual(["declared-module-globs","repository-walk"]);
});
