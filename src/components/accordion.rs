use yew::prelude::*;
use super::accordion_content::{ArchitectureContent, BackgroundContent, SkillsContent};

#[function_component(Accordion)]
pub fn accordion() -> Html {
  let background_open = use_state(|| false);
  let skills_open = use_state(|| false);
  let architecture_open = use_state(|| false);

  let background_class = if *background_open {"open"} else {""};
  let skills_class = if *skills_open {"open"} else {""};
  let architecture_class = if *architecture_open {"open"} else {""};

  fn toggle_title_state(title_state: UseStateHandle<bool>) {
    title_state.set(!*title_state);
  }

  html! {
    <div class="accordion mx-auto">
      <div class="accordion__title" onclick={move |_| toggle_title_state(background_open.clone())}>
        <p class={format!("accordion__chevron accordion__chevron--{}", background_class)}>{"›"}</p>{"My background"}
      </div>
      <div class={format!("accordion__content accordion__content--{}", background_class)}>
        <BackgroundContent /> 
      </div>

      <div class="accordion__title" onclick={move |_| toggle_title_state(skills_open.clone())}>
        <p class={format!("accordion__chevron accordion__chevron--{}", skills_class)}>{"›"}</p>{"Technologies and skills"}
      </div>
      <div class={format!("accordion__content accordion__content--{}", skills_class)}>
        <SkillsContent /> 
      </div>

      <div class="accordion__title" onclick={move |_| toggle_title_state(architecture_open.clone())}>
        <p class={format!("accordion__chevron accordion__chevron--{}", architecture_class)}>{"›"}</p>{"My background"}
      </div>
      <div class={format!("accordion__content accordion__content--{}", architecture_class)}>
        <ArchitectureContent /> 
      </div>
    </div>
  }
}
