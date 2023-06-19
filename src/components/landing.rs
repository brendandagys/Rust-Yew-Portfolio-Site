use yew::prelude::*;

use crate::components::ImageTiles;

#[function_component(Landing)]
pub fn landing() -> Html {
    html!(
      <div class="text-center px-auto py-5">
        <div class="title mx-auto my-5 p-4">
          <h1 class="font-6xl">{"TypeScript, Rust, and Python Full-Stack Developer"}</h1>
        </div>
        <p class="subtitle font-4xl my-5">{"Hi, I'm Brendan. Welcome to my site!"}</p>

        <ImageTiles />
      </div>
    )
}
