# Architecture Overview

Rust workspace composed of several focused crates, wired together to control OBS, record video, and upload finished recordings to cloud storage (using S3).

## Data Flow

```mermaid
flowchart TD

    CLI[CLI Layer (clap and argp)] --> CFG[Config Loader - RecordingConfig]
    CFG --> PIPE[Pipeline Controller]
    PIPE --> REC[Recorder Controller - State Machine]
    REC --> UPL[VideoUploader (S3, GCS, Local)]
    REC --> TX[ObsCommand Sender]

    TX --> RT[OBS Runtime Thread]
    RT --> ENG[ObsEngine - uses libobs-wrapper]

    ENG --> CTX[ObsContext - OBS Core Engine]
    ENG --> VID[Video]()
