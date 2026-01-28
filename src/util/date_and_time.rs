use chrono::{TimeZone, Utc};

use crate::email_system::EmailTask;

pub fn log_timestamp(task: &EmailTask) {
    if let Some(ts) = task.created_at {
        // Convert Protobuf Timestamp to Chrono DateTime<Utc>
        match Utc.timestamp_opt(ts.seconds, ts.nanos as u32).single() {
            Some(dt) => {
                let now = Utc::now();
                let duration = now.signed_duration_since(dt);
                let minutes = duration.num_minutes();
                let seconds = duration.num_seconds() % 60;
                let readable_time = dt.format("%Y-%m-%d %H:%M:%S UTC").to_string();

                println!(
                    "[INFO] Message originally created at: {} ({}m {}s ago)",
                    readable_time, minutes, seconds
                );

                // Logic Check: If the message is from the future (clock skew), handle it
                if duration.num_seconds() < 0 {
                    println!("[WARN] Message has a future timestamp. Check VPS clock sync (NTP)!");
                }
            }

            None => {
                eprintln!("[ERROR] Invalid timestamp in EmailTask");
                return;
            }
        }
    }
}
