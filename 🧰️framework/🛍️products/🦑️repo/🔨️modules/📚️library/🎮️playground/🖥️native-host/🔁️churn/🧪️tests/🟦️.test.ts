import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {resolve,join} from "node:path";
import {createScriptProcessEnvelope,scriptProcessEnvironment} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {OwnedCommandFailure,runOwnedCommand} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import {concurrentEditOfV1,churnProtocolV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {createNativeHostSourceEditReceiverV1,nativeHostSourceEditProtocolV1} from "../🟦️.ts";
import cases from "./🔣️.json" with {type:"json"};

test("native host restores only request-bound terminal Source edits from a real child", async () => {
  const workspace=resolve(import.meta.dirname,"../../../../../../../../.."),source=join(workspace,"Cargo.toml");
  const database=new Database(":memory:");
  try {
    for(const row of cases){
      const child=createNativeHostSourceEditReceiverV1(workspace);
      const envelope=createScriptProcessEnvelope({version:1,owner:"test:native-host-source-edit",maximumElapsedMilliseconds:10000},{buildMode:row.mode},Date.now());
      const oracle=database.query("select json_extract(?1, '$.restored') as accepted").get(JSON.stringify(row)) as {accepted:0|1};
      let failure:unknown,drained=false,drainedBeforeTerminal=false;
      const cancel=new AbortController();
      try{
        await runOwnedCommand(process.execPath,[join(import.meta.dirname,"🧪️child.ts"),row.id,source,row.mode],workspace,"test:native-host-source-edit",10000,{
          env:scriptProcessEnvironment(envelope,{...process.env,...child.environment,SEMIO_BUILD_MODE:row.mode}),
          output:{maximumBytes:65536,maximumLines:256},
          signal:cancel.signal,
          onLine:(line,stream)=>{
            if(line==="fixture:drained")drained=true;
            if(line.startsWith(nativeHostSourceEditProtocolV1.marker))drainedBeforeTerminal=drained;
            child.observe(line,stream);
            if(row.id==="cancelled"&&line.startsWith(nativeHostSourceEditProtocolV1.marker))cancel.abort();
          },
        });
      }catch(error){failure=error;}
      if(row.status===null)expect(failure,row.id).toBeDefined();
      else expect(failure instanceof OwnedCommandFailure?failure.outcome.status:0,row.id).toBe(row.status);
      const restored=child.restore(failure);
      expect(Boolean(restored),row.id).toBe(Boolean(oracle.accepted));
      if(restored){
        expect(drainedBeforeTerminal).toBe(true);
        expect(concurrentEditOfV1(restored)).toEqual({reason:"input",path:source,rank:churnProtocolV1().profiles.build.rank-1});
      }
    }
  }finally{database.close();}
},30000);
