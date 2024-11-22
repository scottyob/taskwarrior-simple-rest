use serde::{Serialize, Deserialize};

use tokio::process::Command;
use std::str;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub uuid: Option<String>,
    pub description: String,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

impl Tasks {
    pub async fn get() -> Result<Tasks, Box<dyn Error>> {
        // Run the shell command async
        let output = Command::new("task")
            .arg("export")
            .arg("list")
            .output()
            .await?;

        if !output.status.success() {
            eprint!("Error:  Command execution failed with status {:?}", output.status);
            return Err("Command failed to execute".into())
        }

        let tasks: Tasks = serde_json::from_str(str::from_utf8(&output.stdout)?)?;
        return Ok(tasks);
    }

    pub async fn add(task: Task) -> Result<String, Box<dyn Error>> {
        // Check the uuid is empty
        assert!(matches!(task.uuid, None), "Task expected not to have existing UUID");

        // Run the shell command
        let mut cmd = Command::new("task");
        cmd.arg("add");
        cmd.arg(task.description);

        let notes = task.notes;
        if let Some(ref notes) = notes {
            cmd.arg(format!("notes:\"{}\"", notes));
        }

        let output = cmd.output().await?;
        if !output.status.success() {
            eprint!("Error:  Command execution failed with status {:?}", output.status);
            return Err("Command failed to execute".into())
        }

        let ret = str::from_utf8(&output.stdout)?; 
        return Ok(ret.to_string());
    }    
}
