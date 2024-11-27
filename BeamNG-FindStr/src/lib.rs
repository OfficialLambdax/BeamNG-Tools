use colored::Colorize;


pub struct FindStr {
	find: String,
	find_in: String,
	from: String,
	found_list: Vec<(i32, String)>
}

impl FindStr {
	pub fn new(find: &str, find_in: &str, from: &str) -> FindStr {
		let mut new = FindStr {
			find: find.to_string().to_lowercase(),
			find_in: find_in.to_string(),
			from: from.to_string(),
			found_list: Vec::new(),
		};
		new.process();
		new
	}
	fn process(&mut self) {
		let line_vec: Vec<&str> = self.find_in.lines().collect();
		let mut line_number = 1;
		for line in line_vec {
			if line.to_lowercase().contains(&self.find) {
				let formatted_line = line.to_string().replace("\t", "").replace("  ", "");
				self.found_list.push((line_number, formatted_line));
			}
			line_number += 1;
		}
	}
	pub fn display(&self, max: i32) {
		if self.found_list.len() == 0 {return}
		println!("Found string in file - {}", &self.from);
		let mut displayed = 0;
		for (line_number, line) in &self.found_list {
			if line.len() > 200 {
				println!("  @ {}", line_number);
				println!("  : ## To long ##");
			} else {
				println!("  @ {}", line_number.to_string());
				println!("  : {}", line);
				displayed += 1;
				if displayed == max {
					println!("  .. {} more results in this file", self.found_list.len() - 10);
					break;
				}
			}
		}
		println!("");
	}
	pub fn display_colored(&self, max: i32) {
		if self.found_list.len() == 0 {return}
		println!("{} - {}", "Found string in file".green(), &self.from);
		let mut displayed = 0;
		for (line_number, line) in &self.found_list {
			if line.len() > 200 {
				println!("  @ {}", line_number);
				println!("  : ## To long ##");
			} else {
				let pos = line.to_lowercase().find(&self.find).unwrap();
				let string_before = &line[..pos];
				let string_after = &line[pos + self.find.len()..];
				println!("  @ {}", line_number.to_string().yellow());
				println!("  : {}{}{}", string_before, self.find.red(), string_after);
				displayed += 1;
				if displayed == max {
					println!("");
					println!("  .. {} more results in this file", self.found_list.len() - (max as usize));
					break;
				}
			}
		}
		println!("");
	}
}
