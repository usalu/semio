import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {Database} from "bun:sqlite";
import ts from "typescript";
import corpus from "./🔣️.json";

function source(path:string):ts.SourceFile{return ts.createSourceFile(path,readFileSync(join(process.cwd(),path),"utf8"),ts.ScriptTarget.Latest,true);}
function nodes(tree:ts.Node):readonly ts.Node[]{const result:ts.Node[]=[];const visit=(node:ts.Node)=>{result.push(node);ts.forEachChild(node,visit);};visit(tree);return result;}
function callName(node:ts.CallExpression):string{return ts.isIdentifier(node.expression)?node.expression.text:ts.isPropertyAccessExpression(node.expression)?node.expression.name.text:"";}

/** 🎟️ Neutral parameter DATA and independent AST/SQL agree on every required production boundary. */
test("neutral Hub carried phase has required exact caller and option contracts",()=>{
 expect(corpus.functions).toHaveLength(4);expect(corpus.options).toEqual(["invocation","phaseControl"]);const database=new Database(":memory:");try{for(const row of corpus.functions){const tree=source(row.path),owner=nodes(tree).find(node=>ts.isFunctionDeclaration(node)&&node.name?.text===row.name);if(!owner||!ts.isFunctionDeclaration(owner))throw Error("Actual function owner required");const actual=owner.parameters.map(parameter=>parameter.name.getText(tree));expect(database.query("select json(?1)=json(?2) exact").get(JSON.stringify(actual),JSON.stringify(row.parameters)),row.name).toEqual({exact:1});expect(owner.parameters.every(parameter=>!parameter.questionToken&&!parameter.initializer)).toBe(true);}const tree=source(corpus.functions[2]!.path),owner=tree.statements.find(node=>ts.isTypeAliasDeclaration(node)&&node.name.text==="IoMatrixOptions");if(!owner||!ts.isTypeAliasDeclaration(owner))throw Error("Actual I/O options owner required");for(const field of corpus.options)expect(owner.getText(tree)).toContain(field+":");}finally{database.close();}
});

/** 🧱️ Compiler failure cannot be healed by a PID marker, stale path or a fresh signal after admission. */
test("neutral Hub carried phase prebuild uses one canonical browser producer and preserves failure",()=>{
 const tree=source(corpus.functions[0]!.path),owner=nodes(tree).find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="collabPrebuildPlugins");if(!owner)throw Error("Actual prebuild owner required");const calls=nodes(owner).filter(ts.isCallExpression);expect(calls.filter(node=>callName(node)==="buildComponentClosureV1")).toHaveLength(1);for(const name of["phaseCompilerAdmissionControlV1","acquirePluginBuildLease","markPluginBuildLeaseReady","existsSync","buildPlugin"])expect(calls.some(node=>callName(node)===name),name).toBe(false);expect(nodes(owner).some(ts.isCatchClause)).toBe(false);expect(owner.getText(tree)).toContain('facet:"browser"');expect(owner.getText(tree)).toContain("repositoryCargoNetworkV1(invocation,repoRoot)");
});

/** 🔗️ Existing Hub Script entries carry their actual Original into both harnesses and onward to sweep. */
test("neutral Hub carried phase actual entries and sweep forward their exact owner",()=>{
 const entry=source("🌎️hub/🎛️commands/🚦️gate-commands/🟦️.ts");for(const name of["runCollabE2eCli","runIoMatrixCli"]){const calls=nodes(entry).filter(ts.isCallExpression).filter(node=>callName(node)===name);expect(calls).toHaveLength(1);for(const call of calls){expect(call.arguments[0]?.getText(entry)).toBe("this.invocation");expect(call.arguments[1]?.getText(entry)).toContain("scriptCompilerAdmissionControlV1(this.invocation,");}}const tree=source(corpus.functions[2]!.path),owner=nodes(tree).find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="runIoMatrixCli");if(!owner)throw Error("Actual I/O entry required");expect(nodes(owner).some(node=>ts.isNewExpression(node)&&node.expression.getText(tree)==="AbortController")).toBe(false);const sweep=nodes(tree).find(node=>ts.isVariableDeclaration(node)&&node.name.getText(tree)==="sweep");if(!sweep)throw Error("Actual sweep request required");for(const field of corpus.options)expect(sweep.getText(tree)).toContain(field+": options."+field);
});
