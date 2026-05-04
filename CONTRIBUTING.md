# Contribution Guidelines

## Commit Messages

These rules are adopted from
the [AngularJS commit message convention](https://docs.google.com/document/d/1QrDFcIiPjSLDn3EL15IJygNPiHORgU1_OOAqWjiDU5Y/edit#heading=h.uyo6cb12dt6w).

### Commit Message Format

Each commit message starts with a **type**, a **scope**, and a **subject**.

Below that, the commit message may have a **body**.
After another blank line, the commit message may have an **issue**.

- **type**: what type of change this commit contains.
- **scope**: what item of code this commit is changing.
- **subject**: a short description of the changes.
- **body** (optional): a more in-depth description of the changes.
- **issue** (optional): the JIRA issue number if commit is _directly_ related to a JIRA issue.

```
<type>(<scope>): <subject>
<BLANK LINE>
<body>
<BLANK LINE>
<issue>
```

Examples:

```
feat(*): apply new code style globally

#1337
```

```
chore(build): add incremental build support

The build system now supports incremental builds by using inputs and outputs.
```

```
fix(my-super-lambda): fix slow startup time

The lambda function now starts up in less than 1 second.

#42
```

Any line of the commit message should not be longer 100 characters. This allows the message to be easier
to read on github as well as in various git tools.

### Type

Is recommended to be one of the below items.

* **feat**: A new feature
* **fix**: A bug fix
* **docs**: Documentation only changes
* **style**: Changes that do not affect the meaning of the code (white-space, formatting, missing
  semicolons, etc.)
* **refactor**: A code change that neither fixes a bug or adds a feature
* **test**: Adding missing tests
* **chore**: Changes to the build process or auxiliary tools and libraries such as documentation
  generation

### Scope

The scope could be anything specifying place of the commit change and must match the following regex: `[a-zA-Z0-9._*-]+`

For example `(affectedModule)`, `(*)`, `(class)`, `(build)`, etc...

### Subject

The subject contains succinct description of the change:

* use the imperative, present tense: "change" not "changed" nor "changes"
* don't capitalize first letter
* no dot (.) at the end
