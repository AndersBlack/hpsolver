# hp_solver

hp_solver is a Rust application for solving hierarchical planning problems. <br>
The project has been develop as the Master thesis in Computer Science for Anders Larsen and Andreas Kinch Jessen in 2024. 

## Installation

The application requires Cargo and Rust to be installed on the device.

## Usage

The _/src/bin/_ folder contains runnable instances of the application features which will be explained below. <br>
Problems can be located in the _/problems_ folder

### Running an individual problem

In order to run a single problem, simply run the **run_depth_first** bin using cargo:

```
cargo run --bin run_depth_first path/to/problem/file.hddl path/to/domain/file.hddl
```

### Running a folder of problems

In order to run a single problem, simply run the **run_competition_folder** bin using cargo:

```
cargo run --bin run_competition_folder path/to/problem/folder/
```

A binary has been prepared to run a subset of problems. This can be run using the following command

```
cargo run --bin run_all
```

### Running the entire [IPC2020](https://ipc2020.hierarchical-task.net/) 

The following runs the IPC2020 competition in its entirety. Be aware that since every problem has 30 minutes of available runtime, this command can potentially run for a very long time.

```
cargo run --bin run_competition
```

## Image

An image containing the application can be found a [Image]()

## Additional Software

## Contribution

Special thanks to Luis?
