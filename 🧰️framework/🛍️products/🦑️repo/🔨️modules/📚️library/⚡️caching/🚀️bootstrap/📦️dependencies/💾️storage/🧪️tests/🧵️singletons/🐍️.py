"""🐍️ Singleton packages, Python side: rebuilds the layout of `🔣️.json` on disk, applies the corpus alias plan with its own planner, and resolves every singleton package with its own walk up the `node_modules` chain and with Node, so one physical copy per package is judged by two independent resolvers."""

import json
import os
import shutil
import subprocess
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
with open(os.path.join(HERE, "🔣️.json"), encoding="utf-8") as handle:
    CORPUS = json.load(handle)
LAYOUT = CORPUS["layout"]
POLICY = CORPUS["policy"]["singletonPackages"]


def plan(observation):
    held = {row["name"]: row["version"] for row in observation["owner"]["packages"]}
    if any(name not in POLICY for name in held):
        raise ValueError("undeclared")
    seen, rows = set(), []
    for member in observation["members"]:
        if member["root"] == observation["owner"]["root"]:
            raise ValueError("owner")
        for row in member["packages"]:
            key = (member["root"], row["name"])
            if row["name"] not in POLICY or key in seen or held.get(row["name"]) != row["version"]:
                raise ValueError("refused")
            seen.add(key)
            rows.append({"scope": member["scope"], "root": member["root"], "name": row["name"]})
    return sorted(rows, key=lambda row: (row["root"].encode(), row["name"].encode()))


def generation(scope):
    return LAYOUT["generations"][scope] * 20


def build(root):
    workspace, cache = os.path.join(root, "s"), os.path.join(root, "n")
    stage = lambda scope: os.path.join(cache, "j", generation(scope))
    versions = {**LAYOUT["packages"], **LAYOUT["bystander"]}
    for scope in LAYOUT["scopes"]:
        for name, version in versions.items():
            directory = os.path.join(stage(scope), "node_modules", *name.split("/"))
            os.makedirs(directory)
            with open(os.path.join(directory, "package.json"), "w", encoding="utf-8") as handle:
                json.dump({"name": name, "version": version, "main": "index.js"}, handle)
            with open(os.path.join(directory, "index.js"), "w", encoding="utf-8") as handle:
                handle.write("module.exports=" + json.dumps(scope) + ";")
        folder = os.path.join(workspace, "" if scope == "." else scope)
        os.makedirs(folder, exist_ok=True)
        os.symlink(os.path.join(stage(scope), "node_modules"), os.path.join(folder, "node_modules"), target_is_directory=True)
    nested = LAYOUT["nested"]
    folder = os.path.join(stage(nested["scope"]), "node_modules", nested["parent"], "node_modules", nested["package"])
    os.makedirs(folder)
    with open(os.path.join(folder, "package.json"), "w", encoding="utf-8") as handle:
        json.dump({"name": nested["package"], "version": nested["version"], "main": "index.js"}, handle)
    with open(os.path.join(folder, "index.js"), "w", encoding="utf-8") as handle:
        handle.write('module.exports="nested";')
    os.makedirs(os.path.join(stage("b"), "w", "0", "node_modules"))
    os.makedirs(os.path.join(workspace, "b", "pkg"))
    os.symlink(os.path.join(stage("b"), "w", "0", "node_modules"), os.path.join(workspace, "b", "pkg", "node_modules"), target_is_directory=True)
    return workspace, stage, [workspace, os.path.join(workspace, "b", "pkg"), os.path.join(workspace, "c"), os.path.join(stage(nested["scope"]), "node_modules", nested["parent"])]


def apply(stage):
    observation = {
        "owner": {"scope": ".", "root": stage("."), "packages": [{"name": name, "version": version} for name, version in LAYOUT["packages"].items()]},
        "members": [{"scope": scope, "root": stage(scope), "packages": [{"name": name, "version": version} for name, version in LAYOUT["packages"].items()]} for scope in LAYOUT["scopes"] if scope != "."]
        + [{"scope": LAYOUT["nested"]["scope"], "root": os.path.join(stage(LAYOUT["nested"]["scope"]), "node_modules", LAYOUT["nested"]["parent"]), "packages": [{"name": LAYOUT["nested"]["package"], "version": LAYOUT["nested"]["version"]}]}],
    }
    for alias in plan(observation):
        destination = os.path.join(alias["root"], "node_modules", *alias["name"].split("/"))
        shutil.rmtree(destination)
        os.symlink(os.path.join(stage("."), "node_modules", *alias["name"].split("/")), destination, target_is_directory=True)


def walk(consumer, name):
    directory = consumer
    while True:
        candidate = os.path.join(directory, "node_modules", *name.split("/"))
        if os.path.isdir(candidate):
            return os.path.realpath(candidate)
        parent = os.path.dirname(directory)
        if parent == directory:
            return None
        directory = parent


def node(consumer, name):
    script = 'const {createRequire}=require("node:module"),{realpathSync}=require("node:fs"),{dirname}=require("node:path");process.stdout.write(dirname(realpathSync(createRequire(process.argv[1]+"/x.js").resolve(process.argv[2]+"/package.json"))))'
    done = subprocess.run(["node", "-e", script, consumer, name], capture_output=True, text=True)
    return done.stdout if done.returncode == 0 else None


class Plan(unittest.TestCase):
    def test_every_corpus_case_agrees_with_the_independent_planner(self):
        for row in CORPUS["planCases"]:
            with self.subTest(row["id"]):
                if row["accepted"]:
                    self.assertEqual(plan(row["observation"]), row["aliases"])
                else:
                    with self.assertRaises(ValueError):
                        plan(row["observation"])


class Resolution(unittest.TestCase):
    def test_one_physical_copy_per_singleton_package_and_three_per_bystander(self):
        with tempfile.TemporaryDirectory() as root:
            root = os.path.realpath(root)
            _, stage, consumers = build(root)
            for name in POLICY:
                self.assertEqual(len({walk(consumer, name) for consumer in consumers}), 4 if name == LAYOUT["nested"]["package"] else 3)
            apply(stage)
            for name in POLICY:
                found = {walk(consumer, name) for consumer in consumers}
                self.assertEqual(found, {os.path.join(stage("."), "node_modules", *name.split("/"))})
            for name in LAYOUT["bystander"]:
                self.assertEqual(len({walk(consumer, name) for consumer in consumers}), 3)

    @unittest.skipUnless(shutil.which("node"), "node is absent")
    def test_node_agrees_with_the_python_walk(self):
        with tempfile.TemporaryDirectory() as root:
            root = os.path.realpath(root)
            _, stage, consumers = build(root)
            apply(stage)
            for name in POLICY:
                self.assertEqual({node(consumer, name) for consumer in consumers}, {walk(consumers[0], name)})


if __name__ == "__main__":
    unittest.main()
