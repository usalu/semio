/** 🔗️ Extracts one module-level owner function together with every top-level declaration and import it transitively names, so the owner runs against the module's own bindings instead of a hand-kept binding list. Only the names passed as `provided` (the scenario stubs of a test) are taken from the caller. */
import {readFile} from "node:fs/promises";
import {dirname} from "node:path";
import {fileURLToPath} from "node:url";
import ts from "typescript";

const IMPORT_META_BINDING_V1 = "__importMetaV1";

export type OwnerClosureV1 = Readonly<{owner:string;code:string;imports:Readonly<Record<string,unknown>>;provided:readonly string[]}>;

function bindingNamesV1(name:ts.BindingName):string[]{
 return ts.isIdentifier(name)?[name.text]:name.elements.flatMap(element=>ts.isBindingElement(element)?bindingNamesV1(element.name):[]);
}

function blockScopedNamesV1(statement:ts.Statement):string[]{
 if(ts.isVariableStatement(statement))return statement.declarationList.declarations.flatMap(declaration=>bindingNamesV1(declaration.name));
 return (ts.isFunctionDeclaration(statement)||ts.isClassDeclaration(statement))&&statement.name?[statement.name.text]:[];
}

/** 🧭️ Names an owner reads from outside its own scopes: identifiers bound by an enclosing parameter, block, catch or loop head are local and never pull a declaration in. */
function referencedNamesV1(node:ts.Node):Set<string>{
 const names=new Set<string>();
 const visit=(current:ts.Node,shadow:ReadonlySet<string>):void=>{
  if(ts.isIdentifier(current)){
   const parent=current.parent,property=(ts.isPropertyAccessExpression(parent)&&parent.name===current)||((ts.isPropertyAssignment(parent)||ts.isPropertyDeclaration(parent)||ts.isMethodDeclaration(parent)||ts.isPropertySignature(parent))&&parent.name===current);
   if(!property&&!shadow.has(current.text))names.add(current.text);
   return;
  }
  const bound:string[]=[];
  if(ts.isFunctionLike(current))for(const parameter of current.parameters)bound.push(...bindingNamesV1(parameter.name));
  if(ts.isBlock(current))for(const statement of current.statements)bound.push(...blockScopedNamesV1(statement));
  if(ts.isCatchClause(current)&&current.variableDeclaration)bound.push(...bindingNamesV1(current.variableDeclaration.name));
  if((ts.isForStatement(current)||ts.isForOfStatement(current)||ts.isForInStatement(current))&&current.initializer&&ts.isVariableDeclarationList(current.initializer))for(const declaration of current.initializer.declarations)bound.push(...bindingNamesV1(declaration.name));
  const inner=bound.length?new Set([...shadow,...bound]):shadow;
  ts.forEachChild(current,child=>visit(child,inner));
 };
 visit(node,new Set());
 return names;
}

/** 🔗️ Reads the module (or `source`, a patched copy of it whose imports still resolve from `moduleUrl`), follows the owner's references through its top-level declarations and imports, and returns the compiled closure. An owner declared in the body of the top-level function `within` is bound together with that body's declarations, and that function's parameters become required stubs. */
export async function prepareOwnerClosureV1({moduleUrl,owner,provided,source,within}:{moduleUrl:URL;owner:string;provided:readonly string[];source?:string;within?:string}):Promise<OwnerClosureV1>{
 const fileName=moduleUrl.pathname,tree=ts.createSourceFile(fileName,source??await readFile(moduleUrl,"utf8"),ts.ScriptTarget.Latest,true,fileName.endsWith(".ts")?ts.ScriptKind.TS:ts.ScriptKind.JS);
 const declared=new Map<string,ts.Statement>(),bindings=new Map<string,{specifier:string;exported:string}>();
 for(const statement of tree.statements){
  if(ts.isImportDeclaration(statement)&&statement.importClause&&!statement.importClause.isTypeOnly){
   const specifier=(statement.moduleSpecifier as ts.StringLiteral).text,clause=statement.importClause,named=clause.namedBindings;
   if(clause.name)bindings.set(clause.name.text,{specifier,exported:"default"});
   if(named&&ts.isNamespaceImport(named))bindings.set(named.name.text,{specifier,exported:"*"});
   else named?.elements.forEach(element=>{if(!element.isTypeOnly)bindings.set(element.name.text,{specifier,exported:(element.propertyName??element.name).text});});
  }else if((ts.isFunctionDeclaration(statement)||ts.isClassDeclaration(statement))&&statement.name)declared.set(statement.name.text,statement);
  else if(ts.isVariableStatement(statement))for(const declaration of statement.declarationList.declarations)if(ts.isIdentifier(declaration.name))declared.set(declaration.name.text,statement);
 }
 const host=within===undefined?undefined:tree.statements.find((statement):statement is ts.FunctionDeclaration=>ts.isFunctionDeclaration(statement)&&statement.name?.text===within);
 if(within!==undefined&&!host?.body)throw Error("Actual enclosing function required: "+within);
 const nested=new Map<string,ts.Statement>(),hostStatements=host?.body?.statements??[];
 for(const statement of hostStatements){
  if((ts.isFunctionDeclaration(statement)||ts.isClassDeclaration(statement))&&statement.name)nested.set(statement.name.text,statement);
  else if(ts.isVariableStatement(statement))for(const declaration of statement.declarationList.declarations)if(ts.isIdentifier(declaration.name))nested.set(declaration.name.text,statement);
 }
 const scopeOf=(name:string)=>nested.get(name)??declared.get(name);
 if(!scopeOf(owner))throw Error("Actual owner declaration required: "+owner);
 const stubbed=new Set(provided),seen=new Set<string>(),statements=new Set<ts.Statement>(),needed=new Set<string>(),queue=[owner];
 while(queue.length){
  const name=queue.pop()!;
  if(seen.has(name)||stubbed.has(name))continue;
  seen.add(name);
  const statement=scopeOf(name);
  if(statement){if(!statements.has(statement)){statements.add(statement);queue.push(...referencedNamesV1(statement));}}
  else if(bindings.has(name))needed.add(name);
 }
 const parameters=(host?.parameters??[]).map(parameter=>parameter.name.getText(tree)).filter(name=>seen.has(name)),required=[...new Set([...provided,...parameters])];
 const render=(statement:ts.Statement):string|undefined=>{
  if(!ts.isVariableStatement(statement))return statement.getText(tree).replace(/^export\s+/u,"");
  const kept=statement.declarationList.declarations.filter(declaration=>!bindingNamesV1(declaration.name).some(name=>stubbed.has(name)));
  if(kept.length===0)return undefined;
  const keyword=statement.declarationList.flags&ts.NodeFlags.Const?"const":statement.declarationList.flags&ts.NodeFlags.Let?"let":"var";
  return keyword+" "+kept.map(declaration=>declaration.getText(tree)).join(",")+";";
 };
 const body=[...tree.statements,...hostStatements].filter(statement=>statements.has(statement)).map(render).filter((text):text is string=>text!==undefined).join("\n").replace(/\bimport\.meta\b/gu,IMPORT_META_BINDING_V1),code=ts.transpileModule(body,{fileName,compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText.replace(/^export \{\};\s*$/mu,""),imports:Record<string,unknown>={};
 if(body.includes(IMPORT_META_BINDING_V1))imports[IMPORT_META_BINDING_V1]=Object.freeze({url:moduleUrl.href,filename:fileURLToPath(moduleUrl),dirname:dirname(fileURLToPath(moduleUrl))});
 for(const name of needed){
  const {specifier,exported}=bindings.get(name)!,namespace=await import(new URL(specifier,moduleUrl).href);
  imports[name]=exported==="*"?namespace:exported==="default"?namespace.default:namespace[exported];
 }
 return {owner,code,imports,provided:required};
}

/** 🧩️ Compiles a prepared closure with the caller's stubs for its `provided` names and returns the owner function. */
export function instantiateOwnerClosureV1(closure:OwnerClosureV1,values:Readonly<Record<string,unknown>>):(...args:any[])=>any{
 const missing=closure.provided.filter(name=>!Object.hasOwn(values,name));
 if(missing.length)throw Error("Stubs required for: "+missing.join(", "));
 const all={...closure.imports,...values};
 return new Function(...Object.keys(all),closure.code+";return "+closure.owner+";")(...Object.values(all));
}
