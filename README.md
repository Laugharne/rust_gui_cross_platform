
# Building Cross-Platform Apps with egui and Rust

[Building Cross-Platform Apps with egui and Rust](https://medium.com/@lyecre/building-cross-platform-apps-with-egui-and-rust-0a0fbb1779dc)

## Dépendances nécessaires

```toml
[dependencies]
eframe = "0.33.3"
egui   = "0.33.3"
image  = "0.25.9"
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

## Causes du décalage souris dans egui/web

### 1. **CSS sur le canvas (cause la plus fréquente)**

egui utilise `getBoundingClientRect()` pour calculer la position de la souris. Si le canvas a du **padding**, une **bordure**, un **margin**, ou une **transformation CSS**, les coordonnées sont faussées.

**Fix CSS :** s'assurer que le canvas n'a aucun décalage involontaire :

```css
canvas {
    display: block;  /* évite l'espace inline sous le canvas */
    margin:  0;
    padding: 0;
    border:  none;
    /* pas de transform: translate(...) */
}

body {
    margin:   0;
    padding:  0;
    overflow: hidden;
}
```


### 2. **Mauvais `devicePixelRatio` / confusion pixels logiques vs physiques**

eframe peut entrer dans une boucle de redimensionnement où il confond les pixels logiques (CSS) et physiques (device), ce qui décale les coordonnées.

Le canvas egui a **deux tailles** :
- `canvas.width/height` → pixels physiques (résolution de rendu)
- `canvas.style.width/height` → pixels CSS (taille affichée)

Si ces deux dimensions ne correspondent pas correctement via le `devicePixelRatio`, les coordonnées souris (toujours en pixels CSS) sont mal interprétées.

**Fix dans l'index HTML :** forcer le canvas à occuper exactement l'espace CSS attendu :

```html
<style>
    html, body {
        margin  : 0;
        padding : 0;
        overflow: hidden;
        height  : 100%;
    }
    canvas {
        display: block;
        width  : 100%;
        height : 100%;
    }
</style>
```


### 3. **Canvas intégré dans une page avec d'autres éléments HTML**

Quand le canvas est intégré dans une page plus large et qu'il a une bordure ou du padding, egui utilise `getBoundingClientRect()` mais ne soustrait pas correctement ces valeurs. Ce bug a été corrigé dans les versions récentes d'eframe — vérifier que vous utilisez **eframe ≥ 0.28**.


### 4. **Firefox avec `privacy.resistFingerprinting`**

Firefox avec `privacy.resistFingerprinting=true` (activé par "Enhanced Tracking Protection") fait mentir les coordonnées souris à l'application, causant exactement ce décalage. La solution côté utilisateur est de désactiver ce réglage pour le site, ou de mettre `privacy.resistFingerprinting` à `false` dans `about:config`.

Ce cas n'est pas corrigeable côté code egui.


### 5. **Zoom navigateur différent de 100%**

Si l'utilisateur a zoomé dans son navigateur, le `devicePixelRatio` change et peut provoquer un décalage si egui ne se recalibrait pas correctement. eframe gère normalement cela, mais vérifier que vous n'overridez pas `pixels_per_point` manuellement avec une valeur fixe :

```rust
// ❌ À éviter si le zoom peut varier
// ctx.set_pixels_per_point(2.0);

// ✅ Laisser eframe gérer automatiquement
```


## Ressources supplémentaires
- [egui Documentation](https://docs.rs/egui)
- [eframe Documentation](https://docs.rs/eframe)
- [egui Github](https://github.com/emilk/egui)
- [Trunk Documentation](https://trunkrs.dev/)
- [egui Web Demo](https://www.egui.rs/)
- [Rust GUI with Neowin - YouTube](https://www.youtube.com/playlist?list=PLOeWRYj1QznUX08O4K1Ibh1YM9G_ew6iM)
- [GoCelesteAI / Repositories · GitHub](https://github.com/GoCelesteAI?tab=repositories&q=EGUI&type&language&sort)

