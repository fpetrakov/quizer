# quizer
rust app that creates cli quiz for you!

## Installation

1. Download binary release or build from source using cargo
2. Make binary executable 
```console
chmod +x /path/to/binary/quizer
```

## Usage

1. Create questions.ini file with questions, for example:
```console
[1]
Question="What is my favorite color?"
Answer="Red"

[2]
Question="What is my name?"
Answer="Fedya"
```
2. Run quizer and pass path to questions.ini file
```console
quizer /path/to/questions.ini
```