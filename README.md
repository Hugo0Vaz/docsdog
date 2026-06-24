# DocsDog

![docdoc](/assets/docdoc.png)

DocsDog is an open code documentation framework, born from the need to better
integrate formal documentation into coding.

DocsDog is based on:

- A modified specification of [Mardown SRS-Template](https://github.com/jam01/SRS-Template).
- A modified specification of [Mardown Architechtural Decision Records](https://github.com/adr/madr).
- DocsDog CoreSPT specification: [DocsDog SPT Specification](./docsdog-spec/specification.md).
- DocsDog CLI, that provides the linking between docs and artifacts and also checks.

# DocsDog CoreSPT

DocsDog CoreSPT is an open specification for expressing semantic relationships between
source code and external artifacts.


It defines a language-independent metadata model that enables traceability
across software systems, documentation, architecture, requirements, APIs,
infrastructure, and other engineering artifacts.

Because the docsdog's SPT model specification intentionally does not provide the
language implementations and focuses only on defining guardrails of how the
system should work, contributors are free to implement the models in any language.

## Implementations

| Language | Repo                                                         |
|----------|--------------------------------------------------------------|
| PHP      | [Hugo0Vaz/docsdog-php](https://github.com/Hugo0Vaz/docsdog-php) |
