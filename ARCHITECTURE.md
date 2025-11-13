# Architecture Overview

Rust workspace composed of several focused crates, wired together to control OBS, record video, and upload finished recordings to cloud storage (using S3).

## Data Flow

```mermaid
flowchart TD
    subgraph App["obs-recorder Application"]
        CLI["CLI Layer (clap/argp)"] --> CFG["Config Loader (YAML/TOML -> RecordingConfig)"]
        CFG --> Pipe["Pipeline Controller"]

        subgraph RecordingSubsystem["Recording Subsystem"]
            Pipe --> REC["Recorder Controller (state machine)"]
            REC --> UPL["VideoUploader Trait (S3/GCS/Local)"]
        end

        REC -->|start/stop commands| RTx["mpsc::Sender<ObsCommand>"]
    end

    subgraph Runtime["OBS Runtime Thread"]
        RTx --> RT["mpsc::Receiver<ObsCommand>"]

        RT --> ENG["ObsEngine (owns libobs-wrapper handles)"]

        subgraph EngineLayers["ObsEngine Internals"]
            ENG --> CTX["ObsContext (libobs core engine)"]
            ENG --> VID["Video Module (VideoEncoderInfo, resolution, fps)"]
            ENG --> AUD["Audio Module (AudioEncoderInfo)"]
            ENG --> SRC["Sources Module (SourceInfo, ObsData)"]
            ENG --> OUT["Output Module (OutputInfo, ffmpeg_muxer)"]
        end
    end

    OUT --> FILE["Recorded File (MKV/MP4)"]
    FILE --> UPL
