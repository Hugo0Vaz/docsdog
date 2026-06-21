# Docs Dog

Docs Dog is a opinionated project documentation tool. It aims to provide an easy
way to link project documentation and implementation.

## QuickStart

1. Install docdog program.
2. Create a doc.
3. Track your Use Cases, Requirements implementation in code.
4. Generate the report.

Docs dog assumes the following:
- Requirements are elicited using *User Stories*.
- The user stories are translated to *Use Cases*.
- The language used is amongst the following (to be expanded...):
    - PHP
- Requirements are of type:
    - Functional: what the system shall do
    - Performance: how well the system must perform
    - Interface: how the system interacts with external systems
    - Operational: conditions during normal and degraded operation
    - Constraints: design limitations (regulatory, physical, technology choices)
    - Verification: how requirements will be confirmed as met
    - Quality: reliability, maintainability, security, and similar "-ilities"

## How to implement the documentation

For **SOFTWARE REQUIREMENTS (SRS)** template it's used an adapted version of the
great [MSRS](https://github.com/jam01/SRS-Template) that uses mardown to enable
the documentation of requirements in plain text files. However, it is no great
at tracking the because it's markdown format is too loose. DocDog's proposes a
similar template that uses YAML frontmatter in the same markdown files to manage
tracking documentation file to their perspective implementations.

For *Architechtural Decision Records (ADR's)* it's used another great template
the [MADRS](https://adr.github.io/madr/). Once again it suffers the same problem
of the previous template, and it' solved in the same exact way.

## The documentation is in place, now what?

Now you follow the guide for you language on how to tag, your implementations to
the documentation.

DocDog implements in each language construct the following tags:
- UseCase
- Requirement
- Decision
- Architecture
- ApiContract

For each tag is passed and ID that matches the ID of the documentation document.

### PHP

1. Install the library `composer require docdog/php`

2. Tag your implementations.

```php
use DocRef\Attributes\UseCase;
use DocRef\Attributes\Requirement;

#[UseCase("UC-001")]
#[Requirement("REQ-145")]
class CreateInvoice
{
    public function execute() {}
}
```

## I've added all the documentation and implementation tagging, I want my graph!

Now that all tagging and documentation is added there's a few options to check.

1. `docdog check`: runs all docdog's checks.
2. `docdog check uc`: checks what use cases are missing implementation or tests.
3. `docdog check rq`: checks what reqs are missing implementation or tests.
4. `docdog check tests`: check what tests are missing implementations.
5. `docdog dt`: creates a map of the dependendcy tree between usecases and reqs.
