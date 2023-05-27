use yew::prelude::*;

use crate::components::IconGallery;

#[function_component(WhoIAm)]
pub fn who_i_am() -> Html {
    html! {
      <div class="who-i-am container-fluid justify-content-center m-0 pb-4">

        <div class="col-xs-12">
          <div class="who-i-am__text px-4 py-3 pt-4 mx-auto text-center">
            <h1 class="my-6 font-4xl">{"Who I am"}</h1>
            <p>{"I am a passionate developer with a strong foundation in TypeScript/JavaScript, Python, and other languages. I have experience in analytics as well as with Cloud services and technologies, mainly AWS."}</p>
            <p>{"I love to solve problems and to build things. I have professional experience creating and designing software solutions through object-oriented as well as procedural programming. In my spare time you can find me outside biking, playing guitar, and working on numerous personal projects to further my interest and knowledge."}</p>
            <p>{"Thanks for visiting my site and reading this far! Below is a more detailed description about myself, and some examples of projects I've created. Feel free to contact me at the bottom of the page!"}</p>
          </div>
        </div>

        <p class="who-i-am__technologies mt-5 mb-4 text-center">{"A FEW OF MY FAVORITE TECHNOLOGIES..."}</p>
        
        <IconGallery />

      </div>
    }
}
