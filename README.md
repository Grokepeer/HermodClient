# Hermod In-Memory DB Client

## Hermod DB Client

Hermod Client interface. This client connects over TCP to the API of a Hermod Host to allow asynchronous operations by a user onto the host's database. All prompts by the user are forwarded, as is, to the host so for all documentation of commands take a look at the [Hermod](https://github.com/grokepeer/hermod/) GitHub page. This client interface was created mainly for debuggin purposes, not as a tool to the end-user. It's much suggested that a proper custom TCP interface is always implemented to communicate to the DB host directly.
