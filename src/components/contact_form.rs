use yew::prelude::*;

#[function_component(ContactForm)]
pub fn contact_form() -> Html {
    html!(
      <div class="contact-form px-4">
        <h1 class="font-6xl text-white text-center mb-3">{"Contact Me"}</h1>
        <p>{"If you'd like to chat about a project, please complete the form below. I'll get back to you soon!"}</p>

        <div class="container form-container">

          <form class="contact-form__form">
            <div class="row">
              <div class="form-group col-sm-6">
                <label for="name">{"Name"}</label>
                <input type="text" class="form-control" id="name" placeholder="Name..." />
              </div>

              <div class="form-group col-sm-6">
                <label for="email">{"Email address"}</label>
                <input type="email" class="form-control" id="email" placeholder="Email address..." />
              </div>
            </div>

            <div class="form-group">
              <label for="subject">{"Subject"}</label>
              <input type="text" class="form-control" id="subject" placeholder="Subject..." />
            </div>

            <div class="form-group">
              <label for="message">{"Message"}</label>
              <textarea class="form-control" id="message" rows="5" placeholder="Message..."></textarea>
            </div>

            <button type="submit" class="btn btn-lg btn-primary mt-4 px-4 py-2">{"Send"}</button>
          </form>
        </div>
      </div>
    )
}
