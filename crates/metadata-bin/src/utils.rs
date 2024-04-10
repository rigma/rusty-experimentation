use clap::{
    builder::TypedValueParser,
    error::{ContextKind, ContextValue, ErrorKind},
    Arg, Command, Error,
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Clone, Debug, Default)]
pub(crate) struct IpAddrParser;

impl IpAddrParser {
    pub fn new() -> Self {
        Default::default()
    }
}

impl TypedValueParser for IpAddrParser {
    type Value = IpAddr;

    fn parse_ref(
        &self,
        cmd: &Command,
        arg: Option<&Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let raw = if let Some(raw) = value.to_str() {
            String::from(raw)
        } else {
            let mut err = Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
            if let Some(arg) = arg {
                err.insert(
                    ContextKind::InvalidArg,
                    ContextValue::String(arg.to_string()),
                );
            }

            err.insert(
                ContextKind::InvalidValue,
                ContextValue::String("must be a valid UTF-8 string".to_owned()),
            );
            return Err(err);
        };

        match (raw.parse::<Ipv4Addr>(), raw.parse::<Ipv6Addr>()) {
            (Ok(address), _) => Ok(IpAddr::V4(address)),
            (_, Ok(address)) => Ok(IpAddr::V6(address)),
            _ => {
                let mut err = Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
                if let Some(arg) = arg {
                    err.insert(
                        ContextKind::InvalidArg,
                        ContextValue::String(arg.to_string()),
                    );
                }

                err.insert(
                    ContextKind::InvalidValue,
                    ContextValue::String("must be a valid IPv4 or IPv6 address".to_owned()),
                );
                Err(err)
            }
        }
    }
}
