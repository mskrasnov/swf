# ⛅ SWF - Simple Weather Forecast

![](assets/swf.gif)

SWF is a console program for get weather data for a given location. Uses OpenWeatherMap API. Supports configuration files with description of the main program parameters.

Displays ASCII art of the current weather, the time at the selected location (the time for weather data was measured, not the current time), and the temperature. More features will be added in the future (maybe).

![](assets/swf.png)

## Usage

- `--location` - location where you want to display weather: `--location=Vladimir,Vladimir\ Oblast,RU`
- `--units=` - units in which you want to output the information. Valid values:
  - `metric`;
  - `imperial`;
- `--api-key=` - OWM API key;
- `--config=` - path to the alternate config (default: `~/.config/swf/toml`);

Keys passed to the program have higher priority than data from the config.

## Configuration

The configuration is described in a TOML format file. It describes the following parameters:

- Location;
- Units;
- API Key;

The configuration is contained in the `~/.config/swf.toml`.

Each of the parameters is *optional*, i.e. may not be in the config. If a parameter is not in the config, it must be specified by passing the value to the corresponding key.

Example:

```toml
location = "Vladimir,RU"
units = "metric"
```

The `api_key` parameter is not specified here, so it must be specified in the `--api-key` passed to `swf`:

```bash
swf --api-key=8sdfg6sd79f69sd87f6gsd9679
```

## Local build

### Dependencies

- `rustc` and `cargo` for build;

### Build

```bash
cargo build --release
cp -v ./target/release/swf ~/.local/bin
```

## Support me (Russia)

Если вы хотите поддержать этот проект, вы можете либо принять участие в его разработке, либо отправить донат мне на карту сбербанка:

> 2202 2062 5233 5406 (Михаил Сергеевич К.)

Участие в разработке SWF или донаты покажут, что эта утилита ещё кому-то нужна, а значит будут мотивировать меня продолжать разработку и добавлять новые функции и исправлять ошибки.

## License

SWF distributed under MIT license.
