use {
    crate::Result,
    handlebars::Handlebars,
    serde::ser::Serialize,
    std::{fs, path::PathBuf},
};

pub struct Template {
    templates_dir: PathBuf,
}

impl Template {
    pub fn new(templates_dir: &'static str) -> Result<Self> {
        let templates_dir = PathBuf::from(templates_dir);
        Ok(Self {
            // cache: HashMap::new(),
            templates_dir,
        })
    }

    pub fn render<T>(&self, name: &'static str, data: T) -> Result<String>
    where
        T: Serialize,
    {
        // always rendering in dev-reload mode, currently.
        let tmpl = fs::read_to_string(self.templates_dir.join(format!("{}.hbs", name))).unwrap();
        let s = Handlebars::new().render_template(&tmpl, &data).unwrap();
        Ok(s)
    }
}
