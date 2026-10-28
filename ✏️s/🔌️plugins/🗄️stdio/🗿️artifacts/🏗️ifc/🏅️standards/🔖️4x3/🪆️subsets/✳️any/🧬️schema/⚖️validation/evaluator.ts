import type {ExpressControl,ExpressExpression as Expr,ExpressProgram,ExpressStatement as Stmt,ExpressVariable} from './ast.ts';
import {ifc4x3Catalog} from '../📚️catalog/🟦️.ts';
import {captureExpressControlScope,ExpressEvaluationError} from './control.ts';
import {ExpressReadScope} from './read-scope.ts';
export {ExpressEvaluationError} from './control.ts';
/** 🎲 Owned EXPRESS values retain exact integers and entity identity. */
export interface ExpressValue {kind:'unknown'|'logical'|'integer'|'real'|'string'|'binary'|'enum'|'aggregate'|'entity';value?:bigint|number|string|ExpressValue[];types?:string[];aggregate?:'LIST'|'SET'|'BAG'|'ARRAY';lower?:bigint;identity?:bigint|symbol;fields?:Map<string,ExpressValue>;groups?:Map<string,Map<string,ExpressValue>>}
export type ExpressScope=Map<string,ExpressValue>;
const unknown:ExpressValue={kind:'unknown'};
const logical=(value:string):ExpressValue=>({kind:'logical',value});
const integer=(value:bigint):ExpressValue=>({kind:'integer',value});
export const expressAggregate=(value:ExpressValue[],aggregate:ExpressValue['aggregate']='LIST',lower=1n):ExpressValue=>({kind:'aggregate',value,aggregate,lower});
const truth=(v:ExpressValue)=>v.kind==='logical'?v.value as string:'unknown';
const items=(v:ExpressValue)=>v.kind==='aggregate'?v.value as ExpressValue[]:[];
const missing=(v:ExpressValue)=>v.kind==='unknown';
const numeric=(v:ExpressValue)=>v.kind==='integer'||v.kind==='real';
const bool=(v:boolean)=>logical(v?'true':'false');
const enumerationNames=new Set(Object.values(ifc4x3Catalog.types).flatMap(type=>type.kind==='enum'?type.members:[]));
/** ⚖ Scalar comparison preserves entity identity and exact integral REAL equality. */
function scalarEqual(a:ExpressValue,b:ExpressValue):ExpressValue{
 if(missing(a)||missing(b))return logical('unknown');
 if(numeric(a)&&numeric(b))return bool(a.value===b.value||a.kind!==b.kind&&Number(a.value)===Number(b.value)&&Number.isInteger(Number(a.kind==='real'?a.value:b.value))&&BigInt(Number(a.kind==='real'?a.value:b.value))===(a.kind==='integer'?a.value:b.value));
 if(a.kind!==b.kind)return bool(false);if(a.kind==='entity')return bool(a.identity===b.identity);return bool(a.value===b.value);
}/** 🧠 Asynchronous bounded interpreter with schema-specific resolution hooks. */
export class ExpressEvaluator {
 #work=0;#depth=0;readonly work!:number;readonly depth!:number;readonly declarations=new WeakMap<ExpressScope,Map<string,string>>();readonly functions=new Map<string,ExpressProgram['functions'][number]>();
 private selectParents?:Map<string,string[]>;
 readonly #readScopes=new WeakMap<ExpressScope,ExpressReadScope>();readonly #readValues=new WeakMap<ExpressValue,ExpressReadScope>();
 readonly control!:Readonly<ExpressControl>;readonly #controlScope:ReturnType<typeof captureExpressControlScope>;
 constructor(readonly program?:ExpressProgram,control:ExpressControl={},readonly resolveAttribute?:(entity:ExpressValue,name:string,qualifier?:string,scope?:ExpressScope)=>Promise<ExpressValue>,readonly usedIn?:(entity:ExpressValue,role:string)=>Promise<ExpressValue>){this.#controlScope=captureExpressControlScope(control);Object.defineProperties(this,{work:{get:()=>this.#work,enumerable:true,configurable:false},depth:{get:()=>this.#depth,enumerable:true,configurable:false},control:{value:this.#controlScope.control,enumerable:true,writable:false,configurable:false}});for(const fn of program?.functions??[])this.functions.set(fn.name.toUpperCase(),fn);}
 /** 🪢 Nested procedural and derived scopes inherit one explicit read authority. */
 inheritScope(target:ExpressScope,parent?:ExpressScope,value?:ExpressValue):void{const frame=parent&&this.#readScopes.get(parent)||value&&this.#readValues.get(value);if(frame)this.#readScopes.set(target,frame);}
 proceduralScope(parent?:ExpressScope):ExpressScope{const scope=new Map<string,ExpressValue>();this.#readScopes.set(scope,parent&&this.#readScopes.get(parent)||new ExpressReadScope(()=>this.tick()));return scope;}
 private async queryScope(parent:ExpressScope):Promise<ExpressScope>{const scope=this.proceduralScope(parent);for(const [name,value] of parent){await this.tick();scope.set(name,value);}return scope;}
 private async rootScope(scope:ExpressScope):Promise<ExpressScope>{return this.#readScopes.has(scope)?scope:this.queryScope(scope);}
 async readValue(value:ExpressValue,scope?:ExpressScope,anchor?:ExpressValue):Promise<ExpressValue>{const frame=scope&&this.#readScopes.get(scope)||anchor&&this.#readValues.get(anchor);if(!frame)return value;const result=await frame.read(value);if(result.kind==='entity'||result.kind==='aggregate')this.#readValues.set(result,frame);return result;}
 typedView(value:ExpressValue,types:string[],scope?:ExpressScope):ExpressValue{const view={...value,types},frame=scope&&this.#readScopes.get(scope)||this.#readValues.get(value);if(frame?.values.get(value)===value){frame.values.set(view,view);this.#readValues.set(view,frame);}return view;}
 async tick(offset?:number):Promise<void>{this.#controlScope.check(offset);if(++this.#work>(this.#controlScope.control.maximumWork??10_000_000))throw new ExpressEvaluationError('work-budget','EXPRESS evaluation work budget exceeded',offset);if(this.#work%(this.#controlScope.control.batchSize??2048)===0)await this.#controlScope.checkpoint({phase:'evaluate',completed:this.#work,total:this.#controlScope.control.maximumWork??10_000_000},offset);}
 async evaluate(e:Expr,scope:ExpressScope=new Map()):Promise<ExpressValue>{await this.tick(e.offset);const root=!this.#readScopes.has(scope);scope=await this.rootScope(scope);if(++this.#depth>(this.#controlScope.control.maximumDepth??256)){this.#depth--;throw new ExpressEvaluationError('depth','EXPRESS evaluation depth exceeded',e.offset);}try{let result=await this.readValue(await this.expression(e,scope),scope);if(root)result=await this.#readScopes.get(scope)!.publish(result);this.#controlScope.check(e.offset);return result;}finally{this.#depth--;}}
 private async expression(e:Expr,scope:ExpressScope):Promise<ExpressValue>{const operands=e.operands??[],get=(i:number)=>this.evaluate(operands[i]!,scope);switch(e.kind){
 case 'unknown':return unknown;case 'integer':return integer(BigInt(e.text!));case 'real':{const value=Number(e.text);if(!Number.isFinite(value))throw new ExpressEvaluationError('numeric-overflow','REAL literal exceeds finite evaluation precision',e.offset);return{kind:'real',value};}case 'string':return{kind:'string',value:e.text!};case 'logical':return logical(e.text!);
 case 'identifier':{const name=e.name!,bound=scope.get(name);if(bound)return bound;const self=scope.get('SELF');if(self?.kind==='entity'){const value=await this.attribute(await this.readValue(self,scope),name,scope);if(!missing(value)||self.fields?.has(name)||this.program?.derived.some(rule=>rule.label===name&&(self.types??[]).includes(rule.owner))||this.program?.inverse.some(rule=>rule.name===name&&(self.types??[]).includes(rule.owner)))return value;}if(name==='PI')return{kind:'real',value:Math.PI};if(ifc4x3Catalog.entities[name]||ifc4x3Catalog.types[name]||enumerationNames.has(name))return{kind:'enum',value:name};throw new ExpressEvaluationError('unbound',`Unbound EXPRESS identifier ${name}`,e.offset);}
 case 'aggregate':{const result:ExpressValue[]=[];for(const entry of e.entries??[]){const value=await this.evaluate(entry.value,scope),repeat=entry.repeat?await this.evaluate(entry.repeat,scope):integer(1n);if(missing(repeat))return unknown;if(repeat.kind!=='integer'||(repeat.value as bigint)<0n)throw new ExpressEvaluationError('repeat-count','Invalid aggregate repetition',e.offset);for(let i=0n;i<(repeat.value as bigint);i++){await this.tick();result.push(value);}}return expressAggregate(result);}
 case 'qualify':{const value=await get(0);return value.kind==='entity'?this.typedView(value,[e.name!],scope):value;}
 case 'attribute':return this.attribute(await get(0),e.name!,scope);
 case 'index':{const value=await get(0),index=await get(1);if(missing(value)||index.kind!=='integer')return unknown;const position=index.value as bigint;if(value.kind==='aggregate'){const at=position-(value.lower??1n);return at<0n||at>=BigInt(items(value).length)?unknown:items(value)[Number(at)]!;}if(value.kind==='string')return this.stringRange(String(value.value),position,position);return unknown;}
 case 'slice':{const value=await get(0),lo=await get(1),hi=await get(2);if(lo.kind!=='integer'||hi.kind!=='integer')return unknown;const lower=lo.value as bigint,upper=hi.value as bigint;if(value.kind==='string')return this.stringRange(String(value.value),lower,upper);if(value.kind!=='aggregate')return unknown;const start=lower-(value.lower??1n),end=upper-(value.lower??1n)+1n;if(start<0n||end<=start||end>BigInt(items(value).length))return unknown;const selected:ExpressValue[]=[];for(let at=Number(start);at<Number(end);at++){await this.tick();selected.push(items(value)[at]!);}return expressAggregate(selected);} case 'query':{const population=await get(0);if(missing(population))return unknown;const result:ExpressValue[]=[];for(const value of items(population)){const local=await this.queryScope(scope);local.set(e.name!,value);if(truth(await this.evaluate(operands[1]!,local))==='true')result.push(value);}return expressAggregate(result,population.aggregate,population.lower);}
 case 'interval':{const value=await get(1),operators=e.operator!.split(',');return this.logicalBinary('AND',await this.binary(operators[0]!,await get(0),value),await this.binary(operators[1]!,value,await get(2)));}
 case 'unary':{const value=await get(0);if(e.operator==='NOT')return logical(truth(value)==='true'?'false':truth(value)==='false'?'true':'unknown');if(missing(value))return unknown;if(!numeric(value))throw new ExpressEvaluationError('operand','Numeric unary operand required',e.offset);return e.operator==='+'?value:{...value,value:value.kind==='integer'?-(value.value as bigint):-Number(value.value)};}
 case 'binary':{const left=await get(0);if(e.operator==='AND'&&truth(left)==='false')return logical('false');if(e.operator==='OR'&&truth(left)==='true')return logical('true');return this.binary(e.operator!,left,await get(1));}
 case 'call':{const name=operands[0]?.name;if(!name)throw new ExpressEvaluationError('call','Named callable required',e.offset);if(name==='NVL'){const value=await get(1);return missing(value)?get(2):value;}const args:ExpressValue[]=[];for(const argument of operands.slice(1))args.push(await this.evaluate(argument,scope));return this.call(name,args,scope);}
 }}
 async stringRange(value:string,lower:bigint,upper:bigint):Promise<ExpressValue>{if(lower<1n||upper<lower||upper>BigInt(value.length))return unknown;const result:string[]=[];let position=0;for(const character of value){if(position%4096===0)await this.tick();position++;if(BigInt(position)>=lower)result.push(character);if(BigInt(position)===upper)return{kind:'string',value:result.join('')};}return unknown;}
 async stringLength(value:string):Promise<number>{let length=0;for(const character of value){if(length%4096===0)await this.tick();length++;}return length;} private logicalBinary(op:string,a:ExpressValue,b:ExpressValue):ExpressValue{const x=truth(a),y=truth(b);return logical(op==='AND'?(x==='false'||y==='false'?'false':x==='unknown'||y==='unknown'?'unknown':'true'):op==='OR'?(x==='true'||y==='true'?'true':x==='unknown'||y==='unknown'?'unknown':'false'):x==='unknown'||y==='unknown'?'unknown':x!==y?'true':'false');}
 async equal(a:ExpressValue,b:ExpressValue,identity=false,depth=0):Promise<ExpressValue>{
  await this.tick();if(depth>(this.#controlScope.control.maximumDepth??256))throw new ExpressEvaluationError('depth','Equality depth exceeded');
  if(missing(a)||missing(b))return logical('unknown');
  if(a.kind==='string'&&b.kind==='string'||a.kind==='binary'&&b.kind==='binary'){const left=String(a.value),right=String(b.value);if(left.length!==right.length)return bool(false);for(let at=0;at<left.length;at+=4096){await this.tick();if(left.slice(at,at+4096)!==right.slice(at,at+4096))return bool(false);}return bool(true);}
  if(a.kind==='aggregate'&&b.kind==='aggregate'){
   const left=items(a),right=items(b);if(left.length!==right.length)return bool(false);const unordered=['SET','BAG'].includes(a.aggregate??'')||['SET','BAG'].includes(b.aggregate??'');
   if(unordered){if(await this.matching(left,right,identity,false,depth)===left.length)return bool(true);return await this.matching(left,right,identity,true,depth)===left.length?logical('unknown'):bool(false);}
   let uncertain=false;for(let index=0;index<left.length;index++){const result=truth(await this.equal(left[index]!,right[index]!,identity,depth+1));if(result==='false')return bool(false);uncertain ||= result==='unknown';}return logical(uncertain?'unknown':'true');
  }  if(a.kind==='entity'&&b.kind==='entity'&&a.identity!==undefined&&a.identity===b.identity)return bool(true);
  if(a.kind==='entity'&&b.kind==='entity'&&!identity){if((a.types??[]).some(name=>!(b.types??[]).includes(name))||(b.types??[]).some(name=>!(a.types??[]).includes(name)))return bool(false);const left=a.fields??new Map(),right=b.fields??new Map();if(left.size!==right.size)return bool(false);let uncertain=false;for(const [name,value] of left){const result=truth(await this.equal(value,right.get(name)??unknown,false,depth+1));if(result==='false')return bool(false);uncertain ||= result==='unknown';}return logical(uncertain?'unknown':'true');}
  return scalarEqual(a,b);
 }
 async matching(left:ExpressValue[],right:ExpressValue[],identity:boolean,possible:boolean,depth=0):Promise<number>{
  if(left.length>right.length)return 0;const edges:number[][]=[];
  for(const a of left){const row:number[]=[];for(let index=0;index<right.length;index++){const result=truth(await this.equal(a,right[index]!,identity,depth+1));if(result==='true'||possible&&result==='unknown')row.push(index);}edges.push(row);}
  const matched=new Map<number,number>(),leftMatched=new Map<number,number>();let size=0;
  for(let start=0;start<left.length;start++){const queue=[start],visited=new Set([start]),parent=new Map<number,number>();let free:number|undefined;
   for(let at=0;at<queue.length&&free===undefined;at++){const node=queue[at]!;for(const target of edges[node]!){await this.tick();if(parent.has(target))continue;parent.set(target,node);const owner=matched.get(target);if(owner===undefined){free=target;break;}if(!visited.has(owner)){visited.add(owner);queue.push(owner);}}}
   if(free===undefined)continue;let target=free;for(;;){const node=parent.get(target)!,previous=leftMatched.get(node);matched.set(target,node);leftMatched.set(node,target);if(previous===undefined)break;target=previous;}size++;
  }return size;
 }
 async binary(op:string,a:ExpressValue,b:ExpressValue):Promise<ExpressValue>{
 if(['AND','OR','XOR'].includes(op))return this.logicalBinary(op,a,b);
 if(['=','<>',':=:',':<>:'].includes(op)){const equal=await this.equal(a,b,op.includes(':'));return op==='='||op===':=:'?equal:logical(truth(equal)==='unknown'?'unknown':truth(equal)==='true'?'false':'true');}
 if(missing(a)||missing(b))return ['<','<=','>','>=','IN','LIKE'].includes(op)?logical('unknown'):unknown;
 if(op==='IN'){let uncertain=false;for(const value of items(b)){const equal=truth(await this.equal(a,value,true));if(equal==='true')return bool(true);uncertain ||= equal==='unknown';}return logical(uncertain?'unknown':'false');}
 if(op==='||'&&a.kind==='entity'&&b.kind==='entity')return{kind:'entity',identity:Symbol(),fields:new Map([...(a.fields??[]),...(b.fields??[])]),groups:new Map([...(a.groups??[]),...(b.groups??[])]),types:[...new Set([...(a.types??[]),...(b.types??[])])]};
 if(a.kind==='aggregate'||b.kind==='aggregate'){
 const left=a.kind==='aggregate'?items(a):[a],right=b.kind==='aggregate'?items(b):[b];
 const find=async(values:ExpressValue[],value:ExpressValue)=>{let index=-1,uncertain=false;for(let i=0;i<values.length;i++){const equal=truth(await this.equal(value,values[i]!,true));if(equal==='true'){index=i;break;}uncertain ||= equal==='unknown';}return{index,uncertain};};
 if(op==='<='||op==='>='){const from=op==='<='?left:right,to=op==='<='?right:left;if(await this.matching(from,to,true,false)===from.length)return bool(true);return await this.matching(from,to,true,true)===from.length?logical('unknown'):bool(false);}
 const uncertain=():never=>{throw new ExpressEvaluationError('aggregate-indeterminate','Aggregate output membership is indeterminate');};
 if(op==='+'){const result=left.slice();for(const value of right){await this.tick();if((a.aggregate??b.aggregate)==='SET'){const found=await find(result,value);if(found.index>=0)continue;if(found.uncertain)uncertain();}result.push(value);}return expressAggregate(result,a.aggregate??b.aggregate);}
 if(op==='-'){const result=left.slice();for(const value of right){const found=await find(result,value);if(found.index>=0)result.splice(found.index,1);else if(found.uncertain)uncertain();}return expressAggregate(result,a.aggregate);}
 if(op==='*'){const result:ExpressValue[]=[],remaining=right.slice(),kind=a.aggregate==='SET'||b.aggregate==='SET'?'SET':a.aggregate;for(const value of left){const found=await find(remaining,value);if(found.index>=0){if(kind!=='SET'||(await find(result,value)).index<0)result.push(value);if(kind==='BAG')remaining.splice(found.index,1);}else if(found.uncertain)uncertain();}return expressAggregate(result,kind);}
 throw new ExpressEvaluationError('aggregate-operator',`Unsupported aggregate operator ${op}`);} if(['<','<=','>','>='].includes(op)){const left=a.kind==='logical'?['false','unknown','true'].indexOf(String(a.value)):a.value as bigint|number|string,right=b.kind==='logical'?['false','unknown','true'].indexOf(String(b.value)):b.value as bigint|number|string;return bool(op==='<'?left<right:op==='<='?left<=right:op==='>'?left>right:left>=right);}
 if(op==='+'&&a.kind==='string'&&b.kind==='string')return{kind:'string',value:String(a.value)+String(b.value)};
 if(!numeric(a)||!numeric(b))throw new ExpressEvaluationError('numeric-operand',`Numeric operands required for ${op}`);
 if(['/','DIV','MOD'].includes(op)&&Number(b.value)===0)return unknown;
 if(a.kind==='integer'&&b.kind==='integer'&&op!=='/'){const x=a.value as bigint,y=b.value as bigint;if(op==='+')return integer(x+y);if(op==='-')return integer(x-y);if(op==='*')return integer(x*y);if(op==='DIV')return integer(x/y);if(op==='MOD')return integer(x%y);if(op==='**'&&y>=0n){if(y>100_000n)throw new ExpressEvaluationError('numeric-budget','Exponent budget exceeded');return integer(x**y);}}
 if(!['+','-','*','/','**'].includes(op))throw new ExpressEvaluationError('unsupported-operator','Unsupported numeric operator '+op);const x=Number(a.value),y=Number(b.value),value=op==='+'?x+y:op==='-'?x-y:op==='*'?x*y:op==='/'?x/y:op==='**'?x**y:NaN;if(!Number.isFinite(value))throw new ExpressEvaluationError('numeric-overflow','Arithmetic exceeds finite evaluation precision');return{kind:'real',value};
 }
 async attribute(value:ExpressValue,name:string,scope?:ExpressScope):Promise<ExpressValue>{if(value.kind==='enum'&&ifc4x3Catalog.types[String(value.value)]?.kind==='enum'&&ifc4x3Catalog.types[String(value.value)]!.members.includes(name))return{kind:'enum',value:name,types:[String(value.value)]};if(value.kind!=='entity')return unknown;const field=value.fields?.get(name);return this.readValue(field&&!missing(field)?field:await this.resolveAttribute?.(value,name,value.types?.length===1?value.types[0]:undefined,scope)??field??unknown,scope,value);}
 async declare(declarations:readonly ExpressVariable[],scope:ExpressScope):Promise<void>{for(const declaration of declarations){const value=declaration.initial?await this.evaluate(declaration.initial,scope):unknown;for(const name of declaration.names){if(!this.declarations.has(scope))this.declarations.set(scope,new Map());this.declarations.get(scope)!.set(name,declaration.type);scope.set(name,await this.coerce(value,declaration.type,scope));}}}
 async copy(value:ExpressValue,depth=0):Promise<ExpressValue>{await this.tick();if(depth>(this.#controlScope.control.maximumDepth??256))throw new ExpressEvaluationError('depth','Aggregate copy depth exceeded');if(value.kind!=='aggregate')return value;const result:ExpressValue[]=[];for(const item of items(value))result.push(await this.copy(item,depth+1));return{...value,value:result};}
 async coerce(value:ExpressValue,type:string,scope:ExpressScope):Promise<ExpressValue>{value=await this.copy(value);const aggregate=/^\s*(LIST|SET|BAG|ARRAY)\s*(?:\[([^:]+):([^\]]+)\])?/i.exec(type);if(aggregate&&value.kind==='aggregate'){let lower=value.lower??1n;if(aggregate[1]!.toUpperCase()==='ARRAY'&&aggregate[2]){const bound=scope.get(aggregate[2].trim().toUpperCase());lower=bound?.kind==='integer'?bound.value as bigint:/^-?\d+$/.test(aggregate[2].trim())?BigInt(aggregate[2].trim()):1n;}return{...value,aggregate:aggregate[1]!.toUpperCase() as ExpressValue['aggregate'],lower};}return value;}
 ancestry(name:string):string[]{const result:string[]=[];while(name&&!result.includes(name)){result.push(name);const type=ifc4x3Catalog.types[name];name=ifc4x3Catalog.entities[name]?.supertype??(type?.typeExpr?.kind==='named'?type.typeExpr.name!:'');}return result;}
 primitiveTypes(value:ExpressValue):string[]{return value.kind==='integer'?['INTEGER','REAL','NUMBER']:value.kind==='real'?['REAL','NUMBER']:value.kind==='logical'?['LOGICAL',...(truth(value)==='unknown'?[]:['BOOLEAN'])]:value.kind==='aggregate'?[value.aggregate??'LIST']:value.kind==='unknown'?[]:[value.kind.toUpperCase()];}
 /** 🕸️ Close actual runtime seeds over released SELECT membership without inferring defined aliases. */
 private async typeOf(value:ExpressValue):Promise<ExpressValue>{
  const prefix=ifc4x3Catalog.schema+'.',names=[...new Set(value.types??this.primitiveTypes(value))],bare=(name:string)=>name.startsWith(prefix)?name.slice(prefix.length):name.includes('.')?undefined:name;
  if(names.some(name=>{const key=bare(name);return key&&(ifc4x3Catalog.entities[key]||ifc4x3Catalog.types[key]);})){
   if(!this.selectParents){
    const parents=new Map<string,string[]>();
    for(const type of Object.values(ifc4x3Catalog.types)){await this.tick();if(type.kind==='select')for(const member of type.members){await this.tick();const owners=parents.get(member)??[];owners.push(type.name.toUpperCase());parents.set(member,owners);}}
    this.selectParents=parents;
   }
   const seen=new Set(names.map(name=>bare(name)??name));
   for(let index=0;index<names.length;index++){await this.tick();const key=bare(names[index]!);if(key)for(const parent of this.selectParents.get(key)??[]){await this.tick();if(!seen.has(parent)){seen.add(parent);names.push(parent);}}}
  }
  return expressAggregate([...new Set(names.map(value=>value.startsWith('IFC')&&!value.includes('.')?prefix+value:value))].map(value=>({kind:'string',value})),'SET');
 }
 async call(name:string,args:ExpressValue[],scope:ExpressScope=new Map()):Promise<ExpressValue>{const root=!this.#readScopes.has(scope);scope=await this.rootScope(scope);const local:ExpressValue[]=[];for(const arg of args)local.push(await this.readValue(arg,scope));const result=await this.readValue(await this.invoke(name,local,scope),scope);return root?this.#readScopes.get(scope)!.publish(result):result;}
 private async invoke(name:string,args:ExpressValue[],scope:ExpressScope):Promise<ExpressValue>{await this.tick();name=name.toUpperCase();const a=args[0]??unknown,b=args[1]??unknown;
 if(name==='EXISTS')return bool(!missing(a));
 if(name==='TYPEOF')return this.typeOf(a);
 if(name==='USEDIN'){if(!this.usedIn)throw new ExpressEvaluationError('context','USEDIN requires a population context');return this.usedIn(a,String(b.value??''));}
 if(missing(a)&&['SIZEOF','HIINDEX','LOINDEX','LENGTH','BLENGTH','ABS','SQRT'].includes(name))return unknown;
 if(name==='SIZEOF')return integer(BigInt(items(a).length));if(name==='HIINDEX')return integer((a.lower??1n)+BigInt(items(a).length)-1n);if(name==='LOINDEX')return integer(a.lower??1n);
 if(name==='LENGTH')return integer(BigInt(await this.stringLength(String(a.value))));if(name==='BLENGTH')return integer(BigInt((String(a.value).length-1)*4-Number(String(a.value)[0])));
 if(name==='ABS')return a.kind==='integer'?integer((a.value as bigint)<0n?-(a.value as bigint):a.value as bigint):{kind:'real',value:Math.abs(Number(a.value))};if(name==='SQRT')return Number(a.value)<0?unknown:{kind:'real',value:Math.sqrt(Number(a.value))};
 const fn=this.functions.get(name);if(fn){const local=this.proceduralScope(scope);let index=0;for(const parameter of fn.parameters)for(const key of parameter.names)local.set(key,await this.coerce(args[index++]??unknown,parameter.type,scope));await this.declare(fn.locals,local);const result=await this.statements(fn.body,local);return this.coerce(result?.kind==='return'?result.value!:unknown,fn.resultType,local);}
 const declaration=ifc4x3Catalog.entities[name];if(declaration){const fields=new Map<string,ExpressValue>();for(let index=0;index<declaration.attributes.length;index++)fields.set(declaration.attributes[index]!.name.toUpperCase(),await this.copy(args[index]??unknown));return{kind:'entity',identity:Symbol(),types:this.ancestry(name),fields,groups:new Map([[name,fields]])};}
 if(ifc4x3Catalog.types[name])return{...a,types:[...new Set([...this.ancestry(name),...(a.types??this.primitiveTypes(a))])]};throw new ExpressEvaluationError('unsupported-call',`Unsupported EXPRESS callable ${name}`);
 }
 async statements(body:readonly Stmt[],scope:ExpressScope):Promise<{kind:'return'|'escape'|'skip';value?:ExpressValue}|undefined>{if(!this.#readScopes.has(scope))this.#readScopes.set(scope,new ExpressReadScope(()=>this.tick()));for(const statement of body){await this.tick(statement.offset);const expressions=statement.expressions??[],get=(i:number)=>this.evaluate(expressions[i]!,scope);switch(statement.kind){case 'return':return{kind:'return',value:await get(0)};case 'escape':return{kind:'escape'};case 'skip':return{kind:'skip'};case 'null':break;case 'call':await get(0);break;case 'assign':await this.assign(expressions[0]!,await get(1),scope);break;
 case 'block':{const flow=await this.statements(statement.body??[],scope);if(flow)return flow;break;}
 case 'if':{const flow=await this.statements(truth(await get(0))==='true'?statement.body??[]:statement.otherwise??[],scope);if(flow)return flow;break;}
 case 'case':{const selector=await get(0);let selected=statement.otherwise??[];outer:for(const entry of statement.cases??[])for(const label of entry.labels)if(truth(await this.equal(selector,await this.evaluate(label,scope)))==='true'){selected=entry.body;break outer;}const flow=await this.statements(selected,scope);if(flow)return flow;break;}
 case 'repeat':{let current=statement.name?await get(0):integer(1n),last=statement.name?await get(1):unknown,step=statement.name?await get(2):integer(1n);if(statement.name&&(current.kind!=='integer'||last.kind!=='integer'||step.kind!=='integer'||step.value===0n))break;for(let iteration=0;;iteration++){if(iteration>=(this.#controlScope.control.maximumIterations??1_000_000))throw new ExpressEvaluationError('iteration-budget','EXPRESS iteration budget exceeded',statement.offset);await this.tick();if(statement.name){if((step.value as bigint)>0n?(current.value as bigint)>(last.value as bigint):(current.value as bigint)<(last.value as bigint))break;scope.set(statement.name,current);}if(statement.while&&truth(await this.evaluate(statement.while,scope))!=='true')break;const flow=await this.statements(statement.body??[],scope);if(flow?.kind==='return')return flow;if(flow?.kind==='escape')break;if(statement.until&&truth(await this.evaluate(statement.until,scope))==='true')break;if(statement.name)current=integer((current.value as bigint)+(step.value as bigint));}break;}
 }}return undefined;}
 private async assign(target:Expr,value:ExpressValue,scope:ExpressScope):Promise<void>{value=await this.copy(value);if(target.kind==='identifier'){const type=this.declarations.get(scope)?.get(target.name!);scope.set(target.name!,type?await this.coerce(value,type,scope):value);return;}if(target.kind==='attribute'){const owner=await this.evaluate(target.operands![0]!,scope);if(owner.kind!=='entity')throw new ExpressEvaluationError('assignment','Entity target required');owner.fields?.set(target.name!,value);return;}if(target.kind==='index'){const owner=await this.evaluate(target.operands![0]!,scope),index=await this.evaluate(target.operands![1]!,scope);if(owner.kind!=='aggregate'||index.kind!=='integer')throw new ExpressEvaluationError('assignment','Aggregate target required');const at=Number((index.value as bigint)-(owner.lower??1n));if(at<0||at>=items(owner).length)throw new ExpressEvaluationError('assignment-index','Aggregate assignment index outside bounds');items(owner)[at]=value;return;}throw new ExpressEvaluationError('assignment','Unsupported assignment target');}
}
/** 🧮 Execute one canonical expression without losing integer literal precision. */
export async function evaluateExpressExpression(expression:Expr,scope:ExpressScope=new Map(),control:ExpressControl={}):Promise<ExpressValue>{return new ExpressEvaluator(undefined,control).evaluate(expression,scope);}














