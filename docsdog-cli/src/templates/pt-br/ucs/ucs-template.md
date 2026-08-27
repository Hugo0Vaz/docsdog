# Especificação de Casos de Uso
## Para {{project name}}

Versão 0.1  
Preparado por {{author}}  
{{organization}}  
{{date_modified}}

## Sumário
<!-- TOC -->
* [1. Introdução](#1-introdução)
    * [1.1 Propósito do Documento](#11-propósito-do-documento)
    * [1.2 Escopo do Produto](#12-escopo-do-produto)
    * [1.3 Definições, Acrônimos e Abreviações](#13-definições-acrônimos-e-abreviações)
    * [1.4 Referências](#14-referências)
    * [1.5 Visão Geral do Documento](#15-visão-geral-do-documento)
* [2. Contexto do Sistema](#2-contexto-do-sistema)
    * [2.1 Limite do Sistema](#21-limite-do-sistema)
    * [2.2 Catálogo de Atores](#22-catálogo-de-atores)
    * [2.3 Diagrama de Casos de Uso](#23-diagrama-de-casos-de-uso)
* [3. Inventário de Casos de Uso](#3-inventário-de-casos-de-uso)
* [4. Modelos de Caso de Uso](#4-modelos-de-caso-de-uso)
* [5. Apêndices](#5-apêndices)
<!-- TOC -->

## Histórico de Revisões

| Nome | Data | Motivo das Alterações | Versão |
|------|------|-----------------------|--------|
|      |      |                       |        |
|      |      |                       |        |

## 1. Introdução
💬 _Fornece uma visão geral da Especificação de Casos de Uso e orienta o leitor sobre os requisitos comportamentais do sistema._

➥ Resuma brevemente o propósito da UCS, escopo do produto, público-alvo e como o documento está organizado. Não inclua detalhes aqui; referencie as seções relevantes.

### 1.1 Propósito do Documento
💬 _Esclarece por que esta UCS existe, o que contém e quem deve usá-la._

➥ Declare o propósito da UCS em 2–4 frases. Nomeie os públicos principais (ex.: produto, engenharia, QA, UX, stakeholders) e como a utilizam ao longo do ciclo de vida do software.

💡 Dicas:
- Enfatize que os casos de uso descrevem o comportamento do sistema externamente observável na forma de interações ator-objetivo, não a implementação interna.
- Mencione que os casos de uso individuais são criados usando um dos modelos referenciados na Seção 4.
- Esclareça a relação com o SRS (casos de uso servem como requisitos funcionais estruturados) e o Registro de ADR (decisões arquiteturais podem restringir como os casos de uso são realizados).

### 1.2 Escopo do Produto
💬 _Define o sistema cujo comportamento é especificado por estes casos de uso._

➥ Identifique o produto/sistema por nome e versão/release. Em 3–5 frases, descreva o limite do sistema — o que está dentro vs. fora — e quais subsistemas ou áreas de funcionalidade são cobertos. Observe quaisquer casos de uso intencionalmente adiados.

💡 Dicas:
- Referencie a Seção 1.2 do SRS para o escopo mais amplo do produto.
- O limite do sistema deve mapear diretamente para o campo Escopo nos casos de uso individuais.

### 1.3 Definições, Acrônimos e Abreviações
💬 _Ajude os leitores a entender termos do domínio e terminologia de casos de uso._

➥ Forneça um glossário de termos específicos do domínio e do formalismo de casos de uso usados neste documento.

💡 Dicas:
- Defina termos específicos de Cockburn se o modelo completo for usado (ex.: escopo, nível, extensões, garantias mínimas, partes interessadas e interesses).
- Mantenha as entradas em ordem alfabética e consistentes com o glossário do SRS.

| Termo          | Definição                                                                                           |
|----------------|-----------------------------------------------------------------------------------------------------|
| Ator           | Um papel desempenhado por uma pessoa, sistema ou dispositivo que interage com o sistema para alcançar um objetivo |
| Extensão       | Um fluxo alternativo de eventos ramificando de um passo no cenário de sucesso principal (Cockburn)  |
| Pré-condição   | Uma condição que deve ser verdadeira antes que o caso de uso possa começar                          |
| Ator Principal | O ator que inicia o caso de uso para alcançar um objetivo                                           |
| Escopo         | O limite do sistema sob design — a "caixa preta" que o caso de uso descreve (Cockburn)              |
| UCS            | Use Case Specification — Este documento, o catálogo de casos de uso para um sistema                 |

### 1.4 Referências
💬 _Lista fontes externas que são normativas ou informativas para esta UCS._

➥ Cite o SRS, Registro de ADR, especificações de design UX, documentos de processo de negócio ou padrões de domínio. Para cada uma, inclua título, autor/proprietário, versão, data e localização/URL. Indique se é normativa (vinculante) ou informativa (orientação).

💡 Dicas:
- Se existir um documento de visão/escopo ou roadmap do produto, referencie-o para os objetivos do usuário que orientam os casos de uso.

### 1.5 Visão Geral do Documento
💬 _Guia breve para navegação na UCS._

➥ Resuma o que cada seção principal cobre (Contexto do Sistema, Inventário de Casos de Uso, Modelos, Apêndices), observe quaisquer convenções do documento e mencione como as atualizações e o histórico de revisões são gerenciados.

💡 Dicas:
- Mantenha em 3–5 frases com foco na navegação e convenções.

## 2. Contexto do Sistema
💬 _Define o limite e os atores externos ao redor do sistema._

### 2.1 Limite do Sistema
💬 _Esclarece o que está dentro vs. fora do sistema sob design._

➥ Em 2–4 frases, descreva o limite do sistema — o conjunto de responsabilidades e comportamentos que o sistema possui. Este limite define o Escopo de cada caso de uso nesta especificação. Referencie quaisquer diagramas de contexto ou arquitetura do sistema.

💡 Dicas:
- Use a mesma definição de limite em todos os casos de uso para consistência.
- Se o sistema faz parte de um ecossistema maior, descreva seu papel e integrações principais.

### 2.2 Catálogo de Atores
💬 _Levantamento de todos os atores que interagem com o sistema._

➥ Liste cada ator (pessoa, sistema externo, dispositivo ou gatilho baseado em tempo) e seus objetivos ou responsabilidades. Agrupe os atores por tipo (principal vs. de apoio). Para cada ator, observe os casos de uso em que participam.

💡 Dicas:
- Atores são papéis, não pessoas ou sistemas específicos. Nomeie-os adequadamente (ex.: "Cliente," não "João," e "Gateway de Pagamento," não "Stripe").
- Inclua sistemas externos, jobs agendados e sensores/dispositivos como atores.
- Derive os atores da Seção 2.4 Características do Usuário do SRS — o SRS define *quem* são os usuários e seus atributos (expertise, níveis de acesso, frequência, necessidades de acessibilidade); o Catálogo de Atores mapeia esses mesmos papéis para os casos de uso em que participam. Mantenha os nomes dos atores consistentes em ambos os documentos.

| Ator              | Tipo      | Descrição / Responsabilidades | Participa em (IDs UC) |
|-------------------|-----------|-------------------------------|-----------------------|
| Cliente           | Principal | Faz pedidos, rastreia status  | UC-001, UC-002, UC-005 |
| Gateway de Pagamento | Secundário | Processa transações de pagamento | UC-001, UC-004        |
| Admin             | Principal | Gerencia catálogo e usuários | UC-010, UC-011        |
| Agendador         | Principal | Gera relatórios diários      | UC-020                |

### 2.3 Diagrama de Casos de Uso
💬 _Visão geral visual dos atores e seus relacionamentos com os casos de uso._

➥ Coloque ou referencie um diagrama de casos de uso UML que mostre todos os atores, casos de uso e relacionamentos (include/extend). Se o diagrama for grande, divida por área funcional ou grupo de atores.

💡 Dicas:
- Mantenha os diagramas em alto nível — diagramas de caso de uso são sobre contexto, não fluxo de controle.
- Referencie os documentos de caso de uso individuais para fluxos detalhados; não os duplique aqui.

## 3. Inventário de Casos de Uso
💬 _O índice de todos os casos de uso para este sistema. Este é o núcleo da UCS._

➥ Mantenha uma tabela com links para cada documento de caso de uso individual. Cada entrada deve fornecer contexto suficiente para navegar no inventário rapidamente.

💡 Dicas:
- Use um esquema de ID consistente (ex.: UC-001, UC-002) mapeado para os IDs de requisito do SRS.
- Ordene por prioridade ou área funcional para ajudar os leitores a priorizar a implementação.
- Marque claramente os casos de uso adiados ou planejados.

| ID     | Nome do Caso de Uso    | Ator Principal | Nível        | Prioridade | Status | Referência SRS |
|--------|------------------------|----------------|--------------|------------|--------|----------------|
| UC-001 | Fazer um Pedido        | Cliente        | User-goal    | Alta       | feito  | REQ-FUNC-010   |
| UC-002 | Rastrear Status do Pedido | Cliente     | User-goal    | Alta       | feito  | REQ-FUNC-011   |
| UC-003 | Processar Reembolso    | Agente de Suporte | User-goal | Média      | rascunho | REQ-FUNC-015 |
| UC-004 | Validar Método de Pagamento | Gateway Pagamento | Subfunção | Alta   | feito  | REQ-FUNC-020   |
| UC-005 | Gerar Relatório de Vendas | Agendador   | Summary      | Baixa      | planejado | REQ-FUNC-030 |

### 3.1 Ciclo de Vida dos Status
💬 _Define os valores de status permitidos e seus significados._

➥ Descreva o ciclo de vida de um caso de uso (ex.: rascunho → revisado → aprovado → implementado → verificado). Defina o que cada status significa e quem pode fazer a transição entre os status.

💡 Dicas:
- Alinhe com o fluxo de trabalho do projeto (ex.: colunas do quadro ágil, status de verificação do SRS).

### 3.2 Convenção de Níveis
💬 _Define os níveis de abstração usados nesta especificação (Cockburn)._

➥ Descreva os níveis de caso de uso e como aplicá-los para evitar misturar camadas de abstração:

| Nível        | Propósito | Escopo típico | Exemplo |
|--------------|-----------|---------------|---------|
| Summary (Resumo)     | Processo de negócio de alto nível, abrange múltiplos objetivos de usuário | Sistema ou empresa | "Processar um Pedido da Colocação à Entrega" |
| User-goal (Objetivo do usuário)   | A unidade primária de trabalho — uma única sessão, único objetivo | Funcionalidade ou fluxo de trabalho | "Fazer um Pedido" |
| Subfunção | Um sub-passo reutilizável chamado por múltiplos casos de uso user-goal | Componente ou serviço | "Validar Método de Pagamento" |

💡 Dicas:
- A maioria dos casos de uso em uma especificação deve estar no nível user-goal. Resumos fornecem contexto; subfunções extraem comportamento reutilizável.
- Um caso de uso de resumo pode referenciar seus casos de uso user-goal constituintes em seu cenário de sucesso principal.

## 4. Modelos de Caso de Uso
💬 _Modelos para documentos de caso de uso individuais. Cada caso de uso é um documento independente criado usando um destes modelos._

➥ Referencie os modelos e descreva quando usar cada um. O modelo Cockburn é o padrão abrangente; o modelo UML Simplificado é adequado para casos de uso de menor risco ou bem compreendidos.

| Modelo | Estilo | Descrição |
|--------|--------|-----------|
| `uc-template.md` | Cockburn (completo) | Abrangente — para casos de uso complexos, com múltiplos stakeholders ou alto risco, onde casos extremos e interesses dos stakeholders importam |
| `uc-template-bare.md` | Cockburn (enxuto) | Mesmas seções do modelo Cockburn completo, com placeholders vazios para redação mais rápida |
| `uc-template-minimal.md` | UML Simplificado | Simplificado — para casos de uso padrão ou bem compreendidos onde a análise completa de stakeholders é desnecessária |
| `uc-template-bare-minimal.md` | UML Simplificado (enxuto) | Mesmas seções do modelo UML Simplificado, com placeholders vazios para autoria rápida |

💡 Dicas:
- Use o modelo Cockburn como padrão para funcionalidades novas ou de alto impacto. Reduza para UML Simplificado para ferramentas internas, operações CRUD ou casos de uso com um único ator e poucas extensões.
- Na dúvida, comece com Cockburn — o ato de preencher partes interessadas e interesses frequentemente revela requisitos ocultos.

## 5. Apêndices
💬 _Material de apoio opcional, como diagramas, modelos de processo de negócio ou personas de atores._

➥ Inclua qualquer informação suplementar que apoie os casos de uso. Referencie em vez de duplicar quando possível.

💡 Dicas:
- Personas de atores ou mapas de jornada podem ser incluídos aqui para fornecer contexto mais rico para os casos de uso.
- Faça referência cruzada com os Apêndices do SRS para documentação mais ampla do sistema.