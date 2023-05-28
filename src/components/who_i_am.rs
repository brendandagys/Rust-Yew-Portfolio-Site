use yew::prelude::*;

use crate::components::IconGallery;

#[function_component(WhoIAm)]
pub fn who_i_am() -> Html {
    let technology_icons = vec![
        "javascript-icon.svg".to_string(),
        "typescript-icon.svg".to_string(),
        "react-icon.svg".to_string(),
        "redux-icon.svg".to_string(),
        "nextjs-icon.svg".to_string(),
        "python-icon.svg".to_string(),
        "django-icon.svg".to_string(),
        "cplusplus-icon.svg".to_string(),
        "node-js-icon.svg".to_string(),
        "docker-icon.svg".to_string(),
        "graphql-icon.svg".to_string(),
        "postgresql-icon.svg".to_string(),
        "sql-icon.svg".to_string(),
        "git-icon.svg".to_string(),
        "redis-icon.svg".to_string(),
        "material-ui-icon.svg".to_string(),
        "tailwindcss-icon.svg".to_string(),
        "bootstrap5-icon.png".to_string(),
        "html5-icon.svg".to_string(),
        "css-icon.svg".to_string(),
        "jquery-icon.svg".to_string(),
    ];

    let certification_icons = vec![
      "sap.png".to_string(),
      "dop.png".to_string(),
      "scs.png".to_string(),
      "soa.png".to_string(),
      "saa.png".to_string(),
      "dva.png".to_string(),
      "ccp.png".to_string(),
    ];
    
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
        <IconGallery icons={technology_icons}/>

        <p class="who-i-am__certifications mt-5 mb-4 text-center">{"MY CERTIFICATIONS:"}</p>
        <IconGallery icons={certification_icons} size={140} />

      </div>
    }
}
