pub mod register;

pub use register::Register;

pub use route::*;

// 注册路由
pub mod route {
  pub const ACCOUNT_REGISTER: &str = "/account/register";
}
