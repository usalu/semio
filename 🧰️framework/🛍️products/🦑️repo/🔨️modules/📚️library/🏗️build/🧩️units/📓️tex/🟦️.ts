import {mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {sha256V1} from "../🔏️key/🟦️.ts";
import {compareCodePointsV1, sortedCodePointsV1, type UnitDependencyV1, type UnitGraphV1, type UnitsPolicyV1, type UnitV1} from "../🕸️graph/🟦️.ts";
import type {ExecuteOutcomeV1, ExecuteRequestV1} from "../🏃️engine/🟦️.ts";

/** 📓️ Removes TeX comments (an unescaped `%` to the end of the line). */
export const stripTexCommentsV1 = (text: string): string => text.split("\n").map(line => line.replace(/(^|[^\\])((?:\\\\)*)%.*$/, "$1$2")).join("\n");
const normalize = (path: string): string => {const out: string[] = []; for (const part of path.split("\\").join("/").split("/")) {if (part === "" || part === ".") continue; if (part === "..") out.pop(); else out.push(part);} return out.join("/");};

/** 📓️ What a document source refers to. */
export type TexReferencesV1 = {readonly inputs: readonly string[]; readonly bibliographies: readonly string[]; readonly graphics: readonly string[]; readonly packages: readonly string[]; readonly classes: readonly string[]; readonly externalDocuments: readonly string[]; readonly isDocument: boolean};
const braces = (text: string, command: string): string[] => [...text.matchAll(new RegExp(`\\\\${command}\\s*(?:\\[[^\\]]*\\])?\\s*\\{([^{}]*)\\}`, "g"))].map(match => match[1]!.trim());
/** 📓️ Scans a TeX source for the files it pulls in (`\input`, `\include`, `\subfile`, `\bibliography`, `\addbibresource`, `\includegraphics`, `\usepackage`, `\documentclass`) and for `\externaldocument` (xr) edges. */
export function scanTexV1(text: string): TexReferencesV1 {
  const clean = stripTexCommentsV1(text), split = (values: readonly string[]): string[] => values.flatMap(value => value.split(",").map(part => part.trim()).filter(Boolean));
  return {
    inputs: [...braces(clean, "input"), ...braces(clean, "include"), ...braces(clean, "subfile")], bibliographies: [...split(braces(clean, "bibliography")), ...braces(clean, "addbibresource")], graphics: braces(clean, "includegraphics"),
    packages: split(braces(clean, "usepackage")), classes: braces(clean, "documentclass"), externalDocuments: braces(clean, "externaldocument"), isDocument: /\\documentclass\b/.test(clean),
  };
}

/** 📓️ The interface of a document: its auxiliary file reduced to what other documents and later passes read. `\newlabel` and `\bibcite` lines sorted (their order is an artifact of the page flow), `\@writefile{toc|lof|lot}` lines kept in order, `\bibstyle`/`\bibdata` kept; everything else (`\relax`, `\providecommand`, `\gdef`, citation order bookkeeping) dropped. */
export function normalizeTexAuxV1(aux: string): string {
  const labels: string[] = [], cites: string[] = [], rest: string[] = [];
  for (const raw of aux.split("\n")) {
    const line = raw.trim();
    if (line.startsWith("\\newlabel{")) labels.push(line);
    else if (line.startsWith("\\bibcite{")) cites.push(line);
    else if (/^\\@writefile\{(toc|lof|lot)\}/.test(line) || line.startsWith("\\bibstyle{") || line.startsWith("\\bibdata{")) rest.push(line);
  }
  return [...sortedCodePointsV1(labels), ...sortedCodePointsV1(cites), ...rest].join("\n");
}
/** 📓️ The interface digest of an auxiliary file. */
export const texAuxDigestV1 = (aux: string): string => sha256V1("polyglot-tex-aux-v1\n" + normalizeTexAuxV1(aux));

export type TexWorkspaceSnapshotV1 = {readonly files: readonly string[]; readonly read: (path: string) => string};
const GRAPHIC_EXTENSIONS = ["", ".pdf", ".png", ".jpg", ".jpeg", ".eps", ".svg"];
/** 📓️ Plans the unit graph of TeX documents: every file with `\documentclass` is a document unit owning that file; included files, bibliographies, graphics and local packages and classes are inputs, `\externaldocument` becomes an `interface` edge (interface kind `tex-aux`, closure direct: the cross references of the other document's auxiliary file). */
export function planTexUnitsV1(snapshot: TexWorkspaceSnapshotV1, options: {readonly policy: UnitsPolicyV1; readonly toolchain: string}): UnitGraphV1 {
  const files = new Set(snapshot.files), mains = sortedCodePointsV1(snapshot.files.filter(path => path.endsWith(".tex") && scanTexV1(snapshot.read(path)).isDocument));
  const idOf = (main: string): string => "tex:" + main.slice(0, -".tex".length), byStem = new Map(mains.map(main => [main.slice(0, -".tex".length), main] as const));
  const resolveFile = (directory: string, name: string, extensions: readonly string[]): string | undefined => {for (const extension of extensions) {const candidate = normalize(directory === "" ? name + extension : directory + "/" + name + extension); if (files.has(candidate)) return candidate;} return undefined;};
  const units: UnitV1[] = mains.map(main => {
    const directory = main.includes("/") ? main.slice(0, main.lastIndexOf("/")) : "", inputs = new Set<string>(), seen = new Set<string>([main]), queue = [main], dependencies = new Map<string, UnitDependencyV1>();
    while (queue.length) {
      const current = queue.pop()!, currentDirectory = current.includes("/") ? current.slice(0, current.lastIndexOf("/")) : "", refs = scanTexV1(snapshot.read(current));
      for (const name of refs.inputs) {const hit = resolveFile(currentDirectory, name, ["", ".tex"]) ?? resolveFile(directory, name, ["", ".tex"]); if (hit && !seen.has(hit)) {seen.add(hit); inputs.add(hit); queue.push(hit);}}
      for (const name of refs.bibliographies) {const hit = resolveFile(currentDirectory, name, ["", ".bib"]) ?? resolveFile(directory, name, ["", ".bib"]); if (hit) inputs.add(hit);}
      for (const name of refs.graphics) {const hit = resolveFile(currentDirectory, name, GRAPHIC_EXTENSIONS) ?? resolveFile(directory, name, GRAPHIC_EXTENSIONS); if (hit) inputs.add(hit);}
      for (const name of refs.packages) {const hit = resolveFile(currentDirectory, name, [".sty"]) ?? resolveFile(directory, name, [".sty"]); if (hit) inputs.add(hit);}
      for (const name of refs.classes) {const hit = resolveFile(currentDirectory, name, [".cls"]) ?? resolveFile(directory, name, [".cls"]); if (hit) inputs.add(hit);}
      for (const name of refs.externalDocuments) {const stem = normalize(directory === "" ? name : directory + "/" + name), other = byStem.get(stem); if (other && other !== main) dependencies.set(idOf(other), {unit: idOf(other), use: "interface"});}
    }
    const stem = main.slice(main.lastIndexOf("/") + 1, -".tex".length);
    return {
      id: idOf(main), language: "tex", kind: "document", label: main, sources: [main], inputs: sortedCodePointsV1(inputs), dependencies: [...dependencies.values()].sort((a, b) => compareCodePointsV1(a.unit, b.unit)), externals: [],
      interface: {kind: "tex-aux"}, outputs: [`${stem}.pdf`], toolchain: {id: options.toolchain, parts: []},
      action: {argv: ["tectonic", "-X", "compile", main, "--outdir", "{out}", "--keep-intermediates", "--keep-logs"], env: {SOURCE_DATE_EPOCH: "0", FORCE_SOURCE_DATE: "1", SEMIO_TEX_STEM: stem}},
    };
  });
  return {version: 1, units};
}

const listFiles = (root: string, extension: string): string[] => {const found: string[] = []; const walk = (directory: string): void => {try {for (const entry of readdirSync(directory, {withFileTypes: true})) {const path = join(directory, entry.name); if (entry.isDirectory()) walk(path); else if (path.endsWith(extension)) found.push(path);}} catch {/* absent */}}; walk(root); return found.sort();};
/** 🏭️ The process effect of the TeX executor. */
export type TexRunnerV1 = (argv: readonly string[], options: {readonly cwd: string; readonly env: Readonly<Record<string, string>>; readonly signal: AbortSignal}) => Promise<{readonly code: number; readonly output: string}>;
/** 🏭️ Executes a document with the pinned `tectonic` (`-X compile`, intermediates kept, deterministic dates): the interface digest is that of the normalized `.aux`, the artifact the normalized auxiliary text other documents read through xr (placed next to the source as `<name>.aux` before the compile), the output digest that of the bibliography `.bbl` when there is one plus the PDF. Not run in this repository's verification: no TeX engine is installed on the verification host. */
export function texUnitExecutorV1(options: {readonly workspace: string; readonly outputRoot: string; readonly run: TexRunnerV1}): (request: ExecuteRequestV1) => Promise<ExecuteOutcomeV1> {
  return async request => {
    const {unit} = request, out = join(options.outputRoot, sha256V1(unit.id).slice(0, 24)), started = performance.now();
    rmSync(out, {recursive: true, force: true}); mkdirSync(out, {recursive: true});
    const stem = unit.action.env.SEMIO_TEX_STEM ?? "document", staged: string[] = [];
    for (const dependency of request.dependencies) if (dependency.artifact) {const target = join(options.workspace, dirname(unit.sources[0]!), dependency.unit.slice(dependency.unit.lastIndexOf("/") + 1) + ".aux"); writeFileSync(target, dependency.artifact); staged.push(target);}
    const result = await options.run(unit.action.argv.map(part => part.split("{out}").join(out)), {cwd: options.workspace, env: unit.action.env, signal: request.signal}), milliseconds = Math.round(performance.now() - started);
    for (const path of staged) rmSync(path, {force: true});
    if (result.code !== 0) return {ok: false, diagnostics: result.output.trim(), outputs: sha256V1("failed:" + result.output), interface: null, milliseconds};
    const aux = join(out, stem + ".aux"), bbl = listFiles(out, ".bbl")[0], pdf = join(out, stem + ".pdf");
    let auxText = "";
    try {auxText = readFileSync(aux, "utf8");} catch {return {ok: false, diagnostics: `no ${stem}.aux was kept`, outputs: sha256V1("missing"), interface: null, milliseconds};}
    return {ok: true, diagnostics: "", outputs: sha256V1([bbl ? sha256V1(readFileSync(bbl)) : "", sha256V1(readFileSync(pdf))].join("\n")), interface: texAuxDigestV1(auxText), artifact: normalizeTexAuxV1(auxText), milliseconds};
  };
}
