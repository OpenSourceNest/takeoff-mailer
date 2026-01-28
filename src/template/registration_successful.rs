use lettre::message::Body;
use tera::{Context, Tera};

pub fn attendee_registration_successful(first_name: &str) -> Result<Body, String> {
    let mut tera = Tera::default();

    let raw_html = include_str!("./registration_successful.html");

    let mut context = Context::new();
    context.insert("name", first_name);

    match tera.render_str(raw_html, &context) {
        Ok(html) => {
            eprintln!(
                "[TEMPLATE RENDERED] Successfully rendered template for {}",
                first_name
            );
            return Ok(Body::new(html));
        }

        Err(e) => {
            eprintln!("[TEMPLATE ERROR] Could not render template: {:?}", e);
            return Err(format!("Could not render template: {:?}", e));
        }
    };
}
