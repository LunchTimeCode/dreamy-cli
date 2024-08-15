# Command-Line Help for `dy`

This document contains the help content for the `dy` command-line program.

**Command Overview:**

* [`dy`↴](#dy)
* [`dy markdown`↴](#dy-markdown)
* [`dy init`↴](#dy-init)
* [`dy init-global`↴](#dy-init-global)
* [`dy check`↴](#dy-check)
* [`dy deps`↴](#dy-deps)
* [`dy global-deps`↴](#dy-global-deps)

## `dy`

dreamy cli

**Usage:** `dy [COMMAND]`

###### **Subcommands:**

* `markdown` — [STABLE] print markdown doc of qwit to std out
* `init` — [STABLE] creates an example config
* `init-global` — [STABLE] creates an global example config
* `check` — [PREVIEW] checks licenses on github
* `deps` — [PREVIEW] get all deps of an repo
* `global-deps` — [PREVIEW] get all deps of an org



## `dy markdown`

[STABLE] print markdown doc of qwit to std out

**Usage:** `dy markdown`



## `dy init`

[STABLE] creates an example config

**Usage:** `dy init`



## `dy init-global`

[STABLE] creates an global example config

**Usage:** `dy init-global`



## `dy check`

[PREVIEW] checks licenses on github

**Usage:** `dy check [OPTIONS] --token <TOKEN>`

###### **Options:**

* `-t`, `--token <TOKEN>`
* `-o`, `--org <ORG>`
* `-r`, `--repo <REPO>`



## `dy deps`

[PREVIEW] get all deps of an repo

**Usage:** `dy deps [OPTIONS] --token <TOKEN>`

###### **Options:**

* `-t`, `--token <TOKEN>`
* `-o`, `--org <ORG>`
* `-r`, `--repo <REPO>`



## `dy global-deps`

[PREVIEW] get all deps of an org

**Usage:** `dy global-deps [OPTIONS] --token <TOKEN>`

###### **Options:**

* `-t`, `--token <TOKEN>`
* `-o`, `--org <ORG>`
* `-r`, `--repos-path <REPOS_PATH>`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

