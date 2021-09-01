//sixtyfps::include_modules!();

mod course;
mod calendar;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if course::check_if_csv_exists("WEBPROGEG") {
        course::read_csv_to_text("WEBPROGEG");
    } else {
        course::get_course_info("2021-2022-1", "Tárgykód", "WEBPROGEG").await?;
    }
    //calendar::write_courses_to_ics("");
    Ok(())
}