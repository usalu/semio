# Summary

SQLite schema of the repo client checkpoint export.

# Docs

## 🧬️schema

Scope `repo.client.sqlite` (`https://semio.tech/schema/repo/client/sqlite/schema.json`).

- `🧬️schema/🗄️.sql` — native SQLite implementation, seven tables.
- `🧬️schema/🔣️.json` — draft-07 contract, one `<Table>Row` export per table, each annotated
  `x-semio-persistence: local-only`.

Column names and nullability of both files are kept identical by
`📦️packages/🟦️typescript/🔬️schema.test.ts`.

The exported entity set is the one the client materializes in `ExportResult`
(`⌨️cli/📤️event_export.go`): technologies, bundles, folders, files, sections and definitions, rooted in
one repo checkpoint. Shared repo state lives in the repo server's PostgreSQL schema
(`🖥️server/🧬️schema/`), never here.

# 💯️Requirements

```mermaid
erDiagram
    repo ||--o{ folder : contains
    folder ||--o{ folder : nests
    folder ||--o{ technology : hosts
    folder ||--o{ bundle : hosts
    technology ||--o{ bundle : ships
    folder ||--o{ file : contains
    file ||--o{ section : contains
    section ||--o{ definition : contains
    repo {
        int id PK
        string name
        string summary
        string checkpoint
    }
    folder {
        int id PK
        int parent_folder_id FK
        int kind
        string name
        string summary
    }
    technology {
        int id PK
        int folder_id FK
        int kind
        string name
        string summary
    }
    bundle {
        int id PK
        int technology_id FK
        int folder_id FK
        int kind
        string name
        string summary
    }
    file {
        int id PK
        int parent_folder_id FK
        int kind
        string name
        string summary
    }
    section {
        int id PK
        int file_id FK
        string name
        string summary
    }
    definition {
        int id PK
        int section_id FK
        int kind
        string name
        string summary
        string code
    }
```
