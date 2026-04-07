
# Building Cross-Platform Apps with egui and Rust

[Building Cross-Platform Apps with egui and Rust](https://medium.com/@lyecre/building-cross-platform-apps-with-egui-and-rust-0a0fbb1779dc)


```toml
[dependencies]
eframe = "0.33.3"
egui   = "0.33.3"
```

**Web App :**
```toml
[dependencies]
eframe  = "0.33.3"
egui    = "0.33.3"
image   = "0.25.9"
web-sys = "0.3.85"

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen-futures = "0.4.58"
web-sys              = "0.3.85"
```

**Finger printing :**
Firefox avec `privacy.resistFingerprinting=true` (activé par "**_Enhanced Tracking Protection_**") fait mentir les coordonnées souris à l'application, causant un décalage. La solution côté utilisateur est de désactiver ce réglage pour le site, ou de mettre `privacy.resistFingerprinting` à `false` dans `about:config`.

Ce cas n'est pas corrigeable côté code **egui**.


## Ressources supplémentaires
- [egui Documentation](https://docs.rs/egui)
- [eframe Documentation](https://docs.rs/eframe)
- [egui Github](https://github.com/emilk/egui)
- [Trunk Documentation](https://trunkrs.dev/)
- [egui Web Demo](https://www.egui.rs/)
- [Rust GUI with Neowin - YouTube](https://www.youtube.com/playlist?list=PLOeWRYj1QznUX08O4K1Ibh1YM9G_ew6iM)
- [GoCelesteAI / Repositories · GitHub](https://github.com/GoCelesteAI?tab=repositories&q=EGUI&type&language&sort)

