use ics::{Event, ICalendar};
use ics::properties::{Description, DtEnd, DtStamp, DtStart, ProdID, RRule, Summary, TzID, UID};
use uuid::Uuid;

pub fn write_courses_to_ics(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("Órarend-{}.ics", chrono::Local::now().date());

    let uuid = Uuid::new_v4().to_simple();
    
    let now = chrono::Local::now().format("%Y%m%dT%H%M%S");
    let mut event = Event::new(uuid, now);

    let mut timetable = ICalendar::new("2.0", "ics-rs");

    timetable.save_file(filename)?;
    Ok(())
}