use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IconGalleryProps {
    pub icons: Vec<String>,
    pub size: Option<i16>,
}

#[function_component(IconGallery)]
pub fn icon_gallery(IconGalleryProps { icons, size }: &IconGalleryProps) -> Html {
    let icons_html = icons.iter().map(|icon| {
        html! {
            <img
              class="m-3"
              src={format!("images/{icon}")}
              alt=""
              width={format!("{}", size.clone().unwrap_or(65))}
              height={format!("{}", size.clone().unwrap_or(65))}
            />
        }
    });

    html!(
        <div class="d-flex flex-wrap justify-content-center px-4">
          { for icons_html }
        </div>
    )
}
