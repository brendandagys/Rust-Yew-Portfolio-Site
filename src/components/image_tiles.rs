use yew::prelude::*;

#[function_component(ImageTiles)]
pub fn image_tiles() -> Html {
  html!(
      <div class="w-100">
        <div class="image-tiles mx-auto">
          <img src="images/neighborhood-portrait.jpg" alt="neighborhood-portrait" />
          <img src="images/algonquin-portrait.jpg" alt="algonquin-portrait" />
          <img src="images/boston.jpg" alt="boston" />
          <img src="images/algonquin-canoeing.jpg" alt="algonquin-canoeing" />
          <img src="images/ibanez-artcore-ag95-guitar.jpg" alt="ibanez-guitar" />
        </div>
      </div>
  )
}
