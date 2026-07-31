# os

os is a cooperative pseudo operating system with version control, collaboration, plugin-mechanisms, renderers and kernels built in.

[s](`s/AGENTS.md`) is one concrete os instance for the design domain.

# 🔌 Plugin

A plugin is a manifest and a collection of apps.

# 🎛️ App

An app has a engine, 

# 🛂 Manifest

A manifest defines schemas, commands (along with cmd and cde), 

# Schema

A schema for an artifact with definitions.

# Definition

A definition for an entity.

# Entity

An entity inside a artifact.

# Format

A format is a schema for storing artifacts in a handcrafted domain specific language text.

# Protocol

A protocol is a binary schema for storing messages.

# Engine

A stateful headless computational engine with bidirectional streaming.

The engine maintains a pack buffer for the artifact. Alternative versions of the artifact are computed on the fly by materializing patches.

# Command

A command send to the engine.

# Cde

A cde is a native binary representations for a commands.

Cdes are used for communication and storage.

# Cmd

An cmd is a native text representation for commands.

Cdes are used for logging and llms.

# Operation

An operation is a command that modifies a artifact.

# Patch

A patch is a protocol that modifies a artifact.

# Op

An op is protocol that modifies a artifact.

# Document

A artifact is the data for an app.

# Pack

A pack is a binary representation of a artifact.

# Dsl

A dsl is a textual representation of a artifact.

# Space

A space is the ultimate version controlled container for artifacts.

# 🗿 Artifact

An artifact is 

## Draft

A draft is a volatile artifact.

## Asset

An asset is a persisted artifact (with optional time to live).

# User

## Author

An author is a user with write permission.

## Specator

A spectator is a user with only read permission.