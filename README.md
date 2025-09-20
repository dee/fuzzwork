# Fuzzwork

A lightweight library for downloading and updating SQLite database dumps for EVE Online from fuzzwork.co.uk.

## Overview

Fuzzwork provides a simple and reliable way to keep your EVE Online static data up-to-date by automatically downloading and managing SQLite database dumps from the Fuzzwork data export service. This library serves as a building block for EVE Online applications that need access to the latest game data.

## Features

- 🔄 Automatic download of EVE Online SQLite database dumps
- 📦 Simple update mechanism to keep data current
- 🛠️ Lightweight and easy to integrate
- 🎯 Built specifically for use as a dependency in larger projects
- 📊 Access to comprehensive EVE Online static data

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fuzzwork = "0.1.0"
```

## Usage

This library provides a simple interface for downloading and managing EVE Online SQLite database dumps from fuzzwork.co.uk. Check the documentation for detailed API reference.

## Database Schema

The SQLite database contains EVE Online's static data including:

- **Items** - All items, ships, modules, etc.
- **Market Groups** - Market category structure  
- **Types** - Item type definitions
- **Regions** - Solar system and region data
- **Stations** - Station and structure information
- And much more EVE Online static data

Refer to the [EVE Online SDE documentation](https://developers.eveonline.com/resource/resources) for complete schema details.

## Requirements

- Rust 1.70+ (or latest stable)
- Windows (Linux support coming soon)
- Internet connection for downloading updates
- Sufficient disk space for SQLite database (~200-300 MB)

## Data Storage

On Windows, the SQLite database is automatically downloaded and extracted to:
```
%PROGRAMDATA%\fuzzwork\sqlite-latest.sqlite
```

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Data Source

This library downloads data from [fuzzwork.co.uk](https://www.fuzzwork.co.uk/), which provides regularly updated SQLite exports of EVE Online's Static Data Export (SDE).

## License

[MIT License](LICENSE) - see the LICENSE file for details.

## Acknowledgments

- **Fuzzwork** for providing reliable EVE Online data exports
- **CCP Games** for EVE Online and the Static Data Export
- The EVE Online developer community

## Related Projects

If you're building EVE Online tools, you might also be interested in:
- [EVE Online ESI](https://esi.evetech.net/) - Official EVE Online API
- [pyfa](https://github.com/pyfa-org/Pyfa) - Python fitting assistant
- [EVE Online SDE](https://developers.eveonline.com/resource/resources) - Official static data

---

*This project is not affiliated with CCP Games or EVE Online.*