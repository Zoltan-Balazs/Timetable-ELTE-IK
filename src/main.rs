//sixtyfps::include_modules!();

use reqwest::{Client, Error, header::CONTENT_TYPE};

async fn get_course_info(semester: &str, type_of_request: &str, id: &str, limit: i16) -> Result<(), Error> /* Result<Request<String>, Error>  -> polars::frame::DataFrame */ {
    let url = "http://to.ttk.elte.hu/test.php";
    let client = Client::new();

    let body = match type_of_request {
        "Tárgynév" => format!("melyik={}&felev={}&limit={}&targynev={}", "nevalapjan", semester, limit, id),
        "Tárgykód" => format!("melyik={}&felev={}&limit={}&targykod={}", "kodalapjan", semester, limit, id),
        "Oktatónév" => format!("melyik={}&felev={}&limit={}&oktnev={}", "oktnevalapjan", semester, limit, id),
        "Oktatókód" => format!("melyik={}&felev={}&limit={}&oktnev={}", "oktneptunalapjan", semester, limit, id),
        _ => panic!("Request type incorrect!")
    };

    let res = client
        .post(url)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?;

    let text = res.text().await?;

    println!("{}", text);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_course_info("2021-2022-1", "Tárgynév", "WEBPROG", 1000).await?;
    Ok(())
}