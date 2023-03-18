
// This is all copiolt code and is hella buggy
pub fn parse_file(file: String, config: &mut Config) -> Status {

	for data_line in file.lines()
								.map(|l| l.trim())
								.filter(|line| !line.starts_with('#') && !line.is_empty()) {

		let mut split = data_line.splitn(2, '=');
		let key = split.next().unwrap().trim();
		let value = split.next().unwrap().trim();

		match key {
			"desktop" => {
				let mut split = value.splitn(2, ':');
				config.desktop.display = split.next().unwrap().trim().to_string();
				config.desktop.command = split.next().unwrap().trim().to_string();
			},
			"login" => config.login = value.to_string(),
			"password" => config.password = value.to_string(),
			_ => println!("Unknown key: {}", key),
		}
	}

	Status::Ok
}