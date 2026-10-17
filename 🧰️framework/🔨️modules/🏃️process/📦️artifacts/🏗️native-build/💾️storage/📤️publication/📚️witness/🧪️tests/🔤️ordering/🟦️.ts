/** 🔤️ Ordering: 1 test covering publication witness name pages preserve Python UTF8 order with bounded cached encodings. */
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import ts from "typescript";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json";
import schema from "./🧬️schema.json";

test("publication witness name pages preserve Python UTF8 order with bounded cached encodings",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const text=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("witness.ts",text,ts.ScriptTarget.Latest,true),enumerators:ts.ArrowFunction[]=[],comparators:ts.VariableDeclaration[]=[];
 for(const statement of tree.statements)if(ts.isVariableStatement(statement))for(const node of statement.declarationList.declarations)if(ts.isIdentifier(node.name)&&node.name.text==="compare")comparators.push(node);
 const scan=(node:ts.Node):void=>{if(ts.isVariableDeclaration(node)&&ts.isIdentifier(node.name)&&node.name.text==="enumerate"&&node.initializer&&ts.isArrowFunction(node.initializer))enumerators.push(node.initializer);ts.forEachChild(node,scan);};scan(tree);
 expect(enumerators.length).toBe(1);expect(comparators.length).toBe(1);
 const body=enumerators[0]!.body;if(!ts.isBlock(body))throw Error("Actual directory enumerator body required");
 const setup=body.statements.slice(0,body.statements.findIndex(ts.isWhileStatement)).filter(ts.isVariableStatement).map(node=>node.getText(tree)).join("\n"),loops:ts.ForStatement[]=[];
 const find=(node:ts.Node):void=>{if(ts.isForStatement(node)&&ts.isBlock(node.statement)&&node.statement.statements.some(child=>ts.isIfStatement(child)&&child.expression.getText(tree).includes("paths.join(current,entry.name).length>policy.maximumPathUnits")))loops.push(node);ts.forEachChild(node,find);};find(body);
 expect(loops.length).toBe(1);const loop=loops[0]!.statement;if(!ts.isBlock(loop))throw Error("Actual bounded entry loop required");
 const guard=loop.statements.findIndex(node=>ts.isIfStatement(node)&&node.expression.getText(tree).includes("paths.join(current,entry.name).length>policy.maximumPathUnits")),selection=loop.statements.slice(guard+1).map(node=>node.getText(tree)).join("\n"),code=ts.transpileModule("const "+comparators[0]!.getText(tree)+";function page(entries,cursor){"+setup+"for(const entry of entries){"+selection+"}return {names:selected,keys:typeof selectedBytes==='undefined'?[]:selectedBytes,cursor:typeof cursorBytes==='undefined'?undefined:cursorBytes};}",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 const oracle=["import sys,json,bisect","x=json.loads(sys.argv[1]);w=x['protocol']['window']","def key(s):return s.encode('utf-16-le','surrogatepass').decode('utf-16-le','replace').encode('utf-8')","def run(names):"," cursor=None;result=[]"," while True:","  selected=[];keys=[]","  for name in names:","   b=key(name)","   if cursor is not None and b<=key(cursor):continue","   at=bisect.bisect_left(keys,b)","   if at<len(selected)and selected[at]==name:return None","   if at<w:","    selected.insert(at,name);keys.insert(at,b)","    if len(selected)>w:selected.pop();keys.pop()","  if not selected:return result","  result.extend(selected);cursor=selected[-1]","print(json.dumps({r['id']:run(r['names'])for r in x['cases']},ensure_ascii=True))"].join("\n"),python=spawnSync(process.platform==="win32"?"python":"python3",["-c",oracle,JSON.stringify(fixture)],{encoding:"utf8",timeout:10000,maxBuffer:131072});
 expect(python.status,python.stderr).toBe(0);const independent=JSON.parse(python.stdout)as Record<string,string[]|null>,db=new Database(":memory:");
 try{for(const row of [...fixture.cases.map(row=>({...row,width:0})),...fixture.protocol.widths.map(width=>({id:"width-"+width,names:Array.from({length:width},(_,index)=>"f"+String(index).padStart(5,"0")),expected:"complete",width}))]){
  let encodings=0,comparisons=0,pages=0,cursor:string|undefined;const names:string[]=[],ports={from:(value:string)=>{encodings++;return Buffer.from(value);},compare:(a:Uint8Array,b:Uint8Array)=>{comparisons++;return Buffer.compare(a,b);}},page=new Function("Buffer","policy",code+";return page;")(ports,{nameWindow:fixture.protocol.window})as(entries:{name:string}[],cursor:string|undefined)=>{names:string[];keys:(Buffer|undefined)[];cursor:Buffer|undefined};let fault:unknown;
  try{for(;;){const actual=page(row.names.map(name=>({name})),cursor);pages++;expect(actual.names.length).toBeLessThanOrEqual(fixture.protocol.window);expect(actual.keys.length).toBeLessThanOrEqual(actual.names.length);expect(actual.keys.reduce((total,key)=>total+(key?.byteLength??0),0)).toBeLessThanOrEqual(fixture.protocol.maximumSelectedKeyBytes);expect(actual.cursor?.byteLength??0).toBeLessThanOrEqual(fixture.protocol.maximumUtf8BytesPerName);if(!actual.names.length)break;names.push(...actual.names);cursor=actual.names.at(-1);}}catch(error){fault=error;}
  if(row.expected==="duplicate"){expect(fault).toBeInstanceOf(Error);expect((fault as Error).message).toBe("Duplicate publication entry name");expect(independent[row.id]).toBeNull();continue;}
  expect(fault).toBeUndefined();expect(names).toEqual(row.width?row.names:independent[row.id]!);
  if(row.width){const count=row.width,windows=Math.ceil(count/fixture.protocol.window),expected=db.query("with recursive pages(k,m)as(select 0,?1 union all select k+1,max(0,m-64)from pages where k<?2)select ?2+1 pages,?1*?2+sum(case when m<=64 then m*(m-1)/2 else 2016+64*(m-64)end) comparisons,?1*(?2+1)+?2 encodings from pages").get(count,windows)as{pages:number;comparisons:number;encodings:number};expect(pages).toBe(expected.pages);expect(comparisons).toBe(expected.comparisons);expect(encodings).toBeLessThanOrEqual(expected.encodings);}
 }
 }finally{db.close();}
},15000);
