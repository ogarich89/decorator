use decorator::static_decorator::{Days, HoursDecorator, MinutesDecorator, SecondsDecorator, Time};

fn main() {
    let with_hours = HoursDecorator::new(Days);
    let with_minutes = MinutesDecorator::new(with_hours);
    let with_seconds = SecondsDecorator::new(with_minutes);

    let days = 2;
    let seconds = with_seconds.convert(days);
    println!("Days: {}, Seconds: {}", days, seconds)
}
