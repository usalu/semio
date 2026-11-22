/** 🔑️ Contract: the Cargo target key identifies what decides artifact compatibility (toolchain, Cargo, target, profile, features, package selection, rustflags, Cargo configuration, lane) and never the content of a `Cargo.toml` or `Cargo.lock`, which Cargo's own fingerprints handle inside one target directory. The key derivation is written twice, in production (`cargoRecipeInputsV1` with `cargoTargetRecipeCanonicalV1`) and in the independent Python oracle `./🐍️.py`, and both agree with the language-agnostic corpus `./🔣️.json`.
 * @see ../../🟦️.ts */
import{test,expect}from"bun:test";
import{createHash}from"node:crypto";
import{spawnSync}from"node:child_process";
import{fileURLToPath}from"node:url";
import{requireTestToolV1}from"../../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{cargoRecipeInputsV1,cargoRecipeKeyedV1}from"../../🟦️.ts";
import{cargoTargetRecipeCanonicalV1}from"../../../../../📦️artifacts/🏗️native-build/🎯️recipe/🟦️.ts";
import corpus from"./🔣️.json";

type Row={path:string;sha256:string};
const oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url)),vectors=fileURLToPath(new URL("./🔣️.json",import.meta.url));
const production=async(observed:readonly Row[],override:Record<string,unknown>={})=>{
 const digests=new Map(observed.map(row=>[row.path,row.sha256])),physical={digest:async(path:string)=>digests.get(path),recheck:async()=>{}};
 const result=await cargoRecipeInputsV1(observed.map(row=>row.path),["app"],physical,undefined,{read:async()=>undefined}),canonical=cargoTargetRecipeCanonicalV1({version:1,...corpus.recipe,...override,inputs:result.inputs});
 return{keyed:result.inputs.map(row=>row.path).sort(),sha256:createHash("sha256").update(canonical).digest("hex")};
};

test("the production key and the independent Python oracle reach the verdict of every corpus case",async()=>{
 const run=spawnSync(requireTestToolV1("python"),["-I",oracle,vectors],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 const verdicts=JSON.parse(run.stdout);
 for(const row of corpus.cases){
  const actual=await production(row.observed,"recipe" in row?row.recipe as Record<string,unknown>:{});
  const expected=(corpus.expect as Record<string,{keyed:string[];sha256:string}>)[row.id]!;
  expect(actual,row.id+" production").toEqual(expected);
  expect(verdicts[row.id],row.id+" oracle").toEqual(expected);
 }
});

test("lock and manifest content never moves the key; toolchain, target, profile, features, packages, rustflags, configuration and lane do",()=>{
 const hash=(id:string)=>(corpus.expect as Record<string,{sha256:string}>)[id]!.sha256;
 for(const pair of corpus.pairs)expect(hash(pair.a)===hash(pair.b),pair.a+" vs "+pair.b).toBe(pair.sameKey);
 expect(new Set(corpus.cases.filter(row=>!corpus.pairs.some(pair=>pair.b===row.id&&pair.sameKey)).map(row=>hash(row.id))).size).toBe(corpus.cases.filter(row=>!corpus.pairs.some(pair=>pair.b===row.id&&pair.sameKey)).length);
});

test("only Cargo.toml and Cargo.lock of any directory are witnesses, by file name on both separators",()=>{
 for(const path of["/w/Cargo.toml","/w/Cargo.lock","C:\\w\\app\\Cargo.toml","/w/a/b/Cargo.lock"])expect(cargoRecipeKeyedV1(path),path).toBe(false);
 for(const path of["/w/.cargo/config.toml","/w/.cargo/config","/w/rust-toolchain.toml","/w/rust-toolchain","/h/.cargo/config.toml","/w/config.toml","/w/NotCargo.toml","/w/Cargo.toml.bak"])expect(cargoRecipeKeyedV1(path),path).toBe(true);
});

test("a witness is still rechecked while it is not keyed",async()=>{
 let rechecked=0;const digests=new Map([["/w/Cargo.lock","1".repeat(64)],["/w/.cargo/config.toml","2".repeat(64)]]),seen:string[]=[];
 const result=await cargoRecipeInputsV1([...digests.keys()],["app"],{digest:async path=>{seen.push(path);return digests.get(path);},recheck:async()=>{rechecked++;}},undefined,{read:async()=>undefined});
 expect(result.inputs).toEqual([{path:"/w/.cargo/config.toml",sha256:"2".repeat(64)}]);expect(seen).toEqual(["/w/Cargo.lock","/w/.cargo/config.toml"]);
 await result.recheck();expect(rechecked).toBe(1);
});
