use yew::prelude::*;

#[function_component(ArchitectureContent)]
pub fn architecture_content() -> Html {
    html! (
        
    )
}

#[function_component(BackgroundContent)]
pub fn background_content() -> Html {
    html! (
        <div class="m-5">
            <p>
              {
              "I live in Toronto, Ontario, Canada and graduated from Ryerson University
              with a Bachelor’s degree in Biomedical Engineering. After working for a
              few years as an engineer, I developed a passion for coding, data analysis,
              and machine learning. Although we learned most of these skills in
              university, I chose to further my education in these areas and apply it to
              the field of healthcare, which I am passionate about. Since then, I have
              worked at two large Ontario hospitals in data-centric roles."
              }
            </p>
            <p>
              {
                "At these organizations, I was able to identify many processes that were
                  ripe for improvement and apply software solutions to fix them. This
                  involved the automation of numerous tasks, such as data retrieval
                  (databases, files, FTP servers, etc.), data cleaning/munging, and
                  dissemination of data and analyses (emails, file folders, applications,
                  etc.). This skillset improved the way that the organizations utilized
                  their data holdings, dashboards, and various reports that come from
                  multifarious internal and external systems."
              }
            </p>
        </div>
    )
}

#[function_component(SkillsContent)]
pub fn skills_content() -> Html {
    html! (
        
    )
}
