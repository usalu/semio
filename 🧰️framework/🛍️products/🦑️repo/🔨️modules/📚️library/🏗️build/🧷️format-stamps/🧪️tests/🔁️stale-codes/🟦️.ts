/** 🧷️ Contract: the stale implementation codes are declared once in the churn protocol and every declared code is recognised by the restart of the format stamps. */
import {expect,test} from "bun:test";
import churn from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🔣️.json";
import {staleImplementationOfV1} from "../../🟨️.mjs";

test("the stale implementation codes are declared once and every declared code is recognised by the restart",()=>{
 for(const code of churn.definitions.Protocol.const.staleImplementationCodes)expect(staleImplementationOfV1(Object.assign(new Error("m"),{code}))?.origin,code).toBe("error");
 expect(churn.definitions.Protocol.const.staleImplementationCodes).toContain(churn.definitions.Protocol.const.staleImplementationCode);
});
