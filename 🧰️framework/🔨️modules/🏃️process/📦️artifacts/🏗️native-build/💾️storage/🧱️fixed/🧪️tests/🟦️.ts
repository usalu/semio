import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {nativeLinuxLoopMountV1,nativeFixedVhdFooterV1} from "../🟦️.ts";
import {nativeStoragePlatformV1} from "../../🌐️platform/🟦️.ts";
import {requireNativeStoragePathV1} from "../../🌐️platform/📋️contract/🟦️.ts";
import {mkdtempSync,mkdirSync,symlinkSync,lstatSync,rmSync} from "node:fs";
import {join,resolve} from "node:path";

test("native fixed storage selects every declared desktop platform and refuses foreign hosts",()=>{
 const database=new Database(":memory:");try{for(const row of fixture.platforms){const expected=database.query("SELECT CASE ? WHEN 'darwin' THEN 'macos' WHEN 'linux' THEN 'linux' WHEN 'win32' THEN 'windows' ELSE NULL END AS backend").get(row.platform) as {backend:"macos"|"linux"|"windows"|null};expect(row.backend).toBe(expected.backend);if(expected.backend===null)expect(()=>nativeStoragePlatformV1(row.platform)).toThrow();else expect(nativeStoragePlatformV1(row.platform)).toBe(expected.backend);}console.log("[DEBUG] Fixed storage host dispatch agrees with independent SQLite desktop selection");}finally{database.close();}
});

test("native output path admission rejects dangling symlinks before any materialization",()=>{
 if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw Error("Native storage path tests require explicit SEMIO_TEST_ARTIFACT_DIR");
 const root=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR),"p-")),database=new Database(":memory:");
 try{mkdirSync(join(root,"nested"));symlinkSync(join(root,"absent"),join(root,"dangling"),process.platform==="win32"?"junction":"dir");expect(lstatSync(join(root,"dangling")).isSymbolicLink()).toBe(true);for(const row of fixture.paths){const path=resolve(root,row.path),oracle=database.query("SELECT instr(? || '/', ? || '/') = 1 AND ? NOT LIKE 'dangling/%' AS admitted").get(path,root,row.path) as {admitted:number};expect(Boolean(oracle.admitted)).toBe(row.accepted);if(row.accepted)expect(requireNativeStoragePathV1(root,path)).toBe(path);else expect(()=>requireNativeStoragePathV1(root,path)).toThrow();}console.log("[DEBUG] No-follow output path admission refuses dangling symlink and foreign boundary before writes");}finally{database.close();rmSync(root,{recursive:true,force:true});}
});

test("neutral fixed backings require exact bounded system identities",()=>{
 const validate=new Ajv({strict:false}).compile(schema),db=new Database(":memory:");db.run("CREATE TABLE backing(type INTEGER,bytes INTEGER,file INTEGER)");
 expect(validate({version:1,root:"/repo/fixed/m",image:"/repo/fixed/fixed.img",filesystemBytes:33554432,maximumPhysicalBytes:34603008})).toBe(true);
 expect(validate({version:1,root:"/repo/fixed/m",image:"/repo/fixed/fixed.img",filesystemBytes:33554432,maximumPhysicalBytes:34603008,ambient:true})).toBe(false);
 for(const row of fixture.linux){if(row.valid)expect(nativeLinuxLoopMountV1(row.line+"\n",row.root).device).toMatch(/^7:/u);else expect(()=>nativeLinuxLoopMountV1(row.line+"\n",row.root)).toThrow();}
 for(const row of fixture.vhd){const footer=Buffer.alloc(512);footer.write("conectix");footer.writeBigUInt64BE(0xffffffffffffffffn,16);footer.writeBigUInt64BE(BigInt(row.currentBytes),40);footer.writeBigUInt64BE(BigInt(row.currentBytes),48);footer.writeUInt32BE(row.type,60);footer.writeUInt32BE((~footer.reduce((sum,byte)=>sum+byte,0))>>>0,64);db.run("INSERT INTO backing VALUES(?,?,?)",[row.type,row.currentBytes,row.fileBytes]);const independent=db.query("SELECT type=2 AND file=bytes+512 AND bytes<=33554432 AS valid FROM backing ORDER BY rowid DESC LIMIT 1").get() as {valid:number};expect(Boolean(independent.valid)).toBe(row.valid);if(row.valid)expect(nativeFixedVhdFooterV1(footer,row.fileBytes,34603008,33554432)).toBe(row.currentBytes);else expect(()=>nativeFixedVhdFooterV1(footer,row.fileBytes,34603008,33554432)).toThrow();footer[20]=footer[20]!^1;expect(()=>nativeFixedVhdFooterV1(footer,row.fileBytes,34603008,33554432)).toThrow();}
 db.close();
});
