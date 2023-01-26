# Important Stuff for Devs <!-- omit in toc -->

## Table of Contents <!-- omit in toc -->

- [Symbol Meaning](#symbol-meaning)
- [ToDos](#todos)
  - [High Priority](#high-priority)
  - [Features](#features)
  - [Functions](#functions)
    - [Task Related](#task-related)
    - [Profile Related](#profile-related)
  - [Documentation](#documentation)
- [Database Structure](#database-structure)
  - [Task Structure](#task-structure)
  - [Profile Structure](#profile-structure)
- [Useful Resources](#useful-resources)
  - [Project Related](#project-related)
  - [Code Related](#code-related)

## Symbol Meaning

| Symbol | Meaning       |
| :----: | :-----------: |
| ✅     | Done          |
| 🚧     | Doing         |
| ❌     | Won't Do      |
| ⚙️     | Under Testing |

## ToDos

### High Priority

- ✅ use SQLite for local db

### Features

- support tags
- multiple profiles
- support
  - Windows
  - 🚧 Linux
  - MacOS
- use wtd as an alias to what-to-do-cli

### Functions

#### Task Related

- ✅ add new task
- ✅ remove task
- ✅ edit task
- ✅ show tasks
  
#### Profile Related

- add new profile
- remove profile
- edit profile
- show profiles
- select profile

### Documentation

- 🚧 create CONTRIBUTING.md and CODE_OF_CONDUCT.md

## Database Structure

### Task Structure

```json
"task": {
    "name": "TASK_NAME",
    "tags": ["TAG_LIST"],
    "status": "TASK_STATUS"
}
```

### Profile Structure

```json
"profile": {
    "name": "PROFILE_NAME"
}
```

## Useful Resources

### Project Related

- [Branching Strategy (trunk-based development)](https://www.atlassian.com/continuous-delivery/continuous-integration/trunk-based-development)
- 
### Code Related

- [Database (sqlite)](https://www.sqlite.org/index.html)
- [Programming Language (rust)](https://www.rust-lang.org/)
- [ORM and Query Builder (diesel)](https://diesel.rs/)
- [Command Line Argument Parser (clap)](https://docs.rs/clap/latest/clap/)
