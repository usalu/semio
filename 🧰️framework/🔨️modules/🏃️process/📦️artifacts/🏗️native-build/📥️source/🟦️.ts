import { createHash } from "node:crypto";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { inspectRustCompileReferences, rustAttributes, rustFindTopLevel, rustStringValue, rustTokenPairs, rustTokenSegments, rustTokens, rustVisibility, type RustGeneratedTokenOutput } from "../../../../📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

/** 📎️ Cfg-selected authored input; compiler dep-info must still qualify generated ownership. */
export interface RustSourceInputV1 { readonly path: string; readonly digest: string; readonly bytes: number; readonly activeBytes: number; readonly activeLines: number; readonly kind: "source" | "embedded"; }

/** 🏷️ What the compilation unit that owns a closure contributes to it: its manifest directory (the value of `env!("CARGO_MANIFEST_DIR")`, so manifest-relative
 * inputs are authored, not generated) and the observer of token output it emits as a generator (a proc-macro's `quote!` carrying `include_str!` belongs to
 * the crates that expand it and is qualified by their compiler dep-info, never counted as an input of the generator). */
export type RustSourceOwnerV1 = Readonly<{ manifestDirectory: string; generated: (file: string, output: RustGeneratedTokenOutput) => void }>;

/** 🧭️ Resolves authored production mounts with exact cfg and refuses missing or escaped ownership. Without an `owner`, manifest-relative inputs and emitted
 * token output are refused; `OUT_DIR` inputs and inputs reached through a local macro expansion always require compiler ownership. */
export async function readRustSourceClosureV1(entries: readonly string[], root: string, cfg: readonly string[], read: (path: string) => Promise<Uint8Array | null>, owner?: RustSourceOwnerV1): Promise<readonly RustSourceInputV1[]> {
  const files = new Map<string, RustSourceInputV1>(), visited = new Set<string>(), decoder = new TextDecoder();
  const visit = async (path: string, base = dirname(path), embedded = false): Promise<void> => {
    const name = relative(root, path).replaceAll("\\", "/");
    if (isAbsolute(name) || name === ".." || name.startsWith("../")) throw Error("Rust source escaped declared workspace: " + path);
    const key = path + "\0" + base + "\0" + (embedded ? "embedded" : "source");
    if (visited.has(key)) return;
    visited.add(key);
    const bytes = await read(path);
    if (bytes === null) throw Error("Missing Rust source input: " + name);
    const selected = embedded ? "" : selectRustSourceV1(decoder.decode(bytes), cfg);
    const digest=createHash("sha256").update(bytes).digest("hex"),prior=files.get(name);if(prior&&prior.digest!==digest)throw Error("Rust source changed during closure: "+name);
    if(!prior||!embedded)files.set(name, { path: name, digest, bytes: bytes.byteLength, activeBytes: embedded ? bytes.byteLength : Buffer.byteLength(selected), activeLines: embedded || !selected.trim() ? 0 : selected.split("\n").length, kind: embedded ? "embedded" : "source" });
    if (embedded) return;
    for (const reference of inspectRustCompileReferences(selected, owner ? (output) => owner.generated(name, output) : undefined)) {
      if (reference.kind === "path") continue;
      if (reference.expansion || reference.base === "generated" || reference.base === "manifest" && !owner) throw Error("Generated Rust input requires compiler ownership: " + name);
      await visit(reference.base === "manifest" ? join(owner!.manifestDirectory, reference.path) : resolve(dirname(path), reference.path), dirname(path), reference.kind !== "include");
    }
    const tokens = rustTokens(selected), pairs = rustTokenPairs(tokens);
    const scope = async (start: number, end: number, moduleBase: string, fileScope: boolean): Promise<void> => {
      for (let index = start; index < end;) {
        if(tokens[index]?.text==="#"&&tokens[index+1]?.text==="!"&&tokens[index+2]?.text==="["){const close=pairs.get(index+2);if(close===undefined)throw Error("Invalid inner Rust attribute");index=close+1;continue;}
        const attrs = rustAttributes(tokens, pairs, index, end), visibility = rustVisibility(tokens, pairs, attrs.next), at = visibility.next;
        if (tokens[at]?.text === "mod" && tokens[at + 1]?.kind === "identifier") {
          const name = tokens[at + 1]!.text.replace(/^r#/u, ""), boundary = at + 2;
          const mounts = attrs.ranges.filter(([a]) => tokens[a]?.text === "path").map(([a, b]) => b === a + 3 && tokens[a + 1]?.text === "=" ? rustStringValue(tokens[a + 2]) : null);
          if (mounts.length > 1 || mounts.includes(null)) throw Error("Ambiguous Rust source mount: " + name);
          const mount = mounts[0] ?? undefined;
          if (tokens[boundary]?.text === "{") {
            const close = pairs.get(boundary)!;
            await scope(boundary + 1, close, resolve(moduleBase, mount ?? name), false);
            index = close + 1; continue;
          }
          if (tokens[boundary]?.text === ";") {
            const candidates = mount === undefined ? [join(moduleBase, name + ".rs"), join(moduleBase, name, "mod.rs")] : [resolve(fileScope ? dirname(path) : moduleBase, mount)];
            const present: string[] = [];
            for (const candidate of candidates) if (await read(candidate) !== null) present.push(candidate);
            if (present.length !== 1) throw Error("Missing or ambiguous Rust module: " + name);
            const child = present[0]!;
            await visit(child, mount === undefined && child.endsWith(name + ".rs") ? join(dirname(child), name) : dirname(child));
            index = boundary + 1; continue;
          }
        }
        const boundary = rustFindTopLevel(tokens, pairs, at, end, new Set([";", "{"]));
        if (boundary < 0) break;
        index = (pairs.get(boundary) ?? boundary) + 1;
      }
    };
    await scope(0, tokens.length, base, true);
  };
  for (const entry of entries) await visit(resolve(root, entry));
  return [...files.values()].sort((a, b) => a.path.localeCompare(b.path));
}

/** 🎛️ Evaluates one Rust conditional predicate against the complete compiler cfg snapshot. */
export function rustCfgMatchesV1(source: string, cfg: readonly string[]): boolean {
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), known = new Set(cfg);
  if (tokens.some((token, index) => ["(", ")", "[", "]", "{", "}"].includes(token.text) && !pairs.has(index))) throw Error("Unbalanced Rust cfg expression");
  const evaluate = (start: number, end: number): boolean => {
    const head = tokens[start];
    if (head?.kind !== "identifier") throw Error("Rust cfg requires an identifier");
    if (tokens[start + 1]?.text === "(") {
      const close = pairs.get(start + 1);
      if (close === undefined || close + 1 !== end || !["all", "any", "not"].includes(head.text)) throw Error("Invalid Rust cfg predicate");
      const ranges = rustTokenSegments(tokens, pairs, start + 2, close, ",").filter(([a, b]) => a < b), values = ranges.map(([a, b]) => evaluate(a, b));
      if (head.text === "not" && values.length !== 1) throw Error("Rust cfg not requires exactly one predicate");
      return head.text === "all" ? values.every(Boolean) : head.text === "any" ? values.some(Boolean) : !values[0];
    }
    if (end === start + 1) {
      if (head.text === "feature") throw Error("Rust cfg feature requires a value");
      return known.has(head.text);
    }
    const value = rustStringValue(tokens[start + 2]);
    if (end !== start + 3 || tokens[start + 1]?.text !== "=" || value === null) throw Error("Invalid Rust cfg equality");
    return known.has(`${head.text}=${value}`);
  };
  return evaluate(0, tokens.length);
}

/** ✂️ Selects authored Rust tokens by compiler cfg; generated code still requires compiler dep-info. A `macro_rules!` body stays authored text:
 * its attributes (`#[cfg($scope)]`) are selected where the macro expands, not where it is defined.
 * @see https://doc.rust-lang.org/reference/macros-by-example.html */
export function selectRustSourceV1(source: string, cfg: readonly string[]): string {
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), edits: { start: number; end: number; text: string }[] = [];
  const enclosure=new Map<number,number>(),stack:number[]=[];for(let index=0;index<tokens.length;index++){if(stack.length)enclosure.set(index,stack.at(-1)!);if(tokens[index]?.text==="{")stack.push(index);else if(tokens[index]?.text==="}")stack.pop();}
  if (tokens.some((token, index) => ["(", ")", "[", "]", "{", "}"].includes(token.text) && !pairs.has(index))) throw Error("Unbalanced Rust source delimiter");
  const attribute = (start: number, end: number): string | null => {
    const head = tokens[start]?.text;
    if (head !== "cfg" && head !== "cfg_attr") return source.slice(tokens[start]!.start, tokens[end - 1]!.end);
    if (tokens[start + 1]?.text !== "(" || pairs.get(start + 1) !== end - 1) throw Error("Invalid Rust conditional attribute");
    const ranges = rustTokenSegments(tokens, pairs, start + 2, end - 1, ",");
    if (head === "cfg") return rustCfgMatchesV1(source.slice(tokens[start + 2]?.start ?? 0, tokens[end - 1]!.start), cfg) ? "" : null;
    if (ranges.length < 2 || !ranges[0] || ranges[0][0] === ranges[0][1]) throw Error("Rust cfg_attr requires predicate and attributes");
    const [a, b] = ranges[0];
    if (!rustCfgMatchesV1(source.slice(tokens[a]!.start, tokens[b - 1]!.end), cfg)) return "";
    const selected = ranges.slice(1).map(([a, b]) => attribute(a, b));
    return selected.includes(null) ? null : selected.filter(Boolean).map(value => `#[${value}]`).join(" ");
  };
  for (let index = 0; index < tokens.length; index++) {
    const transcriber = tokens[index]?.text === "macro_rules" && tokens[index + 1]?.text === "!" && tokens[index + 2]?.kind === "identifier" ? pairs.get(index + 3) : undefined;
    if (transcriber !== undefined) { index = transcriber; continue; }
    if(tokens[index]?.text==="#"&&tokens[index+1]?.text==="!"&&tokens[index+2]?.text==="["){
      const close=pairs.get(index+2);if(close===undefined)throw Error("Invalid inner Rust attribute");const start=index+3;
      if(!["cfg","cfg_attr"].includes(tokens[start]?.text??"")){index=close;continue;}
      const selected=attribute(start,close),open=enclosure.get(index);
      if(open!==undefined&&(tokens[open-2]?.text!=="mod"||tokens[open-1]?.kind!=="identifier"))throw Error("Unsupported inner cfg-owned Rust scope");
      if(selected===null){if(open===undefined)return "";const last=pairs.get(open)!;while(edits.length&&edits.at(-1)!.start>=tokens[open]!.end)edits.pop();edits.push({start:tokens[open]!.end,end:tokens[last]!.start,text:" "});index=last;}
      else{const text=selected?selected.startsWith("#[")?selected.replaceAll("#[","#!["):`#![${selected}]`:"";edits.push({start:tokens[index]!.start,end:tokens[close]!.end,text});index=close;}
      continue;
    }
    if (tokens[index]?.text !== "#" || tokens[index + 1]?.text !== "[") continue;
    const attrs = rustAttributes(tokens, pairs, index, tokens.length);
    if (!attrs.ranges.some(([a]) => ["cfg", "cfg_attr"].includes(tokens[a]?.text ?? ""))) { index = attrs.next - 1; continue; }
    const selected = attrs.ranges.map(([a, b]) => attribute(a, b));
    if (selected.includes(null)) {
      const boundary = rustFindTopLevel(tokens, pairs, attrs.next, tokens.length, new Set([";", ",", "{"]));
      if (boundary < 0) throw Error("Unsupported cfg-owned Rust item");
      let last = pairs.get(boundary) ?? boundary;
      if (tokens[last + 1]?.text === ";" || tokens[last + 1]?.text === ",") last++;
      edits.push({ start: tokens[index]!.start, end: tokens[last]!.end, text: "" });
      index = last;
    } else {
      const text = selected.filter(Boolean).map(value => value!.startsWith("#[") ? value : `#[${value}]`).join(" ");
      edits.push({ start: tokens[index]!.start, end: tokens[attrs.next - 1]!.end, text });
      index = attrs.next - 1;
    }
  }
  let result = source;
  for (const edit of edits.reverse()) result = result.slice(0, edit.start) + edit.text + result.slice(edit.end);
  return result;
}
