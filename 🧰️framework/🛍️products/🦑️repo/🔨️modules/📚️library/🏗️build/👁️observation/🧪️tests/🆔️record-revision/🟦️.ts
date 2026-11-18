import{afterAll,expect,test}from"bun:test";
import{copyFileSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{pathToFileURL}from"node:url";
import{createHash}from"node:crypto";

const source=join(import.meta.dir,"..","..","🟨️.mjs"),schemaPath=join(import.meta.dir,"..","..","🧬️schema","🔣️.json"),scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`record-revision-${label}-`));scratch.push(path);return path;};

/** 🆔️ The text the module names its record revision with when its own bytes are `edit`ed (a trailing comment) and its schema is `stamp`ed: the module is loaded from the real path and told by a loader plugin that its physical source is the edited copy. */
async function revisionOf(label:string,edit:string,stamp?:string):Promise<string>{
 const directory=temp(label),physical=join(directory,"🟨️.mjs");
 mkdirSync(join(directory,"🧬️schema"));
 writeFileSync(physical,readFileSync(source,"utf8")+edit);
 if(stamp===undefined)copyFileSync(schemaPath,join(directory,"🧬️schema","🔣️.json"));
 else writeFileSync(join(directory,"🧬️schema","🔣️.json"),readFileSync(schemaPath,"utf8").replace("semio.graph-record/2",stamp));
 const preload=join(directory,"preload.mjs");
 writeFileSync(preload,`import{plugin}from"bun";plugin({name:"physical",setup(build){build.onLoad({filter:/👁️observation\\/🟨️\\.mjs$/u},async({path})=>({contents:"import.meta.semioFrozenAuthorizers=true;import.meta.semioPhysicalSourceURL="+${JSON.stringify(JSON.stringify(pathToFileURL(physical).href))}+";\\n"+await Bun.file(path).text(),loader:"js"}));}});`);
 const probe=join(directory,"probe.mjs");
 writeFileSync(probe,`import{projectRecordKeyTextV1}from${JSON.stringify(pathToFileURL(source).href)};process.stdout.write(projectRecordKeyTextV1(null));`);
 const child=Bun.spawnSync([process.execPath,"--preload",preload,probe],{cwd:directory,env:{...process.env},stdout:"pipe",stderr:"pipe"});
 if(child.exitCode!==0)throw new Error(child.stderr.toString().slice(0,2000));
 return child.stdout.toString();
}

test("the record revision is the schema-declared stamp of the record rules: editing the module that computes records leaves it unchanged, bumping the stamp moves it",async()=>{
 const stamp=JSON.parse(readFileSync(schemaPath,"utf8")).$defs.record.const.stamp as string;
 const plain=await revisionOf("plain","\n"),edited=await revisionOf("edited","\n// an unrelated edit of the module\nexport const unrelatedV1=1;\n"),bumped=await revisionOf("bumped","\n",stamp+"-next");
 expect(plain).toBe(JSON.stringify([stamp,null,null]));
 expect(edited,"editing the bytes of the module does not move the revision").toBe(plain);
 expect(bumped,"declaring another record stamp moves it").toBe(JSON.stringify([stamp+"-next",null,null]));
 expect(plain).not.toContain(createHash("sha256").update(readFileSync(source)).digest("hex"));
},120000);
