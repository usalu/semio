import{readFileSync,readdirSync,existsSync}from"node:fs";
import{dirname,join,resolve}from"node:path";
import{fileURLToPath,pathToFileURL}from"node:url";
import ts from"typescript";

export interface ParserOwnerHarnessV1{
 readonly files:readonly string[];
 readonly homes:ReadonlyMap<string,readonly{readonly file:string;readonly parse:(value:unknown)=>unknown}[]>;
 diagnostics():readonly string[];
 publication(document:string,name:string):{readonly present:boolean;readonly origins:readonly string[]};
}

/** 🗃️ Every physical TypeScript input of the owner, including facets and excluding test/package collections. */
function compilerFiles(root:string):string[]{return readdirSync(root,{withFileTypes:true}).filter(entry=>entry.name!=="🧪️tests"&&entry.name!=="📦️packages").flatMap(entry=>entry.isDirectory()?compilerFiles(join(root,entry.name)):entry.name.endsWith(".ts")?[join(root,entry.name)]:[])}

/** 🧪️ Actual parser implementations, complete strict compilation and original public declarations of one normative document's owner. */
export async function parserOwnerHarnessV1(rootUrl:URL):Promise<ParserOwnerHarnessV1>{
 const root=fileURLToPath(rootUrl),files=compilerFiles(root),homes=new Map<string,{file:string;parse:(value:unknown)=>unknown}[]>();
 async function discover(directory:string):Promise<void>{
  if(directory!==root&&existsSync(join(directory,"🔣️.json"))&&existsSync(join(directory,"🟦️.ts")))return;
  for(const entry of readdirSync(directory,{withFileTypes:true})){
   if(entry.name==="🧪️tests"||entry.name==="📦️packages")continue;
   const file=join(directory,entry.name);
   if(entry.isDirectory()){await discover(file);continue}
   if(!entry.name.endsWith(".ts"))continue;
   const source=ts.createSourceFile(file,readFileSync(file,"utf8"),ts.ScriptTarget.Latest,true),names=source.statements.filter(ts.isFunctionDeclaration).filter(row=>row.body&&row.modifiers?.some(modifier=>modifier.kind===ts.SyntaxKind.ExportKeyword)&&row.name&&/^parse[A-Z]/u.test(row.name.text)).map(row=>row.name!.text);
   if(!names.length)continue;
   const module:Record<string,(value:unknown)=>unknown>=await import(pathToFileURL(file).href);
   for(const name of names){if(typeof module[name]!=="function")throw Error("Actual parser implementation required: "+file+"#"+name);homes.set(name,[...(homes.get(name)??[]),{file,parse:module[name]!}])}
  }
 }
 await discover(root);
 let compiled:ts.Program|undefined;
 function program():ts.Program{if(compiled)return compiled;const file=join(root,"🟦️.ts"),config=ts.findConfigFile(dirname(file),ts.sys.fileExists),loaded=config?ts.readConfigFile(config,ts.sys.readFile):{config:{}},parsed=ts.parseJsonConfigFileContent(loaded.config,ts.sys,config?dirname(config):dirname(file));return compiled=ts.createProgram(files,{...parsed.options,types:parsed.options.types??[],strict:true,noEmit:true,skipLibCheck:true})}
 return{files,homes,diagnostics:()=>ts.getPreEmitDiagnostics(program()).map(row=>(row.file?.fileName??"")+":"+(row.file&&row.start!==undefined?row.file.getLineAndCharacterOfPosition(row.start).line+1:0)+": "+ts.flattenDiagnosticMessageText(row.messageText,"\n")),publication:(document,name)=>{const compiled=program(),checker=compiled.getTypeChecker(),source=compiled.getSourceFile(join(root,document)),module=source&&checker.getSymbolAtLocation(source),symbol=module&&checker.getExportsOfModule(module).find(entry=>entry.name===name);if(!symbol)return{present:false,origins:[]};const target=symbol.flags&ts.SymbolFlags.Alias?checker.getAliasedSymbol(symbol):symbol;return{present:true,origins:[...new Set(target.declarations?.map(entry=>resolve(entry.getSourceFile().fileName))??[])]}}};
}
