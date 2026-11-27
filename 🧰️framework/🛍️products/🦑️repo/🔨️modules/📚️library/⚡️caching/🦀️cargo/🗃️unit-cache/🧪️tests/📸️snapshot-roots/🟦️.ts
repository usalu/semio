/** 📸️ Contract: a unit compiled from one checkout snapshot is the same cache entry in every other snapshot and in the live tree whenever its inputs are byte-equal; only a crate whose bytes changed, and the crates that consume its artifact, get a new key. */
import {expect,test} from "bun:test";
import {checkoutSnapshotRootOfV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/📸️checkout-snapshot/🧭️place/🟦️.ts";
import {localKeyV1,portableRootsV1} from "../../🛰️fleet/🪪️identity/🟦️.ts";

const live="/Users/dev/semio",environment={HOME:"/Users/dev",CARGO_HOME:"/Users/dev/.cargo",RUSTUP_HOME:"/Users/dev/.rustup"};
const snapshotA="/Users/dev/semio/.cache/s/0123456789ab",snapshotB="/Users/dev/semio/.cache/s/fedcba987654";

type Crate=Readonly<{name:string;relative:string;source:string;dependsOn:readonly string[]}>;
const crates:readonly Crate[]=[
 {name:"base",relative:"🧰️framework/base",source:"base-v1",dependsOn:[]},
 {name:"mid",relative:"🧰️framework/mid",source:"mid-v1",dependsOn:["base"]},
 {name:"leaf",relative:"✏️s/leaf",source:"leaf-v1",dependsOn:["mid"]},
 {name:"other",relative:"✏️s/other",source:"other-v1",dependsOn:[]},
];
const keysIn=(root:string,sources:Readonly<Record<string,string>>)=>{
 const keys=new Map<string,string>();
 for(const crate of crates){
  const cwd=root+"/"+crate.relative,snapshot=checkoutSnapshotRootOfV1(cwd),roots=portableRootsV1({cwd,workspace:snapshot??live,hints:[cwd],...environment.CARGO_HOME?{cargoHome:environment.CARGO_HOME}:{},rustupHome:environment.RUSTUP_HOME});
  const input={toolchain:"rustc 1.0",normalized:["--crate-name",crate.name,cwd+"/src/lib.rs","--remap-path-prefix="+root+"/"+crate.relative+"="+crate.name,"--out-dir",root+"/target"],sourceSha256:sources[crate.name]??crate.source,externs:crate.dependsOn.map(name=>({name,sha256:"artifact:"+(sources[name]??crates.find(row=>row.name===name)!.source)})),environment:[["CARGO_MANIFEST_DIR",cwd]] as [string,string][]};
  keys.set(crate.name,localKeyV1(input,roots));
 }
 return keys;
};

test("snapshot roots are recognised by place alone",()=>{
 expect(checkoutSnapshotRootOfV1(snapshotA+"/🧰️framework/base")).toBe(snapshotA);
 expect(checkoutSnapshotRootOfV1(snapshotA)).toBe(snapshotA);
 expect(checkoutSnapshotRootOfV1(live+"/🧰️framework/base")).toBeUndefined();
 expect(checkoutSnapshotRootOfV1("/x/.cache/s/short/a")).toBeUndefined();
});

test("equal inputs give equal unit keys in two snapshots and in the live tree",()=>{
 const a=keysIn(snapshotA,{}),b=keysIn(snapshotB,{}),l=keysIn(live,{});
 for(const crate of crates){expect(a.get(crate.name)).toBe(b.get(crate.name));expect(a.get(crate.name)).toBe(l.get(crate.name));}
});

test("with one changed crate only it and the crates consuming its artifact get new keys",()=>{
 const before=keysIn(snapshotA,{}),after=keysIn(snapshotB,{mid:"mid-v2"});
 const changed=crates.filter(crate=>before.get(crate.name)!==after.get(crate.name)).map(crate=>crate.name).sort();
 expect(changed).toEqual(["leaf","mid"]);
});
