import{expect,test}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

/** 🧹️ Contract: removing the Bun package cache is idempotent against a concurrent remover: an entry that vanishes between the listing and its removal is already removed, never an error (a second installer or purge of the same cache used to fail the whole stage with ENOENT), while a missing root stays refused. */
test("a sibling removed by someone else during the walk does not fail the removal",async()=>{
 const{removeJavascriptDependencyTreeV1:remove}=await import("../../🟦️.ts"),root=mkdtempSync(join(tmpdir(),"tree-removal-"));
 try{
  mkdirSync(join(root,"a"));mkdirSync(join(root,"z"));mkdirSync(join(root,"z","deep"),{recursive:true});writeFileSync(join(root,"z","deep","file"),"x");
  for(let index=0;index<300;index++)writeFileSync(join(root,"a","f"+index),"x");
  let ticks=0;
  const removed=await remove(root,async()=>{ticks++;rmSync(join(root,"z"),{recursive:true,force:true});},false);
  expect(ticks).toBeGreaterThan(0);
  expect(removed).toBeGreaterThan(299);
  expect(existsSync(root)).toBe(true);
  expect(existsSync(join(root,"a"))).toBe(false);
  expect(existsSync(join(root,"z"))).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a missing root is still refused",async()=>{
 const{removeJavascriptDependencyTreeV1:remove}=await import("../../🟦️.ts");
 await expect(remove(join(tmpdir(),"tree-removal-absent-"+process.pid),async()=>{},true)).rejects.toThrow();
});

test("a file a concurrent writer adds while a directory is emptied is removed by the next pass",async()=>{
 const{removeJavascriptDependencyTreeV1:remove}=await import("../../🟦️.ts"),root=mkdtempSync(join(tmpdir(),"tree-removal-"));
 try{
  mkdirSync(join(root,"a"));
  for(let index=0;index<300;index++)writeFileSync(join(root,"a","f"+index),"x");
  let written=false;
  await remove(root,async()=>{if(!written){written=true;mkdirSync(join(root,"a","late"));writeFileSync(join(root,"a","late","g"),"x");}},false);
  expect(written).toBe(true);
  expect(existsSync(join(root,"a"))).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

/** 🧽️ Contract: the Bun cache purge never fails because something keeps writing into the cache while it is emptied: every entry moves by one atomic rename out of the cache (its identity stays), what a writer keeps busy is left to the next purge. The independent oracle is the Python standard library listing the cache. */
test("a writer that keeps adding files cannot fail the purge, and the cache is empty with its identity kept",async()=>{
 const{purgeJavascriptDependencyBunCacheV1:purge}=await import("../../🟦️.ts"),{spawnSync}=await import("node:child_process"),{lstatSync,readdirSync}=await import("node:fs"),root=mkdtempSync(join(tmpdir(),"purge-"));
 try{
  mkdirSync(join(root,"bun-cache","@types","node@1"),{recursive:true});mkdirSync(join(root,"tmp"));
  for(let index=0;index<600;index++)writeFileSync(join(root,"bun-cache","@types","node@1","f"+index),"x");
  writeFileSync(join(root,"bun-cache","loose"),"x");
  const before=lstatSync(join(root,"bun-cache")).ino;let writes=0;
  const removed=await purge(root,async()=>{const folder=readdirSync(join(root,"tmp")).find(name=>name.startsWith(".purge-"));if(folder===undefined)return;try{writeFileSync(join(root,"tmp",folder,"@types","late"+writes++),"x");}catch{}});
  expect(removed).toBeGreaterThan(0);
  expect(lstatSync(join(root,"bun-cache")).ino).toBe(before);
  const listed=spawnSync("python3",["-c","import os,sys;print(len(os.listdir(sys.argv[1])))",join(root,"bun-cache")],{encoding:"utf8"});
  expect(listed.stdout.trim()).toBe("0");
  expect(writes).toBeGreaterThan(0);
  writes=-1e9;
  await purge(root,async()=>{});
  expect(readdirSync(join(root,"tmp")).filter(name=>name.startsWith(".purge-"))).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a store without the temporary role is purged in place",async()=>{
 const{purgeJavascriptDependencyBunCacheV1:purge}=await import("../../🟦️.ts"),root=mkdtempSync(join(tmpdir(),"purge-"));
 try{mkdirSync(join(root,"bun-cache","a"),{recursive:true});writeFileSync(join(root,"bun-cache","a","f"),"x");expect(await purge(root,async()=>{})).toBe(2);expect(existsSync(join(root,"bun-cache","a"))).toBe(false);expect(existsSync(join(root,"bun-cache"))).toBe(true);}finally{rmSync(root,{recursive:true,force:true});}
});

/** 🔬️ Contract: the paths of a Bun "moving to cache dir failed" error are observed on the filesystem (root-relative, existence of path and parent, one device), because the redacted diagnostic hides them. */
test("a Bun move failure is described by what exists on disk",async()=>{
 const{observeJavascriptDependencyMoveFailuresV1:observe}=await import("../../🟦️.ts"),root=mkdtempSync(join(tmpdir(),"moves-"));
 try{
  mkdirSync(join(root,"tmp"));mkdirSync(join(root,"bun-cache"));writeFileSync(join(root,"tmp","made"),"x");
  const text=(name:string,from:string,to:string)=>`\u001b[31merror\u001b[0m: moving "${name}" to cache dir failed: ENOENT: No such file or directory (rename())\n  From: ${from}\n    To: ${to}\n`;
  const rows=observe(text("@noble/hashes",join(root,"tmp","gone"),join(root,"bun-cache","@noble","hashes@1"))+text("@x/y",join(root,"tmp","made"),join(root,"bun-cache","y@1"))+text("a","/elsewhere/none","/elsewhere/none2"),root);
  expect(rows.map(row=>row.package)).toEqual(["@noble/hashes","@x/y","a"]);
  expect(rows[0]).toEqual({package:"@noble/hashes",from:"tmp/gone",fromExists:false,fromParentExists:true,to:"bun-cache/@noble/hashes@1",toExists:false,toParentExists:false,sameDevice:null,toParentKind:"missing",toParentCreate:"ok",cacheKind:"directory",cachePlainCreate:"ok",cacheDevice:true});
  expect(rows[1]).toMatchObject({fromExists:true,toExists:false,toParentExists:true,sameDevice:true});
  expect(rows[1]).toMatchObject({toParentKind:"directory",toParentCreate:"present"});
  writeFileSync(join(root,"bun-cache","@blocked"),"x");
  const blocked=observe(text("@blocked/p",join(root,"tmp","made"),join(root,"bun-cache","@blocked","p@1")),root);
  expect(blocked[0]).toMatchObject({toParentKind:"file",toParentCreate:"present"});
  const bare=observe(text("q","made",join(root,"bun-cache","q@1")),root);
  expect(bare[0]).toMatchObject({from:"tmp/made",fromExists:true});
  expect(rows[2]).toMatchObject({from:"outside",to:"outside",fromParentExists:false});
  expect(observe("error: unrelated",root)).toEqual([]);
  expect(observe(Array.from({length:9},(_,i)=>text("p"+i,join(root,"tmp","a"),join(root,"tmp","b"))).join(""),root).length).toBe(4);
  const oracle=Bun.spawnSync(["python3","-c","import os,sys;r=sys.argv[1];print(os.path.exists(os.path.join(r,'tmp','made')),os.path.exists(os.path.join(r,'bun-cache','@noble')))",root]).stdout.toString().trim();
  expect(oracle).toBe("True True");
 }finally{rmSync(root,{recursive:true,force:true});}
});

/** 🌱️ Contract: the scope directories of the scoped packages of the lock files exist in the Bun cache before an install, so a rename into `<cache>/@scope/…` never waits for Bun to create the parent; a relative `To` of a move error is read against the cache. */
test("scope directories of the lock are made in the cache, and a relative target is observed under it",async()=>{
 const{preseedJavascriptDependencyBunCacheScopesV1:seed,observeJavascriptDependencyMoveFailuresV1:observe}=await import("../../🟦️.ts"),root=mkdtempSync(join(tmpdir(),"seed-"));
 try{
  const cache=join(root,"bun-cache");mkdirSync(cache);mkdirSync(join(root,"tmp"));
  const lock=Buffer.from(JSON.stringify({packages:{"@noble/hashes":["@noble/hashes@1.8.0"],"@isaacs/cliui":["x"],"@isaacs/fs-minipass":["y"],"left-pad":["z"],"@bad/../x":["w"]}}));
  const made=seed(cache,[{path:"bun.lock",bytes:lock},{path:"package.json",bytes:Buffer.from('"@ignored/pkg"')}]);
  expect(made).toEqual(["@isaacs","@noble"]);
  expect(readdirSync(cache).sort()).toEqual(["@isaacs","@noble"]);
  expect(seed(cache,[{path:"bun.lock",bytes:lock}])).toEqual(["@isaacs","@noble"]);
  const listed=Bun.spawnSync(["python3","-c","import os,sys;print(sorted(os.listdir(sys.argv[1])))",cache]).stdout.toString().trim();
  expect(listed).toBe("['@isaacs', '@noble']");
  const rows=observe(`error: moving "@noble/hashes" to cache dir failed: ENOENT (rename())\n  From: ${join(root,"tmp","gone")}\n    To: @noble/hashes@1.8.0@@host@@@1\n`,root,cache);
  expect(rows[0]).toMatchObject({to:"bun-cache/@noble/hashes@1.8.0@@host@@@1",toExists:false,toParentExists:true,from:"tmp/gone",fromExists:false,sameDevice:true});
 }finally{rmSync(root,{recursive:true,force:true});}
});

/** 🪟️ Contract: a file a running program maps (Windows refuses to unlink a mapped esbuild.exe with EPERM, a rename is allowed) never fails the removal of its generation: it moves aside, the tree is gone, and a later removal unlinks it once the program has ended. Windows semantics are simulated by an unlink that refuses the mapped path; the independent oracle is the Python standard library listing the directories. */
test("a mapped executable is moved aside and unlinked by the next removal",async()=>{
 const{removeJavascriptDependencyTreeV1:remove}=await import("../../🟦️.ts"),parent=mkdtempSync(join(tmpdir(),"tree-removal-")),root=join(parent,".x420f71600f321d023284"),bin=join(root,"node_modules","@esbuild","win32-x64");
 try{
  mkdirSync(bin,{recursive:true});writeFileSync(join(bin,"esbuild.exe"),"MZ");writeFileSync(join(bin,"other.js"),"x");
  const{unlinkSync}=await import("node:fs");let mapped=true;
  const unlink=(path:string)=>{if(mapped&&path.endsWith("esbuild.exe")){throw Object.assign(new Error("EPERM: operation not permitted, unlink"),{code:"EPERM"});}unlinkSync(path);};
  await remove(root,async()=>{},true,unlink);
  expect(existsSync(root)).toBe(false);
  expect(readdirSync(join(parent,".busy"))).toHaveLength(1);
  const oracle=Bun.spawnSync(["python3","-I","-c","import os,sys;print(sum(len(f) for _,_,f in os.walk(sys.argv[1])))",parent]);
  expect(oracle.stdout.toString().trim()).toBe("1");
  mapped=false;
  const next=join(parent,".x0000000000000000000a");mkdirSync(next);writeFileSync(join(next,"f"),"x");
  await remove(next,async()=>{},true,unlink);
  expect(existsSync(join(parent,".busy"))).toBe(false);
  expect(readdirSync(parent)).toHaveLength(0);
 }finally{rmSync(parent,{recursive:true,force:true});}
});

test("a permission error that is not a busy file is still refused", async()=>{
 const{removeJavascriptDependencyTreeV1:remove}=await import("../../🟦️.ts"),parent=mkdtempSync(join(tmpdir(),"tree-removal-")),root=join(parent,"t");
 try{
  mkdirSync(root);writeFileSync(join(root,"f"),"x");
  const deny=()=>{throw Object.assign(new Error("EIO"),{code:"EIO"});};
  await expect(remove(root,async()=>{},true,deny)).rejects.toThrow("EIO");
 }finally{rmSync(parent,{recursive:true,force:true});}
});
