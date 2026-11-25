import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{lstatSync,mkdirSync,mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import fixtures from"./🧫️fixtures/🔣️.json" with{type:"json"};
import{javascriptDependencyCapsuleIdentityV1 as identity,javascriptDependencyStampV1}from"../../🟦️.ts";

for(const row of fixtures.cases)test(`🧷️ capsule identity: ${row.name}`,()=>{
 expect(identity(JSON.stringify(row.observed))===identity(JSON.stringify(row.recorded))).toBe(row.same);
});

test.skipIf(process.platform!=="darwin")("🧷️ capsule identity survives a metadata write that moves only ctime, as macOS does with com.apple.macl",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"capsule-identity-"))),capsule=join(root,".p0123abcd");
 try{
  mkdirSync(capsule);const before=javascriptDependencyStampV1(lstatSync(capsule,{bigint:true}));
  await new Promise(done=>setTimeout(done,20));
  expect(spawnSync("xattr",["-w","tech.semio.test",String(Date.now()),capsule]).status).toBe(0);
  const after=javascriptDependencyStampV1(lstatSync(capsule,{bigint:true}));
  expect(after).not.toBe(before);
  expect(identity(after)).toBe(identity(before));
 }finally{rmSync(root,{recursive:true,force:true});}
});
