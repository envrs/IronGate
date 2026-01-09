pub mod argument;
pub mod environment;
pub mod errors;
pub mod execution;
pub mod script;
pub mod shell;

pub use argument::Argument;
pub use environment::Environment;
pub use errors::ShellError;
pub use execution::ShellExecutor;
pub use script::Script;
pub use shell::Shell;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_echo() {
        let executor = ShellExecutor::builder().build();
        let output = executor.execute("echo 'hello world'").await.unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.trim().contains("hello world"));
    }

    #[tokio::test]
    async fn test_multiline_script() {
        let executor = ShellExecutor::builder().build();
        let script = if cfg!(windows) {
            r#"
            $A="Hello"
            $B="World"
            echo "$A $B"
            "#
        } else {
            r#"
            A="Hello"
            B="World"
            echo "$A $B"
            "#
        };
        let output = executor.execute(script).await.unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.trim().contains("Hello World"));
    }

    #[tokio::test]
    async fn test_environment_variables() {
        let mut env = Environment::new();
        env.insert("IRONGATE_TEST", "verified");

        let executor = ShellExecutor::builder().environment(env).build();

        let script = if cfg!(windows) { "echo $env:IRONGATE_TEST" } else { "echo $IRONGATE_TEST" };

        let output = executor.execute(script).await.unwrap();
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.trim().contains("verified"));
    }
}
