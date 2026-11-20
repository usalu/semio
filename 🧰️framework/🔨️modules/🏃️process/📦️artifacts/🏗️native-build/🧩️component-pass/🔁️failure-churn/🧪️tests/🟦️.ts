import {expect,test} from "bun:test";
import {resolve} from "node:path";
import {blake3} from "@noble/hashes/blake3.js";
import {failedDependencyDriftV1,runCargoPassAttemptsV1} from "../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

const utf8 = (text:string) => new TextEncoder().encode(text);
const oracle = (text:string) => Buffer.from(blake3(utf8(text))).toString("hex");
const pkg = (name:string) => "path+file:///workspace/" + name + "#0.1.0";
const unit = (name:string,indexes:readonly number[],packageName=name) => ({
  identity:name,pkg_id:pkg(packageName),dependencies:indexes.map(index=>({index})),
});
const units = [unit("failed",[1]),unit("direct",[2]),unit("transitive",[]),unit("unrelated",[]),unit("host",[],"direct")];

for (const row of fixture.cases) test(row.name,async () => {
  const workspace=resolve("workspace"),build=resolve("build"),target=resolve("target");
  const artifacts:{identity:string;filenames:string[]}[]=[],files=new Map<string,Uint8Array>(),events:string[]=[];
  const checksum=oracle(fixture.consumed);
  expect(utf8(fixture.replacement).length).toBe(utf8(fixture.consumed).length);
  for (const identity of ["direct","transitive","unrelated","host"]) {
    const source=resolve(workspace,identity+".rs"),stem=resolve(build,"out",identity+"-abc");
    artifacts.push({identity,filenames:[resolve(build,"out","lib"+identity+"-abc.rmeta")]});
    files.set(source,utf8(identity===row.changed ? fixture.replacement : fixture.consumed));
    files.set(stem+".d",utf8("# checksum:blake3="+checksum+" file_len:"+utf8(fixture.consumed).length+" "+source+"\n"));
  }

  const result=runCargoPassAttemptsV1(async suffix=>{
    events.push(suffix ? "retry" : "initial");
    return suffix ? row.retryExit : row.initialExit;
  },async exit=>{
    if (exit===0) return false;
    const drift=await failedDependencyDriftV1({
      workspace,build,target,graph:{units},artifacts,failed:new Map([[pkg("failed"),new Set([row.code])]]),
      read:async path=>files.get(path),
    });
    if (!drift) return false;
    expect(drift).toEqual({path:resolve(workspace,row.changed+".rs"),expected:checksum,observed:oracle(fixture.replacement)});
    events.push("replan");
    if (!row.graphStable) throw Error("graph changed");
    return true;
  });
  if (row.exit===null) await expect(result).rejects.toThrow("graph changed");
  else expect(await result).toBe(row.exit);
  expect(events).toEqual(row.attempts===2 ? ["initial","replan","retry"] : row.graphStable ? ["initial"] : ["initial","replan"]);
});
