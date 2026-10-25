import { existsSync } from "node:fs";
import { join } from "node:path";
import { type PolicyCrateRef, policyNewSurfacePriority } from "../🏛️foundation/🟦️.ts";
import type { BreachRecord } from "../../../🚨️policy/🔖️breach/🟦️.ts";
import { canonicalFilenameForKind, canonicalPrimaryFilenameForKind, loadTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { policyStripEmoji } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { policyReaddirSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { artifactFacetPathIsDeclared } from "../../../🔍️discovery/🪪️mutation-owner-identity/🟦️.ts";
import { policySurfaceRoots } from "../../../🔍️discovery/🗺️surface/🟦️.ts";
import { taxonomyMappedFilename } from "../../../🧹️normalization/🔣️taxonomy/🏷️filenames/🟦️.ts";

/**
 * 📏️Taxonomy validator, discovery-contract clause 1: every `🗿️artifacts/<a>/` may only contain the known
 * artifact child vocabulary (`taxonomy.artifactChildDirs`, plus its own leaf file), every `🏅️standards/`
 * descends into `🪆️subsets/<s>/` (`taxonomy.subsetChildDirs`, including the two surfaces), and every
 * `🪟️windows/<w>/` may only contain `taxonomy.windowChildDirs` (plus its own leaf file).
 */
export function policyTaxonomyDirsBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    const ownerRoot = crate.ownerRel;
    const scopeId = crate.pluginId || policyStripEmoji(ownerRoot.split("/").pop() ?? "");

    const artifactsRoot = `${ownerRoot}/${taxonomy.artifactsDirName}`;
    const schemaChildDirs = taxonomy.schemaChildDirs;
    const representationDirs = taxonomy.representationDirs;
    const ioDirectionDirs = taxonomy.ioDirectionDirs;
    const ioDirectionChildDirs = taxonomy.ioDirectionChildDirs;
    const recognizedArtifactChildDirs = [...taxonomy.artifactChildDirs, ...taxonomy.newArtifactChildDirs];

    //#region NestedFacetWalk
    const validateSchemaFacet = (nestedRoot: string, breachScope: string): void => {
      for (const nested of policyReaddirSafe(repoRoot, nestedRoot).filter((e) => e.isDirectory)) {
        if (schemaChildDirs.includes(nested.name)) {
          const repRoot = `${nestedRoot}/${nested.name}`;
          for (const rep of policyReaddirSafe(repoRoot, repRoot).filter((e) => e.isDirectory)) {
            if (artifactFacetPathIsDeclared(`🧬️schema/${nested.name}/${rep.name}`, taxonomy)) continue;
            breaches.push({
              id: `taxonomy-dirs-artifact-${repRoot}-${rep.name}`,
              summary: `"${repRoot}/${rep.name}" is not a recognized semantic schema child`,
              kind: "taxonomy/dirs",
              scope: breachScope,
              priority: policyNewSurfacePriority(crate, "medium"),
              reason: "Schema contains semantic declarations and mutation/inference members; representations belong to I/O.",
              solution: `Move codec "${rep.name}" to 🚪️io/<representation>/${nested.name}, or author a semantic member.`,
            });
          }
          continue;
        }
        breaches.push({
          id: `taxonomy-dirs-artifact-${nestedRoot}-${nested.name}`,
          summary: `"${nestedRoot}/${nested.name}" is not a recognized schema child dir`,
          kind: "taxonomy/dirs",
          scope: breachScope,
          priority: policyNewSurfacePriority(crate, "medium"),
          reason: `Discovery contract: 🧬️schema may only contain ${schemaChildDirs.join(", ")}.`,
          solution: `Move "${nested.name}" into a schemaChildDirs member.`,
        });
      }
    };
    const validateIoFacet = (nestedRoot: string, breachScope: string): void => {
      for (const nested of policyReaddirSafe(repoRoot, nestedRoot).filter((e) => e.isDirectory)) {
        if (representationDirs.includes(nested.name)) {
          const repRoot = `${nestedRoot}/${nested.name}`;
          for (const facet of policyReaddirSafe(repoRoot, repRoot).filter((entry) => entry.isDirectory)) {
            if (artifactFacetPathIsDeclared(`🚪️io/${nested.name}/${facet.name}`, taxonomy)) continue;
            breaches.push({ id: `taxonomy-dirs-artifact-${repRoot}-${facet.name}`, summary: `"${repRoot}/${facet.name}" is not a semantic codec facet`, kind: "taxonomy/dirs", scope: breachScope, priority: "high", reason: "Native I/O representations own semantic codec facets.", solution: `Use ${taxonomy.ioSemanticCollectionDirNames.join(", ")} under ${repRoot}.` });
          }
          continue;
        }
        if (!ioDirectionDirs.includes(nested.name)) {
          breaches.push({
            id: `taxonomy-dirs-artifact-${nestedRoot}-${nested.name}`,
            summary: `"${nestedRoot}/${nested.name}" is not a recognized io direction dir`,
            kind: "taxonomy/dirs",
            scope: breachScope,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Discovery contract: 🚪️io contains ${[...ioDirectionDirs, ...representationDirs].join(", ")}.`,
            solution: `Move "${nested.name}" below a native representation or use a foreign I/O direction.`,
          });
          continue;
        }
        const expectedChild = ioDirectionChildDirs[nested.name];
        const directionRoot = `${nestedRoot}/${nested.name}`;
        for (const codec of policyReaddirSafe(repoRoot, directionRoot).filter((e) => e.isDirectory)) {
          if (expectedChild && codec.name === expectedChild) {
            const codecRoot = `${directionRoot}/${codec.name}`;
            for (const mid of policyReaddirSafe(repoRoot, codecRoot).filter((e) => e.isDirectory)) {
              if (mid.name !== taxonomy.artifactsDirName) {
                breaches.push({
                  id: `taxonomy-dirs-artifact-${codecRoot}-${mid.name}`,
                  summary: `"${codecRoot}/${mid.name}" is not ${taxonomy.artifactsDirName}`,
                  kind: "taxonomy/dirs",
                  scope: breachScope,
                  priority: policyNewSurfacePriority(crate, "medium"),
                  reason: `Discovery contract: deserializers/serializers nest under ${taxonomy.artifactsDirName}/<stdio-artifact>.`,
                  solution: `Move "${mid.name}" under ${taxonomy.artifactsDirName}/.`,
                });
                continue;
              }
              // wildcard stdio artifact dirs under artifacts/
            }
            continue;
          }
          breaches.push({
            id: `taxonomy-dirs-artifact-${directionRoot}-${codec.name}`,
            summary: `"${directionRoot}/${codec.name}" is not the declared io direction child`,
            kind: "taxonomy/dirs",
            scope: breachScope,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Discovery contract: ${nested.name} must contain ${expectedChild}.`,
            solution: `Rename "${codec.name}" to ${expectedChild}.`,
          });
        }
      }
    };
    //#endregion NestedFacetWalk

    for (const artifact of policyReaddirSafe(repoRoot, artifactsRoot).filter((e) => e.isDirectory)) {
      const artifactDir = `${artifactsRoot}/${artifact.name}`;
      const artifactScope = `${scopeId}/${policyStripEmoji(artifact.name)}`;
      for (const child of policyReaddirSafe(repoRoot, artifactDir).filter((e) => e.isDirectory)) {
        if (recognizedArtifactChildDirs.includes(child.name)) {
          if (child.name === "🧬️schema") {
            validateSchemaFacet(`${artifactDir}/${child.name}`, artifactScope);
          } else if (child.name === "🚪️io") {
            validateIoFacet(`${artifactDir}/${child.name}`, artifactScope);
          } else if (child.name === taxonomy.standardsDirName) {
            //#region SubsetFacetWalk
            const standardsRoot = `${artifactDir}/${child.name}`;
            for (const standard of policyReaddirSafe(repoRoot, standardsRoot).filter((e) => e.isDirectory)) {
              const standardDir = `${standardsRoot}/${standard.name}`;
              for (const standardChild of policyReaddirSafe(repoRoot, standardDir).filter((e) => e.isDirectory)) {
                if (!(taxonomy.standardChildDirs ?? []).includes(standardChild.name)) {
                  breaches.push({
                    id: `taxonomy-dirs-standard-${standardDir}-${standardChild.name}`,
                    summary: `"${standardDir}/${standardChild.name}" is not a recognized standard child dir`,
                    kind: "taxonomy/dirs",
                    scope: artifactScope,
                    priority: policyNewSurfacePriority(crate, "medium"),
                    reason: `Discovery contract: a standard dir may only contain ${(taxonomy.standardChildDirs ?? []).join(", ")}.`,
                    solution: `Move "${standardChild.name}" into a recognized standardChildDirs member.`,
                  });
                  continue;
                }
                if (standardChild.name !== taxonomy.subsetsDirName) continue;
                const subsetsRoot = `${standardDir}/${standardChild.name}`;
                for (const subset of policyReaddirSafe(repoRoot, subsetsRoot).filter((e) => e.isDirectory)) {
                  const subsetDir = `${subsetsRoot}/${subset.name}`;
                  const subsetScope = `${artifactScope}/${policyStripEmoji(subset.name)}`;
                  for (const subsetChild of policyReaddirSafe(repoRoot, subsetDir).filter((e) => e.isDirectory)) {
                    if (!taxonomy.subsetChildDirs.includes(subsetChild.name)) {
                      breaches.push({
                        id: `taxonomy-dirs-subset-${subsetDir}-${subsetChild.name}`,
                        summary: `"${subsetDir}/${subsetChild.name}" is not a recognized subset child dir`,
                        kind: "taxonomy/dirs",
                        scope: subsetScope,
                        priority: policyNewSurfacePriority(crate, "medium"),
                        reason: `Discovery contract: a subset dir may only contain ${taxonomy.subsetChildDirs.join(", ")}.`,
                        solution: `Move "${subsetChild.name}" into a recognized subsetChildDirs member.`,
                      });
                      continue;
                    }
                    if (subsetChild.name === "🧬️schema") {
                      validateSchemaFacet(`${subsetDir}/${subsetChild.name}`, subsetScope);
                    } else if (subsetChild.name === "🚪️io") {
                      validateIoFacet(`${subsetDir}/${subsetChild.name}`, subsetScope);
                    } else if (Object.values(taxonomy.surfaceDirNames ?? {}).includes(subsetChild.name)) {
                      const surfaceDir = `${subsetDir}/${subsetChild.name}`;
                      for (const surfaceChild of policyReaddirSafe(repoRoot, surfaceDir).filter((e) => e.isDirectory)) {
                        if (taxonomy.surfaceChildDirs.includes(surfaceChild.name)) continue;
                        breaches.push({
                          id: `taxonomy-dirs-surface-${surfaceDir}-${surfaceChild.name}`,
                          summary: `"${surfaceDir}/${surfaceChild.name}" is not a recognized surface child dir`,
                          kind: "taxonomy/dirs",
                          scope: subsetScope,
                          priority: policyNewSurfacePriority(crate, "medium"),
                          reason: `Discovery contract: a surface dir may only contain ${taxonomy.surfaceChildDirs.join(", ")}.`,
                          solution: `Move "${surfaceChild.name}" into a recognized surfaceChildDirs member.`,
                        });
                      }
                    }
                  }
                }
              }
            }
            //#endregion SubsetFacetWalk
          }
          continue;
        }
        breaches.push({
          id: `taxonomy-dirs-artifact-${artifactDir}-${child.name}`,
          summary: `"${artifactDir}/${child.name}" is not a recognized artifact component dir`,
          kind: "taxonomy/dirs",
          scope: artifactScope,
          priority: policyNewSurfacePriority(crate, "medium"),
          reason: `Discovery contract: an artifact dir may only contain ${recognizedArtifactChildDirs.join(", ")}.`,
          solution: `Move "${child.name}" into a recognized component dir, or if it's a genuinely new taxonomy vocabulary word, add it to 🔣️taxonomy.json's artifactChildDirs with a ticket citation.`,
        });
      }
    }

    const walkForWindows = (relDir: string): void => {
      for (const entry of policyReaddirSafe(repoRoot, relDir).filter((e) => e.isDirectory)) {
        const childRel = `${relDir}/${entry.name}`;
        if (entry.name === taxonomy.windowsDirName) {
          for (const w of policyReaddirSafe(repoRoot, childRel).filter((e) => e.isDirectory)) {
            const windowDir = `${childRel}/${w.name}`;
            for (const child of policyReaddirSafe(repoRoot, windowDir).filter((e) => e.isDirectory)) {
              if (taxonomy.windowChildDirs.includes(child.name)) continue;
              // 🧪️ `testOwnerKinds` names 🪟️windows an owner kind, so a window carries its own
              // tests/fixtures beside its facets — ownership dirs, never window facet vocabulary.
              if (taxonomy.testOwnerKinds.includes(taxonomy.windowsDirName) && (child.name === taxonomy.testsDirName || child.name === taxonomy.testFixturesDirName)) continue;
              breaches.push({
                id: `taxonomy-dirs-window-${windowDir}-${child.name}`,
                summary: `"${windowDir}/${child.name}" is not a recognized window child dir`,
                kind: "taxonomy/dirs",
                scope: `${scopeId}/${policyStripEmoji(w.name)}`,
                priority: policyNewSurfacePriority(crate, "medium"),
                reason: `Discovery contract: a window dir may only contain ${taxonomy.windowChildDirs.join(", ")}.`,
                solution: `Move "${child.name}" into a recognized window-child dir, or if it's a genuinely new taxonomy vocabulary word, add it to 🔣️taxonomy.json's windowChildDirs with a ticket citation.`,
              });
            }
          }
          continue;
        }
        walkForWindows(childRel);
      }
    };
    for (const surfaceRoot of policySurfaceRoots(repoRoot, ownerRoot, taxonomy)) walkForWindows(surfaceRoot);
  }
  return breaches;
}

/**
 * 🪟️Window completeness: every app window declares every required facet explicitly. Empty facets
 * carry only their tracked marker; components are forbidden at facet level and required on every
 * specific item. Shape, marker, and implementation languages come from shared vocabulary.
 */
export function policyWindowCompletenessBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const emptyMarker = canonicalFilenameForKind(taxonomy.windowEmptyFacetFileKindId, taxonomy);
  const componentFilenames = new Set(Object.values(taxonomy.componentFileKinds).map((fileKindId) => canonicalPrimaryFilenameForKind(fileKindId, taxonomy)));
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    const scopeId = crate.pluginId || policyStripEmoji(crate.ownerRel.split("/").pop() ?? "");
    const walk = (relDir: string): void => {
      for (const entry of policyReaddirSafe(repoRoot, relDir).filter((candidate) => candidate.isDirectory)) {
        const childRel = `${relDir}/${entry.name}`;
        if (entry.name === taxonomy.windowsDirName) {
          for (const window of policyReaddirSafe(repoRoot, childRel).filter((candidate) => candidate.isDirectory)) {
            const windowDir = `${childRel}/${window.name}`;
            const actual = new Set(policyReaddirSafe(repoRoot, windowDir).filter((candidate) => candidate.isDirectory).map((candidate) => candidate.name));
            for (const required of taxonomy.windowRequiredChildDirs) {
              const capabilityDir = `${windowDir}/${required}`;
              if (!actual.has(required)) {
                breaches.push({
                  id: `taxonomy-window-completeness-${windowDir}-${required}`,
                  summary: `"${windowDir}" is missing required window capability dir "${required}"`,
                  kind: "taxonomy/window-completeness",
                  scope: `${scopeId}/${policyStripEmoji(window.name)}`,
                  priority: "high",
                  reason: `Every window must explicitly carry ${taxonomy.windowRequiredChildDirs.join(", ")}; an empty capability is valid, an absent capability is not.`,
                  solution: `Add ${capabilityDir}/${emptyMarker}; replace the marker with specific item directories when the facet gains members.`,
                });
                continue;
              }
              const members = policyReaddirSafe(repoRoot, capabilityDir).filter((candidate) => candidate.isDirectory);
              const markerRel = `${capabilityDir}/${emptyMarker}`;
              for (const filename of componentFilenames) {
                if (!existsSync(join(repoRoot, capabilityDir, filename))) continue;
                breaches.push({
                  id: `taxonomy-window-facet-component-${capabilityDir}-${filename}`,
                  summary: `"${capabilityDir}" contains forbidden facet-level component leaf "${filename}"`,
                  kind: "taxonomy/window-facet-component",
                  scope: `${scopeId}/${policyStripEmoji(window.name)}/${policyStripEmoji(required)}`,
                  priority: "high",
                  reason: `Window facets are collections; only specific item directories may contain component leaves.`,
                  solution: `Move ${capabilityDir}/${filename} into ${capabilityDir}/<specific-item>/${filename}, or remove it and add ${markerRel} when the facet is empty.`,
                });
              }
              if (members.length === 0) {
                if (!existsSync(join(repoRoot, markerRel))) {
                  breaches.push({
                    id: `taxonomy-window-empty-facet-${capabilityDir}`,
                    summary: `Empty window facet "${capabilityDir}" is missing marker "${emptyMarker}"`,
                    kind: "taxonomy/window-empty-facet",
                    scope: `${scopeId}/${policyStripEmoji(window.name)}/${policyStripEmoji(required)}`,
                    priority: "high",
                    reason: `An empty required facet must remain tracked without pretending that the facet itself is a specific component.`,
                    solution: `Add ${markerRel}.`,
                  });
                }
                continue;
              }
              if (existsSync(join(repoRoot, markerRel))) {
                breaches.push({
                  id: `taxonomy-window-populated-facet-marker-${capabilityDir}`,
                  summary: `Populated window facet "${capabilityDir}" still contains empty marker "${emptyMarker}"`,
                  kind: "taxonomy/window-empty-facet",
                  scope: `${scopeId}/${policyStripEmoji(window.name)}/${policyStripEmoji(required)}`,
                  priority: "high",
                  reason: `A facet cannot be both empty and populated with specific items.`,
                  solution: `Remove ${markerRel}.`,
                });
              }
              for (const member of members) {
                const unit = { name: member.name, rel: `${capabilityDir}/${member.name}` };
                for (const lang of taxonomy.windowComponentLangs) {
                  const filename = taxonomyMappedFilename(taxonomy, taxonomy.componentFileKinds, lang);
                  if (existsSync(join(repoRoot, unit.rel, filename))) continue;
                  breaches.push({
                    id: `taxonomy-window-component-${unit.rel}-${lang}`,
                    summary: `"${unit.rel}" is missing required ${lang} component leaf "${filename}"`,
                    kind: "taxonomy/window-component",
                    scope: `${scopeId}/${policyStripEmoji(window.name)}/${policyStripEmoji(unit.name)}`,
                    priority: "high",
                    reason: `Every specific window capability item must mirror ${taxonomy.windowComponentLangs.join(", ")}.`,
                    solution: `Add ${unit.rel}/${filename}.`,
                  });
                }
              }
            }
          }
          continue;
        }
        walk(childRel);
      }
    };
    for (const surfaceRoot of policySurfaceRoots(repoRoot, crate.ownerRel, taxonomy)) walk(surfaceRoot);
  }
  return breaches;
}

/**
 * 🎚️Mode completeness: the twin of {@link policyWindowCompletenessBreaches} one level up. A mode is a
 * state-owning scope, so it declares every `modeRequiredChildDirs` member explicitly — its `🪟️windows`
 * collection plus its own three state lanes (`🎚️config` persisted-local, `👥️presence`
 * ephemeral-shared, `🫧️transient` ephemeral-local). An empty child is valid and carries only the
 * tracked `windowEmptyFacetFileKindId` marker; an absent child is not, and a child that is both empty
 * and populated is a contradiction. Ticket 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION
 * wave A0: before it, `modeChildDirs` did not exist at all and a mode declared no children whatsoever.
 * Optional structural capabilities such as `🎮️commands` remain in `modeChildDirs` without forcing
 * an empty marker into every mode.
 */
export function policyModeCompletenessBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const required: readonly string[] = taxonomy.modeRequiredChildDirs ?? taxonomy.modeChildDirs ?? [];
  const emptyMarker = canonicalFilenameForKind(taxonomy.windowEmptyFacetFileKindId, taxonomy);
  const breaches: BreachRecord[] = [];
  if (required.length === 0) return breaches;
  for (const crate of crates) {
    const scopeId = crate.pluginId || policyStripEmoji(crate.ownerRel.split("/").pop() ?? "");
    const walk = (relDir: string): void => {
      for (const entry of policyReaddirSafe(repoRoot, relDir).filter((candidate) => candidate.isDirectory)) {
        const childRel = `${relDir}/${entry.name}`;
        if (entry.name === taxonomy.modesDirName) {
          for (const mode of policyReaddirSafe(repoRoot, childRel).filter((candidate) => candidate.isDirectory)) {
            const modeDir = `${childRel}/${mode.name}`;
            const actual = new Set(policyReaddirSafe(repoRoot, modeDir).filter((candidate) => candidate.isDirectory).map((candidate) => candidate.name));
            for (const child of required) {
              const childDir = `${modeDir}/${child}`;
              const markerRel = `${childDir}/${emptyMarker}`;
              if (!actual.has(child)) {
                breaches.push({
                  id: `taxonomy-mode-completeness-${modeDir}-${child}`,
                  summary: `"${modeDir}" is missing required mode child dir "${child}"`,
                  kind: "taxonomy/mode-completeness",
                  scope: `${scopeId}/${policyStripEmoji(mode.name)}`,
                  priority: "high",
                  reason: `Every mode must explicitly carry ${required.join(", ")}; an empty child is valid, an absent child is not.`,
                  solution: `Add ${childDir}/${emptyMarker}; replace the marker with specific item directories when the child gains members.`,
                });
                continue;
              }
              const members = policyReaddirSafe(repoRoot, childDir).filter((candidate) => candidate.isDirectory);
              if (members.length === 0) {
                if (!existsSync(join(repoRoot, markerRel))) {
                  breaches.push({
                    id: `taxonomy-mode-empty-child-${childDir}`,
                    summary: `Empty mode child "${childDir}" is missing marker "${emptyMarker}"`,
                    kind: "taxonomy/mode-empty-child",
                    scope: `${scopeId}/${policyStripEmoji(mode.name)}/${policyStripEmoji(child)}`,
                    priority: "high",
                    reason: `An empty required child must remain tracked without pretending that the child itself is a specific component.`,
                    solution: `Add ${markerRel}.`,
                  });
                }
                continue;
              }
              if (existsSync(join(repoRoot, markerRel))) {
                breaches.push({
                  id: `taxonomy-mode-populated-child-marker-${childDir}`,
                  summary: `Populated mode child "${childDir}" still contains empty marker "${emptyMarker}"`,
                  kind: "taxonomy/mode-empty-child",
                  scope: `${scopeId}/${policyStripEmoji(mode.name)}/${policyStripEmoji(child)}`,
                  priority: "high",
                  reason: `A child cannot be both empty and populated with specific items.`,
                  solution: `Remove ${markerRel}.`,
                });
              }
            }
          }
          continue;
        }
        walk(childRel);
      }
    };
    for (const surfaceRoot of policySurfaceRoots(repoRoot, crate.ownerRel, taxonomy)) walk(surfaceRoot);
  }
  return breaches;
}
