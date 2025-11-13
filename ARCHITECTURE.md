# Architecture Overview

Rust workspace composed of several focused crates, wired together to control OBS, record video, and upload finished recordings to cloud storage (using S3).

## High-Level Data Flow

```mermaid
flowchart LR
    subgraph CLI["cli crate (binary)"]
        CLI_ARGS["Command-line args (clap)"]
    end

    subgraph APP["app crate"]
        SERVICE["AppContext / Service\n(record_and_upload, etc.)"]
    end

    subgraph CORE["core crate"]
        CFG["Config types\n(AppConfig, ObsConfig,\nRecordingConfig, StorageConfig)"]
        DOMAIN["Domain types\n(RecordingProfile, FrameRate,\nResolution, RecorderState,\nStorageBackend, etc.)"]
        TRAITS["Traits\nRecorder, Uploader"]
        ERRORS["Error enums\nAppError, UploadError"]
    end

    subgraph OBSCLIENT["obs-client crate"]
        OBSREC["ObsRecorder\nimpl Recorder"]
    end

    subgraph S3["storage-s3 crate"]
        S3UP["S3Uploader\nimpl Uploader"]
    end

    subgraph EXT_OBS["External: OBS"]
        OBS["OBS Studio\n+ obs-websocket"]
    end

    subgraph EXT_S3["External: Cloud Storage"]
        BUCKET["S3 Bucket\n(or S3-compatible)"]
    end

    CLI_ARGS -->|"parse & load config"| CFG
    CFG --> SERVICE
    DOMAIN --> TRAITS
    TRAITS --> SERVICE

    SERVICE -->|"uses dyn Recorder"| OBSREC
    SERVICE -->|"uses dyn Uploader"| S3UP

    OBSREC -->|"obs-websocket"| OBS
    S3UP -->|"PutObject etc."| BUCKET
