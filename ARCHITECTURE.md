# Architecture Overview

Rust workspace composed of several focused crates, wired together to control OBS, record video, and upload finished recordings to cloud storage (using S3).

## High-Level Data Flow

```mermaid
flowchart LR
    subgraph CLI["cli crate (binary)"]
        CLI_ARGS["Command-line args (clap)"]
    end

    subgraph APP["app crate"]
        SERVICE["AppContext / Service(record_and_upload, etc.)"]
    end

    subgraph CORE["core crate"]
        CFG["Config types (AppConfig, ObsConfig, RecordingConfig, StorageConfig)"]
        DOMAIN["Domain types (RecordingProfile, FrameRate,Resolution, RecorderState,StorageBackend, etc.)"]
        TRAITS["TraitsRecorder, Uploader"]
        ERRORS["Error enumsAppError, UploadError"]
    end

    subgraph OBSCLIENT["obs-client crate"]
        OBSREC["ObsRecorderimpl Recorder"]
    end

    subgraph S3["storage-s3 crate"]
        S3UP["S3Uploaderimpl Uploader"]
    end

    subgraph EXT_OBS["External: OBS"]
        OBS["OBS Studio+ obs-websocket"]
    end

    subgraph EXT_S3["External: Cloud Storage"]
        BUCKET["S3 Bucket(or S3-compatible)"]
    end

    CLI_ARGS -->|"parse & load config"| CFG
    CFG --> SERVICE
    DOMAIN --> TRAITS
    TRAITS --> SERVICE

    SERVICE -->|"uses dyn Recorder"| OBSREC
    SERVICE -->|"uses dyn Uploader"| S3UP

    OBSREC -->|"obs-websocket"| OBS
    S3UP -->|"PutObject etc."| BUCKET
