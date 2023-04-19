import './style.css'
// import { setupCounter } from './counter.js'

 import init, { process } from "./wasm/img_map.js"
  async function main() {
    await init();
  }
  main();
  const original_img = document.getElementById("img");
  const regex_base64 = /data:image\/([a-zA-Z]*);base64,([^\"]*)/;

  document.getElementById("file").addEventListener("change", (event) => {
    let img_file = event.target.files[0];
    var reader = new FileReader();
    reader.onloadend = function (e) {
      img.src = reader.result;
    }
    reader.readAsDataURL(img_file);
    original_img.style.display = "inline";
  });


  const btn = document.querySelectorAll(".rw");

  btn.forEach(element => {
    element.addEventListener("click", () => {
      if (original_img.src == ""){
        return
      }

      let filter = element.textContent;
      element.setAttribute("disabled", true);

      const base64_img = original_img.src.match(regex_base64)[2];
      let processed_base64 = process(base64_img, filter);

      element.setAttribute("disabled", false);

      original_img.src = "data:image/png;base64," + processed_base64;
      document.getElementById("saveit").href = original_img.src;
    });
  });
// document.querySelector('#app').innerHTML = `
//   <div>
//     <div class="card">
//       <button id="counter" type="button"></button>
//     </div>
//   </div>
// `

// setupCounter(document.querySelector('#counter'))
