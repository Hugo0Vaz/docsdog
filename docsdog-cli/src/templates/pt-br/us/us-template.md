---
# Estes são elementos de metadados opcionais. Sinta-se à vontade para remover qualquer um deles.
status: "{draft | ready | in-progress | done | deferred | waived}"
date: {YYYY-MM-DD quando a história foi atualizada pela última vez}
priority: "{critical | high | medium | low}"
effort: "{XS | S | M | L | XL}"
source: {ID do requisito SRS ou referência de épico}
---

# {título curto, representativo da história de usuário}

## História de Usuário

💬 O formato clássico de história de usuário: quem, o quê e por quê.

➥ Escreva uma única frase na forma:

> Como **{papel}**, quero **{funcionalidade/capacidade}** para que **{benefício/motivo}**.

💡 Dicas:
- Mantenha o papel concreto — uma persona ou classe de usuário, não um cargo.
- O benefício deve ser o verdadeiro porquê, não uma reformulação da funcionalidade.
- Se você não consegue articular o benefício, a história pode não estar pronta.

## Critérios de Aceitação

💬 Condições observáveis e testáveis que devem ser atendidas para que a história seja considerada concluída.

➥ Liste cada critério como um marcador, formulado como uma declaração testável.

💡 Dicas:
- Use o formato: "Dado {pré-condição}, Quando {ação}, Então {resultado esperado}".
- Cada critério deve ser independentemente verificável — respondível com sim/não.
- Se um critério requer um ambiente, conjunto de dados ou papel específico, declare-o explicitamente.

* Dado ..., Quando ..., Então ...
* Dado ..., Quando ..., Então ...

## Observações

💬 Dicas de implementação, restrições de design, dependências, riscos ou questões em aberto que afetam esta história.

➥ Capture qualquer coisa que a equipe precise saber além da declaração da história e dos critérios de aceitação. Mantenha breve — prefira links para especificações detalhadas ou ADRs.

💡 Dicas:
- Observe dependências de outras histórias, APIs ou sistemas externos.
- Sinalize premissas que, se provadas erradas, mudariam o escopo ou a abordagem.

## Mais Informações

💬 Links de apoio, referências e rastreabilidade.

➥ Vincule a artefatos relacionados: requisitos do SRS, ADRs, casos de uso, mockups de UI, especificações de API ou casos de teste.

💡 Dicas:
- Mantenha esta seção orientada a links em vez de duplicar informações.
- Referencie o épico ou funcionalidade pai se esta história fizer parte de um esforço maior.