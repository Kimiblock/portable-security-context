use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectError{
	#[error("Error connecting to Wayland socket: {0:#?}")]
	ConnectError(wayrs_client::ConnectError),
}

pub async fn connect() -> Result<wayrs_client::Connection<()>, ConnectError> {
	wayrs_client::
		Connection::connect().
		map_err(ConnectError::ConnectError)
}
