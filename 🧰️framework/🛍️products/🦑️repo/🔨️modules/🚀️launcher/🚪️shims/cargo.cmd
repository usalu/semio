@echo off
rem 🚪️ The managed cargo entry for Windows: every compiling subcommand goes through the repository's Cargo ingress gate, everything else through the tool registry.
setlocal
if not defined SEMIO_CARGO_STORE set SEMIO_CARGO_STORE=isolated
if not defined SEMIO_BUILD_MODE set SEMIO_BUILD_MODE=prototype
for %%c in (build b check c test t run r bench clippy doc d rustc rustdoc fix miri install) do if "%~1"=="%%c" (
  repo cargo --store=%SEMIO_CARGO_STORE% --configuration=%SEMIO_BUILD_MODE% -- %*
  exit /b %ERRORLEVEL%
)
repo tools exec cargo -- %*
exit /b %ERRORLEVEL%
