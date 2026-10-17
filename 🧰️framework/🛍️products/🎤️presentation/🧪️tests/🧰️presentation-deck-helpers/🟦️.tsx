/** 🧰️ Shared helpers of the presentation react deck suites: the test source locator, the global stylesheet text with its section imports inlined and the stock reveal auto-animate matcher host.
 *
 * @see ../../🎯️targets/⚛️react/🟦️.tsx — the module under test
 */
import type { AutoAnimateMatcherHost } from "../../🎯️targets/⚛️react/🟦️.tsx";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

type TestSource = { readonly directory: string; readonly url: string };

export const source: TestSource = { directory: dirname(fileURLToPath(import.meta.url)), url: import.meta.url };

export const stylesheetRelativePath = "../../📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️.css";

/** 🧵️ Reads a stylesheet with its relative `@import` lines replaced by the imported text, in order. */
const readStylesheetTree = (path: string): string =>
  readFileSync(path, "utf8").replace(/^@import "(\.[^"]+)";$/gmu, (_line, relative: string) => readStylesheetTree(join(dirname(path), relative)));

export const globalsCssSource = readStylesheetTree(join(dirname(fileURLToPath(source.url)), stylesheetRelativePath));

export function stockRevealAutoAnimateMatcherHost(): AutoAnimateMatcherHost {
  return {
    findAutoAnimateMatches(pairs, fromScope, toScope, selector, serializer) {
      const reserved = new Set<HTMLElement>();
      for (const fromElement of fromScope.querySelectorAll<HTMLElement>(selector)) {
        const fromKey = serializer(fromElement);
        for (const toElement of toScope.querySelectorAll<HTMLElement>(selector)) {
          if (reserved.has(toElement) || serializer(toElement) !== fromKey) {
            continue;
          }
          pairs.push({ from: fromElement, to: toElement });
          reserved.add(toElement);
          break;
        }
      }
    },
  };
}
