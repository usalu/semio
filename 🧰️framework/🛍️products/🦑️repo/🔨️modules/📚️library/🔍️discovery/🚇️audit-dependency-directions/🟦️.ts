import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 🚇️ Audits the dependency direction roles and rules, the Cargo dependency direction rules and the area layers. */
export function auditDependencyDirections(audit: TaxonomyAudit): void {
  const { taxonomy, problems, record, workspacePath } = audit;
  if (record(taxonomy.dependencyDirections, "dependencyDirections") && record(taxonomy.dependencyDirections.roles, "dependencyDirections.roles") && record(taxonomy.dependencyDirections.rules, "dependencyDirections.rules")) {
    for (const [role, contribution] of Object.entries(taxonomy.dependencyDirections.roles)) {
      if (!role || !record(contribution, `dependencyDirections.roles.${role}`)) continue;
      if (!Array.isArray(contribution.ownerPaths) || !Array.isArray(contribution.externalPackages)) problems.push(`dependencyDirections.roles.${role} requires ownerPaths and externalPackages arrays.`);
      else {
        for (const path of contribution.ownerPaths) workspacePath(path, `dependencyDirections.roles.${role}.ownerPaths`);
        for (const name of contribution.externalPackages) if (typeof name !== "string" || !/^(?:@[a-z0-9][a-z0-9._-]*\/)?[a-z0-9][a-z0-9._-]*$/u.test(name)) problems.push(`dependencyDirections.roles.${role} has an invalid external package.`);
      }
    }
    const vendoredPaths: unknown = taxonomy.dependencyDirections.vendoredPaths;
    if (!Array.isArray(vendoredPaths) || new Set(vendoredPaths).size !== vendoredPaths.length) problems.push("dependencyDirections.vendoredPaths requires a unique array of vendored third-party source paths.");
    else for (const path of vendoredPaths) workspacePath(path, "dependencyDirections.vendoredPaths");
    for (const [name, rule] of Object.entries(taxonomy.dependencyDirections.rules)) {
      if (!name || !record(rule, `dependencyDirections.rules.${name}`)) continue;
      for (const [field, roles] of [["fromRoles", rule.fromRoles], ["toRoles", rule.toRoles]] as const) {
        if (!Array.isArray(roles) || !roles.length) problems.push(`dependencyDirections.rules.${name}.${field} requires nonempty roles.`);
        else for (const role of roles) if (!taxonomy.dependencyDirections.roles[role]) problems.push(`dependencyDirections.rules.${name}.${field} references unknown role ${JSON.stringify(role)}.`);
      }
    }
  }
  if (record(taxonomy.dependencyDirections, "dependencyDirections") && record(taxonomy.dependencyDirections.roles, "dependencyDirections.roles")) {
    const boundary: unknown = taxonomy.dependencyDirections.uiBoundary, roles = taxonomy.dependencyDirections.roles;
    if (record(boundary, "dependencyDirections.uiBoundary")) {
      const { version, subjectRole, foundationRoles, testSupportRoles } = boundary;
      if (Object.keys(boundary).sort().join("|") !== "foundationRoles|subjectRole|testSupportRoles|version" || version !== 1) problems.push("dependencyDirections.uiBoundary requires exactly version 1, subjectRole, foundationRoles and testSupportRoles.");
      else if (typeof subjectRole !== "string" || !roles[subjectRole]) problems.push("dependencyDirections.uiBoundary.subjectRole references an unknown role.");
      else for (const [field, selected] of [["foundationRoles", foundationRoles], ["testSupportRoles", testSupportRoles]] as const) {
        if (!Array.isArray(selected) || !selected.length || new Set(selected).size !== selected.length || selected.some((role) => typeof role !== "string" || !roles[role] || role === subjectRole)) problems.push(`dependencyDirections.uiBoundary.${field} requires distinct known roles other than the subject.`);
      }
      if (Array.isArray(foundationRoles) && Array.isArray(testSupportRoles) && foundationRoles.some((role) => testSupportRoles.includes(role))) problems.push("dependencyDirections.uiBoundary foundation and test support roles must be disjoint.");
    }
  }
  if (record(taxonomy.cargoDependencyDirections, "cargoDependencyDirections") && record(taxonomy.cargoDependencyDirections.rules, "cargoDependencyDirections.rules")) {
    const roles = taxonomy.cargoDependencyDirections.roles;
    const ownerRoles = taxonomy.cargoDependencyDirections.ownerRoles;
    if (!Array.isArray(roles) || !roles.length || roles.some((role) => typeof role !== "string" || !role) || new Set(roles).size !== roles.length) problems.push("cargoDependencyDirections requires distinct declared semantic roles.");
    else for (const [name, rule] of Object.entries(taxonomy.cargoDependencyDirections.rules)) {
      if (!name || !record(rule, `cargoDependencyDirections.rules.${name}`)) continue;
      for (const selected of [rule.fromRoles, rule.toRoles]) if (!Array.isArray(selected) || new Set(selected).size !== selected.length || selected.some((role) => !roles.includes(role))) problems.push(`cargoDependencyDirections.rules.${name} references an unknown semantic role.`);
      if (Object.keys(rule).some(key => !["fromRoles", "toRoles", "fromOwnerPaths", "toOwnerSegments"].includes(key))) problems.push("Cargo direction rules must use only declared selector fields.");
      if ((!rule.fromRoles?.length && !rule.fromOwnerPaths?.length) || (!rule.toRoles?.length && !rule.toOwnerSegments?.length)) problems.push("Cargo direction rules require nonempty source and target selectors.");
      if (rule.toOwnerSegments !== undefined && (!Array.isArray(rule.toOwnerSegments) || !rule.toOwnerSegments.length || new Set(rule.toOwnerSegments).size !== rule.toOwnerSegments.length || rule.toOwnerSegments.some((segment: unknown) => typeof segment !== "string" || !segment || /[/\\\\]/u.test(segment) || segment === "." || segment === ".."))) problems.push("Cargo direction rules require distinct exact target owner segments.");
      if (rule.fromOwnerPaths !== undefined) {
        if (!Array.isArray(rule.fromOwnerPaths) || !rule.fromOwnerPaths.length || new Set(rule.fromOwnerPaths).size !== rule.fromOwnerPaths.length) problems.push(`cargoDependencyDirections.rules.${name} requires distinct source owner patterns.`);
        else for (const path of rule.fromOwnerPaths) {
          if (typeof path !== "string" || !path) problems.push(`cargoDependencyDirections.rules.${name} requires readable source owner patterns.`);
          try { new RegExp(path, "u"); } catch { problems.push(`cargoDependencyDirections.rules.${name} has an invalid source owner pattern.`); }
        }
      }
    }
    if (!Array.isArray(ownerRoles) || !ownerRoles.length || new Set(ownerRoles.map((rule) => rule.path)).size !== ownerRoles.length) problems.push("cargoDependencyDirections requires distinct physical owner role classifications.");
    else for (const rule of ownerRoles) {
      if (typeof rule.path !== "string" || !rule.path || !Array.isArray(rule.roles) || !rule.roles.length || new Set(rule.roles).size !== rule.roles.length || rule.roles.some((role: string) => !roles?.includes(role))) problems.push("cargoDependencyDirections owner classifications require readable paths and known distinct roles.");
      try { new RegExp(rule.path, "u"); } catch { problems.push(`cargoDependencyDirections owner classification has an invalid path pattern: ${JSON.stringify(rule.path)}.`); }
    }
    if (JSON.stringify(taxonomy.roles) !== JSON.stringify(roles)) problems.push("Package discovery and Cargo direction must use the same semantic role vocabulary.");
  }
  if (record(taxonomy.areaLayers, "areaLayers")) for (const [area, layer] of Object.entries(taxonomy.areaLayers)) {
    if (area === "compose" || area.startsWith("compose/") || area === "temp/compose" || area.startsWith("temp/compose/")) problems.push("Opaque compose prefixes must not appear in areaLayers.");
    if (!["framework", "implementation", "repo-wide"].includes(layer)) problems.push(`areaLayers[${JSON.stringify(area)}] is invalid.`);
  }
}
