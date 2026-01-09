use crate::environment::Environment;
use crate::errors::ShellError;
use crate::script::Script;
use crate::shell::Shell;
use bstr::ByteSlice;
use std::process::Output;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct ShellExecutor {
    #[builder(default = Shell::current())]
    shell: Shell,
    #[builder(default, setter(into))]
    environment: Environment,
}

impl ShellExecutor {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn with_environment(mut self, environment: Environment) -> Self {
        self.environment = environment;
        self
    }

    pub async fn execute(&self, script: &str) -> Result<Output, ShellError> {
        let script = Script::new(script);
        self.execute_script(&script).await
    }

    pub async fn execute_script(&self, script: &Script) -> Result<Output, ShellError> {
        let mut command = Command::new(self.shell.binary());

        command
            .args(self.shell.args())
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        for (key, value) in self.environment.variables() {
            command.env(
                key.to_os_str()
                    .map_err(|e| ShellError::ExecutionFailed(format!("Invalid env key: {}", e)))?,
                value.to_os_str().map_err(|e| {
                    ShellError::ExecutionFailed(format!("Invalid env value: {}", e))
                })?,
            );
        }

        let mut child = command.spawn()?;

        let mut stdin = child.stdin.take().ok_or_else(|| {
            ShellError::ExecutionFailed("Failed to open stdin for shell process".to_string())
        })?;

        stdin.write_all(script.content()).await?;
        drop(stdin);

        let output = child.wait_with_output().await?;
        Ok(output)
    }
}

impl Default for ShellExecutor {
    fn default() -> Self {
        Self::new()
    }
}
