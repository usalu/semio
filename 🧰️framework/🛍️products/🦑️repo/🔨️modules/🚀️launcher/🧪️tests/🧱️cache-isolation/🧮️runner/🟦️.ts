import{readFileSync}from"node:fs";
import{join,resolve}from"node:path";
const root=resolve(process.argv[2]),names=["main.ts","big.ts","dep.ts"],images=new Map<string,Buffer>(names.map(name=>[join(root,name),readFileSync(join(root,name))]));
Bun.plugin({name:"captured",setup(build){
 build.onResolve({filter:/.*/,namespace:"semio-source"},item=>{const name=item.path.replace(/^semio-source:/,"");if(!images.has(name))throw Error("Uncaptured require namespace image refused");return{path:name,namespace:"semio-source"};});
 build.onResolve({filter:/.*/},item=>{const importer=item.importer?.replace(/^[/]?semio-source:/,""),candidate=item.path.startsWith("/")?item.path:join(importer??root,"..",item.path);if(!images.has(candidate))throw Error("Uncaptured import refused: "+candidate);return{path:candidate,namespace:"semio-source"};});
 build.onLoad({filter:/.*/,namespace:"semio-source"},item=>{const name=resolve(item.path),bytes=images.get(name);if(!bytes)throw Error("Uncaptured implementation module refused: "+name);return{contents:new Bun.Transpiler({loader:"ts",target:"bun"}).transformSync(bytes.toString("utf8")),loader:"js"};});
}});
console.log(JSON.stringify({out:(await import(join(root,"main.ts"))).out}));
