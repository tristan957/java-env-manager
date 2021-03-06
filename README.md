[![pipeline status](https://gitlab.com/tristan957/java-env-manager/badges/master/pipeline.svg)](https://gitlab.com/tristan957/java-env-manager/commits/master)
[![License Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-yellow.svg)](https://www.apache.org/licenses/LICENSE-2.0)

# Java Environment Manager

A version manager for Java

## How it Works

A Java distribution comes packaged like this:

```text
jdk/
├── bin
│   └── ...
├── conf
│   └── ...
├── include
│   └── ...
├── jmods
│   └── ...
├── legal
│   └── ...
├── lib
│   └── ...
├── man
│   └── ...
└── release
```

When you install the distrbution through your package manager, your executables
can get overwritten if you have both JDK 7 and JDK 8 installed. Java Environment
Manager will create symlinks to the binaries which should be located in something
akin to `/usr/lib64/openjdk-8/bin` into `JAVA_ENV_MANAGER_HOME/bin`. From there
the `JAVA_ENV_MANAGER_HOME/bin` will need to be added to your `PATH` before
`/usr/bin`. Now it is off to the races. You can change distributions
dynamically on a project-by-project basis.

## How to Build

### Commands

For Debug:

```text
cargo build
```

For Release:

```text
cargo build --release
```

Executables are located at `target/debug` or `target/release`.

```text
USAGE:
    java-env-manager [FLAGS] [SUBCOMMAND]

FLAGS:
    -d, --debug      Send debug messages to standard out
    -h, --help       Prints help information
    -v, --version    Prints the Java Environment Manager version

SUBCOMMANDS:
    add        Add a Java version
    doctor     Checks for problems with the settings.json file
    help       Prints this message or the help of the given subcommand(s)
    init       Initialize the Java Environment Manager
    list       List all known Java versions
    remove     Remove a Java version
    set        Set the current Java version
    update     Update the path a name points to
    version    Print the name of the current Java version
    which      Print the path to the current Java executables
```

### Running Tests

```text
cargo test
```

## Roadmap to 1.0

1. Complete unit testing
2. Set up logging

## Further

1. Finish GUI
2. Check if it works on windows
