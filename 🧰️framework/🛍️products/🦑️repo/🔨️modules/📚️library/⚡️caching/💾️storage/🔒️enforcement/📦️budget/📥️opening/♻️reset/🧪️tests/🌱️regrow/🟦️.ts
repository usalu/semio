/** 🧪️ Contract: the pressed regrow is a closed script verb for the three elastic partitions only, every regrow target of the project runs it with `--pressed`, and the boolean regrow of the product is the outcome `retired` of the same function. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import budget from "../../../../🔣️policy.json" with {type:"json"};
import {readElasticRegrowArgumentsV1} from "../../📜️script.ts";

const project=JSON.parse(readFileSync(process.env.R172_MIRROR?join(process.env.R172_MIRROR,"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/📋️project.json"):join(import.meta.dirname,"../../📋️project.json"),"utf8")) as {targets:Record<string,{options:{command:string;cwd:string}}>};
test("only an elastic profile and the pressed flag are read",()=>{
 expect(readElasticRegrowArgumentsV1(["rust","--pressed"])).toEqual({profile:"rust",pressed:true});
 expect(readElasticRegrowArgumentsV1(["graph"])).toEqual({profile:"graph",pressed:false});
 for(const bad of [[],["nx"],["rust","--force"],["rust","--pressed","x"],["../rust"]])expect(()=>readElasticRegrowArgumentsV1(bad)).toThrow();
});
test("every elastic partition of the budget has a regrow target that runs the pressed verb in this directory, and no other partition has one",()=>{
 const targets=Object.keys(project.targets).filter(name=>name.startsWith("cache-regrow-")).map(name=>name.slice("cache-regrow-".length)).sort();
 expect(targets).toEqual(Object.keys(budget.elastic).sort());
 for(const name of targets){expect(project.targets["cache-regrow-"+name].options.command).toBe("repo ./📜️script.ts regrow "+name+" --pressed");expect(project.targets["cache-regrow-"+name].options.cwd).toBe(project.targets["cache-reset-"+name].options.cwd);}
});
