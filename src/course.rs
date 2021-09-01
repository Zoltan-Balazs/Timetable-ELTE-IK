use reqwest::Client;
use table_extract::Table;
use csv::WriterBuilder;

pub async fn get_course_info(semester: &str, type_of_request: &str, id: &str) -> Result<(), reqwest::Error> {
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

    let res = client
        .post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?;

    let text = res.text().await?;

    match write_html_table_to_csv(&text, &id) {
        Ok(()) => Ok(()),
        _ => panic!("Error writing the data of {} file!", id)
    }
}

pub fn write_html_table_to_csv(response: &str, id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let html_table = Table::find_first(&response).unwrap();
    let filename = format!("{}-{}.csv", id, chrono::Local::now().date());
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