# RDTR v0.1 Specification
Remote Declarative Terminal Runtime
Status: Draft
Generated: 2026-02-22T19:46:15.171470 UTC

---

## 1. Overview

RDTR defines a deterministic, declarative, keyboard-first terminal UI runtime.

It provides:
- Layout Tree Model
- Overlay System
- Mutation Model
- WASM Sandbox Execution
- Binary Transport Protocol
- Deterministic Rendering Guarantee

RDTR intentionally excludes:
- Styling systems (no CSS equivalent)
- Arbitrary coordinate rendering
- Mouse input (v0)

---

## 2. Determinism Rule

Final frame MUST be a pure function of:

(BaseLayoutTree, OverlayStack, TerminalSize)

---

## 3. Layout Model

Each node:

Node {
  id: u32,
  kind: NodeKind,
  properties: PropertyMap,
  children: Vec<Node>
}

NodeKind:
- Container
- Text
- Input
- Button
- List
- Spacer

Containers use constraint-based layout (grow/shrink model).
Absolute positioning is NOT permitted in base layout.

---

## 4. Overlay System

Overlays form a separate stack rendered above the base layout.

Overlay {
  id: u32,
  anchor: Option<NodeId>,
  position: Center|Top|Bottom|Left|Right|Above|Below|LeftOf|RightOf,
  content: Node
}

Overlays do not alter base layout geometry.

---

## 5. Mutation Model

WASM returns MutationOps:

- ReplaceNode
- RemoveNode
- InsertChild
- UpdateText
- UpdateProperty
- SetFocus
- OpenOverlay
- CloseOverlay

Engine MUST validate tree integrity.

---

## 6. WASM Model

- One module per session
- Sandboxed
- No filesystem access
- No raw network access
- All side effects through host API

---

## 7. Rendering Model

1. Resolve layout
2. Render to 2D char buffer
3. Composite overlays
4. Diff previous frame
5. Emit minimal ANSI updates

---

## 8. Transport

Binary, length-prefixed messages.
Encrypted transport required (e.g., QUIC/TLS).
