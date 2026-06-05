# Contributing to GHOSTWALL

Obrigado por considerar contribuir com o GHOSTWALL! Este documento fornece diretrizes para contribuir com o projeto.

## 🤝 Como Contribuir

### Reportando Bugs

Antes de criar um issue de bug, por favor:

1. Verifique se o bug já foi reportado
2. Use um título claro e descritivo
3. Inclua informações do ambiente:
   - Sistema operacional
   - Versão do Rust
   - Versão do Node.js (se aplicável)
4. Descreva os passos para reproduzir
5. Inclua logs relevantes

### Sugerindo Features

Ao sugerir uma nova feature:

1. Verifique se a feature já foi solicitada
2. Use um título claro
3. Descreva a feature em detalhes
4. Explique o caso de uso
5. Considere se a feature se alinha com a missão do projeto

### Pull Requests

#### Processo

1. Fork o repositório
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Faça suas mudanças
4. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
5. Push para a branch (`git push origin feature/AmazingFeature`)
6. Abra um Pull Request

#### Diretrizes de Commit

- Use mensagens de commit claras e descritivas
- Siga o formato conventional commits:
  - `feat:` para novas features
  - `fix:` para bug fixes
  - `docs:` para mudanças na documentação
  - `style:` para mudanças de formatação
  - `refactor:` para refatoração
  - `test:` para adicionar ou modificar testes
  - `chore:` para tarefas de manutenção

#### Requisitos de Código

- **Rust:**
  - O código deve passar em `cargo clippy`
  - O código deve estar formatado com `cargo fmt`
  - Adicione testes para novas funcionalidades
  - Mantenha a cobertura de testes acima de 80%

- **TypeScript/React:**
  - O código deve passar no linter
  - Use TypeScript strict mode
  - Siga os padrões de código existentes

- **Flutter:**
  - Siga os guias de estilo do Flutter
  - Use dart format

#### Revisão de Código

Todos os Pull Requests passam por revisão de código. Por favor:

- Seja paciente durante o processo de revisão
- Responda aos comentários dos revisores
- Faça as mudanças solicitadas
- Mantenha o PR atualizado

## 📝 Regras de Linguagem

Este projeto tem regras específicas de linguagem para manter a integridade e evitar afirmações falsas:

### ✅ Use

- "Compatível com"
- "Potencialmente associado"
- "Indicador de exposição"
- "Infraestrutura compatível"
- "Pode indicar"

### ❌ Não Use

- "Confirmado" (sem evidência técnica)
- "Comprovado" (sem evidência técnica)
- "Detectado especificamente RuView"
- "Detectado espionagem"
- "Afirmar detecção sem evidência"

## 🧪 Testes

### Executando Testes

```bash
# Rust
cargo test

# Com coverage
cargo tarpaulin --out Html

# Flutter
flutter test
```

### Adicionando Testes

- Adicione testes unitários para novas funções
- Adicione testes de integração para novas features
- Mantenha os testes rápidos e determinísticos

## 📚 Documentação

- Atualize o README se necessário
- Adicione comentários no código para lógica complexa
- Atualize a documentação da API se adicionou novos endpoints
- Adicione exemplos de uso para novas features

## 🎯 Escopo do Projeto

Este projeto foca em:
- Auditoria de RF
- Detecção de indicadores compatíveis com Wi-Fi Sensing/CSI
- Avaliação de risco de exposição RF
- Privacidade e processamento local

Fora do escopo:
- Detecção específica de soluções proprietárias
- Afirmar espionagem sem evidência
- Funcionalidades que dependem de nuvem por padrão

## 📧 Comunicação

- Use GitHub Issues para bugs e features
- Use GitHub Discussions para perguntas gerais
- Seja respeitoso e construtivo
- Siga o Código de Conduta

## 🏆 Reconhecimento

Contribuidores serão reconhecidos no arquivo CONTRIBUTORS.md

## 📄 Licença

Ao contribuir, você concorda que suas contribuições serão licenciadas sob MIT ou Apache-2.0

---

Obrigado por contribuir com o GHOSTWALL! 🎉
