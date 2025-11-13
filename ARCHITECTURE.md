# Architecture Overview

Rust workspace composed of several focused crates, wired together to control OBS, record video, and upload finished recordings to cloud storage (using S3).

## Data Flow

```mermaid
flowchart TD

    CLI[CLI → Domain Types] --> CFG[Config Loader]
    CFG --> PIPE[Pipeline]

    PIPE --> REC[Recorder Controller]
    REC --> TX[ObsCommand Sender]

    TX --> RT[Runtime Thread]
    RT --> ENG[ObsEngine]

    ENG --> CTX[Context]
    ENG --> ENC[Encoders]
    ENG --> CAP[Capture Sources]
    ENG --> OUT[Output]

    OUT --> FILE[Video File]
    FILE --> UP[Uploader]
