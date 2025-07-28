#[derive(clap::Parser, Debug)]
pub struct ProgramArgs {
    #[clap(long, env, default_value="0.0.0.0")]
    pub http_host: String,
    #[clap(long, env, default_value="8080")]
    pub http_port: u16,
}
