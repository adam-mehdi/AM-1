#[allow(non_snake_case)]
// Import necessary modules from the chrono library
use chrono::{DateTime, Duration, prelude::*};
use std::env;
use std::cmp::max;

fn main() {
  // Get the date and num_cards from the command line arguments
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
      panic!("Please provide a date and num_cards");
  }
  let date_string = &args[1];
  let datetime = DateTime::parse_from_rfc3339(date_string).expect("Invalid date format");
  let num_cards = args[2].parse::<i32>().expect("Invalid num_cards format");

  // Set the new day time and test day time
  let new_day_time = 2;  // 2am marks a new day
  let test_day_time = 14;  // 2pm marks the beginning of the test day

  // Calculate the days until the deadline
  let days_to_go = days_until_deadline(datetime, new_day_time, test_day_time) as i32;
  let num_boxes = get_num_boxes(days_to_go, 1, 0);

  let quotas = compute_quotas(num_cards, days_to_go, num_boxes);
  println!("days_to_go    new_assigned    review_assigned");
  for q in quotas {
    println!("{}             {}               {}", q.days_to_go, q.new_assigned, q.review_assigned);
  }
}


#[derive(Debug)]
pub struct QuotaRecord {
    pub days_to_go: i32,
    pub new_assigned: i32,
    pub review_assigned: i32,
    pub new_quota_initial: i32,
    pub review_quota_initial: i32,
    pub new_practiced: i32,
    pub review_practiced: i32,
}


// computes quotas for `num_cards` given `days_to_go` and `num_boxes`
pub fn compute_quotas(num_cards: i32, days_to_go: i32, num_boxes: i32)  -> Vec<QuotaRecord> {
    assert!(num_cards > 0);

    let n = num_cards;          // number of cards
    let t = days_to_go;         // days until deadline
    let b = num_boxes;          // number of boxes

    let mut sum: i32 = (0..t).sum();
    // avoid division by zero error for when t = 0, 1
    if sum == 0 {
      sum += 1;
    }

    // compute new card quota vector
    let mut nq: Vec<i32> = (0..t).rev().map(|x| x * n / sum).collect();
    let nq_sum = nq.iter().sum::<i32>();

    // enforce sum of NQ equals number of cards by adding remainder
    if let Some(first) = nq.get_mut(0) {
        *first += n - nq_sum;
        // no new cards on last day
        nq.push(0);
    }


    // compute review card quota vector
    let mut rq: Vec<i32> = (0..t).map(|x| x * n * (b - 2) / sum).collect();
    let rq_sum = rq.iter().sum::<i32>();

    // enforce sum of RQ equals number of cards times number of bins minus 2
    if let Some(last) = rq.last_mut() {
        *last += (n * (b - 2)) - rq_sum;
        // user reviews all cards the day of exam
        rq.push(n);
    }

    // review cards if days_to_go == 0
    if days_to_go == 0 {
      nq.push(n);
      rq.push(n * (b - 1));
    }

    let mut quotas = Vec::new();
    for i in 0..nq.len() {
        let dtg = nq.len() - 1 - i; // days to go
        quotas.push(
            QuotaRecord {
                days_to_go: dtg as i32,
                new_assigned: nq[i],
                review_assigned: rq[i],
                new_quota_initial: nq[i],
                review_quota_initial: rq[i],
                new_practiced: 0,
                review_practiced: 0
            }
        );
    }
    quotas
}

/**
 * Returns number of days until the given datetime, counting `new_day_time` as 
 * the time marking transition between days. Returns -1 if now is past the given 
 * datetime.
 * 
 * Args:
 *   datetime: date of the deadline in rfc3339 format
 *   new_day_time: hour (0-24) at which one day switches to the next
 *   test_day_time: hour `h` (0-24) such that if the test is after hour h 
 *                       then the test date is counted 
 */
pub fn days_until_deadline(
    datetime: DateTime<FixedOffset>,
    new_day_time: i64,
    test_day_time: i64
    ) -> i64 {
    // mark of new day: 2am
    // mark that deadline day is day 0: dl >= 2pm
    let mut day_bins: Vec<DateTime<FixedOffset>> = Vec::new();

    let mut inter = Local::now().with_timezone(&datetime.timezone());

    // get next time is at 2am
    inter = get_next_datetime_at_time(inter, new_day_time);
    
    // build up day_bins with datetimes at 2am on consecutive days
    while inter.timestamp() < datetime.timestamp() {
      day_bins.push(inter.clone());
      inter = get_next_datetime_at_time(inter, new_day_time);
    }


    if day_bins.len() == 0 {
      // on exam day
      if Local::now().timestamp() < datetime.timestamp() {
        return 0;
      }
      // past exam
      return -1;
    }

    // get deadline time in DateTime<Local>
    let dl_time = day_bins.last().unwrap().checked_add_signed(
      Duration::seconds((test_day_time - new_day_time) * 60 * 60)).unwrap();

    // time before test time does not count as a new day if it is before 2pm
    if datetime.hour() < test_day_time as u32 {
      day_bins.pop();
    }

    day_bins.push(dl_time);
    
    let now = Local::now().timestamp();

    for i in 0..day_bins.len() {
      if now < day_bins[i].timestamp() {
        let days = day_bins.len() - 1 - i;
        return days as i64;
      }
    }
    panic!("no day bins found")
}

fn get_next_datetime_at_time(dt: DateTime<FixedOffset>, time: i64) -> DateTime<FixedOffset> {
  // get time at 2am ahead of now
  let h = dt.hour() as i64;
  let m = dt.minute() as i64;
  let s = dt.second() as i64;
  let h_until_2am;
  if h < 2 {
      h_until_2am = time - h;
  } else {
      h_until_2am = 24 + time - h;
  }

  let thresh_ts = dt.checked_add_signed(
    Duration::seconds(h_until_2am as i64 * 60 * 60 - m * 60 - s)).unwrap();
  thresh_ts
}


pub fn get_num_boxes(days_to_go: i32, study_intensity: i32, num_reset: i32) -> i32 {
  let t = days_to_go;
  // bins generated by recursive equation x_n = x_{n-1} + 2^n + 1 
  // applied 5 times to (2, 6)
  let bins = vec![
        (0, 1), (2, 6), (7, 15), (16, 32), (33, 65), (66, 130), (131, 259)];
  let mut i = 0;
    let mut found_bin = false;
  for (a, b) in bins {
    if a <= t && t <= b {
            found_bin = true;
      break;
    }
    i += 1;
  }
  assert!(found_bin, "deadline must be between 0 and 259 days in the future");
  let mut num_boxes = 2 + i;

  // discount num_boxes based on study_intensity and num_reset
  num_boxes = max(2, num_boxes - (2 - study_intensity));
  num_boxes = max(2, num_boxes - num_reset);

  num_boxes
}
