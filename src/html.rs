use askama::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

pub fn render_html(raw: String) -> String {
    let hello = HelloTemplate { name: &raw };
    hello.render().unwrap()
}
