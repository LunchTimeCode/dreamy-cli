# Command-Line Help for `dy`

This document contains the help content for the `dy` command-line program.

**Command Overview:**

* [`dy`↴](#dy)
* [`dy markdown`↴](#dy-markdown)
* [`dy init_global`↴](#dy-init_global)
* [`dy gh_command`↴](#dy-gh_command)
* [`dy global_deps`↴](#dy-global_deps)
* [`dy global_deps server`↴](#dy-global_deps-server)

## `dy`

dreamy cli

**Usage:** `dy [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `markdown` — [STABLE] print markdown doc of qwit to std out
* `init_global` — [STABLE] creates an global example config
* `gh_command` — [STABLE] gh cli command to get all repos
* `global_deps` — [STABLE] get all deps of an github organisation

###### **Options:**

* `-r`, `--raw`

  Default value: `false`



## `dy markdown`

[STABLE] print markdown doc of qwit to std out

**Usage:** `dy markdown`



## `dy init_global`

[STABLE] creates an global example config

**Usage:** `dy init_global`



## `dy gh_command`

[STABLE] gh cli command to get all repos

**Usage:** `dy gh_command`



## `dy global_deps`

[STABLE] get all deps of an github organisation

**Usage:** `dy global_deps [OPTIONS] --token <TOKEN> [COMMAND]`

###### **Subcommands:**

* `server` — 

###### **Options:**

* `-t`, `--token <TOKEN>`
* `-o`, `--org <ORG>` — [STABLE] github organisation
* `-r`, `--repos_path <REPOS_PATH>` — [STABLE] path to a json file with all repositories to scrape [default: repos.json]
* `-a`, `--ashtml` — [PREVIEW] render as html

  Default value: `false`
* `-H`, `--html_type <HTML_TYPE>` — [PREVIEW] render licenses or dependencies [default: dependencies]

  Possible values: `licenses`, `dependencies`




## `dy global_deps server`

**Usage:** `dy global_deps server [OPTIONS]`

###### **Options:**

* `-p`, `--port <PORT>`

  Default value: `3000`
* `-s`, `--schedule <SCHEDULE>` — Poll github every, seconds: []s, minutes: []m, hours: []h, days: []D, weeks: []W, months: []M. Examples: "30s", "2W", "2h"

  Default value: `1m`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

