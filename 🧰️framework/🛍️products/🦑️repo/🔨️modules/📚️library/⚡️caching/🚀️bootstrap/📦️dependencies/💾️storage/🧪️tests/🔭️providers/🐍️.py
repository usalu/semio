"""🔭️ Hoisted source resolution, Python side: the fixture row `hoisted` of `🔣️.json` that `🟦️.ts` runs through the storage provider walk. Here the layout is rebuilt on disk and resolved by two independent oracles: Node's own `require.resolve` (third party) and a scope-root-owner model that never climbs past the member slot. The stray unowned `node_modules` with a dangling link is removed before Node resolves, as in `🟦️.ts`; the model never looks at it. Run with `python3 <this file>`."""

import json
import os
import shutil
import subprocess
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
with open(os.path.join(HERE, "🔣️.json"), encoding="utf-8") as handle:
    ROW = json.load(handle)["hoisted"]


def write(path, document):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(document, handle)


def layout(root):
    workspace, cache = os.path.join(root, "s"), os.path.join(root, "n")
    slot = os.path.join(cache, "j", "a" * 20, "w", "0")
    owner = os.path.join(cache, "j", "a" * 20, "node_modules")
    member = os.path.join(workspace, ROW["member"])
    stray = os.path.join(workspace, "mid", "node_modules")
    document = {"name": "hoisted-p", "version": "1.0.0", "dependencies": {"dep": "^1.0.0"}}
    write(os.path.join(member, "package.json"), document)
    write(os.path.join(workspace, "package.json"), {"name": "scope", "version": "1.0.0"})
    write(os.path.join(slot, "package.json"), document)
    write(os.path.join(owner, "dep", "package.json"), {"name": "dep", "version": ROW["providerVersion"]})
    os.makedirs(os.path.join(slot, "node_modules"))
    os.makedirs(os.path.join(stray, ROW["strayEntries"][0]))
    os.symlink(os.path.join(root, "retired", "dep"), os.path.join(stray, "dep"))
    os.symlink(os.path.join(slot, "node_modules"), os.path.join(member, "node_modules"))
    os.symlink(owner, os.path.join(workspace, "node_modules"))
    return workspace, member, stray, owner


def scope_root_version(workspace, owner):
    link = os.path.join(workspace, "node_modules")
    target = os.path.realpath(link)
    if target != os.path.realpath(owner):
        return None
    with open(os.path.join(target, "dep", "package.json"), encoding="utf-8") as handle:
        return json.load(handle)["version"]


def node_version(member):
    script = 'const {createRequire}=require("node:module");process.stdout.write(require(createRequire(process.argv[1]+"/package.json").resolve("dep/package.json")).version)'
    done = subprocess.run(["node", "-e", script, member], capture_output=True, text=True)
    return done.stdout if done.returncode == 0 else None


class HoistedResolution(unittest.TestCase):
    def test_the_scope_root_owner_provides_the_expected_edge(self):
        with tempfile.TemporaryDirectory() as root:
            workspace, _, _, owner = layout(root)
            self.assertEqual(scope_root_version(workspace, owner), ROW["expectedEdge"]["version"])

    @unittest.skipUnless(shutil.which("node"), "node is absent")
    def test_node_agrees_once_the_stray_ancestor_is_gone(self):
        with tempfile.TemporaryDirectory() as root:
            workspace, member, stray, owner = layout(root)
            shutil.rmtree(stray)
            self.assertEqual(node_version(member), ROW["expectedEdge"]["version"])
            self.assertEqual(scope_root_version(workspace, owner), node_version(member))


if __name__ == "__main__":
    unittest.main()
