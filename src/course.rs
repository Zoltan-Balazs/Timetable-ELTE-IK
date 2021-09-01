use csv::{Reader, WriterBuilder};
use reqwest::Client;
use std::path::Path;
use table_extract::Table;

pub async fn get_course_info(semester: &str, type_of_request: &str, id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://to.ttk.elte.hu/test.php";
    let limit: i16 = 1000;
    let client = Client::new();
    let body = match type_of_request {
        "Tárgynév" => format!("melyik={}&felev={}&limit={}&targynev={}", "nevalapjan", semester, limit, id),
        "Tárgykód" => format!("melyik={}&felev={}&limit={}&targykod={}", "kodalapjan", semester, limit, id),
        "Oktatónév" => format!("melyik={}&felev={}&limit={}&oktnev={}", "oktnevalapjan", semester, limit, id),
        "Oktatókód" => format!("melyik={}&felev={}&limit={}&oktnev={}", "oktneptunalapjan", semester, limit, id),
        _ => panic!("Request type {} incorrect!", type_of_request)
    };

    let response = client
        .post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?;

    let text = response.text().await?;

    match write_html_table_to_csv(&text, &id) {
        Ok(()) => Ok(()),
        _ => panic!("Error writing the data of {} file!", id)
    }
}

pub fn write_html_table_to_csv(response: &str, id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let html_table = Table::find_first(&response).unwrap();
    let filename = format!("{}-{}.csv", id, chrono::Local::now().format("%Y%m%d"));
    let mut csv_writer = WriterBuilder::new().from_path(filename)?;
    let mut data_vec: Vec<&String> = Vec::new();

    for row in &html_table {
        for cell in row {
            data_vec.push(&cell);
        }
        csv_writer.write_record(&data_vec)?;
        data_vec.clear();
    }

    csv_writer.flush()?;
    Ok(())
}

pub fn read_csv_to_text(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("{}-{}.csv", id, chrono::Local::now().format("%Y%m%d"));
    let mut csv_reader = Reader::from_path(filename)?;

    for result in csv_reader.records() {
        let record = result?;
        //println!("{:?}", record);
    }

    Ok(())
}

pub fn check_if_csv_exists(id: &str) -> bool {
    let filename = format!("{}-{}.csv", id, chrono::Local::now().format("%Y%m%d"));
    Path::new(&filename).exists()
}