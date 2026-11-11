# Shared worktree fleet: operator runbook

One logical worktree is edited live by 50 or more machines (macOS, Windows, Linux) in about five groups of ten.
Every machine keeps a local replica on its own NVMe. A replica daemon copies every file that the root `.gitignore`
does not ignore to all other machines within about a second, through a journal (the **store**) on the NFS export
of the machine's group. Ignored files stay local. There are no branches, merges, pull requests or merge queue.

The design contract is `SHARED-WORKTREE-FLEET-OVER-NFS/design-contract.md` in the ticket folder of 2026-11-09.
This file is the operator view. The agent view is the section "Shared worktree fleet" of the
[repo README](../../README.md#shared-worktree-fleet).

**State: staged.** The modules were written on a host without Bun, Node, Python or git. The Linux side (group server,
worker, kernel NFS 4.2, systemd) ran on two real Ubuntu 24.04 arm64 machines, then two group servers and a macOS 27 client ran against each other; the figures marked "measured" below
come from `SHARED-WORKTREE-FLEET-OVER-NFS/linux-nfs-report.md` and `mixed-os-report.md` in the ticket folder, which also list what stayed unverified.
Nothing here has run on a real fleet of 50 machines. Values marked "recommendation" are sizing judgements, not measurements.
Settings are marked **measured** when a run on real hardware shows their effect (Linux with kernel NFS 4.2 and macOS 27 so far) and **by reasoning** when they follow from how the client works but no machine confirmed them (Windows store clients); the table "Client caches" says which is which.
Run day 0 first with one group of two machines before you roll it out to all five.

## Parts

| Directory | Role |
|---|---|
| `📒️journal` | Store format: events, hybrid logical clock, segments, blobs, snapshots. |
| `🚧️boundary` | What is shared: root `.gitignore` matcher, path policy, scanner, hashing. |
| `🔁️replica` | The commands `seed`, `join`, `run`, `status`, `snapshot`, `relay`, `compact`, `verify`, `bootstrap-archive`, `bootstrap-settle`. |
| `🔁️replica/🥾️bootstrap` | The tar writer, the archive refresh of the relay and the settle step of the bootstrap flow. |
| `🗺️topology` | The topology file and the commands `env`, `mount-plan`, `server-plan`, `worker-service`, `facts`, `validate` (`validate --reachable` also probes the hosts) and `preflight` (checks a provisioned machine). |
| `🗺️topology/🥾️provision` | `🐚️.sh` (macOS, Linux) and `🔵️.ps1` (Windows): provision one worker or one group server from an empty disk. |

## Commands

Every command runs as `repo ./📜️script.ts <command> [options]` in the directory that owns it: the replica
commands in `🔁️replica`, the topology commands in `🗺️topology` (both below this module). The current directory only
has to hold `📜️script.ts`: the replica script finds the repository root, and so the default worktree, from its own
location. The scripts have no `--help`; a wrong command or option is refused with a message that lists what is accepted. The
replica commands print their messages in English and German, `--lang en|de` picks one. Options are `--name value` or `--name=value`.

| Replica command | Options | What it does |
|---|---|---|
| `seed` | `--group --origin` `[--worktree --store]` | First machine only: publish the whole shared tree and write `.cache/f/machine.json`. |
| `join` | `--group --origin` `[--worktree --store]` | Materialise the tree from the newest snapshot plus the journal. A non-empty tree must hold only shared files that match the store by hash, otherwise it is refused; matching files are adopted. |
| `run` | `[--worktree --store --poll --scan --debounce]` | The replica daemon. Needs `machine.json`. NDJSON progress on stdout. |
| `status` | `[--worktree --store --json --deep]` | Daemon, store reachability, queues (`--deep` scans the tree for the outgoing count and for the number of refused paths), overwritten files kept in `.cache/f/lost`, lag per origin. Needs `machine.json`. |
| `snapshot` | `[--worktree --store]` | Write a full manifest to the store. |
| `relay` | `[--group --store --worktree --peers --poll --bootstrap-minutes --concurrency --once]` | Group server: copy the other groups' origins into the own store, `--concurrency` origins and blobs at a time (default 8, 1 to 64); unless `--once`, also rebuild the bootstrap archive. |
| `compact` | `[--worktree --store --retention-days --collect-blobs]` | Snapshot, then drop this origin's old sealed segments and snapshots older than the retention. With `--collect-blobs` (group servers only) also the blobs that the folded manifest does not reference and that are older than the retention, streamed directory by directory (memory follows the live tree, not the garbage; progress per directory, cancellable). Every compaction also removes staging files in `<store>/v1/tmp` older than 6 hours. |
| `verify` | `[--worktree --store --json]` | Rehash the tree and compare it with state and store; exit 1 on drift. Also lists the paths that the path policy refuses (`refused`, `refusedTotal`; not drift). |
| `bootstrap-archive` | `[--worktree --store]` | Write the bootstrap archive now (needs `machine.json`). |
| `bootstrap-settle` | `[--worktree --store]` | Prepare an extracted tree for `join` (needs no identity, refuses a joined tree). |

`--worktree` defaults to the repository root, `--store` to the store of `machine.json`, else to the `storeMount` of
the topology for this OS. `relay --peers <group>=<directory>[,<group>=<directory>...]` replaces the peer mounts of the topology
(a rehearsal on one host points them at local store directories; without the option the topology decides). `--group` and `--origin` default to `machine.json` where the command allows it.

| Topology command | Options | Prints |
|---|---|---|
| `env` | `--group [--shell sh\|powershell]` | `SEMIO_REMOTE_CACHE_ENDPOINT`, `SEMIO_REMOTE_CACHE_ACCESS=read-write`, `SEMIO_REMOTE_CACHE_INSTANCE`. |
| `mount-plan` | `--group [--platform --part]` | Mount of the group store for the OS, by the protocol of the group server (NFS, or SMB for a Windows server without Server for NFS). On Linux also the sysctl.d file with the inotify limits (parts `sysctl-path`, `sysctl`); on Windows the `cache` part that switches off the attribute and name caches of the SMB or NFS client. |
| `server-plan` | `--group [--part]` | Setup of the group server on the platform that the topology names for it (Linux, macOS or Windows). Every platform has `prepare`, `provision`, `start`, `relay-peers`, `cache-environment-target`, `cache-environment`, `cache-target`, `cache-unit`, `replica-unit`, `relay-target`, `relay-unit`; Linux adds `exports-target`, `exports`, `sysctl-target`, `sysctl`, `fstab`; macOS `exports-target`, `exports`, `nfs-conf-target`, `nfs-conf`, `peer-mount-target`, `peer-mount-unit`; Windows `feature`, `share`, `firewall`. |
| `worker-service` | `--group [--platform --part --role worker\|server]` | Service definitions; parts `target`, `definition` (replica daemon), `compact-target`, `compact-definition` and, on Linux, `timer-target`, `timer-definition`. `--role server` renders the group server's compaction with `--collect-blobs` (on the server's own platform). |
| `facts` | `--group [--platform --shell]` | Paths, protocol, mount values and server facts for the provisioning scripts. |
| `preflight` | `[--group] [--worktree --store --timeout <ms> --lang en\|de]` | Run on any machine after provisioning: checks without changing anything, see "Preflight" under Day 0. Progress per finished check on stderr, the report on stdout in English and German (`--lang` picks one), exit 1 when a check failed or Ctrl+C cancelled it. |
| `validate` | `[--file] [--reachable [--timeout <ms>]]` | Validate the topology offline. With `--reachable` it then resolves the store host and the cache host of every group and opens a TCP connection to the store port (2049, or 445 for an SMB group) and to the cache port (6090), each limited by `--timeout` (default 3000 ms, 100 to 60000). Progress lines (one per finished probe) go to stderr, the report per group to stdout, the exit code is 1 when a probe fails. Ctrl+C cancels the probes and reports `reachability cancelled`. It does not check mountd/rpcbind (111, 1048, 892) of NFSv3 servers. |

## Two writers of one file

Nothing is merged and nothing blocks. When a remote edit reaches a machine that has an **unpublished local edit** of the
same path (offline, or inside the poll interval), the daemon compares the two by wall time: the local file's modification
time against the wall-clock time inside the remote event, which is the moment the other machine **published** it.

- Local edit later: it is kept, reported as `kept`, and published at once with a new clock value, so it wins everywhere.
- Local edit earlier or equal: the remote version is applied and the local bytes are copied to
  `.cache/f/lost/<hlc>/<path>` (reported as `overwritten`). Copy back what you want to keep.
- A local delete has no modification time and loses to any remote edit: the file comes back, nothing is published.
  A remote delete is compared with a dirty local file like any other event.
- Both machines edited offline: the first one to publish wins, the other's edit goes to `lost/` unless its modification
  time is later than that publish time.

The honest limit: the clock inside an event is the time of publication, not the time the key was pressed. Writes to the
same file on two machines within the publish latency (about half a second) are ordered by who published first, and an
edit made offline is ordered by when it was published. Every machine converges to the same version either way. The
comparison reads one machine's file clock against another's event clock, so keep NTP running.

## Network layout

- Five physical networks, one per group, form one logical network. Each group has one **group server** that runs
  four services: the share that holds the store, the cache server (`repo remote-cache serve`, port 6090 in
  the topology), the relay and a replica daemon of its own. The group server is a fleet machine: its origin is
  `<group>-server` (for example `g1-server`) and its worktree is the canonical path of its own operating system.
  The server may be a Linux, a macOS or a Windows machine; the topology names which (`server.platform`).
- A **worker** talks only to its own group's share (2049/tcp for NFS, 445/tcp for SMB; NFSv3 also needs rpcbind 111
  and mountd) and its own group's cache server. It needs no route to the other groups.
- The group servers must reach each other. Each relay mounts the other four exports read-only (the `peers` entries
  of the topology, for example `/mnt/semio-peers/g2`) and copies what is missing into its own store. An origin is
  only ever pulled from the store of its home group, so there are no loops, and below `origins/<originId>/`
  there is one writer per store: the home machine in its home store, that group's relay in every other store.
- Consequence: every group store holds the whole fleet's data, not only its own group's. The size of one store
  is the size of the fleet (see Capacity).

### Server platforms and the protocol each worker mounts

| Group server (`server.platform`, `server.protocol`) | Linux workers | macOS workers | Windows workers |
|---|---|---|---|
| Linux, `nfs`: systemd, `/etc/exports.d/semio-fleet.exports`, `nfs-server` | NFS 4.2 (`nfs4`) | NFS 4 (`mount_nfs vers=4`) | NFS 3 (Client for NFS, `mount.exe`) |
| macOS, `nfs`: launchd, `/etc/exports`, `nfsd` (serves NFS 3) | NFS 3 (`nolock`) | NFS 3 (`nolocks`) | NFS 3 |
| Windows Server, `nfs`: Server for NFS, `New-NfsShare` (NFS 3) | NFS 3 | NFS 3 | NFS 3 |
| Windows 10/11 or a server without Server for NFS, `smb`: `New-SmbShare` | not supported | not supported | SMB (`net use S:`) |

The group server picks no protocol silently: `server.protocol` in the topology decides, the scripts only check it
(the PowerShell script refuses `nfs` when `Install-WindowsFeature` does not exist, and says to set `smb`). A Linux or macOS
worker of an `smb` group is refused with "Windows workers required" by `facts` and `mount-plan`; a group with
`smb` can therefore only have Windows workers, and no macOS group server may exist anywhere in a fleet that has
an `smb` group (a macOS relay cannot read an SMB store; `readTopologyV1` refuses the topology). The relays of the other
group servers read each store read-only: a Linux server mounts NFS peers by `nfs4` (Linux peer) or `nfs` v3 (macOS or
Windows peer) and SMB peers by `cifs` with `/etc/semio/smb-peer.credentials`; a macOS server mounts NFS peers at
`/Volumes/semio-peers/<id>` with a LaunchDaemon; a Windows server reads the peers by UNC path (`\\host\share`) without a mount; its `feature` part therefore also installs Client for NFS (`NFS-Client` on Windows Server, the optional features on Windows 10/11) when any peer group is served over NFS.

How the three servers differ:

- **Linux.** `/etc/exports.d/semio-fleet.exports` (read-write for `server.network`, read-only for the peer servers),
  an fstab block (bind mount of the export at the store mount, peer mounts), systemd units for cache and relay.
- **macOS.** The export must lie below `/Users` or `/Volumes` (the root volume is read-only; the topology refuses other
  paths). `/etc/exports` gets two lines (workers read-write by `-network/-mask`, peers `-ro`), `/etc/nfs.conf` pins the mount
  daemon to port 892. The server has no mount of its own store: its replica, relay and compaction use the exported folder itself
  (`--store`, recorded in `machine.json`). Cache and relay are LaunchDaemons; the cache reads
  `/etc/semio/remote-cache.env` (mode 0600, owner = the server account) through `sh`.
- **Windows.** `nfs.export` is the single-segment share name (`/semio-store`) and `server.directory` the local folder
  (`C:\semio-store`). Server for NFS (`FS-NFS-Service`, Windows Server only) shares it with `New-NfsShare` (anonymous
  access mapped to uid 1500, read-only entries for the peer servers); the `smb` fallback creates the local accounts
  `semio-fleet` (workers, Change) and `semio-fleet-peer` (relays, Read) with `New-SmbShare`. Windows workers of an SMB group
  store the account once with `cmdkey /add:<host> /user:<server name>\semio-fleet /pass:<password>` (the `credential` part of
  `mount-plan`). The firewall allows the store ports from `server.network` and from the peer servers only. Cache and relay
  are Scheduled Tasks (`semio-fleet-cache`, `semio-fleet-relay`) next to the replica task; the cache task reads
  `C:\ProgramData\semio\remote-cache.env`, which only the server account can read. All three tasks run at logon of the
  server account (give the machine an automatic logon, or register them with a password).

### The one file the operator edits

`🗺️topology/🔣️.json` is the only file the operator edits to make the fleet real. The hosts in it are placeholders
(`nfs.gN.semio.internal`, `cache.gN.semio.internal`, `10.0.N.0/24`). Per group set `nfs.host`, `nfs.export`,
`server.platform`, `server.protocol`, `server.network` (CIDR of the group's workers; `server.directory` on Windows),
`cache.endpoint` and `cache.instance`; per fleet the canonical `worktree` and `storeMount` per OS and the `replica`
timings. Everything else is derived from it and never edited by hand: the mount lines, exports, fstab blocks, units,
LaunchDaemons, Scheduled Task XML, firewall rules, the cache environment of every machine, the scripts' checks (they
refuse a topology file that differs from the one in the worktree) and the Python oracle in `🧪️tests`. After the
edit run `validate` (offline, checks shape and the server rules), then `validate --reachable` from a machine of each
group network: it reports per group whether the hosts resolve and the store and cache ports answer. Re-provision the
machines whose group changed; the scripts are safe to repeat.

## Provisioning and the bootstrap archive

A brand-new machine has an empty disk, so it has no worktree, no `repo` launcher and no Bun, yet `join` lives in the
worktree. The group server closes the gap with an archive of its own worktree.

**The archive.** `<storeRoot>/v1/bootstrap/worktree.tar` is a POSIX pax tar written by our own code (only `node:`
built-ins) with one regular-file entry per shared file of the folded store manifest, read from the group server's local
worktree. Pax extended headers carry the long UTF-8 emoji paths and, from 8 GiB, the file size. Links are not in the
tar: `join` creates them. A file whose hash differs from the manifest (not yet replicated to the server, being
written, vanished) is skipped. `manifest.json` next to it is `{"version":1,"hlc":…,"files":n,"bytes":n,"sha256":…}`:
`files` and `bytes` count the entries and their content, `sha256` is the hash of the whole tar. The tar is staged in
`<storeRoot>/v1/tmp` and renamed, then the manifest the same way, so a reader that sees a mismatch retries. The writer
streams in 1 MiB chunks (bounded memory), reports progress as NDJSON and stops cleanly on cancellation. All names in
the store stay ASCII.

**Refresh.** The relay on the group server rebuilds the archive when it starts and then every
`replica.bootstrapMinutes` of the topology (default 60; `relay --bootstrap-minutes <n>` overrides it). The archive
needs the server's own replica daemon (`run`) to keep the server's worktree current; the server plan installs both.
`bootstrap-archive` builds it once by hand. A failed build is reported as an `error` event and retried after at
most two minutes (so a store that is still empty gets its archive soon after the first files arrive); staging files of a
crashed build older than six hours are removed by the next build. A relay started with `--group` but without `machine.json`
does not build it.

**Provision one machine.** `🗺️topology/🥾️provision/🐚️.sh <group> <origin> [--user <account>] [--archive <directory>]`
(macOS, Linux, as root or through sudo) and `🔵️.ps1 <GroupId> <OriginId> [-ArchivePath <directory>] [-User <account>]`
(Windows, elevated) take the machine from an empty disk to a running replica. Copy the `🗺️topology` directory of
the repository to the machine first: the scripts read the worktree path and the NFS export from the topology file next to
them, before any tool exists, and refuse to continue when the topology command later derives other values from the
worktree. Steps, in this order:

1. Create the canonical worktree directory. A directory that already holds a checkout skips
   the archive; one that is neither empty nor a checkout is refused. **Windows: the script first sets
   `HKLM\SYSTEM\CurrentControlSet\Control\FileSystem\LongPathsEnabled = 1` and reads it back** (the tree has deep emoji paths;
   the path policy allows 256 characters below the root, and `MAX_PATH` is 260), and keeps the worktree root short:
   the schema accepts only a drive and one directory of at most 12 characters (`C:\semio`), the script refuses a longer one.
   The setting reaches processes that start afterwards; a shell that was open before needs a restart. Git for Windows is
   not involved (`.git/` is not replicated), but any other tool that opens these paths must be long-path aware.
2. Windows mounts the store first (Client for NFS); macOS and Linux mount the export read-only at a temporary
   directory. Copy `worktree.tar` and `manifest.json` to local disk (the `--archive` directory instead, if given),
   verify the sha256 of the tar against `manifest.json` (five attempts, 20 s apart, because the archive may be rebuilt
   while it is copied) and extract it into the empty worktree with the OS tar: `tar -xf` as the worker account on
   macOS and Linux (with a UTF-8 locale), `tar.exe` (bsdtar, reads pax UTF-8 names) on Windows 10 1803 or later. macOS and
   Linux then compare the number of extracted files with `manifest.json`, Windows the number of entries in the tar.
3. Build the standalone `repo` launcher for the account (`🚀️launcher/📥️install/🐚️.sh` or `🔵️.ps1`, from the checkout, into the account's
   home; it fetches its pinned Go itself and needs no compiler and no package manager, only `curl` or Python 3 and `tar`) and
   link it as `~/.local/bin/repo` (Windows: the launcher directory goes on the user `PATH`). On macOS and Linux the script then
   links `/usr/local/bin/repo` to it, which the service definitions use. **Joining the fleet never depends on the native build**:
   a fix for a broken build arrives through the shared worktree itself, so everything up to the service start runs through this
   launcher and the machine is syncing before the native bootstrap is even tried.
4. Mount the store for good (fstab line on Linux, LaunchDaemon on macOS, logon task on Windows), write the cache
   environment, install the replica daemon and the daily `compact` (see Day 2).
5. `bootstrap-settle`, then `join --group <group> --origin <origin>`. The archive can be up to `bootstrapMinutes`
   old, so some files differ from the store by then. `join` refuses a tree with such files, so `bootstrap-settle` first
   renames names that the OS tar extracted as NFD (macOS `tar` does this) back to NFC, then moves every shared file that differs from the store, or that the store deleted, to
   `.cache/f/lost/bootstrap-<ms>/<path>` (on macOS and Linux a file whose executable bit differs from the store counts as
   different; it never deletes, it refuses a store without files and a tree that already has `machine.json`); where the store holds a version of a moved file, settle puts that version back at once (otherwise the subsystem's own
   scripts, if they changed since the archive, would be gone before `join` starts); `join` then
   adopts the matching files and materialises the rest. The extracted files are recorded in the replica state as seen
   now, so the first scan of the daemon trusts those whose modification time is more than 2 s older and rehashes the others
   once (the racy-timestamp rule); the daemon follows the store before its first publish, so nothing of the archive is
   published as a local edit.
6. Start the replica daemon and the compact timer (on a group server also the cache and the relay).
7. **Last: the native bootstrap** (`🔩️native/🥾️bootstrap/🐚️.sh setup` or `🔵️.ps1 setup`, as the worker account): it installs the C/C++
   toolchain, Node.js, Rust, .NET, uv, builds and installs the native `repo` and runs `workspace:setup`. It needs network access and,
   on Linux, package-manager rights for the account (sudo) or the packages installed as root beforehand
   (`apt-get install -y build-essential pkg-config curl git unzip`; the script names the exact line of the machine's package manager
   (`apt-get`, `dnf`, `zypper`, `pacman`), `xcode-select --install` on macOS, and `winget` plus the
   `winget install ... Microsoft.VisualStudio.2022.BuildTools` command on Windows). Its output goes to
   `.cache/f/native-bootstrap.log` and the terminal. A completion marker `.cache/f/native-bootstrap.done` in the ignored local state
   is written only after it succeeded and the native `repo` exists; a run without the marker (or without the launcher) repeats the
   bootstrap, so one that failed half way is redone by the next run. The Windows script requires `-User` to be the account of the
   elevated shell, because the bootstrap installs per account.
   On Linux the native bootstrap runs through `sudo` without a login shell, so the profile files of step 5 do not reach it: the script
   hands it the `SEMIO_REMOTE_CACHE_*` variables of `env` and the write token of the account (`SEMIO_REMOTE_CACHE_TOKEN` from the
   environment of the script or the `export` line of `~/.profile` or `~/.zprofile`) through a `0600` file in `.cache/f` that it removes
   afterwards, never as an argument. That is how a fresh machine takes the project graph of its group from the cache server instead of
   inferring all projects cold (a cold graph needs a 7 to 8 GB heap and hours); the server's own native bootstrap is the warm-up that
   uploads it. Linux machines with less than 16 GiB of RAM plus swap also get a swap file (`/var/lib/semio-swap`, step 4) so that a
   cold build that does happen is slow instead of killed for lack of memory.

   **When the native bootstrap fails** (or its build tools are missing and cannot be installed), nothing above is undone and the
   script does not fail the fleet setup: it prints `NATIVE BUILD PENDING` with the failing command, the reason, the log and its
   last lines, leaves the marker absent and exits with code 3. `preflight` shows the same state as the warning `native build`.
   The machine is a member of its group and syncs, so the fix usually arrives by itself through the shared worktree: run the
   script again (it repeats only what is missing, then the native bootstrap) after the cause is fixed or the fix has arrived.

   | Exit code | Meaning |
   |---|---|
   | 0 | fleet ready and native build complete |
   | 1 | error: the failing step is named, nothing after it ran (Windows: the script threw) |
   | 2 | usage error (shell script) |
   | 3 | **fleet ready, native build pending**: joined, services running, native bootstrap failed or not possible, marker absent |

**Linux inotify limits.** The replica daemon watches the tree recursively, one inotify watch per directory, and the tree has
about 122,000 directories (121,884 measured in the rehearsal); the kernel default of 8,192 watches would drop events
silently. The script writes `/etc/sysctl.d/90-semio-fleet.conf` (`fs.inotify.max_user_watches=1048576`,
`fs.inotify.max_user_instances=1024`, the `sysctl` part of `mount-plan --platform linux`), applies it with `sysctl -p`
and reads every key back; it stops with the key and the value when a limit stayed too low (inside a container the host
must raise them). Watches cost kernel memory (about 1 KiB each, so 122k directories need roughly 130 MiB per user
that runs a daemon); the high limit leaves room for editors and build tools next to the daemon. Measured on Linux: a daemon on the
full tree holds 154,895 watches (the tree has 154,899 directories including empty ones and the ones that `repo` creates) of the
1,048,576 allowed; each of the 3 daemons on one account counts separately, so the limit is per user, not per daemon.

**Re-running the script.** It can be repeated at any time (new topology, new service definitions, a repaired mount).
Before it changes anything it reads `.cache/f/machine.json` of the worktree, if present: when the origin or the group in it
differs from the arguments, it stops with a message that names both and says to pass the right ones or to remove the worktree state; when
both are equal, `join` (or `seed`) is skipped, but the mounts, limits, environment, services and tasks are installed again
and restarted when a definition changed. A machine that was provisioned with another group must be reset by hand, never
silently rejoined.

Every step probes first, so a failed run can simply be repeated; a worktree from an interrupted extraction must be
emptied by hand (the script says so).

**Language.** The launcher refuses a start that chooses no language, and a fresh Ubuntu has `LANG=C.UTF-8`, which names none. The
script therefore runs every `repo` call with `SEMIO_LANGUAGE` (default `en`, override it in the environment of the script), and the
generated Linux cache unit carries `Environment=SEMIO_LANGUAGE=en`. The replica units do not need it: the replica prints
both languages.

**Changed mount plan.** The Linux worker replaces its own `/etc/fstab` line (same source and mount point, other options) and
stops with the foreign line when another source is mounted there. A mount that is already up keeps its old options: the
script warns; stop the replica services, `umount` and `mount` the store to apply the new ones (`remount` cannot change
`lookupcache`). The macOS script warns the same way when its mount LaunchDaemon changed while the store is mounted; the Windows
script restarts the Client for NFS service (`nfsadmin client stop`, `nfsadmin client start`) after it changed a cache value, before it mounts.

**Client caches (what is measured and what is reasoning).** The replica `stat`s `head.json` every 250 ms and the file is replaced by a
rename, so a client that answers a lookup from a cached name or from cached directory attributes shows the old head. Every store
client is therefore set to revalidate; the settings differ because the clients do:

| Client | Setting (mount plan and provisioning script) | Status |
|---|---|---|
| Linux NFS 4.2 and 3 (also the peer mounts of a Linux group server) | `actimeo=1,lookupcache=none` | **Measured** (`linux-nfs-report.md`): without `lookupcache=none` followers see 46 of 100 new heads, with `lookupcache=positive` after 505 ms median, with `none` after 10 ms; end to end 305 ms against 696 ms. |
| macOS NFS 4 and 3 (workers and the peer mounts of a macOS group server) | `acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache` | **Measured on macOS 27 (NFS 4.0 client of a Linux server)** (`mixed-os-report.md`; the NFS 3 variant for a macOS server was not run). `stat` of a replaced head, 100 replacements: with these options all 100 seen after 2 ms median (4 ms maximum); with the `mount_nfs` defaults only 28 of 100 seen at all, 672 ms median, 1.3 s late; with `actimeo=1` 446 ms median and 917 ms at p95; with `noac` as with the plan. `mount_nfs` has no `lookupcache`: `acdirmin=acdirmax=0` is what revalidates the name. Cost of the poll (four heads four times a second, `nfsstat -c`): 65 NFS calls per second with the plan, 79 with `noac`, 11.5 with `actimeo=1`, 0 with the defaults (which is why they are stale). `nonegnamecache` made no difference to the head and stays for names that appear later. The macOS client speaks NFS 4.0 for `vers=4` and NFS 4.1 for `vers=4.1` (same results, not needed), reads and writes in 32 KiB (`rsize` is not raised: nothing measured needs it) and has no `nconnect`. |
| Windows SMB (workers of an `smb` group, and a Windows server that reads an `smb` peer) | `Set-SmbClientConfiguration -DirectoryCacheLifetime 0 -FileInfoCacheLifetime 0 -FileNotFoundCacheLifetime 0` (the `cache` part of `mount-plan`) | **By reasoning.** The cmdlet and its parameters are the documented ones; the defaults (10 s, 10 s and 5 s) would show a replaced head up to 10 s late. Not run on a Windows machine. |
| Windows Client for NFS (workers of an NFS group, and a Windows server that reads an NFS peer) | two DWORD values `RemoteWriteCache` and `FileAttributeCache` = 0 under `HKLM\SOFTWARE\Microsoft\ClientForNFS\CurrentVersion\Users\Default\Cache`, then `nfsadmin client stop` and `nfsadmin client start` (the `cache` part of `mount-plan`) | **Documented by Microsoft for older Windows, not yet confirmed on Windows 11/Server 2022.** Source: [Steps to disable Client for NFS cache](https://learn.microsoft.com/en-us/archive/blogs/technet/sfu/steps-to-disable-client-for-nfs-cache) (key, value names, DWORD 0, restart of the service; the same two values are in [All (well, almost) about Client for NFS](https://learn.microsoft.com/en-us/archive/blogs/msdn/sfu/all-well-almost-about-client-for-nfs-configuration-and-performance)). `mount.exe` has no cache option. The script creates the key if missing, sets the values, reads them back and restarts the service before it mounts. The folder content cache (30 s) is a different setting (Microsoft KB 894071), not set here. Whether the client honours the values on a current Windows is the first thing to check on day 0 (below). |
| Linux `cifs` (a Linux group server reading an SMB peer) | `actimeo=1` | Unchanged and not measured: a head of an SMB group can reach a Linux relay up to one second late, which only delays the cross-group copy. |

**NFSv4 delegations (measured).** A macOS client that replaces a file which another client holds a read delegation on waits about 4 s
(the Linux server answers `NFS4ERR_DELAY` while it recalls the delegation, and the macOS client retries late): every second save from
a macOS machine reached the group 4 to 8 s late (median 4,270 ms, p95 8,252 ms, 49 of 200 saves overtaken), because the relay of another group, or
any other worker, polls the `head.json` of every origin and so holds a delegation on it. Every Linux group server therefore sets
`fs.leases-enable=0` (`sysctl` part of `server-plan`, file `/etc/sysctl.d/91-semio-fleet-server.conf`, applied and read back by the script before
the NFS server starts): no delegations are granted, and the same saves arrive after 347 ms median (p95 477 ms, none overtaken). The macOS mount option `nocallback` does not help,
because the delegations belong to the other clients. Linux clients retry after 100 ms and never showed the stall.

Acceptance on a macOS or Windows client, before the fleet relies on it: save a file on a machine of another OS in the same group and watch
it appear (the sync latency probes of the ticket folder, `nfs-xwrite.py` and `nfs-xread.py`, do exactly this); it must appear within
about a second, not after 3 s to 60 s. If it does not, the settings of that row are not enough and the preflight cannot see it.

Flags of the scripts: `--user` (default: the sudo caller) is the account that owns the worktree and runs the daemons.
`--archive <directory>` replaces the store of the group as the source of the archive (a peer group mount, see day 0).
`--first-machine` (Windows `-FirstMachine`) is for the very first machine of the fleet: a checkout is at the canonical
path, the store is empty, and `seed` runs instead of `join`; it is refused when the store already has origins.
`--server` (Windows `-Server`) marks the group server, the first one (`--first-machine --server`) or a later one, on the platform
that the topology names for it (the script refuses another one). The script does the whole server setup of the
`server-plan` of that platform: creates the folders, writes the exports (Linux `/etc/exports.d`, macOS `/etc/exports` and `/etc/nfs.conf`;
Windows `New-NfsShare` or `New-SmbShare` plus firewall rules), mounts or reads the peers, installs the cache and relay
services and starts them after the join. It never writes the cache tokens: `/etc/semio/remote-cache.env` (Windows
`C:\ProgramData\semio\remote-cache.env`) must exist with real values for both lines of `server-plan --part cache-environment`, or the
script stops. On Linux the store is the export bind-mounted at the store mount (no NFS client entry); on macOS and Windows the
store root is the exported folder (`--store`). The daily compaction is installed with `--collect-blobs`; a later server (no
`--first-machine`) needs `--archive` (Windows `-ArchivePath`) and, before settle and join, runs `relay --once` with the peers of
the `relay-peers` part to copy the other groups into its still empty store.

## Day 0

1. **Topology.** Edit `🗺️topology/🔣️.json` (the one file, see "The one file the operator edits"): group ids (`g1` to `g5`), NFS host and export,
   server platform, protocol and network, cache endpoint and instance, peer mount points per group, the canonical worktree
   path and store mount per OS, the replica timings. Run `validate`, then `validate --reachable`. The topology is a shared source file, so after the first machine has seeded, edits to it propagate
   like any other file; change it on one machine and let it replicate.
2. **Canonical path.** Cache hits across machines need identical checkout paths, so every replica lives at the
   worktree path that the topology fixes for its OS (default `/srv/semio`, `/Users/Shared/semio`, `C:\semio`). Do
   not pick another path, not even a symlink to it.
3. **Cache tokens.** Create two tokens of the characters `A-Za-z0-9._~+/=-`, for example `openssl rand -hex 32`:
   a read token and a write token. Keep them in your password manager.
4. **First machine: the server of the first group (say `g1`).** This is the only machine that starts from a checkout.
   1. `server-plan --group g1 --part prepare` prints the commands that create the `semio` account (uid 1500, home
      `/var/lib/semio`, it owns the worktree, the store and the cache and runs all server services), the store and
      cache directories, `/etc/semio` and the mount points (Linux). On macOS and Windows it prints the folders and
      the Windows features or local accounts instead; apply it. The script writes the exports and the fstab block itself
      (step 4 below); a peer whose server does not exist yet is skipped with a warning and mounted at boot (`nofail`) or by the next run.
   2. Write `/etc/semio/remote-cache.env` (owner root, group `semio`, mode 0640) with the two lines of
      `server-plan --part cache-environment` and the real tokens. The cache server refuses a non-loopback
      address without a token, and the unit binds `0.0.0.0`.
   3. Copy a complete checkout (`.git/` not needed) to the worktree path and `chown -R semio:semio` it. Put the
      `🗺️topology` directory somewhere the script can read it.
   4. Run `bash 🥾️provision/🐚️.sh g1 g1-server --user semio --first-machine --server`. It builds the launcher, mounts the store, seeds it (about 270k
      files, 5 GB, as long as it takes to hash and copy 5 GB to NFS), installs the replica and compact units and starts them, and runs the native bootstrap last
      (give `semio` sudo rights for the packages until it has finished, or install the toolchain first; exit code 3 means that the server is ready and only the native build is pending).
   5. The script already installed the cache and relay services (Linux units, macOS LaunchDaemons, Windows tasks) and started them after the join;
      `--part start` prints the equivalent commands. The relay builds the first bootstrap archive. Run `snapshot` once so that
      later machines join from a snapshot instead of replaying the journal. Check `http://<cache host>:6090/healthz`.
   6. Never run `seed` or `--first-machine` on a second machine. It would publish a second, competing copy of every file.
5. **The other four group servers (`g2` to `g5`).** On each: apply `prepare` (it mounts all peer exports, including `g1`), write
   the token file as above, then
   `bash 🥾️provision/🐚️.sh <group> <group>-server --user semio --archive /mnt/semio-peers/g1/v1/bootstrap --server`
   (the `provision` part of `server-plan` prints this line with the first peer's mount), then `--part start`. The new
   server has no archive of its own yet, so it takes the archive of `g1` from the peer mount, runs one relay round
   to fill its own store, settles and joins. Its relay then builds the archive of its own group. Wait until `status`
   shows the other origins before you join workers in that group.
6. **Workers.** On every other machine, as root (macOS, Linux) or elevated (Windows): copy the `🗺️topology` directory,
   run the provisioning script with the group and a fleet-unique origin id (`^[a-z0-9][a-z0-9-]{0,39}$`, for example
   `g2-m07`) and `--user <account>` (Windows `-User`). Then give the account the write token (see Cache tokens
   and the worker environment) and run `verify`; it must exit 0, then run `preflight` (below).
   **Which machine of a group is the server.** Only a machine with a 2 TB disk can be (the fleet's machines have 128 GB to 2 TB of
   NVMe, and the store of a group needs 0.5 to 1.5 TB, see Capacity): every group needs at least one 2 TB machine, and that one
   is the group server. Take the one that is always on, on a wired network, with 8 cores and 32 GB of RAM; a group without a 2 TB
   machine gets a dedicated NVMe disk for its server, or a shorter `replica.retentionDays` in the topology (7 days halves the blobs). The 128 GB machines are
   workers only. The preflight on the server fails below 50 GiB and warns below 300 GiB free in the store volume.

   **Preflight.** After provisioning, on every machine (worker or group server), run in the topology folder
   `repo ./📜️script.ts preflight` (`--group <id>` when `.cache/f/machine.json` is missing, `--timeout <ms>` per check, default 5000, `--lang en|de`).
   It reads and measures, and changes nothing but one probe file in `<store>/v1/tmp` that it creates, reads back and deletes at once.
   Checks, each with its own verdict (`ok`, `WARN`, `FAIL`, `skip`):

   | Check | Passes when |
   |---|---|
   | identity | `machine.json` exists and names this group; the origin `<group>-server` is the group server |
   | store mount | the store is mounted from the source of the topology (Linux and macOS: `mountinfo`/`mount`; Windows: `net use`); on a Linux group server it is a local bind mount, on macOS and Windows servers the folder exists |
   | mount options | Linux: `hard`, `lookupcache=none` and the one-second attribute caches are in effect; macOS: `acdirmin=0`, `acdirmax=0`, `nonegnamecache` (a warning when `nfsstat -m` does not show them); skipped on Windows (see client caches) |
   | store writable | a probe file in `v1/tmp` is written, read back and removed |
   | cache endpoint | `<endpoint>/healthz` answers 200 with `status: ok` |
   | clock skew | the clock differs from the group server by at most 2 s (warning above, failure from 30 s). It uses the `Date` header of the server's cache endpoint (one-second resolution, round trip subtracted), so it is a measurement to about 0.5 s; 30 s is half of the 60 s after which a receiver holds an event back, so that two machines that are each below it still differ by less than 60 s. The group server itself is the reference and skips this check |
   | time synchronisation | the OS reports a synchronised clock (`timedatectl` NTPSynchronized, `systemsetup -getusingnetworktime`, `w32tm /query /status` with a real source); a warning otherwise (the provisioning scripts enable it: `timedatectl set-ntp true`, `systemsetup -setusingnetworktime on`, the `w32time` service; chrony is the recommendation on Linux) |
   | inotify limits (Linux) | `max_user_watches >= 1048576` and `max_user_instances >= 1024`, the values of the sysctl file |
   | long paths (Windows) | `LongPathsEnabled` is 1 |
   | client caches (Windows) | the SMB lifetimes or the NFS registry values are 0 for the protocols this machine reads |
   | free disk | the worktree volume has at least 10 GiB (warning below 30 GiB: tree 4.5 GiB, bootstrap copy 5 GiB, caches); on a server the store volume at least 50 GiB (warning below 300 GiB) |
   | native build | the completion marker `.cache/f/native-bootstrap.done` of the provisioning exists; **pending** (a warning, never a failure) while the native bootstrap has not completed, with the log `.cache/f/native-bootstrap.log` and the way to retry it (run the provisioning script again) |
   | services | the replica daemon runs (systemd instance, LaunchDaemon `state = running`, scheduled task `Running`), the daily compaction is scheduled (a warning otherwise), and on a server the cache and the relay run |
   | replica status | `status` reaches the store, the daemon runs, no origin needs a new `join`; a warning when an origin is more than 1,000 events behind |

   The exit code is 1 when a check failed or Ctrl+C cancelled the run. Progress lines (`[n/total] mark name: detail`) go to stderr as each check finishes.
   Its probes (mount tables, registry, `schtasks`, `timedatectl`) are tested only with injected outputs and, on macOS, with the real
   system calls; the parsing of the real `mount` and `nfsstat -m` output of macOS 27 is verified (`mount options: ok` for the plan, `FAIL ... acregmin=1 (found acregmin=5), acregmax=1 (found acregmax=60), ...` for the defaults);
   `net use` and `reg query` are unverified.

7. **Archivist (optional).** Designate one machine for history. It is an ordinary fleet machine that also has
   git and a `.git/` of its own, which is not replicated. It commits its replica on a timer and pushes `main`.
   The skills `commit`, `micro-commit` and `merging` in `.agents/skills/` apply only there. Keep it in one group
   and never run git mutations anywhere else.

## Cache tokens and the worker environment

Provisioning writes the machine environment of `env` (`SEMIO_REMOTE_CACHE_ENDPOINT` = the cache of the machine's own
group, `SEMIO_REMOTE_CACHE_ACCESS=read-write`, `SEMIO_REMOTE_CACHE_INSTANCE`): to `/etc/profile.d/semio-fleet.sh` on
Linux, `launchctl setenv` plus `/etc/zshenv` and `/etc/profile` on macOS, the Machine environment on Windows. The
build tooling resolves its remote cache contract from exactly these variables
(`📚️library/🏗️build/🌐️remote-cache`): the endpoint switches the cache on, `read-write` lets it upload local results, and the bearer
token is read from the variable named by `SEMIO_REMOTE_CACHE_TOKEN_ENVIRONMENT`, default `SEMIO_REMOTE_CACHE_TOKEN`.

The scripts never write a token, because those profile files are readable by every local user. The operator gives
each worker account the **write token** in `SEMIO_REMOTE_CACHE_TOKEN` (a read token would make every upload fail with 403):

- Linux: a line `export SEMIO_REMOTE_CACHE_TOKEN=<write-token>` in the account's `~/.profile`, file mode 0600.
- macOS: the same line in `~/.zprofile` of the account, mode 0600.
- Windows: `[Environment]::SetEnvironmentVariable('SEMIO_REMOTE_CACHE_TOKEN','<write-token>','User')` as that account.

Only the build tools need the token; the replica daemons do not. Whether every way an agent starts a build
(ssh, IDE terminal, service) inherits the variable has not been checked: run `repo remote-cache status` as the
agent's own user, it prints whether the token variable is set (never its value). For a consumer that must not
upload (a CI job), give it the read token and `SEMIO_REMOTE_CACHE_ACCESS=read-only`. To rotate, change
`/etc/semio/remote-cache.env`, restart `semio-fleet-cache.service` and replace the variable in the accounts.

## Day 2

**Look at the state.** `status` on any machine shows its replica daemon, the store, the queues and the lag per
origin; `--deep` also scans the tree to count the unpublished local changes. The output of `run` (the service log) reports `refused` paths,
`overwritten` files (a local edit that lost, kept in `lost/`) and `kept` paths (a local edit that was newer than the remote one and was published), and the output of `relay` shows whether the relay is current. A refused path is a developer
problem (a name that is not NFC, a Windows-illegal name, a case collision, an absolute symlink, a path longer than 256
characters); the file stays local until it is renamed. The 88 absolute symlinks inside ticket `mirror/` and `basetree`
trees are refused this way on purpose. Every refused path is visible in four places: a `refused` event with its reason
(`too-long`, `not-nfc`, ...) when a scan or a file event meets it, once per daemon run; `status --deep` counts them
(`refused`); `verify` lists them (`refused`, `refusedTotal`, at most 1,000 paths; they do not make `verify` fail); and
the daemon's follow of a remote event for such a path reports `refused` and drops it. A path inside the policy that the
operating system still cannot open (`ENAMETOOLONG`, e.g. a short limit and a deep worktree root) is listed by the scan
as refused `too-long` and, when a remote event meets it, reported as `skipped` (`too-long`) and dropped; it is never
silently ignored and never fails the walk.

**Windows and files held open.** A file that another process holds open (an editor, an indexer, antivirus) makes the
replica's rename, delete or write fail with `EBUSY`, `EPERM` or `EACCES`. Nothing is lost then: the incoming event is
already in the persisted queue when the cursor moves past it, the path is reported once as `blocked` (reason `EBUSY`,
`EPERM`, `EACCES` or `sharing-violation`), retried in place a few times, then at growing intervals (0.5 s to 30 s) on
every follow round, and applied after the release, also after a restart. A local file that cannot be read for publishing is
returned for retry (`blocked`) and published later; a journal write that stays blocked after all retries (`head.json`
held by the virus scanner of the store host) publishes nothing, keeps the events for the next round and never crashes
the daemon. **Symlinks without privilege:** where the account may not create symbolic links (Windows without Developer Mode
or the privilege), the entry is reported as `skipped` with the reason `symlink-unsupported`, removed from the queue
(not retried), nothing is published for it, and every other file of the round is applied; `verify` lists it as
`missing-local` until the privilege exists and the machine `join`s again or the link is created by hand.

**Verify.** `verify` rehashes the local tree and compares it with the local state and the store. It exits non-zero
on drift. Run it after an outage, after a disk problem and, as a recommendation, once a day from a timer. The
provisioning scripts do not install a timer for it.

**Compact.** Every origin compacts its own journal: the provisioning scripts install a daily compact job next to the
replica daemon (systemd timer `semio-fleet-compact@<account>.timer` with a random delay of up to 3 h, LaunchDaemon
`tech.semio.fleet.compact` at 03:30, Scheduled Task `semio-fleet-compact` at 03:30). `compact` writes a snapshot, then
deletes this origin's sealed segments below the snapshot and its older snapshots, all only when older than the retention
(14 days). It never touches another origin's directory, takes no daemon lock and can run next to `run`. Blobs are
collected only with `--collect-blobs`, and only the group server does that: `provision.sh --server` installs its job as
`compact --collect-blobs` (`worker-service --role server`; the collection streams, 2,048 doomed blobs per directory at most
wait in memory, and the references published since the fold are added before every deletion batch), the jobs of workers never delete a blob, because a worker's
view of the other origins may lag and a blob that looks unreferenced to it can still be needed. The server's own
replica sees the whole group store, including the origins that the relay copied in. A machine that has been offline for
longer than the retention cannot follow the journal any more: `join` it again.

**Staging files after `kill -9`.** Writers stage into `<store>/v1/tmp` (names `<origin>-<pid>-<hex>`; the relay uses
`relay-<hex>`) and `.cache/f/tmp`. Opening a replica (`run`, `seed`, `join`) removes the whole local `tmp` and the
own origin's leftovers in the store whose process is dead (a live `compact` of the same origin is spared); `compact` on any
machine removes every file in the store `tmp` that is older than 6 hours, so also the relay's and other origins'
leftovers (NFS `.nfs*` placeholders are skipped).

**Add a machine.** Provision it as in day 0 step 6. Choose an unused origin id. Never reuse the origin id of
a machine that is gone for a replica with an empty journal: other groups still hold the old journal of that
origin, and a restarted journal with the same id would conflict with it.

**Remove a machine.** Stop its service. Its origin directory stays in all stores until compaction removes
the segments that no snapshot needs.

**Replace a group server.**
1. Stop the daemons of the group's workers or let them run: they keep working locally and queue their publishes
   while the store is unreachable.
2. Provision the new server with `server-plan`, with the same host name if possible (otherwise change the
   topology and replicate it).
3. Copy the old export to it with its permissions (`rsync -a`) while the old server is still readable. This is the
   supported path: the store of a group contains the only copy of that group's own origins.
4. Start the export, the cache server, the replica daemon and the relay. The cache is only a cache; an empty one just
   misses until it is warm again.
5. When the workers see the store again they publish what they queued. Run `verify` on a few of them.
6. If the old export is lost and there is no backup, the contract defines no reverse path: relays only pull an
   origin from its home store. The fallback, untested, is to stop all daemons of the group, give every machine of
   the group a new origin id, pick the machine whose tree is right, `seed` it into the new store, and `join` the
   others. That republishes the whole tree with newer clocks, so it wins over older state. Do it in a quiet period.
   Back the export up (snapshot-capable storage plus a nightly copy) so that you do not need this.

**Recover a machine whose replica drifted.** Symptoms: `verify` exits non-zero, builds see files that other
machines do not have, or a disk was restored from a backup.
1. Copy the files you still need out of `.cache/f/lost/` (and any unpublished local edit) to a safe place.
2. Stop the `run` service.
3. Run `verify` to see what differs. If the difference is only unpublished local edits, start `run` again and
   they publish.
4. Otherwise `join` accepts only a tree whose shared files match the store, so move the differing files away (or empty
   the worktree and provision the machine again), run `join --group <groupId> --origin <originId>` with the same origin
   id, then `verify`, then start `run`.

**Clock skew.** A receiver does not apply an event whose clock is more than 60 s ahead of its wall clock: it reports
`clock-skew` once per outage, keeps its read position in front of that event, goes on with the other origins and applies
the event when its own wall clock has caught up. Keep every machine on NTP: the provisioning scripts check it and try to
switch it on (Linux `timedatectl set-ntp true`, with a warning that recommends chrony when the clock stays unsynchronised; macOS
`systemsetup -setusingnetworktime on`; Windows the `w32time` service set to automatic and started), and `preflight` measures the
skew against the group server (warning above 2 s, failure from 30 s). If one machine ran ahead, fix its clock; its held events
then arrive by themselves.

**Store unreachable.** Workers keep editing and building locally and queue their publishes. They need no action when
the server returns. Check `status` afterwards.

## Capacity

Stated hardware: machines with 4 to 16 cores, 8 to 128 GB RAM and 128 GB to 2 TB of NVMe.

- **Replica memory.** Measured on the real tree (269,331 shared files): an idle `run` daemon holds 240 to 290 MB resident
  and `seed`, `join` and `verify` peak below 520 MB (before: 0.8 to 1.3 GB idle and 1.8 to 2.8 GB peak). The in-memory state is
  one compact row per path, not a map of objects; plan about 1 MB of resident memory per 1,000 shared paths on top of the
  runtime's 30 MB. Measured on Linux (Bun 1.4.2, 288k paths): idle `run` 207 to 229 MiB, `join` into an empty directory 342 to
  410 MiB, `verify` 296 to 374 MiB. The inotify watches add kernel memory that the process does not show: three daemons on
  three full trees (468k watches) left about 1.0 GB of slab on a 3 GB machine, mostly the directory inodes that the watches keep
  in memory, and a Python tool of 1.2 GB was killed by the OOM killer next to them. **Worker minimum: 8 GB RAM works** (measured:
  about 0.25 GB for the idle daemon, about 0.5 GB at the peak of `join` and `verify`; the rest is for builds and the inotify slab of about
  130 MiB per tree); more for every extra replica tree.
- **Replica.** The shared tree is about 270k files and 5 GB. A replica fits the smallest 128 GB disk. The disk
  is pressured by ignored data, not by the replica: the build partitions in `.cache/b`, Cargo and Bun outputs and
  `node_modules`. Run `repo preflight` on the 128 GB machines and size their cache budgets accordingly. **Worker minimum: 128 GB
  NVMe** (measured: the tree is 4.5 GiB; the fleet preflight fails below 10 GiB free and warns below 30 GiB). A bootstrap
  needs another 5 GB of free space for the local copy of the archive while it is extracted. Such a machine is a worker, never a group server.
- **Store.** The store is the blobs plus the journal.
  - Blobs are whole files, stored once per content hash, never deltas. The first seed adds about 5 GB, minus
    duplicate contents. Every later save of a changed file adds one new blob of that file's size.
  - The journal is one JSON line per change, roughly 200 to 400 bytes. The seed alone is about 270k lines, under
    100 MB.
  - The bootstrap archive is one more copy of the shared tree (about 5 GB) per group store.
  - Blobs and sealed segments older than 14 days (`replica.retentionDays`) are dropped by `compact` when no snapshot needs
    them. Steady state is the live tree plus 14 days of churn. **Per group server: 0.5 TB (agents work 8 h a day) to 1.5 TB (round the
    clock) of blobs, plus 10 to 31 GB of journal, the tree (5 GB), the two newest snapshots per origin (up to 9 GB) and the bootstrap
    archive (5 GB). Recommendation: a disk of at least 2 TB.** Basis: measured per operation (11.3 KB of blobs and 258 bytes of
    journal per operation in the 50-machine rehearsal) times the rate of 100 operations per second for the whole fleet, so the totals
    are extrapolations (`scale-report.md`), not measurements of a fleet. A 2 TB disk is enough for 8 h a day with room to spare and
    tight for round-the-clock work (1.55 TB plus the 0.2 TB cache directory below is 88 % of 2 TB): shorten `replica.retentionDays`
    (7 days roughly halves the blobs) or give the server a larger disk. Watch the real growth for two weeks and adjust. Because
    every store holds all groups' data, the figure does not shrink with the group size. The fleet's own machines have 128 GB to 2 TB
    of NVMe, so the machine of a group that has the 2 TB disk is the group server (Day 0, step 6).
  - Use local SSD or NVMe on the group server, not another network file system. Back it up.
  - **Compaction cadence.** Every origin compacts daily (workers and servers: systemd timer with up to 3 h random delay on
    Linux, 03:30 as LaunchDaemon on macOS and as Scheduled Task on Windows; installed by the provisioning scripts), and the group
    server's daily job is `compact --collect-blobs`. Daily is a limit, not a taste: the blob collector keeps every doomed file in memory
    while it runs, which the rehearsal extrapolates to 0.6 to 1.7 GB for one day of garbage and 4 GB for a week. A worker compaction
    takes 2 to 7 s on the real tree (peak 460 MB), a collection a few minutes to 30 minutes on a day of garbage.
- **Poll load.** Each daemon reads the `head.json` of every origin every 250 ms. With 50 origins that is 200
  reads per second of a file of about 100 bytes per machine, and 2000 per second for a group of ten. That is
  trivial for a server with NFSv4 and the page cache, but the NFS caches matter. Measured on Linux (kernel 6.8, NFS 4.2):
  `head.json` is replaced by a rename; with the mount defaults a `stat` of the new head came late (only 46 of 100 versions were
  ever seen, up to 2.3 s late), with `actimeo=1,lookupcache=positive` after 505 ms median and 960 ms p95, and with
  `lookupcache=none` after 10 ms. The Linux mount options therefore use `actimeo=1,lookupcache=none`; end to end a save on the
  server appears on a worker in 305 ms median (p95 424 ms), against 696 ms (p95 1,173 ms) with `lookupcache=positive`. The price
  is about four `LOOKUP` calls per head per poll (32 RPC/s for two origins polled every 250 ms, so about 800 RPC/s for a machine
  that follows 50 origins). On macOS the same poll costs 65 NFS calls per second for four origins, so about 800 per second for 50 origins
  (measured, table "Client caches"). Windows is not measured: confirm that a save appears elsewhere within about a second before you rely on it. The Windows
  settings of the table "Client caches" are by reasoning, not measured.
- **Latency (measured).** A save appears on another machine of the same group after about 0.3 to 0.5 s (Linux NFS 4.2 with the mount
  options above: median 305 to 388 ms, p95 424 to 535 ms; macOS rehearsal with 50 daemons on local stores: median 343 ms, p95 696 ms; **macOS 27 against a Linux server over real NFS 4.0, 200 saves each way
  (`mixed-os-report.md`): Linux to macOS median 324 ms (p95 436 ms), macOS to Linux median 347 ms (p95 477 ms), none overtaken**) and on a machine of another group
  after about 0.8 to 1 s (**two Linux group servers over real NFS, 200 saves: group 1 to a worker of group 2 median 839 ms, p95 1,318 ms; group 2 to group 1 median 1,000 ms, p95 1,475 ms**;
  the relay polls once a second, so a copy waits up to one second and a half). The peer mount must have the plan options: with the NFS defaults the relay saw a replaced head
  up to 3 s late, the median was 889 ms, p95 2,944 ms and 91 of 200 saves were overtaken before they were copied).
  Files of 14 to 48 MB take 0.7 s in the group and 1.1 s across groups at the median. A host at load or a store that was away shows up in the tail first
  (p95 of 5 s while a daemon catches up, up to 30 s after a 30 s store outage).
- **Writes.** A save writes a blob, appends to one segment and replaces one `head.json`. Large bursts (a
  generated tree committed by a tool, a dependency update) are many small files; expect the burst to take
  seconds to minutes, not milliseconds.
- **Relay traffic.** Every blob crosses the links between groups once per other group. Peak traffic is the
  size of the changes times four. Keep the cross-group links well above the sustained change rate.
- **Cache server.** It stores build results of its group only; the default store is 2.5 GiB of a 3 GiB
  budget, which is too small for ten machines. The generated unit sets `--max-bytes` to 200 GiB, so put the store
  on its own NVMe volume of at least 250 GB (`/var/lib/semio-remote-cache`) and adjust the number in the topology
  module if your volume differs. Watch `repo remote-cache status` for the hit rate and evictions and grow the
  volume if recent entries are evicted before they are reused. Entries are limited to 512 MiB by default.
- **Server memory and cores.** The NFS server needs memory for the page cache of the active tree (a few GB
  beyond the store's hot set) and a few cores. The relay and the cache server are light on CPU. Recommendation: 8
  cores, 32 GB RAM for a group server, 10 GbE if the group network has it.

## What breaks the model

- A worker at a non-canonical path builds, but never gets cache hits from other machines.
- A path ignored too late: a new generated or machine-local file that the root `.gitignore` does not ignore is
  replicated to everyone. Fix the rule first; then delete the file, which also replicates.
- Git mutations on a worker. `.git/` is not shared, so a worker's git state is meaningless to the fleet.
- Reusing an origin id with a new journal, or running `seed` twice.
- A clock more than 60 s ahead of the others.
- Expecting the keystroke order across machines: two writes to one file within the publish latency are ordered by
  publish time (see "Two writers of one file").
- A Windows NFS mount of the store under a name with non-ASCII characters. The store only uses ASCII names; keep
  `storeMount` ASCII too.

## Not verified

The Windows provisioning script ran end to end under PowerShell 7 against a mock layer (`🗺️topology/🧪️tests`, skipped without `pwsh`): the parser accepts it and every generated snippet, the scheduled task XML validates against the published Task Scheduler schema (set `SEMIO_TASK_SCHEMA` to the schema file and the test runs `xmllint` against it), and the recorded cmdlet calls follow the promised order for the NFS and SMB variants, first, joining and repeated runs. No Windows machine ran it, so the real cmdlets, executables and services stay unverified (the operator checklist is in `windows-ci-report.md` of the ticket folder). Nothing else here was run. The server setups for macOS and Windows, the SMB fallback, the inotify step and the repeated run were checked
only statically on a macOS host (see `provisioning-review.md` in the ticket folder for the list that could not be executed):
`nfsd` and its `exports` syntax with two lines for one folder, the NFS version that `nfsd` serves, `New-NfsShare` and
`Grant-NfsSharePermission` precedence between the all-machines entry and the per-peer read-only entries, `cmdkey` with a
local account name, scheduled tasks at logon on a server, the long-path setting for every tool that opens the tree.

Verified on Linux (Ubuntu 24.04 arm64 VMs on a macOS host, kernel NFS 4.2 server and client; figures and commands in
`linux-nfs-report.md` of the ticket folder): the provisioning script as server and as worker, the exports, the fstab block and
line, the sysctl file, the systemd units (`systemd-analyze verify` clean) and the timer, the cache server and the relay under
systemd, `/usr/local/bin/repo` as a symlink to the per-account launcher under systemd, the bootstrap archive through the NFS
mount (sha256, `tar -xf`, file count), `bootstrap-settle` followed by `join` on a real extracted tree, seed, join, run,
verify, an NFS outage of 30 s and `kill -9` of a daemon in the middle of a publish.

Preflight and the macOS and Windows client caches: `preflight` was run against injected probe outputs (Linux, macOS, Windows, worker and
group server, every verdict) and once on a macOS machine with the real system calls; the real outputs of `mount` and `nfsstat -m` of macOS 27 and of `/proc/self/mountinfo` are parsed correctly (see below), those of `net use`, `reg query` and
`schtasks` were not parsed yet. The Windows SMB and NFS client settings are by reasoning (see "Client caches"); the Windows NFS
registry value names in particular were not verified on a Windows machine.

Verified between systems (`mixed-os-report.md` of the ticket folder): a macOS 27 machine as an NFS 4.0 client of a Linux group server (an unprivileged mount, the
plan's options, latency both ways, bursts, NFC and NFD names, case-only renames, executable bits, symlinks, a 30 s server stop, idle cost, the real
`mount` and `nfsstat -m` output in `preflight`), two Linux group servers with real peer mounts (relay both ways over NFS, a 30 s stop of one server, `kill -9` of the
relay in the middle of a copy, the `--archive` peer flow), and the native bootstrap on fresh Linux machines up to its last step: `cargo` passes once the C toolchain
that the provisioning script lists is installed (without `cc` the pinned `cargo build` exits 101 with ``linker `cc` not found``), but `repo run workspace:setup`
fails in the resident graph service on every machine tried (first `Cannot find module 'typescript'`, then `Original captured runtime entry provider required`),
so the completion marker is only written by hand, see the report.

Still unverified: a macOS group server and NFS 3 for a macOS or Windows server, a macOS worker whose mount is made by the LaunchDaemon (the tests mounted by hand, unprivileged, with
`noresvport` against an export with `insecure`), the other distributions' package managers, the figures under the production load, the tar writer against bsdtar on
Windows with the real tree, the native bootstrap as a service account on macOS, the scheduled tasks and their XML on Windows, the Windows NFS and SMB client settings,
and `compact` that removes segments or blobs while a daemon runs (on Linux it ran next to the daemons and found nothing to remove).
