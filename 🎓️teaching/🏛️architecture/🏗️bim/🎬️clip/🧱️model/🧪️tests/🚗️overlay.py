"""🚗 Exact binary64 polygon-set area through an owned JSON test interface."""

from collections import Counter
from fractions import Fraction
import json
import math


class _Edge:
    """📐 Precomputed exact line and original floating endpoint bounds."""
    __slots__ = ("x", "y", "dx", "dy", "low", "high", "slope", "intercept", "bounds")

    def __init__(self, a, b):
        self.x, self.y = a
        self.dx, self.dy = b[0] - a[0], b[1] - a[1]
        self.low, self.high = min(a[0], b[0]), max(a[0], b[0])
        self.slope = self.dy / self.dx if self.dx else None
        self.intercept = self.y - self.slope * self.x if self.dx else None
        self.bounds = (float(self.low), min(float(a[1]), float(b[1])), float(self.high), max(float(a[1]), float(b[1])))


def _polygons(values):
    if type(values) is not list:
        raise TypeError("Expected polygon array")
    for polygon in values:
        if type(polygon) is not list or not polygon:
            raise TypeError("Expected polygon rings")
        for ring in polygon:
            if type(ring) is not list or len(ring) < 4:
                raise TypeError("Expected closed coordinate ring")
            for point in ring:
                if type(point) is not list or len(point) != 2 or any(type(v) not in (int, float) or not math.isfinite(float(v)) for v in point):
                    raise TypeError("Expected finite binary64 coordinates")
            if ring[0] != ring[-1]:
                raise ValueError("Expected closed coordinate ring")
    return values


def _literal(polygons):
    return Counter(tuple(tuple(tuple(float(v).hex() for v in point) for point in ring) for ring in polygon) for polygon in polygons)


def _compile(actual, expected):
    coordinates = {}
    edges = {}
    critical = set()
    def point(values):
        result = []
        for value in values:
            value = float(value)
            if value not in coordinates:
                coordinates[value] = Fraction.from_float(value)
            result.append(coordinates[value])
        critical.add(result[0])
        return tuple(result)
    def polygons(values):
        result = []
        for polygon in values:
            current = []
            for ring in polygon:
                vertices = [point(value) for value in ring]
                for a, b in zip(vertices, vertices[1:]):
                    if a == b:
                        continue
                    key = tuple(sorted((a, b)))
                    if key not in edges:
                        edges[key] = _Edge(a, b)
                    current.append(edges[key])
            result.append(current)
        return result
    left, right = polygons(actual), polygons(expected)
    return left, right, list(edges.values()), critical


def _crossings(edges, critical):
    """🔀 Parallel overlaps change only at already registered endpoint x-values."""
    ordered = sorted(edges, key=lambda edge: edge.bounds[0])
    for i, a in enumerate(ordered):
        for b in ordered[i + 1:]:
            if b.bounds[0] > a.bounds[2]:
                break
            if b.bounds[3] < a.bounds[1] or b.bounds[1] > a.bounds[3]:
                continue
            denominator = a.dx * b.dy - a.dy * b.dx
            if not denominator:
                continue
            x, y = b.x - a.x, b.y - a.y
            t = (x * b.dy - y * b.dx) / denominator
            if not 0 <= t <= 1:
                continue
            u = (x * a.dy - y * a.dx) / denominator
            if 0 <= u <= 1:
                critical.add(a.x + t * a.dx)


def _intervals(polygons, x):
    intervals = []
    for polygon in polygons:
        hits = sorted(edge.slope * x + edge.intercept for edge in polygon if edge.slope is not None and edge.low < x < edge.high)
        if len(hits) % 2:
            raise ValueError("Polygon parity is not closed")
        intervals.extend((a, b) for a, b in zip(hits[::2], hits[1::2]) if a < b)
    intervals.sort()
    merged = []
    for low, high in intervals:
        if merged and low <= merged[-1][1]:
            merged[-1] = (merged[-1][0], max(merged[-1][1], high))
        else:
            merged.append((low, high))
    return merged


def _xor_length(left, right):
    total = sum((b - a for a, b in left), Fraction()) + sum((b - a for a, b in right), Fraction())
    i = j = 0
    while i < len(left) and j < len(right):
        low, high = max(left[i][0], right[j][0]), min(left[i][1], right[j][1])
        if low < high:
            total -= 2 * (high - low)
        if left[i][1] < right[j][1]:
            i += 1
        else:
            j += 1
    return total


def _xor_area(actual, expected):
    """🧮 Integrate exact linear stripe lengths between every segment crossing."""
    if _literal(actual) == _literal(expected):
        return 0.0
    left, right, edges, critical = _compile(actual, expected)
    _crossings(edges, critical)
    positions = sorted(critical)
    areas = []
    for low, high in zip(positions, positions[1:]):
        x = (low + high) / 2
        length = _xor_length(_intervals(left, x), _intervals(right, x))
        area = (high - low) * length
        if area < 0:
            raise ArithmeticError("Negative exact stripe area")
        if area:
            areas.append(float(area))
    return math.fsum(areas)


def overlay_json(request: str) -> str:
    """🧩 Return ordinary JSON XOR areas without a floating topology kernel."""
    if not isinstance(request, str):
        raise TypeError("Overlay request must be a JSON string")
    data = json.loads(request)
    if type(data) is not dict or set(data) != {"cases"} or type(data["cases"]) is not list:
        raise TypeError("Expected overlay cases")
    ids = set()
    result = []
    for case in data["cases"]:
        if type(case) is not dict or set(case) != {"id", "actual", "expected"} or type(case["id"]) is not str or not case["id"] or case["id"] in ids:
            raise ValueError("Expected distinct case identities")
        ids.add(case["id"])
        area = _xor_area(_polygons(case["actual"]), _polygons(case["expected"]))
        if not math.isfinite(area):
            raise ArithmeticError("Overlay area exceeds binary64 output")
        result.append({"id": case["id"], "xor_area": area})
    return json.dumps({"engine": "fraction-sweep", "cases": result}, allow_nan=False, separators=(",", ":"))
