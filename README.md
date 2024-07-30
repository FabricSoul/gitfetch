# Gitfetch

Gitfetch is a command-line information tool written in Rust, inspired by [Neofetch](https://github.com/dylanaraps/neofetch). It provides a visually appealing way to display Git contribution information.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

## Features

- Prints out contribution information similar to how Neofetch displays system information
- Automatically detects the global Git user
- Allows specifying any user and year for contribution data
- Customizable contribution graph and output color

## Installation

### Prerequisites

- Git (optional, for global user detection)

### Arch Linux

```bash
sudo pacman -S gitfetch
```

### Homebrew

```bash
brew install gitfetch
```

### Installing from crates.io

Install Gitfetch directly from crates.io using Cargo:

```bash
cargo install --locked gitfetch
```

## Usage

1. Generate a new [GitHub Token](https://github.com/settings/tokens) with the `read:user` scope to fetch data from GitHub.

2. Add your token to Gitfetch:

   ```bash
   gitfetch add-token <your-token-here>
   ```

3. Run Gitfetch:

   ```bash
   gitfetch
   ```

### Optional Arguments

- `-u` or `--user`: Specify a GitHub username
- `-y` or `--year`: Specify a year for contribution data

Example:

```bash
gitfetch -u FabricSoul -y 2023
```

## Configuration

Configuration path is `~/.config/gitfetch/config.toml`

### \[graph_colors\]

`level1`: "r,g,b"
`level2`: "r,g,b"
`level3`: "r,g,b"
`level4`: "r,g,b"

### \[text_colors\]

`info_color`: "r,g,b"

### \[graph_data\]

`percentiles`: [usize,uszie,uszie,uszie]

## Roadmap

- [x] Customize the graph color
- [x] Customize output text color
- [x] Customize graph display
- [x] Use `git` to get username
- [x] Specify a user
- [x] Specify a year
- [x] Display highest contribution
- [x] Display longest streak
- [x] Display current streak
- [ ] Add support for other Git hosting platforms

## Contributing

We welcome contributions to Gitfetch! Here's how you can help:

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a pull request

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

---

**Note:** Gitfetch is under active development. Features and documentation may be incomplete or subject to change. We appreciate your feedback and contributions!
