use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectError{
	#[error("Error connecting to Wayland socket: {0:#?}")]
	ConnectError(wayrs_client::ConnectError),

	#[error("Error setting blocking roundtrip mode for connection: {0:#?}")]
	SetBlockingRoundtripErr(std::io::Error),
}

pub async fn connect() -> Result<wayrs_client::Connection<()>, ConnectError> {
	let mut conn = wayrs_client::
		Connection::connect().
		map_err(ConnectError::ConnectError)?;
	conn.blocking_roundtrip().map_err(ConnectError::SetBlockingRoundtripErr)?;
	Ok(conn)
}
