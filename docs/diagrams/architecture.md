# RDTR Architecture Overview

```mermaid
flowchart LR
    Server -->|Binary Protocol (QUIC)| Engine
    Engine --> LayoutResolver
    Engine --> OverlayStack
    Engine --> WasmRuntime

    WasmRuntime -->|MutationOps| Engine
    LayoutResolver --> FrameBuffer
    OverlayStack --> FrameBuffer

    FrameBuffer --> DiffEngine
    DiffEngine --> Terminal
```

### Description

- Server sends layout + WASM
- WASM produces validated mutation operations
- Engine resolves layout deterministically
- Overlay stack applied
- Frame buffer generated
- Diff engine computes minimal terminal updates

---

## 1.2 Rendering Pipeline

`docs/diagrams/render-pipeline.md`

# Rendering Pipeline

```mermaid
flowchart TD
    A[Base Layout Tree]
    B[Overlay Stack]
    C[Terminal Size]

    A --> D[Layout Resolution]
    B --> D
    C --> D

    D --> E[Frame Buffer]
    E --> F[Diff Algorithm]
    F --> G[Terminal Output]
```

All mutations must pass through the engine validator.

---

## 1.4 Overlay Stack Model

`docs/diagrams/overlay-stack.md`

```md
# Overlay Stack Model

```mermaid
flowchart TD
    Base[Base Layout]
    O1[Overlay 1]
    O2[Overlay 2]

    Base --> Frame
    O1 --> Frame
    O2 --> Frame

    Frame --> Terminal
```

Overlay resolution rules:

- Stack-based
- Topmost overlay receives focus
- Geometry computed by engine

---

## 1.5 Binary Protocol Lifecycle

`docs/diagrams/protocol-lifecycle.md`

```md
# Protocol Lifecycle

```mermaid
sequenceDiagram
    participant Client
    participant Server

    Client->>Server: Connect (QUIC)
    Server-->>Client: LayoutTree + WASM
    Client->>Client: Instantiate Runtime

    loop Interaction
        Client->>Server: EventFrame
        Server-->>Client: MutationFrame
    end
```
