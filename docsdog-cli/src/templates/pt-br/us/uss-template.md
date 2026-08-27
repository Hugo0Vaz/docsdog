# Especificação de Histórias de Usuário
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
* [2. Visão Geral do Produto](#2-visão-geral-do-produto)
    * [2.1 Perspectiva do Produto](#21-perspectiva-do-produto)
    * [2.2 Classes de Usuário](#22-classes-de-usuário)
* [3. Inventário de Histórias](#3-inventário-de-histórias)
* [4. Modelo de História](#4-modelo-de-história)
* [5. Apêndices](#5-apêndices)
<!-- TOC -->

## Histórico de Revisões

| Nome | Data | Motivo das Alterações | Versão |
|------|------|-----------------------|--------|
|      |      |                       |        |
|      |      |                       |        |

## 1. Introdução
💬 _Fornece uma visão geral da Especificação de Histórias de Usuário e orienta o leitor sobre os comportamentos pretendidos do sistema a partir da perspectiva do usuário._

➥ Resuma brevemente o propósito da USS, escopo do produto, público-alvo e como o documento está organizado. Não inclua detalhes aqui; referencie as seções relevantes.

### 1.1 Propósito do Documento
💬 _Esclarece por que esta USS existe, o que contém e quem deve usá-la._

➥ Declare o propósito em 2–4 frases. Nomeie os públicos principais (ex.: produto, engenharia, QA, design, stakeholders) e como a utilizam ao longo do ciclo de vida de entrega.

💡 Dicas:
- Enfatize que a USS captura o comportamento pretendido do sistema a partir da perspectiva do usuário, expresso como histórias de usuário.
- Mencione que as histórias individuais são criadas usando o Modelo de História (Seção 4).
- Esclareça a relação com o SRS (histórias refinam requisitos funcionais) e a UCS (histórias e casos de uso descrevem visões complementares do comportamento).

### 1.2 Escopo do Produto
💬 _Define o sistema cujos comportamentos são capturados por estas histórias._

➥ Identifique o produto/sistema por nome e versão/release. Em 3–5 frases, descreva quais funcionalidades, fluxos de trabalho ou subsistemas são cobertos. Observe quaisquer áreas adiadas ou excluídas.

💡 Dicas:
- Referencie a Seção 1.2 do SRS para o escopo mais amplo do produto.
- Se existirem especificações de histórias separadas para subsistemas, observe-as aqui.

### 1.3 Definições, Acrônimos e Abreviações
💬 _Ajude os leitores a entender termos do domínio e terminologia de histórias._

➥ Forneça um glossário de termos usados nesta especificação.

💡 Dicas:
- Mantenha as entradas em ordem alfabética e consistentes com o glossário do SRS.

| Termo | Definição                                                                                                                   |
|-------|-----------------------------------------------------------------------------------------------------------------------------|
| SRS   | Software Requirements Specification — Documento que descreve o propósito, requisitos e natureza de um software              |
| UCS   | Use Case Specification — Catálogo de casos de uso descrevendo interações ator-objetivo com o sistema                       |
| US    | User Story — Uma descrição curta e simples de uma funcionalidade contada a partir da perspectiva da pessoa que deseja a nova capacidade |
| USS   | User Story Specification — Este documento, o catálogo de histórias de usuário para um sistema                              |

### 1.4 Referências
💬 _Lista fontes externas que são normativas ou informativas para esta USS._

➥ Cite o SRS, UCS, Registro de ADR, designs UX ou documentos de processo de negócio. Para cada um, inclua título, autor/proprietário, versão, data e localização/URL. Indique se é normativo (vinculante) ou informativo (orientação).

💡 Dicas:
- Prefira links estáveis ou caminhos de repositório em vez de URLs voláteis.

### 1.5 Visão Geral do Documento
💬 _Guia breve para navegação na USS._

➥ Resuma o que cada seção principal cobre (Visão Geral do Produto, Inventário de Histórias, Modelo, Apêndices), observe quaisquer convenções do documento e mencione como as atualizações e o histórico de revisões são gerenciados.

💡 Dicas:
- Mantenha em 3–5 frases com foco na navegação e convenções.

## 2. Visão Geral do Produto
💬 _Contexto que molda as histórias._

### 2.1 Perspectiva do Produto
💬 _Posiciona o sistema dentro de seu contexto mais amplo._

➥ Descreva o contexto e a origem do sistema — seja novo, uma substituição ou parte de uma família. Referencie sistemas relacionados ou dependências que afetam o escopo das histórias.

💡 Dicas:
- Destaque sistemas upstream/downstream e limites de propriedade.

### 2.2 Classes de Usuário
💬 _Os papéis e personas para os quais as histórias são escritas._

➥ Liste classes de usuário, papéis ou personas referenciadas nas histórias. Observe seus objetivos, expertise, frequência de uso e quaisquer atributos distintivos.

💡 Dicas:
- Derive da Seção 2.4 Características do Usuário do SRS. Mantenha os nomes consistentes entre os documentos.
- As histórias usam estes papéis na cláusula "Como...".

| Classe de Usuário | Descrição / Objetivos | Frequência |
|-------------------|-----------------------|------------|
|                   |                       |            |
|                   |                       |            |

## 3. Inventário de Histórias
💬 _O índice de todas as histórias de usuário para este sistema. Este é o núcleo da USS._

➥ Mantenha uma tabela com links para cada documento de história individual. Cada entrada deve fornecer contexto suficiente para navegar no inventário rapidamente.

💡 Dicas:
- Use um esquema de ID consistente (ex.: US-001, US-002).
- Ordene por prioridade ou área funcional.
- Marque claramente as histórias adiadas ou planejadas.

| ID     | Título | Classe de Usuário | Prioridade | Esforço | Status | Referência SRS |
|--------|--------|-------------------|------------|---------|--------|----------------|
| US-001 |        |                   |            |         |        |                |
| US-002 |        |                   |            |         |        |                |

### 3.1 Ciclo de Vida dos Status
💬 _Define os valores de status permitidos e seus significados._

➥ Descreva o ciclo de vida de uma história (ex.: rascunho → pronto → em progresso → concluído). Defina o que cada status significa e quem pode fazer a transição entre os status.

💡 Dicas:
- Alinhe com o fluxo de trabalho da equipe (ex.: colunas do quadro Kanban).

### 3.2 Convenção de Prioridade
💬 _Define os níveis de prioridade usados nas histórias._

| Prioridade | Descrição |
|------------|-----------|
| critical   | Deve ser entregue; bloqueia o progresso da funcionalidade principal |
| high       | Importante; deve ser entregue em breve |
| medium     | Valioso; entregar quando a capacidade permitir |
| low        | Desejável; entregar se e quando possível |

### 3.3 Convenção de Esforço
💬 _Define a escala de dimensionamento de esforço usada nas histórias._

| Esforço | Descrição |
|---------|-----------|
| XS      | Trivial — algumas horas |
| S       | Pequeno — um ou dois dias |
| M       | Médio — alguns dias |
| L       | Grande — uma semana ou mais |
| XL      | Extra grande — deve ser dividido em histórias menores |

## 4. Modelo de História
💬 _Modelos para documentos de história de usuário individuais. Cada história é um documento independente criado usando um destes modelos._

➥ Referencie os arquivos de modelo e descreva quando usar cada um.

| Modelo | Descrição |
|--------|-----------|
| `us-template.md` | Completo — com comentários de orientação (💬) e dicas (💡) para autoria cuidadosa |
| `us-template-bare.md` | Enxuto — apenas placeholders, para redação rápida quando a orientação não é necessária |

💡 Dicas:
- Comece com o modelo completo quando a história envolve múltiplos stakeholders ou dependências.
- Use o modelo enxuto para histórias diretas e bem compreendidas.

## 5. Apêndices
💬 _Material de apoio opcional, como personas, mapas de jornada ou referências de design._

➥ Inclua qualquer informação suplementar que apoie as histórias. Referencie em vez de duplicar quando possível.

💡 Dicas:
- Personas ou mapas de jornada do usuário podem fornecer contexto mais rico para as histórias.
- Faça referência cruzada com os Apêndices do SRS e UCS para documentação mais ampla do sistema.