use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IconGalleryProps {
    pub icons: Vec<String>,
}

#[function_component(IconGallery)]
pub fn icon_gallery(IconGalleryProps { icons }: &IconGalleryProps) -> Html {
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
