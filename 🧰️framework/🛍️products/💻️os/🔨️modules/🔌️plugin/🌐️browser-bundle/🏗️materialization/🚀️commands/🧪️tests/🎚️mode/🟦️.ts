import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import ts from"typescript";
import corpus from"./🔣️.json"with{type:"json"};
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🎚️ Exercises the actual pre-effect publication predicates as pure DATA against the two-intent SQL oracle. */
test("neutral component browser publication admits the exact Original mode in both actual owners",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 const path=new URL("../../🟦️.ts",import.meta.url),file=ts.createSourceFile(path.pathname,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),database=new Database(":memory:");
 try{for(const name of["publishComponentBrowserV1","publishComponentModuleV1"]){
  const owner=file.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text===name);if(!owner?.body)throw Error("Actual component browser publication owner required");
  const guard=owner.body.statements.find((node):node is ts.IfStatement=>ts.isIfStatement(node)&&node.expression.getText(file).includes("receipt.profile"));if(!guard||!ts.isThrowStatement(guard.thenStatement))throw Error("Actual pre-effect receipt-mode refusal required");
  const expression=guard.expression.getText(file),identifiers:string[]=[];const visit=(node:ts.Node)=>{if(ts.isIdentifier(node))identifiers.push(node.text);ts.forEachChild(node,visit);};visit(guard.expression);expect(identifiers.every(value=>["receipt","profile","mode"].includes(value))).toBe(true);
  const refuse=new Function("receipt","mode","return ("+expression+");")as(receipt:Readonly<{profile:string}>,mode:string)=>boolean;
  const actual=corpus.cases.map(row=>!refuse({profile:row.profile},row.mode)),expected=corpus.cases.map(row=>Boolean((database.query("select ?1 in ('prototype','release') and ?1=?2 admitted").get(row.mode,row.profile)as{admitted:number}).admitted));expect(expected).toEqual(corpus.cases.map(row=>row.accept));
  expect(actual).toEqual(expected);
  const before=owner.body.statements.slice(0,owner.body.statements.indexOf(guard)).map(node=>node.getText(file)).join("\n");expect(before).toContain("readNativeCallerAuthorityV1(work.invocation.capabilities).original.buildMode");expect(before).not.toContain("withRepositoryCacheStoreV1(");
 }}finally{database.close();}
});

/** 📌️ Replays only actual receipt predicates as DATA while retaining every other model custody check in Source. */
test("neutral component model readers retain exact two-intent receipt predicates",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const database=new Database(":memory:"),observations:{owner:string;right:string;selection:string;actual:boolean[];expected:boolean[]}[]=[];
 try{for(const row of corpus.readers){
  const file=ts.createSourceFile(row.path,readFileSync(row.path,"utf8"),ts.ScriptTarget.Latest,true),owner=file.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text===row.owner);if(!owner?.body)throw Error("Actual component model reader required");
  const predicates:ts.BinaryExpression[]=[];const visit=(node:ts.Node)=>{if(ts.isBinaryExpression(node)&&node.operatorToken.kind===ts.SyntaxKind.ExclamationEqualsEqualsToken&&node.left.getText(file)==="receipt.profile")predicates.push(node);ts.forEachChild(node,visit);};visit(owner.body);expect(predicates).toHaveLength(1);const predicate=predicates[0];if(!predicate)throw Error("Actual model receipt-mode predicate required");
  const right=predicate.right.getText(file),expression=predicate.getText(file),identifiers:string[]=[];const inspect=(node:ts.Node)=>{if(ts.isIdentifier(node))identifiers.push(node.text);ts.forEachChild(node,inspect);};inspect(predicate);expect(identifiers.every(value=>["receipt","profile","mode","intent"].includes(value))).toBe(true);
  const refuse=new Function("receipt","mode","intent","return ("+expression+");")as(receipt:Readonly<{profile:string}>,mode:string,intent:Readonly<{mode:string}>)=>boolean,actual=corpus.cases.map(value=>!refuse({profile:value.profile},value.mode,{mode:value.mode})),expected=corpus.cases.map(value=>Boolean((database.query("select ?1 in ('prototype','release') and ?1=?2 admitted").get(value.mode,value.profile)as{admitted:number}).admitted));
  observations.push({owner:row.owner,right,selection:row.selection,actual,expected});
 }for(const row of observations){expect(row.actual,row.owner).toEqual(row.expected);expect(row.right).toBe(row.selection);}}finally{database.close();}
});


/** 🪪️ The reader must compare intent DATA with its actual Original before entering any managed products store. */
test("neutral component model reader refuses a different Original mode before storage entry",()=>{
 const row=corpus.readers.find(value=>value.owner==="withComponentModelV1");if(!row)throw Error("Actual component model reader declaration required");const file=ts.createSourceFile(row.path,readFileSync(row.path,"utf8"),ts.ScriptTarget.Latest,true),owner=file.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text===row.owner);if(!owner?.body)throw Error("Actual component model reader required");const guards=owner.body.statements.filter(ts.isIfStatement),predicates:ts.BinaryExpression[]=[];for(const guard of guards){const visit=(node:ts.Node)=>{if(ts.isBinaryExpression(node)&&node.left.getText(file)==="intent.mode"&&node.right.getText(file)==="authority.original.buildMode")predicates.push(node);ts.forEachChild(node,visit);};visit(guard.expression);}expect(predicates).toHaveLength(1);const predicate=predicates[0];if(!predicate)throw Error("Actual pre-store Original mode predicate required");const refuse=new Function("intent","authority","return ("+predicate.getText(file)+");")as(intent:{mode:string},authority:{original:{buildMode:string}})=>boolean,database=new Database(":memory:");try{for(const value of corpus.cases.filter(value=>value.profile==="prototype"||value.profile==="release")){const actual=!refuse({mode:value.mode},{original:{buildMode:value.profile}}),oracle=Boolean((database.query("select ?1=?2 admitted").get(value.mode,value.profile)as{admitted:number}).admitted);expect(actual).toBe(oracle);}}finally{database.close();}
});
