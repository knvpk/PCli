use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        let _ = tera.add_raw_templates(vec![
            (
                "sts_id_token.html",
                include_str!("templates/sts_id_token.html"),
            ),
            ("sts_mfa.html", include_str!("templates/sts_mfa.html")),
        ]);
        tera.autoescape_on(vec![".html"]);
        tera
    };
}
