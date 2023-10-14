import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { Spinner } from "./spinner.ts";
import { Map } from "./map.ts";

class App {
  map: Map;
  yearSelection: HTMLSelectElement | null;
  spinner: Spinner;
  exportUuid: string | null = null;

  constructor() {
    this.spinner = new Spinner();
    this.yearSelection = <HTMLSelectElement | null>(
      document.getElementById("year_selection")
    );

    this.map = new Map();

    const load_fit_files = document.getElementById("load_fit_files");
    if (load_fit_files != null) {
      load_fit_files.addEventListener("click", (_: Event) =>
        this.load_activities(),
      );
    }

    const display = document.getElementById("display");
    if (display != null) {
      display.addEventListener("click", (_: Event) =>
        this.map.display_data(this.exportUuid, this.yearSelection),
      );
    }
  }

  private async load_activities() {
    const selectedPath = await open({
      directory: true,
    });

    this.spinner.show_spinner();
    const uuid = <string>await invoke("load_fit_files", { path: selectedPath });

    if (uuid != null) {
      this.exportUuid = uuid;
      this.spinner.hide_spinner();
      await this.populate_dropdown(uuid);
    }
  }

  private async populate_dropdown(uuid: string) {
    const availableYears: number[] | null = await invoke(
      "get_available_years",
      { uuid: uuid },
    );

    if (availableYears != null) {
      availableYears.forEach((year) => {
        const option = document.createElement("option");
        option.text = year.toString();

        if (this.yearSelection != null) {
          this.yearSelection.add(option);
        }
      });
    }
  }
}

new App();
