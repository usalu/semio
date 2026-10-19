/** 🗺️ Source navigation and bundle metadata
 *
 * Bundle caching for source navigation.
 */

import { bundleCache, type BundleInfo } from "../🎩️globals/🟦️.ts";
import { getTreeRoot, type TreeNodeData } from "../🏬️uri-resolution/🟦️.ts";

// #region 🗺️Source Navigation & Language Diagnostics
// Source navigation maintains bundle caches.

/** 🔺️ updateBundleCache holds the data fields for a updateBundleCache record.
 **/
export async function updateBundleCache() {
  const root = await getTreeRoot();
  if (!root) return;
  const bundles: BundleInfo[] = [];
  function walk(node: TreeNodeData) {
    if (node.Kind === "bundle" && node.Data) {
      bundles.push({ id: node.Data.name || node.Label, root: node.Data.root || "" });
    }
    for (const child of node.Children || []) walk(child);
  }
  walk(root);
  if (bundles.length > 0) bundleCache.current = bundles;
}
// #endregion 🗺️Source Navigation & Language Diagnostics
