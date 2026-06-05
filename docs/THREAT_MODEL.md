# Threat Model GHOSTWALL

## Visão Geral

Este documento descreve o modelo de ameaças do GHOSTWALL e as medidas de segurança implementadas.

## Ameaças Identificadas

### 1. Monitoramento RF Não Autorizado

**Descrição:** Terceiros utilizando hardware compatível com CSI para monitorar movimento e presença.

**Indicadores:**
- Hardware ESP32-S3, ESP32-C6
- Broadcom com Nexmon
- Operação contínua
- UDP pequenos e frequentes
- Telemetria MQTT

**Mitigação:**
- Detecção de hardware compatível
- Análise de padrões de tráfego
- Alertas de cluster de sensoriamento

### 2. Exposição de Dados Locais

**Descrição:** Acesso não autorizado ao banco de dados local.

**Mitigação:**
- Criptografia AES-256-GCM do banco
- Permissões de arquivo restritas
- Opção de criptografia configurável

### 3. Intercepção de Comunicação

**Descrição:** Interceptação de tráfego entre componentes.

**Mitigação:**
- TLS 1.3 para comunicação externa
- Comunicação local via Unix sockets quando possível
- Validação de certificados

### 4. Injeção de Assinaturas Maliciosas

**Descrição:** Assinaturas de threat intelligence falsas ou maliciosas.

**Mitigação:**
- Assinaturas versionadas
- Verificação de integridade
- Fontes confiáveis apenas
- Assinatura Ed25519

### 5. Escalada de Privilégios

**Descrição:** Exploração de vulnerabilidades para ganhar privilégios elevados.

**Mitigação:**
- Princípio de menor privilégio
- Sandbox no Tauri
- Validação de entrada
- Rust memory safety

## Superfície de Ataque

### Componentes Externos
- API Server (port 8080)
- Dashboard Desktop
- Mobile App

### Componentes Internos
- Scanner (requer privilégios de rede)
- Storage (acesso ao sistema de arquivos)

## Controles de Segurança

### Autenticação
- N/A (processamento local)

### Autorização
- Controle de acesso ao sistema de arquivos
- Permissões de rede

### Criptografia
- Dados em repouso: AES-256-GCM
- Dados em trânsito: TLS 1.3
- Assinaturas: Ed25519
- Hashing: SHA-256

### Logging
- Logs estruturados com tracing
- Logs sensíveis omitidos
- Rotação de logs

### Validação
- Validação de entrada em todos os endpoints
- Sanitização de dados
- Type safety via Rust

## Privacidade

### Coleta de Dados
- MAC addresses
- SSIDs
- RSSI
- Padrões de tráfego

### Retenção
- Configurável pelo usuário
- Padrão: 30 dias

### Compartilhamento
- Nunca compartilhado sem consentimento explícito
- Opção de modo offline

## Compliance

### GDPR
- Processamento local por padrão
- Consentimento explícito
- Direito ao esquecimento (deleção de dados)

### LGPD
- Mesmos princípios do GDPR
- Conformidade com lei brasileira

## Recomendações

### Para Usuários
- Manter software atualizado
- Revisar logs regularmente
- Usar modo offline quando possível
- Revisar permissões de rede

### Para Desenvolvedores
- Seguir princípios de secure coding
- Realizar code reviews
- Testar segurança regularmente
- Manter dependências atualizadas
