import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {keyEnvironmentV1,unitKeyV1} from "../../../../🔏️key/🟦️.ts";
import {localKeyV1,portableRootsV1,remoteKeyV1} from "../../🟦️.ts";

/** 🏠️ The local unit key is as portable as the fleet key: the same crate content under two checkout roots, two task views or two machines is one local entry, and a unit whose bytes name a real path stays keyed on it. */
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const input=(root:string)=>{
 const environment=keyEnvironmentV1({CARGO_MANIFEST_DIR:root+"/crates/core",CARGO_PKG_NAME:"core",CARGO_CRATE_NAME:"core"},undefined);
 return {toolchain:"rustc 1",normalized:["--crate-name","core","crates/core/src/lib.rs","--out-dir",root+"/target/debug/deps"],sourceSha256:sha("source"),externs:[{name:"--extern x",sha256:sha("x")}],environment};
};
const roots=(root:string)=>portableRootsV1({cwd:root,workspace:root,hints:[root+"/crates/core"]});

test("the same content under two roots is one local key",()=>{
 const left=input("/work/views/aaaa1111/r"),right=input("/home/other/checkout");
 expect(unitKeyV1(left),"the raw key carries the root: the defect").not.toBe(unitKeyV1(right));
 expect(localKeyV1(left,roots("/work/views/aaaa1111/r"),[])).toBe(localKeyV1(right,roots("/home/other/checkout"),[]));
});

test("a changed source, extern or toolchain is another local key",()=>{
 const root="/work/views/aaaa1111/r",base=input(root),at=roots(root),key=localKeyV1(base,at,[]);
 expect(localKeyV1({...base,sourceSha256:sha("other")},at,[])).not.toBe(key);
 expect(localKeyV1({...base,externs:[{name:"--extern x",sha256:sha("y")}]},at,[])).not.toBe(key);
 expect(localKeyV1({...base,toolchain:"rustc 2"},at,[])).not.toBe(key);
});

test("a unit that holds a real path stays keyed on it and differs from the clean unit",()=>{
 const a="/work/views/aaaa1111/r",b="/work/views/bbbb2222/r",clean=localKeyV1(input(a),roots(a),[]);
 expect(localKeyV1(input(a),roots(a),[a+"/crates/core"])).not.toBe(clean);
 expect(localKeyV1(input(a),roots(a),[a+"/crates/core"])).not.toBe(localKeyV1(input(b),roots(b),[b+"/crates/core"]));
});

test("the local key and the fleet key agree on what they ignore",()=>{
 const a="/work/views/aaaa1111/r",b="/home/other/checkout",platform={os:"darwin",arch:"arm64"} as never;
 expect(remoteKeyV1(platform,input(a),roots(a))).toBe(remoteKeyV1(platform,input(b),roots(b)));
});
