const {join,relative} = require("node:path");
const collections = require("./🔣️policy.json").collections;

/** 🧫️ Locates testing ancestry while preserving genuine immediate module members. */
function workspaceTestingCollectionRoot(root,directory) {
  const parts=relative(root,directory).replaceAll("\\","/").split("/");
  const index=parts.findIndex((part,index)=>collections.names.includes(part) && parts[index-1]!==collections.moduleMember);
  return index<0?undefined:join(root,...parts.slice(0,index+1));
}
module.exports={workspaceTestingCollectionRoot};
