const slashed=path=>String(path).replaceAll("\\","/");

/** 🏷️ The owners whose config file a stale fact row names: the content of a config member dirties its owner whichever owner or the environment holds the row (the root project claims no facts by path, so the shared setup's read of its config belongs to the environment). `rows[id].path` is absolute below `workspaceRoot` or workspace-relative; `configs` maps a workspace-relative config path to its owner. */
export function staleConfigOwnersV1({rows,stale,configs,workspaceRoot}){
 const base=workspaceRoot===undefined?undefined:slashed(workspaceRoot).replace(/\/+$/u,"")+"/",owners=new Set();
 for(const id of stale){
  let text=slashed(rows[id].path);
  if(base!==undefined&&text.startsWith(base))text=text.slice(base.length);
  else if(text.startsWith("/")||/^[A-Za-z]:\//u.test(text))continue;
  const owner=configs.get(text);if(owner!==undefined)owners.add(owner);
 }
 return owners;
}
