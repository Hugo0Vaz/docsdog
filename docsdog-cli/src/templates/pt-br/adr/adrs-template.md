# Registro de Decisões de Arquitetura
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
* [2. Áreas de Decisão](#2-áreas-de-decisão)
    * [2.1 Princípios de Arquitetura](#21-princípios-de-arquitetura)
    * [2.2 Motivadores da Decisão](#22-motivadores-da-decisão)
* [3. Registro de Decisões](#3-registro-de-decisões)
* [4. Modelo de Decisão](#4-modelo-de-decisão)
* [5. Apêndices](#5-apêndices)
<!-- TOC -->

## Histórico de Revisões

| Nome | Data | Motivo das Alterações | Versão |
|------|------|-----------------------|--------|
|      |      |                       |        |
|      |      |                       |        |

## 1. Introdução
💬 _Introdução ao Registro de Decisões de Arquitetura e como ele se relaciona com o conjunto de documentação do projeto. Orienta o leitor sobre o processo de ADR e seu papel no ciclo de vida do projeto._

➥ Resuma brevemente o propósito do Registro de ADR, escopo, relação com outros documentos (SRS, documentos de arquitetura, roadmaps) e como o documento está organizado. Não inclua detalhes aqui; referencie as seções relevantes.

### 1.1 Propósito do Documento
💬 _Esclarece por que este Registro de ADR existe, o que contém e quem deve usá-lo._

➥ Declare o propósito em 2–4 frases. Nomeie os públicos principais (ex.: arquitetura, engenharia, produto, operações, segurança) e como o utilizam ao longo do ciclo de vida do projeto.

💡 Dicas:
- Enfatize que o Registro de ADR captura e indexa decisões arquiteturais, tornando o histórico de decisões e sua fundamentação rastreáveis.
- Mencione que os registros de decisão individuais são criados usando o Modelo de Decisão (Seção 4).
- Explique como as decisões são classificadas e quando devem ser revisitadas.

### 1.2 Escopo do Produto
💬 _Define o(s) sistema(s) regido(s) por estas decisões arquiteturais._

➥ Identifique o produto/sistema por nome e versão/release. Em 3–5 frases, descreva o escopo da arquitetura sendo regida: subsistemas, componentes, integrações ou preocupações transversais. Esclareça os limites — o que está no escopo vs. fora do escopo deste Registro de ADR.

💡 Dicas:
- Referencie o SRS ou documento de visão/escopo para o escopo mais amplo do produto.
- Se um Registro de ADR separado rege um subsistema, observe isso aqui.

### 1.3 Definições, Acrônimos e Abreviações
💬 _Ajude os leitores a entender termos especializados usados no registro de decisões._

➥ Forneça um glossário de termos, acrônimos e abreviações usados nas ADRs.

💡 Dicas:
- Inclua termos como ADR (Registro de Decisão de Arquitetura), preocupação transversal, atributo de qualidade, trade-off, etc.
- Mantenha as entradas em ordem alfabética e consistentes com o glossário do SRS.

| Termo | Definição                                                                                               |
|-------|---------------------------------------------------------------------------------------------------------|
| ADR   | Architecture Decision Record — Um documento que captura uma decisão arquitetural significativa          |
| ADRL  | Architecture Decision Record Log — Este documento, o índice e catálogo de todas as ADRs                 |
| SRS   | Software Requirements Specification — Documento que descreve o propósito, requisitos e natureza do sistema |

### 1.4 Referências
💬 _Lista fontes externas que são normativas ou informativas para o processo de ADR._

➥ Cite padrões, frameworks de arquitetura, SRS, roadmap, documentos de design ou guias de estilo. Para cada um, inclua título, autor/proprietário, versão, data e localização/URL. Indique se é normativo (vinculante) ou informativo (orientação).

💡 Dicas:
- Referencie o modelo de decisão de arquitetura e o framework de tomada de decisão utilizado.

### 1.5 Visão Geral do Documento
💬 _Guia breve para navegação no Registro de ADR._

➥ Resuma o que cada seção principal cobre (Áreas de Decisão, Registro de Decisões, Modelo, Apêndices), observe quaisquer convenções do documento e mencione como as atualizações e o histórico de revisões são gerenciados.

💡 Dicas:
- Mantenha em 3–5 frases com foco na navegação e convenções.

## 2. Áreas de Decisão
💬 _Fornece o contexto que enquadra as decisões arquiteturais. Pense nisso como os "acordos de trabalho" da equipe de arquitetura._

### 2.1 Princípios de Arquitetura
💬 _Princípios abrangentes que guiam e restringem as decisões arquiteturais._

➥ Defina 5–15 princípios de arquitetura que servem como diretrizes. Para cada um, declare o princípio em si e uma breve justificativa. Os princípios devem ser acionáveis, testáveis e orientar a tomada de decisão.

💡 Dicas:
- Use o formato: "[Princípio] — [Justificativa]."
- Exemplos: "API-first — Todas as capacidades são expostas através de APIs versionadas." "Prefira o simples ao complexo — Escolha padrões comprovados e tecnologias bem compreendidas."
- Alinhe os princípios de arquitetura com as Restrições do Produto do SRS (Seção 2.3).

| # | Princípio | Justificativa | Fonte |
|---|-----------|---------------|-------|
| 1 |           |               |       |
| 2 |           |               |       |

### 2.2 Motivadores da Decisão
💬 _Forças recorrentes, restrições ou atributos de qualidade que influenciam as decisões arquiteturais._

➥ Liste os principais motivadores de decisão que se aplicam a muitas ADRs (ex.: escalabilidade, custo, tempo de lançamento, conformidade, habilidades da equipe, vendor lock-in). Esses motivadores informam a análise de trade-off nas ADRs individuais.

💡 Dicas:
- Vincule cada motivador à seção relevante de QoS do SRS (3.3) ou Restrição do Produto (2.3) quando aplicável.
- Diferencie restrições obrigatórias de qualidades fortemente desejadas.

## 3. Registro de Decisões
💬 _O índice de todas as decisões arquiteturais tomadas para este produto/sistema. Este é o núcleo do Registro de ADR._

➥ Mantenha uma tabela com links para cada ADR individual. Cada entrada deve fornecer contexto suficiente para que os leitores entendam a decisão rapidamente e naveguem para o registro completo.

💡 Dicas:
- Atribua IDs sequenciais (ex.: ADR-0001, ADR-0002) ou um esquema hierárquico por domínio/componente.
- Mantenha o registro ordenado por ID ou data e marque decisões substituídas com um ponteiro para a ADR de substituição.
- Arquive em vez de excluir decisões que não são mais aplicáveis.

| ID       | Título                          | Status     | Data       | Área de Decisão | Substitui / Substituído por |
|----------|---------------------------------|------------|------------|-----------------|-----------------------------|
| ADR-0001 | Usar PostgreSQL como BD principal | accepted | 2025-01-15 | Armazenamento   |                             |
| ADR-0002 | Adotar arquitetura orientada a eventos | proposed | 2025-02-01 | Mensageria      |                             |
| ADR-0003 | Migrar autenticação de JWT para OAuth | deprecated | 2024-11-20 | Segurança       | substituído por ADR-0005    |

### 3.1 Ciclo de Vida dos Status
💬 _Define os valores de status permitidos e seus significados._

➥ Descreva o ciclo de vida de um registro de decisão (ex.: proposed → accepted → deprecated ou superseded). Defina o que cada status significa e quem pode mover uma decisão entre os status.

💡 Dicas:
- Mantenha o ciclo de vida simples — resista a adicionar muitos estados.
- Um diagrama de estado visual pode ajudar.

## 4. Modelo de Decisão
💬 _Modelo para registros de decisão de arquitetura individuais. Cada ADR é um documento independente criado usando este modelo._

➥ Referencie o arquivo de modelo e descreva quando e como usá-lo. Os modelos existem em vários níveis de detalhe — escolha o apropriado para a complexidade e o impacto da decisão.

| Modelo | Descrição |
|--------|-----------|
| `adr-template.md` | Completo — para decisões significativas e de alto impacto |
| `adr-template-minimal.md` | Mínimo — para decisões de médio impacto que precisam de registro sem análise completa |
| `adr-template-bare.md` | Enxuto — para redação mais rápida com todas as seções presentes |
| `adr-template-bare-minimal.md` | Enxuto-mínimo — para decisões leves e de baixo risco |

💡 Dicas:
- Escolha o nível do modelo com base na complexidade da decisão, reversibilidade e impacto nos stakeholders.
- Na dúvida, comece com o modelo completo — é mais fácil cortar do que adicionar análise faltante depois.

## 5. Apêndices
💬 _Material de apoio, como glossários, diagramas ou referências de estilo arquitetural._

➥ Inclua qualquer informação suplementar que apoie as ADRs, como diagramas de arquitetura, radar tecnológico, frameworks de tomada de decisão ou terminologia específica do domínio.

💡 Dicas:
- Referencie o material aqui em vez de duplicá-lo. Vincule ao SRS, documentos de design ou wiki de arquitetura.
- Inclua um calendário de decisões ou roadmap se as decisões estiverem vinculadas a releases ou marcos.