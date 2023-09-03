use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HelloTemplate<'a> {
    pub title: &'a str,
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct About<'a> {
    pub title: &'a str,
}

#[derive(Template)]
#[template(path = "debt.html")]
pub struct Debt<'a> {
    pub title: &'a str,
}

#[derive(Template)]
#[template(path = "finances.html")]
pub struct Finances<'a> {
    pub title: &'a str,
}

#[derive(Template)]
#[template(path = "income.html")]
pub struct Income<'a> {
    pub title: &'a str,
}

#[derive(Template)]
#[template(path = "retirement.html")]
pub struct Retirement<'a> {
    pub title: &'a str,
}

#[derive(Template)]
#[template(path = "investment.html")]
pub struct Investment<'a> {
    pub title: &'a str,
}

#[derive(Template)]
#[template(path = "budget.html")]
pub struct Budget<'a> {
    pub title: &'a str,
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFound<'a> {
    pub title: &'a str,
}
