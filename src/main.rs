mod create_context;
mod info;
mod connect;
mod check;
use thiserror::Error;

#[derive(Debug, Error)]
enum MainError {
	#[error("Could not spawn tokio thread: {0:#?}")]
	SpawnError(tokio::task::JoinError),

	#[error("Could not connect: {0:#?}")]
	ConnectError(connect::ConnectError),

	#[error("Compositor does not support security-context-v1")]
	SecurityContextNotSupported,
}

#[tokio::main]
async fn main() -> Result<(), MainError> {
	let conn_spawn = tokio::spawn(connect::connect());
	let conf_spawn = tokio::spawn(async {
		info::SandboxInfo::get()
	});

	let conn = conn_spawn
		.await
		.map_err(MainError::SpawnError)?;
	let conn = conn.map_err(MainError::ConnectError)?;


	let support_spawn = tokio::spawn(check::security_context_supported(conn));

	{
		let is_supported = support_spawn.await.map_err(MainError::SpawnError);
		match is_supported.unwrap() {
			true	=> {}
			false	=> {
				return Err(
					MainError::SecurityContextNotSupported
				)
			}
		};
	}

	Ok(())
}


