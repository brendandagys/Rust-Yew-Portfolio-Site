use yew::prelude::*;

#[function_component(IconGallery)]
pub fn icon_gallery() -> Html {
    let icons = vec![
        "javascript-icon.svg",
        "typescript-icon.svg",
        "react-icon.svg",
        "redux-icon.svg",
        "nextjs-icon.svg",
        "python-icon.svg",
        "django-icon.svg",
        "cplusplus-icon.svg",
        "node-js-icon.svg",
        "docker-icon.svg",
        "graphql-icon.svg",
        "postgresql-icon.svg",
        "sql-icon.svg",
        "git-icon.svg",
        "redis-icon.svg",
        "material-ui-icon.svg",
        "tailwindcss-icon.svg",
        "bootstrap5-icon.png",
        "html5-icon.svg",
        "css-icon.svg",
        "jquery-icon.svg",
    ];

    let icons_html = icons.iter().map(|icon| {
        html! {
            <img class="m-3" src={format!("images/{icon}")} alt="" width="65" height="65" />
        }
    });

    html!(
        <div class="d-flex flex-wrap justify-content-center">
          { for icons_html }
        </div>
    )
}
