// 🧪️ `Semio.Repo.Test` — the .NET native host of the repository test platform.
//
// The taxonomy filename `🔷️component.cs` is not an xUnit discovery name, and committing a wrapper
// test project next to it would create a duplicate test hierarchy. The coordinator materializes a
// cache-local project that links this support library and the committed adapter; nothing here is
// generated. The host never parses a feature file — the plan is the whole contract.

using System.Security.Cryptography;
using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace Semio.Repo.Test;

#region 🔖️Protocol

/// <summary>🧫️ One immutable fixture the coordinator resolved for this case.</summary>
public sealed record Fixture(
    [property: JsonPropertyName("uri")] string Uri,
    [property: JsonPropertyName("scope")] string Scope,
    [property: JsonPropertyName("name")] string Name,
    [property: JsonPropertyName("path")] string Path,
    [property: JsonPropertyName("digest")] string Digest);

/// <summary>🥒️ One Given/When/Then step of a planned scenario.</summary>
public sealed record Step(
    [property: JsonPropertyName("keyword")] string Keyword,
    [property: JsonPropertyName("text")] string Text);

/// <summary>🥒️ One planned scenario, already expanded and level-filtered by the coordinator.</summary>
public sealed record Scenario(
    [property: JsonPropertyName("id")] string Id,
    [property: JsonPropertyName("name")] string Name,
    [property: JsonPropertyName("level")] string Level,
    [property: JsonPropertyName("mode")] string Mode,
    [property: JsonPropertyName("seed")] string? Seed,
    [property: JsonPropertyName("steps")] List<Step>? Steps);

/// <summary>📋️ The owned execution plan one host receives.</summary>
public sealed record Plan(
    [property: JsonPropertyName("owner")] string Owner,
    [property: JsonPropertyName("case")] string Case,
    [property: JsonPropertyName("capability")] string Capability,
    [property: JsonPropertyName("comparison")] string Comparison,
    [property: JsonPropertyName("featureHash")] string FeatureHash,
    [property: JsonPropertyName("level")] string Level,
    [property: JsonPropertyName("role")] string Role,
    [property: JsonPropertyName("implementation")] string Implementation,
    [property: JsonPropertyName("workDir")] string WorkDir,
    [property: JsonPropertyName("outputDir")] string OutputDir,
    [property: JsonPropertyName("resultsPath")] string ResultsPath,
    [property: JsonPropertyName("fixtures")] List<Fixture>? Fixtures,
    [property: JsonPropertyName("scenarios")] List<Scenario>? Scenarios);

/// <summary>💬️ One message attached to a result.</summary>
public sealed record Diagnostic(
    [property: JsonPropertyName("severity")] string Severity,
    [property: JsonPropertyName("message")] string Message,
    [property: JsonPropertyName("detail")] string? Detail = null);

#endregion 🔖️Protocol

#region 🔖️Adapter

/// <summary>🎯️ What one scenario handler returns: the raw artifact and the compared projection.</summary>
public sealed class Outcome
{
    /// <summary>🎯️ Creates an outcome from a projection and an optional raw artifact.</summary>
    public Outcome(object? projection, byte[]? raw = null, IReadOnlyList<Diagnostic>? diagnostics = null)
    {
        Projection = projection;
        Raw = raw;
        Diagnostics = diagnostics ?? Array.Empty<Diagnostic>();
    }

    public object? Projection { get; }
    public byte[]? Raw { get; }
    public IReadOnlyList<Diagnostic> Diagnostics { get; }
}

/// <summary>🧭️ Everything one scenario handler is given.</summary>
public sealed class Context
{
    internal Context(Plan plan, Scenario scenario, string role, string repoRoot)
    {
        Plan = plan;
        Scenario = scenario;
        Role = role;
        RepoRoot = repoRoot;
    }

    public Plan Plan { get; }
    public Scenario Scenario { get; }
    public string Role { get; }
    public string RepoRoot { get; }
    public string WorkDir => Plan.WorkDir;

    /// <summary>🧫️ Absolute path of a declared fixture; an undeclared URI throws.</summary>
    public string Fixture(string uri)
    {
        var match = (Plan.Fixtures ?? new List<Fixture>()).FirstOrDefault(entry => entry.Uri == uri)
            ?? throw new KeyNotFoundException($"fixture {uri} is not part of this plan — declare it in the feature file");
        return System.IO.Path.Combine(RepoRoot, match.Path);
    }

    /// <summary>🧫️ Bytes of a declared fixture.</summary>
    public byte[] FixtureBytes(string uri) => File.ReadAllBytes(Fixture(uri));

    /// <summary>🧫️ Copies an immutable fixture into the work directory and returns the mutable copy.</summary>
    public string CopyFixture(string uri, string? asName = null)
    {
        var source = Fixture(uri);
        Directory.CreateDirectory(WorkDir);
        var target = System.IO.Path.Combine(WorkDir, asName ?? System.IO.Path.GetFileName(source));
        File.Copy(source, target, overwrite: true);
        return target;
    }

    /// <summary>🎲️ Deterministic seed declared by the scenario's <c>@seed-…</c> tag.</summary>
    public long Seed => long.TryParse(Scenario.Seed, out var value) ? value : 0;
}

/// <summary>🧭️ One implementation's registration for a case.</summary>
public sealed class Adapter
{
    private readonly Dictionary<string, Func<Context, Outcome>> handlers = new();

    /// <summary>🧭️ Starts a registration for the given implementation id.</summary>
    public Adapter(string implementation = "dotnet") => Implementation = implementation;

    public string Implementation { get; }

    /// <summary>🔮️ Registers the reference-implementation handler for one scenario.</summary>
    public Adapter Oracle(string scenario, Func<Context, Outcome> handler)
    {
        handlers[scenario + "::oracle"] = handler;
        return this;
    }

    /// <summary>🎯️ Registers this repository's handler for one scenario.</summary>
    public Adapter Subject(string scenario, Func<Context, Outcome> handler)
    {
        handlers[scenario + "::subject"] = handler;
        return this;
    }

    internal Func<Context, Outcome>? Handler(string scenario, string role) => handlers.TryGetValue(scenario + "::" + role, out var handler) ? handler : null;
}

#endregion 🔖️Adapter

#region 🔖️Runner

/// <summary>🚪️ .NET host entry.</summary>
public static class TestHost
{
    private static readonly JsonSerializerOptions PlanOptions = new() { PropertyNameCaseInsensitive = true };

    /// <summary>#⃣ The coordinator's content digest: sha256, hex, truncated to 32 characters.</summary>
    public static string Digest(byte[]? payload) => Convert.ToHexString(SHA256.HashData(payload ?? Array.Empty<byte>())).ToLowerInvariant()[..32];

    private static string? FlagValue(string[] argv, string flag)
    {
        var index = Array.IndexOf(argv, flag);
        return index >= 0 && index + 1 < argv.Length ? argv[index + 1] : null;
    }

    private static string RepoRootFrom(string start)
    {
        var directory = System.IO.Path.GetFullPath(start);
        for (var i = 0; i < 32; i++)
        {
            if (File.Exists(System.IO.Path.Combine(directory, "nx.json")) && File.Exists(System.IO.Path.Combine(directory, "package.json"))) return directory;
            var parent = System.IO.Path.GetDirectoryName(directory);
            if (parent is null || parent == directory) break;
            directory = parent;
        }
        return Directory.GetCurrentDirectory();
    }

    /// <summary>🚪️ Loads the plan, executes every planned scenario against the adapter, emits JSONL.</summary>
    public static int RunMain(Adapter adapter, string[] argv)
    {
        var planPath = FlagValue(argv, "--plan");
        var outPath = FlagValue(argv, "--out");
        if (planPath is null || outPath is null)
        {
            Console.Error.WriteLine("usage: host --plan <plan.json> --out <results.jsonl>");
            return 2;
        }
        var plan = JsonSerializer.Deserialize<Plan>(File.ReadAllText(planPath), PlanOptions);
        if (plan is null)
        {
            Console.Error.WriteLine($"malformed plan {planPath}");
            return 2;
        }
        var repoRoot = RepoRootFrom(plan.WorkDir);
        Directory.CreateDirectory(plan.WorkDir);
        Directory.CreateDirectory(plan.OutputDir);

        var lines = new List<string>();
        var failed = false;
        foreach (var scenario in plan.Scenarios ?? new List<Scenario>())
        {
            var started = DateTime.UtcNow;
            var diagnostics = new List<Diagnostic>();
            string status;
            object? projection = null;
            byte[]? raw = null;
            var handler = adapter.Handler(scenario.Id, plan.Role);
            if (handler is null)
            {
                failed = true;
                status = "errored";
                diagnostics.Add(new Diagnostic("error", $"adapter has no {plan.Role} registration for scenario {scenario.Id}"));
            }
            else
            {
                try
                {
                    var outcome = handler(new Context(plan, scenario, plan.Role, repoRoot));
                    status = "passed";
                    projection = outcome.Projection;
                    raw = outcome.Raw;
                    diagnostics.AddRange(outcome.Diagnostics);
                }
                catch (Exception error)
                {
                    failed = true;
                    status = "failed";
                    diagnostics.Add(new Diagnostic("error", error.Message, error.ToString()));
                }
            }

            var projectionBytes = Encoding.UTF8.GetBytes(JsonSerializer.Serialize(projection));
            var output = new Dictionary<string, object?>
            {
                ["rawHash"] = Digest(raw),
                ["projectionHash"] = Digest(projectionBytes),
                ["projection"] = projection,
            };
            if (raw is not null)
            {
                var rawPath = System.IO.Path.Combine(plan.OutputDir, $"{scenario.Id}.{plan.Role}.raw");
                File.WriteAllBytes(rawPath, raw);
                output["rawPath"] = rawPath;
            }
            var projectionPath = System.IO.Path.Combine(plan.OutputDir, $"{scenario.Id}.{plan.Role}.projection.json");
            File.WriteAllBytes(projectionPath, projectionBytes);
            output["projectionPath"] = projectionPath;

            var result = new Dictionary<string, object?>
            {
                ["testId"] = $"{plan.Owner}::{plan.Case}::{scenario.Id}::{plan.Implementation}::{plan.Role}",
                ["owner"] = plan.Owner,
                ["case"] = plan.Case,
                ["scenario"] = scenario.Id,
                ["implementation"] = plan.Implementation,
                ["role"] = plan.Role,
                ["level"] = scenario.Level,
                ["status"] = status,
                ["durationMs"] = (DateTime.UtcNow - started).TotalMilliseconds,
                ["seed"] = scenario.Seed ?? string.Empty,
                ["featureHash"] = plan.FeatureHash,
                ["output"] = output,
                ["diagnostics"] = diagnostics,
            };
            lines.Add(JsonSerializer.Serialize(result));
        }

        Directory.CreateDirectory(System.IO.Path.GetDirectoryName(System.IO.Path.GetFullPath(outPath))!);
        File.WriteAllText(outPath, lines.Count > 0 ? string.Join("\n", lines) + "\n" : string.Empty);
        return failed ? 1 : 0;
    }
}

#endregion 🔖️Runner
