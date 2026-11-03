/** 🕓️ The paths of a membership change list that are pending crate manifests: files the host marked `pending` (a crate manifest without a build project owner) that lie below no owner root, so they neither make the workspace stale nor enter the graph; a pending file below an owner root is an ordinary file of that owner. `ownerOf(path)` names the owner of a path or is undefined. */
export function pendingCratePathsV1(changes,ownerOf){
 const pending=new Set();
 for(const change of changes)if(change.pending&&!change.directory&&ownerOf(change.path)===undefined)pending.add(change.path);
 return pending;
}
