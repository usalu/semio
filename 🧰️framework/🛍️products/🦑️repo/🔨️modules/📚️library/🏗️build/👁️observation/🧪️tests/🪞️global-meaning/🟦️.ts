import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {observeGlobalFactV1,registerGlobalFactV1,settleObservedComputationV1} from "../../🟨️.mjs";

/** 🪞️ Contract: two reads of one `global` question agree when the `meaning` registered beside its resolver agrees (a closure walk answers the same although the digest of a file behind it moved), and contradict when the meaning differs; without a registered meaning the whole fact is compared; the meaning equals an independent Python extraction.
 * @see ../../🟨️.mjs */
const python=(fact:object)=>{const run=spawnSync("python3",["-I","-c","import json,sys;print(json.dumps(json.loads(sys.argv[1])['answer']))",JSON.stringify(fact)],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const read=(name:string,...facts:object[])=>settleObservedComputationV1(()=>{for(const fact of facts)observeGlobalFactV1(name,"/workspace",["question"],fact);return "settled";});

test("a digest that moved under an equal answer is no contradiction",async()=>{
 registerGlobalFactV1("gm-meaning",()=>true,undefined,fact=>fact.answer);
 const first={digest:"a",answer:"x"},second={digest:"b",answer:"x"};
 expect(python(first)).toEqual(python(second));
 expect(await read("gm-meaning",first,second)).toBe("settled");
});

test("a different answer is a contradiction that fails after the bounded rounds",async()=>{
 registerGlobalFactV1("gm-meaning",()=>true,undefined,fact=>fact.answer);
 const first={digest:"a",answer:"x"},second={digest:"b",answer:"y"};
 expect(python(first)).not.toEqual(python(second));
 await expect(read("gm-meaning",first,second)).rejects.toThrow("Conflicting inference observations");
},30000);

test("without a registered meaning the whole fact is compared",async()=>{
 registerGlobalFactV1("gm-whole",()=>true);
 await expect(read("gm-whole",{digest:"a",answer:"x"},{digest:"b",answer:"x"})).rejects.toThrow("Conflicting inference observations");
},30000);
