<div align="center">

<img src="./logo.png" width="300" alt="GhostWall Logo"/>

# GhostWall

### Open Source RF Privacy Firewall

**Visibility • Auditing • Exposure Assessment for RF Sensing Environments**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Active%20Development-green.svg)]()
[![Platform](https://img.shields.io/badge/Platform-Cross--Platform-orange.svg)]()
[![Open Source](https://img.shields.io/badge/Open%20Source-Yes-success.svg)]()

</div>

---

## Overview

GhostWall is an Open Source RF Privacy Firewall designed to help individuals, researchers, security professionals and organizations understand their exposure to modern RF sensing technologies.

The project focuses on:

- RF visibility
- Wi-Fi sensing exposure assessment
- Device discovery
- Infrastructure auditing
- Risk scoring
- RF privacy awareness

GhostWall does **not** claim to detect espionage or identify specific proprietary systems.

Instead, it analyzes measurable indicators associated with infrastructures that may be compatible with Wi-Fi sensing and RF-based monitoring technologies.

---

## Why GhostWall?

Recent advances in wireless sensing technologies allow environments to be analyzed using radio signals.

Research in areas such as:

- Wi-Fi Sensing
- Channel State Information (CSI)
- Device-Free Sensing
- Radio Tomographic Imaging
- RF Fingerprinting

has demonstrated that movement, occupancy and environmental changes can be inferred from wireless signals.

GhostWall was created to provide transparency, visibility and risk assessment for these environments.

---

## Project Status

⚠️ Experimental Research Project

GhostWall is currently under active development.

Features, detection models and scoring algorithms may evolve as research progresses.

| Feature | Status |
|----------|---------|
| Dashboard | ✅ Implemented |
| Device Inventory | ✅ Implemented |
| RSIE Engine | ✅ Implemented |
| Alert Center | ✅ Implemented |
| Reporting Engine | ✅ Implemented |
| RF Exposure Scoring | ✅ Implemented |
| Threat Intelligence | 🚧 In Progress |
| RF Discovery Engine | 🚧 In Progress |
| RF Fingerprinting | 🚧 In Progress |
| Router Agent (OpenWRT) | 📋 Planned |
| Mobile Application | 📋 Planned |
| Community Signatures | 📋 Planned |

---

# Core Principles

GhostWall is built around four principles:

### Visibility

Understand what RF-capable devices exist within an environment.

### Transparency

Provide evidence-based analysis rather than assumptions.

### Privacy

Keep data processing local whenever possible.

### Research

Advance understanding of RF sensing exposure through open collaboration.

---

# What GhostWall Does

### RF Discovery

Identify devices visible within the environment.

Examples:

- Access Points
- ESP32 Devices
- Raspberry Pi Systems
- Wireless IoT Devices
- RF Infrastructure Components

---

### RF Exposure Assessment

Evaluate environmental characteristics that may facilitate RF sensing.

Examples:

- Coverage density
- Device placement
- RF propagation conditions
- Infrastructure concentration

---

### RSIE Engine

RF Sensing Indicators of Exposure.

GhostWall evaluates indicators associated with infrastructures commonly used in RF sensing research.

Examples:

- CSI-compatible hardware
- Multi-node deployments
- Continuous operation patterns
- MQTT telemetry
- UDP telemetry flows

---

### Device Inventory

Maintain a centralized inventory of observed devices.

Collected information may include:

- MAC Address
- Vendor
- RSSI
- Channel
- Device Type
- Risk Indicators

---

### Reporting Engine

Generate professional reports in:

- HTML
- JSON
- PDF (planned)

Reports include:

- Exposure Scores
- Risk Indicators
- Device Inventory
- Recommendations

---

### Threat Intelligence

Local and community-driven RF signature database.

Supports:

- Signature Versioning
- Integrity Validation
- Community Research Contributions

---

# RSIE Rules

GhostWall uses the RF Sensing Indicators of Exposure framework.

Current baseline indicators include:

| Rule | Description |
|--------|------------|
| GW-RF-001 | CSI-compatible hardware detected |
| GW-RF-002 | Multiple compatible devices present |
| GW-RF-003 | Continuous operational profile |
| GW-RF-004 | High-frequency UDP telemetry |
| GW-RF-005 | MQTT telemetry patterns |
| GW-RF-010 | Potential sensing cluster characteristics |
| GW-FP-001 | False positive reduction |

These indicators contribute to overall risk and exposure scoring.

---

# Architecture

```text
Dashboard
    │
    ▼
REST API
    │
    ▼
RSIE Engine
    │
    ├── Device Inventory
    ├── Threat Intelligence
    ├── Reporting Engine
    └── Exposure Assessment
```

---

# Technology Stack

## Frontend

- React
- TypeScript
- Vite

## Backend

- Node.js
- Express

## Database

- Local Storage Layer
- Drizzle ORM

## Future Stack

- Rust Core Engine
- OpenWRT Agent
- Mobile Client
- PostgreSQL Support

---

# Screenshots

## Dashboard

Coming Soon

---

## Exposure Map

Coming Soon

---

## Device Inventory

Coming Soon

---

# Installation

## Clone Repository

```bash
git clone https://github.com/NaiCodeBr/ghostwall.git
cd ghostwall
```

## Install Dependencies

```bash
npm install
```

## Start Development Environment

```bash
npm run dev
```

Application will be available at:

```text
http://localhost:3000
```

---

# Security Philosophy

GhostWall follows a privacy-first approach.

Goals:

- Local-first processing
- Minimal data collection
- Transparent analysis
- Open-source reviewability

Cryptographic protections and secure communications will continue to evolve as the platform matures.

---

# Non Goals

GhostWall is NOT:

- A Wi-Fi jammer
- A radio interference tool
- A surveillance platform
- A proof-of-espionage detector
- A CSI collection framework
- A signal disruption system
- A tool to identify specific proprietary platforms

GhostWall focuses on visibility and exposure assessment.

---

# Roadmap

## Version 0.1

- Dashboard
- RSIE Engine
- Device Inventory
- Reporting

## Version 0.5

- Threat Intelligence
- Community Signatures
- Enhanced Discovery

## Version 1.0

- OpenWRT Agent
- Mobile Applications
- Distributed Analysis
- Advanced RF Exposure Assessment

---

# Contributing

Contributions are welcome.

Areas of interest:

- RF Research
- Wireless Networking
- Security Engineering
- Privacy Technologies
- Open Source Development

Please review:

- CONTRIBUTING.md
- SECURITY.md
- CODE_OF_CONDUCT.md

before submitting pull requests.

---

# Research References

The project is inspired by publicly available research and open-source initiatives in:

- Wi-Fi Sensing
- Channel State Information (CSI)
- RF Fingerprinting
- Device-Free Sensing
- Wireless Environmental Monitoring

Examples:

- https://github.com/ruvnet/RuView
- https://github.com/StevenMHernandez/ESP32-CSI-Tool
- https://github.com/seemoo-lab/nexmon_csi

GhostWall is an independent project and is not affiliated with any of the above initiatives.

---

# License

Released under the MIT License.

See LICENSE for details.

---

<div align="center">

### GhostWall

Open Source RF Privacy Firewall

Built for visibility, transparency and RF privacy research.

</div>
