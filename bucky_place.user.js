// ==UserScript==
// @name         rPlace overlays
// @namespace    http://tampermonkey.net/
// @version      0.3
// @description  try to take over the canvas!
// @author       coolreader18
// @match        https://hot-potato.reddit.com/embed*
// @icon         https://www.google.com/s2/favicons?sz=64&domain=reddit.com
// @grant        none
// ==/UserScript==
if (window.top !== window.self) {
  window.addEventListener('load', () => {
    const makeImg = (src, static) => {
      const i = document.createElement("img");
      Object.assign(i.style, { position: "absolute", left: 0, top: 0, imageRendering: "pixelated", width: "2000px" });
      const upd = () => i.src = `https://raw.githubusercontent.com/${src}?${performance.now()}`;
      upd();
      if (!static) setInterval(upd, 30 * 1000);
      document.getElementsByTagName("mona-lisa-embed")[0].shadowRoot.children[0].getElementsByTagName("mona-lisa-canvas")[0].shadowRoot.children[0].appendChild(i);
    };
    makeImg("coolreader18/bucky_place/main/bucky_overlay.png", false);
  }, false);
}
