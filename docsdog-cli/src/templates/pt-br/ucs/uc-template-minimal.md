---
# Estes são elementos de metadados opcionais. Sinta-se à vontade para remover qualquer um deles.
status: "{draft | reviewed | approved | implemented | verified | deferred | waived}"
date: {YYYY-MM-DD quando o caso de uso foi atualizado pela última vez}
priority: "{critical | high | medium | low}"
source: {ID do requisito SRS ou referência de história de usuário}
---

# {nome do caso de uso em frase verbal, representando o objetivo do ator}

| ID do Caso de Uso | {UC-NNN} |
|-------------------|----------|

## Descrição
💬 _Um resumo conciso do caso de uso — o que o ator alcança._

➥ Em 1–3 frases, descreva o objetivo e o contexto. Este é o "elevator pitch" do caso de uso.

💡 Dicas:
- Foque no resultado, não nos passos.
- Mencione o ator principal e o valor entregue.

## Atores
💬 _Quem participa deste caso de uso._

➥ Liste o ator principal primeiro, seguido por quaisquer atores secundários (sistemas externos, serviços ou papéis de apoio).

💡 Dicas:
- Ator principal: aquele que inicia o caso de uso para alcançar um objetivo.
- Atores secundários: aqueles com quem o sistema interage durante a execução, mas que não iniciam.

- **Principal:** Cliente
- **Secundário:** Gateway de Pagamento, Serviço de E-mail

## Pré-condições
💬 _O que deve ser verdadeiro antes que o caso de uso possa começar._

➥ Liste pré-condições aplicadas pelo sistema. Estas são garantias, não premissas sobre o comportamento do ator.

💡 Dicas:
- "Cliente está autenticado" ✓
- "Cliente abre o aplicativo" ✗ (isso é um passo, não uma pré-condição)

## Pós-condições
💬 _O que é verdadeiro após o caso de uso ser concluído com sucesso._

➥ Descreva o estado do sistema, mudanças de dados e a situação do ator após o objetivo ser alcançado.

💡 Dicas:
- Para casos de erro, confie nos fluxos de exceção para descrever resultados parciais. As pós-condições aqui são sobre o sucesso.

## Fluxo Básico
💬 _O caminho feliz — a sequência de passos quando tudo dá certo._

➥ Escreva uma lista numerada de passos descrevendo ações do ator e respostas do sistema. Mantenha conciso — 5–10 passos.

💡 Dicas:
- Use o formato: "<número>. <Ator> <ação>. O Sistema <resposta>."
- Mantenha um nível consistente de abstração. Evite mergulhar em detalhes de UI ou implementação.
- Cada passo deve representar uma interação, não uma sub-rotina.

1. Cliente seleciona "Finalizar Compra" do carrinho.
2. Sistema apresenta o resumo do pedido, opções de envio e métodos de pagamento salvos.
3. Cliente confirma o endereço de entrega e seleciona um método de pagamento.
4. Sistema valida o método de pagamento e calcula o total final.
5. Cliente confirma o pedido.
6. Sistema processa o pagamento, cria o pedido e exibe uma página de confirmação com o número do pedido.

## Fluxos Alternativos
💬 _Ramificações ou variações válidas do fluxo básico — maneiras diferentes de realizar o mesmo passo._

➥ Liste caminhos alternativos que o ator pode tomar e que ainda levam ao sucesso. Estes não são erros — são escolhas.

💡 Dicas:
- Referencie de qual passo eles se ramificam.
- Mantenha cada fluxo alternativo autocontido ou retorne a um passo no fluxo básico.

- **No passo 3 — Cliente adiciona um novo endereço de entrega:**
    1. Sistema apresenta o formulário de cadastro de endereço.
    2. Cliente preenche e salva o novo endereço.
    3. Retorna ao passo 3 com o novo endereço selecionado.
- **No passo 3 — Cliente alterna entre métodos de pagamento salvos:**
    1. Sistema exibe a lista de métodos de pagamento salvos.
    2. Cliente seleciona um método diferente.
    3. Retorna ao passo 3.

## Fluxos de Exceção
💬 _Condições de erro, falhas e caminhos de recuperação._

➥ Liste coisas que podem dar errado, como o sistema as detecta e como ele responde ou se recupera.

💡 Dicas:
- Inclua timeouts, falhas de validação, erros de sistemas externos e violações de regras de negócio.
- Especifique se o caso de uso pode continuar, tentar novamente ou deve terminar.

- **No passo 4 — Método de pagamento está expirado:**
    1. Sistema marca o método como expirado e notifica o Cliente.
    2. Cliente deve selecionar ou adicionar um método de pagamento diferente.
- **No passo 4 — Validação do pagamento falha (recusado):**
    1. Sistema exibe o motivo da recusa (sem expor dados sensíveis).
    2. Cliente pode tentar novamente com um método diferente ou cancelar o caso de uso.
- **No passo 6 — Pagamento é bem-sucedido mas a criação do pedido falha:**
    1. Sistema estorna o pagamento e notifica o Cliente.
    2. Sistema registra a inconsistência para investigação de operações.
- **No passo 6 — Pagamento expira:**
    1. Sistema notifica o Cliente sobre o timeout.
    2. Sistema agenda um job de reconciliação de status do pagamento.
    3. Cliente é aconselhado a verificar o status do pedido em 5 minutos.

## Mais Informações
💬 _Contexto de apoio, referências cruzadas e itens em aberto._

➥ Vincule a requisitos SRS relacionados, ADRs, designs de UI, suítes de teste ou anote quaisquer questões em aberto ou premissas.

💡 Dicas:
- Mantenha orientado a links em vez de duplicar conteúdo.
- Sinalize premissas ou decisões pendentes que possam afetar este caso de uso.