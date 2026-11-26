import catalogue from "../🔣️catalog.json" with { type: "json" };

/** 🚩️ The part of one catalogue flag that decides how its value is spelled. */
export type BuildOptionFlagV1 = Readonly<{ name: string; aliases?: readonly string[]; value: string; operations?: readonly string[] }>;

/** 📚️ The part of the neutral operations catalogue that names the options. */
export type BuildOptionCatalogV1 = Readonly<{ flags: readonly BuildOptionFlagV1[] }>;

/** 🔤️ Rewrites every `--option value` of an option the catalogue declares with a required value (enumerations, numbers, paths, lists; the global `--tool`, `--workers` and `--lang` among them) to `--option=value`, under the canonical long name and before the `--` separator, so that every later reader sees one spelling; options that take their value only separately, optionally or not at all, and everything after `--` stay as typed. */
export function normalizeBuildOptionsV1(args: readonly string[], catalog: BuildOptionCatalogV1 = catalogue as BuildOptionCatalogV1): string[] {
  const required = new Map<string, string>();
  for (const flag of catalog.flags) if (flag.value === "required") for (const spelling of [flag.name, ...(flag.aliases ?? [])]) required.set(spelling, flag.name);
  const result: string[] = [];
  for (let index = 0; index < args.length; index++) {
    const argument = args[index]!, canonical = required.get(argument), next = args[index + 1];
    if (argument === "--") { result.push(...args.slice(index)); break; }
    if (canonical !== undefined && next !== undefined && next !== "--" && !next.startsWith("-")) { result.push(`${canonical}=${next}`); index++; }
    else result.push(argument);
  }
  return result;
}

/** 🩹️ The command line the neutral operations catalogue declares to repair one named drift (`repairs`), the single source every message that advises a repair spells its command from; an undeclared repair is a programming error, never a guessed command. */
export function buildRepairCommandV1(id:string):string{
 const row=(catalogue as unknown as {repairs?:readonly {id:string;command:string}[]}).repairs?.find(candidate=>candidate.id===id);
 if(row===undefined)throw Error("Declared repair required: "+id);
 return row.command;
}

/** 🧭️ Projects operation arguments after global selections are consumed, preserving help and the opaque child tail. */
export function buildOperationArgumentsV1(args:readonly string[],catalog:BuildOptionCatalogV1=catalogue as BuildOptionCatalogV1):string[]{
 const flags=catalog.flags.filter(flag=>flag.operations===undefined&&flag.value==="required"),globals=new Set(flags.flatMap(flag=>[flag.name,...(flag.aliases??[])])),normalized=normalizeBuildOptionsV1(args,{flags}),boundary=normalized.indexOf("--"),head=normalized.slice(0,boundary<0?undefined:boundary);
 return [...head.filter(argument=>!globals.has(argument.split("=",1)[0]!)),...(boundary<0?[]:normalized.slice(boundary))];
}
