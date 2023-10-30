# Heatmap-app

Was tired of paying Strava premium to look at the personnal Heatmap so
re-created it as a Desktop Application with Tauri (Rust + Ts).

## How to run

### Required

- rust
- nodejs
- yarn

### Run dev build

```shell
yarn tauri dev
```

### Build production build

```shell
yarn tauri build
```

### Data source

Once the application is up and running, you will need a strava export of all your activities in either fit or gpx format.
To obtain that data you could get it from your Strava Settings page. Once you download that folder, extract it and your activities
are located in `activities/`.
