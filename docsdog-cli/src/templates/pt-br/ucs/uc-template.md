---
# Estes são elementos de metadados opcionais. Sinta-se à vontade para remover qualquer um deles.
status: "{draft | reviewed | approved | implemented | verified | deferred | waived}"
date: {YYYY-MM-DD quando o caso de uso foi atualizado pela última vez}
priority: "{critical | high | medium | low}"
frequency: "{once | daily | weekly | monthly | on-demand | continuous}"
source: {ID do requisito SRS ou referência de história de usuário}
---

# {nome do caso de uso em frase verbal, representando o objetivo do ator}

| ID do Caso de Uso | {UC-NNN} |
|-------------------|----------|

## Escopo
💬 _O limite do sistema — a "caixa" que este caso de uso descreve._

➥ Declare qual sistema ou subsistema está sob design. Isso deve corresponder ao Limite do Sistema definido na UCS (Seção 2.1).

💡 Dicas:
- Use o mesmo nome e limite do sistema consistentemente em todos os casos de uso.
- Exemplo: "A plataforma de e-commerce Acme" ou "O subsistema de Processamento de Pedidos."

## Nível
💬 _O nível de abstração deste caso de uso, conforme a hierarquia de Cockburn._

➥ Escolha um:
- **Summary (Resumo)** — Um processo de negócio de alto nível que abrange múltiplos objetivos de usuário, frequentemente referenciando outros casos de uso em seus passos.
- **User-goal (Objetivo do usuário)** — A unidade primária de trabalho, concluída em uma única sessão por um ator. Este é o nível padrão para a maioria dos casos de uso.
- **Subfunction (Subfunção)** — Um sub-passo reutilizável chamado por outros casos de uso; não é um objetivo de usuário independente.

💡 Dicas:
- A maioria dos casos de uso deve ser user-goal. Resumos fornecem contexto; subfunções extraem comportamento compartilhado.
- Se você se encontrar com mais de 20 passos no cenário de sucesso principal, pode estar misturando passos de user-goal e subfunção.

## Ator Principal
💬 _O ator que inicia a interação para alcançar o objetivo declarado._

➥ Nomeie o ator principal — um papel, não uma pessoa ou instância de sistema específica.

💡 Dicas:
- Use o nome do ator do Catálogo de Atores (UCS Seção 2.2), que deriva da Seção 2.4 Características do Usuário do SRS. Mantenha os nomes dos atores consistentes entre os documentos para manter a rastreabilidade.
- Se o gatilho for baseado em tempo (ex.: um agendador), trate o agendador como o ator principal.

## Atores Secundários
💬 _Atores com os quais o sistema interage durante o caso de uso, mas que não o iniciam._

➥ Liste quaisquer atores de apoio (sistemas externos, serviços ou papéis humanos) que participam.

💡 Dicas:
- Isto é opcional se nenhum ator secundário estiver envolvido.
- Cada ator secundário deve aparecer em pelo menos um passo do cenário de sucesso principal ou extensões.

## Partes Interessadas e Interesses
💬 _Todos com interesse no resultado deste caso de uso e o que lhes importa. Esta é uma assinatura do Cockburn — revela expectativas conflitantes antes do design começar._

➥ Liste cada grupo de stakeholders e seu interesse/preocupação principal em relação a este caso de uso. Seja específico.

💡 Dicas:
- Vá além do óbvio (ex.: Marketing pode querer análises, o Encarregado de Privacidade pode querer coleta mínima de dados, o CFO pode querer prevenção de fraudes).
- Esta tabela frequentemente revela requisitos ocultos — não a pule para casos de uso de alto risco.

| Stakeholder            | Interesse / Preocupação                                  |
|------------------------|----------------------------------------------------------|
| Cliente                | Checkout rápido e sem erros; confirmação do pedido       |
| Marketing              | Análises de conversão; rastreamento de carrinho abandonado |
| Gateway de Pagamento   | Códigos de erro claros; limites de retentativa; sinais de fraude |
| Encarregado de Privacidade | PII minimizado; consentimento capturado; retenção de dados respeitada |
| CFO                    | Sem cobranças duplicadas; reconciliação precisa          |

## Pré-condições
💬 _O que deve ser verdadeiro antes que o caso de uso possa começar._

➥ Liste as pré-condições — estados, dados ou condições garantidas pelo sistema no início. Estas são aplicadas pelo sistema, não presumidas do ator.

💡 Dicas:
- Pré-condições são garantias aplicadas pelo sistema, não ações do ator (ex.: "Cliente está autenticado" ✓; "Cliente abre o aplicativo" ✗).
- Evite listar o gatilho como uma pré-condição.

## Garantias Mínimas
💬 _O que o sistema promete mesmo quando o caso de uso falha._

➥ Declare as garantias que o sistema mantém independentemente do resultado. Isto é crítico para auditoria, conformidade e confiança.

💡 Dicas:
- Exemplos: "Nenhum dinheiro muda de mãos," "A tentativa é registrada para auditoria," "Sem atualizações parciais," "O usuário é retornado a um estado seguro."
- Se o caso de uso não tem garantias de falha significativas, declare "Nenhuma" explicitamente.

## Garantias de Sucesso
💬 _O que é verdadeiro após uma conclusão bem-sucedida._

➥ Descreva as pós-condições — o estado do sistema, os dados e a situação do ator após o objetivo ser alcançado.

💡 Dicas:
- Seja específico: "Pedido é persistido com status 'confirmado', inventário é decrementado, um e-mail de confirmação é enfileirado para entrega."

## Gatilho
💬 _O evento que faz o caso de uso começar._

➥ Declare o gatilho como um evento simples (ex.: "Cliente toca em 'Finalizar Pedido'," "Agendador dispara à meia-noite UTC," "Agente de suporte seleciona 'Iniciar Reembolso'").

## Cenário de Sucesso Principal
💬 _O caminho primário — a sequência de passos quando tudo dá certo._

➥ Escreva uma lista numerada de passos, idealmente de 3 a 9. Cada passo descreve uma única interação entre um ator e o sistema, formulada como uma frase completa.

💡 Dicas:
- Cockburn insiste em cenários curtos. Se você exceder 9 passos, considere dividir em um caso de uso de resumo que chama subfunções.
- Use o formato: "<número>. <Ator> <ação>. O Sistema <resposta>." — sempre tornando o ator e o sistema explícitos.
- Mantenha os passos no nível de abstração user-goal. Não mergulhe em detalhes de campo aqui (é para isso que servem as extensões e subfunções).

1. Cliente seleciona "Finalizar Compra" do carrinho.
2. Sistema apresenta o resumo do pedido, opções de envio e métodos de pagamento salvos.
3. Cliente confirma o endereço de entrega e seleciona um método de pagamento.
4. Sistema valida o método de pagamento e calcula o total final.
5. Cliente confirma o pedido.
6. Sistema processa o pagamento, cria o pedido, decrementa o inventário e exibe uma página de confirmação com o número do pedido.

## Extensões
💬 _Cada ramificação alternativa, de falha ou caso extremo a partir do cenário de sucesso principal. Numeradas pelo passo que estendem._

➥ Para cada passo no cenário de sucesso principal, liste todas as extensões possíveis. Use numeração hierárquica (ex.: 3a, 3a1, 3b).

💡 Dicas:
- Extensões são de primeira classe — cada coisa interessante que pode dar diferente vive aqui. É aqui que o verdadeiro trabalho de especificação acontece.
- Para extensões complexas que merecem seu próprio sub-caso de uso, referencie-o (ex.: "→ Ver UC-004 Validar Método de Pagamento").
- Não se esqueça de timeouts, cancelamentos e falhas de validação de dados.

- **3a. Endereço de entrega é inválido:**
    1. Sistema destaca os campos inválidos e solicita que o Cliente os corrija.
    2. Cliente corrige o endereço.
    3. Retorna ao passo 3.
- **3b. Cliente deseja adicionar um novo endereço de entrega:**
    1. Sistema apresenta o formulário de cadastro de endereço.
    2. Cliente preenche e salva o novo endereço.
    3. Retorna ao passo 3 com o novo endereço selecionado.
- **4a. Método de pagamento selecionado está expirado:**
    1. Sistema marca o método como expirado e notifica o Cliente.
    2. Retorna ao passo 3.
- **4b. Validação do pagamento falha (recusado):**
    1. Sistema exibe o motivo da recusa sem expor dados sensíveis do gateway.
    2. Cliente pode tentar novamente com um método diferente ou cancelar.
    3a. Cliente tenta novamente → Retorna ao passo 3.
    3b. Cliente cancela → Caso de uso termina. Garantias mínimas se aplicam.
- **6a. Pagamento é bem-sucedido mas a criação do pedido falha (condição de corrida de inventário):**
    1. Sistema estorna o pagamento e notifica o Cliente.
    2. Sistema registra a inconsistência para investigação de operações.
- **6b. Pagamento expira:**
    1. Sistema notifica o Cliente sobre o timeout.
    2. Sistema agenda um job de reconciliação de status do pagamento.
    3. Cliente é aconselhado a verificar o status do pedido em 5 minutos.

## Informações Relacionadas
💬 _Contexto adicional que não se encaixa em outro lugar, mas afeta implementação, teste ou priorização._

➥ Capture notas, metas de desempenho, regras de negócio, referências de UI/UX ou questões em aberto.

💡 Dicas:
- Faça referência cruzada com casos de uso relacionados (incluídos ou estendidos).
- Anote quaisquer requisitos não funcionais relevantes para este caso de uso (ex.: "Passo 6 deve completar em 2 segundos p95").
- Sinalize decisões em aberto ou premissas.

| Categoria      | Detalhe                                                                 |
|----------------|-------------------------------------------------------------------------|
| Desempenho     | Processamento de pagamento (passo 6) deve completar em 2 segundos p95  |
| Regra de Negócio | Pedidos acima de R$ 10.000 exigem revisão manual de fraude antes da confirmação |
| Referência UI  | Ver wireframes de checkout v2.1 no Figma                                |
| Questão Aberta | Devemos oferecer buy-now-pay-later no lançamento inicial?              |
| UCs Relacionados | UC-004 (Validar Método de Pagamento), UC-007 (Ver Histórico de Pedidos) |

## Mais Informações
💬 _Links de apoio, notas de design, premissas e referências de rastreabilidade._

➥ Vincule a artefatos relacionados: requisitos do SRS, ADRs que restringem este caso de uso, especificações de UI, contratos de API, suítes de teste ou diagramas de arquitetura.

💡 Dicas:
- Use isto como um repositório para qualquer coisa que ajude futuros leitores a entender o contexto.
- Mantenha-o orientado a links em vez de duplicar informações.