import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { CstParser, Lexer, createToken, EOF } from "chevrotain";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(import.meta.dir, "../..");
const corpus = JSON.parse(readFileSync(resolve(root, "🧫️fixtures/🏗️grammar/🔣️.json"), "utf8"));
const nullable = /^field-value\s*=\s*"="\s+value\?\s*$/m.test(readFileSync(resolve(root, "📖️.grammar.semio"), "utf8"));
const Space = createToken({ name: "Space", pattern: /\s+/, group: Lexer.SKIPPED });
const Text = createToken({ name: "Text", pattern: /"(?:\\.|[^"\\])*"/ });
const Float = createToken({ name: "Float", pattern: /-?inf\b|nan(?:32|64)_[\da-fA-F]+\b|-?\d+\.\d*(?:[eE][+-]?\d+)?|-?\d+[eE][+-]?\d+/ });
const Int = createToken({ name: "Int", pattern: /-?\d+/ });
const Ident = createToken({ name: "Ident", pattern: /[A-Za-z_][A-Za-z0-9_.-]*/ });
const Equal = createToken({ name: "Equal", pattern: /=/ });
const Comma = createToken({ name: "Comma", pattern: /,/ });
const OpenList = createToken({ name: "OpenList", pattern: /\[/ });
const CloseList = createToken({ name: "CloseList", pattern: /\]/ });
const OpenRecord = createToken({ name: "OpenRecord", pattern: /\{/ });
const CloseRecord = createToken({ name: "CloseRecord", pattern: /\}/ });
const tokens = [Space, Text, Float, Int, Ident, Equal, Comma, OpenList, CloseList, OpenRecord, CloseRecord];
const lexer = new Lexer(tokens);

class GrammarOracle extends CstParser {
  document = this.RULE("document", () => { this.CONSUME(Ident); this.MANY(() => this.SUBRULE(this.field)); this.CONSUME(EOF); });
  field = this.RULE("field", () => {
    this.CONSUME(Ident);
    this.OPTION(() => {
      this.CONSUME(Equal);
      if (nullable) this.OPTION2(() => this.SUBRULE(this.value));
      else this.SUBRULE2(this.value);
    });
  });
  scalar = this.RULE("scalar", () => this.OR([
    { ALT: () => this.SUBRULE(this.field) }, { ALT: () => this.CONSUME(Text) },
    { ALT: () => this.CONSUME(Int) }, { ALT: () => this.CONSUME(Float) },
  ]));
  value = this.RULE("value", () => this.OR([
    { ALT: () => { this.SUBRULE(this.scalar); this.OPTION(() => { this.CONSUME(Comma); this.SUBRULE(this.value); }); } },
    { ALT: () => { this.CONSUME(OpenList); this.MANY(() => this.SUBRULE2(this.value)); this.CONSUME(CloseList); } },
    { ALT: () => { this.CONSUME(OpenRecord); this.MANY2(() => this.SUBRULE(this.field)); this.CONSUME(CloseRecord); } },
  ]));
  constructor() { super(tokens, { recoveryEnabled: false }); this.performSelfAnalysis(); }
}

test("closed DWG empty-record corpus retains the literal zero-field authority", () => {
  const db = new Database(":memory:");
  try {
    const row = db.query("select json_extract(?1,'$.record.fieldCount') as fields,json_array_length(?1,'$.cases') as cases").get(JSON.stringify(corpus)) as { fields: number; cases: number };
    expect(row).toEqual({ fields: 0, cases: 15 });
    expect(new Set(corpus.cases.map((row: { id: string }) => row.id)).size).toBe(15);
  } finally { db.close(); }
});

test("independent Chevrotain recognizes the authored empty-record grammar", () => {
  const parser = new GrammarOracle();
  for (const row of corpus.cases) {
    const result = lexer.tokenize(row.text);
    parser.input = result.tokens;
    parser.document();
    expect(result.errors.length === 0 && parser.errors.length === 0, row.id).toBe(row.accepted);
  }
  console.log(`dwg:grammar-shape cases=${corpus.cases.length} emptyRecordFields=${corpus.record.fieldCount} oracle=Chevrotain`);
});
