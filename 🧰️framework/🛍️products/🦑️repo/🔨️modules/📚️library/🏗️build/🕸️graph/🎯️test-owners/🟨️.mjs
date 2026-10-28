/** 🎯️ Selects which test owners a partial inference discovers again and discovers them one by one, each inside its own observation capture, so that a project of a test owner carries every fact its own discovery read and is stale exactly when one of them changed. */

/** 🧪️ Selects the feature files whose test owners a partial inference discovers again, grouped by the `🧪️tests` directory that owns them (`<owner>/🧪️tests/<case>/<feature>`): all of them when `all` (cold build, moved environment), otherwise an owner already published (`known` holds the roots of the test owners of the generation) when it is stale itself (its root is a stale root) or a stale root lies below it (a case directory that gained a feature), and an owner not yet published when a stale root is, lies below or contains its directory (a first case below a project that changed). */
export function selectTestFeaturesV1(features,{all,roots,known}){
 const parent=path=>path.slice(0,Math.max(0,path.lastIndexOf("/"))),stale=[...roots],groups=new Map(),decided=new Map();
 for(const feature of features){
  const owner=parent(parent(feature));let chosen=decided.get(owner);
  if(chosen===undefined){chosen=all||(known.has(owner)?roots.has(owner)||stale.some(root=>root.startsWith(owner+"/")):stale.some(root=>root==="."||owner===root||owner.startsWith(root+"/")||root.startsWith(owner+"/")));decided.set(owner,chosen);}
  if(!chosen)continue;
  const list=groups.get(owner);if(list)list.push(feature);else groups.set(owner,[feature]);
 }
 return groups;
}

/** 🔢️ The code point order of test owner directories: the order of their bytes, the order `🧪️test` discovers them in. */
export const compareTestOwnersV1=(a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b));

/** 📏️ How many test owners are discovered one by one; a larger selection (a cold build, a moved environment) is discovered in one call, which shares the walks of the native dependencies between its owners, and every project of it then carries the facts of that whole call. */
export const SEPARATE_TEST_OWNERS_V1=64;

/**
 * 🔭️ Discovers the test owners of `groups` (see {@link selectTestFeaturesV1}) in discovery order. What all owners share is computed once inside its own capture (`tests.discoverTestSharedV1`). Up to `separate` owners are discovered one by one, each inside another capture with that shared state, and the facts of each project it contributes are the shared facts followed by the facts of its own capture; more owners are discovered in one call and every project carries the facts of the shared state followed by the facts of that call. `capture(compute)` answers `{value, observations}`.
 * The shared state may carry a `walk` (see `testShared` of `🧪️test`): the rows that support the command walk of the testing domain and the derived fact that stands for it. Every project then holds the facts of the shared capture, those support rows, the derived row and the facts of its own capture, and the derived facts are answered once, whatever the number of owners.
 * Answers the contributions in discovery order, the facts by project name and the derived facts.
 */
export async function discoverTestOwnersV1({tests,capture,context,groups,separate=SEPARATE_TEST_OWNERS_V1}){
 const contributions=[],rows=new Map();
 if(!groups.size)return{contributions,rows,derived:[]};
 const shared=await capture(()=>tests.discoverTestSharedV1(context)),ordered=[...groups].sort(([a],[b])=>compareTestOwnersV1(a,b)),walk=shared.value?.walk,derived=walk?.derived??[],held=[...shared.observations,...(walk?.rows??[]),...derived.map(fact=>fact.row)];
 const discover=async features=>{
  const one=await capture(()=>tests.discoverTestBuildProjectsV1(features,{},{...context,testShared:shared.value})),facts=[...held,...one.observations];
  for(const entry of one.value){contributions.push(entry);for(const name of Object.keys(entry[1].projects??{}))rows.set(name,facts);}
 };
 if(ordered.length>separate)await discover(ordered.flatMap(([,features])=>features));
 else for(const[,features]of ordered)await discover(features);
 return{contributions,rows,derived};
}
