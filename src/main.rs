use yew::prelude::*;

mod components;
use crate::components::{Landing, WhoIAm, ContactForm, Footer};

#[function_component(App)]
fn app() -> Html {
    html!(
        <div class="app">
          <Landing />
          <WhoIAm />
          <ContactForm />
          <Footer />
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
