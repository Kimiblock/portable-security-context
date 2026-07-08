mod create_context;
mod info;
mod connect;
mod check;

#[tokio::main]
async fn main() -> std::process::ExitCode {
	let conf_spawn = tokio::spawn(async {
		info::SandboxInfo::get()
	});

	std::process::ExitCode::SUCCESS
}


