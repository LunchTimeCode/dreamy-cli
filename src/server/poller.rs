use anyhow::anyhow;
use schedule::{Agenda, Job, Schedule};
use chrono::Utc;




fn create_agenda(cron: &str)-> anyhow::Result<Agenda>{
let s: Box<dyn Schedule> = cron.parse().map_err(|_| anyhow!("could not parse cron expression"))?;
    
   let mut a = Agenda::new();
   
   a.add(Job::new(|| {
        println!("at second     :: {}", Utc::now());
    }, s ));
   
   Ok(a)
}

