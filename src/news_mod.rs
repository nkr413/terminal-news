pub mod news_fn {
	#![allow(warnings)]

	// PACKAGES
	use tokio::io;
	use std::fs;
	use std::fs::File;
	use std::result::Result;
	use std::error::Error;
	use std::path::Path;

	use reqwest::{Client, header::USER_AGENT};
	use serde::{Deserialize, Serialize};
	use serde_json::Value;
	use colored::Colorize;
	use online::check;

	// MODULES
	pub use crate::set_key::api_fn;


	#[derive(Deserialize, Serialize, Debug)]
	struct Settings {
		ctg: String,
		lng: String,
		zen: String
	}


	fn create_folder() -> Result<(), Box<dyn Error>> {
		fs::create_dir_all("./settings-fld")?;
		Ok(())
	}

	fn get_settings() -> Settings {
		let fl_path = String::from("./settings-fld/settings.json");

		if Path::new(&fl_path).exists() == false {
			File::create(&fl_path).expect("Error encountered while creating file!");

			let obj = Settings { ctg: "all".to_string(), lng: "us".to_string(), zen: "off".to_string() };
			fs::write(&fl_path, serde_json::to_string_pretty(&obj).unwrap()).expect("Unable to write file");
		}

		let data = fs::read_to_string(&fl_path).expect("wrong 1");
		return serde_json::from_str(&data.to_string()).expect("wrong 2");
	}

	#[tokio::main]
	async fn do_request(url: &String) -> Result<(), Box<dyn Error>> {
		let stn: Settings = get_settings();
		let mut new_url = format!("");

		if stn.ctg == "all" {
			new_url = format!("https://newsapi.org/v2/top-headlines?country={}&apiKey={}", stn.lng, url);
		} else {
			new_url = format!("https://newsapi.org/v2/top-headlines?country={}&category={}&apiKey={}", stn.lng, stn.ctg, url);
		}

		if check(None).await.is_ok() == false {
			println!("\n-- Check your internet connection ! --\n");
			return Ok(());
		}

		let client = Client::new();
		let resp = client.get(&new_url)
			.header(USER_AGENT, "reqwest")
			.send()
			.await?
			.text()
			.await?;

		let mut file = File::create("./settings-fld/data.json").expect("Error encountered while creating file!");
		fs::write("./settings-fld/data.json", &resp).expect("Unable to write file");

		let v: Value = serde_json::from_str(&resp)?;
		let sub_value: Vec<Value> = serde_json::from_str(&v["articles"].to_string())?;

		let mut news_id: i32 = 1;

		for i in &sub_value {
			if &stn.zen == "on" {
				println!("\n{}: {}\n\n{}\n\n\n", news_id, i["title"], i["url"]);
			} else {
				println!("\n├ {}: {}\n│\n├ Description: {}\n├ Source: {}\n├ Url: {}\n└ Date: {}\n\n\n", news_id, i["title"].to_string().black().on_white(), i["description"], i["source"]["name"], format!("{}", i["url"]).bold(), i["publishedAt"]);
			}
			news_id += 1;
		}

		println!("\n--- Terminal News ---\nCategory: {}, Country: {}", stn.ctg, stn.lng);
		
		Ok(())
	}

	pub fn show_news() {
		create_folder();
		let fl_path = String::from("./settings-fld/api_key.txt");
		clearscreen::clear().unwrap();

		if Path::new(&fl_path).exists() == true {
			let data = fs::read_to_string(&fl_path).expect("Something went wrong reading the file");
			do_request(&data);
		} else {
			println!("You didn't insert API key. You need API key to read the news !");
			return api_fn::set_api_key();
		}
	}
}