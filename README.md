# RustCProject

A basic template to create a library in Rust that can be directly used in C/C++.

## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)

## Introduction

RustCProject is a template repository for creating libraries in Rust that can be used in C and C++ projects. It provides a basic setup that includes Rust, Shell, C++, and CMake configurations.

## Getting Started

### Prerequisites

- Rust: [Install Rust](https://www.rust-lang.org/tools/install)
- CMake: [Install CMake](https://cmake.org/install/)
- C/C++ compiler: Ensure you have a compatible C/C++ compiler installed

### Building the Project

To build the project, you can use the `compil.sh` script provided in the repository. This script handles building the Rust library, copying the necessary files, and compiling the C++ project.

1. Clone the repository:
   ```sh
   git clone https://github.com/MoutOgm/RustCProject.git
   cd RustCProject
   ```

2. Run the build script:
   ```sh
   ./compil.sh
   ```

3. To execute the compiled program, use:
   ```sh
   ./compil.sh run
   ```

### Using the `build-cxx` Repo

To duplicate a library from the `build-cxx` repository with minimal history, you can clone it using:
```sh
git clone --depth 1 https://github.com/username/build-cxx.git
```

This command will clone only the latest snapshot of the `build-cxx` repository.
