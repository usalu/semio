import{createBoundedCacheV1}from"../../../🧊️bounded/🟨️.mjs";
import{buildRepairCommandV1}from"../../../⚙️operations/🔤️options/🟦️.ts";
import{createHash}from"node:crypto";
import{join}from"node:path";
import{graphQueueRequestedAtV1,graphQueueYieldsV1,yieldGraphQueueV1}from"../🛌️yield/🟨️.mjs";
import{GENERATION_POLICY_V1,carriedStagesV1,canonicalJsonV1,compareCodePointsV1,createAnswersV1,createGenerationStoreV1,createNodeFilePortV1,readGenerationExternalInputsV1,readGenerationPointerV1,resolveGenerationExternalInputV1,rowKeyV1}from"../🟨️.mjs";
import{createGraphPhaseClockV1}from"../../⏱️phases/🟨️.mjs";
import{decodeVerifiedScopeV1}from"../🔭️coverage/🟨️.mjs";
import{readPinnedGenerationV1}from"../🧷️pin/🟨️.mjs";
import{CUSTODY_POLICY_V1}from"../🔐️custody/🟨️.mjs";
import{closureMissingErrorV1}from"../../🩹️closure-repair/🟨️.mjs";
import{prepareClosureRepairV1,readClosureRepairV1,recoverClosureRepairV1,verifyClosureRepairCandidateV1}from"../🩹️closure-repair/🟨️.mjs";
import{runtimeCollectGarbageV1}from"../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

/** 🗣️ The bilingual texts of the graph generation; the schema file declares the same table and a law keeps both equal. */
export const GENERATION_MESSAGES_V1=Object.freeze({
 en:Object.freeze({
  current:"graph: serving generation {generation} without verification (graph policy current)",
  verified:"graph: generation {generation} verified, {checked} facts checked",
  waiting:"graph: waiting for the graph build of a peer ({seconds} s in the queue, generation {generation} was the newest when this command queued)",
  covered:"graph: generation {generation} was sealed by a peer after this command began and already covers it, nothing is checked again",
  pinned:"graph: generation {generation} pinned by the invoking build serves this command ({count} projects), no queue and no check of the live tree",
  building:"graph: refreshing {dirty} projects (generation {generation})",
  published:"graph: generation {generation} published, {computed} projects refreshed in {seconds} s ({carried} kept their edges)",
  indexed:"graph: fact index of generation {generation}: {rows} facts of {owners} projects, {expanded} MB expanded ({percent} % of the {limit} MB limit), {packed} MB packed",
  swept:"graph: {removed} superseded observation blocks removed ({megabytes} MB, {headers} records kept their blocks)",
  closure:"graph: {count} projects of the closure verified, no other project read",
  cold:"graph: no usable generation ({reason}), building the first one",
  stale:"graph: serving generation {generation} built by another implementation (graph policy current)",
  planned:"graph: {count} projects planned by a child process that has exited",
  summary:"graph: {rows} stale facts ({kinds}) leave {projects} projects stale",
  reason:"graph: {project} is stale: {why}",
  reasonStamp:"the inference protocol of {inferrer} changed from {recorded} to {current}",
  reasonInput:"the input file {path} changed",
  reasonListing:"the entries of the directory {path} changed",
  reasonData:"the declared data file {path} changed",
  reasonAnalysis:"the source analysis of {path} changed",
  reasonFact:"the recorded {kind} fact of {path} changed",
  reasonDerived:"the derived result of {path} changed",
  cutoff:"graph: {questions} derived results asked again after {rows} stale facts: {held} unchanged, {changed} changed",
  reasonMembership:"the workspace file {path} was {change}",
  reasonMore:"graph: {count} more stale inputs are not listed",
  pendingCrate:"graph: the crate manifest {path} has no build project owner yet; it stays out of the graph and does not make the workspace stale until its owner row is registered (run {command})",
  withheldDependency:"graph: {project} is withheld: {source} depends on projects that this graph snapshot does not contain ({target}), created or removed while the graph was built; run the command again to include them, or remove them from the native sources of {project}",
  withheldSource:"graph: {project} is withheld: {source} could not be inferred (sha256 {sha}): {reason}; fix the cause and run the command again",
  changeAdded:"added",
  changeRemoved:"removed",
  changeChanged:"changed",
  merged:"graph: generation {generation} published, {computed} projects refreshed in {seconds} s ({carried} kept their edges) and merged onto the generation a peer published meanwhile ({theirs} projects of peers kept)",
  restored:"graph: generation {generation} ({owners} projects) restored from the remote cache, {documents} documents ({megabytes} MB) downloaded in {seconds} s, from a state {age} min old with {stale} projects stale",
  uploaded:"graph: generation {generation} ({owners} projects) uploaded to the remote cache, {documents} documents ({megabytes} MB) in {seconds} s",
  restoredHead:"graph: restored fleet head origin={origin} producerGeneration={generation} scope={scope} manifest={manifest}",
  uploadedHead:"graph: published fleet head origin={origin} producerGeneration={generation} scope={scope} manifest={manifest}",
  exchangeFailed:"graph: the remote cache did not exchange the generation ({reason}), going on without it",
  storing:"graph: all project results are in, storing {count} project fragments",
  healing:"graph: {count} projects carry edges of an older environment, deriving them again under the published one",
  indexing:"graph: fragments stored, merging the facts of {count} projects into the fact index",
  resumed:"graph: generation {generation} ({owners} projects) was built by a command that was cut off and is published now; the tree is checked against it next",
  resumedWaves:"graph: {done} of {total} projects were derived by a command that was cut off, deriving the rest",
  sealing:"graph: fact index merged, packing and publishing generation {generation}",
  sealedAge:"graph: answering from sealed generation {generation} ({owners} projects), {age} old, without verification and without inference (graph policy current)",
  projectNeverSealed:"graph: this machine holds no sealed generation of the workspace to answer {project} from, and a question about one project under graph policy current never starts the inference that would seal one. Start the whole read on purpose (repo graph --graph-policy fresh).",
  neverSealed:"graph: this machine holds no sealed generation of the whole workspace, and a question about every project never starts the hours of inference it would need.{fleet} {progress} Ask about one project (repo show project <name>), or start the whole read on purpose (repo graph --graph-policy fresh).",
  fleetHolds:" The fleet cache holds a sealed generation; the whole read fetches it first.",
  progressRunning:"The whole read of this machine is at {phase}, {done} of {total}, running for {held}.",
  progressIdle:"No whole read is running on this machine.",
 }),
 de:Object.freeze({
  current:"Graph: Generation {generation} wird ohne Prüfung geliefert (Graph-Richtlinie current)",
  verified:"Graph: Generation {generation} geprüft, {checked} Fakten kontrolliert",
  waiting:"Graph: wartet auf den Graph-Aufbau eines anderen Prozesses ({seconds} s in der Warteschlange, Generation {generation} war beim Einreihen die neueste)",
  covered:"Graph: Generation {generation} wurde von einem anderen Prozess nach dem Beginn dieses Befehls versiegelt und deckt ihn bereits ab, nichts wird erneut geprüft",
  pinned:"Graph: die vom aufrufenden Build gepinnte Generation {generation} bedient diesen Befehl ({count} Projekte), ohne Warteschlange und ohne Prüfung des Arbeitsbaums",
  building:"Graph: {dirty} Projekte werden erneuert (Generation {generation})",
  published:"Graph: Generation {generation} veröffentlicht, {computed} Projekte in {seconds} s erneuert ({carried} behielten ihre Kanten)",
  indexed:"Graph: Faktenindex der Generation {generation}: {rows} Fakten von {owners} Projekten, {expanded} MB entpackt ({percent} % der Grenze von {limit} MB), {packed} MB gepackt",
  swept:"Graph: {removed} überholte Beobachtungsblöcke entfernt ({megabytes} MB, {headers} Aufzeichnungen behalten ihre Blöcke)",
  closure:"Graph: {count} Projekte des Abschlusses geprüft, kein weiteres Projekt gelesen",
  cold:"Graph: keine brauchbare Generation ({reason}), die erste wird aufgebaut",
  stale:"Graph: Generation {generation} stammt von einer anderen Implementierung und wird geliefert (Graph-Richtlinie current)",
  planned:"Graph: {count} Projekte von einem beendeten Kindprozess geplant",
  summary:"Graph: {rows} veraltete Fakten ({kinds}) lassen {projects} Projekte veralten",
  reason:"Graph: {project} ist veraltet: {why}",
  reasonStamp:"das Inferenzprotokoll von {inferrer} wechselte von {recorded} zu {current}",
  reasonInput:"die Eingabedatei {path} hat sich geändert",
  reasonListing:"die Einträge des Verzeichnisses {path} haben sich geändert",
  reasonData:"die deklarierte Datendatei {path} hat sich geändert",
  reasonAnalysis:"die Quellanalyse von {path} hat sich geändert",
  reasonFact:"der aufgezeichnete Fakt {kind} von {path} hat sich geändert",
  reasonDerived:"das abgeleitete Ergebnis von {path} hat sich geändert",
  cutoff:"Graph: {questions} abgeleitete Ergebnisse nach {rows} veralteten Fakten erneut abgefragt: {held} unverändert, {changed} verändert",
  reasonMembership:"die Arbeitsbereichsdatei {path} wurde {change}",
  reasonMore:"Graph: {count} weitere veraltete Eingaben werden nicht aufgelistet",
  pendingCrate:"Graph: das Crate-Manifest {path} hat noch keinen Build-Projekt-Eigner; es bleibt außerhalb des Graphen und macht den Arbeitsbereich nicht veraltet, bis seine Eigner-Zeile eingetragen ist ({command} ausführen)",
  withheldDependency:"Graph: {project} wird zurückgehalten: {source} hängt von Projekten ab, die dieser Graph-Schnappschuss nicht enthält ({target}); sie wurden während des Graph-Aufbaus angelegt oder entfernt; den Befehl erneut ausführen, um sie aufzunehmen, oder sie aus den nativen Quellen von {project} entfernen",
  withheldSource:"Graph: {project} wird zurückgehalten: {source} konnte nicht abgeleitet werden (sha256 {sha}): {reason}; die Ursache beheben und den Befehl erneut ausführen",
  changeAdded:"hinzugefügt",
  changeRemoved:"entfernt",
  changeChanged:"geändert",
  merged:"Graph: Generation {generation} veröffentlicht, {computed} Projekte in {seconds} s erneuert ({carried} behielten ihre Kanten) und auf die Generation eines anderen Prozesses zusammengeführt ({theirs} Projekte anderer Prozesse übernommen)",
  restored:"Graph: Generation {generation} ({owners} Projekte) aus dem Remote-Cache wiederhergestellt, {documents} Dokumente ({megabytes} MB) in {seconds} s geladen, von einem Stand von vor {age} Min. mit {stale} veralteten Projekten",
  uploaded:"Graph: Generation {generation} ({owners} Projekte) in den Remote-Cache hochgeladen, {documents} Dokumente ({megabytes} MB) in {seconds} s",
  restoredHead:"Graph: Flottenstand wiederhergestellt origin={origin} producerGeneration={generation} scope={scope} manifest={manifest}",
  uploadedHead:"Graph: Flottenstand veröffentlicht origin={origin} producerGeneration={generation} scope={scope} manifest={manifest}",
  exchangeFailed:"Graph: der Remote-Cache hat die Generation nicht ausgetauscht ({reason}), es geht ohne sie weiter",
  storing:"Graph: alle Projektergebnisse liegen vor, {count} Projektfragmente werden gespeichert",
  healing:"Graph: {count} Projekte tragen Kanten einer älteren Umgebung, sie werden unter der veröffentlichten Umgebung neu abgeleitet",
  indexing:"Graph: Fragmente gespeichert, die Fakten von {count} Projekten werden in den Faktenindex eingefügt",
  resumed:"Graph: Generation {generation} ({owners} Projekte) wurde von einem abgebrochenen Befehl aufgebaut und wird jetzt veröffentlicht; danach wird der Arbeitsbaum dagegen geprüft",
  resumedWaves:"Graph: {done} von {total} Projekten wurden von einem abgebrochenen Befehl abgeleitet, der Rest wird abgeleitet",
  sealing:"Graph: Faktenindex zusammengeführt, Generation {generation} wird verpackt und veröffentlicht",
  sealedAge:"Graph: Antwort aus der versiegelten Generation {generation} ({owners} Projekte), {age} alt, ohne Prüfung und ohne Ableitung (Graph-Richtlinie current)",
  projectNeverSealed:"Graph: Diese Maschine hat keine versiegelte Generation des Arbeitsbereichs, aus der {project} beantwortet werden könnte, und eine Frage nach einem Projekt unter der Graph-Richtlinie current startet nie die Ableitung, die eine versiegelte Generation erzeugen würde. Starte das ganze Lesen absichtlich (repo graph --graph-policy fresh).",
  neverSealed:"Graph: Diese Maschine hat keine versiegelte Generation des ganzen Arbeitsbereichs, und eine Frage über alle Projekte startet nie die stundenlange Ableitung, die sie bräuchte.{fleet} {progress} Frage nach einem Projekt (repo show project <Name>) oder starte das ganze Lesen absichtlich (repo graph --graph-policy fresh).",
  fleetHolds:" Der Flotten-Cache hält eine versiegelte Generation; das ganze Lesen holt sie zuerst.",
  progressRunning:"Das ganze Lesen dieser Maschine steht bei {phase}, {done} von {total}, seit {held}.",
  progressIdle:"Auf dieser Maschine läuft kein ganzes Lesen.",
 }),
});

/** 💬️ Renders one message in the language the caller resolved with the shared resolver. */
export function renderGenerationMessageV1(key,fields,language){
 const template=GENERATION_MESSAGES_V1[language]?.[key];
 if(template===undefined)throw Error("Closed generation message required: "+key);
 return template.replace(/\{([a-z]+)\}/gu,(_token,name)=>String(fields[name]));
}

/** 🚧️ The fields of the message that words one withheld project: the project, its declaring source, the projects it waits for, the start of the digest of the observed bytes and the reason it recorded (a source that does not parse, a failure of the inference itself: the file bytes are only the identity of the withholding, never the diagnosis). */
function withheldFieldsV1(row){return{project:row.project??row.root,source:row.source,target:row.missing?.join(", ")??"",sha:row.sha256.slice(0,12),reason:String(row.reason??"").replace(/\s+/gu," ")};}

/** 🕸️ Words one withheld project in the language of the caller: the projects a test project waits for, or the source that does not parse. */
export function describeUnavailableProjectV1(row,language){return renderGenerationMessageV1(row.missing?"withheldDependency":"withheldSource",withheldFieldsV1(row),language);}

/**
 * 🌐️ Withholds from one graph snapshot every test project that depends on a project the snapshot does not hold: its row (declaring file from `configs`, digest of that file from `digest`, English reason, the missing names) joins `unavailable` under the project root and the project leaves `projects`.
 * Test inference reads project names from the live disk while the inventory is the snapshot of one run, so a project created or removed meanwhile is a dependency "not yet in this generation": only the test project waits and the next run finds the project. Returns the names withheld.
 */
export function withholdDanglingTestProjectsV1({projects,configs,unavailable,dangling,digest}){
 for(const{source,root,missing}of dangling){
  const file=configs[source]??root;
  unavailable.set(root,{root,project:source,source:file,sha256:digest(file),reason:renderGenerationMessageV1("withheldDependency",{project:source,source:file,target:missing.join(", ")},"en").slice(0,GENERATION_POLICY_V1.withheldReasonCharacters),missing:[...missing]});
  delete projects[source];
 }
 return dangling.map(row=>row.source);
}

/** 📣️ Says, once per build, which projects the partial inference withheld and what repairs each of them. */
export function announceWithheldProjectsV1(host,part){for(const owner of part.owners)if(owner.unavailable)host.say(owner.unavailable.missing?"withheldDependency":"withheldSource",withheldFieldsV1(owner.unavailable));}

/** 🚦️ Reads the graph policy from `--graph-policy fresh|current` (before `--`) or the environment; `--graph` is taken by the portable dry-run alias. Returns the policy, whether anything chose it and the remaining arguments. */
export function extractGraphPolicyV1(args,environment){
 let policy=environment.SEMIO_BUILD_GRAPH_POLICY??"fresh",explicit=environment.SEMIO_BUILD_GRAPH_POLICY!==undefined;const remaining=[];
 for(let index=0;index<args.length;index++){
  const argument=args[index];
  if(argument==="--"){remaining.push(...args.slice(index));break;}
  if(argument==="--graph-policy"){policy=args[++index]??"";explicit=true;continue;}
  if(argument.startsWith("--graph-policy=")){policy=argument.slice("--graph-policy=".length);explicit=true;continue;}
  remaining.push(argument);
 }
 if(!GENERATION_POLICY_V1.policies.includes(policy))throw Error("Graph policy fresh or current required: "+policy);
 return{policy,explicit,args:remaining};
}

/** 🧱️ Splits the facts of the loaded implementation by their consumers: a data file nobody covers belongs to every project and the environment, the stamp of an inferrer of scope `all` or `environment` belongs to the environment, and the stamp of a project inferrer belongs to nobody here because every project record names exactly the stamps it took part in. */
export function scopeImplementationRowsV1(rows){
 const all=[],environment=[],seen=new Set();
 for(const{scope,...row}of rows){
  const key=rowKeyV1(row);if(seen.has(key))continue;seen.add(key);
  if(row.kind!=="stamp")all.push(row);else if(scope==="environment"||scope==="all")environment.push(row);
 }
 return{all,environment};
}

/** 🪪️ The digest that stamps a generation: the declared stamp of every inferrer and the bytes of every data file the implementation reads, never the bytes of code. */
export function implementationDigestV1(rows){
 return createHash("sha256").update(canonicalJsonV1(rows.map(row=>[row.kind,row.path,row.kind==="stamp"?row.fact:row.fact.sha256]).sort((a,b)=>compareCodePointsV1(a[1],b[1])||compareCodePointsV1(String(a[2]),String(b[2]))))).digest("hex");
}

/** 🪆️ The tasks an orchestrating target launches as nested runs (`metadata.semio.execution` of kind `orchestration`): the nested runs read the generation the build pins, so a slice needs their closure too. */
const orchestrationChildrenV1=operation=>{const execution=operation?.metadata?.semio?.execution;return execution?.kind==="orchestration"&&Array.isArray(execution.children)?execution.children.map(child=>({project:child.project,target:child.target})):[];};

/** 🪶️ The part of a fragment that {@link sliceClosureV1} reads: the targets of its edges, per target the input references and `projects` rows of its inputs, its `dependsOn` and its declared orchestration children, and the same of its named input groups. A closure of hundreds of projects keeps this instead of every parsed node while it is computed. */
export function sliceViewV1(fragment){
 const reads=inputs=>(Array.isArray(inputs)?inputs:[]).flatMap(input=>typeof input==="string"?input.startsWith("{")||input.startsWith("!")?[]:[input]:input&&typeof input==="object"&&(typeof input.input==="string"||input.projects!==undefined)?[Object.fromEntries(["input","projects","dependencies"].filter(key=>input[key]!==undefined).map(key=>[key,input[key]]))]:[]);
 const targets={},named=fragment.node?.namedInputs;
 for(const[name,operation]of Object.entries(fragment.node?.targets??{})){const children=orchestrationChildrenV1(operation);targets[name]={inputs:reads(operation?.inputs),dependsOn:operation?.dependsOn,...(children.length?{metadata:{semio:{execution:{kind:"orchestration",children}}}}:{})};}
 return{edges:fragment.edges.map(edge=>({target:edge.target})),node:named&&typeof named==="object"?{targets,namedInputs:Object.fromEntries(Object.entries(named).map(([group,inputs])=>[group,reads(inputs)]))}:{targets}};
}

/**
 * 🍰️ The owners a selected scope needs, as a fixpoint over the generation: every admitted owner admits the owners it imports (its dependency edges, transitively); only the active targets follow the owners their `inputs` name as `projects`, through their named input groups too (`group`, `^group` over the dependency edges, the group a `projects` row names on that owner), their `dependsOn` (`^target` over the dependency edges, `project:target`, `target`, objects with `projects` and `dependencies`, `*` for the active target only) and their declared orchestration children (a nested run is served from the generation the outer build pins, so its tasks belong to the slice). Catalog provider owners join alone (not the owners they import: a consumer reads their catalog source file only) when an active owner lies below one of their consumer roots.
 * `owners` maps every owner name to its root, `external` names the external nodes; `fragment(name)` yields `{edges, node}`. A task, a root or an edge to a name that is neither an owner nor an external node refuses with `GENERATION_UNKNOWN_OWNER`: a slice is never silently incomplete.
 */
export async function sliceClosureV1({owners,fragment,selection,roots=[],providers=[],external=[]}){
 const cache=new Map(),known=new Set(Object.keys(owners)),outside=new Set(external),admitted=new Set(),active=new Map(),queue=[];
 const get=async name=>{let row=cache.get(name);if(!row){row=await fragment(name);cache.set(name,row);}return row;};
 const unknown=name=>Object.assign(Error("Unknown project in the closure: "+name),{code:"GENERATION_UNKNOWN_OWNER"});
 const admit=async name=>{
  const todo=[name];
  while(todo.length){const at=todo.pop();if(admitted.has(at))continue;if(!known.has(at)){if(at===name||!outside.has(at))throw unknown(at);continue;}admitted.add(at);for(const edge of(await get(at)).edges)todo.push(edge.target);}
 };
 const task=async(name,target,optional)=>{
  if(!known.has(name))throw unknown(name);await admit(name);
  if(optional&&!(await get(name)).node?.targets?.[target])return;
  const set=active.get(name)??new Set();if(!set.has(target)){set.add(target);active.set(name,set);queue.push([name,target]);}
 };
 const groups=new Set(),imported=async name=>[...new Set((await get(name)).edges.map(edge=>edge.target))].filter(value=>known.has(value));
 const reach=async(name,inputs)=>{for(const input of Array.isArray(inputs)?inputs:[]){
  if(typeof input==="string"){const group=input.startsWith("^")?input.slice(1):input;for(const owner of input.startsWith("^")?await imported(name):[name]){const key=owner+"\0"+group,values=(await get(owner)).node?.namedInputs?.[group];if(!Array.isArray(values)||groups.has(key))continue;groups.add(key);await reach(owner,values);}}
  else if(input&&typeof input==="object"&&(typeof input.input==="string"||input.projects!==undefined))for(const owner of input.dependencies===true?await imported(name):input.projects===undefined?[name]:typeof input.projects==="string"?[input.projects]:Array.isArray(input.projects)?input.projects:[]){if(!known.has(owner))continue;await admit(owner);if(typeof input.input==="string")await reach(owner,[input.input]);}
 }};
 if(selection.kind==="tasks")for(const row of selection.tasks)await task(row.project,row.target,false);else{for(const name of roots)await admit(name);for(const name of roots)for(const target of selection.targets??[])await task(name,target,true);}
 while(queue.length){
  const[name,target]=queue.shift(),row=await get(name),operation=row.node?.targets?.[target];if(!operation)continue;
  const dependencies=[...new Set(row.edges.map(edge=>edge.target))].filter(value=>known.has(value));
  await reach(name,operation.inputs);
  for(const dependency of Array.isArray(operation.dependsOn)?operation.dependsOn:[]){
   if(typeof dependency==="string"){
    if(dependency.startsWith("^"))for(const value of dependencies)await task(value,dependency.slice(1),true);
    else if(dependency.includes(":")){const at=dependency.lastIndexOf(":");await task(dependency.slice(0,at),dependency.slice(at+1),false);}
    else await task(name,dependency,false);
   }else if(dependency&&typeof dependency==="object"&&typeof dependency.target==="string"){
    const projects=dependency.dependencies?dependencies:dependency.projects===undefined?[name]:dependency.projects==="*"?[...known]:typeof dependency.projects==="string"?[dependency.projects]:dependency.projects;
    for(const value of projects)await task(value,dependency.target,dependency.dependencies===true);
   }
  }
  for(const child of orchestrationChildrenV1(operation))await task(child.project,child.target,false);
 }
 const inside=(root,path)=>root==="."||path===root||path.startsWith(root+"/");
 for(const provider of providers){
  if(![...active.keys()].some(name=>provider.consumerRoots.some(root=>inside(root,owners[name]))))continue;
  let best;for(const[name,root]of Object.entries(owners))if(inside(root,provider.sourcePath)&&(best===undefined||root.length>owners[best].length))best=name;
  if(best!==undefined)admitted.add(best);
 }
 return admitted;
}

/** 🪦️ The previous owners a build still knows: an owner whose config was removed is dropped by the generation, so it must neither stay a kept stub of the inference (its edges would be refreshed for an owner that no longer exists) nor a member of the graph context. */
export function survivingStubsV1(stubs,removed){const gone=new Set(removed);return Object.fromEntries(Object.entries(stubs).filter(([name])=>!gone.has(name)));}

/** 🔢️ The edges of one owner in the order of their canonical text, so the fragment of an owner does not depend on the order in which its files were visited (a discovery of all owners and of some of them publish the same bytes). */
function orderedEdgesV1(edges){return edges.map(edge=>[canonicalJsonV1(edge),edge]).sort((a,b)=>a[0]<b[0]?-1:a[0]>b[0]?1:0).map(row=>row[1]);}

/** ♻️ After how many batches of rows the streamed claim collects garbage, so the parsed rows of the pass never pile up in the resident set. */
const COLLECT_EVERY_BATCHES=32;

/** 🗃️ Whether rows are a row log list (counted, keyed, read in batches) and not an array of row objects. */
const lazyRowsV1=rows=>!Array.isArray(rows)&&typeof rows?.concat==="function"&&typeof rows.keys==="function";

/** 🧩️ Shapes what one partial inference produced into the compute result of the store: every owner carries the facts of its own inference record plus the captured facts below its roots, edges go to their source owner, facts nobody consumed belong to the environment.
 * `workspaceRoot` makes row paths comparable (rows carry absolute paths); the root project never claims facts by path. A part whose `rows` is a row log list is claimed in one streamed pass over the log (ids only, a batch of rows at a time); a part with an array of rows is claimed over the sorted array.
 */
export function shapeComputeResultV1({workspaceRoot,part,stubs,separator="/",implementation={all:[],environment:[]}}){
 const processed=new Set([...part.owners.map(owner=>owner.name),...part.edgeOnly.map(owner=>owner.name)]),edgesOf=new Map(),other=[];
 for(const edge of part.edges){if(processed.has(edge.source)){let list=edgesOf.get(edge.source);if(!list)edgesOf.set(edge.source,list=[]);list.push(edge);}else other.push(edge);}
 const prefix=root=>root==="."?undefined:workspaceRoot+separator+root.split("/").join(separator)+separator,prefixes=roots=>roots.filter(Boolean).map(prefix).filter(Boolean);
 const slots=[],claim=(base,roots)=>slots.push({base,prefixes:prefixes(roots)})-1;
 const joined=(rows,extra)=>lazyRowsV1(rows)?rows.concat(extra):[...rows,...extra];
 const owners=[],slotOf=[];
 for(const owner of part.owners){slotOf.push(claim(owner.nodeRows??[],[owner.root,owner.nativeRoot]));owners.push({name:owner.name,root:owner.root,kind:owner.unavailable?"unavailable":"project",node:owner.unavailable??owner.node,edges:orderedEdgesV1(edgesOf.get(owner.name)??[]),targets:owner.unavailable?[]:Object.keys(owner.node?.targets??{}),...(owner.config?{config:owner.config}:{}),...(owner.nativeRoot?{nativeRoot:owner.nativeRoot}:{}),...(owner.tag?{tag:owner.tag}:{}),...(owner.reads?{reads:owner.reads}:{})});}
 const edgeOnly=[];
 for(const owner of part.edgeOnly){const stub=stubs[owner.name];edgeOnly.push({name:owner.name,slot:claim([],[stub.root,stub.nativeRoot]),...(owner.streamed?{}:{edges:orderedEdgesV1(edgesOf.get(owner.name)??[]),...(part.readsOf?.(owner.name)?{reads:part.readsOf(owner.name)}:{})})});}
 const claims=lazyRowsV1(part.rows)&&part.rows.log!==undefined?claimStreamedV1({all:part.rows,slots,workspaceRoot,separator}):claimListedV1({all:part.rows,slots}),all=implementation.all;
 owners.forEach((owner,at)=>{owner.rows=joined(claims.rows[slotOf[at]],all);});
 for(const owner of edgeOnly){const{name,slot,...rest}=owner;owners.push({name,partial:"edges",...rest,rows:claims.rows[slot]});}
 const environment=claims.environment(part.env.rows,[...implementation.all,...implementation.environment]);
 part.rows?.log?.seal?.();
 return{owners,...(part.carried?.length?{carried:part.carried}:{}),...(part.readsOf?{readsOf:part.readsOf}:{}),...(part.reEdge?{reEdge:async(name,fragment)=>orderedEdgesV1(await part.reEdge(name,fragment))}:{}),...(part.derived?.length?{derived:part.derived}:{}),env:{digest:part.env.digest,rows:environment,...(part.env.parts?{parts:part.env.parts}:{})},global:{version:1,externalNodes:part.externalNodes??{},edges:other,unavailable:part.parseUnavailable??[]},replaceTags:part.replaceTags??[],...(Number.isSafeInteger(part.inferred)?{inferred:part.inferred}:{}),...(part.phases?{phases:part.phases}:{}),...(part.dispose?{dispose:part.dispose}:{})};
}

/** 📚️ Claims over an array of rows: the rows below the roots of each slot come from a path-sorted copy of the array; what no slot holds is the environment's. */
function claimListedV1({all,slots}){
 const unclaimed=new Set(),ordered=all.map((row,at)=>{unclaimed.add(rowKeyV1(row));return[row.path,at];}).sort((a,b)=>a[0]<b[0]?-1:a[0]>b[0]?1:a[1]-b[1]);
 const under=entry=>{let low=0,high=ordered.length;while(low<high){const middle=(low+high)>>>1;if(ordered[middle][0]<entry)low=middle+1;else high=middle;}const found=[];for(;low<ordered.length&&ordered[low][0].startsWith(entry);low++)found.push(ordered[low][1]);return found;};
 const keysOf=rows=>lazyRowsV1(rows)?rows.keys():(function*(){for(const row of rows)yield rowKeyV1(row);})(),joined=(rows,extra)=>lazyRowsV1(rows)?rows.concat(extra):[...rows,...extra];
 const claimedLists=new WeakSet(),claim=({base,prefixes:list})=>{
  const found=list.length?[...new Set(list.flatMap(under))].sort((a,b)=>a-b):[],wanted=new Map();
  for(const at of found){const row=all[at],key=rowKeyV1(row);unclaimed.delete(key);if(!wanted.has(key))wanted.set(key,row);}
  const claimBase=unclaimed.size>0&&!claimedLists.has(base);
  if(claimBase)claimedLists.add(base);
  if(claimBase||wanted.size)for(const key of keysOf(base)){
   if(claimBase)unclaimed.delete(key);
   if(wanted.size)wanted.delete(key);
   if(!wanted.size&&(!claimBase||!unclaimed.size))break;
  }
  return wanted.size?joined(base,[...wanted.values()]):base;
 };
 const rows=slots.map(claim);
 return{rows,environment:(own,extra)=>[...own,...(unclaimed.size?all.filter(row=>unclaimed.has(rowKeyV1(row))):[]),...extra]};
}

/** 🌊️ Claims over a row log list in one pass: every row of the log is read once in batches, the slots whose roots lie above its path are found by its ancestor directories, and only row numbers are kept (a bitmap of the claimed numbers, the numbers a slot holds beyond its own list). Memory is a few bytes per row whatever the number of owners. */
function claimStreamedV1({all,slots,workspaceRoot,separator}){
 const log=all.log,start=workspaceRoot+separator,batch=2048;
 let bits=new Uint8Array((log.size()>>>3)+8);
 const mark=id=>{const byte=id>>>3;if(byte>=bits.length){const next=new Uint8Array(Math.max(byte+8,bits.length*2));next.set(bits);bits=next;}bits[byte]|=1<<(id&7);},marked=id=>(id>>>3)<bits.length&&(bits[id>>>3]&(1<<(id&7)))!==0;
 const idsOf=base=>lazyRowsV1(base)&&base.log===log&&!base.tails.length?base.ids:Uint32Array.from(lazyRowsV1(base)?base.rows():base,row=>log.id(row));
 const known=new Map(),sortedOf=base=>{let ids=known.get(base);if(ids===undefined){ids=base.length?Uint32Array.from(idsOf(base)).sort():new Uint32Array(0);known.set(base,ids);}return ids;};
 const has=(ids,id)=>{let low=0,high=ids.length;while(low<high){const middle=(low+high)>>>1;if(ids[middle]<id)low=middle+1;else high=middle;}return low<ids.length&&ids[low]===id;};
 const byPrefix=new Map(),sorted=slots.map(slot=>sortedOf(slot.base)),extras=slots.map(()=>[]);
 slots.forEach((slot,at)=>{for(const prefix of slot.prefixes){const list=byPrefix.get(prefix);if(list===undefined)byPrefix.set(prefix,[at]);else if(list[list.length-1]!==at)list.push(at);}});
 const visit=(id,row)=>{
  const path=row.path;if(!byPrefix.size||typeof path!=="string"||!path.startsWith(start))return;
  for(let at=start.length;;){const next=path.indexOf(separator,at);if(next<0)return;const hit=byPrefix.get(path.slice(0,next+1));if(hit!==undefined)for(const slot of hit)if(!has(sorted[slot],id)){const list=extras[slot];if(list[list.length-1]!==id)list.push(id);}at=next+1;}
 };
 const ids=all.ids;
 if(byPrefix.size>0)for(let at=0;at<ids.length;at+=batch){const slice=ids.subarray(at,Math.min(ids.length,at+batch)),rows=log.read(slice);for(let item=0;item<slice.length;item++)visit(slice[item],rows[item]);if((at/batch)%COLLECT_EVERY_BATCHES===COLLECT_EVERY_BATCHES-1)runtimeCollectGarbageV1(true);}
 const tailIds=[];for(const tail of all.tails)for(const row of tail){const id=log.id(row);tailIds.push(id);visit(id,row);}
 const marked0=new Set();for(const slot of slots){if(marked0.has(slot.base))continue;marked0.add(slot.base);for(const id of sortedOf(slot.base))mark(id);}
 for(const list of extras)for(const id of list)mark(id);
 const rows=slots.map((slot,at)=>extras[at].length===0?slot.base:log.list([...idsOf(slot.base),...extras[at]]));
 return{rows,environment:(own,extra)=>{
  const builder=log.builder(),unclaimed=function*(rows){for(const id of rows)if(!marked(id))yield id;};
  builder.add(own);
  builder.addIds(unclaimed(ids));
  builder.addIds(unclaimed(tailIds));
  return builder.finish().concat(extra);
 }};
}


/** 🔍️ Names the exact input behind one stale reason of a plan: the stamp of an inferrer, an input file, a directory listing, a declared data file, a source analysis or a workspace file that appeared, vanished or changed. `current` maps stamp paths to the stamps this process carries. */
export function describeGenerationReasonV1(reason,{workspaceRoot,separator="/",current=new Map(),language}){
 const display=path=>path.startsWith(workspaceRoot+separator)?path.slice(workspaceRoot.length+separator.length).split(separator).join("/"):path;
 const project=reason.owners?.[0]??"@env";let key,fields;
 if(reason.change){key="reasonMembership";fields={path:display(reason.change.path),change:renderGenerationMessageV1({added:"changeAdded",removed:"changeRemoved"}[reason.change.kind]??"changeChanged",{},language)};}
 else{
  const{kind,path}=reason.row;
  if(kind==="stamp"){key="reasonStamp";fields={inferrer:path.replace(/^inferrer:/u,""),recorded:String(reason.row.fact),current:current.get(path)??"?"};}
  else if(kind==="file"){key="reasonInput";fields={path:display(path)};}
  else if(kind==="digest"||kind==="field"){key="reasonData";fields={path:display(path)};}
  else if(kind==="readdir"||kind==="readdirNames"||kind==="directories"){key="reasonListing";fields={path:display(path)};}
  else if(kind==="analysis"){key="reasonAnalysis";fields={path:display(path)};}
  else if(kind==="derived"){key="reasonDerived";let first=path;try{first=JSON.parse(path).roots?.[0]??path;}catch{}fields={path:display(String(first)).slice(0,GENERATION_POLICY_V1.pathDisplayCharacters)};}
  else{key="reasonFact";fields={kind,path:display(path)};}
 }
 const why=renderGenerationMessageV1(key,fields,language);
 return{project,key,fields,why,text:renderGenerationMessageV1("reason",{project,why},language)};
}

/** 📢️ Says why a build starts: the reason of a cold start, or the number of stale projects followed by the first stale inputs by name. */
export function announceGenerationBuildV1(host,spec,rows){
 if(spec.cold){host.say("cold",{reason:spec.coldReason??"empty"});return;}
 if(spec.done)return;host.say("building",{dirty:spec.total??spec.owners.length,generation:spec.generation??0});
 if(spec.staleKinds&&(spec.staleRows??0)>0)host.say("summary",{rows:spec.staleRows,kinds:Object.entries(spec.staleKinds).sort((x,y)=>y[1]-x[1]||(x[0]<y[0]?-1:1)).map(([kind,count])=>kind+" "+count).join(", "),projects:spec.total??spec.owners.length});
 const current=new Map(rows.filter(row=>row.kind==="stamp").map(row=>[row.path,row.fact])),language=host.language;
 for(const reason of spec.reasons??[]){const row=describeGenerationReasonV1(reason,{workspaceRoot:host.workspaceRoot,separator:host.separator,current,language});host.say("reason",{project:row.project,why:row.why});}
 const listed=(spec.reasons??[]).filter(reason=>reason.row).length;
 if((spec.staleRows??0)>listed)host.say("reasonMore",{count:spec.staleRows-listed});
}

/** 📊️ Turns the store's progress events into the shared graph phase counters with their bilingual labels; concurrent workers report in one order (a phase is opened once, then advanced), however their events interleave. */
const progressSinksV1=new Set();
/** 📡️ Registers a listener of the phase counters of every generation load of this process (a worker reports them to its service): `{phase, done, total}` at most once per `progressReportMilliseconds` and on every completed phase. Returns the function that removes it. */
export function onGenerationProgressV1(listener){progressSinksV1.add(listener);return()=>progressSinksV1.delete(listener);}

/** 🫀️ Hands every counted phase event of a build also to the claim that holds the build (the door board judges a holder by the last advance of its claim, not by the rows the sidecar watch reads), so no stage of a whole read is silent to either. A build without a claim object, and the running-owner lists, pass through unchanged. */
export function custodyProgressV1(builder,emit){
 if(typeof builder!=="object"||builder===null||typeof builder.progress!=="function")return emit;
 return event=>{
  if(event.running===undefined&&typeof event.phase==="string"&&Number.isSafeInteger(event.done))Promise.resolve(builder.progress({phase:event.phase,done:event.done,total:event.total})).catch(()=>undefined);
  return emit(event);
 };
}

export function createGenerationProgressV1(clock){
 const active=new Set(),seen=new Map();let chain=Promise.resolve(),reportedAt=-Infinity;
 const apply=async({phase,done,total,running,decision})=>{
  if(running!==undefined){if(!active.has(phase))return;await clock.running?.(phase,running,decision);const at=performance.now();if(progressSinksV1.size&&at-reportedAt>=GENERATION_POLICY_V1.progressReportMilliseconds){reportedAt=at;for(const sink of progressSinksV1)try{sink({phase,done,total,running,...decision===undefined?{}:{decision}});}catch{}}return;}
  if(done===0||!active.has(phase)){if(active.has(phase))await clock.end(phase);await clock.begin(phase,total);active.add(phase);seen.set(phase,-1);}
  else if(done<seen.get(phase))return;
  seen.set(phase,done);await clock.advance(phase,done);
  const at=performance.now();if(progressSinksV1.size&&(done>=total||at-reportedAt>=GENERATION_POLICY_V1.progressReportMilliseconds)){reportedAt=at;for(const sink of progressSinksV1)try{sink({phase,done,total});}catch{}}
  if(done>=total){await clock.end(phase);active.delete(phase);}
 };
 const handle=async event=>{
  const{phase,done,total,running}=event;
  if(typeof phase!=="string"||!/^[a-z]+$/u.test(phase))return;
  if(running===undefined&&(!Number.isSafeInteger(done)||done<0||!Number.isSafeInteger(total)||total<0))return;
  try{await apply(event);}catch{active.delete(phase);seen.delete(phase);try{await clock.end(phase);}catch{}}
 };
 return event=>{const run=chain.then(()=>handle(event));chain=run.catch(()=>undefined);return run;};
}

/** 🕓️ The pending crate manifests this process has already reported, so each is named once. */
const pendingCratesReportedV1=createBoundedCacheV1({name:"generation:pending-crates",maxEntries:GENERATION_POLICY_V1.pendingCrateReports,maxBytes:GENERATION_POLICY_V1.pendingCrateReports});
const needsBuild=async(_body,pending)=>({built:false,published:false,needsBuild:true,computed:0,pending});

/** 📦️ Expands fixed project/UTF8-byte groups once and preserves every caught consumer failure. */
export async function expandInChunksV1(projects,expand,{maximumProjects,maximumBytes}){
 const names=Object.keys(projects),result={};
 const pass=async group=>{
  const chunk={};for(const name of group)chunk[name]=projects[name];
  Object.assign(result,await expand(chunk));
 };
 let group=[],bytes=0;
 for(const name of names){const size=Buffer.byteLength(JSON.stringify(projects[name]));if(group.length&&(group.length>=maximumProjects||bytes+size>maximumBytes)){await pass(group);group=[];bytes=0;}group.push(name);bytes+=size;}
 if(group.length)await pass(group);
 return result;
}

/** 🌊️ The streaming twin of {@link expandInChunksV1}: the same fixed project and UTF8 byte groups, formed from `[name, node]` pairs as they arrive, so the raw nodes of a group are released as soon as the group is expanded and only the expanded projects accumulate. */
export async function expandStreamV1(entries,expand,{maximumProjects,maximumBytes}){
 const result={};
 const pass=async group=>{Object.assign(result,await expand(Object.fromEntries(group)));};
 let group=[],bytes=0;
 for await(const entry of entries){const size=Buffer.byteLength(JSON.stringify(entry[1]));if(group.length&&(group.length>=maximumProjects||bytes+size>maximumBytes)){await pass(group);group=[];bytes=0;}group.push(entry);bytes+=size;}
 if(group.length)await pass(group);
 return result;
}

/** ⬇️ Lets an empty generation store take the generation another machine of the group uploaded to the shared remote cache; a refusal or a failure only says so and the build goes on. */
async function restoreRemoteGenerationV1(host,options,implementation,claim,fetchOnly,expectedHead,beforePublication){
 const started=Date.now(),files=options.fs??createNodeFilePortV1(),pointer=join(options.root,GENERATION_POLICY_V1.pointer);
 const fs=claim?.fence?{...files,write:(path,...args)=>path===pointer?claim.fence(async()=>{
  if(beforePublication){
   const next=readGenerationPointerV1(args[0]);
   if(next.manifest===null)throw Error("An adoption must publish a sealed generation");
   await beforePublication({generation:next.generation,localManifest:next.manifest});
   host.signal?.throwIfAborted();
  }
  return files.write(path,...args);
 },{signal:host.signal}):files.write(path,...args)}:files;
 const outcome=await host.exchange.restore({root:options.root,fs,signal:host.signal,implementation,...fetchOnly?{requireComplete:true}:{},...expectedHead?{expectedHead}:{}});
 if(fetchOnly)host.signal?.throwIfAborted();
 if(outcome.status==="restored"&&!expectedHead){
  if(outcome.head)host.say("restoredHead",outcome.head);
  host.say("restored",{generation:outcome.generation,owners:outcome.owners,documents:outcome.documents,megabytes:Math.round(outcome.bytes/1048576),seconds:Math.round((Date.now()-started)/1000),age:Math.round((outcome.ageSeconds??0)/60),stale:outcome.stale??"?"});
 }
 else if(outcome.status==="failed"||outcome.status==="incomplete")host.say("exchangeFailed",{reason:outcome.reason??outcome.status});
 return outcome;
}

/** 🧲️ Settles closure leaves of the exact restored manifest before deciding whether it still needs a rebuild. */
async function settleRestoredClosuresV1(host,engine,outcome,implementation,claim,prepare){
 const missing=outcome?.closures?.missing??[];
 if(!missing.length||outcome.status!=="restored"||outcome.localManifest===undefined)return;
 const current=await engine.read(host.signal).catch(()=>undefined);
 if(!current?.manifest||current.pointer.generation!==outcome.generation||current.pointer.manifest!==outcome.localManifest)return;
 const projects=new Set();
 const unavailable=[];
 for(const row of missing){
  const verdict=await host.settleClosure?.({key:row.key,namespace:row.namespace,projects:row.projects}).catch(()=>undefined);
  if(verdict==="refetched")continue;
  unavailable.push({namespace:row.namespace,key:row.key});
  for(const name of row.projects)projects.add(name);
 }
 if(!unavailable.length)return;
 await askFleetForClosureOwnersV1(host,current.manifest.owners,[...projects],current.pointer.generation,implementation);
 await prepare(current.pointer,unavailable);
 await engine.invalidate({reason:"closure-missing",expected:current.pointer,signal:host.signal,claim});
}

/** 🛰️ Asks the fleet-first decision of a read about the owners that name a leaf nobody could give: the normal path of stale owners (`host.fleetRead`) serves the heavy ones from the fleet and leaves the light ones to this machine. */
async function askFleetForClosureOwnersV1(host,owners,projects,generation,implementation){
 const named=projects.filter(name=>Object.hasOwn(owners,name));
 if(host.fleetRead&&named.length)await host.fleetRead({generation,dirty:named,roots:Object.fromEntries(named.map(name=>[name,owners[name].root])),structural:true,implementation});
}

/** ⬆️ Offers the newest sealed generation to the shared remote cache of the group (only a machine whose access may write); `ifAbsent` offers it only when the cache holds none. */
async function publishRemoteGenerationV1(host,ifAbsent){
 if(!host.exchange?.upload)return;
 await host.withShared(async(store,control)=>{
  const options=host.storeOptions(store,control),started=Date.now(),outcome=await host.exchange.publish({root:options.root,fs:options.fs??createNodeFilePortV1(),signal:host.signal,ifAbsent,stale:0});
  if(outcome.status==="uploaded"){
   if(outcome.head)host.say("uploadedHead",outcome.head);
   host.say("uploaded",{generation:outcome.generation,owners:outcome.owners,documents:outcome.documents,megabytes:Math.round(outcome.bytes/1048576),seconds:Math.round((Date.now()-started)/1000)});
  }
  else if(outcome.status==="failed"||outcome.status==="incomplete"||outcome.status==="oversized")host.say("exchangeFailed",{reason:outcome.reason??outcome.status});
 });
}

/** 🔍️ What the fleet cache holds of the whole workspace for this machine (one short request, nothing downloaded); `undefined` without an exchange or on any fault. */
async function fleetLookV1(host,implementation,requireComplete){
 if(!host.exchange?.look)return undefined;
 return host.withShared((store,control)=>{const options=host.storeOptions(store,control);return host.exchange.look({root:options.root,fs:options.fs??createNodeFilePortV1(),signal:host.signal,implementation,...requireComplete?{requireComplete}:{}});}).catch(()=>undefined);
}

/** 🚪️ The whole-read gate of the fleet for a builder that is about to read: the host's gate lets one machine read and the others wait for its complete generation (then the builder restores it first); without a host gate, for a closure read, a pinned read or the policy current it lets the builder through. The returned `release` ends the claim after the generation was offered. */
async function wholeReadGateV1(host,{identity,implementation,policy,roots,pin,managed}){
 const through={outcome:"skipped",waitedMilliseconds:0,release:async()=>undefined};
 if(!host.wholeGate||roots!==undefined||pin!==undefined||policy==="current")return through;
 if(managed!==undefined)return host.wholeGate({identity,signal:host.signal,...managed});
 try{return await host.wholeGate({identity,signal:host.signal,look:()=>fleetLookV1(host,implementation)});}
 catch(error){if(host.signal?.aborted)throw error;return through;}
}

/** 📥️ Whether a question under the policy current may fetch the complete generation the fleet holds: the head is complete and a restore would take it, and no builder custody is held by a read that is going on here. */
async function currentMayFetchV1(host,implementation){
 const look=await fleetLookV1(host,implementation,true);
 if(look?.state!=="complete"||!look.ahead)return false;
 return await Promise.resolve(host.builderIdle?.({generation:-1,dirty:[],roots:{},structural:true,implementation})??true).catch(()=>false);
}

/** 📥️ Restores one observed complete whole head through the normal graph writer, without planning or assembling a graph. */
export async function restoreExpectedWholeGenerationV1(host,expected,adoption){
 host.signal?.throwIfAborted();
 if(!host.exchange)return{status:"absent"};
 await host.preflight?.();
 const identity=await host.identity(),implementation=implementationDigestV1(await host.implementationRows());
 if(expected.scope!=="all"||expected.identity!==identity||expected.implementation!==implementation)return{status:"foreign"};
 host.scope?.(true);
 const sealed=async options=>{
  const current=await createGenerationStoreV1(options).read(host.signal);
  return current?.manifest?.identity===identity&&current.manifest.implementation===implementation?current:undefined;
 };
 if(await host.withShared((store,control)=>sealed(host.storeOptions(store,control))))return{status:"local"};
 const request={generation:-1,dirty:[],roots:{},structural:true,implementation};
 if(!await host.builderIdle?.(request))return{status:"busy"};
 return host.withBuilder(async(store,control,claim)=>{
  host.signal?.throwIfAborted();
  const options=host.storeOptions(store,control);
  if(await sealed(options))return{status:"local"};
  if(claim?.structural!==true||typeof claim.fence!=="function")return{status:"busy"};
  let planned;
  const outcome=await restoreRemoteGenerationV1(host,options,implementation,claim,true,expected,async local=>{
   planned=local;
   await adoption?.beforePublication(local);
  });
  if(outcome.status!=="restored")return outcome;
  const current=await claim.fence(()=>sealed(options),{signal:host.signal}),head=outcome.head;
  host.signal?.throwIfAborted();
  if(!current||!planned||current.pointer.manifest===null||current.pointer.manifest!==planned.localManifest||current.pointer.generation!==planned.generation||current.pointer.generation!==outcome.generation||!head||head.origin!==expected.origin||head.generation!==expected.generation||head.scope!==expected.scope||head.manifest!==expected.manifest)
   throw Object.assign(Error("Restored whole generation differs from the expected head"),{code:"GENERATION_CORRUPT"});
  host.say("restoredHead",head);
  return{...outcome,localManifest:current.pointer.manifest};
 },request,undefined,"restore");
}

/** 🩹️ How many times one load rebuilds after a build input closure leaf turned out missing: the first rebuild recomputes from a cold plan, each later one first removes the records that still name the leaf (`host.repairClosure`), then the load fails with the original error. */
const CLOSURE_REPAIR_ROUNDS_V1=3;
/** 🔎️ The key of the build input closure leaf a failure says is missing: the `missingClosure` tag, else the `closure-missing:<key>` text or the leaf path of an ENOENT message, on the error or one of its causes (a failure that crossed a boundary keeps its text, not its properties). */
export function missingClosureOfV1(error,depth=4){
 if(error===null||typeof error!=="object"||depth<0)return undefined;
 if(typeof error.missingClosure==="string")return error.missingClosure;
 const match=/closure-missing:([a-f0-9]{64})|build-(?:input|lock)-closures[\\/]([a-f0-9]{64})\.json/u.exec(String(error.message??""));
 if(match&&(error.code==="ENOENT"||/ENOENT|closure-missing/u.test(String(error.message))))return match[1]??match[2];
 return missingClosureOfV1(error.cause,depth-1);
}
/** ⏳️ Words an age in seconds as days, hours and minutes (`3 d 4 h 12 min`, `45 s` below one minute); the units read the same in English and German. */
export function generationAgeTextV1(seconds){
 const total=Math.max(0,Math.floor(seconds)),days=Math.floor(total/86400),hours=Math.floor(total%86400/3600),minutes=Math.floor(total%3600/60);
 if(total<60)return total+" s";
 return[days?days+" d":"",days||hours?hours+" h":"",minutes+" min"].filter(Boolean).join(" ");
}

/** 🕰️ The age in seconds of the sealed generation whose manifest is `digest`: the modification time of its manifest file, the moment a builder published it; undefined when the file cannot be listed. */
export async function generationAgeSecondsV1(fs,root,digest,nowMs){
 const rows=await fs.list(join(root,GENERATION_POLICY_V1.manifests)).catch(()=>undefined),row=rows?.find(file=>file.name===digest+".json");
 return row?.modifiedMs===undefined?undefined:Math.max(0,Math.round((nowMs-row.modifiedMs)/1000));
}

/** 🔭️ Words, in `language`, how far the whole read of this machine is: `builder` is the holder of the graph builder custody (`{phase,done,total,heldSeconds}`) or null when none runs. */
export function renderWholeReadProgressV1(builder,language){
 if(!builder)return renderGenerationMessageV1("progressIdle",{},language);
 return renderGenerationMessageV1("progressRunning",{phase:builder.phase??"-",done:builder.done??0,total:builder.total??"?",held:generationAgeTextV1(builder.heldSeconds??0)},language);
}

/** ⏸️ Polls the held claim’s request during cold discovery and waits for owned work to drain before releasing custody. */
async function discoverWithCustodyV1(claim,signal,discover){
 const controller=new AbortController(),active=signal?AbortSignal.any([signal,controller.signal]):controller.signal;
 let timer,probe,finished=false;
 const schedule=()=>{
  if(finished||active.aborted)return;
  timer=setTimeout(()=>{
   if(active.aborted)return;
   probe=Promise.resolve().then(()=>claim.handover({requestedOnly:true})).catch(()=>undefined).then(stop=>{
    if(!finished&&!active.aborted&&stop)controller.abort(claim.handoverError(stop));
   }).then(schedule);
  },CUSTODY_POLICY_V1.pollMilliseconds);
  timer.unref?.();
 };
 schedule();
 let part;
 try{
  try{active.throwIfAborted();part=await discover(active);}
  finally{finished=true;clearTimeout(timer);await probe;}
  active.throwIfAborted();
  return part;
 }catch(error){
  if(part)await Promise.resolve().then(()=>part.dispose?.()).catch(()=>undefined);
  signal?.throwIfAborted();
  active.throwIfAborted();
  throw error;
 }
}

/** 🕰️ Loads the all-workspace graph (a command whose front door offered a yield ticket and that only waits for a peer's build ends instead of waiting: see `../🛌️yield`), the closure of some projects, or the slice of a selected scope (tasks or literal owners) from the newest sealed generation; `lean` answers only project and target names. A shared reader verifies and serves it without ever taking a writer; only a stale or missing generation enters the builder custody, where a holder that finds the work done by a peer serves that generation. Complete derived answers stay within this load and store root; each fresh attempt rechecks their support and compares them to its current rows.
 * `host` binds the repository: `release()` (optional: drops the in-process caches a construction filled, called once the graph is assembled so a process that goes on to run for hours keeps only the graph), `identity()`, `implementationRows()`, `withShared(body)`, `withBuilder(body)`, `storeOptions(store, control)`, `membership(previous, signal)`, `discoverPart(spec, signal)`, `expand(projects, control)`, `expansionLimits`, `readGraph(value)`, `readLean(value)`, `clock(control)`, `say(key, fields)` and the constants `workspaceRoot`, `namedInputs`, `registryDigest`, `signal`.
 */
export async function loadGenerationGraphV1(host,{policy="fresh",closure,sealedOnly=false,selection,slice,lean,pin,externalInputs=[]}={}){
 externalInputs=readGenerationExternalInputsV1(externalInputs);
 await host.preflight?.();
 const requestedAt=graphQueueRequestedAtV1()??Date.now(),identity=await host.identity(),roots=slice?[...new Set(slice.roots)]:closure?[...new Set(closure)]:undefined,rows=pin===undefined?await host.implementationRows():[],implementation=implementationDigestV1(rows),scoped=scopeImplementationRowsV1(rows),limits=host.expansionLimits??{maximumProjects:GENERATION_POLICY_V1.expansionMaximumProjects,maximumBytes:GENERATION_POLICY_V1.expansionMaximumBytes};
 const wholeCurrent=policy==="current"&&roots===undefined&&pin===undefined;
 if(sealedOnly&&(policy!=="current"||closure===undefined||slice!==undefined||pin!==undefined))throw Error("A current unpinned generation closure is required");
 host.scope?.(roots===undefined&&pin===undefined);
 const answersByRoot=new Map(),deniedRepairs=new Set();let pointer,published=false,repairs=0,carried,sealedAgeSeconds,fetchOnly=false;
 const attempt=async(builder,store,control,expectedHead,repair,localHead)=>{
  const options=host.storeOptions(store,control),writer=builder&&!fetchOnly;
  if(localHead){
   const bytes=await (options.fs??createNodeFilePortV1()).read(join(options.root,GENERATION_POLICY_V1.pointer));
   const current=bytes&&readGenerationPointerV1(bytes);
   if(current?.generation!==localHead.generation||current.manifest!==localHead.manifest)return;
  }
  if(!localHead&&!sealedOnly&&!builder&&host.exchange?.ahead&&pin===undefined&&!wholeCurrent&&await host.exchange.ahead({root:options.root,fs:options.fs??createNodeFilePortV1(),signal:host.signal,implementation}).catch(()=>false))return{needsBuild:true,pending:{generation:-1,dirty:[],roots:{},structural:true},generation:-1};
  let answers=answersByRoot.get(options.root);if(!answers)answersByRoot.set(options.root,answers=createAnswersV1({root:options.root,fs:options.fs,durable:options.durableAnswers!==false,workspaceRoot:host.workspaceRoot}));
  const engine=createGenerationStoreV1({...options,answers,workspaceRoot:host.workspaceRoot,say:(key,fields)=>host.say(key,fields)}),progress=custodyProgressV1(builder,createGenerationProgressV1(host.clock(control)));
   const closeStore={root:options.root,fs:options.fs??createNodeFilePortV1(),signal:host.signal,progress:(done,total)=>done%64===0||done===total?progress({phase:"closures",done,total}):undefined},verifyClosure=host.verifyClosure?.bind(host);
   let stagedDelta;
   const prepare=async(expected,missing)=>{
    if(verifyClosure!==undefined)stagedDelta=await prepareClosureRepairV1({...closeStore,expected,identity,implementation,missing,verifyClosure});
   };
  if(writer&&repair){
   const{pointer:expected,key,namespace,projects,round}=repair,current=await engine.read(host.signal);
   if(current?.manifest&&current.pointer.generation===expected.generation&&current.pointer.manifest===expected.manifest){
    if(typeof builder==="object"&&builder.structural!==true)throw Error("Structural custody is required to repair a generation closure");
    const settle=host.settleClosure?await host.settleClosure({key,namespace,projects}):(round>0?(await host.repairClosure?.(key),"re-derived"):"re-derived");
    host.signal?.throwIfAborted();
    if(settle!=="refetched"){
     await askFleetForClosureOwnersV1(host,current.manifest.owners,projects,expected.generation,implementation);
     host.signal?.throwIfAborted();
      await prepare(expected,[{namespace,key}]);
     await engine.invalidate({reason:"closure-missing",expected,signal:host.signal,claim:builder});
    }
   }
  }
  const restored=builder&&host.exchange?await restoreRemoteGenerationV1(host,options,implementation,builder,fetchOnly,expectedHead):undefined;
  if(fetchOnly){const missing=restored?.closures?.missing[0];if(missing)throw closureMissingErrorV1(missing);}
   else await settleRestoredClosuresV1(host,engine,restored,implementation,builder,prepare);
  if(wholeCurrent||sealedOnly){
   const fs=options.fs??createNodeFilePortV1(),sealed=await engine.read(host.signal);
   if(sealed?.manifest?.identity!==identity){
    const ahead=wholeCurrent&&!builder&&host.exchange?.ahead?await host.exchange.ahead({root:options.root,fs,signal:host.signal,implementation}).catch(()=>false):false;
    return{needsBuild:true,fleetHolds:ahead===true,pending:{generation:-1,dirty:[],roots:{},structural:true},generation:-1};
   }
   if(wholeCurrent)sealedAgeSeconds=await generationAgeSecondsV1(fs,options.root,sealed.pointer.manifest,Date.now());
  }
  const ownerProgress=typeof builder==="object"&&typeof builder.progress==="function"?async(done,total,phase="inference")=>{try{await builder.progress({phase,done,total});}catch{}}:undefined;
  let reached=0;const owners=(done,total,more)=>progress({phase:"owners",done:reached=Math.max(reached,done),total,...more});
  const compute=writer?async(spec,signal)=>{announceGenerationBuildV1(host,spec,rows);const wanted=spec.owners.length+spec.discover.length,before=spec.done??0,overall=(spec.total??spec.owners.length)+spec.discover.length;let inferred=0;if(wanted)await owners(before,overall);const cutAt=Date.now();await host.releaseCutoff?.();const cutMs=Date.now()-cutAt;const discover=active=>host.discoverPart(spec,active,control,()=>{if(wanted&&inferred<wanted-1)owners(before+ ++inferred,overall).catch(()=>undefined);},(running,decision)=>owners(before+inferred,overall,{running,decision}).catch(()=>undefined),ownerProgress);const part=spec.cold&&builder?.handover?await discoverWithCustodyV1(builder,signal,discover):await discover(signal);if(wanted)await owners(before+wanted,overall);announceWithheldProjectsV1(host,part);const releaseAt=Date.now();await host.release?.({more:spec.more===true});const shaped=shapeComputeResultV1({workspaceRoot:host.workspaceRoot,separator:host.separator,part,stubs:spec.stubs,implementation:scoped});return{...shaped,phases:{...shaped.phases,cutoff:cutMs,release:Date.now()-releaseAt}};}:async()=>{throw Error("A shared graph reader never computes");};
  pointer=join(options.root,GENERATION_POLICY_V1.pointer);
   let delta,quarantined=false;
   if(writer&&verifyClosure!==undefined){
    host.signal?.throwIfAborted();
    const bytes=await closeStore.fs.read(pointer),head=bytes?readGenerationPointerV1(bytes):undefined;
    quarantined=head?.manifest===null&&head.reason==="closure-missing";
    if(quarantined&&!deniedRepairs.has(head.generation)){
     const lookup={...closeStore,pointer:head,identity,implementation};
     delta=stagedDelta&&canonicalJsonV1(stagedDelta.pointer)===canonicalJsonV1(head)?stagedDelta:await readClosureRepairV1(lookup)??await recoverClosureRepairV1({...lookup,verifyClosure});
    }
   }
   const qualify=!quarantined?undefined:async(manifest,signal)=>{
    if(!await verifyClosureRepairCandidateV1({...closeStore,manifest,signal,verifyClosure}))throw Object.assign(Error("The repaired generation still has unavailable closures"),{code:"GENERATION_CLOSURE_REPAIR_UNQUALIFIED"});
   };
   const base={identity,implementation,policy:writer?"fresh":policy,published:writer&&host.published?(generation,info)=>host.published(generation,info):undefined,wave:writer&&host.waveDone?info=>host.waveDone(info):undefined,membership:host.membership,compute,prime:writer?host.prime:undefined,signal:host.signal,progress,requestedAt,builder:writer?body=>body(builder===true?undefined:builder):needsBuild,unchanged:writer&&host.planUnchanged?live=>host.planUnchanged(live):undefined,planned:writer&&carried&&host.planUnchanged?{...carried,holds:()=>host.planUnchanged(carried.live)}:undefined,repair:delta&&{generation:delta.generation,parent:delta.parent,owners:delta.owners,receipt:delta.receipt},qualify};
  const closeFn=slice?async(current)=>{const global=await engine.readGlobal(current.manifest);return sliceClosureV1({owners:Object.fromEntries(Object.entries(current.manifest.owners).map(([name,row])=>[name,row.root])),external:Object.keys(global.externalNodes??{}),fragment:name=>engine.readFragment(current.manifest.owners[name]).then(value=>{if(!value)throw Object.assign(Error("Fragment of "+name+" is missing"),{code:"GENERATION_INCOMPLETE"});return sliceViewV1(value);}),selection:slice.selection,roots:slice.roots,providers:slice.providers??[]});}:undefined;
   const started=Date.now();
   let result;
   try{result=pin!==undefined?await readPinnedGenerationV1({engine,pin,identity,roots,closure:closeFn,signal:host.signal}):roots?await engine.refreshClosure({...base,roots,closure:closeFn}):await engine.refresh(base);}
   catch(error){
    if(delta===undefined||error?.code!=="GENERATION_CLOSURE_REPAIR_UNQUALIFIED")throw error;
    host.signal?.throwIfAborted();
    deniedRepairs.add(delta.generation);
    engine.dropIndexes();
    return{needsBuild:true,pending:{generation:delta.generation,dirty:[],roots:{},structural:true},generation:delta.generation};
   }
  engine.dropIndexes();if(typeof builder==="object")await builder.release();
  if(result.served==="built"&&result.published)published=true;
  if(localHead){
   const current=result.current;
   if(result.needsBuild||current?.manifest?.identity!==identity||current.manifest.implementation!==implementation)return;
   if(current.pointer.manifest!==localHead.manifest&&(!result.published||current.manifest.parent!==localHead.manifest))return;
  }
  if(result.needsBuild)return{needsBuild:true,pending:result.pending,planned:result.planned,generation:(await engine.read(host.signal))?.pointer.generation??-1};
  const generation=result.current.manifest.generation;
  if(result.served==="current"){host.say(result.current.manifest.implementation===implementation?"current":"stale",{generation});if(roots===undefined&&sealedAgeSeconds!==undefined)host.say("sealedAge",{generation,owners:Object.keys(result.current.manifest.owners).length,age:generationAgeTextV1(sealedAgeSeconds)});}
  else if(result.served==="covered")host.say("covered",{generation});
  else if(result.served==="pinned")host.say("pinned",{generation,count:result.names?result.names.size:Object.keys(result.current.manifest.owners).length});
  else if(result.served==="built"&&result.published){host.say(result.merged===undefined?"published":"merged",{generation,computed:result.computed,carried:result.carried??0,seconds:Math.round((Date.now()-started)/1000),theirs:result.merged??0});if(result.index)host.say("indexed",{generation,rows:result.index.rows,owners:result.index.owners,expanded:(result.index.expandedBytes/1048576).toFixed(1),percent:(100*result.index.expandedBytes/GENERATION_POLICY_V1.maximumIndexBytes).toFixed(1),limit:GENERATION_POLICY_V1.maximumIndexBytes/1048576,packed:(result.index.packedBytes/1048576).toFixed(1)});}
  else if(host.verbose||lean)host.say(roots?"closure":"verified",roots?{count:result.names.size}:{generation,checked:result.plan?.checked??0});
  for(const path of result.plan?.pending??[])if(!pendingCratesReportedV1.has(path)){pendingCratesReportedV1.set(path,true);host.say("pendingCrate",{path,command:buildRepairCommandV1("build-owners-sync")});}
  if(result.plan?.cutoff?.questions)host.say("cutoff",{questions:result.plan.cutoff.questions,rows:result.plan.stale??0,held:result.plan.cutoff.held,changed:result.plan.cutoff.changed});
  if(lean){const listing=await engine.catalog(result.current,{signal:host.signal});return{graph:host.readLean({version:1,lean:true,projects:Object.fromEntries(listing.projects.map(row=>[row.name,{name:row.name,root:row.root,targets:Object.fromEntries(row.targets.map(target=>[target,{}]))}])),dependencies:[],...(listing.unavailable.length?{unavailable:listing.unavailable}:{})}),result};}
  const stream=engine.assembleStream(result.current,{names:result.names&&[...result.names],externalInputs,compare:(a,b)=>a.localeCompare(b),signal:host.signal,progress});
  let projects;
  try{projects=await expandStreamV1(stream.nodes(),chunk=>host.expand(chunk,control),limits);}
  catch(error){
   const missing=missingClosureOfV1(error);if(missing===undefined)throw error;
   if(localHead)return;
   if(sealedOnly||wholeCurrent||fetchOnly||repairs>=CLOSURE_REPAIR_ROUNDS_V1)throw error.code==="closure-missing"?error:closureMissingErrorV1({namespace:error.missingNamespace??"build-input-closures",key:missing,projects:error.missingProjects??[],cause:error});
   const owners=result.current.manifest.owners,projects=(Array.isArray(error.missingProjects)?error.missingProjects:[]).filter(name=>Object.hasOwn(owners,name)),generation=result.current.manifest.generation;
    const repair=builder||verifyClosure!==undefined?{pointer:result.current.pointer,key:missing,namespace:error.missingNamespace??"build-input-closures",projects,round:repairs++}:undefined;
   return{needsBuild:true,pending:{generation,dirty:builder?[]:projects,roots:builder?{}:Object.fromEntries(projects.map(name=>[name,owners[name].root])),structural:true},generation,repair};
  }
  const assembled=await stream.finish(),externalRoots=[...new Set(externalInputs.map(name=>resolveGenerationExternalInputV1(assembled.externalNodes,name)))].sort(compareCodePointsV1);
  const closureDigest=selection?createHash("sha256").update(canonicalJsonV1([assembled.membership,[...result.names].sort(),...externalRoots.length?[externalRoots]:[]])).digest("hex"):assembled.membership;
  const graph=host.readGraph({version:1,projects,dependencies:assembled.dependencies,namedInputs:host.namedInputs,externalNodes:assembled.externalNodes,...(assembled.unavailable.length?{unavailable:assembled.unavailable}:{}),admission:{selection:selection??{kind:"all"},registry:host.registryDigest,closure:closureDigest,...externalRoots.length?{externalInputs:externalRoots}:{}}});
  if(!sealedOnly&&host.sealed){const manifest=result.current.manifest,owners=Object.keys(manifest.owners).sort(compareCodePointsV1);host.sealed({generation:manifest.generation,manifest:result.current.pointer.manifest,owners,covered:pin!==undefined?decodeVerifiedScopeV1(owners,pin.covered):result.names,atMs:pin!==undefined?pin.covered.atMs:requestedAt});}
  return{graph,result,stats:generationLoadStatsV1(result,Date.now()-started)};
 };
 const readerAt=Date.now(),readShared=local=>host.withShared((store,control)=>attempt(false,store,control,undefined,undefined,local));
 let first=await readShared();
 carriedStagesV1["start.reader"]=(carriedStagesV1["start.reader"]??0)+Date.now()-readerAt;
 if(first.repair&&host.refetchClosure){
  const tried=new Set();
  for(let round=0;first.repair&&round<CLOSURE_REPAIR_ROUNDS_V1;round++){
   const {namespace,key}=first.repair,leaf=namespace+"/"+key;
   if(tried.has(leaf))break;
   tried.add(leaf);
   const fetched=await host.refetchClosure({namespace,key},host.signal).catch(()=>false);
   host.signal?.throwIfAborted();
   if(!fetched)break;
   repairs--;
   first=await readShared();
  }
 }
 if(!first.needsBuild){await host.release?.();if(!sealedOnly)await publishRemoteGenerationV1(host,true);return{graph:first.graph,stats:first.stats};}
 if(wholeCurrent||sealedOnly){
  if(first.fleetHolds&&await currentMayFetchV1(host,implementation)){
   fetchOnly=true;
   try{const fetched=await host.withBuilder((store,control,claim)=>attempt(claim??true,store,control),{dirty:[],structural:true},undefined,"restore");if(!fetched.needsBuild){await host.release?.();return{graph:fetched.graph,stats:fetched.stats};}}finally{fetchOnly=false;}
  }
  if(sealedOnly)throw Object.assign(Error(renderGenerationMessageV1("projectNeverSealed",{project:(closure??[]).join(", ")},host.language)),{code:"GENERATION_NONE_SEALED"});
  const progress=await Promise.resolve(host.wholeReadProgress?.()).catch(()=>undefined);throw Object.assign(Error(renderGenerationMessageV1("neverSealed",{fleet:first.fleetHolds?renderGenerationMessageV1("fleetHolds",{},host.language):"",progress:progress??""},host.language)),{code:"GENERATION_NONE_SEALED"});}
 carried=first.planned;
 const request=first.pending?.dirty.length?{generation:first.pending.generation,dirty:first.pending.dirty,roots:first.pending.roots,structural:first.pending.structural,implementation}:undefined;
 const now=host.now??Date.now,headRetries=new Map();
 let nextLook=0,nextLocalLook=-Infinity,expectedHead,expectedLocal,localCandidate;
 const localWhileWaiting=policy==="fresh"&&pin===undefined,localPollMilliseconds=Math.max(1,host.waitNoticeMilliseconds??GENERATION_POLICY_V1.waitNoticeMilliseconds);
 const deferLocal=candidate=>{
  candidate.delay=Math.min(GENERATION_POLICY_V1.headRetryMilliseconds,candidate.delay*2);
  candidate.retryAt=now()+candidate.delay;
 };
 const localSeal=async()=>{
  if(now()<nextLocalLook)return;
  nextLocalLook=now()+localPollMilliseconds;
  try{
   return await host.withShared(async(store,control)=>{
    const options=host.storeOptions(store,control),fs=options.fs??createNodeFilePortV1();
    const bytes=await fs.read(join(options.root,GENERATION_POLICY_V1.pointer)),head=bytes&&readGenerationPointerV1(bytes);
    if(!head||head.generation<=first.generation||head.generation<(localCandidate?.generation??-1))return;
    if(!localCandidate||head.generation>localCandidate.generation)
     localCandidate={generation:head.generation,manifest:head.manifest,retryAt:-Infinity,delay:localPollMilliseconds};
    const candidate=localCandidate;
    if(head.manifest===null||head.manifest!==candidate.manifest||now()<candidate.retryAt)return;
    const manifest=await createGenerationStoreV1(options).readManifestAt(head.manifest,host.signal).catch(()=>undefined);
    host.signal?.throwIfAborted();
    if(!manifest){deferLocal(candidate);return;}
    candidate.retryAt=Infinity;
    if(manifest.generation===head.generation&&manifest.identity===identity&&manifest.implementation===implementation)return candidate;
   });
  }catch{host.signal?.throwIfAborted();}
 };
 const canRestoreHead=look=>look?.state==="complete"&&look.ahead&&look.candidate!==undefined&&now()>=(headRetries.get(canonicalJsonV1(look.candidate))??-Infinity);
 const publishedWhileWaiting=!localWhileWaiting&&host.exchange?.look===undefined?undefined:{
  available:async()=>{
   if(localWhileWaiting){
    const local=await localSeal();
    host.signal?.throwIfAborted();
    if(local){expectedLocal=local;return true;}
   }
   if(host.exchange?.look===undefined||now()<nextLook)return false;
   nextLook=now()+(host.waitNoticeMilliseconds??GENERATION_POLICY_V1.waitNoticeMilliseconds);
   const look=await fleetLookV1(host,implementation,true);if(!canRestoreHead(look))return false;
   expectedHead=look.candidate;return true;
  },
  restore:async()=>{
   if(expectedLocal){
    const local=expectedLocal;expectedLocal=undefined;
    try{const reused=await readShared(local);if(reused)return reused;}
    catch(error){
     host.signal?.throwIfAborted();
     if(!["GENERATION_CORRUPT","GENERATION_INCOMPLETE","ENOENT"].includes(error?.code))throw error;
    }
    deferLocal(local);
    return;
   }
   const selected=expectedHead,key=canonicalJsonV1(selected);
   headRetries.delete(key);
   if(headRetries.size>=GENERATION_POLICY_V1.headRetryLimit)headRetries.delete(headRetries.keys().next().value);
   headRetries.set(key,Infinity);
   fetchOnly=true;
   try{const fetched=await host.withBuilder((store,control,claim)=>attempt(claim??true,store,control,selected),{dirty:[],structural:true},undefined,"restore");return fetched.needsBuild?undefined:fetched;}
   catch(error){if(host.signal?.aborted)throw error;}
   finally{fetchOnly=false;headRetries.set(key,now()+GENERATION_POLICY_V1.headRetryMilliseconds);}
  }
 };
 if(host.fleetRead&&request)await host.fleetRead(request);
 if(host.attach&&request){const attached=await host.attach(request,async()=>{const again=await host.withShared((store,control)=>attempt(false,store,control));return again.needsBuild?undefined:again;});if(attached){await host.release?.();await publishRemoteGenerationV1(host,true);return{graph:attached.graph,stats:attached.stats};}}
 if(host.prefetch&&request&&!await host.builderIdle?.(request)){host.carryClosures?.(true);const prefetchAt=Date.now();try{const fetched=await host.prefetch(request,undefined,publishedWhileWaiting);if(fetched?.graph!==undefined){await host.release?.();return{graph:fetched.graph,stats:fetched.stats};}}finally{host.carryClosures?.(false);carriedStagesV1["start.prefetch"]=(carriedStagesV1["start.prefetch"]??0)+Date.now()-prefetchAt;}}
 let restored;
 const managed=host.prepareWholeRebuild&&host.wholeGate&&host.exchange?.look&&publishedWhileWaiting&&roots===undefined&&pin===undefined&&policy!=="current"?{
  look:async()=>{
   const seen=await fleetLookV1(host,implementation);
   if(seen?.state!=="complete"||!seen.ahead)return seen;
   const exact=await fleetLookV1(host,implementation,true);
   return canRestoreHead(exact)?exact:{...seen,ahead:false};
  },
  fund:async()=>{
   const funding=await host.prepareWholeRebuild(publishedWhileWaiting);
   if(funding.restored!==undefined){restored=funding.restored;return undefined;}
   return{release:funding.release??(()=>undefined)};
  }
 }:undefined,gateInput={identity,implementation,policy,roots,pin,managed};
 const legacy=managed===undefined?await wholeReadGateV1(host,gateInput):undefined;
 try{
 let queued;
 const peerWait=blocked=>{if(blocked)queued??=Date.now();else queued=undefined;};
 const notice=setInterval(()=>{if(queued===undefined)return;host.say("waiting",{seconds:Math.round((Date.now()-queued)/1000),generation:first.generation});if(pointer!==undefined&&graphQueueYieldsV1({queuedMs:Date.now()-queued,yieldAfterMilliseconds:host.yieldAfterMilliseconds}))yieldGraphQueueV1({generation:first.generation,pointer,requestedAt,queuedMs:Date.now()-queued,exit:host.exitProcess});},host.waitNoticeMilliseconds??GENERATION_POLICY_V1.waitNoticeMilliseconds);notice.unref?.();
  let want=first.pending,repair=first.repair;
 try{
  for(let round=1;;round++){
   const gate=legacy??await wholeReadGateV1(host,gateInput);
   try{
    let built=restored;
    if(managed!==undefined&&gate.candidate!==undefined){
     expectedHead=gate.candidate;
     built=await publishedWhileWaiting.restore();
     if(built===undefined){round--;continue;}
    }
    built??=await host.withBuilder((store,control,claim)=>{peerWait(false);return attempt(claim??true,store,control,undefined,repair);},want,peerWait,"build",publishedWhileWaiting,gate.funded);
    carried=undefined;
    if(!built.needsBuild){
     await host.release?.();
     if(published)await publishRemoteGenerationV1(host,false);
     return{graph:built.graph,stats:built.stats};
    }
    want={...built.pending,structural:built.pending.structural||round>=CUSTODY_POLICY_V1.widenRounds};
    repair=built.repair;
   }finally{if(legacy===undefined)await gate.release();}
  }
 }finally{clearInterval(notice);}
 }finally{await legacy?.release();}
}

/** 📊️ What one load verified and reinferred: the stale facts, the closures asked, the owners recomputed and the owners an unchanged answer cut off, and the milliseconds the verification and refresh took. */
export function generationLoadStatsV1(result,verifiedMilliseconds){
 return Object.freeze({staleFacts:result.plan?.stale??0,closuresVerified:result.plan?.cutoff?.questions??0,ownersReinferred:result.computed??0,ownersCutOff:result.plan?.cutoff?.held??0,doubtfulFacts:result.plan?.doubtful??0,verifiedMilliseconds});
}
