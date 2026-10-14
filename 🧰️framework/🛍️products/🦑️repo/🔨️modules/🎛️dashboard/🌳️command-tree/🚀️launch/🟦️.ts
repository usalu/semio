/** 🚀️ Projects current dashboard owner declarations into scoped VSCode launch controls. */
export type LaunchParameter={id:string;kind:"text"|"choice"|"flag";default?:string|boolean;required?:boolean;values?:readonly {id:string}[]};
export type LaunchTool={id:string;verb?:string;parameters?:readonly LaunchParameter[]};
export type LaunchCommand=LaunchTool&{id:string};
export type LaunchDocument={version:string;configurations:Record<string,any>[];inputs?:Record<string,any>[];[key:string]:unknown};
const slug=/^[a-z0-9]+(?:[._-][a-z0-9]+)*$/;
const ticketPattern=/^\d{2}\/\d{2}\/\d{2}\/[A-Z0-9]+(?:-[A-Z0-9]+)*$/;
const identifier=/^ticket:\d{2}\/\d{2}\/\d{2}\/[A-Z0-9]+(?:-[A-Z0-9]+)*\/[a-z0-9]+(?:[._-][a-z0-9]+)*$/;
const verbs:Record<string,readonly [string,string]>={test:["Test","Prüfen"],gate:["Gate","Freigabe"],check:["Check","Kontrollieren"],generate:["Generate","Erzeugen"],clean:["Clean","Bereinigen"],run:["Run","Ausführen"],serve:["Serve","Bereitstellen"],dev:["Develop","Entwickeln"],task:["Task","Aufgabe"]};

/** 🏗️ Publishes only neutral build routes, including nested serialized process capabilities. */
export function normalizeDashboardLaunchBuildV1<T>(document:T):T{
 const word=(value:string)=>value.replace(/^--skip(?:-nx-cache|NxCache)$/,"--skip-cache").replace(/^--excludeTaskDependencies(?==|$)/,"--exclude-dependencies").replace(/^--outputStyle(?==|$)/,"--output");
 const executable=(value:unknown)=>typeof value==="string"&&(value==="nx"||/(?:^|[/\\])node_modules[/\\]nx[/\\](?:dist[/\\])?bin[/\\]nx\.js$/.test(value));
 const command=(value:string)=>value.replace(/\bbun\s+(?:["'][^"']*⚡️caching\/🚀️bootstrap\/📜️script\.ts["']|[^\s"']*⚡️caching\/🚀️bootstrap\/📜️script\.ts)\s+nx(?=\s|$)/g,"bun repo").replace(/\bbun\s+(?:(?:run|x)\s+)?(?:nx|["'][^"']*node_modules\/nx\/(?:dist\/)?bin\/nx\.js["']|[^\s"']*node_modules\/nx\/(?:dist\/)?bin\/nx\.js)(?=\s|$)/g,"bun repo").replace(/--skip(?:-nx-cache|NxCache)\b/g,"--skip-cache").replace(/--excludeTaskDependencies\b/g,"--exclude-dependencies").replace(/--outputStyle(?==|\s|$)/g,"--output").replace(/\bNX_WORKSPACE_ROOT(?:_PATH)?=/g,"SEMIO_WORKSPACE_ROOT=").replace(/\bNX_[A-Z_]+=(?:"[^"]*"|'[^']*'|[^\s]+)\s*/g,"");
 const visit=(value:unknown,key=""):unknown=>{
  if(Array.isArray(value)){const result=value.map(item=>visit(item,key==="launches"?"arguments":""));if(["args","arguments","selection","command"].includes(key)){if(executable(result[0]))result[0]="repo";else if(result[0]==="bun"&&["run","x"].includes(String(result[1]))&&executable(result[2]))result.splice(1,2,"repo");else if(result[0]==="bun"&&executable(result[1]))result[1]="repo";}return result;}
  if(value&&typeof value==="object")return Object.fromEntries(Object.entries(value).flatMap(([name,item])=>name.startsWith("NX_")||name.startsWith("SEMIO_NX_")?[]:[[name,visit(item,name)]]));
  if(typeof value!=="string")return value;
  if(key==="command")return command(value);
  if(/^[\[{]/.test(value)){try{const parsed=JSON.parse(value),normalized=visit(parsed);return JSON.stringify(normalized)===JSON.stringify(parsed)?value:JSON.stringify(normalized);}catch{}}
  return word(value);
 };
 return visit(document) as T;
}

/** 🪪️ Encodes each parameter independently into a collision-free environment name. */
export function launchParameterEnvironment(id:string):string{
 if(!slug.test(id))throw new Error(`Invalid dashboard parameter ${id}`);
 return "SEMIO_DASHBOARD_INPUT_"+Array.from(id,c=>c.charCodeAt(0).toString(16)).join("_");
}

/** 🎛️ Passes raw editor inputs as native registry arguments, without shell interpolation. */
export function launchParameterArguments(tool:LaunchTool,environment:Readonly<Record<string,string|undefined>>):string[]{
 return (tool.parameters??[]).flatMap(parameter=>{const value=environment[launchParameterEnvironment(parameter.id)];return value===undefined?[]:["--param",`${parameter.id}=${value}`];});
}

/** 🎫️ Captures ticket-owned tools and compounds using the current registry identifier grammar. */
export function ticketLaunchCommands(ticket:string,document:{tools?:readonly LaunchTool[];compounds?:readonly LaunchTool[]}):LaunchCommand[]{
 if(!ticketPattern.test(ticket))throw new Error(`Invalid dashboard ticket ${ticket}`);
 const seen=new Set<string>();
 return [...document.tools??[],...document.compounds??[]].map(tool=>{
  if(!slug.test(tool.id)||seen.has(tool.id))throw new Error(`Invalid or duplicate dashboard tool ${tool.id}`);
  seen.add(tool.id);
  const parameters=new Set<string>();
  for(const parameter of tool.parameters??[]){launchParameterEnvironment(parameter.id);if(parameters.has(parameter.id)||!["text","choice","flag"].includes(parameter.kind))throw new Error(`Invalid dashboard parameter ${parameter.id}`);parameters.add(parameter.id);}
  return {...tool,id:`ticket:${ticket}/${tool.id}`};
 });
}

/** 🧩️ Replaces only one owner's projected entries; unrelated controls and inputs retain their order. */
export function projectDashboardLaunch(current:LaunchDocument,commands:readonly LaunchCommand[],ticket:string):LaunchDocument{
 current=normalizeDashboardLaunchBuildV1(current);
 if(!ticketPattern.test(ticket)||current.version!=="0.2.0"||!Array.isArray(current.configurations))throw new Error("Invalid dashboard launch document");
 const prefix=`semio.dashboard.${ticket}.`,owned=(row:Record<string,any>)=>typeof row.presentation?.group==="string"&&row.presentation.group.startsWith(prefix);
 const retained=current.configurations.filter(row=>!owned(row)),position=current.configurations.findIndex(owned),configurations:Record<string,any>[]=[],inputPrefix=`semio-dashboard-${ticket.replaceAll("/","-")}-`;
 const inputs=(current.inputs??[]).filter(row=>!row.id.startsWith(inputPrefix));
 const ordered=[...commands].sort((a,b)=>(a.verb??"task").localeCompare(b.verb??"task","en")||a.id.localeCompare(b.id,"en"));
 const seen=new Set<string>();
 for(const command of ordered){if(!identifier.test(command.id)||!command.id.startsWith(`ticket:${ticket}/`)||seen.has(command.id))throw new Error(`Invalid or duplicate dashboard command ${command.id}`);seen.add(command.id);}
 for(const [language,index] of [["en",0],["de",1]] as const){
  for(const [order,command] of ordered.entries()){
   const verb=command.verb??"task",env:Record<string,string>={};
   for(const parameter of command.parameters??[]){
    const id=`${inputPrefix}${command.id.split("/").at(-1)}-${parameter.id}-${language}`;
    env[launchParameterEnvironment(parameter.id)]=`\${input:${id}}`;
    const input:Record<string,unknown>={id,type:parameter.kind==="text"?"promptString":"pickString",description:`${parameter.id} [${language.toUpperCase()}]`};
    if(parameter.kind!=="text")input.options=parameter.kind==="flag"?["false","true"]:(parameter.values??[]).map(value=>value.id);
    if(parameter.default!==undefined)input.default=String(parameter.default);
    inputs.push(input);
   }
   configurations.push({name:`${(verbs[verb]??verbs.task!)[index]} · ${command.id} [${language.toUpperCase()}]`,type:"node-terminal",request:"launch",command:`bun repo run @semio-tech/repo-dashboard-rs:launch -- run ${command.id}`,cwd:"${workspaceFolder}",...(Object.keys(env).length?{env}:{}),presentation:{group:prefix+(index+1)+"_"+language+"."+verb,order}});
  }
 }
 return {...current,configurations:[...retained.slice(0,position<0?retained.length:position),...configurations,...retained.slice(position<0?retained.length:position)],inputs};
}
