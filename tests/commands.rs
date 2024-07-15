#[cfg(test)]
mod clap_commands_tests {
    use clap::Parser;

    use todo::cli::{Cli, Commands};

    #[test]
    fn test_list_command() {
        let args = vec!["todo", "list"];
        let cli = Cli::parse_from(args);
        assert!(matches!(cli.command, Commands::List));
    }

    #[test]
    fn test_add_command() {
        let args = vec!["todo", "add", "New Task", "Description of the new task"];
        let cli = Cli::parse_from(args);
        if let Commands::Add { title, description } = cli.command {
            assert_eq!(title, "New Task");
            assert_eq!(description, "Description of the new task");
        } else {
            panic!("Expected Commands::Add variant");
        }
    }

    #[test]
    fn test_remove_command() {
        let args = vec!["todo", "remove", "1"];
        let cli = Cli::parse_from(args);
        if let Commands::Remove { id } = cli.command {
            assert_eq!(id, "1");
        } else {
            panic!("Expected Commands::Remove variant");
        }
    }

    #[test]
    fn test_done_command() {
        let args = vec!["todo", "done", "1"];
        let cli = Cli::parse_from(args);
        if let Commands::Done { id } = cli.command {
            assert_eq!(id, "1");
        } else {
            panic!("Expected Commands::Done variant");
        }
    }

    #[test]
    fn test_invalid_command() {
        let args = vec!["todo", "invalid"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err(), "Expected an error for invalid command");
    }

    #[test]
    fn test_help_command() {
        let args = vec!["todo", "--help"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err(), "Expected an error for help command as it shows help message");
    }
}
