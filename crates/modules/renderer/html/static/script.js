var inited = true;
class ImageOverlayManager {
  constructor(wrapper) {
    this.wrapper = wrapper;
    this.image = wrapper.querySelector(".base-image");
    this.overlays = JSON.parse(
      wrapper.dataset.overlays.replace(/&quot;/g, '"') || "[]",
    );
    wrapper.dataset.overlays = "[]";
    this.overlayElements = [];

    this.image.addEventListener("load", () => this.initializeOverlays());
    window.addEventListener("resize", () => this.updateOverlayPositions());

    if (this.image.complete) {
      this.initializeOverlays();
    }
  }

  createOverlayElement(overlay) {
    const div = document.createElement("div");
    div.className = "text-box";
    div.textContent = overlay.ocrText;
    div.style.backgroundImage = `url('${overlay.background}')`;

    this.updateOverlayPosition(overlay, div);
    $(div).fitText(0.5);
    return div;
  }

  updateOverlayPosition(overlay, element) {
    const scaleX = this.image.clientWidth / this.image.naturalWidth;
    const scaleY = this.image.clientHeight / this.image.naturalHeight;

    element.style.left = overlay.minX * scaleX + "px";
    element.style.top = overlay.minY * scaleY + "px";
    element.style.width = (overlay.maxX - overlay.minX) * scaleX + "px";
    element.style.height = (overlay.maxY - overlay.minY) * scaleY + "px";
  }

  initializeOverlays() {
    this.overlayElements = [];
    this.overlays.forEach((overlay) => {
      const box = this.createOverlayElement(overlay);
      this.wrapper.appendChild(box);
      this.overlayElements.push({ overlay, element: box });
    });
  }

  updateOverlayPositions() {
    this.overlayElements.forEach(({ overlay, element }) => {
      this.updateOverlayPosition(overlay, element);
    });
  }
}

function initImageOverlayManagers() {
  document.querySelectorAll(".canvas-wrapper").forEach((wrapper) => {
    new ImageOverlayManager(wrapper);
  });
}

if (document.readyState === "loading") {
  window.addEventListener("DOMContentLoaded", initImageOverlayManagers);
} else {
  initImageOverlayManagers();
}
