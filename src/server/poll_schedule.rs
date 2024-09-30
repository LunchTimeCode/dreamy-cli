use std::num::ParseIntError;

pub enum PollSchedule {
    Seconds(u32),
    Minutes(u32),
    Hours(u32),
    Days(u32),
    Weeks(u32),
    Months(u32),
}

impl TryFrom<String> for PollSchedule {
    type Error = String;
    fn try_from(value: String) -> Result<Self, String> {
        let default = Ok(PollSchedule::Minutes(1)); // Default value if no valid format is found

        // Check if the input ends with 's' (seconds)
        if let Some(stripped) = value.strip_suffix('s') {
            let seconds = stripped
                .parse::<u32>()
                .map_err(|e: ParseIntError| e.to_string())?;
            return Ok(PollSchedule::Seconds(seconds));
        }

        // Check if the input ends with 'm' (minutes)
        if let Some(stripped) = value.strip_suffix('m') {
            let minutes = stripped
                .parse::<u32>()
                .map_err(|e: ParseIntError| e.to_string())?;
            return Ok(PollSchedule::Minutes(minutes));
        }

        // Check if the input ends with 'h' (hours)
        if let Some(stripped) = value.strip_suffix('h') {
            let hours = stripped
                .parse::<u32>()
                .map_err(|e: ParseIntError| e.to_string())?;
            return Ok(PollSchedule::Hours(hours));
        }

        // Check if the input ends with 'D' (days)
        if let Some(stripped) = value.strip_suffix('D') {
            let days = stripped
                .parse::<u32>()
                .map_err(|e: ParseIntError| e.to_string())?;
            return Ok(PollSchedule::Days(days));
        }

        // Check if the input ends with 'W' (weeks)
        if let Some(stripped) = value.strip_suffix('W') {
            let weeks = stripped
                .parse::<u32>()
                .map_err(|e: ParseIntError| e.to_string())?;
            return Ok(PollSchedule::Weeks(weeks));
        }

        // Check if the input ends with 'M' (months)
        if let Some(stripped) = value.strip_suffix('M') {
            let months = stripped
                .parse::<u32>()
                .map_err(|e: ParseIntError| e.to_string())?;
            return Ok(PollSchedule::Months(months));
        }

        default
    }
}
