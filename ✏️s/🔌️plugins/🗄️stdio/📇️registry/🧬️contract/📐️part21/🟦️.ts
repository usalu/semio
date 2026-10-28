/** 📐️ Schema-neutral ISO 10303-21 values, bounded parser and canonical writer. */
export type Part21Value =
  | {kind: "unset"}
  | {kind: "derived"}
  | {kind: "integer"; value: bigint}
  | {kind: "reference"; value: bigint}
  | {kind: "real"; value: string}
  | {kind: "string"; value: string}
  | {kind: "enum"; value: string}
  | {kind: "binary"; value: string}
  | {kind: "aggregate"; value: Part21Value[]}
  | {kind: "typedValue"; value: {name: string; items: Part21Value[]}};
export interface Part21Record {name: string; args: Part21Value[]}
export interface Part21Instance {id: bigint; entities: Part21Record[]}
export interface Part21Header {fileDescription: Part21Value[]; fileName: Part21Value[]; fileSchema: Part21Value[]}
export interface Part21Document {header: Part21Header; instances: Part21Instance[]}
export interface Part21Options {
  maxWork?: number;
  maxDepth?: number;
  maxInputLength?: number;
  /** 💬️ Observe actual comment ranges in the original UTF-16 source without copying them. */
  onComment?: (start: number, end: number) => void;
  progress?: (work: number, budget: number) => boolean;
}

const U64_MAX = 18446744073709551615n;
const I64_MIN = -9223372036854775808n;
const I64_MAX = 9223372036854775807n;
const REAL = /^[+-]?\d+\.\d*(?:E[+-]?\d+)?$/i;
const NAME = /^[A-Z_][A-Z0-9_]*$/;

/** 🧷️ Canonical Part-21 binary spelling: zero unused high bits in the first hexadecimal nibble. */
export function isPart21Binary(value: string): boolean {
  if (!/^[0-3][0-9A-F]*$/.test(value)) return false;
  if (value.length === 1) return value === "0";
  return Number.parseInt(value[1]!, 16) < 2 ** (4 - Number(value[0]));
}

/** 🚦️ A single owned work/depth/cancellation budget shared across exchange processing phases. */
export class Part21Control {
  work = 0;
  readonly budget: number;
  readonly depth: number;
  private reported = -256;
  constructor(readonly options: Part21Options) {
    this.budget = options.maxWork ?? 64_000_000;
    this.depth = options.maxDepth ?? 64;
    if (!Number.isSafeInteger(this.budget) || this.budget < 1 || !Number.isSafeInteger(this.depth) || this.depth < 1) throw new Error("part21: invalid work or depth limit");
    this.tick(0);
  }
  tick(cost = 1): void {
    this.work += cost;
    if (this.work > this.budget) throw new Error("part21: work limit");
    if (this.work - this.reported >= 256) {
      this.reported = this.work;
      if (this.options.progress?.(this.work, this.budget) === false) throw new Error("part21: cancelled");
    }
  }
  checkDepth(depth: number): void {if (depth > this.depth) throw new Error("part21: depth limit"); this.tick();}
  finish(): void {
    this.reported = this.work;
    if (this.options.progress?.(this.work, this.budget) === false) throw new Error("part21: cancelled");
  }
}

class Reader {
  at = 0;
  readonly control: Part21Control;
  constructor(readonly text: string, readonly options: Part21Options, control?: Part21Control) {
    if (text.length > (options.maxInputLength ?? 16_000_000)) throw new Error("part21: input length limit");
    this.control = control ?? new Part21Control(options);
  }
  fail(reason: string): never {throw new Error("part21: " + reason + " at " + this.at);}
  skip(): void {
    for (;;) {
      while (/\s/.test(this.text[this.at] ?? "") && this.at < this.text.length) {this.at++; this.control.tick();}
      if (!this.text.startsWith("/*", this.at)) return;
      const start = this.at; this.at += 2;
      while (!this.text.startsWith("*/", this.at)) {
        if (this.at >= this.text.length) this.fail("unterminated comment");
        this.at++; this.control.tick();
      }
      this.at += 2; this.options.onComment?.(start, this.at);
    }
  }
  take(token: string): boolean {this.skip(); if (!this.text.startsWith(token, this.at)) return false; this.at += token.length; this.control.tick(token.length); return true;}
  expect(token: string): void {if (!this.take(token)) this.fail("expected " + token);}
  name(): string {
    this.skip(); const match = /^[A-Z_][A-Z0-9_]*/.exec(this.text.slice(this.at));
    if (!match) this.fail("expected entity keyword");
    this.at += match[0].length; this.control.tick(match[0].length); return match[0];
  }
  string(): string {
    this.expect("'"); let result = ""; let page = "A";
    while (this.at < this.text.length) {
      const char = this.text[this.at++]!; this.control.tick();
      if (char === "'") {if (this.text[this.at] === "'") {this.at++; result += "'"; continue;} return result;}
      if (char !== "\\") {result += char; continue;}
      const directive = this.text[this.at++]!;
      if (directive === "\\") {result += "\\"; continue;}
      if (directive === "P") {
        page = this.text[this.at++]!;
        if (!/^[A-I]$/.test(page) || this.text[this.at++] !== "\\") this.fail("malformed alphabet directive");
        continue;
      }
      if (directive === "S") {
        if (this.text[this.at++] !== "\\" || this.at >= this.text.length) this.fail("malformed shifted character");
        if (page !== "A") this.fail("unsupported ISO-8859 alphabet " + page);
        const shifted = this.text.charCodeAt(this.at++);
        if (shifted >= 128) this.fail("shifted character operand must be ASCII");
        result += String.fromCharCode(shifted + 128); continue;
      }
      if (directive !== "X") this.fail("unsupported string directive");
      const width = this.text[this.at++]!;
      if (width === "\\") {result += String.fromCharCode(this.hex(2)); continue;}
      if ((width !== "2" && width !== "4") || this.text[this.at++] !== "\\") this.fail("malformed Unicode directive");
      const units: number[] = [];
      while (!this.text.startsWith("\\X0\\", this.at)) {units.push(this.hex(width === "2" ? 4 : 8));}
      this.at += 4;
      if (units.length === 0) this.fail("empty Unicode directive");
      for (let i = 0; i < units.length; i++) {
        let scalar = units[i]!;
        if (width === "2" && scalar >= 0xd800 && scalar <= 0xdbff) {
          const low = units[++i];
          if (low === undefined || low < 0xdc00 || low > 0xdfff) this.fail("unpaired Unicode surrogate");
          scalar = 0x10000 + ((scalar - 0xd800) << 10) + low - 0xdc00;
        }
        if (scalar > 0x10ffff || scalar >= 0xd800 && scalar <= 0xdfff) this.fail("invalid Unicode scalar");
        result += String.fromCodePoint(scalar);
      }
    }
    return this.fail("unterminated string");
  }
  hex(width: number): number {
    const value = this.text.slice(this.at, this.at + width);
    if (value.length !== width || !/^[0-9A-F]+$/i.test(value)) this.fail("invalid hex group");
    this.at += width; this.control.tick(width); return Number.parseInt(value, 16);
  }
  natural(): bigint {
    this.skip(); const match = /^\d+/.exec(this.text.slice(this.at));
    if (!match) this.fail("expected positive identifier");
    this.at += match[0].length; this.control.tick(match[0].length);
    const value = BigInt(match[0]); if (value < 1n || value > U64_MAX) this.fail("identifier outside u64"); return value;
  }
  list(depth: number): Part21Value[] {
    this.control.checkDepth(depth); this.expect("("); const result: Part21Value[] = [];
    if (this.take(")")) return result;
    do {result.push(this.value(depth + 1));} while (this.take(","));
    this.expect(")"); return result;
  }
  value(depth: number): Part21Value {
    this.control.checkDepth(depth); this.skip(); const char = this.text[this.at];
    if (this.take("$")) return {kind: "unset"};
    if (this.take("*")) return {kind: "derived"};
    if (this.take("#")) return {kind: "reference", value: this.natural()};
    if (char === "'") return {kind: "string", value: this.string()};
    if (char === "(") return {kind: "aggregate", value: this.list(depth)};
    if (this.take(".")) {const value = this.name(); this.expect("."); return {kind: "enum", value};}
    if (this.take('"')) {
      const start = this.at;
      while (this.at < this.text.length && this.text[this.at] !== '"') {this.at++; this.control.tick();}
      const value = this.text.slice(start, this.at); this.expect('"');
      if (!isPart21Binary(value)) this.fail("invalid binary literal");
      return {kind: "binary", value};
    }
    if (char !== undefined && /[+\-0-9]/.test(char)) {
      const match = /^[+-]?\d+(?:\.\d*(?:E[+-]?\d+)?)?/i.exec(this.text.slice(this.at));
      if (!match) this.fail("invalid number");
      const value = match[0]; this.at += value.length; this.control.tick(value.length);
      if (value.includes(".")) {if (!REAL.test(value)) this.fail("invalid real"); return {kind: "real", value};}
      const integer = BigInt(value); if (integer < I64_MIN || integer > I64_MAX) this.fail("integer outside i64");
      return {kind: "integer", value: integer};
    }
    const name = this.name(); return {kind: "typedValue", value: {name, items: this.list(depth)}};
  }
  record(): Part21Record {const name = this.name(); return {name, args: this.list(0)};}
}

/** 📥️ Parses one complete exchange structure with every value and complex record retained. */
export function parsePart21(text: string, options: Part21Options = {}, control?: Part21Control): Part21Document {
  const reader = new Reader(text, options, control);
  reader.expect("ISO-10303-21"); reader.expect(";"); reader.expect("HEADER"); reader.expect(";");
  const header: Part21Header = {fileDescription: [], fileName: [], fileSchema: []};
  for (const [name, key] of [["FILE_DESCRIPTION", "fileDescription"], ["FILE_NAME", "fileName"], ["FILE_SCHEMA", "fileSchema"]] as const) {
    const record = reader.record(); if (record.name !== name) reader.fail("expected mandatory header " + name);
    header[key] = record.args; reader.expect(";");
  }
  reader.expect("ENDSEC"); reader.expect(";"); reader.expect("DATA"); reader.expect(";");
  const instances: Part21Instance[] = [];
  while (!reader.take("ENDSEC")) {
    reader.expect("#"); const id = reader.natural(); reader.expect("="); const entities: Part21Record[] = [];
    if (reader.take("(")) {
      while (!reader.take(")")) entities.push(reader.record());
      if (entities.length === 0) reader.fail("empty complex instance");
    } else entities.push(reader.record());
    reader.expect(";"); instances.push({id, entities});
  }
  reader.expect(";"); reader.expect("END-ISO-10303-21"); reader.expect(";"); reader.skip();
  if (reader.at !== text.length) reader.fail("trailing tokens after exchange terminator");
  reader.control.finish();
  return {header, instances};
}

/** 📤️ Writes an exchange structure from owned values with exact integer/decimal spelling. */
function part21TextWriter(control: Part21Control) {
  const keyword = (value: string): string => {if (!NAME.test(value)) throw new Error("part21: invalid keyword"); return value;};
  const string = (value: string): string => {
    let text = "'";
    for (const char of value) {
      control.tick(); const scalar = char.codePointAt(0)!;
      if (scalar >= 0xd800 && scalar <= 0xdfff) throw new Error("part21: unpaired Unicode surrogate");
      text += char === "'" ? "''" : char === "\\" ? "\\\\" : scalar >= 32 && scalar <= 126 ? char : "\\X4\\" + scalar.toString(16).toUpperCase().padStart(8, "0") + "\\X0\\";
    }
    return text + "'";
  };
  const list = (items: Part21Value[], depth: number): string => {control.checkDepth(depth); return "(" + items.map(item => value(item, depth + 1)).join(",") + ")";};
  const value = (item: Part21Value, depth: number): string => {
    control.checkDepth(depth);
    switch (item.kind) {
      case "unset": return "$";
      case "derived": return "*";
      case "string": return string(item.value);
      case "enum": return "." + keyword(item.value) + ".";
      case "binary": if (!isPart21Binary(item.value)) throw new Error("part21: invalid binary literal"); return '"' + item.value + '"';
      case "integer": if (item.value < I64_MIN || item.value > I64_MAX) throw new Error("part21: integer outside i64"); return item.value.toString();
      case "reference": if (item.value < 1n || item.value > U64_MAX) throw new Error("part21: reference outside u64"); return "#" + item.value;
      case "real": if (!REAL.test(item.value)) throw new Error("part21: invalid decimal real"); return item.value;
      case "aggregate": return list(item.value, depth);
      case "typedValue": return keyword(item.value.name) + list(item.value.items, depth);
    }
  };
  const record = (item: Part21Record): string => keyword(item.name) + list(item.args, 0);
  const instance = (instance: Part21Instance): string => {
    control.tick(); if (instance.id < 1n || instance.id > U64_MAX || instance.entities.length === 0) throw new Error("part21: invalid instance");
    return "#" + instance.id + "=" + (instance.entities.length === 1 ? record(instance.entities[0]!) : "(" + instance.entities.map(record).join(" ") + ")") + ";";
  };
  const header = (header: Part21Header): string => "ISO-10303-21;\nHEADER;\n" + record({name: "FILE_DESCRIPTION", args: header.fileDescription}) + ";\n" + record({name: "FILE_NAME", args: header.fileName}) + ";\n" + record({name: "FILE_SCHEMA", args: header.fileSchema}) + ";\nENDSEC;\nDATA;\n";
  return {header, instance};
}

/** 📤️ Regenerates a canonical exchange through the same fragment writer used by streaming callers. */
export function writePart21(document: Part21Document, options: Part21Options = {}, control = new Part21Control(options)): string {
  const writer = part21TextWriter(control), records = document.instances.map(writer.instance); const output = writer.header(document.header) + records.join("\n") + "\n" + writePart21Footer(); control.finish(); return output;
}
/** 🧵️ Writes canonical opening records through DATA without retaining source instances. */
export function writePart21Header(header: Part21Header, options: Part21Options = {}, control = new Part21Control(options)): string { const output = part21TextWriter(control).header(header); control.finish(); return output; }
/** 📄️ Writes one canonical bounded entity using the shared value and Unicode grammar. */
export function writePart21Instance(instance: Part21Instance, options: Part21Options = {}, control = new Part21Control(options)): string { const output = part21TextWriter(control).instance(instance) + "\n"; control.finish(); return output; }
/** 🏁️ Writes the DATA and exchange terminators. */
export function writePart21Footer(): string { return "ENDSEC;\nEND-ISO-10303-21;\n"; }
