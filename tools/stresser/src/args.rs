use clap::{arg, command, Parser};

#[derive(Parser)]
#[command(author, version, about)]
pub(crate) struct StresserArgs {
    /// IPv4/IPv6/DNS address of a server.
    #[arg(short = 't', long = "target")]
    pub(crate) target_host: String,

    /// Number of sessions.
    #[arg(short = 'c', long = "count")]
    pub(crate) sessions_count: usize,

    /// Name prefix of sessions.
    #[arg(default_value = "Stresser")]
    #[arg(short = 'n', long = "name")]
    pub(crate) name_prefix: String,

    /// Spawn cooldown of sessions in milliseconds.
    /// The lower the value, the more frequently sessions are spawned.
    #[arg(default_value = "10")]
    #[arg(long = "cooldown")]
    pub(crate) spawn_cooldown: u64,

    /// Read buffer size in bytes.
    #[arg(default_value = "4096")]
    #[arg(long = "read-buffer")]
    pub(crate) read_buffer_size: usize,
}
