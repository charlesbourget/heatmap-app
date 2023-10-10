import { invoke } from "@tauri-apps/api/tauri";
import {message, open} from '@tauri-apps/api/dialog';
import * as L from "leaflet";
import {Spinner} from "./spinner.ts";

type HeatmapData = {
  lat: number;
  lng: number;
  count: number;
}

class App {
  heatmapLayer: HeatmapOverlay<string, string, string> | null
  yearSelection: HTMLSelectElement | null
  spinner: Spinner
  exportUuid: string | null = null;

  constructor() {
    this.heatmapLayer = null;
    this.spinner = new Spinner();
    this.yearSelection = <HTMLSelectElement | null>document.getElementById("year_selection")

    addEventListener("DOMContentLoaded", (_: Event) => {
      this.heatmapLayer = this.load_map();
    });

    const load_fit_files = document.getElementById("load_fit_files");
    if (load_fit_files != null) {
      load_fit_files.addEventListener("click", (_: Event) => this.load_activities())
    }

    const display = document.getElementById("display");
    if (display != null) {
      display.addEventListener("click", (_: Event) => this.display_data())
    }
  }

  async display() {

  }

  async load_activities() {
    const selectedPath = await open({
      directory: true
    });

    this.spinner.show_spinner();
    const uuid = <string> await invoke("load_fit_files", { path: selectedPath })

    if (uuid != null) {
      this.exportUuid = uuid;
      this.spinner.hide_spinner();
      await this.populate_dropdown(uuid);
    }
  }

  async populate_dropdown(uuid: string) {
    const availableYears: number[] | null = await invoke("get_available_years", { uuid: uuid })

    if (availableYears != null) {
      availableYears.forEach((year) => {
        const option = document.createElement("option");
        option.text = year.toString();

        if (this.yearSelection != null) {
          this.yearSelection.add(option)
        }
      })
    }
  }

  async display_data() {
    if (this.exportUuid == null) {
      await message('No fit files loaded.', 'Heatmap');
      return;
    }

    if (this.yearSelection == null) {
      await message('No year selected.', 'Heatmap');
      return;
    }


    let year = this.yearSelection.value;

    if (year == "All") {
      // Do something with All
    }

    const data = <HeatmapData[]> await invoke("display_data", { uuid: this.exportUuid, year: parseInt(year) });

    if (data.length == 0) {
        await message('No activities found.', { title: 'Heatmap', type: 'error' });
        return;
      }

      const testData = {
        max: 8,
        data: data
      };

      if (this.heatmapLayer != null) {
        // @ts-ignore
        this.heatmapLayer.setData(testData)
      }
  }

  load_map(): HeatmapOverlay<string, string, string> {
    const baseLayer = L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
      maxZoom: 19,
      attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>'
    });

    const cfg = {
      "radius": 0.0005,
      "maxOpacity": .8,
      "scaleRadius": true,
      "useLocalExtrema": true,
    };

    const heatmapLayer = new HeatmapOverlay(cfg)

    const map = new L.Map('map', {
      center: new L.LatLng(45.5, -73.5),
      // @ts-ignore
      layers: [baseLayer, heatmapLayer],
      zoom: 10
    });

    setTimeout(() => map.invalidateSize(), 3000);

    return heatmapLayer;
  }
}

new App();