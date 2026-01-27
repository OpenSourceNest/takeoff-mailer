use lettre::message::Body;

pub fn AttendeeRegistrationSuccessful(first_name: &str) -> Body {
    return Body::new(format!(
        "Registration was successful! Welcome, {}.",
        first_name
    ));
}
