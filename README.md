# Workout Tracker - Rust WebAssembly Version

This workout tracker application has been remade in Rust using WebAssembly (WASM) for improved performance and type safety.

## 🦀 Rust Implementation

The core application logic has been converted from JavaScript to Rust and compiled to WebAssembly. This provides several benefits:

- **Type Safety**: Rust's strong type system prevents many runtime errors
- **Memory Safety**: Rust manages memory safely without a garbage collector
- **Performance**: WebAssembly can offer better performance for computational tasks
- **Cross-platform**: The same Rust code can be compiled for multiple targets

## 📁 Project Structure

- `src/lib.rs` - Main Rust library with workout tracking logic
- `pkg/` - Generated WebAssembly files
- `script.js` - JavaScript interface that calls the WebAssembly module
- `workout-data.js` - Workout schedule data (converted to Rust-compatible format)
- `index.html` - Main application interface
- `style.css` - Application styling
- `Cargo.toml` - Rust project configuration

## 🔧 Building

To rebuild the WebAssembly module after making changes to the Rust code:

```bash
wasm-pack build --target web --out-dir pkg
```

## 📱 Features

All original features have been preserved:

- Daily workout tracking (sit-ups, push-ups, squats, pull-ups)
- Progress persistence using localStorage
- Calendar view showing completed workouts and rest days
- Responsive design for mobile and desktop
- Progressive Web App (PWA) support for offline use

## 🔄 Fallback Support

The application includes fallback support - if WebAssembly fails to load, it will automatically fall back to the original JavaScript implementation.

## 🚀 Usage

Simply open `index.html` in a web browser. The application will automatically load the WebAssembly module and provide the same user experience as the original JavaScript version.

## 🔗 Original Files

The original JavaScript implementation has been preserved:

- `index-js.html` - Original HTML file
- `script-js.js` - Original JavaScript logic
- `workout-data-js.js` - Original workout data format

## 🛠️ Development

1. Make changes to `src/lib.rs`
2. Rebuild with `wasm-pack build --target web --out-dir pkg`
3. Test the application in a web browser
4. The JavaScript interface in `script.js` handles integration with the DOM

## 📊 Performance

The Rust WebAssembly version provides type-safe workout data management and efficient state handling while maintaining the same user interface and experience as the original JavaScript version.