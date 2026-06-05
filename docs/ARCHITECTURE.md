# Arquitetura GHOSTWALL

## Visão Geral

GHOSTWALL segue uma arquitetura modular em camadas, permitindo escalabilidade e manutenibilidade.

## Camadas

### 1. Core Layer (`core/`)
Define tipos e estruturas compartilhadas:
- `Device` - Representação de dispositivo RF
- `RiskScore` - Sistema de scoring de risco
- `ExposureScore` - Sistema de scoring de exposição
- `Fingerprint` - Assinaturas comportamentais
- `Error` - Tratamento de erros

### 2. Scanner Layer (`scanner/`)
Responsável pela descoberta de dispositivos:
- `DiscoveryEngine` - Descoberta de dispositivos
- `FingerprintEngine` - Análise comportamental
- Platform-specific: Linux (iw/nl80211), macOS (CoreWLAN), Windows (Npcap)

### 3. Risk Engine Layer (`risk-engine/`)
Implementa regras RSIE:
- Avaliação de risco por dispositivo
- Avaliação de cluster
- Geração de recomendações

### 4. Storage Layer (`storage/`)
Persistência de dados:
- SQLite para armazenamento local
- `ExposureScanner` - Cálculo de exposição
- Migrations automáticas

### 5. Threat Intel Layer (`threat-intel/`)
Banco de assinaturas:
- Assinaturas versionadas
- Atualização remota
- Export/Import JSON

### 6. Reporting Layer (`reporting/`)
Geração de relatórios:
- JSON, HTML, PDF
- Inventário completo
- Recomendações automáticas

### 7. API Layer (`api/`)
REST API com Axum:
- Endpoints para todas as operações
- CORS habilitado
- Logging estruturado

### 8. Frontend Layers

#### Desktop (`dashboard/`)
- Tauri + React + TypeScript
- TailwindCSS
- Páginas: Overview, Devices, Exposure, Threats, Reports, Settings

#### Mobile (`mobile/`)
- Flutter
- Provider pattern
- Conexão com API

## Fluxo de Dados

```
[Scanner] → [Storage] → [Risk Engine] → [Threat Intel] → [Reporting]
     ↓           ↓            ↓               ↓              ↓
   Devices   Database    Risk Scores   Signatures     Reports
```

## Segurança

### Criptografia
- AES-256-GCM para dados em repouso
- Ed25519 para assinaturas
- SHA-256 para hashing

### Privacidade
- Processamento local por padrão
- Consentimento explícito para comunicação externa
- TLS 1.3 para comunicação

## Performance

### Otimizações
- Cache de vendor OUI
- Pool de conexões SQLite
- Async/await em Rust
- Lazy loading no frontend

## Escalabilidade

### Horizontal
- API pode ser escalada via Kubernetes
- Docker containers para deployment

### Vertical
- Módulos independentes podem ser otimizados
- SQLite pode ser substituído por PostgreSQL
