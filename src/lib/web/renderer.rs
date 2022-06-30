use crate::web::ctx;

#[derive(Debug,thiserror::Error)]
pub enum RenderError {
    #[error("Rendering Error : {0}")]
    Render(#[from] handlebars::RenderError)
}

pub struct Renderer<'a> (handlebars::Handlebars<'a>);

impl<'a> Renderer<'a> {
    // This function creates the registery that is used for rendering
    pub fn new(template_dir: std::path::PathBuf) -> Self {
        let mut renderer = handlebars::Handlebars::new();
        renderer
            .register_templates_directory(".hbs",&template_dir)
            .expect("Failed to register template");
        Self(renderer)
    }

    // This function converts anything that is serializable to json
    fn convert_to_value<S> (serializable : &S) -> serde_json::Value
    where
        S : serde::Serialize + std::fmt::Debug
    {
        serde_json::to_value(&serializable)
            .expect("Failed to convert structure to a value")
    }

    pub fn render<P> (&self,context:P,errors:&[&str]) -> String
    where
        P : ctx::PageContext + serde::Serialize + std::fmt::Debug,
    {
        // You get the json value
        let mut value = Self::convert_to_value(&context);
        // You mutably borrow the object and then add required keys to it because 
        // our context structs had limited entries so we can add some dynamically here
        if let Some(value) = value.as_object_mut() {
            value.insert("_errors".into(), errors.into());
            value.insert("_title".into(),context.title().into());
            value.insert("_base".into(),context.parent().into());
        }
        self.do_render(context.template_path(),value)
    }

    // Function to actuall render
    fn do_render(&self,path : &str,ctx : serde_json::Value) -> String {
        self.0.render(path,&ctx).expect("Error rendering tempelate")
    }

    // Same function as render you just add aditional user data with some context
    pub fn render_with_data<P,D>
    (
        &self,
        context:P,
        data : (&str,D),
        errors : &[&str]
    ) -> String
    where
        P : ctx::PageContext + serde::Serialize + std::fmt::Debug,
        D : serde::Serialize + std::fmt::Debug
    {
        let mut value = Self::convert_to_value(&context);
        if let Some(value) = value.as_object_mut() {
            value.insert("_errors".into(), errors.into());
            value.insert("_title".into(),context.title().into());
            value.insert("_base".into(),context.parent().into());
            value.insert(data.0.into(),handlebars::to_json(data.1));
        }
        self.do_render(context.template_path(), value)
    }
}