# Architecture Overview

Rust workspace composed of several focused crates, wired together to control OBS, record video, and upload finished recordings to cloud storage (using S3).

## Data Flow

```mermaid
flowchart TD
    subgraph App[obs-recorder Application]
        CLI[CLI Layer\n(clap/argp)] --> CFG[Config Loader\n(YAML/TOML → RecordingConfig)]
        CFG --> Pipe[Pipeline Controller]

        subgraph RecordingSubsystem[Recording Subsystem]
            Pipe --> REC[Recorder Controller\n(state machine)]
            REC --> UPL[VideoUploader Trait\n(S3/GCS/Local)]
        end

        REC -->|start/stop cmds| RTx[mpsc::Sender<ObsCommand>]
    end

    subgraph Runtime[OBS Runtime Thread]
        RTx --> RT[mpsc::Receiver<ObsCommand>]

        RT --> ENG[ObsEngine\n(owns libobs-wrapper handles)]

        subgraph EngineLayers[ObsEngine Internals]
            ENG --> CTX[ObsContext\n(libobs core engine)]
            ENG --> VID[Video Module\n(VideoEncoderInfo,\nresolution,fps)]
            ENG --> AUD[Audio Module\n(AudioEncoderInfo)]
            ENG --> SRC[Sources Module\n(SourceInfo,\nObsData)]
            ENG --> OUT[Output Module\n(OutputInfo,\nffmpeg_muxer)]
        end
    end

    OUT --> FILE[Recorded File (MKV/MP4)]

    FILE --> UPL
