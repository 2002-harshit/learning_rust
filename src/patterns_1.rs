enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

impl TimeUnit {
    /// Return the plural noun for this time unit.
    fn plural(&self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }
}

fn rough_time_to_english(rt: &RoughTime) -> String {
    match rt {
        RoughTime::JustNow => format!("Just now"),
        RoughTime::InThePast(t, val) => format!("{} {} ago", val, t.plural()),
        RoughTime::InTheFuture(t, val) => format!("{} {} from now", val, t.plural()),
    }

    //* it is worthy noting that these patterns on the left hand side of the =>, cosumes the values,and whatever is present in the value is either copied or moved, this is important */
}

fn main() {
    let time1 = RoughTime::InTheFuture(TimeUnit::Days, 2);
    println!("{}", rough_time_to_english(&time1));
}
