/** ⌛️ Repo GraphQL operations
 *
 * The GraphQL operations the extension sends, typed by the types generated from the served schema.
 */

// #region 🔌️Adapters
import type { AnalyzeQuery, AnalyzeQueryVariables, BundlesQuery, BundlesQueryVariables, ContributorsQuery, ContributorsQueryVariables, FileContentQuery, FileContentQueryVariables, FolderContentQuery, FolderContentQueryVariables, GoalsQuery, GoalsQueryVariables, PoliciesQuery, PoliciesQueryVariables, RepoCheckpointsQuery, RepoCheckpointsQueryVariables, RepoStructureQuery, RepoStructureQueryVariables, TicketsQuery, TicketsQueryVariables } from "../🤖️generated/🔗️graphql-types/🟦️.ts";
// #endregion 🔌️Adapters

/** 🕸️ One GraphQL operation source typed by the result and variables the generated types describe for it. */
export type TypedDocument<TResult, TVariables> = string & { readonly __result?: TResult; readonly __variables?: TVariables };

/** 🕸️ Turns one generated GraphQL operation source into the document the repo CLI accepts — its own source text, which is what `graphql --query` is given. */
export function graphql<TResult, TVariables>(source: string): TypedDocument<TResult, TVariables> {
  return source as TypedDocument<TResult, TVariables>;
}

// #region ⌛️Queries
export const RepoStructureDocument = graphql<RepoStructureQuery, RepoStructureQueryVariables>(`
  query RepoStructure {
    repo {
      id
      name
      path
      technologies {
        id
        name
        kind
        root
        bundles {
          id
          name
          kind
          root
          sourceRoot
          projectType
          tags
          uri
        }
      }
      bundles {
        id
        name
        kind
        root
        sourceRoot
        projectType
        tags
        uri
      }
    }
  }
`);

export const RepoCheckpointsDocument = graphql<RepoCheckpointsQuery, RepoCheckpointsQueryVariables>(`
  query RepoCheckpoints {
    repo {
      checkpoints(limit: 100) {
        id
        sha
        title
        date
      }
    }
  }
`);

export const FolderContentDocument = graphql<FolderContentQuery, FolderContentQueryVariables>(`
  query FolderContent($path: String!) {
    folder(path: $path) {
      children {
        path
        name
        uri
      }
      files {
        path
        name
        uri
      }
    }
  }
`);

export const BundlesDocument = graphql<BundlesQuery, BundlesQueryVariables>(`
  query Bundles {
    repo {
      bundles {
        id
        name
        root
        sourceRoot
        projectType
        tags
        uri
      }
    }
  }
`);

export const TicketsDocument = graphql<TicketsQuery, TicketsQueryVariables>(`
  query Tickets($year: Int, $month: Int, $day: Int, $status: TicketStatus) {
    repo {
      tickets(year: $year, month: $month, day: $day, status: $status) {
        id
        year
        month
        day
        slug
        path
        uri
        prompt
        summary
        status
        author {
          github
          name
        }
        llm
        checkpoint
        goal
        dates {
          started
          finished
        }
        interactions {
          prompt
          system
          client
          author
          date
          checkpoint
        }
      }
    }
  }
`);

export const PoliciesDocument = graphql<PoliciesQuery, PoliciesQueryVariables>(`
  query Policies {
    repo {
      policies {
        id
        name
        description
        scopes
        statutes {
          id
          priority
          autofixable
          reason
          solution
        }
      }
    }
  }
`);

export const ContributorsDocument = graphql<ContributorsQuery, ContributorsQueryVariables>(`
  query Contributors {
    repo {
      contributors {
        id
        github
        name
        emails
        links {
          name
          url
        }
        icons {
          avatar
          avatarRound
          github
        }
        contributions {
          checkpoints {
            id
            sha
            title
          }
          tickets {
            year
            months {
              month
              days {
                day
                tickets {
                  id
                  slug
                  title
                  status
                }
              }
            }
          }
          bundles {
            name
            folders {
              name
              files {
                name
                sections {
                  name
                  definitions {
                    name
                  }
                }
              }
            }
          }
        }
      }
    }
  }
`);

export const AnalyzeDocument = graphql<AnalyzeQuery, AnalyzeQueryVariables>(`
  query Analyze($scope: String) {
    analyze(scope: $scope) {
      breachs {
        id
        summary
        priority
        autofixable
        scope
        line
        column
        excerpt
        kind {
          id
          policy {
            id
            name
          }
          reason
          solution
        }
      }
      metrics {
        total
        byPriority {
          high
          medium
          low
        }
        autofixable
      }
    }
  }
`);

export const FileContentDocument = graphql<FileContentQuery, FileContentQueryVariables>(`
  query FileContent($path: String!) {
    file(path: $path) {
      path
      name
      uri
      sections {
        id
        name
        range {
          start
          end
        }
        parent {
          id
        }
        ... on Section {
          children {
            id
            name
            range {
              start
              end
            }
            ... on Section {
              children {
                id
                name
                range {
                  start
                  end
                }
                ... on Section {
                  children {
                    id
                    name
                    range {
                      start
                      end
                    }
                  }
                }
              }
            }
          }
        }
      }
      definitions {
        id
        name
        kind
        range {
          start
          end
        }
        section {
          id
          name
        }
      }
    }
  }
`);

export const GoalsDocument = graphql<GoalsQuery, GoalsQueryVariables>(`
  query Goals {
    repo {
      goals {
        id
        title
        description
        prompt
        status
        dueDate
        client
        llm
        milestone
      }
    }
  }
`);
// #endregion ⌛️Queries
