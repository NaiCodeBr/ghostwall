<div align="center">

![GHOSTWALL Banner](assets/logo.png)

# 🔒 GHOSTWALL

### Open Source RF Privacy Firewall

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg)]()

[Documentation](https://github.com/NaiCodeBr/ghostwall/tree/main/docs) | [API Reference](https://github.com/NaiCodeBr/ghostwall/blob/main/docs/API.md) | [Installation Guide](https://github.com/NaiCodeBr/ghostwall/blob/main/docs/INSTALLATION.md) | [Roadmap](https://github.com/NaiCodeBr/ghostwall/blob/main/docs/ROADMAP.md)

</div>

---

## 📋 Sobre o Projeto

**GHOSTWALL** é uma plataforma Open Source multiplataforma focada em auditoria, detecção, análise e monitoramento de infraestruturas compatíveis com Wi-Fi Sensing, Channel State Information (CSI), Radio Sensing e tecnologias relacionadas.

### ⚠️ Aviso Importante

Este sistema **NÃO** afirma detectar especificamente RuView ou qualquer solução proprietária. O sistema identifica indicadores técnicos compatíveis com arquiteturas de sensoriamento RF.

### 🎯 Missão

Construir o primeiro Firewall de Privacidade RF Open Source, capaz de:

- 🔍 **Descobrir dispositivos RF** - Mapeamento completo do ambiente RF
- 📡 **Identificar dispositivos compatíveis com CSI** - Detecção de hardware ESP32-S3, ESP32-C6, Broadcom
- 📊 **Avaliar risco de exposição RF** - Scoring 0-100 baseado em métricas
- 🎯 **Detectar padrões compatíveis com Wi-Fi Sensing** - Análise comportamental
- 📈 **Produzir score de risco e exposição** - RSIE Engine com 7 regras
- 📄 **Gerar relatórios técnicos** - JSON, HTML, PDF
- 💻 **Operar localmente** - Privacidade preservada
- 🔒 **Não depender de nuvem** - Funcionalidades principais offline

---

## 🏗️ Estrutura do Projeto

```
ghostwall/
├── .github/
│   └── workflows/           # CI/CD GitHub Actions
│       ├── ci.yml           # Pipeline de CI
│       └── release.yml      # Pipeline de Release
├── core/                    # Tipos e utilitários principais
│   ├── src/
│   │   ├── device.rs        # Estrutura de dispositivo RF
│   │   ├── risk.rs          # Sistema de scoring de risco
│   │   ├── exposure.rs      # Sistema de scoring de exposição
│   │   ├── fingerprint.rs  # Assinaturas comportamentais
│   │   └── error.rs         # Tratamento de erros
│   └── tests/               # Testes unitários
├── scanner/                 # Módulo 1: RF Discovery Engine
│   ├── src/
│   │   ├── discovery.rs     # Engine de descoberta
│   │   ├── linux.rs         # Implementação Linux (iw/nl80211)
│   │   ├── macos.rs         # Implementação macOS (CoreWLAN)
│   │   ├── windows.rs       # Implementação Windows (Npcap)
│   │   └── fingerprint.rs   # Módulo 2: RF Fingerprint Engine
│   └── Cargo.toml
├── risk-engine/             # Módulo 3: RSIE Engine
│   ├── src/
│   │   └── lib.rs           # Motor de scoring com 7 regras
│   └── tests/
├── storage/                 # Banco de dados e Módulo 4
│   ├── src/
│   │   ├── lib.rs           # Operações SQLite
│   │   ├── exposure.rs      # RF Exposure Scanner
│   │   └── schema.rs        # Schema do banco
│   └── Cargo.toml
├── threat-intel/            # Módulo 5: Threat Intelligence
│   ├── src/
│   │   └── lib.rs           # Banco de assinaturas
│   └── tests/
├── reporting/               # Módulo 6: Report Engine
│   ├── src/
│   │   └── lib.rs           # Geração JSON/HTML/PDF
│   └── tests/
├── api/                     # REST API
│   ├── src/
│   │   ├── main.rs          # Entry point
│   │   ├── lib.rs           # Router
│   │   ├── handlers.rs      # API handlers
│   │   └── state.rs         # Estado compartilhado
│   └── Cargo.toml
├── dashboard/               # Frontend Desktop
│   ├── src/
│   │   ├── main.tsx         # Entry point
│   │   ├── App.tsx          # Componente principal
│   │   ├── components/
│   │   │   └── Sidebar.tsx  # Navegação
│   │   └── pages/
│   │       ├── Overview.tsx
│   │       ├── Devices.tsx
│   │       ├── ExposureMap.tsx
│   │       ├── Threats.tsx
│   │       ├── Alerts.tsx
│   │       ├── Reports.tsx
│   │       └── Settings.tsx
│   ├── package.json
│   ├── tsconfig.json
│   ├── tailwind.config.js
│   └── index.html
├── mobile/                  # App Mobile (Flutter)
│   ├── lib/
│   │   ├── main.dart
│   │   ├── providers/
│   │   │   └── device_provider.dart
│   │   └── screens/
│   │       └── home_screen.dart
│   └── pubspec.yaml
├── router-agent/            # Agent para roteadores
│   ├── src/
│   │   ├── lib.rs           # Agent logic
│   │   └── main.rs          # Entry point
│   └── Cargo.toml
├── docker/                  # Configuração Docker
│   ├── Dockerfile
│   ├── docker-compose.yml
│   └── .dockerignore
├── docs/                    # Documentação
│   ├── ARCHITECTURE.md      # Arquitetura detalhada
│   ├── API.md               # Referência da API
│   ├── THREAT_MODEL.md      # Modelo de ameaças
│   ├── ROADMAP.md           # Roadmap do projeto
│   ├── INSTALLATION.md      # Guia de instalação
│   └── COMPLIANCE_CHECK.md  # Verificação de conformidade
├── Cargo.toml               # Workspace configuration
├── Cargo.lock               # Lock file
├── LICENSE                  # MIT OR Apache-2.0
├── .gitignore               # Git ignore rules
└── README.md                # Este arquivo
```

---

## 🛠️ Tecnologias Utilizadas

### Backend
- **[Rust 1.75+](https://www.rust-lang.org/)** - Linguagem principal (performance, memory safety)
- **[Axum](https://github.com/tokio-rs/axum)** - Framework web async
- **[SQLite](https://www.sqlite.org/)** - Banco de dados local
- **[Tokio](https://tokio.rs/)** - Runtime async
- **[Serde](https://serde.rs/)** - Serialização/deserialização
- **[Tracing](https://github.com/tokio-rs/tracing)** - Logging estruturado

### Frontend Desktop
- **[Tauri](https://tauri.app/)** - Framework desktop (alternativa leve ao Electron)
- **[React 18](https://react.dev/)** - Biblioteca UI
- **[TypeScript](https://www.typescriptlang.org/)** - Type safety
- **[TailwindCSS](https://tailwindcss.com/)** - CSS utility-first
- **[Lucide React](https://lucide.dev/)** - Ícones
- **[React Router](https://reactrouter.com/)** - Routing

### Mobile
- **[Flutter](https://flutter.dev/)** - App multiplataforma
- **[Provider](https://pub.dev/packages/provider)** - State management
- **[HTTP](https://pub.dev/packages/http)** - Cliente HTTP

### DevOps
- **[Docker](https://www.docker.com/)** - Containerização
- **[GitHub Actions](https://github.com/features/actions)** - CI/CD

---

## 🔒 Requisitos de Segurança

- ✅ **Processamento local por padrão** - Nenhum dado enviado sem consentimento
- ✅ **TLS 1.3** - Comunicação segura
- ✅ **AES-256-GCM** - Criptografia de dados em repouso
- ✅ **Ed25519** - Assinaturas digitais
- ✅ **SHA-256** - Hashing
- ✅ **Privacidade preservada** - Configuração sensível criptografada

---

## 🚀 Instalação Rápida

### Pré-requisitos

- [Rust 1.75+](https://rustup.rs/)
- [Node.js 18+](https://nodejs.org/)
- [Docker](https://www.docker.com/) (opcional)
- [Flutter SDK](https://flutter.dev/docs/get-started/install) (para mobile)

### Backend

```bash
# Clonar repositório
git clone https://github.com/NaiCodeBr/ghostwall.git
cd ghostwall

# Build
cargo build --release

# Executar API
cd api
cargo run --bin ghostwall-api
```

API disponível em `http://localhost:8080`

### Dashboard Desktop

```bash
cd dashboard
npm install
npm run tauri dev
```

### Mobile

```bash
cd mobile
flutter pub get
flutter run
```

### Docker

```bash
cd docker
docker-compose up -d
```

---

## 📖 Uso

### API Endpoints

| Método | Endpoint | Descrição |
|--------|----------|-----------|
| GET | `/api/health` | Health check |
| POST | `/api/scan` | Iniciar varredura RF |
| GET | `/api/devices` | Listar dispositivos |
| GET | `/api/devices/:id` | Detalhes do dispositivo |
| POST | `/api/risk/evaluate` | Avaliar risco |
| GET | `/api/exposure` | Score de exposição |
| POST | `/api/reports` | Gerar relatório |
| GET | `/api/threat-intel` | Threat intelligence |

### Exemplo de Uso

```bash
# Health check
curl http://localhost:8080/api/health

# Iniciar scan
curl -X POST http://localhost:8080/api/scan

# Listar dispositivos
curl http://localhost:8080/api/devices

# Avaliar risco
curl -X POST http://localhost:8080/api/risk/evaluate \
  -H "Content-Type: application/json" \
  -d '{"device_id": "uuid"}'

# Gerar relatório
curl -X POST http://localhost:8080/api/reports \
  -H "Content-Type: application/json" \
  -d '{"format": "html"}'
```

---

## 🧩 Módulos

### Módulo 1: RF Discovery Engine

Descobre dispositivos RF no ambiente:
- Dispositivos Wi-Fi
- Fabricantes (OUI)
- Access Points
- ESP32, Raspberry Pi, Broadcom

**Coleta:** MAC, Vendor, SSID, BSSID, RSSI, Canal, Segurança, Tempo online

**Plataformas:**
- Linux: `iw`, `nl80211`
- macOS: CoreWLAN
- Windows: Npcap

### Módulo 2: RF Fingerprint Engine

Cria assinaturas comportamentais:
- Beacon Interval
- Channel Utilization
- RSSI Stability
- Broadcast Frequency
- Probe Requests

### Módulo 3: RSIE Engine

Motor de scoring baseado em regras:

| Regra | Descrição | Score |
|-------|-----------|-------|
| GW-RF-001 | Hardware compatível com CSI | +10 |
| GW-RF-002 | 3+ dispositivos Espressif | +25 |
| GW-RF-003 | Operação contínua | +15 |
| GW-RF-004 | UDP pequenos e frequentes | +20 |
| GW-RF-005 | Telemetria MQTT | +10 |
| GW-RF-010 | Cluster de sensoriamento | +50 |
| GW-FP-001 | Equipamento conhecido | -20 |

### Módulo 4: RF Exposure Scanner

Avalia facilidade de sensoriamento RF (0-100):
- Cobertura Wi-Fi
- Quantidade de APs
- Distribuição espacial
- Densidade RF
- Quantidade de dispositivos

### Módulo 5: Threat Intelligence

Banco comunitário de assinaturas:
- Vendor
- Fingerprint
- Risk Score
- Tags
- Atualização automática
- Assinaturas versionadas

### Módulo 6: Report Engine

Gera relatórios em:
- JSON
- HTML
- PDF

**Conteúdo:**
- Inventário completo
- Score de Risco
- Score de Exposição
- Dispositivos Suspeitos
- Recomendações automáticas

---

## 💻 Dashboard

O dashboard desktop inclui 7 páginas:

- **Overview** - Visão geral e estatísticas
- **Devices** - Lista de dispositivos descobertos
- **Exposure Map** - Mapa de exposição RF
- **Threats** - Threat intelligence
- **Alerts** - Alertas por nível (Info, Baixo, Médio, Alto, Crítico)
- **Reports** - Geração e download de relatórios
- **Settings** - Configurações e privacidade

---

## 🧪 Desenvolvimento

### Testes

```bash
# Executar todos os testes
cargo test

# Testes com output
cargo test -- --nocapture

# Testes específicos
cargo test --package ghostwall-core
```

### Formatação

```bash
cargo fmt
```

### Lint

```bash
cargo clippy
```

### Build Release

```bash
cargo build --release
```

---

## 🤝 Contribuição

Contribuições são bem-vindas! Por favor:

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

### Código de Conduta

- Seja respeitoso
- Use linguagem apropriada
- Não faça afirmações sem evidência técnica
- Siga as regras de linguagem especificadas

---

## 📄 Licença

Este projeto está licenciado sob as licenças **MIT** ou **Apache-2.0** - você pode escolher.

### MIT License

Copyright (c) 2024 GHOSTWALL Contributors

### Apache-2.0 License

Copyright (c) 2024 GHOSTWALL Contributors

Licensed under the Apache License, Version 2.0

---

## 📚 Documentação

- [Arquitetura](docs/ARCHITECTURE.md) - Detalhes da arquitetura
- [API Reference](docs/API.md) - Documentação completa da API
- [Threat Model](docs/THREAT_MODEL.md) - Modelo de ameaças
- [Installation Guide](docs/INSTALLATION.md) - Guia detalhado de instalação
- [Roadmap](docs/ROADMAP.md) - Planos futuros
- [Compliance Check](docs/COMPLIANCE_CHECK.md) - Verificação de conformidade

---

## 🔗 Referências

- [RuView](https://github.com/ruvnet/RuView) - Referência inicial
- [ESP32-CSI-Tool](https://github.com/StevenMHernandez/ESP32-CSI-Tool) - CSI em ESP32
- [Nexmon CSI](https://github.com/seemoo-lab/nexmon_csi) - CSI em Broadcom

---

## ⚠️ Aviso Legal

Este software é fornecido "como está", sem garantia de qualquer tipo. Os autores não são responsáveis pelo uso indevido deste software.

**Uso Responsável:**
- Este ferramenta deve ser usada apenas para fins legítimos de segurança
- Não use para monitorar redes sem autorização
- Respeite as leis de privacidade locais

---

## 🌟 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=NaiCodeBr/ghostwall&type=Date)](https://star-history.com/#NaiCodeBr/ghostwall&Date)

---

<div align="center">

**[⬆ Voltar ao topo](#-ghostwall)**

Made with ❤️ by NaiCodeBr

</div>
