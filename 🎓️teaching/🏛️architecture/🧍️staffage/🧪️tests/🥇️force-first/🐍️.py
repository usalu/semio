"""🥇️ Oracle of the force-first support solution (🧪️tests/🥇️force-first): numpy singular value decompositions.

Input: path of a JSON document `{cases: [{k: 6x6, f: [6]}]}`. Output: JSON `[[6]]`, per case the wrench of the range of `k` whose force part is closest to the force part of `f` and, among those, whose moment part is closest to the moment part of `f`.
"""
import json
import sys

import numpy


def given(k, f):
    values, vectors = numpy.linalg.eigh(k)
    basis = vectors[:, values > 1e-12 * values.max()]
    a, b = basis[:3], basis[3:]
    start = numpy.linalg.pinv(a, rcond=1e-6) @ f[:3]
    _, singular, rows = numpy.linalg.svd(a)
    rank = int((singular > 1e-6 * singular.max()).sum())
    free = rows[rank:].T
    weights = start + (free @ (numpy.linalg.pinv(b @ free, rcond=1e-6) @ (f[3:] - b @ start)) if free.shape[1] else 0)
    return basis @ weights


def main():
    with open(sys.argv[1], encoding="utf-8") as handle:
        document = json.load(handle)
    json.dump([given(numpy.array(case["k"]), numpy.array(case["f"])).tolist() for case in document["cases"]], sys.stdout, allow_nan=False)


if __name__ == "__main__":
    main()
