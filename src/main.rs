//Entry point
mod cli;
mod commands;
mod replstate;
mod vfs;

fn main() {
    cli::run();
}
