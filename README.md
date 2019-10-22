

<h1> The C O ⅃ L A R - Software </h1>

[![Build Status](https://travis-ci.org/wmealing/loader.svg?branch=master)](https://travis-ci.org/wmealing/loader)

<!-- TABLE OF CONTENTS -->
## Table of Contents

* [About the Project](#about-the-project)
  * [Built With](#built-with)
* [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
* [Usage](#usage)
* [Roadmap](#roadmap)
* [Contributing](#contributing)
* [License](#license)
* [Contact](#contact)
* [Acknowledgements](#acknowledgements)



<!-- ABOUT THE PROJECT -->
## About The Project

The collar is a wearable hardware project intended to be a better assistant for when you're at home and when you're not.  It is an extensible hardware and software platform to suit my needs and maybe yours too.  I'm not going to stop you from hacking on this.

The basic hardware of the rasbperr pi will be available and the standard pins available for sensors to be attached that the wearer may need.

The pi zero apparently doesn't work well with both bluetooth or wifi enabled, not both.. maybe this can be overcome but I haven't figured it out yet.

A list of commonly used resources that I find helpful are listed in the acknowledgements.

### Built With
This section should list any major frameworks that you built your project using. Leave any add-ons/plugins for the acknowledgements section. Here are a few examples.
* [Rust](https://www.rust-lang.org/)
* [Travis-CI](https://travis-ci.org/)


<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

You'll need to have rust installed.

```sh
curl https://sh.rustup.rs -sSf | sh
```

This may ask you a number of questions, best to accept the defaults for now,
at the moment the system is going to target x86, but this will be changing
to ARM shortly.

The goal is to have this running on the pi/beagleboard soon.


### Installation

1. Clone the repo 
```sh
git clone https://github.com/wmealing/loader.git
```

3. Build the project
```sh
cargo build --release

```

4. Copy target/build/loader to your target system.


<!-- USAGE EXAMPLES -->
## Usage

#./loader


<!-- ROADMAP -->
## Roadmap

[X] Initial project started

[eh] Wearable designed ( see http://github.com/wmealing/design/ )

 - [IN PROGRESS] Wearable printed

 - [IN PROGRESS] Audio hardware
   - [x] Ordered audio hardware
   - [?] Order speaker
 
 - [ ] Integrate with amazons transcribe service.
 
 - [ ] Add recharge circuit. 

- [ ] Write online plugin browser
  
  - [ ] Make example plugin

- [ ] Write a FAAS online service

 - [ ]  Write an example FAAs.
 
 
<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request


<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.


<!-- CONTACT -->
## Contact

Your Name - [@wademealing](https://twitter.com/wademealing) -

Project Link: [https://github.com/wmealing/loader](https://github.com/wmealing/loader)



