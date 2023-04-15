use clap::{Command, Arg, ArgAction};

pub fn create_cli() -> Command {
    Command::new("binance")
        .author("AnXi0usCat <mikhail.fjodorov@gmail.com>")
        .version("1.0.0")
        .about("Interacts with Binance WebSocket APIExplains in brief what the program does")
        .arg(
            Arg::new("subscribe")
            .long("subscribe")
            .action(ArgAction::Set)
            .required(true)
        )
}
