import ts from"typescript";
/** 🔭️ One site the TypeScript parser finds: a literal specifier or a computed call, with the module a createRequire call is anchored at. */
export interface AstSiteV1{readonly form:"literal"|"computed";readonly kind:"static"|"dynamic"|"require"|"create-require"|"resolve"|"base";readonly specifier:string|null;readonly anchor:"self"|"other"|null}
const isCreateRequire=(node:ts.Node):node is ts.CallExpression=>ts.isCallExpression(node)&&(ts.isIdentifier(node.expression)&&node.expression.text==="createRequire"||ts.isPropertyAccessExpression(node.expression)&&node.expression.name.text==="createRequire");
const selfAnchored=(node:ts.Expression|undefined,text:string):boolean=>node!==undefined&&(/^(?:import\.meta\.(?:url|filename)|__filename)$/u.test(node.getText())||ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="fileURLToPath"&&selfAnchored(node.arguments[0],text)||ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="pathToFileURL"&&selfAnchored(node.arguments[0],text));
const typeOnlyImport=(node:ts.ImportDeclaration|ts.ExportDeclaration):boolean=>{if(ts.isImportDeclaration(node)){const clause=node.importClause;if(!clause)return false;if(clause.isTypeOnly)return true;return clause.name===undefined&&clause.namedBindings!==undefined&&ts.isNamedImports(clause.namedBindings)&&clause.namedBindings.elements.length>0&&clause.namedBindings.elements.every(element=>element.isTypeOnly);}return node.isTypeOnly||node.exportClause!==undefined&&ts.isNamedExports(node.exportClause)&&node.exportClause.elements.length>0&&node.exportClause.elements.every(element=>element.isTypeOnly);};
/** 📋️ Lists every import, dynamic import, require, createRequire(...)(x), require.resolve and import.meta.resolve site with the TypeScript parser, the independent oracle for the literal-site extractor and the computed-site law. A createRequire whose base is not its own module adds a base site. */
export function astSitesV1(path:string,text:string):readonly AstSiteV1[]{
 const body=text.replace(/^#![^\r\n]*/u,""),source=ts.createSourceFile(path,body,ts.ScriptTarget.Latest,true,/\.[cm]?tsx?$/u.test(path)?ts.ScriptKind.TS:ts.ScriptKind.JS),bound=new Map<string,"self"|"other">([["require","self"]]),rows:AstSiteV1[]=[];
 const anchorOf=(call:ts.CallExpression):"self"|"other"=>selfAnchored(call.arguments[0],body)?"self":"other";
 const collect=(node:ts.Node):void=>{if(ts.isVariableDeclaration(node)&&ts.isIdentifier(node.name)&&node.initializer&&isCreateRequire(node.initializer))bound.set(node.name.text,anchorOf(node.initializer));if(ts.isBinaryExpression(node)&&node.operatorToken.kind===ts.SyntaxKind.EqualsToken&&ts.isIdentifier(node.left)&&isCreateRequire(node.right))bound.set(node.left.text,anchorOf(node.right));ts.forEachChild(node,collect);};collect(source);
 const add=(kind:AstSiteV1["kind"],argument:ts.Expression|undefined,anchor:"self"|"other"|null)=>{const specifier=argument&&ts.isStringLiteralLike(argument)?argument.text:null;rows.push({form:specifier===null?"computed":"literal",kind,specifier,anchor});};
 const anchorOfRequireLike=(node:ts.Expression):"self"|"other"|undefined=>ts.isIdentifier(node)?bound.get(node.text):isCreateRequire(node)?anchorOf(node):undefined;
 const visit=(node:ts.Node):void=>{
  if((ts.isImportDeclaration(node)||ts.isExportDeclaration(node))&&node.moduleSpecifier){if(!typeOnlyImport(node))add("static",node.moduleSpecifier,null);}
  else if(ts.isImportEqualsDeclaration(node)&&ts.isExternalModuleReference(node.moduleReference))add("static",node.moduleReference.expression,null);
  else if(ts.isCallExpression(node)){const callee=node.expression;
   if(Boolean(isCreateRequire(node))){if(anchorOf(node)==="other")rows.push({form:"computed",kind:"base",specifier:null,anchor:"other"});}
   else if(node.arguments.length>0){
    if(callee.kind===ts.SyntaxKind.ImportKeyword)add("dynamic",node.arguments[0],null);
    else if(ts.isIdentifier(callee)&&bound.has(callee.text))add("require",node.arguments[0],bound.get(callee.text)!);
    else if(ts.isCallExpression(callee)&&isCreateRequire(callee))add("create-require",node.arguments[0],anchorOf(callee));
    else if(ts.isPropertyAccessExpression(callee)&&callee.name.text==="resolve"){const anchor=anchorOfRequireLike(callee.expression);if(anchor!==undefined)add("resolve",node.arguments[0],anchor);else if(ts.isMetaProperty(callee.expression))add("resolve",node.arguments[0],"self");}}}
  ts.forEachChild(node,visit);};
 visit(source);return rows;
}
