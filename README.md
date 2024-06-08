### KhaledFlow 🔑

![khaledflow](https://github.com/Christopher-L-Massie/khaledflow/assets/50225179/c7fbb075-1bd2-4480-9885-4f6c462fb9d9)

🎤 **KhaledFlow** - The Major Key to Coding! This esoteric, interpreted programming language is brought to you by the power of Rust. It's simple, it's effective, and it's here to help you shine. Let's get this money!

## Project Structure

```
.
├── build_and_run.sh # The Key to Success: Shell script to build and run the Docker container
├── Dockerfile # Secure the Bag: Dockerfile for building the project environment
├── khaledflow
│ ├── Cargo.toml # Major Configuration: Cargo file for the Rust project
│ ├── src
│ │ ├── interpreter.rs # The Voice of the Interpreter
│ │ ├── lexer.rs # The Lexer, Another One
│ │ ├── main.rs # The Main Event
│ │ └── parser.rs # The Parser, Bless Up
│ └── test_scripts
│ └── output.dj # Test Script, Let's Win
└── README.md # This Document, You Smart
```

## Getting Started

### Prerequisites

- 🐳 Docker - Because you a genius.

### Building and Running the Project

🔑 **Step 1**: Execute the Key to Success, the shell script:

```sh
'./build_and_run.sh'

This script will:

    1. Build the Docker image and tag it as khaledflow_image.
    2. Run the Docker container based on the built image.

Manual Docker Commands

If you prefer to do it manually, here’s how you secure the bag:

    1. Build the Docker Image:

        'docker build -t khaledflow_image .'

    2. Run the Docker Container:

        'docker run --rm khaledflow_image'
```

# Example Script

We got the best example script in the test_scripts directory:

## output.dj:

    ANOTHER_ONE my_string "string value here"
    ANOTHER_ONE my_float 51.1
    ANOTHER_ONE my_number 5
    SING my_number
    SING my_float
    SING my_string

## Project Modules

    Lexer (lexer.rs): Turns your script into tokens. Major key alert!
    Parser (parser.rs): Converts tokens into statements. Bless up!
    Interpreter (interpreter.rs): Executes the statements. Another one!

# Contributing

You smart. You loyal. You got ideas? Bring ‘em in! Open an issue or submit a pull request. We appreciate you.
