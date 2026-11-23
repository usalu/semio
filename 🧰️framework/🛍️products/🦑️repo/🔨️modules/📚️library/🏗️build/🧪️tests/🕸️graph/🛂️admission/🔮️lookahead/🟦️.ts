/** 🔮️ Contract: the admission closure may discover the owners that wait behind the one in work at the same time (`width`), and its result is exactly the one of the sequential walk: the same owners, files, targets and inputs, every owner asked once per distinct question, and a request that changed meanwhile (the targets grew) is another question.
 * @see ../../../../🕸️graph/🚪️admission/🔮️ahead/🟦️.ts */
import {expect,test} from "bun:test";
import {closeBuildOwnerAdmissionV1,readBuildOwnerRegistryV1,readBuildGraphSelectionV1} from "../../../../🕸️graph/🚪️admission/🟦️.ts";
import {createDiscoveryAheadV1} from "../../../../🕸️graph/🚪️admission/🔮️ahead/🟦️.ts";

const COUNT=40,children=(index:number)=>[2*index+1,2*index+2].filter(child=>child<COUNT);
const registry=readBuildOwnerRegistryV1({version:1,owners:Array.from({length:COUNT},(_,index)=>({name:"o"+index,root:"r"+index,manifest:"r"+index+"/📋️project.json",packages:[]})),catalogProviders:[]});
const selection=readBuildGraphSelectionV1({kind:"tasks",tasks:[{project:"o0",target:"test"}]});
const facts=(name:string,targets:readonly string[])=>{const index=Number(name.slice(1));return{project:{name,targets:Object.fromEntries(["test",...targets].map(target=>[target,{dependsOn:target==="test"?["^test"]:[]}]))},files:["r"+index+"/a.ts"],dependencies:children(index).map(child=>"o"+child)};};
const walk=async(width?:()=>number,delay=4)=>{
 const asked:string[]=[];let running=0,peak=0;
 const result=await closeBuildOwnerAdmissionV1(selection,registry,async(owner,targets,inputs)=>{asked.push(owner.name+"\0"+JSON.stringify([targets,inputs]));running++;peak=Math.max(peak,running);try{await new Promise(done=>setTimeout(done,delay));return facts(owner.name,targets);}finally{running--;}},undefined,undefined,undefined,width);
 return{result,asked,peak};
};

test("the closure with a width is exactly the sequential closure and discovers several owners at once",async()=>{
 const sequential=await walk(),parallel=await walk(()=>4);
 expect(parallel.result).toEqual(sequential.result);
 expect(sequential.result.owners).toEqual(Array.from({length:COUNT},(_,index)=>"o"+index).sort());
 expect(sequential.peak).toBe(1);
 expect(parallel.peak).toBeGreaterThan(1);expect(parallel.peak).toBeLessThanOrEqual(5);
 expect(new Set(parallel.asked).size).toBe(parallel.asked.length);
 for(const question of sequential.asked)expect(parallel.asked).toContain(question);
});

test("a width of zero never starts an owner early",async()=>{
 const none=await walk(()=>0);
 expect(none.peak).toBe(1);
});

test("the early flights are bound by the width and an owner asked with other targets is another flight",async()=>{
 let live=0,peak=0;const started:string[]=[];
 const flights=createDiscoveryAheadV1<{owner:string;targets:string[]},string>({key:request=>request.owner+"\0"+request.targets.join(","),width:()=>2,start:async request=>{started.push(request.owner+"\0"+request.targets.join(","));live++;peak=Math.max(peak,live);await new Promise(done=>setTimeout(done,5));live--;return request.owner+":"+request.targets.join(",");}});
 const wanted=["a","b","c","d"].map(owner=>({owner,targets:["t"]}));
 expect(flights.ahead(wanted)).toBe(2);expect(flights.running).toBe(2);
 expect(await Promise.all(wanted.map(request=>flights.take(request)))).toEqual(["a:t","b:t","c:t","d:t"]);
 expect(peak).toBe(4);
 expect(started.filter(row=>row.startsWith("a\0"))).toHaveLength(1);
 expect(await flights.take({owner:"a",targets:["t","u"]})).toBe("a:t,u");
 expect(started.filter(row=>row.startsWith("a\0"))).toHaveLength(2);
});

test("an early flight that fails is a failure of the owner only where the closure asks for it",async()=>{
 const flights=createDiscoveryAheadV1<string,string>({key:request=>request,start:async request=>{if(request==="bad")throw Error("bad owner");return request;},width:()=>3});
 flights.ahead(["bad","good"]);
 expect(await flights.take("good")).toBe("good");
 await expect(flights.take("bad")).rejects.toThrow("bad owner");
});
