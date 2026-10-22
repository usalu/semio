import {buildBackendAlternationV1} from "../../🔌️backends/🟦️.ts";
import catalogue from "../🔣️catalog.json" with { type: "json" };
import { resolveLanguageV1, type LanguageEnvironmentV1, type OsLanguageListV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts";
import { normalizeBuildOptionsV1 } from "../🔤️options/🟦️.ts";

/** 🌐️ A text in every language the catalogue declares. */
export type BuildHelpTextV1=Readonly<Record<string,string>>;

/** 🚩️ One documented flag: its spelling, value shape, the operations that read it and its description. */
export type BuildHelpFlagV1=Readonly<{name:string;aliases?:readonly string[];value:string;valueName?:string;operations?:readonly string[];description:BuildHelpTextV1}>;

export type BuildHelpExampleV1=Readonly<{command:string;description:BuildHelpTextV1}>;

/** 🧩️ One subcommand of an operation; its own flags and examples replace those of the operation when it declares them. */
export type BuildHelpSubcommandV1=Readonly<{name:string;synopsis:string;summary:BuildHelpTextV1;flags?:readonly string[];examples?:readonly BuildHelpExampleV1[]}>;

/** 🧭️ One operation as the help consumes it. */
export type BuildHelpOperationV1=Readonly<{name:string;synopsis:string;summary:BuildHelpTextV1;subcommands?:readonly BuildHelpSubcommandV1[];flags:readonly string[];examples:readonly BuildHelpExampleV1[];notes?:readonly BuildHelpTextV1[]}>;

/** 📚️ The part of the neutral operations catalogue that the help renders. */
export type BuildHelpCatalogV1=Readonly<{languages:readonly string[];labels:Readonly<Record<string,BuildHelpTextV1>>;flags:readonly BuildHelpFlagV1[];operations:readonly BuildHelpOperationV1[]}>;

/** 🎯️ Names what a help request asks for: the whole CLI, one operation or one subcommand. */
export type BuildHelpRequestV1=Readonly<{operation?:string;subcommand?:string}>;

const root="repo [--tool={backends}]",columnLimit=46;
const expandBackends=(value:string)=>value.replaceAll("{backends}",buildBackendAlternationV1());

/** 🗣️ Picks the help language by the shared rule (`--lang`, `SEMIO_LANGUAGE`, `LANGUAGE`, the locale variables, the operating system language list) and never assumes one: when nothing names an offered language the bilingual refusal is thrown. */
export function resolveBuildHelpLanguageV1(languages:readonly string[],args:readonly string[],environment:LanguageEnvironmentV1,osLanguages?:OsLanguageListV1):string{
 return resolveLanguageV1(languages,args,environment,osLanguages).language;
}

/** 🚪️ Chooses the language of the neutral operations catalogue for a CLI invocation. */
export function resolveBuildCliLanguageV1(args:readonly string[],environment:LanguageEnvironmentV1,osLanguages?:OsLanguageListV1):string{
 return resolveBuildHelpLanguageV1((catalogue as BuildHelpCatalogV1).languages,args,environment,osLanguages);
}

/** 🔎️ Reads which command and subcommand a help request names; no command means the whole CLI. */
export function readBuildHelpRequestV1(catalog:BuildHelpCatalogV1,args:readonly string[]):BuildHelpRequestV1{
 const boundary=args.indexOf("--"),positional=normalizeBuildOptionsV1(args.slice(0,boundary<0?undefined:boundary),catalog).filter(argument=>!argument.startsWith("-"));
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

/** 🧷️ Keeps the examples that start with the words of a subcommand, or all of them when none does. */
function scopedExamples(examples:readonly BuildHelpExampleV1[],synopsis:string):readonly BuildHelpExampleV1[]{
 const words:string[]=[];for(const word of synopsis.split(/\s+/u)){if(!/^[a-z][a-z-]*$/u.test(word))break;words.push(word);}
 const own=examples.filter(row=>row.command.startsWith(`repo ${words.join(" ")}`));
 return own.length?own:examples;
}

/** 🖨️ Renders the usage of the whole CLI, one operation or one subcommand from the catalogue in the chosen language. */
export function renderBuildHelpV1(catalogSource:BuildHelpCatalogV1,request:BuildHelpRequestV1,language:string):string{
 const catalog=JSON.parse(expandBackends(JSON.stringify(catalogSource))) as BuildHelpCatalogV1;
 const label=(name:string)=>catalog.labels[name]![language]!,text=(value:BuildHelpTextV1)=>value[language]!,flag=(name:string)=>catalog.flags.find(row=>row.name===name)!;
 const global=catalog.flags.filter(row=>row.operations===undefined),lines:string[]=[];
 const globals=()=>["",`${label("globalOptions")}:`,...table(global.map(row=>[spelling(row),text(row.description)] as const))];
 const languageChoice=catalog.languages.join("|");
 if(request.operation===undefined){
  lines.push(`repo - ${text(catalog.labels.tagline!)}`,"",`${label("usage")}:`,`  ${root} <command> [${label("options").toLowerCase()}] [-- <arguments>]`,`  repo <command> --help [--lang=${languageChoice}]`,"",`${label("commands")}:`,...table(catalog.operations.map(row=>[row.synopsis,text(row.summary)] as const)));
  const scoped=catalog.flags.filter(row=>row.operations!==undefined);
  lines.push("",`${label("options")}:`,...table(scoped.map(row=>[spelling(row),`${text(row.description)} (${label("usedBy")}: ${row.operations!.join(", ")})`] as const)),...globals(),"",`${label("examples")}:`);
  for(const row of catalog.operations)lines.push(`  ${row.examples[0]!.command}`,`      ${text(row.examples[0]!.description)}`);
  return expandBackends(lines.join("\n"));
 }
 const operation=catalog.operations.find(row=>row.name===request.operation)!,subcommand=operation.subcommands?.find(row=>row.name===request.subcommand),flagNames=subcommand?.flags??operation.flags,examples=subcommand?(subcommand.examples??scopedExamples(operation.examples,subcommand.synopsis)):operation.examples;
 lines.push(`repo ${subcommand?.synopsis.split(/\s/u).slice(0,2).join(" ")??operation.name} - ${text((subcommand??operation).summary)}`,"",`${label("usage")}:`,`  ${root} ${(subcommand??operation).synopsis}${flagNames.length?` [${label("options").toLowerCase()}]`:""}`);
 if(operation.subcommands&&!subcommand)lines.push("",`${label("subcommands")}:`,...table(operation.subcommands.map(row=>[row.synopsis,text(row.summary)] as const)));
 if(flagNames.length)lines.push("",`${label("options")}:`,...table(flagNames.map(name=>[spelling(flag(name)),text(flag(name).description)] as const)));
 lines.push(...globals(),"",`${label("examples")}:`);
 for(const example of examples)lines.push(`  ${example.command}`,`      ${text(example.description)}`);
 if(operation.notes?.length)lines.push("",`${label("notes")}:`,...operation.notes.map(note=>`  - ${text(note)}`));
 return expandBackends(lines.join("\n"));
}

/** 📖️ Renders the help for the CLI arguments as they were typed, using the locale of the environment. */
export function buildHelpForArgumentsV1(args:readonly string[],environment:Readonly<Record<string,string|undefined>>):string{
 const catalog=catalogue as BuildHelpCatalogV1;
 return renderBuildHelpV1(catalog,readBuildHelpRequestV1(catalog,args),resolveBuildHelpLanguageV1(catalog.languages,args,environment));
}
