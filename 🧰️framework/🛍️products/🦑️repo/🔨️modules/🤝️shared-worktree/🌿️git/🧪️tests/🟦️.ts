import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,renameSync,rmSync,statSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {testPythonV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {formatHlcV1} from "../../📒️journal/🟦️.ts";
import {refusePathV1} from "../../🚧️boundary/🟦️.ts";
import {applyIndexV1,assembleViewV1,crlfScriptsV1,diffTreeV1,emptyTreeV1,foldRegistersV1,forgetGitV1,gitCandidatesV1,gitGuardsV1,gitPinsHashV1,gitStateV1,gitViewDeltaV1,gitViewV1,GIT_PINS_V1,GIT_VIEW_PINS_V1,spellingV1,volumeV1,INDEX_BASE_V1,INDEX_PREFIX_V1,indexRealmV1,linearizeRefV1,mergeTreesV1,missingObjectsV1,normaliseGitConfigV1,sweepGitGarbageV1,publishIndexV1,readTreeV1,requestRebaseV1,resolveGitV1,stageHealthV1,unmergedV1,updateIndexV1,verifyIndexV1,writeTreeV1,type IndexHostV1} from "../🟦️.ts";

type Tree={path:string;mode:string;file:string};
type Fold=(typeof fixture.folds)[number];
const ajv=new Ajv({strict:true,allowUnionTypes:true}),python=testPythonV1(),oracle=join(import.meta.dirname,"🐍️.py"),scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`git-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const environment={...process.env,GIT_AUTHOR_NAME:"t",GIT_AUTHOR_EMAIL:"t@example.test",GIT_COMMITTER_NAME:"t",GIT_COMMITTER_EMAIL:"t@example.test",GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"};
function cli(worktree:string,args:string[],input?:string,extra:Record<string,string>={}):string{
 const result=Bun.spawnSync(["git",...args],{cwd:worktree,env:{...environment,...extra},stdin:input===undefined?undefined:Buffer.from(input),stdout:"pipe",stderr:"pipe"});
 if(result.exitCode!==0)throw Error(`git ${args.join(" ")}: ${result.stderr.toString()}`);
 return result.stdout.toString();
}
function repository(label:string):string{const root=temp(label);cli(root,["init","-q","-b","main"]);return root;}
const hasGit=Bun.spawnSync(["git","--version"]).exitCode===0,gitTest=hasGit?test:test.skip;

test("the fixture is valid against the schema and every fold names known files",()=>{
 const validate=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/Fixture"});
 expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 for(const fold of fixture.folds){
  const names=new Set(fold.files.map(file=>file.name));
  for(const entry of [...fold.base,...fold.expect,...fold.registers.filter(register=>"file" in register)])expect(names.has((entry as {file:string}).file)).toBe(true);
 }
});

test("the realm covers the base and the entries and nothing else under .cache/f",()=>{
 expect(indexRealmV1(INDEX_BASE_V1)).toBe(true);
 expect(indexRealmV1(`${INDEX_PREFIX_V1}a/b.txt`)).toBe(true);
 for(const path of [".cache/f/index",".cache/f/state.json",".cache/f/indexed/a","src/index","a.txt"])expect(indexRealmV1(path)).toBe(false);
});

test("candidates cover PATH, SEMIO_GIT and the install locations of every operating system, Git for Windows included",()=>{
 const windowsList=gitCandidatesV1({PATH:"C:\\tools;C:\\bin",SEMIO_GIT:"D:\\g\\git.exe",ProgramFiles:"C:\\Program Files","ProgramFiles(x86)":"C:\\Program Files (x86)",LOCALAPPDATA:"C:\\Users\\w\\AppData\\Local"},"win32");
 expect(windowsList[0]).toBe("D:\\g\\git.exe");
 for(const wanted of ["C:\\tools\\git.exe","C:\\Program Files\\Git\\cmd\\git.exe","C:\\Program Files (x86)\\Git\\cmd\\git.exe","C:\\Users\\w\\AppData\\Local\\Programs\\Git\\cmd\\git.exe"])expect(windowsList).toContain(wanted);
 expect(gitCandidatesV1({PATH:"/usr/local/sbin:/opt/x"},"darwin")).toEqual(expect.arrayContaining(["/opt/x/git","/usr/bin/git","/opt/homebrew/bin/git"]));
 expect(gitCandidatesV1({PATH:""},"linux")).toEqual(expect.arrayContaining(["/usr/bin/git","/usr/local/bin/git"]));
});

gitTest("git is resolved, and a worktree without a root .git or with a broken one is absent and never throws",async()=>{
 expect(await resolveGitV1()).not.toBeNull();
 const empty=temp("empty");
 expect(await gitStateV1(empty)).toEqual({present:false,head:null,commit:null,indexTree:null});
 const broken=temp("broken");mkdirSync(join(broken,".git"));writeFileSync(join(broken,".git","HEAD"),"nonsense");
 expect(await gitStateV1(broken)).toEqual({present:false,head:null,commit:null,indexTree:null});
 expect(await writeTreeV1(empty)).toBeNull();
 expect(await diffTreeV1(empty,null,"0".repeat(40))).toBeNull();
 expect(await updateIndexV1(empty,[{path:"a",mode:"100644",oid:"1".repeat(40)}])).toBe(false);
 expect(await normaliseGitConfigV1(empty)).toEqual([]);
});

gitTest("gitStateV1 reports head, commit and stage tree as git does, before and after a commit",async()=>{
 const root=repository("state");
 let state=await gitStateV1(root);
 expect(state.present).toBe(true);expect(state.head).toBe("refs/heads/main");expect(state.commit).toBeNull();
 expect(state.indexTree).toBe(cli(root,["write-tree"]).trim());
 writeFileSync(join(root,"a.txt"),"a\n");cli(root,["add","a.txt"]);
 state=await gitStateV1(root);
 expect(state.indexTree).toBe(cli(root,["write-tree"]).trim());
 cli(root,["commit","-q","-m","one"]);
 state=await gitStateV1(root);
 expect(state.commit).toBe(cli(root,["rev-parse","HEAD"]).trim());
 cli(root,["checkout","-q","--detach"]);
 forgetGitV1(root);
 state=await gitStateV1(root);
 expect(state.head).toBeNull();expect(state.commit).toBe(cli(root,["rev-parse","HEAD"]).trim());
});

gitTest("diff-tree, update-index, read-tree and cat-file agree with git ls-tree",async()=>{
 const root=repository("plumbing");
 const blob=cli(root,["hash-object","-w","--stdin"],"one\n").trim(),other=cli(root,["hash-object","-w","--stdin"],"two\n").trim();
 expect(await updateIndexV1(root,[{path:"d/x",mode:"100644",oid:blob},{path:"y y",mode:"100755",oid:other}])).toBe(true);
 const first=(await writeTreeV1(root))!;
 expect(first).toBe(cli(root,["write-tree"]).trim());
 expect(await updateIndexV1(root,[{path:"d/x",mode:"000000",oid:""},{path:"d",mode:"100644",oid:other}])).toBe(true);
 const second=(await writeTreeV1(root))!;
 const changes=(await diffTreeV1(root,first,second))!;
 expect(changes.map(change=>`${change.mode} ${change.path}`).sort()).toEqual(["000000 d/x","100644 d"]);
 const listed=cli(root,["ls-tree","-r","-z",second]).split("\0").filter(Boolean).map(line=>{const [head,path]=line.split("\t");return `${head!.split(" ")[0]} ${path}`;});
 expect(listed).toEqual(["100644 d","100755 y y"]);
 expect((await diffTreeV1(root,null,second))!.map(change=>change.path)).toEqual(["d","y y"]);
 expect(await readTreeV1(root,first)).toBe(true);
 expect(await writeTreeV1(root)).toBe(first);
 expect(await emptyTreeV1(root)).toBe(cli(root,["hash-object","-t","tree","--stdin"],"").trim());
 const missing=await missingObjectsV1(root,[blob,"2".repeat(40)]);
 expect([...missing!]).toEqual(["2".repeat(40)]);
 expect(await updateIndexV1(root,[{path:"gone",mode:"000000",oid:""}])).toBe(true);
});

gitTest.each(fixture.folds.map((fold,index)=>[fold.name,index] as const))("the fold %s equals the tree of the Python oracle driving the git command line",async(name,index)=>{
 const ran=Bun.spawnSync([python!,"-I",oracle],{stdout:"pipe",stderr:"pipe"});
 expect(ran.exitCode,ran.stderr.toString()).toBe(0);
 const expected=(JSON.parse(ran.stdout.toString()) as {trees:string[]}).trees[index]!,fold=fixture.folds[index] as Fold;
 const root=repository(`fold-${index}`),blobs=new Map(fold.files.map(file=>[file.name,cli(root,["hash-object","-w","--stdin"],file.content).trim()] as const));
 const baseIndex=join(root,".git","base-index");
 for(const entry of fold.base as Tree[])cli(root,["update-index","--add","--cacheinfo",`${entry.mode},${blobs.get(entry.file)},${entry.path}`],undefined,{GIT_INDEX_FILE:baseIndex});
 const baseTree=cli(root,["write-tree"],undefined,{GIT_INDEX_FILE:baseIndex}).trim();
 const registers=(fold.registers as {hlc:string;path:string;file?:string;mode?:string;remove?:true}[]).map(register=>[`${INDEX_PREFIX_V1}${register.path}`,register.remove?{hlc:register.hlc,operation:"delete" as const}:{hlc:register.hlc,operation:"link" as const,target:`${register.mode} ${blobs.get(register.file!)}`}] as const);
 const tree=await foldRegistersV1(root,{hlc:fold.baseHlc,operation:"link",target:`tree ${baseTree}`},registers);
 expect(name).toBeTruthy();
 expect(tree).toBe(expected);
 const listed=(await diffTreeV1(root,null,tree!))!.map(change=>`${change.mode} ${change.oid} ${change.path}`);
 expect(listed).toEqual((fold.expect as Tree[]).map(entry=>`${entry.mode} ${blobs.get(entry.file)} ${entry.path}`));
});

gitTest("the configuration is normalised through git, keeps remotes, drops a foreign commit template and changes nothing the second time",async()=>{
 const root=repository("config");
 cli(root,["config","core.autocrlf","true"]);cli(root,["config","core.filemode","false"]);cli(root,["config","core.fsmonitor","true"]);Bun.spawnSync(["git","config","--unset","core.precomposeunicode"],{cwd:root,env:environment});
 cli(root,["config","remote.origin.url","https://example.test/r.git"]);cli(root,["config","user.name","kept"]);cli(root,["config","commit.template","/somewhere/else/template.txt"]);
 const changed=await normaliseGitConfigV1(root,"linux");
 expect(changed).toEqual(expect.arrayContaining(["core.autocrlf","core.filemode","core.fsmonitor","core.untrackedCache","index.version","gc.auto","gc.autoPackLimit","gc.autoDetach","gc.writeCommitGraph","fetch.writeCommitGraph","receive.autogc","maintenance.auto","commit.template"]));
 const value=(key:string)=>{const result=Bun.spawnSync(["git","config","--local","--get",key],{cwd:root,env:environment,stdout:"pipe"});return result.exitCode===0?result.stdout.toString().trim():null;};
 expect(value("core.autocrlf")).toBe("false");expect(value("core.filemode")).toBe("true");expect(value("core.fsmonitor")).toBe("false");expect(value("core.untrackedCache")).toBe("true");expect(value("index.version")).toBe("4");expect(value("gc.auto")).toBe("0");
 expect(value("gc.autopacklimit")).toBe("0");expect(value("gc.autodetach")).toBe("false");expect(value("gc.writecommitgraph")).toBe("false");expect(value("fetch.writecommitgraph")).toBe("false");expect(value("receive.autogc")).toBe("false");
 expect(value("maintenance.auto")).toBe("false");expect(value("core.logallrefupdates")).toBe("true");expect(value("core.symlinks")).not.toBeNull();expect(value("commit.template")).toBeNull();
 expect(value("remote.origin.url")).toBe("https://example.test/r.git");expect(value("user.name")).toBe("kept");
 expect(await normaliseGitConfigV1(root,"linux")).toEqual([]);
 expect(await normaliseGitConfigV1(root,"darwin")).toEqual(["core.precomposeunicode"]);
 expect(await normaliseGitConfigV1(root,"win32")).toEqual(["core.filemode","core.longpaths"]);
 expect(value("core.safecrlf")).toBe("false");expect(value("core.eol")).toBe("lf");expect(value("core.protectntfs")).toBe("true");expect(value("core.protecthfs")).toBe("true");
 mkdirSync(join(root,"tpl"));writeFileSync(join(root,"tpl","t.txt"),"");cli(root,["config","commit.template",join(root,"tpl","t.txt")]);
 expect(await normaliseGitConfigV1(root,"linux")).toEqual(["core.filemode"]);
 expect(value("commit.template")).toBe(join(root,"tpl","t.txt"));
});

gitTest("the view of a real repository equals what git plumbing says, never touches the index, and is null without a repository or after an abort",async()=>{
 const root=repository("view"),put=(path:string,text:string)=>{const file=join(root,...path.split("/"));mkdirSync(join(file,".."),{recursive:true});writeFileSync(file,text);};
 for(const path of ["a.txt","b.txt","c.txt","d e.txt","gone.txt","moved.txt"])put(path,`${path}\n`);
 cli(root,["add","-A"]);cli(root,["commit","-q","-m","initial"]);
 put("a.txt","a changed\n");
 put("b.txt","b staged\n");cli(root,["add","b.txt"]);
 put("c.txt","c staged\n");cli(root,["add","c.txt"]);put("c.txt","c changed again\n");
 rmSync(join(root,"d e.txt"));
 cli(root,["rm","-q","gone.txt"]);
 cli(root,["mv","moved.txt","moved too.txt"]);
 put("added.txt","added\n");cli(root,["add","added.txt"]);put("added.txt","added and changed\n");
 put("untracked.txt","u\n");put("ünï/日本 \u{1F9EC}.txt","u\n");put("line\nbreak.txt","u\n");put("dir with space/x y.txt","u\n");
 const names=(args:string[]):string[]=>cli(root,args).split("\0").filter(Boolean).map(name=>name.normalize("NFC"));
 const deleted=new Set(names(["ls-files","-d","-z"])),modified=names(["ls-files","-m","-z"]).filter(name=>!deleted.has(name)),untracked=names(["ls-files","-o","--exclude-standard","-z"]),carried=untracked.filter(name=>refusePathV1(name)===undefined);
 const cached=cli(root,["diff","--cached","--name-status","-z","--no-renames"]).split("\0").filter(Boolean),letters=new Map<string,string>();
 for(let index=0;index+1<cached.length;index+=2)letters.set(cached[index+1]!.normalize("NFC"),cached[index]!);
 const lines=[...carried.map(name=>`?? ${name}`),...[...new Set([...modified,...deleted])].map(name=>`${letters.get(name)??" "}${deleted.has(name)?"D":"M"} ${name}`)].sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right)));
 const before=statSync(join(root,".git","index"),{bigint:true});
 utimesSync(join(root,"b.txt"),new Date(1e12),new Date(1e12));
 const view=(await gitViewV1(root))!;
 expect(view).not.toBeNull();
 expect(view.lines).toEqual(lines);
 expect(view).toMatchObject({staged:letters.size,modified:modified.length,deleted:deleted.size,untracked:carried.length,digest:createHash("sha256").update(lines.join("\n")).digest("hex")});
 expect(view.staged).toBe(6);
 expect(view.lines).toContain(" D d e.txt");
 expect(view.lines).toContain("AM added.txt");
 expect(view.lines).toContain("MM c.txt");
 expect(untracked).toContain("line\nbreak.txt");
 expect(view.lines).not.toContain("?? line\nbreak.txt");
 expect(view.lines.some(line=>line.startsWith("M  b.txt")||line.includes("gone.txt")||line.includes("moved"))).toBe(false);
 const after=statSync(join(root,".git","index"),{bigint:true});
 expect([after.ino,after.mtimeNs,after.size]).toEqual([before.ino,before.mtimeNs,before.size]);
 writeFileSync(join(root,".git","index.lock"),"");
 expect((await gitViewV1(root))!.digest).toBe(view.digest);
 rmSync(join(root,".git","index.lock"));
 expect(await gitViewV1(temp("plain"))).toBeNull();
 const aborted=new AbortController();aborted.abort();
 expect(await gitViewV1(root,aborted.signal)).toBeNull();
 const clean=repository("clean");
 expect(await gitViewV1(clean)).toMatchObject({staged:0,modified:0,deleted:0,untracked:0,lines:[],digest:createHash("sha256").update("").digest("hex")});
});

gitTest("an incremental view equals the complete view after every kind of change and is refused for a .gitignore or too many paths",async()=>{
 const root=repository("delta"),put=(path:string,text:string)=>{mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),text);};
 put(".gitignore","ignored/\n*.log\n");put("a.txt","a\n");put("b.txt","b\n");put("d/e.txt","e\n");put("d/f/g.txt","g\n");put("h i.txt","h\n");put("ünï/日本.txt","u\n");
 cli(root,["add","-A"]);cli(root,["commit","-q","-m","base"]);
 let view=(await gitViewV1(root))!,tree=cli(root,["write-tree"]).trim();
 const steps:{label:string;paths:string[];restage?:boolean;act:()=>void}[]=[
  {label:"modify",paths:["a.txt"],act:()=>put("a.txt","a2\n")},
  {label:"untracked",paths:["n/new.txt","n/deep/er.txt"],act:()=>{put("n/new.txt","n\n");put("n/deep/er.txt","n\n");}},
  {label:"ignored",paths:["ignored/x.txt","x.log"],act:()=>{put("ignored/x.txt","i\n");put("x.log","l\n");}},
  {label:"delete tracked",paths:["b.txt"],act:()=>rmSync(join(root,"b.txt"))},
  {label:"delete untracked",paths:["n/new.txt"],act:()=>rmSync(join(root,"n/new.txt"))},
  {label:"directory removed",paths:["d/e.txt","d/f/g.txt"],act:()=>rmSync(join(root,"d"),{recursive:true})},
  {label:"stage",paths:[],restage:true,act:()=>{cli(root,["add","a.txt","n"]);}},
  {label:"stage and change",paths:["a.txt"],restage:true,act:()=>{put("a.txt","a3\n");cli(root,["rm","-q","--cached","n/deep/er.txt"]);}},
  {label:"file becomes directory",paths:["h i.txt","h i.txt/inner.txt"],act:()=>{rmSync(join(root,"h i.txt"));put("h i.txt/inner.txt","x\n");}},
  {label:"directory becomes file",paths:["h i.txt/inner.txt","h i.txt"],act:()=>{rmSync(join(root,"h i.txt"),{recursive:true});put("h i.txt","back\n");}},
  {label:"unstage",paths:[],restage:true,act:()=>cli(root,["reset","-q"])},
  {label:"unicode and space",paths:["ünï/日本.txt","sp ace/x y.txt"],act:()=>{put("ünï/日本.txt","u2\n");put("sp ace/x y.txt","s\n");}},
  {label:"commit",paths:[],restage:true,act:()=>{cli(root,["add","-A"]);cli(root,["commit","-q","-m","two"]);}},
  {label:"mixed reset",paths:[],restage:true,act:()=>cli(root,["reset","-q","--mixed","HEAD~1"])},
 ];
 for(const step of steps){
  step.act();
  const next=await gitViewDeltaV1(root,view,{paths:step.paths,restage:step.restage===true?{before:tree,after:tree=cli(root,["write-tree"]).trim()}:null}),full=(await gitViewV1(root))!;
  expect(next,step.label).not.toBeNull();
  expect(next!.lines,step.label).toEqual(full.lines);
  expect(next,step.label).toMatchObject({staged:full.staged,modified:full.modified,deleted:full.deleted,untracked:full.untracked,digest:full.digest});
  expect([...next!.stage].sort(),step.label).toEqual([...full.stage].sort());
  view=next!;
 }
 expect(await gitViewDeltaV1(root,view,{paths:["sub/.gitignore"],restage:null})).toBeNull();
 expect(await gitViewDeltaV1(root,view,{paths:Array.from({length:20_001},(_,index)=>`p${index}`),restage:null})).toBeNull();
 expect(await gitViewDeltaV1(root,view,{paths:[".git/index",".git/HEAD"],restage:null})).toBe(view);
 expect(await gitViewDeltaV1(temp("plain"),view,{paths:["a.txt"],restage:null})).toBeNull();
 const aborted=new AbortController();aborted.abort();
 expect(await gitViewDeltaV1(root,view,{paths:["a.txt"],restage:null},aborted.signal)).toBeNull();
 put(":(glob)*.txt","literal\n");
 const literal=await gitViewDeltaV1(root,view,{paths:[":(glob)*.txt"],restage:null}),every=(await gitViewV1(root))!;
 expect(literal!.lines).toEqual(every.lines);
});

type Register={hlc:string;operation:"link"|"delete";target?:string};
const stamp=(offset:number,origin:string)=>formatHlcV1({ms:1_700_000_000_000+offset,counter:0},origin);
function fake(worktree:string,options:{server?:boolean;origin?:string}={}){
 const manifest=new Map<string,Register>(),applied=new Map<string,Register>(),queue=new Set<string>(),events:Record<string,unknown>[]=[],origin=options.origin??(options.server===true?"g1-server":"g1-m01");
 let clock=1_800_000_000_000,tick=100;
 const value:IndexHostV1={
  worktree,server:options.server??false,archivist:"g1-server",now:()=>clock,
  base:()=>manifest.get(INDEX_BASE_V1),
  each:visit=>{for(const [path,entry] of manifest)if(path.startsWith(INDEX_PREFIX_V1))visit(path,entry);},
  held:visit=>{for(const [path,entry] of applied)visit(path,entry);},
  pending:()=>new Map([...queue].map(path=>[path,manifest.get(path)!] as const)),
  emit:async list=>{const hlcs:string[]=[];for(const event of list){const entry:Register={hlc:stamp(tick++,origin),operation:event.operation,...(event.target===undefined?{}:{target:event.target})};manifest.set(event.path,entry);applied.set(event.path,entry);hlcs.push(entry.hlc);}return hlcs;},
  settle:paths=>{for(const path of paths){queue.delete(path);const entry=manifest.get(path);if(entry!==undefined)applied.set(path,entry);}},
  enqueue:paths=>{for(const path of paths)queue.add(path);},
  say:event=>{events.push({...event});}
 };
 return Object.assign(value,{manifest,applied,queue,events,advance:(milliseconds:number)=>{clock+=milliseconds;},receive:(path:string,entry:Register)=>{manifest.set(path,entry);queue.add(path);},named:(reason:string)=>events.filter(event=>event.reason===reason||event.event===reason)});
}
const blobOf=(root:string,text:string):string=>cli(root,["hash-object","-w","--stdin"],text).trim();
const staging=(root:string,path:string,oid:string,mode:string="100644"):void=>{cli(root,["update-index","--add","--cacheinfo",`${mode},${oid},${path}`]);};
const files=(root:string,extra:Record<string,string>={}):string[]=>cli(root,["ls-files","-s","-z"],undefined,extra).split("\0").filter(Boolean).map(line=>{const [head,path]=line.split("\t");const [mode,oid]=head!.split(" ");return `${mode} ${oid} ${path}`;});
function treeFrom(root:string,entries:Record<string,string>):string{
 const index=join(root,".git",`scratch-${Math.random().toString(16).slice(2)}`);
 for(const [path,oid] of Object.entries(entries))cli(root,["update-index","--add","--cacheinfo",`100644,${oid},${path}`],undefined,{GIT_INDEX_FILE:index});
 const tree=cli(root,["write-tree"],undefined,{GIT_INDEX_FILE:index}).trim();
 rmSync(index,{force:true});
 return tree;
}
const kind=(path:string):string=>readFileSync(path,"utf8");
const lostEntries=(root:string):string[]=>existsSync(join(root,".cache","f","lost"))?readdirSync(join(root,".cache","f","lost")):[];

gitTest("a fold-derived known tree publishes a local stage even when the local record is garbage, and an unchanged stage publishes nothing",async()=>{
 const root=repository("known"),host=fake(root,{server:true});
 staging(root,"a.txt",blobOf(root,"a\n"));
 expect((await publishIndexV1(host)).base).toBe(true);
 staging(root,"b.txt",blobOf(root,"b\n"));
 expect((await publishIndexV1(host)).published).toBe(1);
 mkdirSync(join(root,".cache","f"),{recursive:true});
 writeFileSync(join(root,".cache","f","git-index.json"),"{garbage");
 forgetGitV1(root);
 staging(root,"c.txt",blobOf(root,"c\n"));
 const result=await publishIndexV1(host);
 expect(result).toMatchObject({published:1,base:false});
 expect([...host.manifest.keys()].filter(path=>path.startsWith(INDEX_PREFIX_V1)).sort()).toEqual([`${INDEX_PREFIX_V1}b.txt`,`${INDEX_PREFIX_V1}c.txt`]);
 forgetGitV1(root);
 expect(await publishIndexV1(host)).toMatchObject({published:0});
});

gitTest("a stage with no proof of its origin is replaced by the fold, the old index file is kept under lost and an event names the replaced entries",async()=>{
 const root=repository("noproof"),host=fake(root);
 const local=blobOf(root,"mine\n"),base=blobOf(root,"base\n"),entry=blobOf(root,"entry\n");
 staging(root,"a.txt",local);
 host.receive(INDEX_BASE_V1,{hlc:stamp(1,"g1-server"),operation:"link",target:`tree ${treeFrom(root,{"base.txt":base})}`});
 host.receive(`${INDEX_PREFIX_V1}e.txt`,{hlc:stamp(2,"g1-server"),operation:"link",target:`100644 ${entry}`});
 await applyIndexV1(host);
 expect(files(root)).toEqual([`100644 ${base} base.txt`,`100644 ${entry} e.txt`]);
 const kept=host.events.find(event=>event.event==="overwritten");
 expect(kept).toMatchObject({stage:"git",reason:"local-stage-replaced",count:3});
 const name=String(kept!.lost).replace("lost/","");
 expect(lostEntries(root)).toContain(name);
 expect(files(root,{GIT_INDEX_FILE:join(root,".cache","f","lost",name)})).toEqual([`100644 ${local} a.txt`]);
 expect(host.queue.size).toBe(0);
 expect((await stageHealthV1(root)).state).toBe("shared");
});

gitTest("a missing .git/index is restored from the registers without publishing a delete",async()=>{
 const root=repository("restore"),host=fake(root,{server:true});
 staging(root,"a.txt",blobOf(root,"a\n"));staging(root,"d/b.txt",blobOf(root,"b\n"));
 await publishIndexV1(host);
 const before=files(root),count=host.manifest.size;
 rmSync(join(root,".git","index"));
 await publishIndexV1(host);
 expect(files(root)).toEqual(before);
 expect(host.manifest.size).toBe(count);
 expect(host.events.some(event=>event.event==="repaired"&&event.reason==="index-missing")).toBe(true);
});

gitTest("unmerged entries are reported once, never published, never reset by an apply, and sharing resumes after they are resolved",async()=>{
 const root=repository("unmerged"),host=fake(root,{server:true}),one=blobOf(root,"1\n"),two=blobOf(root,"2\n"),three=blobOf(root,"3\n");
 staging(root,"a.txt",one);
 await publishIndexV1(host);
 cli(root,["update-index","--index-info"],`100644 ${one} 1\tu.txt\n100644 ${two} 2\tu.txt\n100644 ${three} 3\tu.txt\n`);
 expect(await unmergedV1(root)).toBe(3);
 const count=host.manifest.size;
 for(let round=0;round<2;round++){forgetGitV1(root);await publishIndexV1(host);}
 expect(host.manifest.size).toBe(count);
 expect(host.events.filter(event=>event.reason==="unmerged")).toHaveLength(1);
 expect(await stageHealthV1(root)).toEqual({state:"degraded",reason:"unmerged"});
 host.receive(INDEX_BASE_V1,{hlc:stamp(500,"g1-server"),operation:"link",target:`tree ${treeFrom(root,{"z.txt":one})}`});
 const bytes=readFileSync(join(root,".git","index"));
 expect((await applyIndexV1(host)).queued).toBe(1);
 expect(readFileSync(join(root,".git","index")).equals(bytes)).toBe(true);
 expect(host.queue.size).toBe(1);
 staging(root,"u.txt",two);
 host.advance(60_000);
 const result=await applyIndexV1(host);
 expect(result.queued).toBe(0);
 expect(await stageHealthV1(root)).toEqual({state:"shared",reason:null});
 expect(files(root)).toEqual([`100644 ${one} z.txt`]);
});

gitTest("an object that has not arrived is a waiting state with one event, the stage is untouched meanwhile, and sharing resumes when it arrives",async()=>{
 const root=repository("objects"),host=fake(root),base=blobOf(root,"base\n");
 host.receive(INDEX_BASE_V1,{hlc:stamp(1,"g1-server"),operation:"link",target:`tree ${treeFrom(root,{"base.txt":base})}`});
 await applyIndexV1(host);
 const late=cli(root,["hash-object","--stdin"],"late\n").trim();
 host.receive(`${INDEX_PREFIX_V1}late.txt`,{hlc:stamp(2,"g1-server"),operation:"link",target:`100644 ${late}`});
 for(let round=0;round<3;round++){host.advance(40_000);await applyIndexV1(host);}
 expect(host.events.filter(event=>event.reason==="objects-missing")).toHaveLength(1);
 expect(await stageHealthV1(root)).toEqual({state:"waiting",reason:"objects-missing"});
 expect(files(root)).toEqual([`100644 ${base} base.txt`]);
 blobOf(root,"late\n");
 host.advance(40_000);
 await applyIndexV1(host);
 expect(await stageHealthV1(root)).toEqual({state:"shared",reason:null});
 expect(files(root)).toContain(`100644 ${late} late.txt`);
});

gitTest("a held index.lock backs off from one to thirty seconds and is reported once; a member removes a lock older than ten minutes by itself and completes, the archivist keeps it as a degraded state for its operator",async()=>{
 const root=repository("lock"),host=fake(root),base=blobOf(root,"base\n"),entry=blobOf(root,"entry\n");
 host.receive(INDEX_BASE_V1,{hlc:stamp(1,"g1-server"),operation:"link",target:`tree ${treeFrom(root,{"base.txt":base})}`});
 await applyIndexV1(host);
 host.receive(`${INDEX_PREFIX_V1}e.txt`,{hlc:stamp(2,"g1-server"),operation:"link",target:`100644 ${entry}`});
 const lock=join(root,".git","index.lock");
 writeFileSync(lock,"");
 await applyIndexV1(host);
 host.advance(500);await applyIndexV1(host);
 host.advance(700);await applyIndexV1(host);
 host.advance(1000);await applyIndexV1(host);
 expect(host.events.filter(event=>event.reason==="index-busy")).toHaveLength(1);
 expect(await stageHealthV1(root)).toEqual({state:"waiting",reason:"index-busy"});
 expect(host.queue.size).toBe(1);
 utimesSync(lock,(Date.now()-700_000)/1000,(Date.now()-700_000)/1000);
 host.advance(60_000);await applyIndexV1(host);
 expect(existsSync(lock)).toBe(false);
 expect(Number(host.events.find(event=>event.event==="repaired"&&event.reason==="index-lock-stale")!.seconds)).toBeGreaterThanOrEqual(600);
 host.advance(60_000);
 expect((await applyIndexV1(host)).queued).toBe(0);
 expect(await stageHealthV1(root)).toEqual({state:"shared",reason:null});
 expect(files(root)).toContain(`100644 ${entry} e.txt`);
});

gitTest("the paths an interrupted apply was writing are not published as local edits after a restart",async()=>{
 const root=repository("crash"),host=fake(root,{server:true}),one=blobOf(root,"1\n"),remote=blobOf(root,"remote\n"),mine=blobOf(root,"mine\n");
 staging(root,"a.txt",one);
 await publishIndexV1(host);
 host.receive(`${INDEX_PREFIX_V1}r.txt`,{hlc:stamp(900,"g1-server"),operation:"link",target:`100644 ${remote}`});
 staging(root,"r.txt",remote);staging(root,"m.txt",mine);
 mkdirSync(join(root,".cache","f"),{recursive:true});
 writeFileSync(join(root,".cache","f","git-index.json"),JSON.stringify({version:1,state:"shared",reason:null,since:0,applying:{base:false,paths:["r.txt"]}}));
 forgetGitV1(root);
 const before=[...host.manifest.keys()];
 await publishIndexV1(host);
 expect([...host.manifest.keys()].filter(path=>!before.includes(path))).toEqual([`${INDEX_PREFIX_V1}m.txt`]);
 expect(host.manifest.get(`${INDEX_PREFIX_V1}r.txt`)!.hlc).toBe(stamp(900,"g1-server"));
});

gitTest("a very large change is published as entries in batches and never as a new base, and the fold of the registers equals the stage",async()=>{
 const root=repository("large"),host=fake(root,{server:true}),blob=blobOf(root,"bulk\n");
 staging(root,"first.txt",blobOf(root,"first\n"));
 await publishIndexV1(host);
 cli(root,["update-index","--index-info"],Array.from({length:6000},(_,index)=>`100644 ${blob}\tbulk/f${index}.txt\n`).join(""));
 const result=await publishIndexV1(host);
 expect(result).toMatchObject({published:6000,base:false});
 expect([...host.manifest.keys()].filter(path=>path===INDEX_BASE_V1)).toHaveLength(1);
 expect(await verifyIndexV1(host)).toMatchObject({ok:true,skipped:false});
});

gitTest("a change the entry policy cannot represent (names no operating system holds, absolute links) is carried as the base tree instead: the archivist loses nothing, a member ends with the exact tree",async()=>{
 const root=repository("policy"),host=fake(root,{server:true}),mate=repository("policy-mate"),other=fake(mate);
 staging(root,"first.txt",blobOf(root,"first\n"));
 await publishIndexV1(host);
 staging(root,"ok.txt",blobOf(root,"ok\n"));
 staging(root,"a:b.txt",blobOf(root,"colon\n"));
 staging(root,"CON.txt",blobOf(root,"device\n"));
 staging(root,"x.",blobOf(root,"dot\n"));
 staging(root,"safe-link",blobOf(root,"ok.txt"),"120000");
 staging(root,"abs-link",blobOf(root,"/etc/passwd"),"120000");
 const result=await publishIndexV1(host);
 expect(result).toMatchObject({published:1,base:true});
 expect([...host.manifest.keys()].filter(path=>path.startsWith(INDEX_PREFIX_V1))).toEqual([]);
 expect(host.events.filter(event=>event.event==="skipped").map(event=>event.reason).sort()).toEqual(["unrepresentable","unrepresentable","unrepresentable","unsafe-link"]);
 expect(host.events.some(event=>event.event==="published"&&event.reason==="unrepresentable")).toBe(true);
 expect(host.manifest.get(INDEX_BASE_V1)!.target).toBe(`tree ${cli(root,["write-tree"]).trim()}`);
 const tree=treeFrom(mate,{"first.txt":blobOf(mate,"first\n"),"a:b.txt":blobOf(mate,"colon\n")});
 other.receive(INDEX_BASE_V1,{hlc:stamp(1,"g1-server"),operation:"link",target:`tree ${tree}`});
 other.receive(`${INDEX_PREFIX_V1}fine.txt`,{hlc:stamp(3,"g1-server"),operation:"link",target:`100644 ${blobOf(mate,"fine\n")}`});
 other.receive(`${INDEX_PREFIX_V1}mine.txt`,{hlc:stamp(4,"g1-m02"),operation:"link",target:`100644 ${blobOf(mate,"mine\n")}`});
 await applyIndexV1(other);
 expect(files(mate).map(line=>line.split(" ").slice(2).join(" "))).toEqual(["a:b.txt","fine.txt","first.txt"]);
 expect(other.queue.size).toBe(0);
});

gitTest("tracked shell scripts with a carriage return are listed, scripts with LF and other files are not",async()=>{
 const root=repository("crlf");
 for(const [name,text] of [["a.sh","echo a\r\n"],["dir/b.sh","echo b\n"],["c.txt","c\r\n"],["dir/d.sh","echo d\r\n"]] as const){putFile(root,name,text);cli(root,["add","--",name]);}
 putFile(root,"e.sh","echo e\r\n");
 expect(await crlfScriptsV1(root)).toEqual(["a.sh","dir/d.sh"]);
 expect(await crlfScriptsV1(temp("no-repository"))).toBeNull();
});

gitTest("a rebase request publishes a base and lists the registers of other machines it voids, with the old stage kept on the machine that loses entries",async()=>{
 const root=repository("rebase"),host=fake(root,{server:true}),other=blobOf(root,"other\n");
 staging(root,"a.txt",blobOf(root,"a\n"));
 await publishIndexV1(host);
 host.manifest.set(`${INDEX_PREFIX_V1}y.txt`,{hlc:stamp(900,"g1-server"),operation:"link",target:`100644 ${other}`});
 host.manifest.set(`${INDEX_PREFIX_V1}gone.txt`,{hlc:stamp(901,"g1-server"),operation:"delete"});
 await requestRebaseV1(root);
 forgetGitV1(root);
 await publishIndexV1(host);
 const published=host.events.filter(event=>event.event==="published"&&event.reason==="rebase");
 expect(published).toHaveLength(1);
 expect(published[0]).toMatchObject({count:1,path:INDEX_BASE_V1});
 const lines=kind(join(root,".cache","f",String(published[0]!.lost))).split("\n").filter(Boolean);
 expect(lines).toEqual([`y.txt\tg1-server\t100644 ${other}`]);
});

const operations=[["index.version","4"],["core.untrackedCache","true"],["core.splitIndex","true"]] as const;
gitTest.each([...operations.map(([key,value])=>[`${key}=${value}`,[[key,value]]] as const),["all three",operations.map(pair=>[...pair])] as const])("write-tree, update-index and read-tree stay correct with %s",async(_name,settings)=>{
 const root=repository("extensions"),host=fake(root,{server:true});
 for(const [key,value] of settings)cli(root,["config",key,value]);
 staging(root,"a.txt",blobOf(root,"a\n"));staging(root,"d/b.txt",blobOf(root,"b\n"));
 writeFileSync(join(root,"a.txt"),"a\n");
 cli(root,["status","--porcelain"]);
 await publishIndexV1(host);
 const entry=blobOf(root,"entry\n");
 host.receive(`${INDEX_PREFIX_V1}e.txt`,{hlc:stamp(900,"g1-server"),operation:"link",target:`100644 ${entry}`});
 await applyIndexV1(host);
 staging(root,"local.txt",blobOf(root,"local\n"));
 await publishIndexV1(host);
 expect(files(root)).toEqual(expect.arrayContaining([`100644 ${entry} e.txt`]));
 expect(await verifyIndexV1(host)).toMatchObject({ok:true});
 expect(await writeTreeV1(root)).toBe(cli(root,["write-tree"]).trim());
 expect(existsSync(join(root,".cache","f","tmp"))?readdirSync(join(root,".cache","f","tmp")):[]).toEqual([]);
 expect(cli(root,["fsck","--no-dangling"],undefined)).toBeDefined();
});

gitTest("a sparse index keeps working through publish and apply",async()=>{
 const root=repository("sparse");
 for(const [path,text] of [["a.txt","a\n"],["d/b.txt","b\n"],["e/c.txt","c\n"]] as const){mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),text);}
 cli(root,["add","-A"]);cli(root,["commit","-q","-m","initial"]);
 const init=Bun.spawnSync(["git","sparse-checkout","init","--cone","--sparse-index"],{cwd:root,env:environment,stdout:"pipe",stderr:"pipe"});
 if(init.exitCode!==0)return;
 const host=fake(root,{server:true});
 await publishIndexV1(host);
 const entry=blobOf(root,"entry\n");
 host.receive(`${INDEX_PREFIX_V1}e2.txt`,{hlc:stamp(900,"g1-server"),operation:"link",target:`100644 ${entry}`});
 await applyIndexV1(host);
 expect(files(root)).toEqual(expect.arrayContaining([`100644 ${entry} e2.txt`]));
 expect(await verifyIndexV1(host)).toMatchObject({ok:true});
});

const mergeOracle=join(import.meta.dirname,"🐍️merge.py");
type Case=Readonly<{name:string;base:Record<string,string>;ours:Record<string,string>;theirs:Record<string,string>;wins:boolean}>;
const matrix:readonly Case[]=[
 {name:"both sides touch different paths",base:{"a":"0","b":"0"},ours:{"a":"1","b":"0"},theirs:{"a":"0","b":"2","c":"2"},wins:false},
 {name:"one path changed differently, others unrelated, ours older",base:{"f1":"0","f2":"0"},ours:{"f1":"A","f2":"A"},theirs:{"f1":"0","f2":"B","f3":"B"},wins:false},
 {name:"one path changed differently, others unrelated, theirs later",base:{"f1":"0","f2":"0"},ours:{"f1":"A","f2":"A"},theirs:{"f1":"0","f2":"B","f3":"B"},wins:true},
 {name:"the same change on both sides",base:{"f":"0"},ours:{"f":"S","g":"1"},theirs:{"f":"S","h":"2"},wins:false},
 {name:"theirs deletes what ours changed, ours older",base:{"f":"0","g":"0"},ours:{"f":"A","g":"0"},theirs:{"g":"0"},wins:false},
 {name:"theirs deletes what ours changed, theirs later",base:{"f":"0","g":"0"},ours:{"f":"A","g":"0"},theirs:{"g":"0"},wins:true},
 {name:"ours deletes what theirs left alone",base:{"f":"0","g":"0"},ours:{"g":"0"},theirs:{"f":"0","g":"2"},wins:false},
 {name:"a root commit on both sides",base:{},ours:{"a":"1"},theirs:{"b":"2","a":"2"},wins:true},
];
gitTest.each(matrix.map(item=>[item.name,item] as const))("the per-path merge of %s equals the Python oracle driving the git command line",async(_name,item)=>{
 const root=repository("merge"),blobs=new Map<string,string>(),blob=(text:string)=>{if(!blobs.has(text))blobs.set(text,blobOf(root,`${text}\n`));return blobs.get(text)!;};
 const build=(entries:Record<string,string>)=>treeFrom(root,Object.fromEntries(Object.entries(entries).map(([path,text])=>[path,blob(text)])));
 const base=Object.keys(item.base).length===0?null:build(item.base),ours=build(item.ours),theirs=build(item.theirs);
 const merged=(await mergeTreesV1(root,base,ours,theirs,item.wins))!;
 expect(merged).not.toBeNull();
 const ran=Bun.spawnSync([python!,"-I",mergeOracle,root,base??"empty",ours,theirs,item.wins?"1":"0"],{stdout:"pipe",stderr:"pipe"});
 expect(ran.exitCode,ran.stderr.toString()).toBe(0);
 const expected=JSON.parse(ran.stdout.toString()) as {entries:string[];conflicts:string[]};
 const listed=cli(root,["ls-tree","-r","-z",merged.tree]).split("\0").filter(Boolean).map(line=>{const [head,path]=line.split("\t");const [mode,,oid]=head!.split(" ");return `${mode} ${oid} ${path}`;});
 expect(listed).toEqual(expected.entries);
 expect([...merged.conflicts].sort()).toEqual(expected.conflicts);
});

gitTest("a file against a directory of the same name is a conflict that keeps one side whole and never a whole-tree fallback",async()=>{
 const root=repository("dirfile"),one=blobOf(root,"1\n"),two=blobOf(root,"2\n"),three=blobOf(root,"3\n"),other=blobOf(root,"o\n");
 const base=treeFrom(root,{a:one,keep:one}),ours=treeFrom(root,{a:two,keep:one,x:other}),theirs=treeFrom(root,{"a/b":three,keep:one,y:other});
 const names=(tree:string)=>cli(root,["ls-tree","-r","--name-only",tree]).split("\n").filter(Boolean);
 expect(names((await mergeTreesV1(root,base,ours,theirs,false))!.tree)).toEqual(["a","keep","x","y"]);
 const later=(await mergeTreesV1(root,base,ours,theirs,true))!;
 expect(names(later.tree)).toEqual(["a/b","keep","x","y"]);
 expect(later.conflicts).toEqual(["a","a/b"].filter(path=>later.conflicts.includes(path)));
 expect(later.conflicts.length).toBeGreaterThan(0);
});

function commitTree(root:string,tree:string,parent:string|null,message:string,when:number):string{
 const date=`${when} +0000`;
 return cli(root,["commit-tree",tree,...(parent===null?[]:["-p",parent]),"-m",message],undefined,{GIT_AUTHOR_DATE:date,GIT_COMMITTER_DATE:date}).trim();
}

gitTest("linearising concurrent commits that touch one path differently keeps the winner's unrelated changes and the loser's unrelated changes, whichever side is later",async()=>{
 for(const [label,loserWhen,expectedF2] of [["loser earlier",1_700_000_100,"A"],["loser later",1_700_000_300,"B"]] as const){
  const root=repository(`linear-${label.replace(" ","-")}`),text=new Map<string,string>(),blob=(value:string)=>{if(!text.has(value))text.set(value,blobOf(root,`${value}\n`));return text.get(value)!;};
  const build=(entries:Record<string,string>)=>treeFrom(root,Object.fromEntries(Object.entries(entries).map(([path,value])=>[path,blob(value)])));
  const parent=commitTree(root,build({f1:"0",f2:"0"}),null,"parent",1_700_000_000);
  const winner=commitTree(root,build({f1:"A",f2:"A"}),parent,"winner",1_700_000_200),loser=commitTree(root,build({f1:"0",f2:"B",f3:"B"}),parent,"loser",loserWhen);
  cli(root,["update-ref","refs/heads/main",loser]);
  const result=await linearizeRefV1(root,"refs/heads/main",loser,winner);
  expect(result.state).toBe("rewritten");
  const tip=cli(root,["rev-parse","refs/heads/main"]).trim(),read=(path:string)=>cli(root,["show",`${tip}:${path}`]).trim();
  expect(read("f1"),label).toBe("A");
  expect(read("f3"),label).toBe("B");
  expect(read("f2"),label).toBe(expectedF2);
  expect(result.state==="rewritten"?result.conflicts:[]).toEqual(["f2"]);
  expect(cli(root,["rev-parse",`${tip}~1`]).trim()).toBe(winner);
 }
});

gitTest("a local commit that the winning history already holds as an earlier rewrite is dropped even when the later committer time makes its change win the merge",async()=>{
 const root=repository("rewrite-twice"),text=new Map<string,string>(),blob=(value:string)=>{if(!text.has(value))text.set(value,blobOf(root,`${value}\n`));return text.get(value)!;};
 const build=(entries:Record<string,string>)=>treeFrom(root,Object.fromEntries(Object.entries(entries).map(([path,value])=>[path,blob(value)])));
 const parent=commitTree(root,build({f:"0"}),null,"parent",1_700_000_000);
 const first=commitTree(root,build({f:"a"}),parent,"first",1_700_000_100),original=commitTree(root,build({f:"c"}),parent,"c5",1_700_000_500);
 const rewrite=commitTree(root,build({f:"c"}),first,"c5",1_700_000_500),incoming=commitTree(root,build({f:"d"}),rewrite,"d",1_700_000_200);
 const local=commitTree(root,build({f:"c",g:"g"}),original,"c7",1_700_000_600);
 cli(root,["update-ref","refs/heads/main",local]);
 const result=await linearizeRefV1(root,"refs/heads/main",local,incoming);
 expect(result.state).toBe("rewritten");
 const messages=cli(root,["log","--format=%s","refs/heads/main"]).trim().split("\n");
 expect(messages.filter(item=>item==="c5")).toHaveLength(1);
 expect(messages.filter(item=>item==="c7")).toHaveLength(1);
 expect(messages).toEqual(["c7","d","c5","first","parent"]);
});

gitTest("a ref whose objects have not arrived reports a growing wait and the wait ends when they arrive",async()=>{
 const root=repository("wait"),first=blobOf(root,"1\n"),parent=commitTree(root,treeFrom(root,{f:first}),null,"parent",1_700_000_000);
 cli(root,["update-ref","refs/heads/main",parent]);
 const ghost="e".repeat(40);
 const one=await linearizeRefV1(root,"refs/heads/main",parent,ghost);
 await Bun.sleep(30);
 const two=await linearizeRefV1(root,"refs/heads/main",parent,ghost);
 expect(one.state).toBe("retry");expect(two.state).toBe("retry");
 expect(two.state==="retry"?two.waitedMs:0).toBeGreaterThanOrEqual(25);
 const child=commitTree(root,treeFrom(root,{f:first,g:first}),parent,"child",1_700_000_100);
 expect((await linearizeRefV1(root,"refs/heads/main",parent,child)).state).toBe("linear");
});

gitTest("the stage record the replica writes is valid against the schema in every state, and the document of a garbage record reads as shared",async()=>{
 const validate=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/StageRecord"});
 const root=repository("record"),host=fake(root),base=blobOf(root,"base\n");
 host.receive(INDEX_BASE_V1,{hlc:stamp(1,"g1-server"),operation:"link",target:`tree ${treeFrom(root,{"base.txt":base})}`});
 await applyIndexV1(host);
 const read=()=>JSON.parse(readFileSync(join(root,".cache","f","git-index.json"),"utf8")) as unknown;
 writeFileSync(join(root,".git","index.lock"),"");
 host.receive(`${INDEX_PREFIX_V1}e.txt`,{hlc:stamp(2,"g1-server"),operation:"link",target:`100644 ${blobOf(root,"e\n")}`});
 await applyIndexV1(host);
 expect(validate(read()),JSON.stringify(validate.errors)).toBe(true);
 expect(read()).toMatchObject({state:"waiting",reason:"index-busy",applying:{base:false,paths:["e.txt"]}});
 rmSync(join(root,".git","index.lock"));host.advance(60_000);await applyIndexV1(host);
 expect(validate(read()),JSON.stringify(validate.errors)).toBe(true);
 expect(read()).toMatchObject({state:"shared",reason:null,applying:null});
 writeFileSync(join(root,".cache","f","git-index.json"),"{garbage");
 expect(await stageHealthV1(root)).toEqual({state:"shared",reason:null});
});

const putFile=(root:string,path:string,text:string="x\n"):void=>{const file=join(root,...path.split("/"));mkdirSync(join(file,".."),{recursive:true});writeFileSync(file,text);};
const bytewise=(lines:string[]):string[]=>[...lines].sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right)));

gitTest("the view ignores every machine-local input: global and system configuration, excludes and XDG ignore files, inherited GIT_CONFIG variables and the local ignorecase",async()=>{
 const root=repository("pinned"),home=temp("home");
 putFile(root,".gitignore","Backup/\n");putFile(root,"a.txt");cli(root,["add","-A"]);cli(root,["commit","-q","-m","initial"]);
 putFile(root,"b.txt");putFile(root,"x.tmp");putFile(root,"backup/x.txt");putFile(root,"Backup/y.txt");
 cli(root,["config","core.ignorecase","true"]);cli(root,["config","core.excludesFile",join(home,"local-excludes")]);
 const clean=(await gitViewV1(root))!;
 expect(clean.lines).toContain("?? b.txt");expect(clean.lines).toContain("?? x.tmp");expect(clean.lines).toContain("?? backup/x.txt");
 expect(clean.lines.some(line=>line.includes("Backup/y.txt"))).toBe(false);
 writeFileSync(join(home,".gitconfig"),`[core]\n\texcludesFile = ${join(home,"excludes").replaceAll("\\","/")}\n\tquotepath = true\n\tautocrlf = true\n[status]\n\tshowUntrackedFiles = no\n`);
 writeFileSync(join(home,"excludes"),"b.txt\n");writeFileSync(join(home,"local-excludes"),"x.tmp\n");
 mkdirSync(join(home,".config","git"),{recursive:true});writeFileSync(join(home,".config","git","ignore"),"*.tmp\n");
 writeFileSync(join(root,".git","info","exclude"),"");
 const saved={...process.env};
 Object.assign(process.env,{HOME:home,USERPROFILE:home,XDG_CONFIG_HOME:join(home,".config"),GIT_CONFIG_COUNT:"1",GIT_CONFIG_KEY_0:"core.excludesFile",GIT_CONFIG_VALUE_0:join(home,"excludes"),GIT_CONFIG_PARAMETERS:"'core.ignorecase=true'"});
 delete process.env.GIT_CONFIG_GLOBAL;delete process.env.GIT_CONFIG_NOSYSTEM;
 try{
  forgetGitV1(root);
  const hostile=(await gitViewV1(root))!;
  expect(hostile.lines).toEqual(clean.lines);
  expect(hostile.digest).toBe(clean.digest);
 }finally{for(const key of Object.keys(process.env))if(!(key in saved))delete process.env[key];Object.assign(process.env,saved);}
});

gitTest("a case-only rename, a renamed directory and an NFD spelling give the lines that Linux git prints, on every operating system",async()=>{
 const root=repository("spelling"),nfc="caf\u00e9.txt",nfd="cafe\u0301.txt";
 putFile(root,"Foo.txt");putFile(root,"Dir/in.txt");putFile(root,nfc);putFile(root,"keep.txt");symlinkSync("keep.txt",join(root,"link"));putFile(root,"run.sh","#!/bin/sh\n");chmodSync(join(root,"run.sh"),0o755);
 cli(root,["add","-A"]);cli(root,["commit","-q","-m","initial"]);
 forgetGitV1(root);
 expect((await gitViewV1(root))!.lines).toEqual([]);
 renameSync(join(root,"Foo.txt"),join(root,"tmp-foo"));renameSync(join(root,"tmp-foo"),join(root,"foo.txt"));
 renameSync(join(root,"Dir"),join(root,"tmp-dir"));renameSync(join(root,"tmp-dir"),join(root,"dir"));
 renameSync(join(root,nfc),join(root,"tmp-cafe"));renameSync(join(root,"tmp-cafe"),join(root,nfd));
 chmodSync(join(root,"keep.txt"),0o755);chmodSync(join(root,"run.sh"),0o644);
 const view=(await gitViewV1(root))!;
 expect(view.lines).toEqual([" D Dir/in.txt"," D Foo.txt",` D ${nfc}`,`?? ${nfc}`,"?? dir/in.txt","?? foo.txt"]);
 expect(view).toMatchObject({deleted:3,untracked:3,modified:0,staged:0});
});

gitTest("the spelling check names absent paths, their case and normalisation variants and the NFD-only ones",async()=>{
 const root=temp("spelling-check");
 putFile(root,"present.txt");putFile(root,"foo.txt");putFile(root,"dir/x.txt");putFile(root,"cafe\u0301.txt");
 const spelling=await spellingV1(root,["present.txt","Foo.txt","gone.txt","Dir/x.txt","dir/X.txt","caf\u00e9.txt"]);
 const insensitive=(await volumeV1(root)).caseInsensitive;
 expect(spelling.absent).toEqual(["Foo.txt","gone.txt","Dir/x.txt","dir/X.txt","caf\u00e9.txt"]);
 expect(spelling.variants).toEqual(["Foo.txt","Dir/x.txt","dir/X.txt","caf\u00e9.txt"]);
 expect(spelling.composed).toEqual(["caf\u00e9.txt"]);
 expect(insensitive===(process.platform==="darwin"||process.platform==="win32")).toBe(true);
});

gitTest("the guards count what makes a machine see something else: info/exclude rules, case-dependent ignores, a ignored tracked file, refused tracked names and links, paths over the budget and the executable bit",async()=>{
 const root=repository("guards");
 putFile(root,".gitignore","Backup/\n*.log\n");putFile(root,"a.txt");putFile(root,"run.sh","#!/bin/sh\n");putFile(root,"kept.log");putFile(root,"CON.txt");putFile(root,`${"d".repeat(120)}/${"e".repeat(120)}/f.txt`);
 symlinkSync("/etc",join(root,"abs"));
 cli(root,["add","-f","-A"]);cli(root,["commit","-q","-m","initial"]);
 chmodSync(join(root,"run.sh"),0o755);putFile(root,"backup/x.txt");putFile(root,"Backup/hidden.txt");
 writeFileSync(join(root,".git","info","exclude"),"# comment\n\nscratch/\n");
 forgetGitV1(root);
 const guards=(await gitGuardsV1(root,{platform:"win32",longPaths:false,modeless:false}))!;
 expect(guards.pins).toBe(gitPinsHashV1());expect(guards.pins).toMatch(/^[0-9a-f]{12}$/u);
 expect(guards.version).toMatch(/^\d+\.\d+/u);
 expect(guards).toMatchObject({infoExclude:1,trackedIgnored:1,trackedRefused:2,overBudget:1,modeMismatch:1,caseMismatch:0});
 expect(guards.caseIgnore).toBe(1);
 expect(guards.samples.some(line=>line==="refused CON.txt")&&guards.samples.some(line=>line==="unsafe-link abs")&&guards.samples.some(line=>line==="mode run.sh")).toBe(true);
 expect((await gitGuardsV1(root,{platform:"win32",longPaths:true}))!.overBudget).toBe(0);
 expect((await gitGuardsV1(root,{platform:"win32",longPaths:true,modeless:true}))!.modeMismatch).toBeNull();
 expect(await gitGuardsV1(temp("no-repository"))).toBeNull();
});

test("the pins are one list for every operating system and the hash changes with any pin",()=>{
 expect(GIT_PINS_V1.map(([key])=>key)).toEqual(expect.arrayContaining(["core.excludesFile","core.attributesFile","core.quotepath","core.precomposeunicode","core.autocrlf","core.safecrlf","core.eol","core.longpaths"]));
 expect(GIT_VIEW_PINS_V1).toEqual([["core.ignorecase","false"],["core.filemode","false"],["core.symlinks","true"]]);
 expect(gitPinsHashV1()).toBe(gitPinsHashV1());
});

test("dead temporary files of git writers in the pack directory are swept after an hour of silence and live or foreign files stay",async()=>{
 const root=mkdtempSync(join(tmpdir(),"git-sweep-"));
 scratch.push(root);
 const directory=join(root,".git","objects","pack");
 mkdirSync(directory,{recursive:true});
 for(const name of ["tmp_pack_7JgB2y",".tmp-123-pack-abc.pack","pack-1.pack","pack-1.idx","tmp_idx_Zz"])writeFileSync(join(directory,name),Buffer.alloc(1000));
 const first=await sweepGitGarbageV1(root,{now:Date.now()+1_800_000});
 expect(first).toEqual({files:0,bytes:0});
 const second=await sweepGitGarbageV1(root,{now:Date.now()+2*3_600_000});
 expect(second).toEqual({files:3,bytes:3000});
 expect(readdirSync(directory).sort()).toEqual(["pack-1.idx","pack-1.pack"]);
 expect(await sweepGitGarbageV1(join(root,"missing"))).toEqual({files:0,bytes:0});
});

test("the view leaves out every path the replica never carries, untracked, modified or deleted (a member that adopted the stage of the archivist reads a tracked path it cannot hold as deleted), so machines agree",()=>{
 const far="deep/"+Array.from({length:8},(_,index)=>String(index).repeat(36)).join("/")+"/leaf.txt";
 const shared=["?? ok/a.txt"," M tracked.txt","AM staged.txt"," D gone.txt"],local=["?? worker/round-1.txt.sample-2026-11-15T01:45:25Z.txt",`?? ${far}`,"?? trailing./x.txt","?? dev/aux.txt","?? bad\\name.txt"];
 const without=assembleViewV1(shared,[]),holding=assembleViewV1([...shared,...local],[]);
 expect(far.length).toBeGreaterThan(256);
 expect(holding.lines).toEqual(without.lines);
 expect(holding.digest).toBe(without.digest);
 expect(holding).toMatchObject({modified:2,deleted:1,untracked:1});
 const tracked=assembleViewV1([...shared," M worker/tracked:colon.txt",` D ${far}`],["M worker/tracked:colon.txt"]);
 expect(tracked.lines).toEqual(without.lines);
 expect(tracked.digest).toBe(without.digest);
 expect(tracked.staged).toBe(1);
});

gitTest("a view read from disk agrees between a machine holding refused files and links and one without them, and between disk modes",async()=>{
 const make=(label:string,extra:boolean,mode:number):string=>{
  const root=repository(label);
  mkdirSync(join(root,"src"),{recursive:true});writeFileSync(join(root,"src/run.sh"),"echo\n");writeFileSync(join(root,"a.txt"),"a\n");
  cli(root,["add","-A"]);cli(root,["commit","-q","-m","base"]);
  chmodSync(join(root,"src/run.sh"),mode);
  writeFileSync(join(root,"shared.txt"),"s\n");symlinkSync("a.txt",join(root,"inner-link"));
  if(extra){
   mkdirSync(join(root,"workers"),{recursive:true});writeFileSync(join(root,"workers/x.txt.sample-2026-11-15T01:45:25Z.txt"),"x\n");
   symlinkSync("/etc/hosts",join(root,"absolute-link"));symlinkSync("../../outside",join(root,"climbing-link"));
   const long=join(root,...Array.from({length:9},(_,index)=>`${String(index).repeat(30)}`));mkdirSync(long,{recursive:true});writeFileSync(join(long,"leaf.txt"),"l\n");
  }
  return root;
 };
 const plain=make("carried-plain",false,0o644),holding=make("carried-holding",true,0o755),one=(await gitViewV1(plain))!,two=(await gitViewV1(holding))!;
 expect(two.lines).toEqual(one.lines);
 expect(two.digest).toBe(one.digest);
 expect(one.lines).toContain("?? inner-link");
 expect(one.lines).toContain("?? shared.txt");
 const step=async(label:string,root:string):Promise<void>=>{
  writeFileSync(join(root,"workers-two.txt"),"w\n");symlinkSync("/etc/hosts",join(root,"another-absolute"));
  const next=await gitViewDeltaV1(root,(await gitViewV1(root))!,{paths:["workers-two.txt","another-absolute"],restage:null}),full=(await gitViewV1(root))!;
  expect(next,label).not.toBeNull();expect(next!.lines,label).toEqual(full.lines);expect(next!.digest,label).toBe(full.digest);
  expect(full.lines,label).toContain("?? workers-two.txt");expect(full.lines.some(line=>line.includes("another-absolute")),label).toBe(false);
 };
 await step("plain",plain);await step("holding",holding);
});


const debugOf=(root:string,path:string):string=>cli(root,["ls-files","--debug","--",path]);
const treeFiles=(root:string,tree:string):string[]=>cli(root,["ls-tree","-r",tree]).split("\n").filter(Boolean).map(line=>{const [head,path]=line.split("\t");const [mode,,oid]=head!.split(" ");return `${mode} ${oid} ${path}`;});

gitTest("the stage of the archivist wins on a member: a stage a person made is replaced once, kept under lost, the stat data of unchanged entries survives, entries of other machines are void and nothing is published",async()=>{
 const root=repository("archivist-wins"),member=fake(root);
 writeFileSync(join(root,"a.txt"),"a\n");writeFileSync(join(root,"b.txt"),"b\n");
 cli(root,["add","a.txt","b.txt"]);
 const a=blobOf(root,"a\n"),c=blobOf(root,"c\n"),d=blobOf(root,"d\n"),stat=debugOf(root,"a.txt");
 const tree=treeFrom(root,{"a.txt":a,"c.txt":c});
 member.receive(INDEX_BASE_V1,{hlc:stamp(1,"g1-server"),operation:"link",target:`tree ${tree}`});
 member.receive(`${INDEX_PREFIX_V1}ghost.txt`,{hlc:stamp(2,"g1-m02"),operation:"link",target:`100644 ${d}`});
 await applyIndexV1(member);
 expect(files(root)).toEqual([`100644 ${a} a.txt`,`100644 ${c} c.txt`]);
 expect(debugOf(root,"a.txt")).toBe(stat);
 expect(member.queue.size).toBe(0);
 expect(member.events.filter(event=>event.event==="overwritten")).toHaveLength(1);
 const names=lostEntries(root).filter(name=>name.startsWith("index-"));
 expect(names).toHaveLength(1);
 expect(files(root,{GIT_INDEX_FILE:join(root,".cache","f","lost",names[0]!)})).toEqual([`100644 ${a} a.txt`,`100644 ${blobOf(root,"b\n")} b.txt`]);
 const count=member.manifest.size,before=member.events.length;
 staging(root,"mine.txt",blobOf(root,"mine\n"));
 cli(root,["rm","-q","--cached","a.txt"]);
 const result=await publishIndexV1(member);
 expect(result).toMatchObject({published:0,base:false});
 expect(member.manifest.size).toBe(count);
 expect(files(root)).toEqual([`100644 ${a} a.txt`,`100644 ${c} c.txt`]);
 expect(member.events.slice(before).filter(event=>event.event==="overwritten")).toHaveLength(1);
 expect(lostEntries(root).filter(name=>name.startsWith("index-"))).toHaveLength(2);
 expect(await writeTreeV1(root)).toBe(tree);
 expect(await publishIndexV1(member)).toMatchObject({published:0});
 expect(lostEntries(root).filter(name=>name.startsWith("index-"))).toHaveLength(2);
 expect(existsSync(join(root,".git","index.lock"))).toBe(false);
 expect(existsSync(join(root,".cache","f","tmp"))?readdirSync(join(root,".cache","f","tmp")):[]).toEqual([]);
});

gitTest("an index that is absent, corrupt, unmerged or far off is rebuilt from the tree of the archivist, with the old file kept and no worktree file touched",async()=>{
 const root=repository("far-off"),member=fake(root),a=blobOf(root,"a\n"),b=blobOf(root,"b\n");
 writeFileSync(join(root,"a.txt"),"a\n");
 const tree=treeFrom(root,{"a.txt":a,"deep/b.txt":b});
 member.receive(INDEX_BASE_V1,{hlc:stamp(1,"g1-server"),operation:"link",target:`tree ${tree}`});
 await applyIndexV1(member);
 const expected=treeFiles(root,tree),worktree=readdirSync(root).sort();
 const settle=async():Promise<void>=>{forgetGitV1(root);member.advance(60_000);await publishIndexV1(member);};
 writeFileSync(join(root,".git","index"),"this is not an index\n");
 await settle();
 expect(files(root)).toEqual(expected);
 expect(lostEntries(root).filter(name=>name.startsWith("index-"))).toHaveLength(1);
 rmSync(join(root,".git","index"));
 await settle();
 expect(files(root)).toEqual(expected);
 cli(root,["update-index","--index-info"],`100644 ${a} 1\ta.txt\n100644 ${b} 2\ta.txt\n100644 ${a} 3\ta.txt\n`);
 expect(await unmergedV1(root)).toBe(3);
 await settle();
 expect(await unmergedV1(root)).toBe(0);
 expect(files(root)).toEqual(expected);
 cli(root,["update-index","--index-info"],Array.from({length:300},(_,index)=>`100644 ${b} 0\tstale/f${index}.txt\n`).join(""));
 await settle();
 expect(files(root)).toEqual(expected);
 expect(await gitStateV1(root)).toMatchObject({indexTree:tree});
 expect(readdirSync(root).sort()).toEqual(worktree);
 expect(readFileSync(join(root,"a.txt"),"utf8")).toBe("a\n");
 expect(lostEntries(root).filter(name=>name.startsWith("index-")).length).toBeLessThanOrEqual(3);
 expect(member.events.filter(event=>event.event==="error")).toEqual([]);
 expect((await stageHealthV1(root)).state).toBe("shared");
});

gitTest("a tree whose objects have not arrived leaves the stage untouched with one waiting event and is adopted when they arrive; a held index.lock makes the member wait and the lock is never overwritten",async()=>{
 const root=repository("adopt-wait"),mate=repository("adopt-mate"),member=fake(root),have=blobOf(root,"have\n"),late=blobOf(mate,"late\n");
 staging(root,"mine.txt",blobOf(root,"mine\n"));
 blobOf(mate,"have\n");
 const tree=treeFrom(mate,{"have.txt":have,"late.txt":late});
 for(const oid of [tree]){const dir=join(root,".git","objects",oid.slice(0,2));mkdirSync(dir,{recursive:true});writeFileSync(join(dir,oid.slice(2)),readFileSync(join(mate,".git","objects",oid.slice(0,2),oid.slice(2))));}
 member.receive(INDEX_BASE_V1,{hlc:stamp(1,"g1-server"),operation:"link",target:`tree ${tree}`});
 const before=files(root);
 for(let round=0;round<3;round++){member.advance(40_000);await applyIndexV1(member);}
 expect(member.events.filter(event=>event.reason==="objects-missing")).toHaveLength(1);
 expect(await stageHealthV1(root)).toEqual({state:"waiting",reason:"objects-missing"});
 expect(files(root)).toEqual(before);
 expect(existsSync(join(root,".git","index.lock"))).toBe(false);
 const dir=join(root,".git","objects",late.slice(0,2));mkdirSync(dir,{recursive:true});
 writeFileSync(join(dir,late.slice(2)),readFileSync(join(mate,".git","objects",late.slice(0,2),late.slice(2))));
 writeFileSync(join(root,".git","index.lock"),"held by someone else");
 member.advance(40_000);await applyIndexV1(member);
 expect(await stageHealthV1(root)).toEqual({state:"waiting",reason:"index-busy"});
 expect(readFileSync(join(root,".git","index.lock"),"utf8")).toBe("held by someone else");
 expect(files(root)).toEqual(before);
 rmSync(join(root,".git","index.lock"));
 member.advance(40_000);await applyIndexV1(member);
 expect(await stageHealthV1(root)).toEqual({state:"shared",reason:null});
 expect(files(root)).toEqual(treeFiles(mate,tree));
 expect(member.queue.size).toBe(0);
});

gitTest("the archivist carries a tree with unrepresentable paths as the base, so a member reaches the exact tree of the archivist and the verify of the fold agrees",async()=>{
 const archivist=repository("long-archivist"),mate=repository("long-member"),server=fake(archivist,{server:true}),member=fake(mate);
 const long=`${"d".repeat(120)}/${"e".repeat(120)}/${"f".repeat(60)}/leaf.txt`;
 expect(long.length).toBeGreaterThan(256);
 staging(archivist,"first.txt",blobOf(archivist,"first\n"));
 await publishIndexV1(server);
 staging(archivist,long,blobOf(archivist,"long\n"));staging(archivist,"fine.txt",blobOf(archivist,"fine\n"));
 await publishIndexV1(server);
 expect(server.manifest.get(INDEX_BASE_V1)!.target).toBe(`tree ${cli(archivist,["write-tree"]).trim()}`);
 const tree=cli(archivist,["write-tree"]).trim();
 expect(Bun.spawnSync(["cp","-R",`${join(archivist,".git","objects")}/.`,join(mate,".git","objects")]).exitCode).toBe(0);
 for(const [path,entry] of server.manifest)member.receive(path,entry);
 await applyIndexV1(member);
 expect(await writeTreeV1(mate)).toBe(tree);
 expect(treeFiles(mate,tree)).toContain(`100644 ${blobOf(mate,"long\n")} ${long}`);
 expect(await verifyIndexV1(member)).toMatchObject({ok:true,skipped:false});
 expect(member.queue.size).toBe(0);
});
