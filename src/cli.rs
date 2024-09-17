use crate::bindings::exports;
use crate::bindings::wasi::cli::terminal_input::TerminalInput;
use crate::bindings::wasi::cli::terminal_output::TerminalOutput;
use crate::bindings::wasi::cli::{
    environment, exit, stderr, stdin, stdout, terminal_stderr, terminal_stdin, terminal_stdout,
};

impl exports::wasi::cli::environment::Guest for () {
    fn get_environment() -> Vec<(String, String)> {
        environment::get_environment()
    }

    fn get_arguments() -> Vec<String> {
        environment::get_arguments()
    }

    fn initial_cwd() -> Option<String> {
        environment::initial_cwd()
    }
}

impl exports::wasi::cli::stdin::Guest for () {
    fn get_stdin() -> exports::wasi::io::streams::InputStream {
        stdin::get_stdin().into()
    }
}

impl exports::wasi::cli::stdout::Guest for () {
    fn get_stdout() -> exports::wasi::io::streams::OutputStream {
        stdout::get_stdout().into()
    }
}

impl exports::wasi::cli::stderr::Guest for () {
    fn get_stderr() -> exports::wasi::io::streams::OutputStream {
        stderr::get_stderr().into()
    }
}

impl exports::wasi::cli::terminal_input::Guest for () {
    type TerminalInput = TerminalInput;
}

impl exports::wasi::cli::terminal_input::GuestTerminalInput for TerminalInput {}

impl exports::wasi::cli::terminal_output::Guest for () {
    type TerminalOutput = TerminalOutput;
}

impl exports::wasi::cli::terminal_output::GuestTerminalOutput for TerminalOutput {}

impl exports::wasi::cli::terminal_stdin::Guest for () {
    fn get_terminal_stdin() -> Option<exports::wasi::cli::terminal_input::TerminalInput> {
        terminal_stdin::get_terminal_stdin()
            .map(exports::wasi::cli::terminal_input::TerminalInput::new)
    }
}

impl exports::wasi::cli::terminal_stdout::Guest for () {
    fn get_terminal_stdout() -> Option<exports::wasi::cli::terminal_output::TerminalOutput> {
        terminal_stdout::get_terminal_stdout()
            .map(exports::wasi::cli::terminal_output::TerminalOutput::new)
    }
}

impl exports::wasi::cli::terminal_stderr::Guest for () {
    fn get_terminal_stderr() -> Option<exports::wasi::cli::terminal_output::TerminalOutput> {
        terminal_stderr::get_terminal_stderr()
            .map(exports::wasi::cli::terminal_output::TerminalOutput::new)
    }
}

impl exports::wasi::cli::exit::Guest for () {
    fn exit(status: Result<(), ()>) {
        exit::exit(status)
    }
}
