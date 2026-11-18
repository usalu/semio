/** 🩺️ Contract: a SQLite file that cannot be opened is explained with its path, the errno of a create probe and the free space, in English and German, and keeps its original code; a missing parent directory of a database opened for creation is created once. */
import{expect,test}from"bun:test";
import{mkdtempSync,rmSync,writeFileSync,existsSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{SqliteDatabaseV1,explainSqliteOpenFailureV1,type SqliteFilesystemV1}from"../../🟦️.ts";

test("a database below a regular file is refused with the path, the probe errno and both languages",()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-r79b-sqlite-"));
 try{writeFileSync(join(root,"file"),"x");const path=join(root,"file","lease.sqlite");let caught:unknown;
  try{new SqliteDatabaseV1(path,{create:true,readwrite:true});}catch(error){caught=error;}
  const error=caught as Error&{path?:string;probe?:string};expect(error).toBeInstanceOf(Error);expect(error.message).toContain(path);expect(error.message).toContain("create probe ENOTDIR");expect(error.message).toContain("Datenbank");expect(error.path).toBe(path);expect(error.probe).toBe("ENOTDIR");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a missing parent directory of a database opened for creation is created once; a read-only open does not create it",()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-r79b-sqlite-"));
 try{const path=join(root,"a","b","lease.sqlite"),database=new SqliteDatabaseV1(path,{create:true,readwrite:true});database.exec("CREATE TABLE t(x)");database.close();expect(existsSync(path)).toBe(true);
  expect(()=>new SqliteDatabaseV1(join(root,"c","d","x.sqlite"),{readonly:true})).toThrow();expect(existsSync(join(root,"c"))).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("only an open failure is rewritten and the original code survives",()=>{
 const original=Object.assign(Error("unable to open database file"),{code:"SQLITE_CANTOPEN",errno:14});
 const explained=explainSqliteOpenFailureV1(original,"/nonexistent-r79b/dir/x.sqlite") as Error&{code?:string;errno?:number};expect(explained.code).toBe("SQLITE_CANTOPEN");expect(explained.errno).toBe(14);expect(explained.message).toContain("/nonexistent-r79b/dir/x.sqlite");
 const other=Error("database is locked");expect(explainSqliteOpenFailureV1(other,"/x")).toBe(other);expect(explainSqliteOpenFailureV1(original,":memory:")).toBe(original);
});

const fake=(failure:{open?:string;write?:string}):SqliteFilesystemV1=>({mkdirSync:()=>undefined,openSync:()=>{if(failure.open)throw Object.assign(Error(failure.open),{code:failure.open});return 7;},writeSync:()=>{if(failure.write)throw Object.assign(Error(failure.write),{code:failure.write});return 4096;},closeSync:()=>undefined,unlinkSync:()=>undefined,lstatSync:()=>{throw Object.assign(Error("ENOENT"),{code:"ENOENT"});},statfsSync:()=>({bavail:3,bsize:4096,ffree:0})});
const cantopen=()=>Object.assign(Error("unable to open database file"),{code:"SQLITE_CANTOPEN",errno:14,errcode:14});

test("a full disk or an exhausted quota is named with the directory that is full, in English and German, through the filesystem port (EDQUOT on the write, ENOSPC on the create)",()=>{
 for(const [label,failure,errno] of [["quota on the first block",{write:"EDQUOT"},"EDQUOT"],["disk full at create",{open:"ENOSPC"},"ENOSPC"],["quota at create",{open:"EDQUOT"},"EDQUOT"]] as const){
  const path="/home/deck/.cache/semio/leases/abc.sqlite",error=explainSqliteOpenFailureV1(cantopen(),path,fake(failure)) as Error&{space?:boolean;directory?:string;probe?:string;code?:string;errno?:number};
  expect(error.space,label).toBe(true);expect(error.directory).toBe("/home/deck/.cache/semio/leases");expect(error.probe).toBe(errno);expect(error.code).toBe("SQLITE_CANTOPEN");expect(error.errno).toBe(14);
  expect(error.message.startsWith(`No space left (${errno}) in /home/deck/.cache/semio/leases:`),label).toBe(true);
  expect(error.message).toContain(`Kein Platz mehr (${errno}) in /home/deck/.cache/semio/leases`);expect(error.message).toContain("free 12288 bytes 0 files");expect(error.message).toContain(path);
 }
});

test("an unopenable file whose directory accepts files is not a full disk",()=>{
 const error=explainSqliteOpenFailureV1(cantopen(),"/data/x.sqlite",fake({})) as Error&{space?:boolean;probe?:string};
 expect(error.space).toBe(false);expect(error.probe).toBe("ok");expect(error.message).not.toContain("No space left");expect(error.message).toContain("create probe ok");
 const denied=explainSqliteOpenFailureV1(cantopen(),"/data/x.sqlite",fake({open:"EACCES"})) as Error&{space?:boolean};expect(denied.space).toBe(false);
});
