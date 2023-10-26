import { message } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import * as L from "leaflet";

type HeatmapData = {
  lat: number;
  lng: number;
  count: number;
};

export class Map {
  heatmapLayer: HeatmapOverlay<string, string, string>;

  constructor() {
    this.heatmapLayer = this.load_map();
  }

  load_map(): HeatmapOverlay<string, string, string> {
    const baseLayer = L.tileLayer(
      "https://tile.openstreetmap.org/{z}/{x}/{y}.png",
      {
        maxZoom: 19,
        attribution:
          '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>',
      },
    );

    const cfg = {
      radius: 0.0004,
      maxOpacity: 0.6,
      scaleRadius: true,
      blur: 1,
      useLocalExtrema: false,
    };

    const heatmapLayer = new HeatmapOverlay(cfg);

    const map = new L.Map("map", {
      center: new L.LatLng(45.5, -73.5),
      // @ts-ignore
      layers: [baseLayer, heatmapLayer],
      zoom: 10,
    });

    setTimeout(() => map.invalidateSize(), 3000);

    return heatmapLayer;
  }

  async display_data(
    exportUuid: String | null,
    yearSelection: HTMLSelectElement | null,
  ) {
    if (exportUuid == null) {
      await message("No fit files loaded.", "Heatmap");
      return;
    }

    if (yearSelection == null) {
      await message("No year selected.", "Heatmap");
      return;
    }

    let year = yearSelection.value;

    let data;
    if (year == "All") {
      data = <HeatmapData[]>(
        await invoke("display_all_data", { uuid: exportUuid })
      );
    } else {
      data = <HeatmapData[]>await invoke("display_data", {
        uuid: exportUuid,
        year: parseInt(year),
      });
    }

    await this.setHeatmapLayerData(data);
  }

  private async setHeatmapLayerData(activityData: HeatmapData[]) {
    if (activityData.length == 0) {
      await message("No activities found.", {
        title: "Heatmap",
        type: "error",
      });

      return;
    }

    const data = {
      max: 8,
      min: 0,
      data: activityData,
    };

    this.heatmapLayer.setData(data);
  }
}
