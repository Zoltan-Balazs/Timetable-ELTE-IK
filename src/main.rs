//sixtyfps::include_modules!();

mod course;
//mod calendar;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    course::get_course_info("2021-2022-1", "Tárgykód", "WEBPROGEG").await?;
    Ok(())
}