mod commands;
mod serve;

pub(crate) fn main() {
    let args = commands::cli().get_matches();

    serve::entrypoint(args)
}
