use yew::prelude::*;

struct IconData {
    file: String,
    alt: String,
    url: String,
}

#[function_component(Footer)]
pub fn footer() -> Html {
    let icons = vec![
        IconData {
            file: "github-icon.svg".into(),
            alt: "GitHub profile".into(),
            url: "https://github.com/brendandagys".into(),
        },
        IconData {
            file: "linkedin-icon.svg".into(),
            alt: "LinkedIn profile".into(),
            url: "https://linkedin.com/in/brendandagys".into(),
        },
        IconData {
            file: "email-icon-3.svg".into(),
            alt: "Email me".into(),
            url: "mailto:brendandagys@gmail.com".into(),
        },
    ];

    let icons_html = icons.iter().map(|icon| {
        let IconData { file, alt, url } = icon;

        html!(
          <a href={url.clone()} target="_blank">
            <img
              class="m-3 mb-4"
              src={format!("images/{}", file.clone())}
              alt={alt.clone()}
              width="50"
              height="50"
            />
          </a>
        )
    });

    html!(
      <div class="footer-container"> 
        <div class="footer text-center py-3">
          { for icons_html }
          
          <p class="font-sm">{"Copyright © Brendan Dagys"}</p>
        </div>
      </div>
    )
}
