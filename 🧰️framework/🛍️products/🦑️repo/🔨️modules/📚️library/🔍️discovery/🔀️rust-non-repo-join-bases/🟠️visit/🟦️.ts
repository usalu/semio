/** 🟠️ The visit phase of inspectRustNonRepoJoinBaseSpans. */
import { rustFindTopLevel, rustTokenSegments } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import type { buildTokensState } from "../🔶️tokens/🟦️.ts";
import type { buildResolveQualifiedFunctionState } from "../🔸️resolve-qualified-function/🟦️.ts";
import type { Binding } from "../🟢️harness/🟦️.ts";

/** 🟡 Builds visit of inspectRustNonRepoJoinBaseSpans. */
export function buildVisit({ tokens, pairs, tupleCallNonRepoPositions, rootEnd, walkJoinChain }: Pick<ReturnType<typeof buildTokensState>, "tokens" | "pairs" | "rootEnd" | "walkJoinChain"> & Pick<ReturnType<typeof buildResolveQualifiedFunctionState>, "tupleCallNonRepoPositions">) {
  /** 🚶️ Walks one token range with its non-repo bindings, recording every literal join chain off a proven non-repo root. A tuple-pattern destructure `let (a, _, c) = qualifier::fn(..);` binds each named (non-`_`) position whose slot is independently proven non-repo by `tupleCallNonRepoPositions`, and an unrecognized shape binds nothing (the prior, always-safe default). A plain `let` proves its right-hand side against the OLD binding of the name (self-shadowing, e.g. `let out_dir = PathBuf::from(out_dir)`) before clearing it, as `bodyReturnsNonRepo` does: deleting first would make the reference to the shadowed value unprovable. */
  const visit = (start: number, end: number, bindings: Map<string, Binding>): void => {
    for (let index = start; index < end;) {
      const token = tokens[index]!;
      if (tokens[index + 1]?.text === "!" && ["(", "[", "{"].includes(tokens[index + 2]?.text ?? "")) {
        index = (pairs.get(index + 2) ?? end - 1) + 1;
        continue;
      }
      if (token.text === "fn") {
        const paramsOpen = rustFindTopLevel(tokens, pairs, index + 1, end, new Set(["("]));
        const paramsClose = paramsOpen < 0 ? undefined : pairs.get(paramsOpen);
        const braceStart = paramsClose === undefined ? -1 : rustFindTopLevel(tokens, pairs, paramsClose + 1, end, new Set(["{", ";"]));
        const braceEnd = braceStart >= 0 && tokens[braceStart]?.text === "{" ? pairs.get(braceStart) : undefined;
        if (paramsClose !== undefined && braceEnd !== undefined) {
          const fresh = new Map<string, Binding>();
          for (const [first, last] of rustTokenSegments(tokens, pairs, paramsOpen + 1, paramsClose, ",")) {
            const nameIndex = tokens[first]?.text === "mut" ? first + 1 : first;
            if (tokens[nameIndex]?.kind === "identifier" && tokens[nameIndex + 1]?.text === ":" && nameIndex < last) fresh.set(tokens[nameIndex]!.text, { kind: "nonrepo" });
          }
          visit(braceStart + 1, braceEnd, fresh);
        }
        index = braceStart < 0 ? (paramsClose === undefined ? index + 1 : paramsClose + 1) : (braceEnd ?? braceStart) + 1;
        continue;
      }
      if (token.text === "let") {
        const boundary = rustFindTopLevel(tokens, pairs, index + 1, end, new Set([";"])), stop = boundary < 0 ? end : boundary;
        if (tokens[index + 1]?.text === "(") {
          const patternClose = pairs.get(index + 1);
          const elements = patternClose === undefined ? [] : rustTokenSegments(tokens, pairs, index + 2, patternClose, ",");
          const names: (string | null)[] = [];
          let patternOk = patternClose !== undefined && patternClose < stop && tokens[patternClose + 1]?.text === "=" && elements.length > 0;
          if (patternOk) for (const [first, last] of elements) {
            if (last === first + 1 && tokens[first]?.text === "_") { names.push(null); continue; }
            if (last === first + 1 && tokens[first]?.kind === "identifier") { names.push(tokens[first]!.text); continue; }
            if (last === first + 2 && tokens[first]?.text === "mut" && tokens[first + 1]?.kind === "identifier") { names.push(tokens[first + 1]!.text); continue; }
            patternOk = false; break;
          }
          if (patternOk) {
            for (const name of names) if (name) bindings.delete(name);
            const positions = tupleCallNonRepoPositions(patternClose! + 2, stop, names.length);
            if (positions) for (let position = 0; position < names.length; position++) { const name = names[position]; if (name && positions[position]) bindings.set(name, { kind: "nonrepo" }); }
          }
          index = boundary < 0 ? end : boundary + 1;
          continue;
        }
        const equal = rustFindTopLevel(tokens, pairs, index + 1, stop, new Set(["="]));
        const nameIndex = tokens[index + 1]?.text === "mut" ? index + 2 : index + 1, nameToken = tokens[nameIndex];
        if (equal >= 0 && nameToken?.kind === "identifier" && nameIndex + 1 === equal) {
          const rootAfter = rootEnd(equal + 1, stop, bindings, 1);
          bindings.delete(nameToken.text);
          if (rootAfter !== null) { bindings.set(nameToken.text, { kind: "nonrepo" }); walkJoinChain(rootAfter, stop); }
        } else if (nameToken?.kind === "identifier") bindings.delete(nameToken.text);
        index = boundary < 0 ? end : boundary + 1;
        continue;
      }
      const rootAfter = rootEnd(index, end, bindings, 1);
      if (rootAfter !== null) { index = walkJoinChain(rootAfter, end); continue; }
      const close = pairs.get(index);
      if (close !== undefined && close > index) { visit(index + 1, close, token.text === "{" ? new Map(bindings) : bindings); index = close + 1; }
      else index++;
    }
  };
  visit(0, tokens.length, new Map());
}
