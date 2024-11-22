A (very) simple HTTP API for TaskWarrior.

This doesn't even do authentication.

## API

### `/` endpoint
#### GET 
Gets a list of all active tasks
#### RESPONSE
```
{
    "tasks": [
        {
            "uuid": "uuid-of-task",
            "description": "title of task",
            "notes": "Detailed note of task"
        }
    ]
}
```

### `/new` endpoint
#### POST
Creates a new Taskwarrior task

#### REQUEST
{
    "description": "title",
    "notes": "Task notes",
}

#### RESPONSE
OK!