import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,statSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {UNIT_CACHE_POLICY_V1} from "../../../🔏️key/🟦️.ts";
import {ballastPathV1,ensureBallastV1,releaseBallastV1,tryClaimWithBallastV1} from "../🟦️.ts";

const rules=UNIT_CACHE_POLICY_V1.gc,scratch=():string=>mkdtempSync(join(realpathSync(tmpdir()),"ballast-"));

test("the ballast is created whole when the image has room for four of it, kept when present, and never taken from a nearly full image",()=>{
 const root=scratch();
 try{
  expect(ensureBallastV1(root,()=>({availableBytes:rules.ballastBytes*4-1}))).toBe("skipped");expect(existsSync(ballastPathV1(root))).toBe(false);
  expect(ensureBallastV1(root,()=>undefined)).toBe("skipped");
  expect(ensureBallastV1(root,()=>({availableBytes:rules.ballastBytes*4}))).toBe("created");expect(statSync(ballastPathV1(root)).size).toBe(rules.ballastBytes);
  expect(ensureBallastV1(root,()=>{throw Error("not asked while present");})).toBe("present");
  expect(readdirSync(root)).toEqual([rules.ballastFile]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("giving the ballast back frees its bytes once, a missing ballast frees nothing, and a write that fails leaves no temporary file",()=>{
 const root=scratch();
 try{
  expect(releaseBallastV1(root)).toBe(false);
  ensureBallastV1(root,()=>({availableBytes:Number.MAX_SAFE_INTEGER}));expect(releaseBallastV1(root)).toBe(true);expect(releaseBallastV1(root)).toBe(false);expect(readdirSync(root)).toEqual([]);
  const missing=join(root,"absent");expect(ensureBallastV1(missing,()=>({availableBytes:Number.MAX_SAFE_INTEGER}))).toBe("skipped");expect(existsSync(missing)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a claim refused for lack of space gives the ballast back and succeeds the second time, any other failure and a missing ballast are not hidden",()=>{
 const root=scratch(),locks=join(root,"k");mkdirSync(locks,{recursive:true});
 try{
  ensureBallastV1(root,()=>({availableBytes:Number.MAX_SAFE_INTEGER}));
  const full=Object.assign(Error("ENOSPC: no space left on device"),{code:"ENOSPC"});let attempts=0;
  const claim=tryClaimWithBallastV1(locks,"gc",(directory,name)=>{if(attempts++===0)throw full;return{path:join(directory,name),waitedMs:0,release(){}};});
  expect(claim).toBeDefined();expect(attempts).toBe(2);expect(existsSync(ballastPathV1(root))).toBe(false);
  expect(()=>tryClaimWithBallastV1(locks,"gc",()=>{throw full;})).toThrow("ENOSPC");
  expect(()=>tryClaimWithBallastV1(locks,"gc",()=>{throw Object.assign(Error("denied"),{code:"EACCES"});})).toThrow("denied");
  const real=tryClaimWithBallastV1(locks,"real");expect(real).toBeDefined();real!.release();
 }finally{rmSync(root,{recursive:true,force:true});}
});
