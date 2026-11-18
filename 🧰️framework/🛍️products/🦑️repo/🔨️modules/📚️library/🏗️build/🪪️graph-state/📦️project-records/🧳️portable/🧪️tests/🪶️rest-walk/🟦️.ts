/** 🪶️ Contract: the members of a closure bundle besides its rows and layers (value, sidecar) are moved between the producer's spelling and the host's by walking them, never by writing the whole member out as one JSON text: the result is exactly what the text round trip gave, and no JSON text of the size of the member is made.
 * @see ../../🟦️.ts */
import{expect,test}from"bun:test";
import{hostOfRootV1,localBundleV1,localTextV1,movedRestV1,neutralBundleV1,neutralTextV1,packBundleV1,portableRowsPolicyV1,closureKeyV1,UnportableV1}from"../../🟦️.ts";

const policy=portableRowsPolicyV1,token=policy.rootToken,root="/Users/ueli/Documents/semio",host=hostOfRootV1(root),windows=hostOfRootV1("C:\\work\\semio");
const textWay=(rest:unknown,direction:"neutral"|"local",at=host)=>JSON.parse((direction==="neutral"?neutralTextV1:localTextV1)(JSON.stringify(rest),at,policy));
const samples=(direction:"neutral"|"local")=>{const own=direction==="neutral"?root:token;return[
 {value:{a:own+"/x/y.ts",n:3,f:1.5,t:true,z:null,list:[own+"/q",[own,"keep"],{[own+"/key"]:own+"/v"}]},sidecar:{facts:[[own+"/m.ts","other"]],members:[[own+"/a",own+"/b"],[own+"/a"]]}},
 {value:"plain",sidecar:{members:[["📦️"+own+"/emoji 🧩️.ts"],["quote\" and \\ backslash "+own+"/x"]]}},
 {value:[],sidecar:{}},
];};

test("walking the members gives exactly what the JSON text round trip gave, in both directions",()=>{
 for(const direction of["neutral","local"]as const)for(const rest of samples(direction))expect(movedRestV1(rest,direction,host,policy)).toEqual(textWay(rest,direction));
});

test("members that are no plain JSON, a Windows host and a token of another spelling behave as the text path does",()=>{
 const odd={members:new Set(["a"]),gone:undefined as unknown,n:Number.NaN};
 expect(movedRestV1(odd,"neutral",host,policy)).toEqual(textWay(odd,"neutral"));
 const way={value:root+"/w",sidecar:{members:[[root+"/w"]]}};
 expect(movedRestV1(way,"neutral",windows,policy)).toEqual(textWay(way,"neutral",windows));
 const residue={value:token+"/ok",sidecar:{members:[[policy.residue]]}};
 expect(()=>textWay(residue,"local")).toThrow(UnportableV1);expect(()=>movedRestV1(residue,"local",host,policy)).toThrow(UnportableV1);
});

test("a bundle of a library closure is moved, taken and packed without a JSON text of the size of its sidecar",()=>{
 const members=Array.from({length:60000},(_,at)=>[root+"/📦️a/"+at+"/mod.ts",root+"/b/"+(at%977)+".ts"]),recipe={kind:"walk",root},bundleOf=(own:string)=>({recipeKey:closureKeyV1(recipe),bundle:{recipe,rows:[],layers:[],value:{entry:own+"/entry.ts"},sidecar:{facts:[],members}}as never});
 const largest=[0],real=JSON.stringify;JSON.stringify=((...args:Parameters<typeof JSON.stringify>)=>{const out=real(...args);if(typeof out==="string")largest[0]=Math.max(largest[0]!,out.length);return out;})as typeof JSON.stringify;
 try{
  const packed=packBundleV1(bundleOf(root),host,new Map());expect(packed).toBeDefined();
  const neutral=neutralBundleV1(bundleOf(root),host,new Map());expect(neutral).toBeDefined();
  const local=localBundleV1({recipeKey:neutral!.recipeKey,bundle:neutral!.bundle},host,new Map());expect(local).toBeDefined();
  expect((local!.bundle as any).sidecar.members[7]).toEqual(members[7]);expect((neutral!.bundle as any).sidecar.members[7][0]).toStartWith(token);
 }finally{JSON.stringify=real;}
 expect(largest[0]!,"largest JSON text made while moving the bundle").toBeLessThan(1000000);
},120000);
