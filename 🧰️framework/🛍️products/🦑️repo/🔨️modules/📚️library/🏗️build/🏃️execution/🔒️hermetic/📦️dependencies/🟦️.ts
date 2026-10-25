import {existsSync,lstatSync,readFileSync,readdirSync,readlinkSync,realpathSync} from "node:fs";
import {join,relative,sep} from "node:path";

/** 📦️ One `node_modules` directory the runner builds inside the isolated workspace: links to workspace packages, links to dependency-tree entries and the `.bin` shims of the selected entries. */
export interface HermeticModulesRowV1{path:string;workspace:Record<string,string>;tree:Record<string,string>;bin:Record<string,string>}
/** ⛓️ The declared dependencies of one hermetic action: the directory rows and the content-addressed tree of store entries `<store index>/<identity>` copied from the sorted `stores`. */
export interface HermeticDependenciesV1{rows:HermeticModulesRowV1[];ids:string[];stores:string[]}
/** 🔌️ The two filesystem reads of the shim scan, replaceable so every failure code is testable on any host. */
export interface HermeticShimPortV1{list(directory:string):string[];target(path:string):string}
const systemShims:HermeticShimPortV1={list:directory=>readdirSync(directory),target:path=>readlinkSync(path)},errorCode=(error:unknown)=>(error as NodeJS.ErrnoException)?.code;
/** 🔗️ Lists the `.bin` shims of one `node_modules` directory as name and relative link target. An absent `.bin` and an entry that is not a link (a regular file, as bun writes them on Windows) mean "no shim"; every other failure, such as a refused permission or an I/O error, is thrown, because dropping the shims silently would run the action without its tools. */
export function hermeticShimsV1(modules:string,port:HermeticShimPortV1=systemShims):[string,string][]{
 const bin=join(modules,".bin");let names:string[];
 try{names=port.list(bin).sort(order);}catch(error){if(errorCode(error)==="ENOENT"||errorCode(error)==="ENOTDIR")return [];throw error;}
 const found:[string,string][]=[];
 for(const name of names){try{found.push([name,port.target(join(bin,name))]);}catch(error){if(errorCode(error)!=="ENOENT"&&errorCode(error)!=="EINVAL")throw error;}}
 return found;
}

const order=(left:string,right:string)=>left<right?-1:left>right?1:0,inside=(from:string,path:string):string|undefined=>{const row=relative(from,path);return row===""||row===".."||row.startsWith(".."+sep)?undefined:row.split(sep).join("/");};
const marker=sep+"node_modules"+sep+".bun"+sep;
const locate=(real:string):{store:string;id:string;rest:string}|undefined=>{const at=real.indexOf(marker);if(at<0)return undefined;const rest=real.slice(at+marker.length),cut=rest.indexOf(sep);return cut<0?undefined:{store:real.slice(0,at+marker.length-1),id:rest.slice(0,cut),rest:rest.split(sep).join("/")};};
const entriesOf=(modules:string):[string,string][]=>{const found:[string,string][]=[];for(const name of readdirSync(modules).sort(order)){if([".bin",".cache",".bun"].includes(name))continue;const path=join(modules,name);if(name.startsWith("@")&&!lstatSync(path).isSymbolicLink()&&lstatSync(path).isDirectory())for(const child of readdirSync(path).sort(order))found.push([name+"/"+child,join(path,child)]);else found.push([name,path]);}return found;};

/** 📖️ Reads the dependency generations the checkout links and selects what one action needs: per directory the workspace packages it links whose directory has declared inputs and the named third-party packages, then the transitive closure of their store entries. `directories` are the workspace-relative directories whose own `node_modules` the action sees (the root is always one); `names` are extra packages named by the class and by `externalDependencies`, `packages` says whether third-party packages are selected at all or only workspace links; `declared` is the sorted list of declared input files. Returns `undefined` when the checkout has no installed generation or an entry leaves the generation layout, so the target stays out of the hermetic classes instead of reading the host. [Dependency layout](https://bun.sh/docs/install/isolated). */
export function hermeticDependenciesV1(options:{root:string;directories:readonly string[];names:readonly string[];packages:boolean;declared:readonly string[];check?:()=>void}):HermeticDependenciesV1|undefined{
 const{root,declared}=options,check=options.check??(()=>{}),checkout=realpathSync(root);
 if(!existsSync(join(root,"node_modules")))return undefined;
 const owned=(path:string)=>{const row=inside(checkout,path);return row!==undefined&&!row.startsWith(".cache/")?row:undefined;},has=(directory:string)=>{let low=0,high=declared.length;const prefix=directory===""?"":directory+"/";while(low<high){const middle=(low+high)>>>1;if(declared[middle]!<prefix)low=middle+1;else high=middle;}return directory===""||low<declared.length&&declared[low]!.startsWith(prefix);};
 const used=new Set<string>(),pending:{store:string;id:string}[]=[],seen=new Set<string>(),rows:{path:string;workspace:Record<string,string>;tree:Record<string,{store:string;rest:string}>;bin:Record<string,string>}[]=[];
 const wantedOf=(directory:string):string[]=>{try{const manifest=JSON.parse(readFileSync(join(root,directory,"package.json"),"utf8")) as Record<string,Record<string,string>|undefined>;return Object.entries({...manifest.dependencies,...manifest.devDependencies,...manifest.optionalDependencies}).filter(([,spec])=>!String(spec).startsWith("workspace:")).map(([name])=>name);}catch{return [];}};
 const shims=(modules:string)=>hermeticShimsV1(modules);
 const directories=["",...options.directories.filter(directory=>directory!=="")],visited=new Set<string>();
 for(let position=0;position<directories.length;position++){
  const directory=directories[position]!;if(visited.has(directory))continue;visited.add(directory);
  check();const modules=join(root,directory,"node_modules");if(!existsSync(modules))continue;
  const wanted=new Set(options.packages?directory===""?[...wantedOf(options.directories[0]??""),...options.names]:[...wantedOf(directory),...(directory===options.directories[0]?options.names:[])]:[]),row={path:(directory===""?"":directory+"/")+"node_modules",workspace:{} as Record<string,string>,tree:{} as Record<string,{store:string;rest:string}>,bin:{} as Record<string,string>};
  for(const[name,path]of entriesOf(modules)){
   let real:string;try{real=realpathSync(path);}catch{continue;}
   const workspace=owned(real);
   if(workspace!==undefined){if(has(workspace)){row.workspace[name]=workspace;if(!visited.has(workspace))directories.push(workspace);}continue;}
   if(!wanted.has(name))continue;
   const where=locate(real);if(!where)return undefined;
   row.tree[name]={store:where.store,rest:where.rest};if(!seen.has(where.store+"\0"+where.id)){seen.add(where.store+"\0"+where.id);pending.push({store:where.store,id:where.id});}
  }
  for(const[name,shim]of shims(modules)){const owner=shim.startsWith("../")?shim.slice(3).split("/").slice(0,shim.slice(3).startsWith("@")?2:1).join("/"):"";if(owner&&row.tree[owner]!==undefined)row.bin[name]=shim;}
  rows.push(row);
 }
 for(let position=0;position<pending.length;position++){
  check();const{store,id}=pending[position]!,modules=join(store,id,"node_modules");if(!existsSync(modules))continue;
  for(const[,path]of entriesOf(modules)){if(!lstatSync(path).isSymbolicLink())continue;const where=locate(realpathSync(path));if(!where)return undefined;if(where.store!==store)return undefined;if(!seen.has(store+"\0"+where.id)){seen.add(store+"\0"+where.id);pending.push({store,id:where.id});}}
 }
 for(const row of rows)for(const entry of Object.values(row.tree))used.add(entry.store);
 for(const entry of pending)used.add(entry.store);
 const stores=[...used].sort(order),index=new Map(stores.map((store,position)=>[store,position]));
 return{stores,ids:pending.map(entry=>index.get(entry.store)+"/"+entry.id).sort(order),rows:rows.map(row=>({path:row.path,workspace:row.workspace,tree:Object.fromEntries(Object.entries(row.tree).map(([name,entry])=>[name,index.get(entry.store)+"/"+entry.rest])),bin:row.bin}))};
}
