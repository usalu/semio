import{expect,test}from"bun:test";
import{partOwnerScopeV1}from"../../🟦️.ts";

/** 🔭️ Law: a part read infers the owners of its wave only; the owners it names ahead reach the inference as `aheadRoots` and never as `ownerRoots`, and a cold part infers everything and names nothing. */
test("a warm part infers its roots and names the next roots ahead, a cold part names no scope",()=>{
 const roots=new Set(["a","b"]),ahead=new Set(["c","d"]);
 expect(partOwnerScopeV1({cold:false,roots,ahead})).toEqual({ownerRoots:roots,aheadRoots:ahead});
 expect(partOwnerScopeV1({cold:false,roots})).toEqual({ownerRoots:roots});
 expect(partOwnerScopeV1({cold:true,roots,ahead})).toEqual({});
 expect([...partOwnerScopeV1({cold:false,roots,ahead}).ownerRoots!].some(root=>ahead.has(root))).toBe(false);
});
