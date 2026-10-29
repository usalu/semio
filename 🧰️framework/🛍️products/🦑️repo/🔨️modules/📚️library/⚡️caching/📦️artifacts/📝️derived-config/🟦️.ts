/** 📝️ Derives cargo-nextest scope configurations and validates dependency-update manifest ownership.
 * @see https://nexte.st/docs/configuration/ */
import { existsSync, lstatSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { GlobV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🌐️glob/🟦️.ts";
import { runtimeParseYamlV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";
import {canonicalTestAggregatePolicyV1,deriveCanonicalTestAggregateV1,requireCanonicalTestAggregateV1} from "../../🗂️suites/🧮️aggregate/🟦️.ts";

export const NEXTEST = ".config/nextest.toml";
export const NEXTEST_SCOPES = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/⏱️process-budgets/native-profile.json";
const HOST_FOLDER = "${workspaceFolder}";
const CONTAINER_FOLDER = "${containerWorkspaceFolder}";
const QUICK_OVERRIDES = /^\[\[profile\.quick\.overrides\]\]/m;

function normalize(text: string): string {
  return text.replaceAll("\r\n", "\n");
}

/** 📚️ The workspaces that own a cargo-nextest scope file besides the root one. */
export function nextestScopes(workspace: string): string[] {
  return (JSON.parse(readFileSync(join(workspace, NEXTEST_SCOPES), "utf8")) as { workspaces: { directory: string }[] }).workspaces.map((row) => row.directory).filter((directory) => directory !== ".");
}

/** 🧮️ A scope file is the root file followed by the scope's own `[[profile.quick.overrides]]` tables. */
export function deriveNextest(root: string, scope: string): string {
  const base = normalize(root).trimEnd(), own = normalize(scope), from = own.startsWith(base) ? base.length : 0;
  const first = own.slice(from).search(QUICK_OVERRIDES);
  return first === -1 ? `${base}\n` : `${base}\n\n${own.slice(from + first).trimEnd()}\n`;
}

export function nextestProblems(workspace: string): string[] {
  const root = readFileSync(join(workspace, NEXTEST), "utf8"), problems: string[] = [];
  for (const scope of nextestScopes(workspace)) {
    const path = `${scope}/${NEXTEST}`, own = readFileSync(join(workspace, path), "utf8");
    if (normalize(own) !== deriveNextest(root, own)) problems.push(`${path} must be ${NEXTEST} followed by its own [[profile.quick.overrides]] tables`);
  }
  return problems;
}

/** 🤖️ Matches the configured action directory or the repository root workflow scope used by Dependabot.
 * @see https://github.com/dependabot/dependabot-core/blob/main/github_actions/lib/dependabot/github_actions/file_fetcher.rb */
function actionManifests(folder:string,workspace:string):boolean {
  const yaml = (directory:string):boolean => lstatSync(directory,{throwIfNoEntry:false})?.isDirectory()===true && readdirSync(directory,{withFileTypes:true}).some(row=>row.isFile() && /\.ya?ml$/u.test(row.name));
  return folder===join(workspace) ? ["action.yml","action.yaml"].some(name=>lstatSync(join(folder,name),{throwIfNoEntry:false})?.isFile()===true) || yaml(join(folder,".github","workflows")) : yaml(folder);
}

const DEPENDABOT_MANIFESTS: Readonly<Record<string, (folder: string, workspace:string) => boolean>> = {
  bun: (folder) => existsSync(join(folder, "package.json")),
  cargo: (folder) => existsSync(join(folder, "Cargo.toml")),
  gomod: (folder) => existsSync(join(folder, "go.mod")),
  uv: (folder) => existsSync(join(folder, "pyproject.toml")),
  nuget: (folder) => readdirSync(folder).some((name) => name.endsWith(".csproj")),
  "github-actions": actionManifests,
};

function dependabotDirectories(workspace: string, pattern: string): string[] {
  const relative = pattern.replace(/^\/+/, "");
  if (!relative.includes("*")) return existsSync(join(workspace, relative)) ? [relative] : [];
  return [...new GlobV1(relative).scanSync({ cwd: workspace, onlyFiles: false })].sort();
}

export function dependabotProblems(workspace: string): string[] {
  const document = runtimeParseYamlV1(readFileSync(join(workspace, ".github/dependabot.yml"), "utf8")) as { updates: { "package-ecosystem": string; directory?: string; directories?: string[] }[] };
  const problems: string[] = [], covered = new Map<string, Set<string>>();
  for (const update of document.updates) {
    const ecosystem = update["package-ecosystem"], manifest = DEPENDABOT_MANIFESTS[ecosystem];
    if (!manifest) {
      problems.push(`.github/dependabot.yml: ecosystem ${ecosystem} has no manifest rule`);
      continue;
    }
    for (const pattern of [...(update.directories ?? []), ...(update.directory ? [update.directory] : [])]) {
      const found = dependabotDirectories(workspace, pattern);
      if (!found.length) problems.push(`.github/dependabot.yml: ${ecosystem} directory ${pattern} does not exist`);
      for (const directory of found) {
        covered.set(ecosystem, (covered.get(ecosystem) ?? new Set()).add(directory));
        if (!manifest(join(workspace, directory),workspace)) problems.push(`.github/dependabot.yml: ${ecosystem} directory ${pattern} holds no manifest`);
      }
    }
  }
  const modules = [...readFileSync(join(workspace, "go.work"), "utf8").matchAll(/^\t\.\/(.+)$/gm)].map((match) => match[1]!.trim());
  for (const module of modules) if (!covered.get("gomod")?.has(module)) problems.push(`.github/dependabot.yml: go.work module ${module} has no gomod entry`);
  return problems;
}

/** 🔎️ Every drift between checked-in configuration and its canonical source; empty when all of it is derived. */
export function checkDerivedConfig(workspace: string): string[] {
  return [...nextestProblems(workspace), ...dependabotProblems(workspace),...canonicalTestAggregateProblemsV1(workspace)];
}

/** 🦀️ Rewrites the scoped nextest declarations without changing other derived owners. */
export async function writeNextestConfig(workspace: string): Promise<string[]> {
  const changed: string[] = [];
  const write = async (path: string, next: string): Promise<void> => {
    const file = join(workspace, path);
    if (normalize(readFileSync(file, "utf8")) === normalize(next)) return;
    writeFileSync(file, next);
    changed.push(path);
  };
  const root = readFileSync(join(workspace, NEXTEST), "utf8");
  for (const scope of nextestScopes(workspace)) {
    const path = `${scope}/${NEXTEST}`;
    await write(path, deriveNextest(root, readFileSync(join(workspace, path), "utf8")));
  }
  return changed;
}

/** 🧾️ Checks the authored aggregate against the current closed route authorities. */
export function canonicalTestAggregateProblemsV1(workspace:string):string[]{
 try{const policy=canonicalTestAggregatePolicyV1,project=JSON.parse(readFileSync(join(workspace,policy.manifest),"utf8"));requireCanonicalTestAggregateV1(project,policy.root);return [];}catch(error){return [String(error)];}
}

/** ✍️ Rewrites each derived owner from its current Source declaration. */
export async function writeDerivedConfig(workspace:string):Promise<string[]>{
 const changed=await writeNextestConfig(workspace),policy=canonicalTestAggregatePolicyV1,file=join(workspace,policy.manifest),source=readFileSync(file,"utf8"),project=JSON.parse(source);
 project.targets[policy.target]=deriveCanonicalTestAggregateV1({...project,root:policy.root});
 const next=JSON.stringify(project,null,2)+"\n";
 if(normalize(source)!==next){writeFileSync(file,next);changed.push(policy.manifest);}
 return changed;
}
