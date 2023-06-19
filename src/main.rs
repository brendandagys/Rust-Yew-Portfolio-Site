use yew::prelude::*;

mod components;
use crate::components::{Accordion, ContactForm, Footer, Landing, Projects, WhoIAm};

#[function_component(App)]
fn app() -> Html {
    html!(
        <div class="app">
          <Landing />
          <WhoIAm />
          <div style="position: relative; bottom: 70px;"><Accordion /></div>
          <Projects />
          <ContactForm />
          <Footer />
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
