# BubbleBob

BubbleBob is a scuba diving simulator which calculates to minimum and maximum depth divers can go to according to the gas mix they use.

## Run the app

Follow the following procedure (skip steps if they are not needed):
- Install rust ([click here](https://www.rust-lang.org/tools/install))
- Install the Tauri CLI by running the following command: cargo install tauri-cli
- Clone the repository
- Go to the root of the repository and run the following command: cargo run build

The location of the output file should be displayed in your terminal.

## Operating depth calculations

The app uses the following limits to calculate the operating depth of the gas mix:
- Human body dioxygen needs to sustain life: minimum partial pressure of dioxygen of 0.16
- Central nervous system oxygen toxicity: maximum partial pressure of diogygen of 1.3
- Nitrogen narcosis: maximum partial pressure of dinitrogen of 3.12
- Absolute pressure component of the high-pressure nervous syndrome (HPNS): maximum absolute pressure of helium of 31

## License and copyright

License file: LICENSE.txt
Copyright notice file: copyright notice.txt
