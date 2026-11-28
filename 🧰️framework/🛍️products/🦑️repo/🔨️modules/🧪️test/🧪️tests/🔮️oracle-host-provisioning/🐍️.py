"""🔮️ Validates the host provisioning vectors against `packaging`, the reference PEP 440 implementation."""

import json
import pathlib

from packaging.requirements import Requirement
from packaging.version import Version

VECTORS = json.loads((pathlib.Path(__file__).resolve().parent.parent / "🧫️fixtures" / "🔮️oracle-host-provisioning" / "🔣️.json").read_text(encoding="utf-8"))


def satisfies(actual, pin):
    """🔢️ Whether `actual` meets a pin the way pip reads it: a bare release is exact."""
    if pin is None:
        return True
    specifier = f"=={pin}" if pin[0].isdigit() else pin
    return Requirement(f"x{specifier}").specifier.contains(Version(actual))


def test_requirements_are_valid_pip_requirements():
    for row in VECTORS["requirements"]:
        parsed = Requirement(row["expected"])
        assert parsed.name == row["package"]


def test_satisfies_vectors_agree_with_packaging():
    for row in VECTORS["satisfies"]:
        assert satisfies(row["actual"], row.get("pin")) is row["expected"], row
