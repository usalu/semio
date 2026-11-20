/** 🔭️ Law: `scan()` returns the facts of every entry of a directory, a link not followed, equal to the facts of `entry()` and of an independent reader of the file system, with a bounded number of system calls per entry (a constant, where `names()` followed by `entry()` for each name costs several times as many). */
import {expect,test} from "bun:test";
import {lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,readlinkSync,rmSync,symlinkSync,writeFileSync,realpathSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {types} from "node:util";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import ts from "typescript";
import {takeArtifactRelativeNativeProfileV1,withArtifactRelativeDirectoryV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json"with{type:"json"};
import linuxStatxCases from "./linux-statx.json"with{type:"json"};
import rosterCases from "./roster.json"with{type:"json"};

test("Linux directory scan accepts the statx facts of installed directories, links, and files",()=>{
 const tree=ts.createSourceFile("descriptor.ts",readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true);
 const decoders=tree.statements.filter((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text.startsWith("readArtifactRelative")===true&&node.name.text.includes("LinuxStatx"));
 const findScan=(node:ts.Node):ts.MethodDeclaration|undefined=>{
  if(ts.isMethodDeclaration(node)&&node.name.getText(tree)==="scan"&&node.getText(tree).includes("statx"))return node;
  return ts.forEachChild(node,findScan);
 };
 const scan=findScan(tree)?.getText(tree),linux=/:([A-Za-z]+LinuxStatxV1)\(buffer\),type=kind\(facts\)/u.exec(scan??"")?.[1];
 expect(linux).toBeDefined();
 const code=ts.transpileModule(decoders.map(node=>node.getText(tree)).join("\n"),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 const run=new Function("artifactDescriptorTypesV1","exportedSchema",code+`return { scan:${linux}, publish:readArtifactRelativePublicationLinuxStatxV1 };`)(types,schema);
 const policy=schema.definitions.RelativeFilePublicationProtocol.const,p=schema.definitions.RelativePosixProtocol.const.linux.statx,admit=new Ajv({strict:true}).compile(schema.definitions.RelativeLinuxScanStatxCase),db=new Database(":memory:");
 try{
  for(const row of linuxStatxCases){
   expect(admit(row),row.id).toBe(true);
   const bytes=Buffer.alloc(p.bytes);
   bytes.writeUInt32LE(p.requiredMask,p.mask);
   bytes.writeUInt16LE(row.mode,p.mode);
   bytes.writeUInt32LE(row.links,p.links);
   bytes.writeBigUInt64LE(BigInt(row.size),p.size);
   bytes.writeBigUInt64LE(12345n,p.inode);
   bytes.writeUInt32LE(8,p.deviceMajor);
   bytes.writeUInt32LE(17,p.deviceMinor);
   const oracle=db.query("select (?1 & 61440) = 32768 and ?2 = 1 and ?3 <= ?4 as publish, (8 << 8) | 17 as device").get(row.mode,row.links,row.size,policy.maximumFileBytes)as{publish:number;device:number};
   expect(row.scan,row.id).toBe(true);
   expect(run.scan(bytes),row.id).toMatchObject({dev:BigInt(oracle.device),ino:12345n,mode:BigInt(row.mode),nlink:BigInt(row.links),size:BigInt(row.size)});
   expect(row.publication,row.id).toBe(Boolean(oracle.publish));
   if(row.publication)expect(run.publish(bytes),row.id).toEqual(run.scan(bytes));
   else expect(()=>run.publish(bytes),row.id).toThrow("Bounded single regular Linux publication statx facts required");
  }
 }finally{db.close();}
});

const world=(files:number)=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"descriptor-scan-"));
 for(let index=0;index<files;index++)writeFileSync(join(root,`file-${String(index).padStart(5,"0")}.txt`),"x".repeat(index%7));
 mkdirSync(join(root,"nested"));writeFileSync(join(root,"nested","inner.txt"),"inner");mkdirSync(join(root,"empty"));
 writeFileSync(join(root,"übung-é.txt"),"unicode");
 symlinkSync("file-00000.txt",join(root,"link-to-file"));symlinkSync("nested",join(root,"link-to-dir"));symlinkSync("does-not-exist",join(root,"link-dangling"));
 return root;
};
const sort=(names:string[])=>[...names].sort((a,b)=>Buffer.from(a).compare(Buffer.from(b)));
const calls=(profile:Readonly<Record<string,readonly[number,number]>>)=>["fstatSync","openat","fstatat","statx","closeSync","readlinkat"].reduce((sum,name)=>sum+(profile[name]?.[0]??0),0);

test("scan equals entry() and an independent reader for every entry, links not followed, in the order of names()",async()=>{
 const root=world(40);
 try{
  await withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{
   const rows=await directory.scan(),names=await directory.names();
   expect(rows.map(row=>row.name)).toEqual(names);
   expect(names).toEqual(sort(readdirSync(root)));
   for(const row of rows){
    const { name, ...facts }=row,entry=await directory.entry(name),own=lstatSync(join(root,name),{bigint:true});
    expect(facts).toEqual(entry as never);
    expect([row.dev,row.ino,row.mode,row.size,row.nlink,row.mtimeNs,row.ctimeNs]).toEqual([own.dev,own.ino,own.mode,own.size,own.nlink,own.mtimeNs,own.ctimeNs]);
    expect(row.kind).toBe(own.isSymbolicLink()?"link":own.isDirectory()?"directory":"file");
    if(row.kind==="link")expect(row.target).toBe(readlinkSync(join(root,name)));else expect(row.target).toBeUndefined();
    expect(Object.isFrozen(row)).toBe(true);
   }
   expect(rows.filter(row=>row.kind==="link").map(row=>row.target).sort()).toEqual(["does-not-exist","file-00000.txt","nested"]);
   expect(rows.find(row=>row.name==="nested")?.kind).toBe("directory");
  });
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("system calls per entry are a small constant for scan and several times as many for names() with entry() per name",async()=>{
 const root=world(600);
 try{
  await withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{
   takeArtifactRelativeNativeProfileV1();
   const rows=await directory.scan(),scanned=calls(takeArtifactRelativeNativeProfileV1());
   for(const name of await directory.names())await directory.entry(name);
   const looped=calls(takeArtifactRelativeNativeProfileV1());
   expect(scanned/rows.length).toBeLessThanOrEqual(2.5);
   expect(looped/rows.length).toBeGreaterThanOrEqual(5);
   expect(looped).toBeGreaterThan(2*scanned);
  });
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a checkpoint that refuses stops a scan",async()=>{
 const root=world(300);
 try{
  let count=0;
  await expect(withArtifactRelativeDirectoryV1(root,()=>{if(++count>40)throw Error("stop");},directory=>directory.scan())).rejects.toThrow("stop");
 }finally{rmSync(root,{recursive:true,force:true});}
});


test("exact roster matching preserves the full native scan and POSIX or Windows fences",async()=>{
 const root=world(130),database=new Database(":memory:");
 try{
  const actual=readdirSync(root),canonical=sort(actual),names=new Set([...actual].reverse());
  await withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{
   for(const row of rosterCases){
    const proposed=new Set([...names].filter(name=>name!==row.remove));
    if(row.add!==null)proposed.add(row.add);
    const oracle=database.query<{equal:number},[string,string]>(`
     select not exists(select value from json_each(?1) except select value from json_each(?2))
        and not exists(select value from json_each(?2) except select value from json_each(?1)) equal
    `).get(JSON.stringify(actual),JSON.stringify([...proposed]))!;
    expect(Boolean(oracle.equal),row.id).toBe(row.matches);
    expect(await directory.matchesNames(proposed),row.id).toBe(row.matches);
   }
   const changed=new Set(names),pending=directory.matchesNames(changed);
   changed.clear();
   expect(await pending).toBe(true);
   expect(()=>directory.matchesNames(new Proxy(names,{}))).toThrow("Bounded native expected directory names required");
   expect(()=>directory.matchesNames.call({},names)).toThrow();
   if(process.platform!=="win32"){
    takeArtifactRelativeNativeProfileV1();
    expect(await directory.names()).toEqual(canonical);
    const namesProfile=takeArtifactRelativeNativeProfileV1();
    expect(await directory.matchesNames(new Set(["known-wrong"]))).toBe(false);
    const matchProfile=takeArtifactRelativeNativeProfileV1();
    const counts=(profile:typeof namesProfile)=>Object.fromEntries(Object.entries(profile).map(([name,[count]])=>[name,count]));
    expect(counts(matchProfile)).toEqual(counts(namesProfile));
   }
  });
  const midReadCheck=process.platform==="win32"?4:8;
  let checked=0,armed=false;
  const sentinel=Error("cancel during roster check");
  await expect(withArtifactRelativeDirectoryV1(root,()=>{if(armed&&++checked===midReadCheck)throw sentinel;},async directory=>{
   armed=true;
   await directory.matchesNames(names);
  })).rejects.toBe(sentinel);
  await withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{expect(await directory.matchesNames(names)).toBe(true);});
  for(const mode of ["names","match"])for(const mutation of ["insert","replace"]){
   let checks=0,checking=false;
   const changed=join(root,mutation==="insert"?"foreign":"file-00000.txt");
   await expect(withArtifactRelativeDirectoryV1(root,()=>{
    if(!checking||++checks!==midReadCheck)return;
    if(mutation==="replace")rmSync(changed);
    writeFileSync(changed,"retain");
   },async directory=>{
    checking=true;
    await (mode==="names"?directory.names():directory.matchesNames(names));
   })).rejects.toThrow();
   if(mutation==="insert")rmSync(changed);
  }
 }finally{database.close();rmSync(root,{recursive:true,force:true});}
});
