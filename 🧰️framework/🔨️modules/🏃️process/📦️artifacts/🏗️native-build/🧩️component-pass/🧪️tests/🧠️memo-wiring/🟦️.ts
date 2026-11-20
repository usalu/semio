import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import ts from"typescript";

/** 🧠️ The component build admits every pass through the cross-invocation admission memo: the driver hands the repository memo to the pass, the pass hands it to the selection admission, which hands it to the unit admission. Without it every restart re-read all Source. */
const read=(url:string)=>readFileSync(new URL(url,import.meta.url),"utf8");
const calls=(text:string,name:string)=>{const tree=ts.createSourceFile("x.ts",text,ts.ScriptTarget.Latest,true),found:ts.CallExpression[]=[];const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text===name)found.push(node);ts.forEachChild(node,visit);};visit(tree);return found;};
const property=(call:ts.CallExpression,index:number,name:string)=>{const argument=call.arguments[index];return argument&&ts.isObjectLiteralExpression(argument)?argument.properties.some(row=>ts.isSpreadAssignment(row)?row.getText().includes(name):row.name?.getText()===name):false;};

test("the memo travels from the component driver through the pass and the selection to the unit admission",()=>{
 const pass=read("../../🟦️.ts"),selection=read("../../../📥️source/🎟️admission/🟦️.ts"),driver=read("../../../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts");
 const admits=calls(pass,"admitCargoSelectionV1");expect(admits.length).toBeGreaterThan(0);for(const call of admits)expect(property(call,0,"sourceMemo"),call.getText().slice(0,80)).toBe(true);
 const drivers=calls(driver,"runNativeBuildDriverV1");expect(drivers.length).toBe(1);expect(property(drivers[0]!,0,"sourceMemo")).toBe(true);expect(driver).toContain("sourceMemo:repositoryCargoSourceMemoV1(store)");
 const unit=calls(selection,"admitCargoSourceBoundariesV1");expect(unit.length).toBe(1);expect(unit[0]!.arguments.length).toBe(6);expect(unit[0]!.arguments[5]!.getText()).toBe("selection.sourceMemo");
});
