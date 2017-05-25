blowtorch
=========
> Burn. Efficiently.

blowtorch is a powerful and efficient OS image flasher write in Rust
fueled by the [acetylene](https://github.com/vberset/acetylene) library.

## Installation

### From source

1. Clone the repo: `git clone https://github.com/vberset/blowtorch.git`
2. Move to the folder: `cd blowtorch`
3. Install it with cargo: `cargo install`

## Usage

To burn the file `SomeOS.img` to the device `/dev/mmcblk0`, run

```terminal
blowtorch burn SomeOS.img -d /dev/mmcblk0
```

To get the list of available devices, run

```terminal
blowtorch devices
```

## Contributing

**Note:**
This project is released with a Contributor Code of Conduct.
By participating in this project **you agree** to abide by its terms.
See the [code_of_conduct.md](code_of_conduct.md) file for details.

1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin my-new-feature`
5. Submit a pull request

## Authors

* **Vincent Berset** - Maintainer - [vberset](https://github.com/vberset)

## License

This project is licensed under the GNU Affero General Public License -
see the [LICENSE.md](LICENSE.md) file for details.
