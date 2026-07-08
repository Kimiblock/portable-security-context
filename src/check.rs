pub async fn security_context_supported(
	conn:	wayrs_client::Connection<()>,
) -> bool {
	let protocol = "security-context-v1";
	let protocol = std::ffi::CString::
		new(protocol)
		.unwrap();
	let globals = conn.globals();
	for global in globals {
		if global.interface == protocol {
			return true
		};
	};
	false
}
