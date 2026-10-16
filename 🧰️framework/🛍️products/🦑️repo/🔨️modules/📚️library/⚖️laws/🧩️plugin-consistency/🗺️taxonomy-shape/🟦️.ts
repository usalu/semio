/** 🧭️ Wave 4 plugin consistency policy: structural rules over the Shape V2 taxonomy tree.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */
import { canonicalFilenameForKind, canonicalPrimaryFilenameForKind, createFixedContractResolver, discoverPackages, fixedFilenameContractIdsForPath, loadTaxonomy, semanticDirectoryKindId, type BreachRecord } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { artifactFacetPathIsDeclared, taxonomyRelativePathIsExcluded } from "../../../🔍️discovery/🟦️.ts";
import { pathEmojiStatuteFindings, type PathEmojiEntry } from "../../../../../../../🔨️modules/🪪️identity/🛣️path/🟦️.ts";
import { policyReaddirSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyLineOfIndex } from "../../../🔍️discovery/📍️source-coordinate/🟦️.ts";
import { policySurfaceRoots } from "../../../🔍️discovery/🗺️surface/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME, policyLeadingEmojiPrefix, policyStripEmoji } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { existsSync, lstatSync, readFileSync, statSync } from "node:fs";
import { join, posix } from "node:path";
import { type PolicyCrateRef, policyExtractFnBody, policyNewSurfacePriority } from "../🏛️foundation/🟦️.ts";
import { taxonomyEcosystemEntryFilenames, taxonomyMappedFilename } from "../../../🧹️normalization/🔣️taxonomy/🏷️filenames/🟦️.ts";

//#region 🔧️PolicyRuleTaxonomy
/**
 * 🎫️ Structural rules over the Shape V2 taxonomy tree (`<owner>/📦️packages/<lang>[/🎯️targets/<t>]`,
 * `PolicyCrateRef.shape === "taxonomy"`), master ticket
 * `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`. Their vocabulary is no longer
 * duplicated here: every directory name, leaf filename and line budget below is read from the shared
 * `🔣️taxonomy.json` via `loadTaxonomy()` (mechanism step M1), so a vocabulary edit can never leave the
 * registry validator and this policy disagreeing again.
 *
 * 🚦️Severity: WARN-only (never `"high"`) until the finalization flip promotes them. Findings on
 * framework/hub/s-module owners — surface these rules could not see at all before mechanism step M4
 * removed the Shape-V1-only, plugins-only discoverer — land one notch lower still (`"low"`, via
 * `policyNewSurfacePriority`), because nothing has ever triaged them.
 */




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
    if (crate.shape !== "taxonomy") continue;
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
 * 📏️Window completeness: every app window declares every required facet explicitly. Empty facets
 * carry only their tracked marker; components are forbidden at facet level and required on every
 * specific item. Shape, marker, and implementation languages come from shared vocabulary.
 */
export function policyWindowCompletenessBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const emptyMarker = canonicalFilenameForKind(taxonomy.windowEmptyFacetFileKindId, taxonomy);
  const componentFilenames = new Set(Object.values(taxonomy.componentFileKinds).map((fileKindId) => canonicalPrimaryFilenameForKind(fileKindId, taxonomy)));
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    if (crate.shape !== "taxonomy") continue;
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
 * 📏️Mode completeness: the twin of {@link policyWindowCompletenessBreaches} one level up. A mode is a
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
    if (crate.shape !== "taxonomy") continue;
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

/**
 * 📏️ Per-example unit shape: `📚️examples/<emoji-slug>/{definition leaves, 🖼️assets/, 🧪️tests/}` under
 * every artifact and every app (apps own examples directly — never under `⚙️engine`). Plugin-root
 * `📚️examples` and plural facet dirs are forbidden.
 */
function policyExampleSlugOk(slug: string, taxonomy: ReturnType<typeof loadTaxonomy>): boolean {
  try {
    return new RegExp(taxonomy.exampleSlugPattern, "u").test(slug);
  } catch {
    return false;
  }
}

/** 📚️ True when `relDir` is an immediate child of `📚️examples` (dynamic emoji-slug leaf parent). */
function policyIsExampleSlugDir(relDir: string): boolean {
  const parts = relDir.replaceAll("\\", "/").split("/");
  return parts.length >= 2 && parts[parts.length - 2] === "📚️examples";
}

function policyValidateExampleUnit(
  repoRoot: string,
  exampleRel: string,
  scope: string,
  priority: BreachRecord["priority"],
): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  const slug = exampleRel.split("/").pop() ?? "";
  if ((taxonomy.forbiddenExampleSlugs ?? []).includes(slug) || !policyExampleSlugOk(slug, taxonomy)) {
    breaches.push({
      id: `semio-examples-slug-${exampleRel}`,
      summary: `"${exampleRel}" is not a valid emoji+VS16+kebab example slug`,
      kind: "taxonomy/semio-examples",
      scope,
      priority,
      reason: `Example dirs must match exampleSlugPattern and must not be placeholders (${(taxonomy.forbiddenExampleSlugs ?? []).join(", ")}).`,
      solution: `Rename ${exampleRel} to an emoji+VS16+kebab slug that describes the scenario.`,
    });
  }
  for (const plural of taxonomy.forbiddenExamplePluralDirs ?? []) {
    if (existsSync(join(repoRoot, exampleRel, plural))) {
      breaches.push({
        id: `semio-examples-plural-${exampleRel}-${plural}`,
        summary: `"${exampleRel}/${plural}" plural example facet dir is forbidden`,
        kind: "taxonomy/semio-examples",
        scope,
        priority,
        reason: "Example assets live flat under 🖼️assets/ with kind-emoji prefixes — plural dsls/packs/ops/sprs dirs are gone.",
        solution: `Move files from ${exampleRel}/${plural}/ into ${exampleRel}/${taxonomy.exampleAssetsDirName}/ and delete the plural dir.`,
      });
    }
  }
  const rustLeaf = taxonomyMappedFilename(taxonomy, taxonomy.exampleFileKinds, "🦀️rust");
  const tsLeaf = taxonomyMappedFilename(taxonomy, taxonomy.exampleFileKinds, "🟦️typescript");
  const rustTest = taxonomyMappedFilename(taxonomy, taxonomy.exampleTestFileKinds, "🦀️rust");
  const tsTest = taxonomyMappedFilename(taxonomy, taxonomy.exampleTestFileKinds, "🟦️typescript");
  if (!existsSync(join(repoRoot, exampleRel, rustLeaf))) {
    breaches.push({
      id: `semio-examples-leaf-rs-${exampleRel}`,
      summary: `"${exampleRel}" is missing definition leaf ${rustLeaf}`,
      kind: "taxonomy/semio-examples",
      scope,
      priority,
      reason: "Every example unit needs a Rust definition leaf as the single source of truth for id/labels/assets.",
      solution: `Add ${exampleRel}/${rustLeaf}.`,
    });
  }
  if (!existsSync(join(repoRoot, exampleRel, tsLeaf))) {
    breaches.push({
      id: `semio-examples-leaf-ts-${exampleRel}`,
      summary: `"${exampleRel}" is missing definition leaf ${tsLeaf}`,
      kind: "taxonomy/semio-examples",
      scope,
      priority,
      reason: "Every example unit needs a TypeScript definition leaf for TS consumers.",
      solution: `Add ${exampleRel}/${tsLeaf}.`,
    });
  }
  const assetsRel = `${exampleRel}/${taxonomy.exampleAssetsDirName}`;
  if (!existsSync(join(repoRoot, assetsRel))) {
    breaches.push({
      id: `semio-examples-assets-${exampleRel}`,
      summary: `"${exampleRel}" is missing ${taxonomy.exampleAssetsDirName}/`,
      kind: "taxonomy/semio-examples",
      scope,
      priority,
      reason: "Every example unit carries its assets under 🖼️assets/.",
      solution: `Add ${assetsRel}/ with kind-prefixed asset files.`,
    });
  } else {
    for (const entry of policyReaddirSafe(repoRoot, assetsRel)) {
      if (entry.isDirectory) continue;
      const assetAbs = join(repoRoot, assetsRel, entry.name);
      if (statSync(assetAbs).size === 0) {
        breaches.push({
          id: `semio-examples-zero-byte-asset-${exampleRel}-${entry.name}`,
          summary: `"${assetsRel}/${entry.name}" is a 0-byte placeholder asset`,
          kind: "taxonomy/semio-examples",
          scope,
          priority,
          reason: "Example assets must be real, builder-emitted (or genuinely sourced) content — a 0-byte file proves nothing and can't be decoded by any analyzer test.",
          solution: `Replace ${assetsRel}/${entry.name} with a small file the artifact's own builder/analyzer can actually round-trip, or a real fixture.`,
        });
      }
    }
  }
  const testsRel = `${exampleRel}/${taxonomy.exampleTestsDirName}`;
  if (!existsSync(join(repoRoot, testsRel))) {
    breaches.push({
      id: `semio-examples-tests-${exampleRel}`,
      summary: `"${exampleRel}" is missing ${taxonomy.exampleTestsDirName}/`,
      kind: "taxonomy/semio-examples",
      scope,
      priority,
      reason: "Every example unit carries co-located tests under 🧪️tests/.",
      solution: `Add ${testsRel}/ with ${rustTest} and ${tsTest}.`,
    });
  } else {
    if (!existsSync(join(repoRoot, testsRel, rustTest))) {
      breaches.push({
        id: `semio-examples-test-rs-${exampleRel}`,
        summary: `"${testsRel}" is missing ${rustTest}`,
        kind: "taxonomy/semio-examples",
        scope,
        priority,
        reason: "Example tests must include the Rust test leaf.",
        solution: `Add ${testsRel}/${rustTest}.`,
      });
    }
    if (!existsSync(join(repoRoot, testsRel, tsTest))) {
      breaches.push({
        id: `semio-examples-test-ts-${exampleRel}`,
        summary: `"${testsRel}" is missing ${tsTest}`,
        kind: "taxonomy/semio-examples",
        scope,
        priority,
        reason: "Example tests must include the TypeScript test leaf.",
        solution: `Add ${testsRel}/${tsTest}.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️ Semio example layout: ≥1 emoji-slug under each artifact and each app `📚️examples/`
 * (not under `⚙️engine`); slug matches pattern; definition leaf + 🖼️assets/ + 🧪️tests/; no plural
 * dirs; no plugin-root `📚️examples`.
 */
export function policySemioArtifactExamplesBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const examplesDir = "📚️examples";
  const rustExampleLeaf = taxonomyMappedFilename(taxonomy, taxonomy.exampleFileKinds, "🦀️rust");
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    if (crate.shape !== "taxonomy") continue;
    const ownerRoot = crate.ownerRel;
    const scopeId = crate.pluginId || policyStripEmoji(ownerRoot.split("/").pop() ?? "");
    const priority = policyNewSurfacePriority(crate, "medium");
    if (existsSync(join(repoRoot, ownerRoot, examplesDir))) {
      breaches.push({
        id: `semio-examples-plugin-root-${ownerRoot}`,
        summary: `"${ownerRoot}/${examplesDir}" must not exist at plugin root`,
        kind: "taxonomy/semio-examples",
        scope: scopeId,
        priority: policyNewSurfacePriority(crate, "high"),
        reason: "Examples belong under 🗿️artifacts/<artifact>/📚️examples or a subset's 👁️viewer|✏️editor/📚️examples — never at the plugin root.",
        solution: `Move fixtures into artifact or app ${examplesDir}/<emoji-slug>/ units.`,
      });
    }
    const artifactsRoot = `${ownerRoot}/${taxonomy.artifactsDirName}`;
    for (const artifact of policyReaddirSafe(repoRoot, artifactsRoot).filter((e) => e.isDirectory)) {
      const artifactRel = `${artifactsRoot}/${artifact.name}`;
      const examplesRel = `${artifactRel}/${examplesDir}`;
      const artScope = `${scopeId}/${policyStripEmoji(artifact.name)}`;
      if (!existsSync(join(repoRoot, examplesRel))) {
        breaches.push({
          id: `semio-examples-missing-${examplesRel}`,
          summary: `"${examplesRel}" is missing`,
          kind: "taxonomy/semio-examples",
          scope: artScope,
          priority,
          reason: "Every artifact must ship at least one emoji-slug example unit.",
          solution: `Add ${examplesRel}/<emoji-slug>/{${rustExampleLeaf}, ${taxonomy.exampleAssetsDirName}/, ${taxonomy.exampleTestsDirName}/}.`,
        });
        continue;
      }
      const sets = policyReaddirSafe(repoRoot, examplesRel).filter((e) => e.isDirectory);
      if (sets.length === 0) {
        breaches.push({
          id: `semio-examples-empty-${examplesRel}`,
          summary: `"${examplesRel}" has no example slug directory`,
          kind: "taxonomy/semio-examples",
          scope: artScope,
          priority,
          reason: "Artifact examples must contain at least one emoji-slug example unit.",
          solution: `Add ${examplesRel}/<emoji-slug>/ with definition leaves, assets, and tests.`,
        });
      }
      for (const set of sets) {
        breaches.push(...policyValidateExampleUnit(repoRoot, `${examplesRel}/${set.name}`, artScope, priority));
      }
    }
    for (const surfaceRoot of policySurfaceRoots(repoRoot, ownerRoot, taxonomy)) {
      // 👁️✏️ Unlike the old 🎛️apps facet, 📚️examples is NOT in surfaceRequiredChildDirs (contract
      // §7.5) — a surface with no examples is legal, so this only validates units that DO exist.
      const [subsetName, roleDirName] = surfaceRoot.split("/").slice(-2);
      const surfaceScope = `${scopeId}/${policyStripEmoji(subsetName ?? "")}/${policyStripEmoji(roleDirName ?? "")}`;
      const surfaceExamples = `${surfaceRoot}/${examplesDir}`;
      if (!existsSync(join(repoRoot, surfaceExamples))) continue;
      const sets = policyReaddirSafe(repoRoot, surfaceExamples).filter((e) => e.isDirectory);
      if (sets.length === 0) {
        breaches.push({
          id: `semio-examples-surface-empty-${surfaceExamples}`,
          summary: `"${surfaceExamples}" has no example slug directory`,
          kind: "taxonomy/semio-examples",
          scope: surfaceScope,
          priority,
          reason: "A declared surface 📚️examples/ must contain at least one emoji-slug example unit.",
          solution: `Add ${surfaceExamples}/<emoji-slug>/ with definition leaves, assets, and tests, or remove the empty ${examplesDir}/ dir.`,
        });
      }
      for (const set of sets) {
        breaches.push(...policyValidateExampleUnit(repoRoot, `${surfaceExamples}/${set.name}`, surfaceScope, priority));
      }
    }
  }
  return breaches;
}

export function policyComponentFileBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const rustFileKindId = taxonomy.componentFileKinds["🦀️rust"]!;
  const typescriptFileKindId = taxonomy.componentFileKinds["🟦️typescript"]!;
  const leafFilename = canonicalPrimaryFilenameForKind(rustFileKindId, taxonomy);
  const sourceExtensions = taxonomy.fileKinds[rustFileKindId]!.extensionChains;
  const sourceExtension = sourceExtensions[0]!;
  const tsLeafFilename = canonicalPrimaryFilenameForKind(typescriptFileKindId, taxonomy);
  const tsSourceExtensions = taxonomy.fileKinds[typescriptFileKindId]!.extensionChains;
  const tsSourceExtension = tsSourceExtensions[0]!;
  const artifactFacetDirs = new Set(taxonomy.artifactComponentDirs ?? []);
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    if (crate.shape !== "taxonomy") continue;
    const ownerRoot = crate.ownerRel;

    const walk = (relDir: string): void => {
      const entries = policyReaddirSafe(repoRoot, relDir);
      const parentName = relDir.split("/").pop() ?? "";
      const hasRustFile = entries.some((e) => !e.isDirectory && sourceExtensions.some((extension) => e.name.endsWith(extension)));
      const hasTsFile = entries.some((e) => !e.isDirectory && tsSourceExtensions.some((extension) => e.name.endsWith(extension)));
      const exampleRustLeaf = taxonomyMappedFilename(taxonomy, taxonomy.exampleFileKinds, "🦀️rust");
      const exampleTsLeaf = taxonomyMappedFilename(taxonomy, taxonomy.exampleFileKinds, "🟦️typescript");
      if (policyIsExampleSlugDir(relDir)) {
        if (hasRustFile && !entries.some((e) => !e.isDirectory && e.name === exampleRustLeaf)) {
          breaches.push({
            id: `component-file-example-${relDir}`,
            summary: `"${relDir}" has ${sourceExtension} source but no ${exampleRustLeaf}`,
            kind: "taxonomy/component-file",
            scope: crate.pluginId || crate.ownerRel,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Example slug dirs (grandparent 📚️examples) use ${exampleRustLeaf} as the definition leaf.`,
            solution: `Rename ${relDir}'s primary source file to ${exampleRustLeaf}.`,
          });
        }
        if (hasTsFile && !entries.some((e) => !e.isDirectory && e.name === exampleTsLeaf)) {
          breaches.push({
            id: `component-file-example-ts-${relDir}`,
            summary: `"${relDir}" has ${tsSourceExtension} source but no ${exampleTsLeaf}`,
            kind: "taxonomy/component-file",
            scope: crate.pluginId || crate.ownerRel,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Example slug dirs use ${exampleTsLeaf} as the TypeScript definition leaf.`,
            solution: `Rename ${relDir}'s primary TypeScript file to ${exampleTsLeaf}.`,
          });
        }
      } else if (taxonomy.taxonomyLeafParentDirs.includes(parentName)) {
        // relDir itself is a vocabulary dir (e.g. 🔧️op/) whose immediate .rs files are the leaf — or,
        // for 🎮️commands/🛠️tools/📌️panels, an extra <name>/ nesting level holds the leaf instead.
        if (hasRustFile && !entries.some((e) => !e.isDirectory && e.name === leafFilename)) {
          breaches.push({
            id: `component-file-${relDir}`,
            summary: `"${relDir}" has ${sourceExtension} source but no ${leafFilename}`,
            kind: "taxonomy/component-file",
            scope: crate.pluginId || crate.ownerRel,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Discovery contract: every taxonomy leaf's primary file must be literally named ${leafFilename}.`,
            solution: `Rename ${relDir}'s primary source file to ${leafFilename} (sibling 🦀️<topic>${sourceExtension} files may stay).`,
          });
        }
        if (artifactFacetDirs.has(parentName) && hasTsFile && !entries.some((e) => !e.isDirectory && e.name === tsLeafFilename)) {
          breaches.push({
            id: `component-file-ts-${relDir}`,
            summary: `"${relDir}" has ${tsSourceExtension} source but no ${tsLeafFilename}`,
            kind: "taxonomy/component-file",
            scope: crate.pluginId || crate.ownerRel,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Discovery contract: artifact facet dirs carry the TypeScript taxonomy leaf (${tsLeafFilename}) alongside ${leafFilename} and any mapped artifactSpecFileKinds entry.`,
            solution: `Rename ${relDir}'s primary TypeScript facade to ${tsLeafFilename} (normative 📖️/📡️ *.semio specs mapped in artifactSpecFileKinds may stay as siblings).`,
          });
        }
      }
      for (const entry of entries.filter((e) => e.isDirectory)) walk(`${relDir}/${entry.name}`);
    };
    // 👁️✏️ Surfaces now nest under 🗿️artifacts/*/🏅️standards/*/🪆️subsets/*/{👁️viewer,✏️editor}/, which
    // this recursive walk already reaches — no separate root needed once 🎛️apps is gone.
    walk(`${ownerRoot}/${taxonomy.artifactsDirName}`);
  }
  return breaches;
}

/**
 * 📏️Anti-inlining tripwire (Single-File-Repo hazard ruling, master ticket): a migrated package's entry
 * configured Rust package entry must stay wiring-only (`#[path]` mod declarations + `plugin_exports!(plugin::plugin)`) — no
 * non-trivial `fn`/`impl` body content beyond `taxonomy.libWiringLineBudget`. Catches the exact
 * regression this repo has hit twice before: an agent following the (now-scoped) "single file repo" goal
 * inlining split `#[path]` modules back into the package entry. Plugin identity lives at the plugin root via
 * `Plugin::builder`, never `semio_plugin!{}`.
 */
const POLICY_FN_OR_IMPL_OPEN_RE = /^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:fn\s+\w+\s*(?:<[^>]*>)?\s*\([^;{]*\)[^;{]*|impl(?:<[^>]*>)?\s+[^;{]+)\{\s*$/;

export function policyTaxonomyLibShapeBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const lineBudget = loadTaxonomy().libWiringLineBudget;
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    if (crate.shape !== "taxonomy") continue;
    const abs = join(repoRoot, crate.libRelPath);
    if (!existsSync(abs)) continue;
    const content = readFileSync(abs, "utf8");
    const lines = content.split(/\r?\n/);
    const lineStarts: number[] = [];
    for (let offset = 0, li = 0; li < lines.length; li++) {
      lineStarts.push(offset);
      offset += lines[li]!.length + 1;
    }

    let bodyLines = 0;
    let coveredUntilLine = 0; // 🌱️ skip fn/impl bodies nested inside an already-counted outer body — avoids double-counting a fn declared inside an impl block.
    lines.forEach((line, i) => {
      const lineNo = i + 1;
      if (lineNo <= coveredUntilLine || !POLICY_FN_OR_IMPL_OPEN_RE.test(line)) return;
      const body = policyExtractFnBody(content, lineStarts[i]!);
      const bodyLineList = body.split(/\r?\n/);
      bodyLines += bodyLineList.filter((l) => l.trim() && !l.trim().startsWith("//")).length;
      coveredUntilLine = lineNo + bodyLineList.length - 1;
    });
    if (bodyLines <= lineBudget) continue;
    breaches.push({
      id: `taxonomy-lib-shape-${crate.libRelPath}`,
      summary: `"${crate.libRelPath}" has ~${bodyLines} lines of fn/impl body content — a migrated package entry must stay wiring-only`,
      kind: "taxonomy/lib-shape",
      scope: crate.pluginId || crate.ownerRel,
      priority: policyNewSurfacePriority(crate, "medium"),
      reason: "Single-File-Repo hazard ruling: a taxonomy package entry is #[path] mod wiring + plugin_exports!(plugin::plugin) only — real logic lives in taxonomy component files at the plugin root, never inlined back.",
      solution: `Move the non-trivial fn/impl bodies out of ${crate.libRelPath} into their owning taxonomy component file(s) at the plugin root; the package entry should only declare "#[path = \"...\"] mod ...;" and call plugin_exports!(plugin::plugin).`,
    });
  }
  return breaches;
}

/**
 * 📏️TS analogue of `policyTaxonomyLibShapeBreaches`, added by ticket
 * `26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE` (Single-File-Repo hazard ruling, scope note 2): a
 * `"taxonomy"`-area TypeScript package's configured entry file must stay a wiring-only re-export barrel — counts non-import/export/comment/blank lines
 * and flags a breach past `libWiringLineBudget`. Warn-only (`priority: "medium"`) and still vacuous
 * today: the graduated area state is `"clean"` and no area has graduated yet (that flip is the W6
 * activation step of that ticket). The literal it compares against used to be `"taxonomy"`, a value that
 * is not in `taxonomy.areaStates` at all and therefore could never match — fixed here to `"clean"` while
 * mechanism step M4 was in this region (see `🔣️taxonomy.json`'s `_areaStateComment`).
 */
const POLICY_BARREL_GRADUATED_AREA_STATE = "clean";
const POLICY_BARREL_WIRING_LINE_RE = /^\s*(\/\/.*)?$|^\s*export\s+(type\s+)?(\*|\{[^}]*\})\s+from\s+"[^"]+";?\s*(\/\/.*)?$|^\s*import\s+.+\s+from\s+"[^"]+";?\s*(\/\/.*)?$/;

export function policyTaxonomyBarrelShapeBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const entryFilename = taxonomyEcosystemEntryFilenames(taxonomy, "🟦️typescript")[0];
  if (!entryFilename) return [];
  const breaches: BreachRecord[] = [];
  for (const pkg of discoverPackages(repoRoot, taxonomy)) {
    if (pkg.lang !== "🟦️typescript" || pkg.area !== POLICY_BARREL_GRADUATED_AREA_STATE) continue;
    const entryAbs = join(repoRoot, pkg.packageRel, entryFilename);
    if (!existsSync(entryAbs)) continue;
    const lines = readFileSync(entryAbs, "utf8").split(/\r?\n/);
    const nonWiring = lines.filter((l) => l.trim() && !POLICY_BARREL_WIRING_LINE_RE.test(l));
    if (nonWiring.length <= taxonomy.libWiringLineBudget) continue;
    breaches.push({
      id: `taxonomy-barrel-shape-${pkg.manifestPath}`,
      summary: `"${pkg.packageRel}/${entryFilename}" has ~${nonWiring.length} non-wiring lines — a taxonomy package's entry file must stay a re-export barrel`,
      kind: "taxonomy/barrel-shape",
      scope: pkg.id,
      priority: "medium",
      reason: "Single-File-Repo hazard ruling (TS extension, ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE): a taxonomy package's entry barrel is import/export wiring only — real component logic lives in 🧱️elements/<Element>/ leaf files, never inlined back.",
      solution: `Move the non-wiring content out of ${entryFilename} into its owning 🧱️elements/<Element>/ leaf file(s); the entry file should only import/re-export.`,
    });
  }
  return breaches;
}

const POLICY_PROTOCOL_SEGMENT_RE = /^protocol$/i;

const POLICY_BANNED_STEM_SCAN_ROOTS = ["✏️s", "🧰️framework", "🌎️hub", "♻️mit-bestand"] as const;
const POLICY_VS16 = "\uFE0F";

/**
 * 🚫️ Bans emoji-stripped directory/file stems listed in `taxonomy.bannedNameStems` (e.g. `core`, `shared`).
 * Medium during migration; Wave 4 flips to high once cores are dissolved.
 */
export function policyBannedNameStemBreaches(repoRoot: string): BreachRecord[] {
  const stems = new Set((loadTaxonomy().bannedNameStems ?? []).map((s) => s.toLowerCase()));
  if (stems.size === 0) return [];
  const breaches: BreachRecord[] = [];
  const walk = (relDir: string): void => {
    for (const entry of policyReaddirSafe(repoRoot, relDir)) {
      if (entry.name === "node_modules" || entry.name === "target" || entry.name === "dist" || entry.name === "pkg") continue;
      const childRel = `${relDir}/${entry.name}`;
      const stem = policyStripEmoji(entry.name.replace(/\.(rs|ts|tsx|js|mjs|cjs|go|py|cs)$/, "")).toLowerCase();
      if (stems.has(stem)) {
        breaches.push({
          id: `banned-name-stem-${childRel}`,
          summary: `"${childRel}" uses banned name stem "${stem}" — use a domain concept folder instead`,
          kind: "taxonomy/banned-name-stem",
          scope: childRel,
          priority: "high",
          reason: "Clean mechanism: bannedNameStems forbids vague grab-bag folders (core/shared/util/…).",
          solution: `Rename "${entry.name}" to a domain-specific concept folder and update #[path]/imports.`,
        });
      }
      if (entry.isDirectory) walk(childRel);
    }
  };
  for (const root of POLICY_BANNED_STEM_SCAN_ROOTS) walk(root);
  return breaches;
}

/** 🧷️Whether an exact filename contract applies at this precise scope. */
function policyFilenameContractApplies(repoRoot: string, relDir: string, name: string, taxonomy: ReturnType<typeof loadTaxonomy>, resolver?: ReturnType<typeof createFixedContractResolver>): boolean {
  const segments = relDir.split("/").filter(Boolean);
  let packagesIndex = -1;
  for (let index = segments.length - 1; index >= 0; index--) {
    if (semanticDirectoryKindId(segments[index]!, taxonomy) === "packages") {
      packagesIndex = index;
      break;
    }
  }
  const adjacentEcosystem = ([
    ["Cargo.toml", "🦀️rust"],
    ["go.mod", "🐹️go"],
    ["package.json", "🟦️typescript"],
  ] as const).find(([manifest]) => name === manifest || existsSync(join(repoRoot, relDir, manifest)))?.[1];
  const ecosystemId = packagesIndex >= 0 ? segments[packagesIndex + 1] : adjacentEcosystem;
  const packageRoot = packagesIndex === segments.length - 2 || adjacentEcosystem !== undefined;
  const siblingFixedFilenameContractIds = existsSync(join(repoRoot, relDir, "📋️project.json")) ? ["build-project-manifest"] : undefined;
  for (const contract of Object.values(taxonomy.configurableEntryContracts)) {
    if (contract.filename === name && contract.ecosystemId === ecosystemId && packageRoot) return true;
  }
  const path = relDir ? `${relDir}/${name}` : name;
  return (resolver?.filenameIdsForPath(path, {
    packageRoot,
    ecosystemId,
    parentDirectoryKindId: semanticDirectoryKindId(segments.at(-1) ?? "", taxonomy) ?? undefined,
    siblingFixedFilenameContractIds,
  }) ?? fixedFilenameContractIdsForPath(path, taxonomy, {
    packageRoot,
    ecosystemId,
    parentDirectoryKindId: semanticDirectoryKindId(segments.at(-1) ?? "", taxonomy) ?? undefined,
    siblingFixedFilenameContractIds,
  })).length > 0;
}

/** 🏭️Generated and framework-owned trees whose entry names are not repository taxonomy. */
function policyEmojiEntryIsGeneratedOrFrameworkOwned(relDir: string, name: string): boolean {
  return name.startsWith(".") || name === "pkg" || name === "coverage" || name === "partial_movie_files" || name === "__pycache__";
}

/** 🪪️Whether an entry is free to adopt the repository's emoji-prefixed identity. */
function policyEmojiEntryIsRenamable(repoRoot: string, relDir: string, name: string, isDirectory: boolean, taxonomy: ReturnType<typeof loadTaxonomy>): boolean {
  if (!name || name.startsWith(".") || policyEmojiEntryIsGeneratedOrFrameworkOwned(relDir, name)) return false;
  if (isDirectory) return true;
  return !policyFilenameContractApplies(repoRoot, relDir, name, taxonomy);
}

/** ✨️Whether a name starts with an actual emoji sequence rather than arbitrary non-ASCII text. */
function policyHasLeadingEmoji(name: string): boolean {
  const prefix = policyLeadingEmojiPrefix(name);
  return prefix.length > 0 && /[^\x00-\x7f]/u.test(prefix);
}

/** 🎨️Whether a Latin-stemmed emoji identity is missing the required emoji presentation selector. */
function policyEmojiPrefixNeedsVs16(name: string): boolean {
  const prefix = policyLeadingEmojiPrefix(name);
  const first = [...prefix][0] ?? "";
  return policyHasLeadingEmoji(name) && /^[A-Za-z]/.test(name.slice(prefix.length)) && !/\p{Emoji_Presentation}/u.test(first) && !prefix.includes(POLICY_VS16);
}

/** ✅️Every Git-visible, non-reserved entry has a semantic canonical emoji identity unique among all siblings. */
export function policyEmojiPrefixBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const result = Bun.spawnSync(["git", "ls-files", "-co", "--exclude-standard", "-z"], { cwd: repoRoot });
  if (result.exitCode !== 0) throw new Error(`git-visible path inventory failed: ${result.stderr.toString()}`);
  const inventory = result.stdout.toString().split("\0").filter(Boolean)
    .map((path) => path.replaceAll("\\", "/").replace(/\/$/u, "").normalize("NFC"))
    .flatMap((path) => {
      try { return [{ path, directory: lstatSync(join(repoRoot, path)).isDirectory() }]; } catch { return []; }
    });
  const files = inventory.filter((entry) => !entry.directory).map((entry) => entry.path);
  const directories = new Set<string>();
  for (const entry of inventory) {
    if (entry.directory) directories.add(entry.path);
    for (let parent = posix.dirname(entry.path); parent !== "." && parent !== ""; parent = posix.dirname(parent)) directories.add(parent);
  }
  const reservedSubtrees = new Set(taxonomy.pathEmojiPolicy.reservedSubtreeDirectoryNames.map((name) => name.normalize("NFC").replaceAll(POLICY_VS16, "")));
  const isReservedSubtreeName = (name: string): boolean => reservedSubtrees.has(name.replaceAll(POLICY_VS16, "")) || reservedSubtrees.has(policyStripEmoji(name));
  const resolver = createFixedContractResolver(taxonomy);
  const fixedDirectoryIds = new Map([...directories].map((path) => [path, resolver.directoryIdsForPath(path, {
    parentDirectoryKindId: semanticDirectoryKindId(posix.basename(posix.dirname(path)), taxonomy) ?? undefined,
  })] as const));
  const fixedReservedSubtreeRoots = [...fixedDirectoryIds].filter(([, ids]) => ids.some((id) => taxonomy.fixedDirectoryContracts[id]?.descendants === "reserved")).map(([path]) => path);
  const underReservedSubtree = (path: string): boolean => path.split("/").slice(0, -1).some(isReservedSubtreeName) || fixedReservedSubtreeRoots.some((reservedRoot) => path.startsWith(`${reservedRoot}/`));
  const directoryReserved = (path: string): boolean => (fixedDirectoryIds.get(path) ?? []).length > 0;
  const entries: PathEmojiEntry[] = [
    ...[...directories].map((path): PathEmojiEntry => {
      const relDir = posix.dirname(path) === "." ? "" : posix.dirname(path);
      const name = posix.basename(path);
      return { path, nodeKind: "directory", reserved: underReservedSubtree(path) || isReservedSubtreeName(name) || directoryReserved(path) || !policyEmojiEntryIsRenamable(repoRoot, relDir, name, true, taxonomy) };
    }),
    ...files.map((path): PathEmojiEntry => {
      const relDir = posix.dirname(path) === "." ? "" : posix.dirname(path);
      const name = posix.basename(path);
      return { path, nodeKind: "file", reserved: underReservedSubtree(path) || !policyEmojiEntryIsRenamable(repoRoot, relDir, name, false, taxonomy) || policyFilenameContractApplies(repoRoot, relDir, name, taxonomy, resolver) };
    }),
  ].filter((entry) => !taxonomyRelativePathIsExcluded(entry.path, taxonomy));
  const byPath = new Map(entries.map((entry) => [entry.path, entry]));
  return pathEmojiStatuteFindings(entries, taxonomy.pathEmojiPolicy.genericEmojiIdentities).map((finding): BreachRecord => {
    const entity = byPath.get(finding.path)?.nodeKind === "directory" ? "folder" : "file";
    const label = { missing: "missing-emoji", generic: "generic-emoji", presentation: "noncanonical-emoji-presentation", spacing: "whitespace-after-emoji", duplicate: "emoji-not-unique", multiple: "multiple-emojis", "reserved-emoji": "reserved-name-has-emoji" }[finding.kind];
    const detail = finding.kind === "duplicate" ? ` duplicates sibling "${finding.sibling}"` : finding.emoji ? ` uses "${finding.emoji}"` : "";
    return {
      id: `path-emoji-${finding.kind}-${finding.path}`,
      summary: `"${finding.path}" breaches ${label}${detail}`,
      kind: `${entity}/name/${label}`,
      scope: finding.path,
      priority: "high",
      reason: "Every non-reserved file and folder has exactly one handpicked, meaningful emoji, unique among siblings. Reserved filenames remain literal and unprefixed.",
      solution: finding.kind === "reserved-emoji" ? `Restore the literal reserved basename of "${posix.basename(finding.path)}" and update its exact references.` : `Handpick one meaningful sibling-unique emoji for "${posix.basename(finding.path)}" and update only resolved path references.`,
    };
  });
}

/**
 * 🔌️ Every plugin owner under `✏️s/🔌️plugins/` must carry its contract leaf and every REQUIRED facet directly
 * at its root. `pluginChildDirs` is the ADMITTED set (`policyPluginClosedShapeBreaches` reads it, and it now
 * admits the plugin-root `🧬️schema` scope); `pluginRequiredChildDirs` is the set every plugin owes.
 */
export function policyPluginRootShapeBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const children = taxonomy.pluginRequiredChildDirs;
  const pluginsRoot = "✏️s/🔌️plugins";
  const breaches: BreachRecord[] = [];
  for (const entry of policyReaddirSafe(repoRoot, pluginsRoot)) {
    if (!entry.isDirectory) continue;
    const ownerRel = `${pluginsRoot}/${entry.name}`;
    const nestedContractRel = `${ownerRel}/🔌️plugin`;
    if (existsSync(join(repoRoot, nestedContractRel))) {
      breaches.push({
        id: `plugin-root-nested-${ownerRel}`,
        summary: `"${ownerRel}" contains a redundant 🔌️plugin/ directory`,
        kind: "taxonomy/plugin-root-shape",
        scope: ownerRel,
        priority: "high",
        reason: "Plugin contracts and facets are direct children of the plugin root.",
        solution: `Move ${nestedContractRel}/${POLICY_RS_COMPONENT_LEAF_NAME} and its facet leaves directly into ${ownerRel}/, then remove ${nestedContractRel}/.`,
      });
    }
    if (!existsSync(join(repoRoot, ownerRel, (POLICY_RS_COMPONENT_LEAF_NAME)))) {
      breaches.push({
        id: `plugin-root-leaf-${ownerRel}`,
        summary: `"${ownerRel}" is missing root ${POLICY_RS_COMPONENT_LEAF_NAME}`,
        kind: "taxonomy/plugin-root-shape",
        scope: ownerRel,
        priority: "high",
        reason: "The plugin root must have a leaf component that returns Plugin via Plugin::builder.",
        solution: `Add ${ownerRel}/${POLICY_RS_COMPONENT_LEAF_NAME} exporting pub fn plugin() -> Plugin.`,
      });
    }
    for (const child of children) {
      const childLeaf = join(repoRoot, ownerRel, child, (POLICY_RS_COMPONENT_LEAF_NAME));
      if (!existsSync(childLeaf)) {
        breaches.push({
          id: `plugin-root-child-${ownerRel}/${child}`,
          summary: `"${ownerRel}" is missing direct facet ${child}/${POLICY_RS_COMPONENT_LEAF_NAME}`,
          kind: "taxonomy/plugin-root-shape",
          scope: ownerRel,
          priority: "high",
          reason: `${child} is a required direct plugin-root facet (taxonomy.pluginRequiredChildDirs).`,
          solution: `Add ${ownerRel}/${child}/${POLICY_RS_COMPONENT_LEAF_NAME}.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 🏗️ Bans `semio_plugin!` and `PluginBundle::new` outside the SDK; prefers `Plugin::builder(`.
 * Medium until Wave 3 finishes migrating all plugins.
 */
export function policyPluginBuilderBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const sdkRel = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/";
  for (const crate of crates) {
    if (crate.role !== "plugin" && crate.role !== "extension") continue;
    const abs = join(repoRoot, crate.libRelPath);
    if (!existsSync(abs)) continue;
    if (crate.libRelPath.replaceAll("\\", "/").startsWith(sdkRel)) continue;
    const content = readFileSync(abs, "utf8");
    if (content.includes("semio_plugin!")) {
      breaches.push({
        id: `plugin-builder-macro-${crate.libRelPath}`,
        summary: `"${crate.libRelPath}" still uses semio_plugin! — migrate to Plugin::builder at the plugin root`,
        kind: "taxonomy/plugin-builder",
        scope: crate.pluginId || crate.ownerRel,
        priority: "high",
        reason: ("semio_plugin! is retired; plugin identity lives in the root " + POLICY_RS_COMPONENT_LEAF_NAME + " via typestate PluginBuilder."),
        solution: ("Move registration into the plugin-root " + POLICY_RS_COMPONENT_LEAF_NAME + " using Plugin::builder(...).build() and call plugin_exports!(plugin::plugin)."),
      });
    }
    if (content.includes("PluginBundle::new") || content.includes("PluginBundle {")) {
      breaches.push({
        id: `plugin-builder-bundle-${crate.libRelPath}`,
        summary: `"${crate.libRelPath}" still references PluginBundle — use Plugin::builder / Plugin::new`,
        kind: "taxonomy/plugin-builder",
        scope: crate.pluginId || crate.ownerRel,
        priority: "high",
        reason: "PluginBundle was renamed to Plugin; registration goes through Plugin::builder.",
        solution: "Replace PluginBundle with Plugin::builder(...).",
      });
    }
  }
  return breaches;
}

/**
 * 📏️Discovery-contract clause 4: no path segment (dir or file stem) or `mod`/`struct`/`enum`/`type`
 * identifier fragment named "protocol" may exist under a migrated plugin — `📡️spr` is the only accepted
 * name for that concept going forward. Scoped to `role = "plugin"` owners on purpose: the rename is a
 * plugin-tree contract today, while the framework's own `📡️protocol` kernel module is renamed by the
 * os-kernel merge (plan wave W8c) — flagging it here would report a rename that is scheduled, not late.
 */
export function policySprNamingBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    if (crate.shape !== "taxonomy" || crate.role !== "plugin") continue;
    const ownerRoot = crate.ownerRel;
    const pluginId = crate.pluginId;

    const walk = (relDir: string): void => {
      for (const entry of policyReaddirSafe(repoRoot, relDir)) {
        const childRel = `${relDir}/${entry.name}`;
        const stem = policyStripEmoji(entry.name.replace(/\.rs$/, ""));
        if (POLICY_PROTOCOL_SEGMENT_RE.test(stem)) {
          breaches.push({
            id: `spr-naming-path-${childRel}`,
            summary: `"${childRel}" uses the retired "protocol" name — the taxonomy concept is "spr" (state patch representation)`,
            kind: "taxonomy/spr-naming",
            scope: pluginId,
            priority: "medium",
            reason: "Discovery contract: no 📡️protocol path segment may remain under a migrated plugin; 📡️spr is the only accepted name.",
            solution: `Rename "${entry.name}" to its 📡️spr equivalent.`,
          });
        }
        if (entry.isDirectory) walk(childRel);
      }
    };
    walk(ownerRoot);

    const abs = join(repoRoot, crate.libRelPath);
    if (!existsSync(abs)) continue;
    const content = readFileSync(abs, "utf8");
    const identRe = /\b(?:mod|struct|enum|type)\s+(\w*[Pp]rotocol\w*)\b/g;
    let m: RegExpExecArray | null;
    while ((m = identRe.exec(content))) {
      breaches.push({
        id: `spr-naming-ident-${crate.libRelPath}-${m.index}`,
        summary: `"${crate.libRelPath}" declares "${m[1]}" — identifiers naming the protocol concept must say "Spr"/"spr" instead`,
        kind: "taxonomy/spr-naming",
        scope: pluginId,
        line: policyLineOfIndex(content, m.index),
        priority: "medium",
        reason: "Discovery contract: no protocol-named identifier fragment tied to the app protocol module may remain under a migrated plugin.",
        solution: `Rename "${m[1]}" to its 📡️spr equivalent.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleTaxonomy
