// 🔷️ .NET side of the host protocol conformance case. Written independently of the other four
// adapters against the same frozen contract — pairwise equivalence is the whole point.

using Semio.Repo.Test;

#region 🔖️Scenarios

internal static class Scenarios
{
    /// <summary>#⃣ Projects the owned digest of the shared vector and of a fixed literal.</summary>
    internal static Outcome DigestAndFixtureResolution(Context ctx) => new(new Dictionary<string, object?>
    {
        ["vectorDigest"] = TestHost.Digest(ctx.FixtureBytes("shared://📄️protocol-vector.txt")),
        ["literalDigest"] = TestHost.Digest(System.Text.Encoding.UTF8.GetBytes("semio")),
        ["fixtureName"] = "📄️protocol-vector.txt",
        ["seed"] = ctx.Seed,
        ["level"] = ctx.Scenario.Level,
        ["steps"] = ctx.Scenario.Steps?.Count ?? 0,
    });

    /// <summary>🚫️ An undeclared fixture URI must fail loudly rather than resolve to a default.</summary>
    internal static Outcome FixtureNotInPlanIsAnError(Context ctx)
    {
        var reported = false;
        try
        {
            ctx.Fixture("shared://this-fixture-is-not-declared");
        }
        catch (KeyNotFoundException)
        {
            reported = true;
        }
        return new Outcome(new Dictionary<string, object?> { ["resolverReportedFailure"] = reported });
    }

    /// <summary>⚡️ A host may only write inside the marked test cache.</summary>
    internal static Outcome WorkDirectoryIsCacheLocal(Context ctx)
    {
        var workDir = ctx.WorkDir.Replace("\\", "/");
        return new Outcome(new Dictionary<string, object?>
        {
            ["insideTestCache"] = workDir.Contains("/.🧬semio/🦑️repo/⚡️cache/tests/"),
            ["hasOwnershipMarker"] = File.Exists(Path.Combine(ctx.WorkDir, "🧾️marker.json")),
        });
    }
}

#endregion 🔖️Scenarios

#region 🔖️Registration

internal static class Adapter
{
    /// <summary>🧭️ Registration entry point the generated host calls.</summary>
    internal static Semio.Repo.Test.Adapter Create() => new Semio.Repo.Test.Adapter("dotnet")
        .Subject("digest-and-fixture-resolution", Scenarios.DigestAndFixtureResolution)
        .Subject("fixture-not-in-plan-is-an-error", Scenarios.FixtureNotInPlanIsAnError)
        .Subject("work-directory-is-cache-local", Scenarios.WorkDirectoryIsCacheLocal);
}

#endregion 🔖️Registration
