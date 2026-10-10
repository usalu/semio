import schema from "./🧬️schema/🔣️.json";
import {validateJsonSchemaSubset} from "../../../../../🧬️schema/✅️validator/🟦️.ts";
import {checkScriptInvocation, scriptInvocationBudget, type ScriptInvocation} from "../../../../../🏃️process/🧭️routing/🟦️.ts";

/** 📥️ Declares the original JSON source command and its independently authored child ceiling. */
export type JsonReadSourceCapabilities=Readonly<{sourceRoot:string;sourceCommand:"test-read-policy-source";maximumChildElapsedMilliseconds:number}>;

/** 🎛️ Retains the original incoming ports and one admitted finite child ceiling. */
export interface JsonReadSourceOwner {
  readonly invocation:ScriptInvocation<JsonReadSourceCapabilities>;
  childBudgetMilliseconds():number;
}

/** 🔐️ Admits the exact source owner before any child effect, retaining its original authority. */
export function admitJsonReadSourceOwner(invocation:ScriptInvocation,sourceRoot:string):JsonReadSourceOwner {
  checkScriptInvocation(invocation);
  if(validateJsonSchemaSubset(schema,invocation.capabilities).length) throw Error("Complete JSON source child capabilities required");
  const capabilities=invocation.capabilities as JsonReadSourceCapabilities,maximum=capabilities.maximumChildElapsedMilliseconds;
  if(capabilities.sourceRoot!==sourceRoot||invocation.policy.maximumElapsedMilliseconds===0||maximum>invocation.policy.maximumElapsedMilliseconds) throw Error("Original finite JSON source owner required");
  const original=invocation as ScriptInvocation<JsonReadSourceCapabilities>;
  return {invocation:original,childBudgetMilliseconds:()=>scriptInvocationBudget(original,maximum)};
}
