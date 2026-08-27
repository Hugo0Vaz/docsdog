# Especificação de Requisitos de Software
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
    * [2.2 Funções do Produto](#22-funções-do-produto)
    * [2.3 Restrições do Produto](#23-restrições-do-produto)
    * [2.4 Características do Usuário](#24-características-do-usuário)
    * [2.5 Premissas e Dependências](#25-premissas-e-dependências)
    * [2.6 Distribuição de Requisitos](#26-distribuição-de-requisitos)
* [3. Requisitos](#3-requisitos)
    * [3.1 Interfaces Externas](#31-interfaces-externas)
    * [3.2 Funcionais](#32-funcionais)
    * [3.3 Qualidade de Serviço](#33-qualidade-de-serviço)
    * [3.4 Conformidade](#34-conformidade)
    * [3.5 Design e Implementação](#35-design-e-implementação)
    * [3.6 IA/ML](#36-iaml)
* [4. Verificação](#4-verificação)
* [5. Apêndices](#5-apêndices)
<!-- TOC -->

## Histórico de Revisões

| Nome | Data | Motivo das Alterações | Versão |
|------|------|-----------------------|--------|
|      |      |                       |        |
|      |      |                       |        |

## 1. Introdução
<!-- visão geral do SRS: propósito, escopo, público e organização do documento; evite requisitos detalhados -->

### 1.1 Propósito do Documento
<!-- por que este SRS existe, seus públicos-alvo e como eles o usarão; mantenha em 2–4 frases e evite detalhes de implementação -->

### 1.2 Escopo do Produto
<!-- o produto (nome/versão), seu propósito principal, capacidades-chave e limites. seja breve e foque no "o quê" e "por quê", não no "como" -->

### 1.3 Definições, Acrônimos e Abreviações
<!-- glossário de termos do domínio, acrônimos e abreviações; mantenha as entradas em ordem alfabética -->

| Termo | Definição |
|-------|-----------|
|       |           |
|       |           |

### 1.4 Referências
<!-- fontes externas normativas e informativas; inclua título, proprietário, versão, data, localização/URL e se é normativa ou informativa -->

### 1.5 Visão Geral do Documento
<!-- estrutura e convenções do documento -->

## 2. Visão Geral do Produto
<!-- contexto que molda os requisitos do produto -->

### 2.1 Perspectiva do Produto
<!-- contexto do sistema: um novo produto, uma substituição ou parte de uma família; observe relacionamentos com outros sistemas -->

### 2.2 Funções do Produto
<!-- principais áreas funcionais ou funcionalidades que o produto fornece em 5–10 marcadores concisos -->

### 2.3 Restrições do Produto
<!-- restrições de design e implementação que afetam a solução -->

### 2.4 Características do Usuário
<!-- classes, papéis, expertise, níveis de acesso, frequência de uso e necessidades de acessibilidade ou localização -->

### 2.5 Premissas e Dependências
<!-- premissas sobre ambiente, serviços de terceiros, padrões de uso e outros fatores externos; observe impacto/risco potencial -->

### 2.6 Distribuição de Requisitos
<!-- mapeie os principais requisitos para subsistemas, serviços ou releases/iterações -->

## 3. Requisitos
<!-- requisitos identificáveis, verificáveis e testáveis; evite detalhes de implementação -->

### 3.1 Interfaces Externas
<!-- entradas/saídas (formatos, protocolos, temporização, etc.); referencie esquemas de interface quando disponíveis -->

#### 3.1.1 Interfaces de Usuário
<!-- interações do usuário (elementos de UI, diálogos, fluxos); referencie guias de design/estilo -->

#### 3.1.2 Interfaces de Hardware
<!-- interações com dispositivos físicos (tipos, sinais, etc.) -->

#### 3.1.3 Interfaces de Software
<!-- integrações com outros sistemas (APIs, contratos, proprietário, etc.) -->

### 3.2 Funcionais
<!-- comportamentos externamente observáveis organizados por funcionalidade/caso de uso -->

### 3.3 Qualidade de Serviço
<!-- seção de atributos não funcionais mensuráveis -->

#### 3.3.1 Desempenho
<!-- tempo (latência, vazão, etc.) e espaço (memória, armazenamento, largura de banda, etc.) -->

#### 3.3.2 Segurança
<!-- proteção de dados, identidades e operações (trânsito/repouso, autenticação, criptografia, etc.); segurança física, confidencialidade, privacidade, integridade e disponibilidade -->

#### 3.3.3 Confiabilidade
<!-- capacidade de executar consistentemente conforme especificado (MTBF, redundância/failover, caches, etc.) -->

#### 3.3.4 Disponibilidade
<!-- prontidão para entregar serviço (SLAs alvo, janelas de manutenção, recuperação/restauração, etc.) -->

#### 3.3.5 Observabilidade
<!-- logs, métricas, traces, alertas e dashboards -->

### 3.4 Conformidade
<!-- leis, padrões, contratos ou políticas; cite a autoridade e os critérios verificáveis -->

### 3.5 Design e Implementação
<!-- seção de restrições e mandatos sobre design, implantação e manutenção -->

#### 3.5.1 Instalação
<!-- garanta que o software funcione corretamente em seus ambientes de destino (plataformas compatíveis, pré-requisitos, configuração, etc.) -->

#### 3.5.2 Build e Entrega
<!-- controles para construir e entregar (gerenciamento de dependências, automação, integridade/rastreabilidade, etc.) -->

#### 3.5.3 Distribuição
<!-- implantações distribuídas, dados e dispositivos (topologias, replicação/posicionamento, etc.) -->

#### 3.5.4 Manutenibilidade
<!-- atributos mensuráveis que tornam o software mais fácil de modificar, corrigir e evoluir (modularidade, padrões, documentação, observabilidade, etc.) -->

#### 3.5.5 Reusabilidade
<!-- componentes destinados à reutilização -->

#### 3.5.6 Portabilidade
<!-- capacidade de executar em múltiplos ambientes (SOs/runtimes compatíveis, provedores de nuvem, etc.) -->

#### 3.5.7 Custo
<!-- metas/orçamentos que influenciam o design ou a implementação (gastos em nuvem, por transação, licenciamento, etc.) -->

#### 3.5.8 Prazo
<!-- marcos, datas de entrega e critérios de prontidão -->

#### 3.5.9 Prova de Conceito
<!-- objetivos, escopo, timebox e critérios de sucesso para qualquer POC -->

#### 3.5.10 Gerenciamento de Mudanças
<!-- como as mudanças são introduzidas e comunicadas (categorias, artefatos necessários e fluxo de trabalho, etc.) -->

### 3.6 IA/ML
<!-- seção de requisitos específicos de ML -->

#### 3.6.1 Especificação do Modelo
<!-- propósito do modelo, entradas/saídas, metas de desempenho, dados de validação, versionamento -->

#### 3.6.2 Gerenciamento de Dados
<!-- ciclo de vida dos conjuntos de dados (origem, rotulagem, anonimização, etc.) -->

#### 3.6.3 Guardrails
<!-- controles para que o sistema opere dentro dos limites aprovados (validação/sanitização, filtragem de saída, limites de ação, etc.) -->

#### 3.6.4 Ética
<!-- métricas/aplicação de justiça, transparência e responsabilidade -->

#### 3.6.5 Human-in-the-Loop
<!-- supervisão humana (pontos de revisão, escalonamentos, feedback, etc.) -->

#### 3.6.6 Ciclo de Vida e Operações do Modelo
<!-- implantação, monitoramento, retreinamento e descontinuação -->

## 4. Verificação

| ID do Requisito | Método de Verificação | Link do Teste/Artefato | Status | Evidência |
|-----------------|-----------------------|------------------------|--------|-----------|
|                 |                       |                        |        |           |
|                 |                       |                        |        |           |

## 5. Apêndices