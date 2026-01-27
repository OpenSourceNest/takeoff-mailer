use std::env;

pub struct Config {
    pub rabbitmq_url: String,
    pub mail_password: String,
    pub mail_host: String,
    pub mail_port: u16,
    pub mail_username: String,
    pub mail_sender: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            rabbitmq_url: env::var("RABBITMQ_URL").expect("MISSING RABBITMQ_URL"),
            mail_password: env::var("MAIL_PASSWORD").expect("MISSING MAIL_PASSWORD"),
            mail_host: env::var("MAIL_HOST").expect("MISSING MAIL_HOST"),
            mail_username: env::var("MAIL_USERNAME").expect("MISSING MAIL_USERNAME"),
            mail_sender: env::var("MAIL_SENDER").expect("MISSING MAIL_SENDER"),
            mail_port: env::var("MAIL_PORT")
                .expect("MISSING MAIL_PORT")
                .parse()
                .expect("MAIL_PORT must be a number"),
        }
    }
}
