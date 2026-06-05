# Suporte GHOSTWALL

## 📚 Recursos de Documentação

- [README](README.md) - Visão geral do projeto
- [Installation Guide](docs/INSTALLATION.md) - Guia de instalação detalhado
- [API Reference](docs/API.md) - Documentação completa da API
- [Architecture](docs/ARCHITECTURE.md) - Arquitetura do sistema
- [Threat Model](docs/THREAT_MODEL.md) - Modelo de ameaças
- [Roadmap](docs/ROADMAP.md) - Planos futuros

## 💬 Canais de Comunicação

### GitHub Issues
Use GitHub Issues para:
- Reportar bugs
- Sugerir features
- Fazer perguntas técnicas

### GitHub Discussions
Use GitHub Discussions para:
- Perguntas gerais
- Discussões sobre arquitetura
- Compartilhar experiências

### Email
Para questões não técnicas:
📧 **contact@ghostwall.org**

## ❓ Perguntas Frequentes

### Instalação

**Q: Quais são os pré-requisitos?**
A: Rust 1.75+, Node.js 18+, Docker (opcional)

**Q: O GHOSTWALL funciona no Windows?**
A: Sim, mas algumas funcionalidades de scanning requerem Npcap

**Q: Posso usar o GHOSTWALL em um Raspberry Pi?**
A: Sim, o backend em Rust é compatível com ARM

### Uso

**Q: O GHOSTWALL precisa de internet?**
A: Não, todas as funcionalidades principais funcionam offline

**Q: O GHOSTWALL envia dados para servidores externos?**
A: Não, por padrão todo processamento é local

**Q: Posso usar o GHOSTWALL em múltiplos dispositivos?**
A: Sim, cada dispositivo pode ter sua própria instância

### Segurança

**Q: O GHOSTWALL é seguro?**
A: Sim, usa criptografia AES-256-GCM, TLS 1.3, e processamento local

**Q: O GHOSTWALL pode ser usado para monitorar redes sem permissão?**
A: Não, use apenas em redes que você tem permissão para monitorar

**Q: O GHOSTWALL armazena dados sensíveis?**
A: Apenas dados RF necessários para análise, todos criptografados

### Desenvolvimento

**Q: Como posso contribuir?**
A: Veja o [CONTRIBUTING.md](CONTRIBUTING.md)

**Q: Como executar os testes?**
A: `cargo test` para Rust, `flutter test` para Flutter

**Q: Como reportar um bug?**
A: Abra um issue no GitHub com detalhes do ambiente e passos para reproduzir

## 🐛 Reportando Bugs

Ao reportar um bug, inclua:

1. **Versão:** Versão do GHOSTWALL
2. **Sistema Operacional:** Linux/macOS/Windows e versão
3. **Rust Version:** `rustc --version`
4. **Node.js Version:** `node --version` (se aplicável)
5. **Passos para Reproduzir:** Passos detalhados
6. **Comportamento Esperado:** O que deveria acontecer
7. **Comportamento Atual:** O que aconteceu
8. **Logs:** Logs relevantes (se disponíveis)

## 💡 Sugerindo Features

Ao sugerir uma feature, inclua:

1. **Título:** Título claro e descritivo
2. **Descrição:** Descrição detalhada da feature
3. **Caso de Uso:** Por que esta feature é necessária
4. **Alternativas:** Soluções alternativas consideradas
5. **Implementação:** Sugestões de implementação (opcional)

## 📖 Guia de Troubleshooting

### Problemas Comuns

**Erro: Permission denied ao fazer scan**
- Solução: Execute com sudo (Linux/macOS) ou como administrador (Windows)

**Erro: Port 8080 já em uso**
- Solução: Mude a porta na configuração ou pare o serviço usando a porta

**Erro: Database locked**
- Solução: Verifique se outra instância está rodando, ou delete o arquivo .db-wal

**Dashboard não carrega**
- Solução: Verifique se a API está rodando e se npm install foi executado

### Obtendo Mais Informações

Para debug, habilite logging detalhado:

```bash
RUST_LOG=debug cargo run --bin ghostwall-api
```

## 🎓 Aprendizado

### Para Iniciantes

1. Comece lendo o [README](README.md)
2. Siga o [Installation Guide](docs/INSTALLATION.md)
3. Experimente os exemplos na [API Reference](docs/API.md)
4. Explore o código fonte

### Para Desenvolvedores

1. Leia a [Architecture](docs/ARCHITECTURE.md)
2. Revise o código fonte
3. Execute os testes
4. Contribua com o projeto

## 📞 Contato

- **Issues:** [GitHub Issues](https://github.com/NaiCodeBr/ghostwall/issues)
- **Discussions:** [GitHub Discussions](https://github.com/NaiCodeBr/ghostwall/discussions)
- **Email:** contact@ghostwall.org

## ⏰ Tempo de Resposta

- **Bugs Críticos:** 24-48 horas
- **Features:** 1-2 semanas para resposta inicial
- **Perguntas Gerais:** 2-3 dias

---

Obrigado por usar o GHOSTWALL! 🎉
