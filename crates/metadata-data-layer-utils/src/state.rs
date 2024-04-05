use sqlx::{
    pool::Pool,
    postgres::{PgConnectOptions, Postgres},
};
use std::{borrow::Cow, sync::Arc};

/// A structure that is holding an atomic reference count to a SQLx
/// [Pool](sqlx::pool::Pool) for a [Postgres](sqlx::postgres::Postgres)
/// data backend.
///
/// You can either use as a state of your application or embed it into
/// another application state.
#[derive(Clone, Debug)]
pub struct PoolState {
    inner: Arc<Pool<Postgres>>,
}

impl PoolState {
    /// Instanciate a blank builder that can be used to parametrize a
    /// new instance of a [PoolState] struct.
    #[inline]
    pub fn builder<'a>() -> PoolStateBuilder<'a> {
        PoolStateBuilder::new()
    }

    /// Instanciate a builder filled with variables taken from the
    /// environment, if they are available.
    ///
    /// It will use the following variables:
    ///
    /// - `POSTGRES_APPNAME` will be used to identify the application
    /// in the context of the PostgreSQL connection.
    /// - `POSTGRES_HOST` will be used to get the PostgreSQL database
    /// hostname. The default value is `"localhost"`
    /// - `POSTGRES_PORT` will be used to get the network port to use
    /// to connect with the database. The default value is `5432`
    /// - `POSTGRES_USER` will be used as the username for the
    /// authentication with the PostgreSQL database. The default value
    /// is `postgres`.
    /// - `POSTGRES_PASSWORD` will be used as the password for the
    /// authentication process with the PostgreSQL database.
    /// - `POSTGRES_DATABASE` will be used as the database name
    /// to use once connected to PostgreSQL database.
    #[inline]
    pub fn from_env<'a>() -> PoolStateBuilder<'a> {
        Default::default()
    }

    /// Retrieve an immutable clone of the inner atomic reference
    /// count stored in the structure.
    #[inline]
    pub fn downcast_ref(&self) -> Arc<Pool<Postgres>> {
        Arc::clone(&self.inner)
    }
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
    #[doc(hidden)]
    #[inline]
    pub fn new() -> Self {
        Self {
            application_name: None,
            host: "localhost".into(),
            port: 5432,
            user: "postgres".into(),
            password: None,
            dbname: None,
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_env() -> Self {
        Self::default()
    }

    /// Define the application name to use to associate the connection
    /// with it.
    #[inline]
    pub fn application_name<S>(mut self, application_name: S) -> Self
    where
        S: AsRef<str> + Into<Cow<'a, str>>,
    {
        self.application_name = Some(application_name.into());
        self
    }

    /// Define the hostname of the PostgreSQL database to connect with.
    #[inline]
    pub fn host<S>(mut self, host: S) -> Self
    where
        S: AsRef<str> + Into<Cow<'a, str>>,
    {
        self.host = host.into();
        self
    }

    /// Define the network port to use to connect with the PostgreSQL
    /// database.
    #[inline]
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Define the username to use for the authentication with the
    /// PostgreSQL database.
    #[inline]
    pub fn user<S>(mut self, user: S) -> Self
    where
        S: AsRef<str> + Into<Cow<'a, str>>,
    {
        self.user = user.into();
        self
    }

    /// Define the password to use for the authentication with the
    /// PostgreSQL database.
    #[inline]
    pub fn password<S>(mut self, password: S) -> Self
    where
        S: AsRef<str> + Into<Cow<'a, str>>,
    {
        self.password = Some(password.into());
        self
    }

    /// Define the database name which will receive our SQL queries
    /// once we're authenticated.
    #[inline]
    pub fn dbname<S>(mut self, dbname: S) -> Self
    where
        S: AsRef<str> + Into<Cow<'a, str>>,
    {
        self.dbname = Some(dbname.into());
        self
    }

    /// Consume the current instance of the builder and use the values
    /// collected in it to instanciate a new [PoolState] instance that
    /// is holding connection pool to a PostgreSQL database.
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
            inner: Arc::new(Pool::connect_lazy_with(options)),
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
        let user = std::env::var("POSTGRES_USER")
            .unwrap_or("postgres".to_string())
            .into();
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
