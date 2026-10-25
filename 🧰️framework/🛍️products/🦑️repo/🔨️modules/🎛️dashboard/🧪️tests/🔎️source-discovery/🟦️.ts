import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020";
import glob from "fast-glob";
import * as ts from "typescript";
import lodash from "lodash";
import { readFileSync, writeFileSync, mkdirSync, mkdtempSync, rmSync, symlinkSync } from "node:fs";
import { dirname, join, resolve, relative } from "node:path";
import { spawnSync } from "node:child_process";
import { rustTokens, rustTokenPairs } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";

const dashboard = resolve(import.meta.dir, "../.."), read = (path: string) => JSON.parse(readFileSync(join(dashboard, path), "utf8"));
const corpus = read("🧪️tests/🧫️fixtures/🔎️source-discovery/🔣️.json"), schema = read("🧬️schema/🔎️source-discovery/🔣️.json"), policy = schema.$defs.Policy.const;

function item(source: string, kind: string, name: string): string {
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), at = tokens.findIndex((token, index) => token.text === kind && tokens[index + 1]?.text === name);
  if (at < 0) throw Error(`Actual Rust ${kind} ${name} required`);
  const open = tokens.findIndex((token, index) => index > at && token.text === "{");
  const close = pairs.get(open); if (close === undefined) throw Error(`Complete Rust ${name} required`);
  return source.slice(tokens[at]!.start, tokens[close]!.end);
}

const quoted = (value: string) => JSON.stringify(value), vector = (values: string[]) => values.length ? `vec![${values.map(value => `String::from(${quoted(value)})`).join(",")}]` : "Vec::<String>::new()";
const kind = (value: string) => value[0]!.toUpperCase() + value.slice(1);

test("registry refresh rebases current forms and releases only the replaced source subscription", () => {
  expect(new Ajv({ strict: false }).compile(schema)(corpus)).toBe(true);
  for (const row of corpus.registryChanges) {
    const before = row.before, current = row.after.find((entry: any) => entry.id === before.id), previous = lodash.keyBy(before.fields, (field: any) => field.parameter.id);
    const fields = row.sameRegistry ? before.fields.map((field: any) => ({ id: field.parameter.id, value: field.value, suggested: field.suggested })) : current?.parameters.map((parameter: any) => {
      const old = previous[parameter.id], compatible = old?.parameter.kind === parameter.kind && !old.suggested && (parameter.kind !== "choice" || parameter.values.includes(old.value)) && (parameter.kind !== "flag" || ["true", "false"].includes(old.value));
      return { id: parameter.id, value: compatible ? old.value : null, suggested: false };
    }) ?? [];
    const oldFocus = before.fields[before.focus]?.parameter.id, focus = oldFocus ? fields.findIndex((field: any) => field.id === oldFocus) : fields.length + before.focus - before.fields.length;
    const observed = { open: !!current, entry: current ? row.sameRegistry ? before.entry : row.after.indexOf(current) : null, mutating: current?.mutating ?? null, focus: current ? row.sameRegistry ? before.focus : focus < 0 ? current.parameters.findIndex((parameter: any) => parameter.required) < 0 ? fields.length + 2 : current.parameters.findIndex((parameter: any) => parameter.required) : focus : null, stage: row.sameRegistry ? before.stage : current ? "Configure" : "Browse", fields, released: row.sameRegistry ? 0 : 1 };
    expect(observed, row.id).toEqual(row.expected);
  }
  const parent = process.env.SEMIO_TEST_ARTIFACT_DIR; if (!parent) throw Error("Caller-owned registry refresh artifacts required");
  mkdirSync(parent, { recursive: true }); const workspace = mkdtempSync(join(parent, "sr-"));
  try {
    const source = readFileSync(join(dashboard, "🖥️terminal/🚀️launcher/🦀️.rs"), "utf8");
    const parameter = (row: any) => `Parameter{id:${quoted(row.id)}.into(),kind:ParameterKind::${kind(row.kind)},default:${row.default === null ? "None" : `Some(${quoted(row.default)}.into())`},required:${row.required},values:vec![${row.values.map((value: string) => `(${quoted(value)}.into(),())`).join(",")}]}`;
    const field = (row: any) => `Field{parameter:${parameter(row.parameter)},value:${row.value === null ? "None" : `Some(${quoted(row.value)}.into())`},suggested:${row.suggested}}`;
    const entry = (row: any) => `Entry{id:${quoted(row.id)}.into(),mutating:${row.mutating},parameters:vec![${row.parameters.map(parameter).join(",")}]}`;
    const cases = corpus.registryChanges.map((row: any) => {
      const before = row.before, expected = row.expected, previous = { id: before.id, mutating: before.mutating, parameters: before.fields.map((field: any) => field.parameter) };
      return `{let previous=Arc::new(Registry{commands:vec![${entry(previous)}],released:Mutex::new(Vec::new())});let next=${row.sameRegistry ? "previous.clone()" : `Arc::new(Registry{commands:vec![${row.after.map(entry).join(",")}],released:Mutex::new(Vec::new())})`};let mut state=LauncherState{registry:Some(previous.clone()),nodes:Vec::new(),locale:0,preferences:Preferences,source_session:27,discovery_revision:99,stage:Stage::${before.stage},offset:7,form:Some(Form{entry:${before.entry},id:${quoted(before.id)}.into(),mutating:${before.mutating},fields:vec![${before.fields.map(field).join(",")}],extra_args:"manual args".into(),extra_env:"EXTRA=manual".into(),focus:${before.focus},suggestion:4})};state.set_registry(next.clone());assert_eq!(state.stage,Stage::${expected.stage},${quoted(row.id)});assert_eq!(previous.released.lock().unwrap().as_slice(),${expected.released ? "&[27]" : "&[]"});assert_eq!(state.form.is_some(),${expected.open});${expected.open ? `let form=state.form.unwrap();assert_eq!(form.entry,${expected.entry});assert_eq!(form.mutating,${expected.mutating});assert_eq!(form.focus,${expected.focus});assert_eq!(form.fields.iter().map(|field|(field.parameter.id.clone(),field.value.clone(),field.suggested)).collect::<Vec<_>>(),vec![${expected.fields.map((field: any) => `(${quoted(field.id)}.to_string(),${field.value === null ? "None" : `Some(${quoted(field.value)}.to_string())`},${field.suggested})`).join(",")}]);assert_eq!(form.extra_args,"manual args");assert_eq!(form.extra_env,"EXTRA=manual");assert_eq!(form.fields.iter().map(|field|field.parameter.clone()).collect::<Vec<_>>(),next.commands[form.entry].parameters);` : ""}}`;
    }).join("\n");
    const program = join(workspace, "refresh.rs"), binary = join(workspace, process.platform === "win32" ? "refresh.exe" : "refresh");
    const scaffold = `use std::sync::{Arc,Mutex};#[derive(Clone,Copy,Debug,PartialEq,Eq)] ${item(source,"enum","Stage")} #[derive(Clone,Copy,Debug,PartialEq,Eq)] enum ParameterKind{Text,Choice,Flag} #[derive(Clone,Debug,PartialEq,Eq)] struct Parameter{id:String,kind:ParameterKind,default:Option<String>,required:bool,values:Vec<(String,())>} #[derive(Clone,Debug)] ${item(source,"struct","Field")} #[derive(Clone,Debug)] ${item(source,"struct","Form")} struct Entry{id:String,mutating:bool,parameters:Vec<Parameter>} struct Registry{commands:Vec<Entry>,released:Mutex<Vec<u64>>} impl Registry{fn entries(&self)->&[Entry]{&self.commands} fn parameters(&self,entry:&Entry)->Vec<Parameter>{entry.parameters.clone()} fn release_suggestions(&self,session:u64){self.released.lock().unwrap().push(session);}} struct Preferences;impl Preferences{fn launch_defaults(&self,_:&[Parameter])->Vec<(String,String)>{Vec::new()}} struct LauncherState{registry:Option<Arc<Registry>>,nodes:Vec<String>,locale:u8,preferences:Preferences,source_session:u64,discovery_revision:u64,stage:Stage,offset:usize,form:Option<Form>} fn build_nodes(registry:&Registry,_:u8)->Vec<String>{registry.entries().iter().map(|entry|entry.id.clone()).collect()} impl LauncherState{${item(source,"fn","set_registry")} ${item(source,"fn","refreshed_form")}} fn main(){${cases}println!("[TRACE] actual Rust registry adoption checked current form identities, derived invalidation and subscriber release");}`;
    writeFileSync(program, scaffold);
    const compiled = spawnSync("rustc", ["--edition=2021", "--crate-name=registry_refresh_probe", program, "-o", binary], { encoding: "utf8", timeout: 300_000, maxBuffer: 1_048_576 }); expect(compiled.status, compiled.stderr).toBe(0);
    const run = spawnSync(binary, [], { encoding: "utf8", timeout: 120_000, maxBuffer: 65_536 }); expect(run.status, run.stderr).toBe(0); console.log(run.stdout.trim());
  } finally { const bounded = relative(resolve(parent), resolve(workspace)); if (!bounded || bounded.startsWith("..") || bounded.includes(":")) throw Error("Registry refresh cleanup escaped caller-owned directory"); rmSync(workspace, { recursive: true, force: true }); }
}, 60_000);

test("source suggestions obey shared declarations and independent TypeScript and filesystem oracles", () => {
  expect(new Ajv({ strict: false }).compile(schema)(corpus)).toBe(true);
  const registry = read("🧬️schema/🎮️registry/🔣️.json"), oracle = new Ajv({ strict: false, allErrors: true }); oracle.addSchema(registry);
  const parameter = oracle.getSchema(`${registry.$id}#/$defs/Parameter`)!;
  for (const row of corpus.declarations) expect(parameter(row.parameter), JSON.stringify(row)).toBe(row.accepted);
  for (const row of corpus.files) {
    if (row.kind !== "tests" && !row.path.includes("/🧪️tests/")) continue;
    const tree = ts.createSourceFile(row.path, row.text, ts.ScriptTarget.Latest, true);
    const direct = tree.statements.some(node => ts.isImportDeclaration(node) && ts.isStringLiteral(node.moduleSpecifier) && node.moduleSpecifier.text === "bun:test" && !!node.importClause && !node.importClause.isTypeOnly && (!!node.importClause.name || !!node.importClause.namedBindings && (ts.isNamespaceImport(node.importClause.namedBindings) || node.importClause.namedBindings.elements.some(element => !element.isTypeOnly))));
    expect(direct, row.path).toBe(row.kind === "tests");
  }
});

test("actual Rust source discovery preserves ownership, cancellation, refresh, bounded reads and derived cwd", () => {
  const parent = process.env.SEMIO_TEST_ARTIFACT_DIR; if (!parent) throw Error("Caller-owned source discovery artifacts required");
  mkdirSync(parent, { recursive: true }); const workspace = mkdtempSync(join(parent, "sd-")), outside = workspace + "-outside";
  const source = readFileSync(join(dashboard, "🎮️registry/🔎️source-discovery/🦀️.rs"), "utf8"), declarations = readFileSync(join(dashboard, "🎮️registry/🧾️declarations/🦀️.rs"), "utf8"), form = readFileSync(join(dashboard, "🖥️terminal/🚀️launcher/🦀️.rs"), "utf8");
  try {
    for (const row of corpus.files) { const path = join(workspace, row.path); if (path.length > 256) throw Error("Portable source fixture path capacity exceeded"); mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, row.text); }
    mkdirSync(join(outside, "nested"), { recursive: true }); writeFileSync(join(outside, "📜️script.ts"), "outside"); writeFileSync(join(outside, "nested/📜️script.ts"), "outside nested owner"); symlinkSync(outside, join(workspace, "app/junction"), process.platform === "win32" ? "junction" : "dir");
    const independent = glob.sync("**/*", { cwd: workspace, dot: true, onlyFiles: true, followSymbolicLinks: false, ignore: [...policy.excludedDirectories.map((name: string) => `**/${name}/**`), ...policy.excludedRoots.map((root: string) => `${root}/**`)] });
    expect(independent.filter(path => path.endsWith("📜️script.ts")).sort()).toEqual(corpus.files.filter((row: any) => row.kind === "scripts").map((row: any) => row.path).sort());
    const structures = ["Suggestion", "Suggestions", "SourceRecord", "SourceSnapshot", "SourceDiscovery", "SourcePolicy"].map(name => item(source, "struct", name));
    const functions = ["source_owner", "source_path", "source_attributes", "physical_source", "bun_test_import", "source_kind", "source_scan", "source_changes"].map(name => item(source, "fn", name));
    const actualImpl = item(source, "impl", "Registry");
    const drop = "impl Drop for SourceDiscovery { fn drop(&mut self) { self.cancel.store(true, Ordering::Relaxed); } }";
    const policyLiteral = `SourcePolicy {${Object.entries(policy).map(([key, value]) => `${key.replace(/[A-Z]/g, letter => "_" + letter.toLowerCase())}:${Array.isArray(value) ? vector(value) : value}`).join(",")}}`;
    const scaffold = `use std::collections::{HashMap,HashSet};use std::path::{Path,PathBuf};use std::sync::{Arc,Mutex,atomic::{AtomicBool,Ordering}};use std::time::{Duration,SystemTime};use std::io::Read;
#[derive(Debug,Clone,Copy,PartialEq,Eq)] ${item(declarations, "enum", "SuggestionSource")}
#[derive(Debug,Clone)] struct SuggestionSpec {source:SuggestionSource,project_parameter:Option<String>,directory_parameter:Option<String>}
#[derive(Debug,Clone,Copy,PartialEq,Eq)] ${item(declarations,"enum","ParameterKind")}
impl Default for ParameterKind {fn default()->Self{Self::Text}}
#[derive(Debug,Clone,Default)] struct Parameter {id:String,kind:ParameterKind,default:Option<String>,suggestions:Option<SuggestionSpec>}
#[derive(Debug,Clone)] ${item(form, "struct", "Field")}
struct Form {fields:Vec<Field>,focus:usize,suggestion:usize,extra_args:String,extra_env:String}
impl Form {${["effective","start_slot","args_slot","env_slot"].map(name => item(form,"fn",name)).join("\n")}}
struct LauncherState;
impl LauncherState {${item(form,"fn","invalidate_source_dependents")} ${item(form,"fn","accept_suggestion")} ${item(form,"fn","text")} ${item(form,"fn","normalize")}}

#[derive(Debug,Default)] struct Registry {root:PathBuf,source_projects:Vec<(String,String)>,source_discovery:Mutex<HashMap<String,SourceDiscovery>>,source_cancelled:Mutex<HashSet<u64>>}
#[derive(Debug,Clone,PartialEq,Eq)] ${structures[0]}
#[derive(Debug,Clone,Default)] ${structures[1]}
#[derive(Debug,Clone)] ${structures[2]}
#[derive(Debug,Default)] ${structures[3]}
#[derive(Debug)] ${structures[4]}
${structures[5]}
${drop}
fn source_policy()->&'static SourcePolicy {static POLICY:std::sync::OnceLock<SourcePolicy>=std::sync::OnceLock::new();POLICY.get_or_init(||${policyLiteral})}
${functions.join("\n")}
${actualImpl}`;
    const projects = `vec![${corpus.projects.map((row: any) => `(${quoted(row.id)}.into(),${quoted(row.root)}.into())`).join(",")}]`;
    const cases = corpus.cases.map((row: any, index: number) => `{let parameter=Parameter{suggestions:Some(SuggestionSpec{source:SuggestionSource::${kind(row.source)},project_parameter:Some("project".into()),directory_parameter:None}),..Parameter::default()};let chosen=${row.project === null ? "Vec::new()" : `vec![("project".into(),${quoted(row.project)}.into())]`};let session=${index + 1};let until=std::time::Instant::now()+Duration::from_secs(120);let page=loop{let page=registry.suggestions(&parameter,&chosen,${quoted(row.query)},session);if !page.scanning{break page;}assert!(std::time::Instant::now()<until,"source index deadline");std::thread::sleep(Duration::from_millis(5));};assert_eq!(page.values.iter().map(|row|row.value.clone()).collect::<Vec<_>>(),${vector(row.values)},"case ${index}");assert_eq!(page.values.iter().map(|row|row.project.clone()).collect::<Vec<_>>(),${vector(row.projects)});assert_eq!(page.values.iter().map(|row|row.directory.clone()).collect::<Vec<_>>(),${vector(row.directories)});registry.release_suggestions(session);}`).join("\n");
    const selections = corpus.selectionCases.map((row: any) => `{let parameter=|id:&str|Parameter{id:id.into(),..Parameter::default()};let mut source=parameter("script");source.suggestions=Some(SuggestionSpec{source:SuggestionSource::Scripts,project_parameter:Some("project".into()),directory_parameter:Some("directory".into())});let mut form=Form{fields:vec![Field{parameter:parameter("project"),value:None,suggested:false},Field{parameter:source,value:None,suggested:false},Field{parameter:parameter("directory"),value:None,suggested:false}],focus:1,suggestion:0,extra_args:String::new(),extra_env:String::new()};LauncherState::accept_suggestion(&mut form,&Suggestion{value:${quoted(row.first)}.into(),project:${quoted(row.firstProject)}.into(),directory:${quoted(row.firstDirectory)}.into()});${row.manual === null ? "" : `*LauncherState::text(&mut form).unwrap()=${quoted(row.manual)}.into();`}LauncherState::normalize(&mut form);form.focus=1;LauncherState::accept_suggestion(&mut form,&Suggestion{value:${quoted(row.second)}.into(),project:${quoted(row.secondProject)}.into(),directory:${quoted(row.secondDirectory)}.into()});assert_eq!(Form::effective(&form.fields[2]),Some(${quoted(row.directory)}));assert_eq!(Form::effective(&form.fields[0]),Some(${quoted(row.secondProject)}));}`).join("\n");
    const changes = corpus.projectChanges.map((row: any) => `{let parameter=|id:&str|Parameter{id:id.into(),..Parameter::default()};let mut source=parameter("script");source.suggestions=Some(SuggestionSpec{source:SuggestionSource::Scripts,project_parameter:Some("project".into()),directory_parameter:Some("directory".into())});let mut form=Form{fields:vec![Field{parameter:parameter("project"),value:None,suggested:false},Field{parameter:source,value:None,suggested:false},Field{parameter:parameter("directory"),value:None,suggested:false}],focus:1,suggestion:0,extra_args:String::new(),extra_env:String::new()};LauncherState::accept_suggestion(&mut form,&Suggestion{value:"app/📜️script.ts".into(),project:"app".into(),directory:"app".into()});${row.manualDirectory === null ? "" : `*LauncherState::text(&mut form).unwrap()=${quoted(row.manualDirectory)}.into();`}${row.manualScript === null ? "" : `form.focus=1;*LauncherState::text(&mut form).unwrap()=${quoted(row.manualScript)}.into();`}form.focus=0;*LauncherState::text(&mut form).unwrap()="nested".into();assert_eq!(Form::effective(&form.fields[1]),${row.scriptAfterProject === null ? "None" : `Some(${quoted(row.scriptAfterProject)})`});assert_eq!(Form::effective(&form.fields[2]),${row.directoryAfterProject === null ? "None" : `Some(${quoted(row.directoryAfterProject)})`});form.focus=1;LauncherState::accept_suggestion(&mut form,&Suggestion{value:"app/nested/📜️script.ts".into(),project:"nested".into(),directory:"app/nested".into()});assert_eq!(Form::effective(&form.fields[2]),Some(${quoted(row.directoryAfterSelection)}));}`).join("\n");
    const paths = corpus.rootPaths.map((row: any) => `assert_eq!(source_path(${quoted(row.path)}),${row.accepted});`).join("\n");
    const attributes = corpus.attributes.map((row: any) => `assert_eq!(source_attributes(${row.windows},${row.attributes},${row.symlink}),${row.accepted});`).join("\n");
    const assertions = `
let parameter=Parameter{suggestions:Some(SuggestionSpec{source:SuggestionSource::Scripts,project_parameter:Some("project".into()),directory_parameter:None}),..Parameter::default()};let chosen=vec![("project".into(),"app".into())];
let _=registry.suggestions(&parameter,&chosen,"",100);let _=registry.suggestions(&parameter,&chosen,"",101);registry.cancel_suggestions(100);assert!(registry.suggestions(&parameter,&chosen,"",100).cancelled);assert!(!registry.suggestions(&parameter,&chosen,"",101).cancelled);registry.release_suggestions(100);assert!(!registry.suggestions(&parameter,&chosen,"",102).cancelled);
let shared=registry.source_discovery.lock().unwrap().get("app").unwrap().state.clone();registry.refresh_suggestions(&parameter,&chosen,101);assert!(Arc::ptr_eq(&shared,&registry.source_discovery.lock().unwrap().get("app").unwrap().state));assert_eq!(registry.source_discovery.lock().unwrap().get("app").unwrap().users.len(),2);
let until=std::time::Instant::now()+Duration::from_secs(120);while registry.suggestions(&parameter,&chosen,"",101).scanning {assert!(std::time::Instant::now()<until);std::thread::sleep(Duration::from_millis(5));}
let created=registry.root.join("app/new/📜️script.ts");std::fs::create_dir_all(created.parent().unwrap()).unwrap();std::fs::write(&created,"new").unwrap();let until=std::time::Instant::now()+Duration::from_secs(120);loop{if registry.suggestions(&parameter,&chosen,"new",101).values.iter().any(|row|row.value=="app/new/📜️script.ts"){break;}assert!(std::time::Instant::now()<until,"index must observe added source");std::thread::sleep(Duration::from_millis(20));}
assert!(registry.suggestions(&parameter,&chosen,&"app ".repeat(10000),103).truncated);registry.release_suggestions(103);registry.release_suggestions(101);registry.release_suggestions(102);assert!(registry.source_discovery.lock().unwrap().is_empty());
let same=registry.root.join("📜️script.ts");let previous=std::fs::metadata(&same).unwrap().modified().ok();let watched=vec![(same.clone(),previous);10];let mut cursor=0;assert!(!source_changes(&watched,&mut cursor,3,&AtomicBool::new(false)));assert_eq!(cursor,3);
let state=Mutex::new(SourceSnapshot::default());let cancel=AtomicBool::new(true);let _=source_scan(&registry.root,&registry.source_projects,None,source_policy(),&state,&cancel);assert_eq!(state.lock().unwrap().inspected,0);
let late=registry.root.join("app/late.ts");std::fs::write(&late,format!("{}import {{test}} from 'bun:test';"," ".repeat(source_policy().maximum_source_bytes+1))).unwrap();let state=Mutex::new(SourceSnapshot::default());let _=source_scan(&registry.root,&registry.source_projects,Some("app"),source_policy(),&state,&AtomicBool::new(false));assert!(state.lock().unwrap().truncated,"partial per-file read must be explicit");
println!("[TRACE] exact Rust source index checked shared owner/search vectors, two subscribers, cancellation, physical invalidation and bounded reads");`;
    const program = join(workspace, "probe.rs"), binary = join(workspace, process.platform === "win32" ? "probe.exe" : "probe");
    writeFileSync(program, scaffold + `\nfn main(){let registry=Registry{root:std::env::args().nth(1).unwrap().into(),source_projects:${projects},..Registry::default()};${cases}${paths}${attributes}${selections}${changes}${assertions}}`);
    const compiled = spawnSync("rustc", ["--edition=2021", "--crate-name=source_discovery_probe", program, "-o", binary], { encoding: "utf8", timeout: 300_000, maxBuffer: 1_048_576 });
    expect(compiled.status, compiled.stderr).toBe(0);
    const run = spawnSync(binary, [workspace], { encoding: "utf8", timeout: 300_000, maxBuffer: 65_536 });
    expect(run.status, run.stderr).toBe(0); console.log(run.stdout.trim());
    expect(run.stdout).toContain("[TRACE]");
  } finally { for (const path of [workspace, outside]) { const bounded = relative(resolve(parent), resolve(path)); if (!bounded || bounded.startsWith("..") || bounded.includes(":")) throw Error("Source fixture cleanup escaped its caller-owned directory"); rmSync(path, { recursive: true, force: true }); } }
}, 60_000);
