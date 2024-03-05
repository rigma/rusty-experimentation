use sqlx::{
    self,
    pool::{Pool, PoolConnection},
    postgres::{PgConnectOptions, Postgres},
};
use std::{borrow::Cow, process::Command};

#[derive(Clone, Debug)]
pub struct PoolState {
    inner: Pool<Postgres>,
}

impl PoolState {
    #[inline]
    pub fn new<'a>() -> PoolStateBuilder<'a> {
        PoolStateBuilder::new()
    }

    #[inline]
    pub fn from_env<'a>() -> PoolStateBuilder<'a> {
        Default::default()
    }

    #[inline]
    pub async fn begin_connection(&self) -> Result<PoolConnection<Postgres>, sqlx::Error> {
        self.inner.acquire().await
    }
}

#[inline]
fn whoami<'a>() -> Cow<'a, str> {
    let whoami = Command::new("whoami")
        .output()
        .expect("Unable to invoke whoami command");

    String::from_utf8(whoami.stdout)
        .expect("Unable to parse whoami output")
        .into()
}

#[derive(Debug)]
pub struct PoolStateBuilder<'a> {
    application_name: Option<Cow<'a, str>>,
    host: Cow<'a, str>,
    port: u16,
    user: Cow<'a, str>,
    password: Option<Cow<'a, str>>,
    dbname: Option<Cow<'a, str>>,
}

impl<'a> PoolStateBuilder<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            application_name: None,
            host: "localhost".into(),
            port: 5432,
            user: whoami(),
            password: None,
            dbname: None,
        }
    }

    #[inline]
    pub fn from_env() -> Self {
        Self::default()
    }

    #[inline]
    pub fn application_name<S>(mut self, application_name: S) -> Self
    where
        S: AsRef<str> + Into<Cow<'a, str>>,
    {
        self.application_name = Some(application_name.into());
        self
    }

    #[inline]
    pub fn host<S>(mut self, host: S) -> Self
    where
        S: AsRef<str> + Into<Cow<'a, str>>,
    {
        self.host = host.into();
        self
    }

    #[inline]
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    #[inline]
    pub fn user<S>(mut self, user: S) -> Self
    where
        S: AsRef<str> + Into<Cow<'a, str>>,
    {
        self.user = user.into();
        self
    }

    #[inline]
    pub fn password<S>(mut self, password: S) -> Self
    where
        S: AsRef<str> + Into<Cow<'a, str>>,
    {
        self.password = Some(password.into());
        self
    }

    #[inline]
    pub fn dbname<S>(mut self, dbname: S) -> Self
    where
        S: AsRef<str> + Into<Cow<'a, str>>,
    {
        self.dbname = Some(dbname.into());
        self
    }

    pub fn finalize(self) -> PoolState {
        let mut options = PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.user);

        if let Some(application_name) = self.application_name {
            options = options.application_name(&application_name);
        }
        if let Some(dbname) = self.dbname {
            options = options.database(&dbname);
        }
        if let Some(password) = self.password {
            options = options.password(&password);
        }

        PoolState {
            inner: Pool::connect_lazy_with(options),
        }
    }
}

impl<'a> Default for PoolStateBuilder<'a> {
    fn default() -> Self {
        let application_name = std::env::var("POSTGRES_APPNAME")
            .ok()
            .map(|app_name| app_name.into());
        let host = std::env::var("POSTGRES_HOST")
            .unwrap_or("localhost".to_string())
            .into();
        let port = {
            if let Ok(port) = std::env::var("POSTGRES_PORT") {
                port.parse().unwrap_or(5432)
            } else {
                5432
            }
        };
        let user = {
            if let Ok(user) = std::env::var("POSTGRES_USER") {
                user.into()
            } else {
                whoami()
            }
        };
        let password = std::env::var("POSTGRES_PASSWORD")
            .ok()
            .map(|password| password.into());
        let dbname = std::env::var("POSTGRES_DATABASE")
            .ok()
            .map(|dbname| dbname.into());

        Self {
            application_name,
            host,
            port,
            user,
            password,
            dbname,
        }
    }
}
