/** ♿️ The structural ARIA oracle run over the band, the history panel and the prompt. */
import { join } from "node:path";
import { evalSafe } from "../📄️page/🟦️.ts";
import { page, renderer } from "../🧠️state/🟦️.ts";
import { log, verdict } from "../🧾️report/🟦️.ts";

//#region 🔖️Band
//#region 🔖️AriaOracle

/** ♿️ The structural ARIA oracle as one browser module: `aria-query` (the WAI-ARIA role model: every role's supported
 * attributes, every `aria-*` attribute that exists) and `dom-accessibility-api` (accessible names), the third-party pair the
 * React time-travel laws run in jsdom, here bundled once per run and evaluated into the live page (CDP evaluation, so no page
 * CSP applies). */
const ARIA_ORACLE_SOURCE = 'import { computeAccessibleName, getRole } from "dom-accessibility-api"; import { aria, roles } from "aria-query"; (globalThis as unknown as { __probeAria?: unknown }).__probeAria = { computeAccessibleName, getRole, aria, roles };';

let ariaOracleBundle: string | null = null;

/** 📦️ Bundles {@link ARIA_ORACLE_SOURCE} into one IIFE (resolved from this module's directory, so the repository's installed
 * copies answer); cached for the run. */
const ariaOracle = async () => {
  if (ariaOracleBundle !== null) return ariaOracleBundle;
  const entry = join(import.meta.dir, "aria-oracle.ts");
  const built = await Bun.build({
    entrypoints: [entry],
    target: "browser",
    format: "iife",
    minify: true,
    plugins: [{ name: "time-travel-aria-oracle", setup: (build) => {
      build.onResolve({ filter: /aria-oracle\.ts$/ }, () => ({ path: entry, namespace: "time-travel-aria-oracle" }));
      build.onLoad({ filter: /.*/, namespace: "time-travel-aria-oracle" }, () => ({ contents: ARIA_ORACLE_SOURCE, loader: "ts" }));
    } }],
  });
  if (!built.success || !built.outputs[0]) throw new Error(`aria oracle bundle failed: ${built.logs.map(String).join("; ").slice(0, 400)}`);
  ariaOracleBundle = await built.outputs[0].text();
  return ariaOracleBundle;
};

/** 🧩️ Evaluates the oracle bundle into the current page unless it is already there (a reload or a new page clears it). */
const ensureAriaOracle = async () => {
  if (await evalSafe(() => Boolean((window as unknown as { __probeAria?: unknown }).__probeAria), false)) return true;
  const bundle = await ariaOracle();
  await page.evaluate(bundle).catch((error) => log(`aria oracle evaluate failed ${String(error).split("\n")[0]}`));
  return evalSafe(() => Boolean((window as unknown as { __probeAria?: unknown }).__probeAria), false);
};

type AriaReport = { oracle: boolean; roots: number; checked: number; findings: string[] };

/** 🦮️ The axe-level structural findings under the elements `selectors` match (the W2-B law's rules, in the live page): an
 * unknown `aria-*` attribute, one the element's role does not support, a `labelledby`/`describedby` naming no element, a
 * control (button, field, slider, spinbutton, combobox, tree item, tab, menu item, option, checkbox, switch, link) without an
 * accessible name, and an id repeated under the roots. Hidden subtrees (`aria-hidden`, `hidden`, no layout box) are skipped. */
export const ariaFindings = async (selectors: string[]): Promise<AriaReport> => {
  const oracle = await ensureAriaOracle();
  if (!oracle) return { oracle, roots: 0, checked: 0, findings: ["the aria oracle is not in the page"] };
  return evalSafe(
    (roots) => {
      type Oracle = { computeAccessibleName: (element: Element) => string; getRole: (element: Element) => string | null; aria: ReadonlyMap<string, unknown>; roles: ReadonlyMap<string, { readonly props: Readonly<Record<string, unknown>> }> };
      const oracleApi = (window as unknown as { __probeAria: Oracle }).__probeAria;
      const findings: string[] = [];
      const ids = new Map<string, number>();
      const elements = roots.flatMap((selector) => Array.from(document.querySelectorAll<HTMLElement>(selector)));
      const seen = new Set<Element>();
      let checked = 0;
      for (const root of elements) {
        for (const element of [root, ...Array.from(root.querySelectorAll<HTMLElement>("*"))]) {
          if (seen.has(element)) continue;
          seen.add(element);
          if (element.closest('[aria-hidden="true"], [hidden]')) continue;
          checked += 1;
          if (element.id) ids.set(element.id, (ids.get(element.id) ?? 0) + 1);
          const role = oracleApi.getRole(element);
          const where = `${role ?? element.tagName.toLowerCase()}#${element.id || element.getAttribute("data-node-key") || ""}`;
          for (const { name, value } of Array.from(element.attributes)) {
            if (!name.startsWith("aria-")) continue;
            if (!oracleApi.aria.has(name)) findings.push(`${where}: unknown ${name}`);
            else if (role !== null && oracleApi.roles.get(role) !== undefined && !(name in oracleApi.roles.get(role)!.props) && name !== "aria-hidden") findings.push(`${where}: ${name} not supported`);
            if ((name === "aria-describedby" || name === "aria-labelledby") && value.split(/\s+/u).some((id) => id !== "" && document.getElementById(id) === null)) findings.push(`${where}: ${name} → missing ${value}`);
          }
          const control = element.matches('button, input:not([type="hidden"]), select, textarea, [role="slider"], [role="combobox"], [role="spinbutton"], [role="treeitem"], [role="tab"], [role="menuitem"], [role="option"], [role="checkbox"], [role="switch"], [role="link"]');
          if (control && element.getClientRects().length > 0 && oracleApi.computeAccessibleName(element).trim() === "") findings.push(`${where}: no accessible name`);
        }
      }
      for (const [id, count] of ids) if (count > 1) findings.push(`duplicate id ${id}`);
      return { oracle: true, roots: elements.length, checked, findings };
    },
    { oracle: true, roots: 0, checked: 0, findings: ["the aria oracle threw in the page"] } as AriaReport,
    selectors,
  );
};

/** 🪴️ The roots an ARIA verdict checks: React's band, History panel and open prompt; wgpu's whole ARIA mirror (which projects
 * only the prompt while one is open). */
export const ariaRoots = (kind: "session" | "prompt") =>
  renderer === "wgpu" ? ["#semio-wgpu-accessibility"] : kind === "prompt" ? ['[role="dialog"]'] : ["[data-semio-time-travel]", '[id="framework.panelTab.framework.panel.history"]'];

/** 🏛️ One structural-ARIA verdict over `kind`'s roots: no finding, and the roots exist. */
export const ariaVerdict = async (name: string, kind: "session" | "prompt") => {
  const report = await ariaFindings(ariaRoots(kind));
  return verdict(name, report.oracle && report.roots > 0 && report.findings.length === 0, { ...report, findings: report.findings.slice(0, 24), selectors: ariaRoots(kind), oracle: "aria-query (role model) + dom-accessibility-api (accessible names), bundled into the page" });
};
//#endregion 🔖️AriaOracle
//#endregion 🔖️Band
