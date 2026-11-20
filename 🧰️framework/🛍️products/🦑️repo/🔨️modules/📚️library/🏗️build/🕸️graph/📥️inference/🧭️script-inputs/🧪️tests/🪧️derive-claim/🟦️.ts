/** 🪧️ Contract: the first derivation of a shared closure entry is claimed. Of two processes of one read that need the same missing entry, one takes the claim and derives it; the other waits for the stored entry instead of deriving it a second time, and takes the claim itself only when the first released it without a stored entry (its derivation failed). Without a verdict directory or a store nothing is claimed.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,readdirSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{claimDerivationV1,withClosurePassV1}from"../../🟨️.mjs";

const directory=mkdtempSync(join(realpathSync(tmpdir()),"derive-claim-"));

const id="/w\0recipe-claim-test-"+process.pid+"-"+Date.now();
const empty={load:async()=>undefined};

test("the second claimant waits while the first derives and takes the claim only after the first released it",()=>withClosurePassV1(async()=>{
 const first=await claimDerivationV1(id,empty,{}) as {release:()=>void};
 expect(typeof first.release).toBe("function");
 let second:unknown="pending";const waiting=claimDerivationV1(id,empty,{}).then(value=>{second=value;});
 await Bun.sleep(350);
 expect(second,"derived by the first: the second waits and does not derive").toBe("pending");
 first.release();
 await waiting;
 expect(typeof (second as {release:()=>void}).release,"released without an entry: the second derives").toBe("function");
 (second as {release:()=>void}).release();
},{verdicts:directory}));

test("a cancelled claim waiter keeps the live holder's custody",()=>withClosurePassV1(async()=>{
 const key=id+"-cancel",first=await claimDerivationV1(key,empty,{}) as {release:()=>void};
 const controller=new AbortController(),primary=Error("cancel derivation wait");
 let entered!:()=>void;
 const waiting=new Promise<void>(resolve=>{entered=resolve;});
 const pending=claimDerivationV1(key,{load:async()=>{entered();return undefined;}},{context:{phaseControl:{signal:controller.signal}}}).then(value=>({value}),error=>({error}));
 try{
  await waiting;
  controller.abort(primary);
  const guard=setTimeout(()=>first.release(),400);
  try{
   const outcome=await pending;
   if("value"in outcome)outcome.value?.release?.();
   expect("error"in outcome?outcome.error:undefined).toBe(primary);
   expect(readdirSync(directory)).toHaveLength(1);
  }finally{clearTimeout(guard);}
 }finally{first.release();}
},{verdicts:directory}));

test("no store, no claim",()=>withClosurePassV1(async()=>{
 expect(await claimDerivationV1(id,undefined,{})).toBeUndefined();
 rmSync(directory,{recursive:true,force:true});
},{verdicts:directory}));
