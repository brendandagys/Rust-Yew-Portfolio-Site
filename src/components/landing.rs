use yew::prelude::*;

use crate::components::ImageTiles;

#[function_component(Landing)]
pub fn landing() -> Html {
    html!(
      <div class="landing text-center px-auto py-5">
        <h1 class="font-6xl">{"TypeScript, Rust, and Python full-stack developer"}</h1>
        <p class="font-4xl mb-5">{"Hi, I'm Brendan. Welcome to my site!"}</p>

        <ImageTiles />
      </div>
    )
}
