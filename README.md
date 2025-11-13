# obs-recorder

This is a new Rust project where I’m building a small tool that can talk to OBS (Open Broadcaster Software), start and stop recordings, and then upload the recordings to a cloud bucket (using S3 here). The goal is to learn Rust by having a clean setup with separate crates so each piece stays organized and easy to work on.

Right now ’m putting together the basic structure of the workspace, the core data types, and the traits that describe how recording and uploading should work.

## Current Goals

- Connect to OBS using obs-websocket.
- Start and stop recordings programmatically.
- Save recordings locally.
- Upload the finished files to an S3 bucket or another storage backend.
- Keep everything well-organized using a Rust workspace.

## Architecture

Drafted an architecture of what I think this is gonna look like see [ACHITECTURE.md](ARCHITECTURE.md).