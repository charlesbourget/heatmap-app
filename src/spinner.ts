export class Spinner {
  spinnerDOMElement: HTMLElement | null;

  public constructor() {
    this.spinnerDOMElement = document.getElementById("loader");
  }

  public show_spinner() {
    if (this.spinnerDOMElement != null) {
      this.spinnerDOMElement.style.display = "block";
    }
  }

  public hide_spinner() {
    if (this.spinnerDOMElement != null) {
      this.spinnerDOMElement.style.display = "none";
    }
  }
}
