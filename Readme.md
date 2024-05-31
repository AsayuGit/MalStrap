# MalStrap : The malware analysis bootstraping tool.

MalStrap is a command line tool designed to facilitate malware analysis, by handling project management and producing sample summaries.

## Features :

- Quickly setup a malware analysis project and add samples
- Generate a summary of the current sample
- Integration with VirusTotal
    - Analysis results
    - Reputation
    - Related Domains and IPs
    - Dropped files

## Usage/Examples

#### Syntax
```bash
    mal_strap [command] [option]
```

#### Example
````bash
    EG: ./mal_strap sample -s Tux.png
````

#### Options
|  Option   | Description                                               |
| :-------- | :-------------------------------------------------------- |
| `init`    | Project init                                              |
| `project` | Project actions                                           |
| `sample`  | Sample actions                                            |
| `notes`   | Note generation actions                                   |
| `config`  | Configure workspace settings                              |
| `help`    | Print this message or the help of the given subcommand(s) |

## How to build
```bash
    cargo build --release
```
The compiled binary will be put at "target/release".