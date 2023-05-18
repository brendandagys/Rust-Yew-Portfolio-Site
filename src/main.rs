use yew::prelude::*;

mod video_details;
mod videos_list;

use video_details::VideoDetails;
use videos_list::{Video, VideosList};

#[function_component(App)]
fn app() -> Html {
    let videos = vec![
        Video {
            id: 1,
            title: "How to break prod".to_string(),
            speaker: "Noel Tomlinson".to_string(),
            url: "https://example.com".to_string(),
        },
        Video {
            id: 2,
            title: "Writing JavaScript without types".to_string(),
            speaker: "Josh Dupont".to_string(),
            url: "https://example.com".to_string(),
        },
        Video {
            id: 3,
            title: "Why HTML is a programming language".to_string(),
            speaker: "Sarah Trainor".to_string(),
            url: "https://example.com".to_string(),
        },
    ];

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
          <VideoDetails video={video.clone()} />
        }
    });

    html! {
      <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
          <h3>{"Videos to watch"}</h3>
          <VideosList videos={videos} on_click={on_video_select.clone()} />
        </div>
        { for details }
      </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
