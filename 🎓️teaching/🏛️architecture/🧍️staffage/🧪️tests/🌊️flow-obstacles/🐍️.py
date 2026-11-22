"""🪨️ Independent oracle of the flow obstacles.

- Sphere: the closed-form potential flow of a uniform stream `U` past a sphere of radius `R` (Batchelor 1967, § 6.8), `u = U + R³/(2r³)·(U − 3(U·r̂)r̂)`, at the fixture's points and just outside its surface, its two stagnation points, and its streamlines integrated in arc length by scipy `solve_ivp` (DOP853).
- Shapes: every superellipsoid of the fixture as a triangle mesh (an icosphere whose vertices are moved onto `f = 1`), and for every probe and eye a trimesh ray cast from the probe towards the eye. A probe is `1` (hidden) when the ray meets the shape shrunk to 0.94, `0` (seen) when it misses the shape itself and `-` in between, where the answer depends on the margin of the product.
- Figure: the posed kernel triangles of the sample figure (written by `bun 🧱️mesh.ts <file>`), their digest, the voxels of a grid whose centre lies at least `deepM` inside one of the solids (trimesh containment and closest point) and the ray casts of the probes against all triangles.

Run `UV_PROJECT_ENVIRONMENT=<machine cache>/staffage-oracles/geometry uv run --project 🧪️tests/🧰️support/🐍️oracles --locked --only-group geometry python -I 🐍️.py [--write --mesh <file>]` (the `geometry` oracle set: numpy, scipy, trimesh, rtree). Without `--write` it recomputes the sphere and compares it with the fixture.
"""
import hashlib
import json
import sys
from pathlib import Path

import numpy as np
import trimesh
from scipy.integrate import solve_ivp

FIXTURE = Path(__file__).resolve().parents[2] / "🧫️fixtures" / "🌊️flow-obstacles" / "🔣️.json"
SKIN = 1e-9


def vec(p):
    return np.array([p["x"], p["y"], p["z"]], dtype=float)


def wind_of(ambient):
    phi = np.radians(ambient["windDirectionDeg"])
    return ambient["windMS"] * np.array([np.cos(phi), 0.0, np.sin(phi)])


def potential(u, centre, radius, at):
    r = np.asarray(at, dtype=float) - centre
    d = np.linalg.norm(r)
    rh = r / d
    return u + radius**3 / (2 * d**3) * (u - 3 * np.dot(u, rh) * rh)


def sphere(fixture):
    case = fixture["sphere"]
    centre, radius, u = vec(case["obstacle"]["centre"]), case["obstacle"]["radiiM"][0], wind_of(fixture["ambient"])
    samples = [{"at": at, "u": potential(u, centre, radius, at).tolist()} for at in case["points"]]
    surface = []
    for theta, phi in case["surface"]:
        n = np.array([np.cos(theta), np.sin(theta) * np.cos(phi), np.sin(theta) * np.sin(phi)])
        at = centre + radius * (1 + SKIN) * n
        surface.append({"at": at.tolist(), "normal": n.tolist(), "u": potential(u, centre, radius, at).tolist()})
    uh = u / np.linalg.norm(u)
    stagnation = [(centre + sign * radius * (1 + SKIN) * uh).tolist() for sign in (-1.0, 1.0)]
    streams = []
    step, count = case["stepM"], case["streamPoints"]
    for seed in case["seeds"]:
        def by_arc(_s, state):
            v = potential(u, centre, radius, state[:3])
            speed = float(np.linalg.norm(v))
            return [*(v / speed), 1 / speed]

        arc = solve_ivp(by_arc, (0, step * count), [*seed, 0.0], method="DOP853", t_eval=np.arange(1, count + 1) * step, rtol=1e-11, atol=1e-13)
        streams.append({"seed": seed, "points": np.round(arc.y[:3].T, 9).tolist(), "travelS": np.round(arc.y[3], 9).tolist(), "closestM": float(np.min(np.linalg.norm(arc.y[:3].T - centre, axis=1)) - radius)})
    return {"samples": samples, "surface": surface, "stagnation": stagnation, "equatorSpeedMS": 1.5 * float(np.linalg.norm(u)), "streams": streams}


def level(obstacle, points):
    q = np.stack([(points - vec(obstacle["centre"])) @ vec(axis) / r for axis, r in zip(obstacle["axes"], obstacle["radiiM"])], axis=1)
    return np.sum(np.abs(q) ** obstacle["exponent"], axis=1) ** (1 / obstacle["exponent"])


def shape_mesh(obstacle, scale):
    ball = trimesh.creation.icosphere(subdivisions=4)
    centre = vec(obstacle["centre"])
    world = sum(np.outer(ball.vertices[:, k] * r, vec(axis)) for k, (axis, r) in enumerate(zip(obstacle["axes"], obstacle["radiiM"])))
    on = world / level(obstacle, world + centre)[:, None]
    return trimesh.Trimesh(vertices=centre + scale * on, faces=ball.faces, process=False)


def probes_of(fixture):
    p = fixture["probes"]
    axes = [np.linspace(p["min"][k], p["max"][k], p["counts"][k]) for k in range(3)]
    return np.array([[x, y, z] for x in axes[0] for y in axes[1] for z in axes[2]])


def rays(mesh, origins, eye):
    directions = np.tile(vec(eye["toward"]), (len(origins), 1)) if "toward" in eye else vec(eye["at"]) - origins
    directions = directions / np.linalg.norm(directions, axis=1)[:, None]
    return mesh.ray.intersects_any(origins, directions)


def shapes(fixture):
    probes = probes_of(fixture)
    out = {}
    for shape in fixture["shapes"]:
        small, whole = shape_mesh(shape["obstacle"], 0.94), shape_mesh(shape["obstacle"], 1.0)
        rows = []
        for eye in fixture["eyes"]:
            hidden, grazed = rays(small, probes, eye), rays(whole, probes, eye)
            rows.append("".join("1" if h else "-" if g else "0" for h, g in zip(hidden, grazed)))
        out[shape["id"]] = rows
    return out


def figure(fixture, mesh_path):
    case = fixture["figure"]
    data = json.loads(Path(mesh_path).read_text(encoding="utf-8"))
    vertices = np.array(data["vertices"], dtype=float).reshape(-1, 3)
    triangles = np.array(data["triangles"], dtype=np.int64).reshape(-1, 3)
    digest = hashlib.sha256(json.dumps([[int(round(v * 1e6)) for v in data["vertices"]], data["triangles"]], separators=(",", ":")).encode("utf-8")).hexdigest()
    if digest != data["digest"]:
        raise SystemExit("the mesh file does not carry its own digest")
    size = case["voxelM"]
    lo = np.floor(vertices.min(axis=0) / size) * size - size / 2
    counts = np.ceil((vertices.max(axis=0) - lo) / size).astype(int) + 1
    grid = np.array([[i, j, k] for i in range(counts[0]) for j in range(counts[1]) for k in range(counts[2])])
    centres = lo + grid * size
    deep = np.zeros(len(centres), dtype=bool)
    watertight = 0
    for part in data["parts"]:
        faces = triangles[part["first"] : part["first"] + part["count"]]
        used, inverse = np.unique(faces, return_inverse=True)
        solid = trimesh.Trimesh(vertices=vertices[used], faces=inverse.reshape(-1, 3), process=True)
        watertight += int(solid.is_watertight)
        if not solid.is_watertight or solid.volume <= 0:
            continue
        near = np.all((centres >= solid.bounds[0]) & (centres <= solid.bounds[1]), axis=1)
        index = np.flatnonzero(near)
        if len(index) == 0:
            continue
        inside = solid.contains(centres[index])
        index = index[inside]
        if len(index) == 0:
            continue
        _, distance, _ = trimesh.proximity.closest_point(solid, centres[index])
        deep[index[distance >= case["deepM"]]] = True
    whole = trimesh.Trimesh(vertices=vertices, faces=triangles, process=False)
    probes = probes_of(fixture)
    return {
        "meshDigest": digest,
        "triangles": int(len(triangles)),
        "watertightSolids": watertight,
        "solids": len(data["parts"]),
        "voxels": {"originM": lo.tolist(), "sizeM": size, "counts": counts.tolist(), "deep": [int(i) for i in np.flatnonzero(deep)]},
        "hidden": ["".join("1" if h else "0" for h in rays(whole, probes, eye)) for eye in fixture["eyes"]],
    }


if __name__ == "__main__":
    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    computed = sphere(fixture)
    if "--write" in sys.argv:
        expected = dict(fixture.get("expected", {}))
        expected["sphere"] = computed
        expected["shapes"] = shapes(fixture)
        if "--mesh" in sys.argv:
            expected["figure"] = {**expected.get("figure", {}), **figure(fixture, sys.argv[sys.argv.index("--mesh") + 1])}
        fixture["expected"] = expected
        FIXTURE.write_text(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
        print(json.dumps({"written": sorted(expected.keys()), "figure": {k: v for k, v in expected.get("figure", {}).items() if k in ("triangles", "watertightSolids", "solids")}, "deepVoxels": len(expected.get("figure", {}).get("voxels", {}).get("deep", []))}))
    else:
        worst = 0.0
        for mine, theirs in zip(computed["samples"], fixture["expected"]["sphere"]["samples"]):
            worst = max(worst, *(abs(a - b) for a, b in zip(mine["u"], theirs["u"])))
        print(json.dumps({"worstVelocityDifferenceMS": worst}))
        sys.exit(0 if worst < 1e-12 else 1)
