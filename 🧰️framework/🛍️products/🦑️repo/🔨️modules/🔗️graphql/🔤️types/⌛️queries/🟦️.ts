/** ⌛️ Canonical repository GraphQL operation documents consumed by client type projection and independent validation. */

// #region ⌛️Queries
export const RepoStructureDocument = (`
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

export const RepoCheckpointsDocument = (`
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

export const FolderContentDocument = (`
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

export const BundlesDocument = (`
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

export const TicketsDocument = (`
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

export const PoliciesDocument = (`
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

export const ContributorsDocument = (`
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

export const FileContentDocument = (`
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

export const GoalsDocument = (`
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
