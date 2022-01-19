//! Impement the WASI cache interface using a Redis instance.
//! This is using a Wasmtime host implementation.

use cache::*;
use redis::{Client, Commands, Connection};

pub use cache::add_to_linker;

wit_bindgen_wasmtime::export!("../cache.wit");

/// Redis implementation for the WASI cache interface.
pub struct RedisCache {
    /// The address of the Redis instance.
    pub address: String,

    /// The Redis connection.
    connection: Connection,
}

impl cache::Cache for RedisCache {
    /// Set the payload for the given key.
    /// If provided, the time-to-live argument (in seconds) will be used to set the expiration time.
    fn set(&mut self, key: &str, value: PayloadParam<'_>, ttl: Option<u32>) -> Result<(), Error> {
        self.set(key, value, ttl)?;
        Ok(())
    }

    /// Get the payload for the given key.
    fn get(&mut self, key: &str) -> Result<PayloadResult, Error> {
        Ok(self.get(key)?)
    }

    /// Delete the entry for the given key.
    fn delete(&mut self, key: &str) -> Result<(), Error> {
        Ok(self.delete(key)?)
    }
}

impl RedisCache {
    /// Create a new instance for the cache.
    pub fn new(addr: &str) -> anyhow::Result<Self> {
        let client = Client::open(addr)?;
        let connection = client.get_connection()?;
        Ok(RedisCache {
            address: addr.to_string(),
            connection,
        })
    }

    /// Set the payload in Redis using the given key and optional time-to-live (in seconds).
    fn set(&mut self, key: &str, value: &[u8], ttl: Option<u32>) -> anyhow::Result<()> {
        self.connection.set(key, value)?;
        match ttl {
            Some(s) => self.connection.expire(key, s as usize)?,
            None => {}
        };

        Ok(())
    }

    /// Get the payload stored in Redis using the given key.
    fn get(&mut self, key: &str) -> anyhow::Result<Vec<u8>> {
        let res: Vec<u8> = self.connection.get(key)?;

        Ok(res)
    }

    /// Delete the entry for the given key stored in Redis.
    fn delete(&mut self, key: &str) -> anyhow::Result<()> {
        self.connection.del(key)?;

        Ok(())
    }
}

impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Self::RuntimeError
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use wasi_cap_std_sync::{ambient_authority, Dir, WasiCtxBuilder};
    use wasi_common::WasiCtx;
    use wasmtime::{Config, Engine, Linker, Module, Store};

    use crate::RedisCache;

    const RUST_MODULE: &str = "../rust-consumer/target/wasm32-wasi/release/rust-consumer.wasm";

    #[test]
    fn test_redis() -> Result<()> {
        let mut config = Config::new();
        config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        config.wasm_multi_memory(true);
        config.wasm_module_linking(true);

        let engine = Engine::new(&config)?;
        let module = Module::from_file(&engine, RUST_MODULE)?;
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut Context| &mut cx.wasi)?;

        let rc = RedisCache::new("redis://localhost:6379")?;
        cache::add_to_linker(&mut linker, |ctx| -> &mut RedisCache {
            ctx.runtime_data.as_mut().unwrap()
        })?;

        let ctx = Context {
            runtime_data: Some(rc),
            wasi: default_wasi(),
        };

        let mut store = Store::new(&engine, ctx);
        let instance = linker.instantiate(&mut store, &module)?;

        let start = instance.get_func(&mut store, "_start").unwrap();
        start.call(&mut store, &[], &mut [])?;

        Ok(())
    }

    struct Context {
        pub wasi: WasiCtx,
        pub runtime_data: Option<RedisCache>,
    }

    fn default_wasi() -> WasiCtx {
        let mut ctx = WasiCtxBuilder::new().inherit_stdio();
        ctx = ctx
            .preopened_dir(
                Dir::open_ambient_dir("./target", ambient_authority()).unwrap(),
                "cache",
            )
            .unwrap();

        ctx.build()
    }
}
