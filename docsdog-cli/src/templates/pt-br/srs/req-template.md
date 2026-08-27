---
# Estes são elementos de metadados opcionais. Sinta-se à vontade para remover qualquer um deles.
status: "{draft | proposed | deferred | planned | in-progress | blocked | passed | failed | waived}"
date: {YYYY-MM-DD quando o requisito foi atualizado pela última vez}
---

# <!-- título curto, representativo do requisito -->

## Declaração
➥ Escreva uma declaração de requisito única e testável que especifique comportamentos ou condições observáveis. Use linguagem clara e ativa focada no "o quê" o sistema deve fazer, não no "como" será implementado.

O sistema deve...

💡 Dicas:
- Use linguagem normativa (ex.: "deve") para requisitos obrigatórios e evite qualificadores vagos como "razoável" ou "adequado."

## Justificativa
💬 _Por que o requisito existe — seu valor, intenção e o problema que aborda._

➥ Vincule o requisito a objetivos de negócio, necessidades do usuário, obrigações de conformidade ou mitigação de riscos. Indique os stakeholders principais e as consequências de não atender ao requisito.

💡 Dicas:
- Mantenha a justificativa concisa (uma a três frases) e evite repetir a declaração.
- Mencione quaisquer premissas/dependências que afetem a necessidade do requisito.

## Critérios de Aceitação
➥ Liste critérios verificáveis que constituem o sucesso para o requisito. Especifique saídas observáveis, limites numéricos, restrições de tempo e condições de aprovação/reprovação. Use marcadores ou uma tabela curta para clareza e referencie casos de teste ou dados específicos quando disponíveis.

💡 Dicas:
- Torne cada critério independentemente verificável e não ambíguo; prefira critérios que possam ser verificados com um resultado sim/não.
- Use unidades precisas, tolerâncias e tamanhos de amostra para atributos não funcionais (ex.: latência, vazão, precisão).

## Método de Verificação
Teste | Análise | Inspeção | Demonstração | Outro

➥ Descreva brevemente a abordagem de verificação. Para Testes, referencie IDs de procedimentos ou suítes de teste; para Análise, declare modelos e premissas; para Inspeção ou Demonstração, descreva artefatos ou cenários a serem examinados.

💡 Dicas:
- Se múltiplos métodos se aplicam, indique métodos primários e secundários e seus gatilhos.

## Mais Informações
➥ Forneça contexto de apoio, links e referências que ajudem implementadores e verificadores a entender o requisito.

💡 Dicas:
- Observe quaisquer issues em aberto, premissas ou decisões pendentes que possam afetar o requisito.