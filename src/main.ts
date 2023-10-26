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

    const load_json_file = document.getElementById("load_json_file");
    if (load_json_file != null) {
      load_json_file.addEventListener("click", (_: Event) => this.load_json());
    }

    const display = document.getElementById("display");
    if (display != null) {
      display.addEventListener("click", (_: Event) =>
        this.map.display_data(this.exportUuid, this.yearSelection),
      );
    }

    const exportToJson = document.getElementById("export_to_json");
    if (exportToJson != null) {
      exportToJson.addEventListener("click", (_: Event) =>
        this.export_to_json(),
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

  private async load_json() {
    const selectedFile = await open();

    this.spinner.show_spinner();
    const uuid = <string>(
      await invoke("load_json_export", { path: selectedFile })
    );

    if (uuid != null) {
      this.exportUuid = uuid;
      this.spinner.hide_spinner();
      await this.populate_dropdown(uuid);
    }
  }

  private async export_to_json() {
    const selectedFile = await open({
      directory: true,
    });

    await invoke("create_json_export", {
      path: selectedFile,
      uuid: this.exportUuid,
    });
  }

  private async populate_dropdown(uuid: string) {
    const availableYears: number[] | null = await invoke(
      "get_available_years",
      { uuid: uuid },
    );

    if (availableYears != null) {
      if (this.yearSelection == null) {
        return;
      }

      for (let i = this.yearSelection.options.length - 1; i >= 1; i--) {
        this.yearSelection.remove(i);
      }

      availableYears.forEach((year) => {
        const option = document.createElement("option");
        option.text = year.toString();

        // @ts-ignore
        this.yearSelection.add(option);
      });
    }
  }
}

new App();
