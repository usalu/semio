"""🐍️ Python side of the host protocol conformance case.

Written independently of the other four adapters against the same frozen contract — pairwise
equivalence across five implementations is what stands in for the absent oracle.
"""

from __future__ import annotations

# region 🔖️Imports
import os

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


# region 🔖️Scenarios
def digest_and_fixture_resolution(ctx: Context) -> Outcome:
    """#⃣ Projects the owned digest of the shared vector and of a fixed literal."""
    vector = ctx.fixture_bytes("shared://📄️protocol-vector.txt")
    return Outcome(
        {
            "vectorDigest": digest(vector),
            "literalDigest": digest(b"semio"),
            "fixtureName": "📄️protocol-vector.txt",
            "seed": ctx.seed,
            "level": ctx.scenario["level"],
            "steps": len(ctx.scenario.get("steps", [])),
        }
    )


def fixture_not_in_plan_is_an_error(ctx: Context) -> Outcome:
    """🚫️ An undeclared fixture URI must fail loudly rather than resolve to a default."""
    reported = False
    try:
        ctx.fixture("shared://this-fixture-is-not-declared")
    except KeyError:
        reported = True
    return Outcome({"resolverReportedFailure": reported})


def work_directory_is_cache_local(ctx: Context) -> Outcome:
    """⚡️ A host may only write inside the marked test cache."""
    work_dir = ctx.work_dir.replace("\\", "/")
    return Outcome(
        {
            "insideTestCache": "/.🧬semio/🦑️repo/⚡️cache/tests/" in work_dir,
            "hasOwnershipMarker": os.path.exists(os.path.join(ctx.work_dir, "🧾️marker.json")),
        }
    )


# endregion 🔖️Scenarios


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls."""
    return (
        Adapter("python")
        .subject("digest-and-fixture-resolution", digest_and_fixture_resolution)
        .subject("fixture-not-in-plan-is-an-error", fixture_not_in_plan_is_an_error)
        .subject("work-directory-is-cache-local", work_directory_is_cache_local)
    )


# endregion 🔖️Registration
