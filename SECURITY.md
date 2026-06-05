# Política de Segurança

## Versões Suportadas

| Versão | Suportado |
|--------|-----------|
| 0.1.0  | ✅        |

## Relatando Vulnerabilidades

Se você descobrir uma vulnerabilidade de segurança no GHOSTWALL, por favor relate-a de forma responsável.

### Como Relatar

**NÃO** abra um issue público. Em vez disso, envie um MSG para:

📧 **https://github.com/NaiCodeBr/ghostwall/issues**

Inclua o seguinte em seu relatório:

- Descrição da vulnerabilidade
- Passos para reproduzir
- Impacto potencial
- Sugestão de correção (se possível)

### Processo

1. Você receberá uma confirmação dentro de 48 horas
2. Os mantenedores investigarão a vulnerabilidade
3. Uma correção será desenvolvida
4. Uma versão de segurança será lançada
5. O relatório público será feito após a correção

### Reconhecimento

Relatórios de segurança válidos serão reconhecidos publicamente (com sua permissão).

## Melhores Práticas de Segurança

### Para Usuários

- Mantenha o software atualizado
- Use em redes que você tem permissão para monitorar
- Não compartilhe dados sensíveis
- Revise as configurações de privacidade
- Use autenticação forte se disponível

### Para Desenvolvedores

- Siga as diretrizes de secure coding
- Use dependências atualizadas
- Realize code reviews
- Teste segurança regularmente
- Use criptografia para dados sensíveis

### Para Operadores

- Mantenha o sistema atualizado
- Use firewalls e segmentação de rede
- Monitore logs regularmente
- Faça backup regular
- Use TLS para comunicação

## Criptografia

O GHOSTWALL usa:

- **AES-256-GCM** para dados em repouso
- **Ed25519** para assinaturas digitais
- **SHA-256** para hashing
- **TLS 1.3** para comunicação em trânsito

## Privacidade

O GHOSTWALL é projetado para:

- Processar dados localmente por padrão
- Não enviar dados sem consentimento explícito
- Criptografar configurações sensíveis
- Permitir deleção de dados
- Respeitar leis de privacidade

## Auditoria

O código é:

- Open source e auditável
- Modular e bem documentado
- Testado regularmente
- Revisado por pares

## Compliance

O GHOSTWALL busca compliance com:

- GDPR (Regulamento Geral de Proteção de Dados)
- LGPD (Lei Geral de Proteção de Dados - Brasil)
- Outras leis de privacidade locais

## Perguntas Frequentes

### O GHOSTWALL envia dados para servidores externos?

Não, por padrão todo processamento é local. Dados só são enviados com consentimento explícito do usuário.

### O GHOSTWALL é seguro para uso em produção?

Sim, mas recomendamos:
- Testar em ambiente de staging primeiro
- Revisar as configurações
- Monitorar logs
- Manter atualizado

### O GHOSTWALL pode ser usado para monitorar redes sem permissão?

Não. O GHOSTWALL deve ser usado apenas em redes que você tem permissão para monitorar. O uso não autorizado é ilegal.

## Contato

Para questões de segurança não relacionadas a vulnerabilidades:

📧 **https://github.com/NaiCodeBr/ghostwall/issues**

## Agradecimentos

Agradecemos a todos que relatam vulnerabilidades de forma responsável e ajudam a manter o GHOSTWALL seguro.
