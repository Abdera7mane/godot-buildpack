# Godot Buildpack

[Cloud Native Buildpack](https://buildpacks.io/) for [Godot Engine](https://godotengine.org/) projects.

# Usage

The buildpack will look for `project.godot` or `config.godot` in root directory
of the project. After a successful build, Godot headless and server binaries will
be installed (learn about the differences [here](https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_dedicated_servers.html#headless-versus-server-binaries)).

The following executables names are included in `PATH`:

| executable       | aliases            |
| ---------------- | ------------------ |
| `godot_headless` | `godot_h`          |
| `godot_server`   | `godot`, `godot_s` |

### config.godot

Contains information regarding godot version, the data is formatted in
[TOML](https://toml.io/en/).

|key      | type                 | default  | description                                                      |
| ------- | -------------------- | -------- | ---------------------------------------------------------------- |
| version | string  (*optional*) | `3.5`    | Godot version to download.                                       |
| tag     | string  (*optional*) | `stable` | Release type, can be `stable`, `alpha[n]`, `beta[n]` or `rc[n]`. | 
| mono    | boolean (*optional*) | `false`  | Whether to download the Mono version.                            |

example `config.godot`:

```toml
version = "3.5.1"
tag = "rc1"
mono = false
```
