---
# Estes são elementos de metadados opcionais. Sinta-se à vontade para remover qualquer um deles.
status: "{proposed | rejected | accepted | deprecated | … | superseded by ADR-0123}"
date: {YYYY-MM-DD quando a decisão foi atualizada pela última vez}
decision-makers: {liste todos os envolvidos na decisão}
consulted: {liste todos cujas opiniões são consultadas (normalmente especialistas no domínio); e com quem há comunicação bidirecional}
informed: {liste todos que são mantidos atualizados sobre o progresso; e com quem há comunicação unidirecional}
---

# {título curto, representativo do problema resolvido e da solução encontrada}

## Contexto e Declaração do Problema

{Descreva o contexto e a declaração do problema, por exemplo, em formato livre usando duas a três frases ou na forma de uma história ilustrativa. Você pode querer articular o problema na forma de uma pergunta. Considere adicionar links para quadros de colaboração ou sistemas de gerenciamento de issues. Torne o escopo da decisão explícito, por exemplo, destacando ou apontando elementos estruturais da arquitetura (componentes, conectores, ...).}

<!-- Este é um elemento opcional. Sinta-se à vontade para remover. -->
## Motivadores da Decisão

* {motivador da decisão 1, por exemplo, uma qualidade de software desejada, preocupação enfrentada, restrição ou força}
* {motivador da decisão 2}
* … <!-- a quantidade de motivadores pode variar -->

## Opções Consideradas

* {título da opção 1}
* {título da opção 2}
* {título da opção 3}
* … <!-- a quantidade de opções pode variar -->

## Decisão

Opção escolhida: "{título da opção 1}", porque {justificativa. Ex.: única opção que atende ao critério eliminatório do motivador da decisão | que resolve a força {força} | … | sai-se melhor (veja abaixo)}.

<!-- Este é um elemento opcional. Sinta-se à vontade para remover. -->
### Consequências

* Boa, porque {consequência positiva, ex.: melhoria de uma ou mais qualidades desejadas, …}
* Ruim, porque {consequência negativa, ex.: comprometimento de uma ou mais qualidades desejadas, …}
* … <!-- a quantidade de consequências pode variar -->

<!-- Este é um elemento opcional. Sinta-se à vontade para remover. -->
### Confirmação

{Descreva como a implementação / conformidade da ADR pode/será confirmada. Existe alguma função de fitness automatizada ou manual? Em caso afirmativo, liste-a e explique como é aplicada. O design escolhido e sua implementação estão alinhados com a decisão? Ex.: uma revisão de design/código ou um teste com uma biblioteca como ArchUnit pode ajudar a validar isso. Note que, embora classifiquemos este elemento como opcional, ele está incluído em muitas ADRs.}

<!-- Este é um elemento opcional. Sinta-se à vontade para remover. -->
## Prós e Contras das Opções

### {título da opção 1}

<!-- Este é um elemento opcional. Sinta-se à vontade para remover. -->
{exemplo | descrição | link para mais informações | …}

* Bom, porque {argumento a}
* Bom, porque {argumento b}
<!-- use "neutro" se o argumento dado não pesa nem para bom nem para ruim -->
* Neutro, porque {argumento c}
* Ruim, porque {argumento d}
* … <!-- a quantidade de prós e contras pode variar -->

### {título da outra opção}

{exemplo | descrição | link para mais informações | …}

* Bom, porque {argumento a}
* Neutro, porque {argumento b}
* Ruim, porque {argumento c}
* …

<!-- Este é um elemento opcional. Sinta-se à vontade para remover. -->
## Mais Informações

{Você pode querer fornecer evidências/confiança adicionais para o resultado da decisão aqui e/ou documentar o acordo da equipe sobre a decisão e/ou definir quando/como esta decisão deve ser realizada e se/quando deve ser revisitada. Links para outras decisões e recursos podem aparecer aqui também.}