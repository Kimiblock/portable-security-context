pub struct SandboxInfo {
	pub sandbox_engine:	String,
	pub app_id:		String,
	pub instance_id:	String,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfoError {
	#[error("Could not decode configuration from environment variables: {0:#?}")]
	InvalidEnv(std::env::VarError),
}

impl SandboxInfo {
	pub fn get() -> Result<Self, InfoError> {
		let app_id = std::env::var("appID")
			.map_err(InfoError::InvalidEnv)?;
		let sandbox_engine = std::env::var("sandboxEngine")
			.map_err(InfoError::InvalidEnv)?;
		let instance_id = std::env::var("instanceId")
			.map_err(InfoError::InvalidEnv)?;
		Ok(
			Self {
				app_id:		app_id,
				sandbox_engine:	sandbox_engine,
				instance_id:	instance_id,
			}
		)
	}
}
