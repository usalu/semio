/** 🧷️ Contract: the host capacity policy the build ships (`🔣️policy.json` of the repository build) satisfies the closed `Policy` definition of the host capacity schema, for our reader and for a third-party validator alike, so a schema that gains a required field never refuses every command until the policy follows. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import buildPolicy from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json";
import schema from "../../🧬️schema/🔣️.json";
import {readHostCapacityPolicyV1} from "../../🟦️.ts";

test("the shipped host capacity policy matches the closed Policy definition and names every required swap field",()=>{
 const ajv=new Ajv({strict:false,allErrors:true});ajv.addSchema({$id:"hc",definitions:schema.definitions});
 const valid=ajv.validate({$ref:"hc#/definitions/Policy"},buildPolicy.hostCapacity);
 expect(ajv.errorsText(ajv.errors),"third-party validator").toBe("No errors");
 expect(valid).toBe(true);
 expect(()=>readHostCapacityPolicyV1(buildPolicy.hostCapacity)).not.toThrow();
 const swap=(schema.definitions as any).Policy.properties.swap;
 expect(Object.keys(buildPolicy.hostCapacity.swap).sort()).toEqual([...swap.required].sort());
});
