import catalogue from "../🔣️catalog.json" with { type: "json" };

/** 🌐️ A text in every language the catalogue declares. */
export type BuildHelpTextV1=Readonly<Record<string,string>>;

/** 🚩️ One documented flag: its spelling, value shape, the operations that read it and its description. */
export type BuildHelpFlagV1=Readonly<{name:string;aliases?:readonly string[];value:string;valueName?:string;operations?:readonly string[];description:BuildHelpTextV1}>;

/** 🧩️ One subcommand of an operation. */
export type BuildHelpSubcommandV1=Readonly<{name:string;synopsis:string;summary:BuildHelpTextV1}>;

/** 🧭️ One operation as the help consumes it. */
export type BuildHelpOperationV1=Readonly<{name:string;synopsis:string;summary:BuildHelpTextV1;subcommands?:readonly BuildHelpSubcommandV1[];flags:readonly string[];examples:readonly Readonly<{command:string;description:BuildHelpTextV1}>[]}>;

/** 📚️ The part of the neutral operations catalogue that the help renders. */
export type BuildHelpCatalogV1=Readonly<{languages:readonly string[];labels:Readonly<Record<string,BuildHelpTextV1>>;flags:readonly BuildHelpFlagV1[];operations:readonly BuildHelpOperationV1[]}>;

/** 🎯️ Names what a help request asks for: the whole CLI, one operation or one subcommand. */
export type BuildHelpRequestV1=Readonly<{operation?:string;subcommand?:string}>;

const root="repo [--tool=nx|bazel]",columnLimit=46;

/** 🗣️ Picks the help language: `--lang`, then SEMIO_LANGUAGE, then the locale, then the first language the catalogue declares. */
export function resolveBuildHelpLanguageV1(languages:readonly string[],args:readonly string[],environment:Readonly<Record<string,string|undefined>>):string{
 const boundary=args.indexOf("--"),head=args.slice(0,boundary<0?undefined:boundary);let explicit:string|undefined;
 for(let index=0;index<head.length;index++){const argument=head[index]!;if(argument==="--lang")explicit=head[++index];else if(argument.startsWith("--lang="))explicit=argument.slice(7);}
 if(explicit!==undefined){if(!languages.includes(explicit))throw Error(`Unknown help language: ${explicit} (${languages.join("|")})`);return explicit;}
 for(const value of [environment.SEMIO_LANGUAGE,environment.LC_ALL,environment.LC_MESSAGES,environment.LANG]){const tag=value?.toLowerCase().split(/[._@-]/u)[0];if(tag&&languages.includes(tag))return tag;}
 return languages[0]!;
}

/** 🔎️ Reads which command and subcommand a help request names; no command means the whole CLI. */
export function readBuildHelpRequestV1(catalog:BuildHelpCatalogV1,args:readonly string[]):BuildHelpRequestV1{
 const boundary=args.indexOf("--"),head=args.slice(0,boundary<0?undefined:boundary),positional:string[]=[];
 for(let index=0;index<head.length;index++){const argument=head[index]!;if(argument==="--lang"||argument==="--tool"){index++;continue;}if(!argument.startsWith("-"))positional.push(argument);}
 if(!positional.length)return {};
 const operation=catalog.operations.find(row=>row.name===positional[0]);
 if(!operation)throw Error(`Unknown command: ${positional[0]} (${catalog.operations.map(row=>row.name).join("|")})`);
 const subcommand=operation.subcommands?.find(row=>row.name===positional[1]);
 return subcommand?{operation:operation.name,subcommand:subcommand.name}:{operation:operation.name};
}

function spelling(flag:BuildHelpFlagV1):string{
 const suffix=({none:"",required:`=<${flag.valueName}>`,optional:`[=<${flag.valueName}>]`,separate:` <${flag.valueName}>`} as Record<string,string>)[flag.value]!,aliases=flag.aliases??[];
 return [...aliases.filter(alias=>!alias.startsWith("--")),flag.name+suffix,...aliases.filter(alias=>alias.startsWith("--"))].join(", ");
}

function table(rows:readonly (readonly [string,string])[]):string[]{
 const width=Math.min(columnLimit,Math.max(...rows.map(([left])=>left.length))),lines:string[]=[];
 for(const [left,right] of rows){if(left.length>width)lines.push(`  ${left}`,`  ${" ".repeat(width)}  ${right}`);else lines.push(`  ${left.padEnd(width)}  ${right}`);}
 return lines;
}

/** 🖨️ Renders the usage of the whole CLI, one operation or one subcommand from the catalogue in the chosen language. */
export function renderBuildHelpV1(catalog:BuildHelpCatalogV1,request:BuildHelpRequestV1,language:string):string{
 const label=(name:string)=>catalog.labels[name]![language]!,text=(value:BuildHelpTextV1)=>value[language]!,flag=(name:string)=>catalog.flags.find(row=>row.name===name)!;
 const global=catalog.flags.filter(row=>row.operations===undefined),lines:string[]=[];
 const globals=()=>["",`${label("globalOptions")}:`,...table(global.map(row=>[spelling(row),text(row.description)] as const))];
 const languageChoice=catalog.languages.join("|");
 if(request.operation===undefined){
  lines.push(`repo - ${text(catalog.labels.tagline!)}`,"",`${label("usage")}:`,`  ${root} <command> [${label("options").toLowerCase()}] [-- <arguments>]`,`  repo <command> --help [--lang=${languageChoice}]`,"",`${label("commands")}:`,...table(catalog.operations.map(row=>[row.synopsis,text(row.summary)] as const)));
  const scoped=catalog.flags.filter(row=>row.operations!==undefined);
  lines.push("",`${label("options")}:`,...table(scoped.map(row=>[spelling(row),`${text(row.description)} (${label("usedBy")}: ${row.operations!.join(", ")})`] as const)),...globals(),"",`${label("examples")}:`);
  for(const row of catalog.operations)lines.push(`  ${row.examples[0]!.command}`,`      ${text(row.examples[0]!.description)}`);
  return lines.join("\n");
 }
 const operation=catalog.operations.find(row=>row.name===request.operation)!,subcommand=operation.subcommands?.find(row=>row.name===request.subcommand);
 lines.push(`repo ${subcommand?.synopsis.split(/\s/u).slice(0,2).join(" ")??operation.name} - ${text((subcommand??operation).summary)}`,"",`${label("usage")}:`,`  ${root} ${(subcommand??operation).synopsis}${operation.flags.length?` [${label("options").toLowerCase()}]`:""}`);
 if(operation.subcommands&&!subcommand)lines.push("",`${label("subcommands")}:`,...table(operation.subcommands.map(row=>[row.synopsis,text(row.summary)] as const)));
 if(operation.flags.length)lines.push("",`${label("options")}:`,...table(operation.flags.map(name=>[spelling(flag(name)),text(flag(name).description)] as const)));
 lines.push(...globals(),"",`${label("examples")}:`);
 for(const example of operation.examples)lines.push(`  ${example.command}`,`      ${text(example.description)}`);
 return lines.join("\n");
}

/** 📖️ Renders the help for the CLI arguments as they were typed, using the locale of the environment. */
export function buildHelpForArgumentsV1(args:readonly string[],environment:Readonly<Record<string,string|undefined>>):string{
 const catalog=catalogue as BuildHelpCatalogV1;
 return renderBuildHelpV1(catalog,readBuildHelpRequestV1(catalog,args),resolveBuildHelpLanguageV1(catalog.languages,args,environment));
}
