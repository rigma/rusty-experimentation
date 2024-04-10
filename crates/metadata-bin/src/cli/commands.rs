use crate::utils::IpAddrParser;
use clap::{Arg, Command};

#[inline]
pub(super) fn cli() -> Command {
    Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::new("host")
            .short('H')
            .long("host")
            .value_parser(IpAddrParser::new())
            .help("An IP address mask to use for the port binding")
            .required(true)
        )
        .arg(
            Arg::new("port")
            .short('p')
            .long("port")
            .value_parser(clap::value_parser!(u16))
            .help("The port to use on the hosting machine to bind the socket to the process")
            .default_value("80")
        )
        .arg(
            Arg::new("postgres_host")
                .long("postgres-host")
                .env("POSTGRES_HOST")
                .help("The hostname of the PostgreSQL database to use")
                .default_value("localhost")
        )
        .arg(
            Arg::new("postgres_port")
                .long("postgres-port")
                .env("POSTGRES_PORT")
                .value_parser(clap::value_parser!(u16))
                .help("The port to use to connect with the PostgreSQL database")
                .default_value("5432")
        )
        .arg(
            Arg::new("postgres_user")
                .long("postgres-user")
                .env("POSTGRES_USER")
                .required(true)
                .help("The username to use for the authentication to the PostgreSQL database")
        )
        .arg(
            Arg::new("postgres_password")
                .long("postgres-password")
                .env("POSTGRES_PASSWORD")
                .required(true)
                .help("The password to use for the authentication to the PostgreSQL database")
        )
        .arg(
            Arg::new("postgres_database")
                .long("postgres-database")
                .env("POSTGRES_DATABASE")
                .required(true)
                .help("The database to use once the connection is established with the PostgreSQL database"),
        )
}
