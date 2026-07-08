use thiserror::Error;

#[derive(Debug,Error)]
pub enum CreateContextErr{
	#[error("Error connecting to Wayland socket: {0:#?}")]
	ConnectError(wayrs_client::ConnectError),

	#[error("Error setting blocking roundtrip mode for connection: {0:#?}")]
	SetBlockingRoundtripErr(std::io::Error),

	#[error("Error binding singleton: {0:#?}")]
	BindManagerErr(wayrs_client::global::BindError),

	#[error("Error converting sandbox info to CString: {0:#?}")]
	ConvertCStringErr(std::ffi::NulError),
}

pub async fn create_context(
	/*
		listen_fd must be ready to accept new connections
			when this request is sent by the client.
		In other words, the client must call bind(2) and listen(2) before sending the FD.
	*/
	listen_fd: std::os::fd::OwnedFd,
	/*
		close_fd is a FD that will signal hangup when the compositor should
			stop accepting new connections on listen_fd.
	*/
	close_fd: std::os::fd::OwnedFd,

	info: crate::info::SandboxInfo,
) -> Result<(), CreateContextErr> {
	let mut conn: wayrs_client::Connection<()>
		= wayrs_client::
		Connection::connect().
		map_err(CreateContextErr::ConnectError)?;

	//conn.blocking_roundtrip().map_err(CreateContextErr::SetBlockingRoundtripErr)?;

	let ctx_manager =
		conn.bind_singleton
		::<wayrs_protocols::security_context_v1::wp_security_context_manager_v1::WpSecurityContextManagerV1> (1).map_err(CreateContextErr::BindManagerErr)?;
	let ctx = ctx_manager.create_listener(&mut conn, listen_fd, close_fd);
	let sandbox_engine_id = std::ffi::CString::
		new(info.sandbox_engine).
		map_err(CreateContextErr::ConvertCStringErr)?;
	let app_id = std::ffi::CString::
		new(info.app_id).
		map_err(CreateContextErr::ConvertCStringErr)?;
	let instance_id = std::ffi::CString::
		new(info.instance_id).
		map_err(CreateContextErr::ConvertCStringErr)?;
	ctx.set_sandbox_engine(
		&mut conn,
		sandbox_engine_id,
	);
	ctx.set_app_id(
		&mut conn,
		app_id,
	);
	ctx.set_instance_id(
		&mut conn,
		instance_id,
	);
	ctx.commit(
		&mut conn,
	);
	Ok(())
}
