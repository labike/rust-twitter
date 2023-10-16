use axum::extract::FromRef;
use uchat_query::{AsyncConnectionPool, AsyncConnection, QueryError};

pub mod logging;
pub mod router;
pub mod error;
pub mod extractor;

#[derive(FromRef, Clone)]
pub struct AppState {
  pub db_pool: AsyncConnectionPool, // 数据库连接池
  pub signing_keys: uchat_crypto::sign::Keys, // 签名密钥
  pub rng: rand::rngs::StdRng, // 随机数
}

impl AppState {
  // 从数据库连接池里获取一个连接,等待其返回
  pub async fn connect(&self) -> Result<AsyncConnection, QueryError> {
    self.db_pool.get().await
  }
}

// cli包含生成密钥的功能
pub mod cli {
  use color_eyre::{eyre::Context, Section};
  use rand_core::{CryptoRng, RngCore};
  use uchat_crypto::sign::{EncodedPrivateKey, encode_private_key, Keys};

  // gen_keys将为cookies生成签名密钥, 它是<R>类型的通用函数, R类型必须实现CryptoRng和RngCore
  // 这里使用color_eyre而非标准库中的方法主要是因为color_eyre可以接收任意类型的错误，我们只需要用?来处理而不用指定遇到的是具体的
  // 哪种错误
  pub fn gen_keys<R>(rng: &mut R) -> color_eyre::Result<(EncodedPrivateKey, Keys)>
  where
    R: CryptoRng + RngCore,
  {
    let (private_key, keys) = Keys::generate(rng)?;
    let private_key = encode_private_key(private_key)?;
    Ok((private_key, keys))
  }

  pub fn load_keys() -> color_eyre::Result<Keys> {
    let private_key = std::env::var("API_PRIVATE_KEY")
      .wrap_err("failed to locate private Api key")
      .suggestion("set API_PRIVATE_KEY environment variable")?;
    Ok(Keys::from_encoded(private_key)?)
  }
}
