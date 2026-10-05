# Sewline

**Compliance-Aware SDLC Orchestration Engine for High-Integrity Systems**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](#)
[![Compliance Packs](https://img.shields.io/badge/Compliance-DO--178C%20%7C%20AS9100-indigo.svg)](#)

---

## 📌 Overview

**Sewline** is the missing orchestration and compliance layer for high-integrity software engineering (aerospace, defense, automotive, medical, and regulated finance). It does not replace existing tools like GitHub Actions, Parasoft, or Jama; instead, it **orchestrates, gates, and unifies** them.

By converting regulatory guidelines (DO-178C, AS9100, SOC 2, FedRAMP, CMMC) into executable Policy-as-Code gates, Sewline automatically generates signed, tamper-evident **DSSE (Dead Simple Signing Envelope) attestations** at every stage of the software lifecycle while maintaining full traceability in an embedded graph database.

---

## 📐 Architecture Diagram

```text
                                  +-------------------------------------------------+
                                  |                 USER INTERFACE                  |
                                  |     React + Tailwind Visualization Dashboard     |
                                  +------------------------+------------------------+
                                                           |
                                                      HTTP / gRPC
                                                           |
                                                           v
+-------------------------------------------------------------------------------------------------------------------+
|                                                 SEWLINE ENGINE                                                   |
|                                                                                                                   |
|  +-----------------------------------+     +----------------------------------+     +--------------------------+  |
|  |           Workflow DSL            |     |         Gate Engine              |     |  Context Store           |  |
|  |  Declarative YAML/JSON Pipeline   |---->|  - OPA / Rego Policy Evaluator   |---->|  - Kùzu / Lineage Graph  |  |
|  |  Conditional Branches & Parallel  |     |  - DSSE Attestation Generator    |     |  - Traceability Matrix   |  |
|  +-----------------+-----------------+     +-----------------+----------------+     +--------------------------+  |
|                    |                                         |                                                    |
|                    v                                         v                                                    |
|  +----------------------------------------------------------------------------+                                   |
|  |                              STAGE EXECUTOR                                |                                   |
|  |                       Plugin Adapter Abstraction Layer                      |                                   |
|  +--------+----------------------------------+-----------------------+--------+                                   |
|           |                                  |                       |                                            |
+-----------|----------------------------------|-----------------------|--------------------------------------------+
            |                                  |                       |
            v                                  v                       v
+-----------------------+          +-----------------------+  +-----------------------+
|  GitHub Actions / CI  |          | Parasoft C/C++test    |  | Jama Connect          |
|  (Source & Sync)      |          | (Static Analysis)     |  | (Requirements Trace)  |
+-----------------------+          +-----------------------+  +-----------------------+

                                               ^
                                               | (gRPC / Governance)
                                               v
                                  +-------------------------+
                                  |   AGENT CONTROL PLANE   |
                                  | - Agent Scope Registry  |
                                  | - Human-in-the-Loop     |
                                  | - Immutable Audit Log   |
                                  +-------------------------+

```

---

## 🔑 Key Features

* **🔌 Plugin-Based Tool Adapters:** Unified `StageExecutor` trait allows declarative integration with standard tools (GitHub Actions, Parasoft, Jama, GitLab CI, Jenkins, VectorCast) without code changes.
* **🛡️ Executable Compliance Gates:** Enforce compliance standards (DO-178C DAL A–E, AS9100, SOC 2) as OPA/Rego policies inline with pipeline execution.
* **✍️ Tamper-Evident DSSE Attestations:** Automatically emits signed, content-addressed in-toto DSSE evidence attestations for every passed or failed stage gate.
* **🤖 Cross-Vendor Agent Governance:** Central control plane tracking AI agents (Copilot, Codex, custom LLMs) with fine-grained scoping, human-in-the-loop approvals, and immutable audit logs.
* **🕸️ Graph Context Store:** Built-in lineage graph tracking `Requirement ↔ Code ↔ Static Analysis ↔ Deployment ↔ Incident` relationships across pipeline executions.

---

## 🗂️ Project Layout

```text
sewline/
├── Cargo.toml                 # Workspace manifest
├── crates/
│   ├── sewline-core/          # Workflow engine, adapters, gate evaluator, graph store
│   ├── sewline-agent/         # Agent governance control plane (gRPC service & SQLite)
│   └── sewline-cli/           # CLI runner
├── ui/                        # Web visualization dashboard (React + TS + Tailwind)
├── compliance_packs/          # Pre-built Rego policy packs (DO-178C, AS9100)
├── deploy/                    # Kubernetes operator & multi-tenant CRDs
└── pipelines/                 # Reference pipeline definitions

```

---

## 🚀 Quick Start

### Prerequisites

* **Rust:** 1.75+
* **Node.js:** v18+ & `npm`
* **Protocol Buffers Compiler (`protoc`)**

### 1. Build and Run Engine Tests

Clone the repository and execute the workspace test suite:

```bash
git clone [https://github.com/your-org/sewline.git](https://github.com/your-org/sewline.git)
cd sewline

# Run core engine tests & end-to-end pipeline execution
cargo test --workspace

```

### 2. Start Agent Governance Control Plane

Launch the gRPC agent control plane service:

```bash
cargo run --bin sewline-agent

```

### 3. Launch Web Dashboard

In a separate terminal, navigate to the UI directory and start the development server:

```bash
cd ui
npm install
npm run dev

```

Open your browser at `http://localhost:5173` to interact with pipeline executions, gate statuses, and agent approvals.

---

## 📑 Example Pipeline Definition

Pipelines are written in simple, human-readable, and git-diffable YAML (`pipelines/do178c_sample_pipeline.yaml`):

```yaml
name: DO-178C-DAL-A-Flight-Control-Software
version: "1.0.0"
compliance_profile: DO-178C-DAL-A

stages:
  - id: checkout
    name: Source Code Sync
    adapter: adapter-github-actions
    params:
      repository: "aero-corp/flight-control-core"
      ref: "v1.4.0-rc1"

  - id: static_analysis
    name: Parasoft Static Code Analysis
    adapter: adapter-parasoft
    depends_on: ["checkout"]
    params:
      config: "builtin://MISRA_C_2012"
    gates:
      - id: gate_zero_misra_violations
        name: Zero MISRA C:2012 Violations
        policy_rule: sewline.parasoft.zero_violations
        enforce: true

```

---

## 📜 License

Core orchestration engine and CLI are licensed under the **[MIT License](https://www.google.com/search?q=LICENSE-MIT)**.
Compliance packs and enterprise modules are subject to their respective licenses.

```

---

### GitHub Description Metadata

To set up your GitHub repository page details:

* **Description:** *Compliance-aware SDLC orchestration engine for high-integrity systems (DO-178C, AS9100, SOC 2). Unifies dev tools, enforces policy-as-code gates, and generates DSSE attestations.*
* **Website:** `[https://sewline.dev](https://sewline.dev)` *(or docs link)*
* **Topics/Tags:** `sdlc-orchestration`, `rust`, `compliance-as-code`, `opa-rego`, `dsse-attestation`, `do-178c`, `agent-governance`, `devsecops`

```
