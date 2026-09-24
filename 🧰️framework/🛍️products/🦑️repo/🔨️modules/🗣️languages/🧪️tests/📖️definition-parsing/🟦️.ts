//#region 🔌️Adapters
import ts from "typescript";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔮️Oracle
/**
 * 🔮️ Reads the declarations out of the TypeScript compiler's own syntax tree. The compiler answers
 * two real language questions the subjects must agree with: which top-level declarations exist and
 * on which line each starts, and whether a variable binding's initialiser is callable — an arrow
 * function, a function expression or a class expression — rather than a value.
 */
function declarationKind(node: ts.Node, source: ts.SourceFile): { name: string; startLine: number; kind: string }[] {
  const lineOf = (position: number): number => source.getLineAndCharacterOfPosition(position).line + 1;
  if (ts.isFunctionDeclaration(node) && node.name !== undefined) return [{ name: node.name.text, startLine: lineOf(node.getStart(source)), kind: "function" }];
  if (ts.isClassDeclaration(node) && node.name !== undefined) return [{ name: node.name.text, startLine: lineOf(node.getStart(source)), kind: "class" }];
  if (ts.isInterfaceDeclaration(node)) return [{ name: node.name.text, startLine: lineOf(node.getStart(source)), kind: "interface" }];
  if (ts.isTypeAliasDeclaration(node)) return [{ name: node.name.text, startLine: lineOf(node.getStart(source)), kind: "type" }];
  if (ts.isEnumDeclaration(node)) return [{ name: node.name.text, startLine: lineOf(node.getStart(source)), kind: "enum" }];
  if (ts.isVariableStatement(node)) {
    const list = node.declarationList;
    const declared = (list.flags & ts.NodeFlags.Const) !== 0 ? "const" : (list.flags & ts.NodeFlags.Let) !== 0 ? "let" : "var";
    const startLine = lineOf(node.getStart(source));
    return list.declarations.flatMap((declaration) => {
      if (!ts.isIdentifier(declaration.name)) return [];
      const initializer = declaration.initializer;
      const callable = initializer !== undefined && (ts.isArrowFunction(initializer) || ts.isFunctionExpression(initializer) || ts.isClassExpression(initializer));
      return [{ name: declaration.name.text, startLine, kind: callable ? "function" : declared }];
    });
  }
  return [];
}

function declarations(text: string, fileName: string): { name: string; startLine: number; kind: string }[] {
  const source = ts.createSourceFile(fileName, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  return source.statements.flatMap((statement) => declarationKind(statement, source));
}
//#endregion 🔮️Oracle

//#region 🧭️Adapter
/** 🟦️ TypeScript oracle for the definition parsing case. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "typescript-top-level-declarations": {
      oracle: (ctx) => ({ projection: declarations(new TextDecoder().decode(ctx.fixtureBytes("shared://🟦️sample.ts")), "🟦️sample.ts") }),
    },
    "callable-const-is-a-function": {
      oracle: (ctx) => ({ projection: declarations(new TextDecoder().decode(ctx.fixtureBytes("shared://📖️definition-parsing/🔤️callables.ts")), "🔤️callables.ts") }),
    },
  },
});
//#endregion 🧭️Adapter
