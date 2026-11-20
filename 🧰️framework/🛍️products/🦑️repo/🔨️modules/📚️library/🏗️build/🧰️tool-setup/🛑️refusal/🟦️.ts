import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type {ToolReportV1, ToolStatusV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const policy = schema.definitions.SetupRefusalPolicy.const;
type RefusalStatusV1 = Exclude<ToolStatusV1, "ok" | "deferred">;
type ToolSetupRefusalV1 = Readonly<{id:string;phase:"verify";status:RefusalStatusV1;pin:string;found:string|null;location:string|null;primary:string|null}>;

const bounded = (value:string, length:number):string => value.replace(/[\u0000-\u001f\u007f-\u009f]/gu, " ").slice(0,length).toWellFormed();
const optional = (value:string|null, length:number):string|null => value === null ? null : bounded(value,length);

/** 🛑️ Preserves the verification result and its first diagnostic line for each tool that refused setup. */
export function toolSetupRefusalLinesV1(reports:readonly ToolReportV1[]):string[] {
  return reports.flatMap(({id,status,pin,found,location,message}) => {
    if (status === "ok" || status === "deferred") return [];
    const row:ToolSetupRefusalV1 = {id:bounded(id,policy.id),phase:"verify",status,pin:bounded(pin,policy.version),found:optional(found,policy.version),location:optional(location,policy.location),primary:optional(message?.split(/[\r\n]/u,1)[0] ?? null,policy.primary)};
    return [policy.prefix + JSON.stringify(row)];
  });
}

/** 🧾️ Admits a bounded setup verification refusal to the worker's failure record. */
export function readToolSetupRefusalLineV1(line:string):ToolSetupRefusalV1|undefined {
  if (!line.startsWith(policy.prefix) || line.length > policy.line) return undefined;
  try {
    const row:unknown = JSON.parse(line.slice(policy.prefix.length));
    if (!validateJsonSchemaSubset(schema.definitions.SetupRefusal,row,schema).length) return row as ToolSetupRefusalV1;
  } catch {}
  return undefined;
}
