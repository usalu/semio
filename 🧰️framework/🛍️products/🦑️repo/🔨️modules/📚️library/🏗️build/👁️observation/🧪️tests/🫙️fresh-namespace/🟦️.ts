import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{inventoryAncestryV1,recheckInventoryAncestryV1}from"../../📥️reuse/🟨️.mjs";

/** 🫙️ A machine that never stored a closure of one namespace (a worker that fetches records from the fleet as its first work) has no directory for it: the ancestry of that namespace is an absent leaf directory, not an error, and a directory that appears meanwhile is a typed change that makes the batch look again. */
const none=()=>{};
const fresh=()=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"ancestry-")));mkdirSync(join(root,"build-input-closures"));return root;};

test("the namespace directory of a fresh store may be absent, every ancestor above it may not",()=>{
 const root=fresh();
 try{
  const present=inventoryAncestryV1(root,join(root,"build-input-closures"),none);
  expect(present.absent).toBeUndefined();
  const absent=inventoryAncestryV1(root,join(root,"build-lock-closures"),none);
  expect(absent.absent).toBe(join(root,"build-lock-closures"));
  expect(absent.rows.at(-1)!.path).toBe(root);
  expect(()=>recheckInventoryAncestryV1(absent,none)).not.toThrow();
  expect(()=>inventoryAncestryV1(root,join(root,"missing-parent","build-lock-closures"),none)).toThrow();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a namespace directory that appears after the ancestry was taken is a typed change, not an absence",()=>{
 const root=fresh();
 try{
  const absent=inventoryAncestryV1(root,join(root,"build-lock-closures"),none);
  mkdirSync(join(root,"build-lock-closures"));
  expect(()=>recheckInventoryAncestryV1(absent,none)).toThrow(expect.objectContaining({code:"BUILD_STORE_CHANGED"}));
 }finally{rmSync(root,{recursive:true,force:true});}
});
