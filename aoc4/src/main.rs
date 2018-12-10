/// find sleeping guards
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    find_most_sleepy_guard(&input)
}

fn find_most_sleepy_guard(input: &str) -> Result<()> {
    let mut lines: Vec<&str> = input.lines().collect();
    lines.sort();

    // Build shifts
    let mut shifts = vec![];
    for line in &lines {
        if line.ends_with("begins shift") {
            let guard = get_id(line)?;
            shifts.push(Shift {
                guard,
                actions: vec![],
            })
        } else if line.ends_with("falls asleep") {
            let minute = get_minute(line)?;
            let last_shift = shifts.len() - 1;
            shifts[last_shift].actions.push(Action::Sleep(minute));
        } else if line.ends_with("wakes up") {
            let minute = get_minute(line)?;
            let last_shift = shifts.len() - 1;
            shifts[last_shift].actions.push(Action::Wake(minute));
        } else {
            panic!("Unexpected line: {}", line);
        }
    }
    let mut shifts_by_guard = HashMap::new();
    for shift in &shifts {
        shifts_by_guard
            .entry(shift.guard)
            .or_insert_with(Vec::new)
            .push(shift);
    }
    let mut sleeps = shifts_by_guard.iter();
    let (mut sleepiest, shift) = sleeps.next().ok_or("No guards?")?;
    let mut most_time_asleep: u32 = shift.iter().filter_map(|s| s.time_asleep().ok()).sum();
    for (guard, shift) in sleeps {
        let sleep = shift.iter().filter_map(|s| s.time_asleep().ok()).sum();
        if sleep > most_time_asleep {
            most_time_asleep = sleep;
            sleepiest = guard;
        }
    }

    // part 1
    println!("Sleepiest guard: {:?}", sleepiest);
    let (the_sleepiest, the_minute) =
        find_sleepiest_minute(shifts_by_guard.get(sleepiest).unwrap());
    println!(
        "Sleepiest minute asleep: {} how: {}",
        the_minute, the_sleepiest
    );
    println!("Multiplied: {}", the_minute * sleepiest.0 as u32);

    // part 2
    let mut shifts = shifts_by_guard.iter();
    let (mut max_guard, shift) = shifts.next().unwrap();
    let (mut max_sleepy, mut sleepiest_minute) = find_sleepiest_minute(shift);
    for (guard, shift) in shifts {
        let (this_sleepy, this_sleepiest_minute) = find_sleepiest_minute(shift);
        if this_sleepy > max_sleepy {
            max_sleepy = this_sleepy;
            sleepiest_minute = this_sleepiest_minute;
            max_guard = guard;
        }
    }
    println!(
        "Guard with most sleep on one minute: {:?}  the minute: {}  the total: {}",
        max_guard, sleepiest_minute, max_sleepy
    );
    println!("Guard x Minute: {}", max_guard.0 * sleepiest_minute as u64);
    Ok(())
}

/// Returns: (total sleep time, the minute that it happened at)
fn find_sleepiest_minute(shifts: &[&Shift]) -> (u32, u32) {
    let (mut the_sleepiest, mut the_minute) = (0, 0);
    let mut sleepiest_minutes = HashMap::new();
    for shift in shifts {
        for minute_asleep in shift.times_asleep() {
            let this = sleepiest_minutes.entry(minute_asleep).or_insert(0);
            *this += 1;
            if *this > the_sleepiest {
                the_sleepiest = *this;
                the_minute = minute_asleep;
            }
        }
    }
    (the_sleepiest, the_minute)
}

fn get_id(line: &str) -> Result<Id> {
    let part = line
        .chars()
        .skip_while(|c| *c != '#')
        .skip(1)
        .take_while(|c| c.is_digit(10))
        .collect::<String>();
    Ok(Id(part.parse()?))
}

fn get_minute(line: &str) -> Result<u32> {
    let part = line
        .chars()
        .skip_while(|c| *c != ':')
        .skip(1)
        .take(2)
        .collect::<String>();
    Ok(part.parse()?)
}

#[derive(Debug)]
struct Shift {
    guard: Id,
    actions: Vec<Action>,
}

impl Shift {
    fn time_asleep(&self) -> Result<u32> {
        use self::Action::*;
        let mut total_sleep = 0u32;
        let mut sleep_started = None;
        for action in &self.actions {
            match action {
                Sleep(time) => sleep_started = Some(time),
                Wake(time) => total_sleep += time - sleep_started.expect("woke before sleeping"),
            }
        }
        Ok(total_sleep)
    }

    /// All the minutes that a guard was asleep
    fn times_asleep(&self) -> Vec<u32> {
        use self::Action::*;
        let mut all_sleep_times = Vec::new();
        let mut sleep_start = None;
        for action in &self.actions {
            match action {
                Sleep(time) => sleep_start = Some(time),
                Wake(time) => all_sleep_times.extend(*sleep_start.expect("never slept")..*time),
            }
        }
        all_sleep_times
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Id(u64);

#[derive(Debug)]
enum Action {
    Sleep(u32),
    Wake(u32),
}
