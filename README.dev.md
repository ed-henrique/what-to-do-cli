# Important Stuff for Devs <!-- omit in toc -->

## Symbol Meaning

| Symbol | Meaning       |
| :----: | :-----------: |
| ✅     | Done          |
| 🚧     | Doing         |
| ❌     | Won't Do      |
| ⚙️     | Under Testing |

## TODO

### High Priority

- use SQLite for local db

### Features

- support tags
- multiple profiles
- support Windows, Linux and MacOS

### Functions

#### Task Related

- 🚧 add new task
- remove task
- edit task
- show tasks
  
#### Profile Related

- add new profile
- remove profile
- edit profile
- show profiles
- select profile

### Documentation

- create CONTRIBUTING.md and CODE_OF_CONDUCT.md

## WON'T DO

## DONE

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
