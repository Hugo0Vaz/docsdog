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
💬 _Fornece uma visão geral do documento e orienta o leitor sobre o sistema que está sendo especificado._

➥ Resuma brevemente o propósito do SRS, escopo do produto, público-alvo e como o documento está organizado. Não inclua detalhes aqui; referencie as seções relevantes.

### 1.1 Propósito do Documento
💬 _Esclarece por que este SRS existe, o que contém e quem deve usá-lo._

➥ Declare o propósito do SRS em 2–4 frases. Nomeie os públicos principais (ex.: produto, engenharia, QA, segurança, conformidade, operações) e como o utilizam ao longo do ciclo de vida do software.

💡 Dicas:
- Enfatize que o SRS define o que o sistema deve fazer, não como ele fará.
- Mencione documentos relacionados (visão/escopo, arquitetura, roadmap, contratos) se relevante.

### 1.2 Escopo do Produto
💬 _Define o propósito do produto de software, seus limites e a relação com os objetivos de negócio._

➥ Identifique o produto por nome e versão/release. Em 3–5 frases, descreva seu propósito principal, capacidades-chave e resultados pretendidos. Liste claramente inclusões e exclusões quando este SRS cobrir parte de um sistema maior. Foque no "o quê" e no "por quê".

💡 Dicas:
- Conecte as capacidades aos objetivos de negócio e referencie um documento separado de visão/escopo se relevante.
- Inclua um diagrama simples se ele esclarecer os limites dentro de um sistema maior.

### 1.3 Definições, Acrônimos e Abreviações
➥ Ajude os leitores a entender termos especializados e notação fornecendo um glossário de termos do domínio, acrônimos e abreviações usados no SRS.

💡 Dicas:
- Inclua termos que impactam a interpretação dos requisitos (ex.: "usuário", "tenant", "tempo quase real").
- Mantenha as entradas em ordem alfabética e consistentes em todo o conjunto de documentos.

| Termo | Definição                                                                                                                   |
|-------|-----------------------------------------------------------------------------------------------------------------------------|
| API   | Application Programming Interface — Um conjunto de definições e protocolos para construir e integrar software               |
| SRS   | Software Requirements Specification — Um documento que descreve o propósito, requisitos e natureza de um software           |
| UI    | User Interface — A parte visual da aplicação através da qual um usuário interage com o software                             |

### 1.4 Referências
💬 _Lista fontes externas que são normativas ou informativas para este SRS._

➥ Cite padrões, contratos, políticas, especificações de interface, guias de estilo UX, documentos de caso de uso, decisões arquiteturais ou um documento de visão/escopo. Para cada referência, inclua título, autor/proprietário, versão, data e localização/URL. Indique se cada referência é normativa (vinculante) ou informativa (orientação).

💡 Dicas:
- Prefira links estáveis ou caminhos de repositório em vez de URLs voláteis.

### 1.5 Visão Geral do Documento
💬 _Guia breve da estrutura do SRS para que os leitores possam encontrar rapidamente o que precisam._

➥ Resuma o que cada seção principal cobre (Visão Geral do Produto, Requisitos, Verificação, Apêndices), observe quaisquer convenções do documento e mencione como as atualizações e o histórico de revisões são gerenciados.

💡 Dicas:
- Mantenha em 3–5 frases com foco na navegação e convenções.

## 2. Visão Geral do Produto
💬 _Fornece o contexto que influencia os requisitos do produto._

### 2.1 Perspectiva do Produto
💬 _Posiciona o produto dentro de um ecossistema ou linhagem maior._

➥ Descreva o contexto e a origem do produto, seja um novo produto, substituição ou membro de uma família. Se fizer parte de um sistema maior, explique brevemente os relacionamentos, interfaces externas e dependências principais. Inclua detalhes sobre propriedade, acordos de nível de serviço (SLAs) e modelos de suporte.

💡 Dicas:
- Destaque sistemas upstream/downstream e limites de propriedade.
- Um diagrama de contexto de alto nível pode ajudar a orientar o leitor.

### 2.2 Funções do Produto
💬 _Resumo de alto nível do que o produto permite que usuários ou sistemas façam._

➥ Forneça uma visão geral concisa das principais áreas funcionais/funcionalidades. Adie comportamentos detalhados, dados e casos extremos para a Seção 3.

💡 Dicas:
- 5–10 marcadores geralmente são suficientes neste nível, agrupando funções relacionadas logicamente.
- Inclua um fluxo de dados de alto nível ou diagrama de caso de uso se útil.

### 2.3 Restrições do Produto
💬 _Define limitações contextuais ou condições que moldam o design e a implementação._

➥ Descreva restrições como interfaces obrigatórias, stacks de tecnologia, obrigações regulatórias, linhas de base de QoS, limitações de hardware, famílias de modelos de IA/ML e políticas organizacionais.

💡 Dicas:
- Declare restrições como afirmações "deve" verificáveis (ex.: "deve usar módulos criptográficos validados FIPS 140-3").
- Diferencie restrições externas/internas e obrigatórias/preferenciais.
- Evite decisões de design a menos que sejam realmente vinculantes.

📝 Nota:
Requisitos (Seção 3) define obrigações verificáveis do sistema — comportamentos ou qualidades específicas que o sistema deve exibir para satisfazer os limites descritos nesta seção.

### 2.4 Características do Usuário
💬 _Define os grupos de usuários e os atributos que afetam os requisitos._

➥ Identifique classes de usuário, papéis e personas, observando expertise, níveis de acesso, frequência de uso, necessidades de acessibilidade e objetivos.

💡 Dicas:
- Defina classes de usuário por comportamento, não apenas por cargos.
- Observe considerações de localização e acessibilidade que afetam os requisitos de UI/UX.

### 2.5 Premissas e Dependências
💬 _Fatores ou condições externas presumidas, em oposição a fatos conhecidos, das quais o projeto depende._

➥ Liste premissas sobre ambiente, hardware, padrões de uso, componentes/serviços de terceiros e suporte organizacional. Liste dependências de sistemas externos, bibliotecas ou equipes. Para cada uma, indique o impacto potencial se for provada falsa.

💡 Dicas:
- Vincule premissas ao registro de riscos com responsável e mitigação quando disponível.

### 2.6 Distribuição de Requisitos
💬 _Alocação de requisitos entre componentes ou incrementos._

➥ Mapeie os principais requisitos para subsistemas, serviços ou releases/iterações. Use uma tabela de referência cruzada para mostrar a alocação e identificar claramente os requisitos adiados.

💡 Dicas:
- Observe alocações desconhecidas explicitamente e acompanhe como itens de acompanhamento.

## 3. Requisitos
💬 _Esta seção especifica requisitos **verificáveis** do produto de software para permitir o design e os testes._

➥ Declare os requisitos com nível de detalhe suficiente para design e verificação. Use identificadores únicos, palavras-chave consistentes (deve/deveria/pode) e condições claras. Descreva entradas, processamento em resposta e saídas quando aplicável. Referencie a Restrição do Produto 2.3 relevante que o requisito aborda.

📃 Modelo (aplica-se a **todos** os requisitos):
```markdown
- ID: REQ-FUNC-001
- Título: Título curto, representativo do requisito...
- Declaração: O sistema deve...
- Justificativa: ...
- Critérios de Aceitação: ...
- Método de Verificação: Teste | Análise | Inspeção | Demonstração | Outro
- Mais Informações: Contexto adicional. Links para artefatos relacionados.
```

Esquema de ID de requisito e rastreabilidade:
- Formato do ID: REQ-[AREA]-[NNN]-[VER] (opcional -[VER] se versionado), onde AREA ∈ {FUNC, INT, PERF, SEC, REL, AVAIL, OBS, COMP, INST, BUILD, DIST, MAINT, REUSE, PORT, COST, DEAD, POC, CM, ML}.
- Unicidade: IDs devem ser únicos e imutáveis; alterações incrementam -[VER] e são registradas no Histórico de Revisões.
- Rastreabilidade: Cada artefato de teste pode referenciar o ID do requisito.

💡 Dicas:
- Torne cada requisito testável e não ambíguo, usando métricas padrão e evitando termos vagos (ex.: "amigável", "rápido").

### 3.1 Interfaces Externas
💬 _Especifica todas as entradas e saídas externas, cobrindo interfaces requeridas e fornecidas._

➥ Forneça definições de interface suficientes para implementação e teste.

💡 Dicas:
- Use documentos de controle de interface ou esquemas quando apropriado e referencie-os aqui.

#### 3.1.1 Interfaces de Usuário
💬 _Descreve como os usuários interagem com o sistema em um nível lógico._

➥ Defina elementos de UI, fluxos e padrões a serem seguidos (guias de estilo, diretrizes de acessibilidade). Inclua restrições de layout, controles comuns (ex.: ajuda, busca), atalhos de teclado, comportamento de erro/estado vazio e localização. Mantenha os designs visuais em uma especificação de UI separada e referencie-os.

💡 Dicas:
- Referencie padrões de acessibilidade (ex.: WCAG) e diretrizes específicas de plataforma.
- Considere organizar em subcategorias para clareza: Usabilidade/Acessibilidade e Conveniência.

#### 3.1.2 Interfaces de Hardware
💬 _Detalha interações com dispositivos físicos e plataformas._

➥ Especifique tipos de dispositivos (in)compatíveis, sinais de dados/controle, características elétricas ou mecânicas se relevante e protocolos de comunicação. Inclua expectativas de temporização, vazão e confiabilidade.

💡 Dicas:
- Referencie especificações de hardware aplicáveis e requisitos de certificação.

#### 3.1.3 Interfaces de Software
💬 _Define integrações com outros componentes e serviços de software._

➥ Liste sistemas conectados (nome e versão), serviços/APIs requeridos ou fornecidos, itens de dados/mensagens trocados, estilos/protocolos de comunicação e semânticas de limite/erro/timeout. Identifique dados compartilhados e propriedade.

💡 Dicas:
- Capture políticas de versionamento e compatibilidade retroativa.
- Defina expectativas de autenticação/autorização para cada integração.

### 3.2 Funcionais
💬 _Especifica os comportamentos e funções externamente observáveis que o software deve fornecer._

➥ Organize os requisitos funcionais por funcionalidade, caso de uso ou serviço. Para cada um, descreva gatilhos/entradas, processamento/lógica (em nível de caixa-preta), saídas e condições de erro. Para comportamentos de IA, defina limites de determinismo (ex.: temperatura), critérios de recusa, regras de segurança e pontos de revisão humana.

💡 Dicas:
- Inclua casos extremos e cenários negativos para completude.
- Para funcionalidades de IA, inclua comportamentos de fallback e limites para abstenção.

### 3.3 Qualidade de Serviço
💬 _Atributos de qualidade que restringem ou qualificam o comportamento funcional._

➥ Use métricas, intervalos e condições específicas.

💡 Dicas:
- Quando uma qualidade se aplica apenas a um subconjunto de funções, referencie os IDs de requisito relacionados.
- Forneça justificativa quando as metas abrangem funções para auxiliar decisões de trade-off.

#### 3.3.1 Desempenho
💬 _Expectativas de tempo de resposta, vazão e uso de recursos._

➥ Especifique relações de temporização, cargas de pico/estado estacionário e metas de desempenho sob condições esperadas. Inclua métodos de medição, ambientes e limites de aceitação. Observe quaisquer restrições de tempo real.

💡 Dicas:
- Inclua metas de escalabilidade e premissas de planejamento de capacidade.
- Considere organizar em subcategorias para clareza: Tempo (latência, vazão, etc.) e Espaço (memória, armazenamento, largura de banda, etc.).

#### 3.3.2 Segurança
💬 _Define a proteção de dados, identidades e operações._

➥ Defina requisitos de autenticação, autorização, proteção de dados (em trânsito/em repouso), auditoria e privacidade. Aborde abuso/mau uso e ataques externos (ex.: injeção, exfiltração de dados ou comprometimento de serviço) e inclua padrões seguros e requisitos de resposta a incidentes.

💡 Dicas:
- Diferencie controles obrigatórios vs. práticas recomendadas.
- Considere organizar em subcategorias para clareza: Segurança física (resultados externos prejudiciais), Confidencialidade, Privacidade, Integridade e Disponibilidade.

📝 Nota:
Coloque controles de segurança genéricos aqui (3.3.2) e faça referência cruzada com os controles de suporte conforme necessário:
- Use 3.1 Interfaces Externas para validação em nível de interface e protocolos seguros.
- Use 3.4 Conformidade para obrigações regulatórias/contratuais e evidências de auditoria.
- Use 3.6 IA/ML para proteções de tempo de execução específicas do modelo e governança de dados.

#### 3.3.3 Confiabilidade
💬 _Capacidade de executar consistentemente conforme especificado._

➥ Especifique métricas e técnicas de confiabilidade (ex.: MTBF, orçamentos de erro, retry/backoff, idempotência, redundância). Defina as condições sob as quais a confiabilidade é avaliada e quaisquer comportamentos de failover. Defina degradação graciosa (ex.: componentes de fallback, resultados em cache, heurísticas determinísticas de IA/ML), políticas de timeout/abstenção e rollback para versões anteriores.

#### 3.3.4 Disponibilidade
💬 _Tempo de atividade do sistema e prontidão para entregar serviço._

➥ Defina metas de disponibilidade, janelas de manutenção e mecanismos como checkpointing, recuperação e reinicialização. Inclua redundância geográfica/de zona se aplicável.

💡 Dicas:
- Expresse a disponibilidade em termos significativos para os usuários (ex.: tempo de inatividade por mês) e vincule a SLAs/SLOs.
- Capture o comportamento de scale-out/in que afeta a disponibilidade (ex.: tempo máximo de failover, restrições de quórum).

#### 3.3.5 Observabilidade
💬 _Capacidade de entender o estado e o comportamento do sistema em produção através de telemetria._

➥ Defina requisitos para logs, métricas, traces e profiling: eventos/campos, limites de cardinalidade, amostragem, retenção e tratamento de privacidade/PII na telemetria. Especifique rótulos padrão (ex.: serviço, versão, tenant), propagação de IDs de correlação/trace e políticas de redação. Declare regras de alerta alinhadas com SLO, dashboards e propriedade.

💡 Dicas:
- Evite detalhes de processo de manutenção (mantenha runbooks e políticas de plantão em 3.5.4 Manutenibilidade).

### 3.4 Conformidade
💬 _Requisitos derivados para satisfazer padrões externos, regulamentações ou contratos._

➥ Especifique formatos obrigatórios, convenções de nomenclatura, procedimentos contábeis, direitos e acordos do provedor/usuário, acordos de licenciamento, rastreamento de auditoria, retenção de registros e relatórios. Para cada item de conformidade, referencie 2.3 Restrições do Produto se aplicável, ou cite a fonte autoritativa diretamente.

### 3.5 Design e Implementação
💬 _Restrições ou mandatos que afetam como a solução é projetada, implantada e mantida._

#### 3.5.1 Instalação
💬 _Garante que o software funcione corretamente em seus ambientes de destino._

➥ Defina plataformas/ambientes (in)compatíveis, pré-requisitos, métodos de instalação, configuração de ambiente (ex.: variáveis de ambiente, segredos) e procedimentos de rollback/desinstalação.

💡 Dicas:
- Detalhe expectativas de automação (ex.: IaC, scripts de instalação, imagens de container).
- Mantenha mecânicas de escalabilidade (topologia, multi-região) em 3.5.3 Distribuição; mantenha metas de escalabilidade em 3.3 QoS.

#### 3.5.2 Build e Entrega
💬 _Define os controles para construir, empacotar e entregar artefatos de software para garantir integridade, rastreabilidade e reprodutibilidade._

➥ Defina como o código-fonte é transformado em artefatos implantáveis e movido através dos ambientes. Descreva expectativas de reprodutibilidade de build, gerenciamento de dependências, licenciamento, gerenciamento de configuração, verificação de artefatos e promoção de releases.

💡 Dicas:
- Faça referência cruzada com 3.5.1 Instalação e 3.5.10 Gerenciamento de Mudanças para configuração de ambiente, versionamento e rastreabilidade de releases.
- Evite detalhes de topologia operacional (estes pertencem a 3.5.3 Distribuição).

#### 3.5.3 Distribuição
💬 _Aborda implantações distribuídas geográfica ou organizacionalmente, dados e dispositivos._

➥ Especifique topologias de implantação, abordagens de distribuição/replicação de componentes e dados e runbooks de scale-out, e restrições impostas pela estrutura organizacional ou de rede.

#### 3.5.4 Manutenibilidade
💬 _Atributos que tornam o software mais fácil de modificar, corrigir e evoluir._

➥ Defina expectativas de modularidade, complexidade de código, interfaces, padrões de codificação, observabilidade orientada ao desenvolvedor, documentação, desempenho de entrega de software e gerenciamento de dívida técnica.

#### 3.5.5 Reusabilidade
💬 _Incentiva o aproveitamento de componentes entre produtos ou contextos quando apropriado._

➥ Identifique componentes destinados à reutilização e quaisquer restrições sobre suas dependências ou escolhas tecnológicas. Especifique modularização, estabilidade de API, empacotamento e documentação para permitir a reutilização.

#### 3.5.6 Portabilidade
💬 _Capacidade de executar em múltiplas plataformas ou ambientes com mudanças mínimas._

➥ Especifique sistemas operacionais, arquiteturas de hardware, provedores de nuvem ou runtimes de container (in)compatíveis. Defina camadas de abstração, políticas de configuração e externalização de configurações específicas do ambiente.

#### 3.5.7 Custo
💬 _Considerações financeiras ou metas de custo._

➥ Declare limites orçamentários, metas de custo por transação, restrições de licenciamento ou envelopes de gastos em nuvem que influenciam as decisões de design.

💡 Dicas:
- Mantenha os custos em alto nível, a menos que definidos contratualmente.
- Vincule a um modelo de custo ou premissas de TCO quando disponível.
- Observe expectativas de custo variável vs. fixo que impactam estratégias de escalabilidade.

#### 3.5.8 Prazo
💬 _Expectativas de cronograma que afetam o escopo e a priorização._

➥ Especifique marcos principais, datas de entrega ou fases/incrementos. Indique dependências entre marcos e critérios de prontidão exigidos.

💡 Dicas:
- Use prazos para orientar a distribuição de requisitos (Seção 2.6).

#### 3.5.9 Prova de Conceito
💬 _Valida a viabilidade e reduz riscos de premissas críticas antes da entrega em escala completa._

➥ Defina os objetivos, escopo, critérios de sucesso e timebox para quaisquer POCs. Descreva o que será validado (técnico, usabilidade, desempenho) e como os resultados influenciarão os requisitos ou o design.

💡 Dicas:
- Mantenha as POCs com foco restrito e mensuráveis. Foque nos objetivos de validação, não nos detalhes de implementação.

#### 3.5.10 Gerenciamento de Mudanças
💬 _Controla como as mudanças são introduzidas e comunicadas._

➥ Defina categorias de mudança (quebra, aditiva, correção de bug), fluxo de aprovação e artefatos necessários (changelogs, resumos de avaliação, guias de migração, notas de release). Especifique garantias de compatibilidade retroativa/forward, planos de comunicação com o cliente, cronogramas de depreciação e procedimentos de rollout/rollback.

### 3.6 IA/ML
💬 _Esta seção define requisitos exclusivos para sistemas que incorporam aprendizado de máquina ou componentes orientados a dados em seu núcleo. Esses requisitos complementam os aspectos funcionais, de qualidade e de design nas seções anteriores, mas abordam considerações específicas do ciclo de vida de ML, dados e ética._

#### 3.6.1 Especificação do Modelo
💬 _Define o que cada modelo deve fazer e os critérios mensuráveis para desempenho aceitável._

➥ Descreva o propósito, escopo, comportamento esperado, principais entradas e saídas do(s) modelo(s) e objetivos de desempenho mensuráveis. Observe quaisquer conjuntos de dados de validação, benchmarks ou práticas de versionamento usadas para garantir a reprodutibilidade.

💡 Dicas:
- Diferencie metas de linha de base de melhorias aspiracionais e defina tolerância aceitável para drift.

#### 3.6.2 Gerenciamento de Dados
💬 _Garante integridade, rastreabilidade e ciclo de vida ético dos dados usados no treinamento, validação e operação do modelo._

➥ Especifique origem do conjunto de dados, propriedade, condições de consentimento; processos de rotulagem e controles de qualidade; linhagem de dados, versionamento e reprodutibilidade (treinamento → validação → inferência); armazenamento, controles de acesso e padrões de anonimização/pseudonimização; tratamento de dados ausentes, sintéticos ou aumentados.

#### 3.6.3 Guardrails
💬 _Garantem que o sistema de IA opere de forma segura, previsível e dentro dos limites aprovados._

➥ Especifique como o sistema valida entradas, filtra ou restringe saídas e limita as ações disponíveis para prevenir danos, mau uso ou consequências não intencionais. Inclua mecanismos para detectar e responder a entradas maliciosas ou condições operacionais inseguras.

💡 Dicas:
- Trate "guardrails" nas camadas de entrada, saída e ação.
- Defina procedimentos de escalonamento, logging e rollback quando as restrições de segurança forem acionadas.
- Faça referência cruzada com 3.3.2 Segurança para proteções em nível de sistema e 3.6.4 Ética para expectativas normativas.

#### 3.6.4 Ética
💬 _Aborda justiça, transparência e responsabilidade no comportamento e nos resultados do modelo._

➥ Defina como as considerações éticas serão identificadas, medidas e gerenciadas ao longo do desenvolvimento e operação. Inclua objetivos de justiça, expectativas de explicabilidade e requisitos de documentação ou revisão.

💡 Dicas:
- Use métricas de justiça apropriadas ao contexto (ex.: paridade demográfica, igualdade de oportunidade).
- Considere organizar em subcategorias para clareza: Justiça, Interpretabilidade e Explicabilidade.
- Coordene com 3.6.3 Guardrails para mecanismos de aplicação e 3.6.5 Human-in-the-Loop para supervisão humana.

#### 3.6.5 Human-in-the-Loop
💬 _Especifica o papel da supervisão humana nas decisões influenciadas ou tomadas por modelos de aprendizado de máquina._

➥ Descreva onde e como a revisão, aprovação ou intervenção humana é necessária. Esclareça expectativas de latência ou vazão de revisão, caminhos de escalonamento, mecanismos de feedback, rastreabilidade e auditabilidade das ações humanas.

💡 Dicas:
- Vincule aos papéis aplicáveis definidos em 2.4 Características do Usuário.

#### 3.6.6 Ciclo de Vida e Operações do Modelo
💬 _Define requisitos para implantar, monitorar, retreinar e descontinuar modelos em produção._

➥ Descreva como os modelos transitam do desenvolvimento para a produção, como seu desempenho e qualidade dos dados são monitorados e como o retreinamento ou rollback é acionado e gerenciado. Inclua expectativas de versionamento e arquivamento.

## 4. Verificação
💬 _Descreve como cada requisito será verificado para fornecer evidência objetiva de conformidade._

➥ Descreva os métodos de verificação (teste, métricas canário, análise, inspeção, demonstração) e evidências de teste preferencialmente em uma matriz paralela à Seção 3. Considere adicionar detalhes de ambiente, ferramentas e requisitos de dados de teste.

| ID do Requisito | Método de Verificação | Link do Teste/Artefato | Status | Evidência           |
|-----------------|-----------------------|------------------------|--------|---------------------|
| REQ-FUNC-001    | teste                 | testes/UC01.md         | Passou | relatorios/tuc01.html |
| REQ-SEC-003     | análise               | modelo-ameacas.md      | WIP    |                     |

💡 Dicas:
- Inclua testes positivos e negativos e inclua verificação não funcional (desempenho, segurança, confiabilidade).
- Os artefatos de verificação podem ser versionados e vinculados ao CI/CD.
- Para IA, referencie Model Cards e acompanhe as versões dos conjuntos de dados de avaliação e garanta a reprodutibilidade dos resultados.

## 5. Apêndices
💬 _Material de apoio opcional que auxilia na compreensão sem ser normativo._

➥ Inclua glossários, dicionários de dados, modelos/diagramas, conjuntos de dados de amostra ou análises de impacto de mudanças que apoiam as seções principais. Referencie em vez de duplicar o conteúdo quando possível.

💡 Dicas:
- Mantenha os apêndices organizados e referenciados a partir do texto principal.