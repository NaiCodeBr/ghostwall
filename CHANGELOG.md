# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of GHOSTWALL RF Privacy Firewall
- Module 1: RF Discovery Engine with platform-specific implementations
- Module 2: RF Fingerprint Engine for behavioral signatures
- Module 3: RSIE Engine with 7 risk scoring rules
- Module 4: RF Exposure Scanner (0-100 scoring)
- Module 5: Threat Intelligence database
- Module 6: Report Engine (JSON, HTML, PDF)
- REST API with 8 endpoints
- Desktop Dashboard (Tauri + React + TypeScript)
- Mobile App (Flutter)
- Router Agent for continuous monitoring
- SQLite local database
- Docker configuration
- GitHub Actions CI/CD pipeline
- Comprehensive documentation

### Security
- AES-256-GCM encryption for data at rest
- Ed25519 digital signatures
- SHA-256 hashing
- TLS 1.3 for communication
- Local-only processing by default
- No data sent without explicit consent

### Documentation
- README with complete project overview
- Architecture documentation
- API reference
- Threat model
- Installation guide
- Roadmap
- Contributing guidelines
- Code of conduct
- Security policy
- Support guide

## [0.1.0] - 2024-01-01

### Added
- Initial project structure
- Core types and utilities
- RF Discovery Engine
- Risk Engine
- Storage layer
- Threat Intelligence
- Reporting Engine
- REST API
- Desktop Dashboard
- Mobile App
- Docker configuration
- CI/CD pipeline
- Documentation

### Security
- Implemented encryption standards
- Privacy-preserving design
- Local processing

### Documentation
- Initial documentation set

---

## Versioning

For the versions available, see the [tags on this repository](https://github.com/NaiCodeBr/ghostwall/tags).
