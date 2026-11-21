"""🌀️ Verify mounted planar element cells with independent NumPy quadrature."""

import csv
from pathlib import Path

import numpy as np
from numpy.polynomial.legendre import leggauss


def reference(case):
    """🧮️ Integrate line shape functions or contract a triangle's affine gradients."""
    positions = np.array(case["positions"])[:, :2]
    displacement = np.array(case["displacement"])
    kind = case["kind"]
    density = case["density"]
    elastic = case["e"]
    if kind in ("stress", "strain"):
        affine = np.column_stack((np.ones(3), positions))
        gradients = np.linalg.inv(affine)[1:].T
        area = np.linalg.det(affine) / 2
        field = displacement.reshape(3, 2).T @ gradients
        poisson = case["nu"]
        shear = elastic / (2 * (1 + poisson))
        lame = elastic * poisson / ((1 + poisson) * (1 - 2 * poisson))
        if kind == "stress":
            lame = elastic * poisson / (1 - poisson**2)
        stress = shear * (field + field.T) + lame * np.trace(field) * np.eye(2)
        scale = area * case["thickness"]
        mass = density * scale / 12 * (np.ones((3, 3)) + np.eye(3))
        geometric = scale * gradients @ stress @ gradients.T
        return np.kron(mass, np.eye(2)), np.kron(geometric, np.eye(2))

    span = positions[1] - positions[0]
    length = np.linalg.norm(span)
    direction = span / length
    transverse = np.array([-direction[1], direction[0]])
    area = case["area"]
    dofs = displacement.reshape(2, -1)
    axial_force = elastic * area / length * (dofs[1, :2] - dofs[0, :2]) @ direction
    mass = np.zeros((displacement.size, displacement.size))
    geometric = np.zeros_like(mass)
    for point, weight in zip(*leggauss(4)):
        t = (point + 1) / 2
        weight *= length / 2
        linear = np.array([1 - t, t])
        if kind == "bar":
            mass += density * area * weight * np.kron(np.outer(linear, linear), np.eye(2))
            gradient = np.kron([-1, 1], transverse) / length
        else:
            hermite = np.array([1 - 3*t*t + 2*t**3, length*t*(1 - t)**2,
                                3*t*t - 2*t**3, length*t*t*(t - 1)])
            derivatives = np.array([(-6*t + 6*t*t) / length, 1 - 4*t + 3*t*t,
                                    (6*t - 6*t*t) / length, -2*t + 3*t*t])
            axial = np.kron(linear, np.append(direction, 0))
            lateral = np.r_[hermite[0] * transverse, hermite[1], hermite[2] * transverse, hermite[3]]
            gradient = np.r_[derivatives[0] * transverse, derivatives[1],
                             derivatives[2] * transverse, derivatives[3]]
            mass += density * area * weight * (np.outer(axial, axial) + np.outer(lateral, lateral))
        geometric += axial_force * weight * np.outer(gradient, gradient)
    return mass, geometric


if __name__ == "__main__":
    fixture = Path(__file__).resolve().parents[1] / "🧫️fixtures" / "🌀️mounted-cells" / "🔣️.tsv"
    with fixture.open(encoding="utf-8", newline="") as stream:
        cases = list(csv.DictReader(stream, delimiter="\t"))
    for case in cases:
        for key in ("e", "density", "area", "nu", "thickness"):
            if case[key]:
                case[key] = float(case[key])
        for key in ("positions", "displacement", "mass", "geometric"):
            case[key] = np.fromstring(case[key], sep=",")
        case["positions"] = case["positions"].reshape(-1, 3)
        for key, matrix in zip(("mass", "geometric"), reference(case)):
            np.testing.assert_allclose(matrix.ravel(), case[key], rtol=1e-12, atol=1e-12,
                                       err_msg=f'{case["id"]} {key}')
    print(f'Independent NumPy oracle verified {len(cases)} cases')
