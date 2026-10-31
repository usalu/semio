"""🧭️ The mutations derive's roster laws, Python side: the same fixture as `🟦️.ts`. Oracles: dict/set over full identities and `os.walk`. Run with `python3 <this file>`."""

import json
import os
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
REPOSITORY = os.path.abspath(os.path.join(HERE, *[".."] * 8))
with open(os.path.join(HERE, "..", "🧫️fixtures", "🧭️mutation-roster", "🔣️.json"), encoding="utf-8") as handle:
    FIXTURE = json.load(handle)


def owners(row):
    tail = "t" * row["sharedTail"]
    names = [f"{row['prefix']}{index}/{tail}" for index in range(row["count"])]
    if row["duplicate"]:
        names[row["duplicate"]["index"]] = names[row["duplicate"]["of"]]
    return names


def tail_key(text):
    return text.encode("utf-8")[-FIXTURE["tailBytes"]:]


def first_duplicate(names):
    seen = {}
    for index, name in enumerate(names):
        if name in seen:
            return {"first": seen[name], "index": index}
        seen[name] = index
    return None


class RosterLaws(unittest.TestCase):
    def test_owner_identities_hash_a_bounded_tail_yet_compare_in_full(self):
        for row in FIXTURE["uniqueness"]:
            names = owners(row)
            self.assertEqual(len({tail_key(name) for name in names}), row["expectedBuckets"], row["name"])
            self.assertEqual(first_duplicate(names), row["expected"], row["name"])

    def test_the_energy_aggregate_publishes_one_unique_14_field_record_per_leaf(self):
        law = FIXTURE["records"]
        base = os.path.join(REPOSITORY, law["root"])
        rows = []
        for directory, _, files in os.walk(base):
            if "🔣️.json" in files:
                with open(os.path.join(directory, "🔣️.json"), encoding="utf-8") as handle:
                    record = json.load(handle)
                if "semanticKind" in record:
                    rows.append((directory, record))
        self.assertEqual(len(rows), law["leaves"])
        for directory, record in rows:
            keys = sorted(key for key in record if key != law["optionalExtra"])
            self.assertEqual(len(keys), law["fields"], directory)
            self.assertEqual(keys, sorted(law["requiredFields"]), directory)
            self.assertEqual(os.path.realpath(os.path.join(REPOSITORY, record["owner"])), os.path.realpath(directory), directory)
        for field in law["uniqueFields"]:
            self.assertEqual(len({record[field] for _, record in rows}), law["leaves"], field)


if __name__ == "__main__":
    unittest.main()
