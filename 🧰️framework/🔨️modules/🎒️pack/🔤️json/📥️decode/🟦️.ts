import { JsonMemberPolicy } from "../🧩️members/🟦️.ts";

/** 🌳️ Lossless JSON syntax: number spelling and decoded string code units remain owned data. */
export type JsonSyntaxNode =
  | { readonly kind: "null" }
  | { readonly kind: "boolean"; readonly value: boolean }
  | { readonly kind: "string"; readonly value: string }
  | { readonly kind: "number"; readonly text: string }
  | { readonly kind: "array"; readonly items: readonly JsonSyntaxNode[] }
  | { readonly kind: "object"; readonly members: readonly JsonSyntaxMember[] };
/** 🔑️ Stores a decoded member name without prototype lookup or numeric coercion. */
export interface JsonSyntaxMember { readonly name: string; readonly value: JsonSyntaxNode }
/** 📊️ Counts raw UTF-8 bytes consumed and syntax values admitted. */
export interface JsonSyntaxProgress { readonly bytes: number; readonly nodes: number }
/** 🛑️ Provides finite limits and the host scheduling boundary for an asynchronous reader. */
export interface JsonSyntaxControl { readonly maximumBytes: number; readonly maximumNodes: number; readonly maximumDepth: number; readonly chunk: number; readonly cancelled: () => boolean; readonly progress: (value: JsonSyntaxProgress) => void; readonly yield: () => Promise<void> }
/** 🏷️ Enumerates the reader's closed refusal vocabulary. */
export type JsonSyntaxErrorCode = "byte-budget" | "cancelled" | "depth-budget" | "duplicate-member" | "invalid-control" | "invalid-policy" | "invalid-source" | "invalid-unicode" | "malformed-json" | "node-budget";
/** 🚨️ Identifies a reader refusal and its exact raw-source code-unit offset. */
export class JsonSyntaxError extends Error {
  constructor(readonly code: JsonSyntaxErrorCode, readonly offset: number, message: string) { super(message); this.name = "JsonSyntaxError"; }
}

function reject(code: JsonSyntaxErrorCode, offset: number, message: string): never { throw new JsonSyntaxError(code, offset, message); }
function capture(value: JsonSyntaxControl): JsonSyntaxControl {
  if (value === null || typeof value !== "object" || ![Object.prototype, null].includes(Object.getPrototypeOf(value))) reject("invalid-control", 0, "Expected owned operation data");
  const keys = ["maximumBytes", "maximumNodes", "maximumDepth", "chunk", "cancelled", "progress", "yield"], descriptors = Object.getOwnPropertyDescriptors(value);
  if (Reflect.ownKeys(value).length !== keys.length || keys.some(key => !Object.hasOwn(descriptors, key))) reject("invalid-control", 0, "Unexpected operation members");
  const snapshot: Record<string, unknown> = Object.create(null);
  for (const key of keys) { const descriptor = descriptors[key]!; if (!Object.hasOwn(descriptor, "value") || !descriptor.enumerable) reject("invalid-control", 0, "Operation accessors are not data"); snapshot[key] = descriptor.value; }
  for (const key of keys.slice(0, 4)) if (!Number.isSafeInteger(snapshot[key]) || (snapshot[key] as number) < 1 || (key === "chunk" && (snapshot[key] as number) > 4096)) reject("invalid-control", 0, "Invalid finite operation limit");
  for (const key of keys.slice(4)) if (typeof snapshot[key] !== "function") reject("invalid-control", 0, "Expected operation callback");
  return snapshot as unknown as JsonSyntaxControl;
}

class JsonSyntaxReader {
  #offset = 0;
  #bytes = 0;
  #nodes = 0;
  #chunk = 0;
  constructor(readonly source: string, readonly policy: JsonMemberPolicy, readonly control: JsonSyntaxControl) {}
  #check(): void { if (this.control.cancelled()) reject("cancelled", this.#offset, "JSON operation cancelled"); }
  #emit(): void { this.#check(); this.control.progress(Object.freeze({ bytes: this.#bytes, nodes: this.#nodes })); }
  #peek(): string { return this.source[this.#offset] ?? ""; }
  async #take(): Promise<string> {
    this.#check();
    if (this.#offset >= this.source.length) reject("malformed-json", this.#offset, "Unexpected end of source");
    const unit = this.source.charCodeAt(this.#offset), next = this.source.charCodeAt(this.#offset + 1), previous = this.source.charCodeAt(this.#offset - 1);
    let bytes = unit < 0x80 ? 1 : unit < 0x800 ? 2 : 3;
    if (unit >= 0xd800 && unit <= 0xdbff) { if (!(next >= 0xdc00 && next <= 0xdfff)) reject("invalid-unicode", this.#offset, "Raw source has an unpaired surrogate"); bytes = 4; }
    else if (unit >= 0xdc00 && unit <= 0xdfff) { if (!(previous >= 0xd800 && previous <= 0xdbff)) reject("invalid-unicode", this.#offset, "Raw source has an unpaired surrogate"); bytes = 0; }
    if (this.#bytes + bytes > this.control.maximumBytes) reject("byte-budget", this.#offset, "Raw byte limit reached");
    const result = this.source[this.#offset++]!; this.#bytes += bytes;
    if (++this.#chunk >= this.control.chunk) { this.#chunk = 0; this.#emit(); await this.control.yield(); this.#check(); }
    return result;
  }
  async #space(): Promise<void> { while ([" ", "\t", "\n", "\r"].includes(this.#peek())) await this.#take(); }
  async #expect(expected: string): Promise<void> { if (this.#peek() !== expected) reject("malformed-json", this.#offset, "Unexpected source token"); await this.#take(); }
  async #string(): Promise<string> {
    await this.#expect('"');
    const parts: string[] = [];
    while (this.#peek() !== '"') {
      const token = await this.#take();
      if (token === "\\") {
        const escaped = await this.#take();
        if (escaped === "u") { let hex = ""; for (let index = 0; index < 4; index++) { const digit = await this.#take(); if (!/^[0-9a-fA-F]$/.test(digit)) reject("malformed-json", this.#offset - 1, "Invalid Unicode escape"); hex += digit; } parts.push(String.fromCharCode(Number.parseInt(hex, 16))); }
        else { const escapes: Record<string, string> = { '"': '"', "\\": "\\", "/": "/", b: "\b", f: "\f", n: "\n", r: "\r", t: "\t" }; if (!Object.hasOwn(escapes, escaped)) reject("malformed-json", this.#offset - 1, "Invalid string escape"); parts.push(escapes[escaped]!); }
      } else { if (token.charCodeAt(0) < 0x20) reject("malformed-json", this.#offset - 1, "Unescaped string control"); parts.push(token); }
    }
    await this.#take();
    return parts.join("");
  }
  async #digits(): Promise<void> { if (!/^[0-9]$/.test(this.#peek())) reject("malformed-json", this.#offset, "Number needs a digit"); while (/^[0-9]$/.test(this.#peek())) await this.#take(); }
  async #number(): Promise<JsonSyntaxNode> {
    const start = this.#offset;
    if (this.#peek() === "-") await this.#take();
    if (this.#peek() === "0") await this.#take(); else { if (!/^[1-9]$/.test(this.#peek())) reject("malformed-json", this.#offset, "Invalid number"); await this.#digits(); }
    if (this.#peek() === ".") { await this.#take(); await this.#digits(); }
    if (this.#peek() === "e" || this.#peek() === "E") { await this.#take(); if (this.#peek() === "+" || this.#peek() === "-") await this.#take(); await this.#digits(); }
    return { kind: "number", text: this.source.slice(start, this.#offset) };
  }
  async #value(depth: number): Promise<JsonSyntaxNode> {
    this.#check();
    if (depth > this.control.maximumDepth) reject("depth-budget", this.#offset, "Syntax nesting exceeds limit");
    if (this.#nodes >= this.control.maximumNodes) reject("node-budget", this.#offset, "Syntax values exceed limit");
    this.#nodes++;
    await this.#space();
    const token = this.#peek();
    if (token === '"') return { kind: "string", value: await this.#string() };
    if (token === "[" || token === "{") {
      await this.#take(); await this.#space();
      const items: JsonSyntaxNode[] = [], members: JsonSyntaxMember[] = [], indices = new Map<string, number>(), close = token === "[" ? "]" : "}";
      if (this.#peek() !== close) while (true) {
        if (token === "[") items.push(await this.#value(depth + 1));
        else {
          const name = await this.#string(), previous = indices.get(name);
          if (previous !== undefined && this.policy === JsonMemberPolicy.Reject) reject("duplicate-member", this.#offset, "Decoded object member is repeated");
          await this.#space(); await this.#expect(":");
          const value = await this.#value(depth + 1);
          if (previous !== undefined) members[previous] = { name, value }; else { indices.set(name, members.length); members.push({ name, value }); }
        }
        await this.#space();
        if (this.#peek() === close) break;
        await this.#expect(","); await this.#space();
      }
      await this.#expect(close);
      return token === "[" ? { kind: "array", items } : { kind: "object", members };
    }
    for (const literal of ["null", "true", "false"]) if (token === literal[0]) { for (const char of literal) await this.#expect(char); return literal === "null" ? { kind: "null" } : { kind: "boolean", value: literal === "true" }; }
    return this.#number();
  }
  async read(): Promise<JsonSyntaxNode> {
    this.#emit();
    const value = await this.#value(1);
    await this.#space();
    if (this.#offset !== this.source.length) reject("malformed-json", this.#offset, "Source has trailing tokens");
    this.#emit();
    return value;
  }
}

/** 📥️ Reads lossless syntax under an explicit decoded-member policy and finite caller authority. */
export async function decodeJsonSyntax(source: string, policy: JsonMemberPolicy, control: JsonSyntaxControl): Promise<JsonSyntaxNode> {
  if (typeof source !== "string") reject("invalid-source", 0, "Expected raw JSON text");
  if (policy !== JsonMemberPolicy.Reject && policy !== JsonMemberPolicy.Replace) reject("invalid-policy", 0, "Decoded member policy is mandatory");
  return await new JsonSyntaxReader(source, policy, capture(control)).read();
}
