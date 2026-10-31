import {afterAll} from "bun:test";
import {profile} from "bun:jsc";
import {existsSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {prepareRepoCpuProfileArgumentsV1} from "../🟨️.mjs";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const protocol=schema.definitions.Protocol.const,args=prepareRepoCpuProfileArgumentsV1(process.env);
if(args.length){
 const directory=args[2].slice(protocol.directoryFlag.length+1),name=args[3].slice(protocol.nameFlag.length+1),output=join(directory,name.slice(0,-protocol.artifactSuffix.length)+protocol.testArtifactSuffix);
 if(output.length>protocol.maximumPathCharacters||existsSync(output))throw Error("Fresh bounded CPU sampling artifact required");
 let finish;const held=new Promise(resolve=>finish=resolve),observed=profile(()=>held,protocol.samplingIntervalMicroseconds);
 afterAll(async()=>{finish();const result=await observed;if(typeof result.functions!=="string"||typeof result.bytecodes!=="string"||!result.stackTraces||!Array.isArray(result.stackTraces.traces))throw Error("Actual CPU sampling result required");writeFileSync(output,JSON.stringify({version:1,runtime:"bun",pid:process.pid,functions:result.functions,bytecodes:result.bytecodes,stackTraces:result.stackTraces})+"\n",{flag:"wx",mode:0o600});});
}
