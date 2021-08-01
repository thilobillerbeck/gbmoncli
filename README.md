# gbmoncli

A CLI to control your Gigabyte monitor (currently only G27Q)

## Features

- Change your monitors values like
  - Brightness
  - Contrast
  - ...
- Switch monitor profiles

## Run Locally

You need Rust in order to run this project locally. In addition, you need sure your OS can access your monitor.

If you are set up, you can just open the project directory and run

```bash
  cargo run -- --help
```

## Usage/Examples

```bash
gbmoncli 0.1.0
Thilo Billerbeck <thilo.billerbeck@officerent.de>
A CLI to control your Gigabyte G27Q monitor.

USAGE:
    gbmoncli [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --black-equalizer <0-20>                                                     Change monitor black equalizer
    -b, --brightness <0-100>                                                         Change monitor brightness
    -c, --contrast <0-100>                                                           Change monitor contrast
    -f, --freesync <true|false>                                                      Turn AMD Freesync on/off
        --osd-time <0-6>
            Set the monitors OSD disappearing time (5 second increments)

        --osd-transparency <0-4>                                                     Set the monitors OSD transparency
    -p, --profile <standard|fps|rtsrpg|movie|reader|srgb|custom1|custom2|custom3>    Change the monitor profile
    -s, --sharpness <0-10>                                                           Change monitor sharpness
```

Set monitor brightness to 100% and contrast to 50%

```bash
gbmoncli -b 100 -c 50
```

## Authors

- [@thilobillerbeck](https://github.com/thilobillerbeck)
