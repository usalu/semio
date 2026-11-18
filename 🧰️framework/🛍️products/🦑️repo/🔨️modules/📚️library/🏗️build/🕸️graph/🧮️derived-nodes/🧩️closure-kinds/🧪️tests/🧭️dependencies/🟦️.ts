/** 🧭️ Laws of the fine-grained invalidation of the closure settler: a mini module world on a real temporary directory (relative imports with absent candidates probed, a directory glob, bare imports through `node_modules`, a symlinked package, a compiler-style `paths` map and a package manifest with aliases) is walked by a resolver that records rows like the real walkers (`file`, `lstat`, `realpath`, `fileExists`, `readdir`, relative and bare `resolve`). After every kind of edit the settler, given only the journal of the edit, must (1) derive exactly the closures one of whose recorded rows no longer holds (a missed change is a closure that should have been derived and was not; a needless walk is one that was), (2) report as changed exactly the closures whose answer differs from a full re-walk, (3) check no row of an unrelated edit beyond the rows nothing bounds, and (4) agree with a full re-walk over random edit sequences. The resolver is cross-checked against `createRequire` (an independent implementation) for installed and linked packages. */
import{expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{createRequire}from"node:module";
import{lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,readlinkSync,realpathSync,renameSync,rmSync,statSync,symlinkSync,unlinkSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join,resolve as resolvePath}from"node:path";
import{createClosureSettlerV1,stampOfV1}from"../../🟨️.mjs";
import{createNodeStoreV1}from"../../../💾️store/🟨️.mjs";

type Row={kind:string;path:string;fact:unknown};
const MISSING="!MODULE_NOT_FOUND",ABSENT="!ENOENT";
const sha=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex").slice(0,16);
const tryOr=<T>(work:()=>T,fallback:T):T=>{try{return work();}catch{return fallback;}};

function worldOf(){
 const root=realpathSync(mkdtempSync(join(tmpdir(),"deps-ws-"))),store=mkdtempSync(join(tmpdir(),"deps-store-")),at=(path:string)=>join(root,path);
 const write=(path:string,text:string)=>{mkdirSync(dirname(at(path)),{recursive:true});writeFileSync(at(path),text);};
 const typeOf=(path:string)=>tryOr(()=>{const status=lstatSync(path);return status.isSymbolicLink()?"link:"+readlinkSync(path):status.isDirectory()?"dir":"file";},ABSENT);
 const bytesOf=(path:string)=>tryOr(()=>sha(readFileSync(path)),ABSENT);
 const jsonOf=(path:string):any=>tryOr(()=>JSON.parse(readFileSync(path,"utf8")),undefined);
 const fileExists=(path:string)=>tryOr(()=>statSync(path).isFile(),false);
 const real=(path:string)=>tryOr(()=>realpathSync(path),ABSENT);
 const ancestors=(from:string)=>{const out:string[]=[];for(let at=dirname(from);at.startsWith(root);at=dirname(at)){out.push(at);if(at===root)break;}return out;};
 const nearest=(from:string,name:string)=>{for(const dir of ancestors(from)){const found=jsonOf(join(dir,name));if(found!==undefined)return{dir,json:found};}return undefined;};
 const bare=(from:string,request:string):string=>{
  const paths=nearest(from,"tsconfig.json");
  if(paths?.json.paths?.[request]!==undefined){const target=resolvePath(paths.dir,paths.json.paths[request]);if(fileExists(target))return real(target);}
  const manifest=nearest(from,"package.json");
  if(manifest?.json.aliases?.[request]!==undefined){const target=resolvePath(manifest.dir,manifest.json.aliases[request]);if(fileExists(target))return real(target);}
  for(const dir of ancestors(from)){
   const home=join(dir,"node_modules",request),main=jsonOf(join(home,"package.json"))?.main??"index.js",file=join(home,main);
   if(fileExists(file))return real(file);
   if(fileExists(home+".js"))return real(home+".js");
  }
  return MISSING;
 };
 const relative=(from:string,request:string):string=>{
  const base=resolvePath(dirname(from),request);
  for(const candidate of[base+".js",join(base,"index.js")])if(fileExists(candidate))return candidate;
  return MISSING;
 };
 const factOf=(row:Row):unknown=>{
  switch(row.kind){
   case"file":return bytesOf(row.path);
   case"lstat":return typeOf(row.path);
   case"realpath":return real(row.path);
   case"fileExists":return fileExists(row.path);
   case"readdir":return tryOr(()=>readdirSync(row.path).sort().join(","),ABSENT);
   case"resolve":{const[from,request]=JSON.parse(row.path);return request.startsWith(".")?relative(from,request):bare(from,request);}
  }
  throw Error("unknown kind "+row.kind);
 };
 const hold=async(rows:Row[])=>rows.every(row=>JSON.stringify(factOf(row))===JSON.stringify(row.fact));
 const walk=(entry:string)=>{
  const seen=new Set<string>(),parts:Row[][]=[],files:string[]=[];
  const row=(rows:Row[],kind:string,path:string)=>{rows.push({kind,path,fact:factOf({kind,path,fact:undefined})});};
  const visit=(file:string)=>{
   if(seen.has(file))return;seen.add(file);
   const rows:Row[]=[];row(rows,"lstat",file);row(rows,"file",file);parts.push(rows);
   const text=tryOr(()=>readFileSync(file,"utf8"),undefined);if(text===undefined)return;
   files.push(file+"@"+bytesOf(file));
   for(const line of text.split("\n")){
    const request=/^import "(.*)"$/.exec(line)?.[1];if(request===undefined)continue;
    if(request.endsWith("/*")){const dir=resolvePath(dirname(file),request.slice(0,-2));row(rows,"readdir",dir);for(const name of tryOr(()=>readdirSync(dir).sort(),[] as string[]))if(name.endsWith(".js"))visit(join(dir,name));continue;}
    if(request.startsWith(".")){
     const base=resolvePath(dirname(file),request);
     for(const candidate of[base+".js",join(base,"index.js")]){row(rows,"fileExists",candidate);if(fileExists(candidate))break;}
     row(rows,"resolve",JSON.stringify([file,request]));
     const found=relative(file,request);if(found!==MISSING)visit(found);
     continue;
    }
    row(rows,"resolve",JSON.stringify([file,request]));
    const found=bare(file,request);
    if(found!==MISSING){row(rows,"realpath",join(root,"node_modules",request));row(rows,"lstat",join(root,"node_modules",request));visit(found);}
   }
  };
  visit(entry);
  return{fact:files.sort(),parts};
 };
 const NAMES=Array.from({length:24},(_,at)=>"q"+String(at).padStart(2,"0")),entryOf=(name:string)=>at("app/"+name+"/main.js");
 const imp=(spec:string)=>"import \""+spec+"\"",MENU=["./local","./missing","../../lib/shared","../../lib/dir/*","pkg","linked","@alias","fancy","unknown-pkg","../../lib/deep/inner/x"].map(imp);
 const seed=()=>{
  write("tsconfig.json",JSON.stringify({paths:{"@alias":"lib/shared.js"}}));write("package.json",JSON.stringify({aliases:{fancy:"lib/fancy.js"}}));
  write("lib/shared.js","shared v0\n");write("lib/other.js","other\n");write("lib/fancy.js","fancy\n");write("lib/dir/a.js","a\n");write("lib/dir/b.js","b\n");write("lib/deep/inner/x.js","x\n");
  write("libs/a/index.js","libs a\n");write("libs/b/index.js","libs b\n");
  write("node_modules/pkg/package.json",JSON.stringify({main:"main.js"}));write("node_modules/pkg/main.js","pkg main\n");write("node_modules/pkg/alt.js","pkg alt\n");
  mkdirSync(at("node_modules"),{recursive:true});symlinkSync("../libs/a",at("node_modules/linked"));
  write("docs/readme.md","docs\n");write("other/package.json",JSON.stringify({name:"other"}));
  NAMES.forEach((name,index)=>{
   write("app/"+name+"/local.js","local "+name+"\n");
   const imports=[MENU[index%MENU.length]!,MENU[(index*5+3)%MENU.length]!,...index%4===0?[MENU[(index*7+1)%MENU.length]!]:[]];
   write("app/"+name+"/main.js",(index===7?[imp("../../lib/deep/inner/x")]:[...new Set(imports)]).join("\n")+"\n");
  });
 };
 const truth=()=>new Map(NAMES.map(name=>[name,walk(entryOf(name))]));
 return{root,store,at,write,imp,NAMES,entryOf,seed,truth,hold,factOf,walk,bare,relative,drop:()=>{rmSync(root,{recursive:true,force:true});rmSync(store,{recursive:true,force:true});}};
}

type Edit={paths:string[];subtrees:string[]};
const journal=(edit:Edit)=>{const exact=new Set<string>(),subtrees=new Set(edit.subtrees);for(const path of[...edit.paths,...edit.subtrees]){exact.add(path);exact.add(dirname(path));}return{paths:[...exact],directories:[...subtrees]};};

async function round(world:ReturnType<typeof worldOf>,previous:Map<string,{fact:string[];parts:Row[][]}>|undefined,edit:Edit|undefined){
 const old=previous,now=world.truth(),derived:string[]=[];let rows=0;
 const settler=createClosureSettlerV1({store:createNodeStoreV1({root:world.store})});
 const settled=await settler.settle({
  pseudoRows:new Map(world.NAMES.map(name=>[name,{kind:"derived",path:name,fact:old?.get(name)?.fact??"new"}])),
  derive:async(row:{path:string})=>{derived.push(row.path);const walked=world.walk(world.entryOf(row.path));return{fact:walked.fact,parts:walked.parts};},
  hold:async(list:Row[])=>{rows+=list.length;return world.hold(list);},
  ...edit===undefined?{}:{dirty:journal(edit)},
  vouchStamp:(name:string)=>stampOfV1(name),
 } as never);
 await settler.markIndexed({held:settled.held});
 const broken=old===undefined?[...world.NAMES]:world.NAMES.filter(name=>!old.get(name)!.parts.flat().every(row=>JSON.stringify(world.factOf(row))===JSON.stringify(row.fact)));
 const changed=old===undefined?[...world.NAMES]:world.NAMES.filter(name=>JSON.stringify(old.get(name)!.fact)!==JSON.stringify(now.get(name)!.fact));
 return{now,derived:derived.sort(),rows,slow:(settled as {slow?:number}).slow??-1,settled,broken:broken.sort(),changed:changed.sort()};
}

const sorted=(set:Iterable<string>)=>[...set].sort();

async function scenario(label:string,edit:(world:ReturnType<typeof worldOf>)=>Edit,check?:(result:Awaited<ReturnType<typeof round>>,world:ReturnType<typeof worldOf>)=>void){
 const world=worldOf();
 try{
  world.seed();
  const cold=await round(world,undefined,undefined);expect(cold.derived.length,label+": cold").toBe(world.NAMES.length);
  const change=edit(world),second=await round(world,cold.now,change);
  expect(second.derived,label+": derived exactly the closures one of whose rows broke").toEqual(second.broken);
  expect(sorted(second.settled.changed.keys()),label+": changed exactly the closures whose answer moved").toEqual(second.changed);
  expect(second.settled.failed,label).toBe(0);
  expect(second.settled.held.size+second.settled.kept.size+second.settled.changed.size,label).toBe(world.NAMES.length);
  check?.(second,world);
  const third=await round(world,second.now,{paths:[],subtrees:[]});expect(third.derived,label+": nothing moved since").toEqual([]);
 }finally{world.drop();}
}

const readers=(world:ReturnType<typeof worldOf>,needle:string)=>world.NAMES.filter(name=>world.walk(world.entryOf(name)).parts.flat().some(row=>row.path.includes(needle)||row.fact===needle));

test("the mini resolver agrees with createRequire for installed and linked packages and relative modules",()=>{
 const world=worldOf();
 try{
  world.seed();
  const from=world.entryOf("q00"),require=createRequire(from);
  for(const request of["pkg","linked"])expect(world.bare(from,request),request).toBe(realpathSync(require.resolve(request)));
  expect(world.relative(from,"./local")).toBe(require.resolve("./local"));
  expect(world.relative(from,"./missing")).toBe(MISSING);expect(()=>require.resolve("./missing")).toThrow();
  expect(world.bare(from,"unknown-pkg")).toBe(MISSING);expect(()=>require.resolve("unknown-pkg")).toThrow();
 }finally{world.drop();}
});

test("an edit of a file that closures read moves exactly the closures that read it",async()=>{
 await scenario("edit",world=>{world.write("lib/shared.js","shared v1\n");return{paths:[world.at("lib/shared.js")],subtrees:[]};},(result,world)=>{
  expect(result.derived.length).toBeGreaterThan(0);expect(result.derived.length).toBeLessThan(world.NAMES.length);
  expect(result.derived).toEqual(sorted(readers(world,world.at("lib/shared.js"))));
 });
});

test("the creation of a path that was probed and absent moves exactly the closures that probed it",async()=>{
 await scenario("create",world=>{world.write("app/q01/missing.js","now here\n");return{paths:[world.at("app/q01/missing.js")],subtrees:[]};},(result,world)=>{expect(result.derived).toEqual(["q01"]);expect(result.changed).toEqual(["q01"]);expect(result.slow,"only the closures that probed the directory are asked").toBeLessThan(world.NAMES.length);});
});

test("the deletion of a file that is read moves exactly the closures that read it",async()=>{
 await scenario("delete",world=>{unlinkSync(world.at("app/q00/local.js"));return{paths:[world.at("app/q00/local.js")],subtrees:[]};},result=>{expect(result.derived).toEqual(["q00"]);expect(result.changed).toEqual(["q00"]);});
});

test("a change of a directory listing moves exactly the closures that listed it",async()=>{
 await scenario("listing",world=>{world.write("lib/dir/c.js","c\n");return{paths:[world.at("lib/dir/c.js")],subtrees:[]};},(result,world)=>{expect(result.derived.length).toBeGreaterThan(0);expect(result.derived).toEqual(sorted(readers(world,world.at("lib/dir"))));});
});

test("a compiler-style configuration edit moves only the closures whose bare resolution it changes",async()=>{
 await scenario("tsconfig",world=>{world.write("tsconfig.json",JSON.stringify({paths:{"@alias":"lib/other.js"}}));return{paths:[world.at("tsconfig.json")],subtrees:[]};},(result,world)=>{
  expect(result.derived).toEqual(sorted(readers(world,"@alias")));expect(result.derived.length).toBeGreaterThan(0);expect(result.derived.length).toBeLessThan(world.NAMES.length);
 });
});

test("a package manifest edit moves only the closures whose bare resolution it changes, and an unrelated manifest moves none and checks no row beyond the unbounded ones",async()=>{
 await scenario("manifest",world=>{world.write("package.json",JSON.stringify({aliases:{fancy:"lib/other.js"}}));return{paths:[world.at("package.json")],subtrees:[]};},(result,world)=>{
  expect(result.derived).toEqual(sorted(readers(world,"fancy")));expect(result.derived.length).toBeGreaterThan(0);
 });
 await scenario("unrelated",world=>{world.write("other/package.json",JSON.stringify({name:"other",version:"2"}));world.write("docs/readme.md","docs 2\n");world.write("tsconfig.base.json","{}\n");return{paths:[world.at("other/package.json"),world.at("docs/readme.md"),world.at("tsconfig.base.json")],subtrees:[]};},(result,world)=>{
  expect(result.derived).toEqual([]);expect(result.changed).toEqual([]);expect(result.slow,"no closure is asked: the unrelated paths reach no part").toBe(0);
  const bareRows=new Set(world.NAMES.flatMap(name=>world.walk(world.entryOf(name)).parts.flat().filter(row=>row.kind==="resolve"&&!JSON.parse(row.path)[1].startsWith(".")).map(row=>row.path)));
  expect(result.rows,"only the bare resolutions are asked again, once each, not the rows of every closure that has one").toBeLessThanOrEqual(bareRows.size);
 });
});

test("an installed package edit moves only the closures that resolve it",async()=>{
 await scenario("node_modules",world=>{world.write("node_modules/pkg/package.json",JSON.stringify({main:"alt.js"}));return{paths:[world.at("node_modules/pkg/package.json")],subtrees:[]};},(result,world)=>{
  expect(result.derived).toEqual(sorted(readers(world,'"pkg"')));expect(result.derived.length).toBeGreaterThan(0);
 });
 await scenario("new install",world=>{world.write("node_modules/unknown-pkg.js","now installed\n");return{paths:[world.at("node_modules/unknown-pkg.js")],subtrees:[]};},(result,world)=>{expect(result.derived).toEqual(sorted(readers(world,"unknown-pkg")));expect(result.derived.length).toBeGreaterThan(0);});
});

test("a symlink that is pointed elsewhere moves exactly the closures that resolve through it",async()=>{
 await scenario("symlink",world=>{unlinkSync(world.at("node_modules/linked"));symlinkSync("../libs/b",world.at("node_modules/linked"));return{paths:[world.at("node_modules/linked")],subtrees:[]};},(result,world)=>{expect(result.derived).toEqual(sorted(readers(world,"linked")));expect(result.derived.length).toBeGreaterThan(0);});
});

test("a directory that moves away takes every row below it along, although the journal names only the directory",async()=>{
 await scenario("deep rename",world=>{renameSync(world.at("lib/deep"),world.at("lib/deep-moved"));return{paths:[world.at("lib/deep"),world.at("lib/deep-moved")],subtrees:[world.at("lib/deep"),world.at("lib/deep-moved")]};},(result,world)=>{expect(result.derived.length).toBeGreaterThan(0);expect(result.derived).toEqual(sorted(readers(world,"deep")));expect(result.changed).toEqual(result.derived);});
 await scenario("rename",world=>{renameSync(world.at("lib/dir"),world.at("lib/dir2"));return{paths:[world.at("lib/dir"),world.at("lib/dir2")],subtrees:[world.at("lib/dir"),world.at("lib/dir2")]};},(result,world)=>{expect(result.derived.length).toBeGreaterThan(0);expect(result.derived).toEqual(result.broken);expect(world.NAMES.length).toBeGreaterThan(result.derived.length);});
 await scenario("link target moves",world=>{renameSync(world.at("libs/a"),world.at("libs/a2"));return{paths:[world.at("libs/a"),world.at("libs/a2")],subtrees:[world.at("libs/a"),world.at("libs/a2")]};});
});

test("law: random edit sequences give the answers of a full re-walk, derive exactly the closures with a broken row and never miss a change",async()=>{
 for(let seed=1;seed<=10;seed++){
  const world=worldOf();let state=seed*7919;const draw=(limit:number)=>{state=(Math.imul(state,1103515245)+12345)>>>0;return(state>>>8)%limit;};
  try{
   world.seed();let previous=(await round(world,undefined,undefined)).now;
   for(let step=0;step<8;step++){
    const paths:string[]=[],subtrees:string[]=[],name=world.NAMES[draw(world.NAMES.length)]!;
    for(let count=0;count<1+draw(3);count++){
     const kind=draw(14);
     if(kind===0){world.write("app/"+name+"/local.js","local "+step+" "+draw(5)+"\n");paths.push(world.at("app/"+name+"/local.js"));}
     else if(kind===1){world.write("app/"+name+"/missing.js","m "+step+"\n");paths.push(world.at("app/"+name+"/missing.js"));}
     else if(kind===2){const file=world.at("app/"+name+"/missing.js");try{unlinkSync(file);paths.push(file);}catch{}}
     else if(kind===3){const file=world.at("app/"+name+"/main.js");world.write("app/"+name+"/main.js",["./local","fancy","pkg","../../lib/dir/*","./missing"].map(world.imp).filter(()=>draw(2)===0).join("\n")+"\n");paths.push(file);}
     else if(kind===4){world.write("lib/shared.js","shared "+step+"\n");paths.push(world.at("lib/shared.js"));}
     else if(kind===5){const file=world.at("lib/dir/n"+draw(3)+".js");if(draw(2)===0){world.write("lib/dir/"+file.split("/").pop(),"n\n");paths.push(file);}else{try{unlinkSync(file);paths.push(file);}catch{}}}
     else if(kind===6){world.write("tsconfig.json",JSON.stringify({paths:{"@alias":["lib/shared.js","lib/other.js","lib/missing.js"][draw(3)]}}));paths.push(world.at("tsconfig.json"));}
     else if(kind===7){world.write("package.json",JSON.stringify({aliases:{fancy:["lib/fancy.js","lib/other.js"][draw(2)]}}));paths.push(world.at("package.json"));}
     else if(kind===8){world.write("node_modules/pkg/package.json",JSON.stringify({main:["main.js","alt.js"][draw(2)]}));paths.push(world.at("node_modules/pkg/package.json"));}
     else if(kind===9){const link=world.at("node_modules/linked");try{unlinkSync(link);}catch{}symlinkSync(["../libs/a","../libs/b"][draw(2)]!,link);paths.push(link);}
     else if(kind===10){const from=world.at(["lib/dir","libs/a","lib/deep"][draw(3)]!),to=from+"-moved";try{if(statSync(from).isDirectory()){renameSync(from,to);paths.push(from,to);subtrees.push(from,to);}}catch{try{renameSync(to,from);paths.push(from,to);subtrees.push(from,to);}catch{}}}
     else if(kind===11){world.write("docs/readme.md","docs "+step+"\n");paths.push(world.at("docs/readme.md"));}
     else if(kind===12){world.write("other/package.json",JSON.stringify({name:"other",step}));paths.push(world.at("other/package.json"));}
     else{world.write("node_modules/unknown-pkg.js","installed "+step+"\n");paths.push(world.at("node_modules/unknown-pkg.js"));}
    }
    const result=await round(world,previous,{paths,subtrees});
    expect(result.derived,"seed "+seed+" step "+step+": derived").toEqual(result.broken);
    expect(sorted(result.settled.changed.keys()),"seed "+seed+" step "+step+": changed").toEqual(result.changed);
    expect(result.settled.failed).toBe(0);
    previous=result.now;
   }
  }finally{world.drop();}
 }
},180000);
